use anyhow::{Context, Result};
use clap::{Parser};
use tokio;

mod args;
mod client;
mod custom;
mod read;
mod write;
mod uri;


pub struct CommandResult {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();
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

    println!("{}", result.columns.join("\t"));
    for row in result.rows {
        println!("{}", row.join("\t"));
    }
    Ok(())
}
