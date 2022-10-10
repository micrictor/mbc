use std::collections::VecDeque;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use tokio_serial;
use tokio_modbus::prelude::{Reader, Request, Response, rtu, tcp};
pub use tokio_modbus::client::{Client, Context};
use async_trait::async_trait;
use byteorder::{BigEndian, ReadBytesExt};

use crate::args::Args;
use crate::uri::Proto;

const READ_FILE_RECORD: u8 = 0x14;
const READ_FIFO_QUEUE: u8 = 0x18;
const READ_DEVICE_IDENTIFICATION: u8 = 0x2B;
const MEI_CODE: u8 = 0x0E;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileRecord {
    pub resp_data_len: u8,
    pub file_resp_len: u8,
    pub ref_type: u8,
    pub record_data: Vec<u16>,
}

impl TryFrom<Vec<u8>> for FileRecord {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> Result<FileRecord, Error> {
        if data.len() < 5 {
            return Err(Error::new(ErrorKind::InvalidData, format!("response data len {}, want >= 5", data.len())))
        }
        
        let resp_data_len = data[0];
        let file_resp_len = data[1];
        let ref_type = data[2];

        let mut record_data: Vec<u16> = vec![];
        for i in (2..data.len()).step_by(2) {
            record_data.push((&data[i..i+1]).read_u16::<BigEndian>().unwrap());
        }

        Ok(FileRecord{resp_data_len, file_resp_len, ref_type, record_data})
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)] 
pub enum DeviceIdentificationCode {
    Basic = 1,
    Regular = 2,
    Extended = 3,
    Individual = 4,
    Unknown = 0xFF,
}

impl From<u8> for DeviceIdentificationCode {
    fn from(v: u8) -> Self {
        match v {
            1 => DeviceIdentificationCode::Basic,
            2 => DeviceIdentificationCode::Extended,
            3 => DeviceIdentificationCode::Regular,
            4 => DeviceIdentificationCode::Individual,
            _ => DeviceIdentificationCode::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceConformity {
    Basic = 1,
    Regular = 2,
    Extended = 3,
    BasicWithIndividual = 0x81,
    RegularWithIndividual = 0x82,
    ExtendedWithIndividual = 0x83,
    Unknown,
}

impl From<u8> for DeviceConformity {
    fn from(v: u8) -> Self {
        match v {
            1 => DeviceConformity::Basic,
            2 => DeviceConformity::Extended,
            3 => DeviceConformity::Regular,
            0x81 => DeviceConformity::BasicWithIndividual,
            0x82 => DeviceConformity::ExtendedWithIndividual,
            0x83 => DeviceConformity::RegularWithIndividual,
            _ => DeviceConformity::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceIDObject {
    pub id: u8,
    pub length: u8,
    pub value: Vec<u8>,
}

impl From<VecDeque<u8>> for DeviceIDObject {
    fn from(mut v: VecDeque<u8>) -> Self {
        let id = v.pop_front()
            .unwrap();
        let length = v.pop_front()
            .unwrap();
        DeviceIDObject { id, length, value: v.clone().into() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceIdentification {
    pub mei_type: u8,
    pub read_device_id_code: DeviceIdentificationCode,
    pub conformity_level: DeviceConformity,
    pub more_follows: u8,
    pub next_object_id: u8,
    pub number_of_objects: u8,
    pub objects: Vec<DeviceIDObject>,
}

impl From<VecDeque<u8>> for DeviceIdentification {
    fn from(mut data: VecDeque<u8>) -> DeviceIdentification {
        let mei_type = data.pop_front()
            .unwrap();
        let read_device_id_code: DeviceIdentificationCode = data.pop_front()
            .unwrap()
            .into();
        let conformity_level: DeviceConformity = data.pop_front()
            .unwrap()
            .into();
        let more_follows = data.pop_front()
            .unwrap();
        let next_object_id = data.pop_front()
            .unwrap();
        let number_of_objects = data.pop_front()
            .unwrap();

        let mut objects: Vec<DeviceIDObject> = vec![];
        for _ in 0..number_of_objects {
            let object_len = data[1];
            let buffer_length: usize = usize::from(2 + object_len);
            let mut object_buffer = data.clone();
            object_buffer.truncate(buffer_length);
            objects.push(object_buffer.into());
        }

        DeviceIdentification{mei_type, read_device_id_code, conformity_level, more_follows, next_object_id, number_of_objects, objects}
    }
}

#[async_trait]
pub trait ReaderExt: Reader {
    /// Read a file record.
    /// A file is an organization of records. Each file contains 10000(0x270F) records, 0-indexed.
    /// Though not implemented here, the Modbus spec permits multiple records to be read per call 
    async fn read_file_record(&mut self, file_number: u16, starting_record: u16, record_length: u16) -> Result<FileRecord, Error>;
    /// Read a First-In, First-Out (FIFO) queue of registers on the remote device.
    /// Up to 31 data registers can be read. Queue contents are read, but not cleared.
    async fn read_fifo_queue(&mut self, pointer_address: u16) -> Result<Vec<u16>, Error>;
    /// Read device identification information. Requires device to implement Modbus Encapsulated Interface (MEI).
    async fn read_device_identification(&mut self, id_code: DeviceIdentificationCode, object_id: u8) -> Result<DeviceIdentification, Error>;
}

#[async_trait]
impl ReaderExt for Context {
    async fn read_file_record(&mut self, file_number: u16, starting_record: u16, record_length: u16) -> Result<FileRecord, Error> {
        let mut request: Vec<u8> = vec![6]; // Reference type is always 6.
        let mut args_vec: Vec<u8> = vec![file_number, starting_record, record_length]
            .iter()
            .flat_map(|&x| x.to_be_bytes())
            .collect();
        request.append(&mut args_vec);
        request.insert(0, request.len() as u8);
        let rsp = self.call(Request::Custom(READ_FILE_RECORD, request)).await?;
        match rsp {
            Response::Custom(_func_code, response_vec) => {
                Ok(FileRecord::try_from(response_vec)?)
            },
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid response data"))
        }
        
    }

    async fn read_fifo_queue(&mut self, pointer_address: u16) -> Result<Vec<u16>, Error> {
        let rsp = self.call(Request::Custom(READ_FIFO_QUEUE, pointer_address.to_be_bytes().into())).await?;
        match rsp {
            Response::Custom(_func_code, raw_vec) => {
                let mut response_vec: Vec<u16> = vec![];
                // The first 4 values aren't super meaningful to us, as they describe the size and number of items returned.
                // At some point, it may be worth reading them to fail more gracefully if we recieve malformed responses,
                // but for now we'll just rely on Vec's panic if we try to read out of bounds. 
                for i in (4..raw_vec.len()).step_by(2) {
                    response_vec.push((&raw_vec[i..i+1]).read_u16::<BigEndian>().unwrap());
                }

                Ok(response_vec)
            },
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid response data"))
        }
    }

    async fn read_device_identification(&mut self, id_code: DeviceIdentificationCode, object_id: u8) -> Result<DeviceIdentification, Error> {
        let request: Vec<u8> = vec![MEI_CODE, id_code as u8, object_id];
        
        let rsp = self.call(Request::Custom(READ_DEVICE_IDENTIFICATION, request)).await?;
        match rsp {
            Response::Custom(_func_code, raw_vec) => {
                let vecdeq: VecDeque<u8> = raw_vec.into();
                Ok(vecdeq.into())
            },
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid response data"))
        }
    }
}

async fn get_tcp_client(host: String, port: u16) -> Result<Context, Error> {
    let socket: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
    let ctx = tcp::connect(socket).await?;
    Ok(ctx)
}

async fn get_rtu_client(device_path: String, bitrate: u32, terminal_id: u8) -> Result<Context, Error> {
    let terminal = tokio_modbus::slave::Slave(terminal_id);
    let builder = tokio_serial::new(device_path, bitrate);
    let stream = tokio_serial::SerialStream::open(&builder)?;
    let ctx = rtu::connect_slave(stream, terminal).await?;
    Ok(ctx)
}


// Can't impl TryFrom becuase this is all async
pub async fn context_try_from(args: Args) -> Result<Context, Error> {
    match args.uri.proto {
        Proto::Tcp => Ok(get_tcp_client(args.uri.host, args.uri.port).await?),
        Proto::Rtu => Ok(get_rtu_client(args.uri.host, args.uri.port.into(), args.terminal_id).await?),
    }
}
