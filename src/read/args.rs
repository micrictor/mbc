
use clap::{Args, Subcommand};
use crate::client::DeviceIdentificationCode;

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
    /// input value(s)
    DiscreteInputs(AddrQuantity),
    /// input register value(s)
    InputRegisters(AddrQuantity),
    /// holding register value(s)
    HoldingRegisters(AddrQuantity),
    /// file record(s)
    FileRecords(FileReference),
    /// FIFO queue
    FIFOQueue(QueueAddress),
    /// Device Identification
    DeviceIdentification(DeviceIdentification),
}

#[derive(Args, Clone, Debug)]
pub struct AddrQuantity {
    /// starting address
    #[clap(value_parser)]
    pub address: u16,

    /// number of addresses to read, between 1 and 2000
    #[clap(value_parser = clap::value_parser!(u16).range(1..2001))]
    pub quantity: u16,
}

#[derive(Args, Clone, Debug)]
pub struct FileReference {
    /// file number
    #[clap(value_parser)]
    pub file_number: u16,

    /// starting record number, between 0 and 9999
    #[clap(value_parser = clap::value_parser!(u16).range(0..10000))]
    pub starting_record: u16,

    /// length of the record to be read
    #[clap(value_parser)]
    pub record_length: u16,
}

#[derive(Args, Clone, Debug)]
pub struct QueueAddress {
    /// file number
    #[clap(value_parser)]
    pub pointer_address: u16,
}

#[derive(Args, Clone, Debug)]
pub struct DeviceIdentification {
    /// device identification code.
    #[clap(value_enum)]
    pub device_id_code: DeviceIdentificationCode,
    /// object to be read
    #[clap(value_parser)]
    pub object_id: u8,
}