use clap::{Args, ValueEnum};
use std::future::Future;
use std::result::Result;
use tokio_modbus::client::Context;

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
    function: ReadFuncs
}

pub fn ReadAction(client: Context, args: ReadArgs) {
    ()
}