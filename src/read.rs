use clap::{Args, ValueEnum, Subcommand};
use tokio_modbus::{client::Context, prelude::Reader};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ReadFuncs {
    /// coil value(s)
    Coils,
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
    #[clap(value_enum)]
    function: ReadFuncs,

    #[clap(subcommand)]
    function_args: FuncArgs,
}

#[derive(Args, Debug)]
struct CoilsArgs {
    #[clap(value_parser)]
    address: u16,

    #[clap(value_parser)]
    quantity: u16,
}

#[derive(Debug, Subcommand)]
enum FuncArgs {
    Coils(CoilsArgs),
}

pub fn read_action(client: &mut Context, args: ReadArgs) {
    match (args.function, args.function_args) {
        (ReadFuncs::Coils, FuncArgs::Coils(func_args)) => {
            client.read_coils(func_args.address, func_args.quantity);
        },
        (_, _) => (),
    };
    ()
}