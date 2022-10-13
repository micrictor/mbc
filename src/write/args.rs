
use clap::{Args, Subcommand};

/// Read status information from the remote bus
#[derive(Args, Clone, Debug)]
pub struct WriteArgs {
    #[clap(subcommand)]
    pub function: WriteFuncs,
}

#[derive(Clone, Debug, Subcommand)]
pub enum WriteFuncs {
    /// single coil
    Coil(SingleCoil),
    /// multi coil
    Coils(MultiCoil),
    /// single register
    Register(SingleRegister),
    /// multi register
    Registers(MultiRegister),
    /// file record(s)
    FileRecord(FileRecord),
}

type Address = u16;
type Coil = bool;
type Word = u16;

#[derive(Args, Clone, Debug)]
pub struct SingleCoil {
    /// coil address
    #[clap(value_parser)]
    pub address: Address,

    /// status to write
    #[clap(value_parser)]
    pub status: Coil,
}

#[derive(Args, Clone, Debug)]
pub struct MultiCoil {
    /// starting coil address
    #[clap(value_parser)]
    pub starting_address: Address,

    /// statuses to write
    #[clap(value_parser)]
    pub status: Vec<Coil>,
}

#[derive(Args, Clone, Debug)]
pub struct SingleRegister {
    /// register address
    #[clap(value_parser)]
    pub address: Address,

    /// value to write
    #[clap(value_parser)]
    pub value: Word,
}

#[derive(Args, Clone, Debug)]
pub struct MultiRegister {
    /// starting register address
    #[clap(value_parser)]
    pub starting_address: Address,

    /// values to write
    #[clap(value_parser)]
    pub value: Vec<Word>,
}

#[derive(Args, Clone, Debug)]
pub struct FileRecord {
    /// file number
    #[clap(value_parser)]
    pub file_number: u16,

    /// starting record number, between 0 and 9999
    #[clap(value_parser = clap::value_parser!(u16).range(0..10000))]
    pub record_number: u16,

    /// length of the record to be read
    #[clap(value_parser)]
    pub record_data: Vec<u16>,
}
