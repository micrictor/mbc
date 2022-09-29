use clap::{Args, ValueEnum};

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

pub fn ReadAction(args: ReadArgs) {
    ()
}