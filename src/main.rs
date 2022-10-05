use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use tokio;

mod client;
mod read;
mod uri;

#[derive(Parser, Debug)]
#[clap(author="Michael Torres", about="A CLI for making Modbus requests")]
struct Args {
    /// URI for the Modbus connection. Supported schemes are rtu, tcp
    /// 
    /// For rtu URIs, the port is the bitrate (baud) for the serial interface. Default 9600
    /// For tcp URIs, default port is 502
    /// Examples: rtu:///dev/ttyUSB0, tcp://127.0.0.1:502
    #[clap(value_parser, verbatim_doc_comment)]
    uri: uri::ModbusUri,

    /// Local terminal ID for RTU communication.
    #[clap(value_parser, default_value_t = 42)]
    terminal_id: u8,

    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    Read(read::ReadArgs)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let uri = args.uri.clone();
    let client = client::Context::try_from(args.uri.proto, args.uri.host, args.uri.port, Some(args.terminal_id))
        .await
        .with_context(|| format!("could not open `{}`", uri))?;

    match args.action {
        Action::Read(read_args) => read::ReadAction(client, read_args)
    };
    Ok(())
}
