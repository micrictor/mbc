use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use tokio_serial;
use tokio_modbus::prelude::{Request, Response, rtu, tcp};
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

#[async_trait]
pub trait ReaderExt: tokio_modbus::prelude::Reader {
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
                let resp_data_len = response_vec[0];
                let file_resp_len = response_vec[1];
                let ref_type = response_vec[2];

                let mut record_data: Vec<u16> = vec![];
                for i in (2..response_vec.len()).step_by(2) {
                    record_data.push((&response_vec[i..i+1]).read_u16::<BigEndian>().unwrap());
                }

                Ok(FileRecord{resp_data_len, file_resp_len, ref_type, record_data})
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