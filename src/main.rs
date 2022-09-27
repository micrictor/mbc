use clap::Parser;

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

    #[clap(value_parser)]
    action: String,

    #[clap(value_parser)]
    command: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
