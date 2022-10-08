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
    async fn read_file_record(&mut self, file_number: u16, starting_record: u16, record_length: u16) -> Result<FileRecord, Error>;
}

#[async_trait]
impl ReaderExt for Context {
    async fn read_file_record(&mut self, file_number: u16, starting_record: u16, record_length: u16) -> Result<FileRecord, Error> {
        let mut request: Vec<u8> = vec![6];
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
            _ => Err(Error::new(ErrorKind::InvalidData, "invalud response data"))
        }
        
    }
}

#[tokio::main]
async fn get_tcp_client(host: String, port: u16) -> Result<Context, Error> {
    let socket: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
    let ctx = tcp::connect(socket).await?;
    Ok(ctx)
}

#[tokio::main]
async fn get_rtu_client(device_path: String, bitrate: u32, terminal_id: u8) -> Result<Context, Error> {
    let terminal = tokio_modbus::slave::Slave(terminal_id);
    let builder = tokio_serial::new(device_path, bitrate);
    let stream = tokio_serial::SerialStream::open(&builder)?;
    let ctx = rtu::connect_slave(stream, terminal).await?;
    Ok(ctx)
}

impl TryFrom<Args> for Context {
    type Error = Error;
    fn try_from(args: Args) -> Result<Context, Error> {
        match args.uri.proto {
            Proto::Tcp => get_tcp_client(args.uri.host, args.uri.port),
            Proto::Rtu => get_rtu_client(args.uri.host, args.uri.port.into(), args.terminal_id),
        }
    }
}