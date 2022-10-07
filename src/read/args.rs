
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum ReadFuncs {
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
    pub function: ReadFuncs,
}

#[derive(Args, Clone, Debug)]
pub struct CoilsArgs {
    #[clap(value_parser)]
    pub address: u16,

    #[clap(value_parser)]
    pub quantity: u16,
}