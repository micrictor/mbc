use std::io::Error;
use crate::client::ReaderExt;

pub mod args;

pub async fn read_action(client: &mut dyn ReaderExt, args: args::ReadArgs) -> Result<(), Error>{
    match args.function {
        args::ReadFuncs::Coils(args) => {
            let coil_statuses = client.read_coils(args.address, args.quantity).await?;
            println!("Coil statuses:\n\tAddress\tStatus\n");
            for idx in 0..coil_statuses.len() {
                println!("\t{}\t{}", usize::from(args.address) + idx, coil_statuses[idx])
            };
            Ok(())
        },
        args::ReadFuncs::DiscreteInputs(args) => {
            let inputs = client.read_discrete_inputs(args.address, args.quantity).await?;
            println!("Discrete inputs:\n\tAddress\tStatus\n");
            for idx in 0..inputs.len() {
                println!("\t{}\t{}", usize::from(args.address) + idx, inputs[idx])
            };
            Ok(())
        },
        args::ReadFuncs::HoldingRegisters(args) => {
            let registers = client.read_holding_registers(args.address, args.quantity).await?;
            println!("Registers:\n\tAddress\tValue (hex)\n");
            for idx in 0..registers.len() {
                println!("\t{}\t{:#04x}", usize::from(args.address) + idx, registers[idx])
            };
            Ok(())
        },
        args::ReadFuncs::InputRegisters(args) => {
            let registers = client.read_input_registers(args.address, args.quantity).await?;
            println!("Registers:\n\tAddress\tValue (hex)\n");
            for idx in 0..registers.len() {
                println!("\t{}\t{:#04x}", usize::from(args.address) + idx, registers[idx])
            };
            Ok(())
        },
        args::ReadFuncs::FileRecords(args) => {
            client.read_file_record(args.file_number, args.starting_record, args.record_length);
            Ok(())
        }
    }
}