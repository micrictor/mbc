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

#[async_trait]
pub trait ReaderExt: Reader {
    /// Read a file record.
    /// A file is an organization of records. Each file contains 10000(0x270F) records, 0-indexed.
    /// Though not implemented here, the Modbus spec permits multiple records to be read per call 
    async fn read_file_record(&mut self, file_number: u16, starting_record: u16, record_length: u16) -> Result<FileRecord, Error>;
    /// Read a First-In, First-Out (FIFO) queue of registers on the remote device.
    /// Up to 31 data registers can be read. Queue contents are read, but not cleared.
    async fn read_fifo_queue(&mut self, pointer_address: u16) -> Result<Vec<u16>, Error>;
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


pub async fn context_try_from(args: Args) -> Result<Context, Error> {
    match args.uri.proto {
        Proto::Tcp => Ok(get_tcp_client(args.uri.host, args.uri.port).await?),
        Proto::Rtu => Ok(get_rtu_client(args.uri.host, args.uri.port.into(), args.terminal_id).await?),
    }
}
