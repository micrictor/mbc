use anyhow::Context;
use clap::Args;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, stdin, Error, ErrorKind};
use std::vec;
use tokio_modbus::prelude::{Request, Response};

use crate::CommandResult;
use crate::client::ReaderExt;

fn parse_file_input(arg: &str) -> Result<VecDeque<u8>, anyhow::Error> {
    let mut buf: Vec<u8> = vec![];
    if arg == "" || arg == "-" {
        stdin().lock().read_to_end(&mut buf)
            .with_context(|| format!("failed to read from stdin"))?;
    } else {
        File::open(arg)
            .with_context(|| format!("failed to open '{}'", arg))?
            .read_to_end(&mut buf)
            .with_context(|| format!("failed to read '{}'", arg))?;
    }
    Ok(buf.into())
}

/// Send custom bytestrings to the remote terminal
#[derive(Args, Clone, Debug)]
pub struct CustomArgs {
    #[clap(value_parser = parse_file_input, default_value = "-")]
    pub input_file: VecDeque<u8>,
}


pub async fn custom_action(client: &mut dyn ReaderExt, args: CustomArgs) -> Result<CommandResult, Error> {
    let mut input_buf = args.input_file.clone();
    match input_buf.pop_front() {
        Some(function_id) => {
            let response = client.call(Request::Custom(function_id, input_buf.into())).await?;
            
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
        None => Err(Error::new(ErrorKind::InvalidInput, "input file is empty"))
    }
}