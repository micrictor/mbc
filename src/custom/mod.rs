use anyhow::Context;
use clap::Args;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, stdin};

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