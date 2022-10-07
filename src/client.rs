use std::io::Error as IOError;
use std::net::SocketAddr;
use tokio_serial;
use tokio_modbus::client::{Context as ParentContext, rtu, tcp};

use crate::uri::{ModbusUri, Proto};

pub struct Context {}

async fn try_from_tcp(host: String, port: u16) -> Result<ParentContext, IOError> {
    let socket: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
    let ctx = tcp::connect(socket).await?;
    Ok(ctx)
}

async fn try_from_rtu(device_path: String, bitrate: u32, terminal_id: u8) -> Result<ParentContext, IOError> {
    let terminal = tokio_modbus::slave::Slave(terminal_id);
    let builder = tokio_serial::new(device_path, bitrate);
    let stream = tokio_serial::SerialStream::open(&builder)?;
    let ctx = rtu::connect_slave(stream, terminal).await?;
    Ok(ctx)
}

impl Context {
    pub async fn try_from(uri: ModbusUri, terminal_id: Option<u8>) -> Result<ParentContext, IOError> {
        match uri.proto {
            Proto::Tcp => try_from_tcp(uri.host, uri.port).await,
            Proto::Rtu => try_from_rtu(uri.host, uri.port.into(), terminal_id.unwrap_or(42)).await,
        }
    }
}