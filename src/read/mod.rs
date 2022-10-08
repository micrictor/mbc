use std::io::Error;
use crate::client::ReaderExt;
use crate::CommandResult;

pub mod args;

pub async fn read_action(client: &mut dyn ReaderExt, args: args::ReadArgs) -> Result<CommandResult, Error>{
    match args.function {
        args::ReadFuncs::Coils(args) => {
            let coil_statuses = client.read_coils(args.address, args.quantity).await?;

            let rows: Vec<Vec<String>> = coil_statuses
                .iter()
                .enumerate()
                .map(|(i, &status)| vec![i.to_string(), status.to_string()])
                .collect();
            let columns = vec!["address".to_string(), "status".to_string()];
            Ok(CommandResult { columns, rows })
        },
        args::ReadFuncs::DiscreteInputs(args) => {
            let inputs = client.read_discrete_inputs(args.address, args.quantity).await?;
            println!("Discrete inputs:\n\tAddress\tStatus\n");
            for idx in 0..inputs.len() {
                println!("\t{}\t{}", usize::from(args.address) + idx, inputs[idx])
            };
            Ok(CommandResult { columns: vec![], rows: vec![vec![]] })
        },
        args::ReadFuncs::HoldingRegisters(args) => {
            let registers = client.read_holding_registers(args.address, args.quantity).await?;
            println!("Registers:\n\tAddress\tValue (hex)\n");
            for idx in 0..registers.len() {
                println!("\t{}\t{:#04x}", usize::from(args.address) + idx, registers[idx])
            };
            Ok(CommandResult { columns: vec![], rows: vec![vec![]] })
        },
        args::ReadFuncs::InputRegisters(args) => {
            let registers = client.read_input_registers(args.address, args.quantity).await?;
            println!("Registers:\n\tAddress\tValue (hex)\n");
            for idx in 0..registers.len() {
                println!("\t{}\t{:#04x}", usize::from(args.address) + idx, registers[idx])
            };
            Ok(CommandResult { columns: vec![], rows: vec![vec![]] })
        },
        args::ReadFuncs::FileRecords(args) => {
            let file_record = client.read_file_record(args.file_number, args.starting_record, args.record_length).await?;
            for i in (0..file_record.record_data.len()).step_by(16) {
                print!("{:#04}:", i);
                for j in i..i+16 {
                    if j > file_record.record_data.len() {
                        break;
                    }
                    print!(" {}", file_record.record_data[j]);
                }
                print!("\n");
            }
            Ok(CommandResult { columns: vec![], rows: vec![vec![]] })
        },
        args::ReadFuncs::FIFOQueue(args) => {
            let queue_items = client.read_fifo_queue(args.pointer_address).await?;
            let output_line_map = queue_items.iter().enumerate().map(|(i, &x)| format!("{:#02}: {}\n", i, x));
            let output_lines: Vec<String> = output_line_map.collect();
            print!("{}", output_lines.join(""));
            Ok(CommandResult { columns: vec![], rows: vec![vec![]] })
        }
    }
}