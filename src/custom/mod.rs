use anyhow::{Context, Error};
use clap::{Args};
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::{Read, stdin, ErrorKind};
use std::vec;
use tokio_modbus::prelude::{Request, Response};

use crate::CommandResult;
use crate::client::ReaderExt;

async fn get_buf_for_file(file_name: &str) -> Result<VecDeque<u8>, Error> {
    let mut buf: Vec<u8> = vec![];
    
    if file_name == "" || file_name == "-" {
        stdin().lock().read_to_end(&mut buf)
            .with_context(|| format!("failed to read stdin"))?;
    } else {
        File::open(file_name)
            .with_context(|| format!("failed to open '{}'", file_name))?
            .read_to_end(&mut buf)
            .with_context(|| format!("failed to read '{}'", file_name))?;
    }

    Ok(buf.into())
}
/// Send custom bytestrings to the remote terminal
#[derive(Args, Clone, Debug)]
pub struct CustomArgs {
    #[clap(value_parser, default_value = "-")]
    pub input_file: String,
}

pub async fn custom_action(client: &mut dyn ReaderExt, args: CustomArgs) -> Result<CommandResult, Error> {
    let mut buf = get_buf_for_file(&args.input_file).await?;
    match buf.pop_front() {
        Some(function_id) => {
            let response = client.call(Request::Custom(function_id, buf.into())).await?;
            
            let rows: Vec<Vec<String>> = match response {
                Response::Custom(func_code, data) => data
                        .iter()
                        .map(|&x| vec![
                            format!("0x{:X}", func_code),
                            std::str::from_utf8(&[x])
                                .unwrap()
                                .to_string(),
                        ])
                        .collect::<Vec<Vec<String>>>(),
                _ => vec![vec![]],
            };
            Ok(CommandResult{columns: vec![], rows})
        },
        None => Err(
            Error::new(io::Error::new(ErrorKind::InvalidInput, "input file is empty"))
        )
    }
}