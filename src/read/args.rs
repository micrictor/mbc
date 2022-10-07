
use clap::{Args, Subcommand};

/// Read status information from the remote bus
#[derive(Args, Clone, Debug)]
pub struct ReadArgs {
    #[clap(subcommand)]
    pub function: ReadFuncs,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ReadFuncs {
    /// coil value(s)
    Coils(AddrQuantity),
    // input value(s)
    DiscreteInputs(AddrQuantity),
    // input register value(s)
    InputRegisters(AddrQuantity),
    // holding register value(s)
    HoldingRegisters(AddrQuantity),
}

#[derive(Args, Clone, Debug)]
pub struct AddrQuantity {
    #[clap(value_parser)]
    pub address: u16,

    #[clap(value_parser)]
    pub quantity: u16,
}