use anyhow::{Context, Error};

use crate::client::WriterExt;
use crate::CommandResult;

pub mod args;

pub async fn write_action(client: &mut dyn WriterExt, args: args::WriteArgs) -> Result<CommandResult, Error> {

    match args.function {
        args::WriteFuncs::Coil(coil) => {
            client.write_single_coil(coil.address, coil.status)
                .await
                .with_context(|| format!("failed to write single coil at address '{}' with value '{}'", coil.address, coil.status))?;
            Ok(CommandResult { columns: vec!["status".to_string()], rows: vec![vec!["success".to_string()]] })
        },
        args::WriteFuncs::Coils(coils) => {
            client.write_multiple_coils(coils.starting_address, &coils.status)
                .await
                .with_context(|| 
                    format!("failed to write {} coils starting at address '{}'", coils.status.len(), coils.starting_address)
                )?;
            Ok(CommandResult { columns: vec!["status".to_string()], rows: vec![vec!["success".to_string()]] })
        },
        args::WriteFuncs::Register(register) => {
            client.write_single_register(register.address, register.value)
                .await
                .with_context(||
                    format!("failed to write single register at address '{}' with value '0x{:X}'", register.address, register.value)
                )?;
            Ok(CommandResult { columns: vec!["status".to_string()], rows: vec![vec!["success".to_string()]] })
        },
        args::WriteFuncs::Registers(registers) => {
            client.write_multiple_registers(registers.starting_address, &registers.value)
                .await
                .with_context(||
                    format!("failed to write {} coils starting at address '{}'", registers.value.len(), registers.starting_address)
                )?;
            Ok(CommandResult { columns: vec!["status".to_string()], rows: vec![vec!["success".to_string()]] })
        },
        args::WriteFuncs::FileRecord(file) => {
            let record_len = file.record_data.len();
            client.write_file_record(file.file_number, file.record_number, file.record_data)
                .await
                .with_context(||
                    format!("failed to write file #{} record {} with {} words", file.file_number, file.record_number, record_len)
                )?;
            Ok(CommandResult { columns: vec!["status".to_string()], rows: vec![vec!["success".to_string()]] })
        },
        
    }
}