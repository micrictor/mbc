use clap::{Parser, Subcommand};
use crate::{custom, read, write, uri};

#[derive(Clone, Parser, Debug)]
#[clap(author="Michael Torres", about="A CLI for making Modbus requests")]
pub struct Args {
    /// URI for the Modbus connection. Supported schemes are rtu, tcp
    /// 
    /// For rtu URIs, the port is the bitrate (baud) for the serial interface. Default 9600
    /// For tcp URIs, default port is 502
    /// Examples: rtu:///dev/ttyUSB0, tcp://127.0.0.1:502
    #[clap(value_parser, verbatim_doc_comment)]
    pub uri: uri::ModbusUri,

    /// Local terminal ID for RTU communication.
    #[clap(value_parser, default_value_t = 42)]
    pub terminal_id: u8,

    /// File to write the results to.
    #[clap(long, short, value_parser, default_value = "stdout")]
    pub output_file: String,

    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Action {
    Read(read::args::ReadArgs),
    Custom(custom::CustomArgs),
    Write(write::args::WriteArgs),
}