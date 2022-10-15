use anyhow::{Context, Result};
use clap::{Parser};
use output::OutputPlugin;
use std::{io::{stdout, Write}, fs::OpenOptions};
use crate::output::Output;
use tokio;

mod args;
mod client;
mod custom;
mod read;
mod output;
mod write;
mod uri;


pub struct CommandResult {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();
    let file: Box<dyn Write> = match args.clone().output_file.as_str() {
        "stdout" => Box::new(stdout()),
        _ => {
            let file_name = args.output_file.clone();
            Box::new(OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_name)
                .with_context(|| format!("could not open output file"))?
            )
        }
    };

    let mut client = client::context_try_from(args.clone())
        .await
        .with_context(|| format!("could not open `{}`", args.uri))?;

    let result = match args.action {
        args::Action::Read(read_args) => read::read_action(&mut client, read_args.clone())
            .await
            .with_context(|| format!("could not read `{:?}`", read_args))?,
        args::Action::Custom(custom_args) => custom::custom_action(&mut client, custom_args)
            .await
            .with_context(|| "failed to send custom command")?,
        args::Action::Write(write_args) => write::write_action(&mut client, write_args.clone())
            .await
            .with_context(|| "failed to write")?,
    };

    let mut outputter: Box<dyn output::Output> = match args.output_plugin {
        OutputPlugin::Csv => Box::new(output::CsvOutput{file}),
        OutputPlugin::Tsv => Box::new(output::TsvOutput{file}),
        OutputPlugin::Json => Box::new(output::JsonOutput{file}),
    };
    outputter.write_output(result.columns, result.rows)
        .with_context(|| format!("failed to write output using {:?}", args.output_plugin))?;
    Ok(())
}
