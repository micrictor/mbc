use clap::{Args, ValueEnum, Subcommand};
use tokio_modbus::{client::Context, prelude::Reader};

#[derive(Debug, Subcommand)]
enum ReadFuncs {
    /// coil value(s)
    Coils(CoilsArgs),
    // input value(s)
    DiscreteInputs,
    // input register value(s)
    InputRegisters,
    // holding register value(s)
    HoldingRegisters,
}


/// Read status information from the remote bus
#[derive(Args, Debug)]
pub struct ReadArgs {
    #[clap(subcommand)]
    function: ReadFuncs,
}

#[derive(Args, Clone, Debug)]
struct CoilsArgs {
    #[clap(value_parser)]
    address: u16,

    #[clap(value_parser)]
    quantity: u16,
}

pub fn read_action(client: &mut Context, args: ReadArgs) {
    match args.function {
        ReadFuncs::Coils(args) => {
            client.read_coils(args.address, args.quantity);
        },
        _ => (),
    };
    ()
}