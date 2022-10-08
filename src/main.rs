use anyhow::{Context, Result};
use clap::{Parser};
use tokio;

mod args;
mod client;
mod read;
mod uri;

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();
    let mut client = client::Context::try_from(args.clone())
        .with_context(|| format!("could not open `{}`", args.uri))?;

    match args.action {
        args::Action::Read(read_args) => read::read_action(&mut client, read_args.clone())
            .await
            .with_context(|| format!("could not read `{:?}`", read_args))?
    };
    Ok(())
}
