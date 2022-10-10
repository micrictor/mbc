use std::io::Error;
use anyhow::Context;

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

            let rows: Vec<Vec<String>> = inputs
                .iter()
                .enumerate()
                .map(|(i, &status)| vec![i.to_string(), status.to_string()])
                .collect();
            let columns = vec!["address".to_string(), "status".to_string()];
            Ok(CommandResult { columns, rows })
        },
        args::ReadFuncs::HoldingRegisters(args) => {
            let registers = client.read_holding_registers(args.address, args.quantity).await?;

            let rows: Vec<Vec<String>> = registers
                .iter()
                .enumerate()
                .map(|(i, &value)| vec![i.to_string(), format!("{:#04x}", value)])
                .collect();
            let columns = vec!["address".to_string(), "value".to_string()];
            Ok(CommandResult { columns, rows })
        },
        args::ReadFuncs::InputRegisters(args) => {
            let registers = client.read_input_registers(args.address, args.quantity).await?;
            
            let rows: Vec<Vec<String>> = registers
                .iter()
                .enumerate()
                .map(|(i, &value)| vec![i.to_string(), format!("{:#04x}", value)])
                .collect();
            let columns = vec!["address".to_string(), "value".to_string()];
            Ok(CommandResult { columns, rows })
        },
        args::ReadFuncs::FileRecords(args) => {
            let file_record = client.read_file_record(args.file_number, args.starting_record, args.record_length).await?;

            // File record output logic is a little less standard. 
            // Because file records may be large, I wanted to facilitate parsing by using xxd-compatible output. Thus, each
            // "value" is actually 16 values in hexadecimal notation, space delimited.
            let mut rows: Vec<Vec<String>> = vec![];
            for i in (0..file_record.record_data.len()).step_by(8) {
                let mut row: Vec<String> = vec![format!("{:#04}:", i*2)];
                let mut row_data: Vec<String> = vec![];
                for j in i..i+8 {
                    if j > file_record.record_data.len() {
                        break;
                    }
                    row_data.push(format!("{:X}", file_record.record_data[j]));
                }
                row.push(row_data.join(" "));
                rows.push(row);
            }
            let columns = vec!["offset".to_string(), "value".to_string()];
            Ok(CommandResult { columns, rows })
        },
        args::ReadFuncs::FIFOQueue(args) => {
            let queue_items = client.read_fifo_queue(args.pointer_address).await?;

            let rows: Vec<Vec<String>> = queue_items
                .iter()
                .enumerate()
                .map(|(i, &value)| vec![i.to_string(), format!("{:#04x}", value)])
                .collect();
            let columns = vec!["offset".to_string(), "value".to_string()];
            Ok(CommandResult { columns, rows })
        },
        args::ReadFuncs::DeviceIdentification(args) => {
            let device_id = client.read_device_identification(args.device_id_code, args.object_id).await?;
            let rows: Vec<Vec<String>> = device_id.objects
                .iter()
                .map(|o| vec![
                    o.id.to_string(),
                    std::str::from_utf8(&o.value)
                        .with_context(|| format!("failed to decode object as utf8"))
                        .unwrap()
                        .to_string(),
                    device_id.conformity_level.to_string()
                ])
                .collect();
            let columns = vec![
                "object_id".to_string(),
                "value".to_string(),
                "conformity_level".to_string()
            ];
            Ok(CommandResult { columns, rows })
        },
    }
}