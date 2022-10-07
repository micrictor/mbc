use std::io::{Error, ErrorKind};
use tokio_modbus::{client::Context, prelude::Reader};

pub mod args;

pub async fn read_action(client: &mut Context, args: args::ReadArgs) -> Result<(), Error>{
    match args.function {
        args::ReadFuncs::Coils(args) => {
            let coil_statuses = client.read_coils(args.address, args.quantity).await?;
            println!("Coil statuses:\n\tAddress\tStatus\n");
            for idx in 0..coil_statuses.len() {
                println!("\t{}\t{}", usize::from(args.address) + idx, coil_statuses[idx])
            };
            Ok(())
        },
        _ => Err(Error::new(ErrorKind::InvalidInput, "invalid read function specified")),
    }
}