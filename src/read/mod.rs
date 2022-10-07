use tokio_modbus::{client::Context, prelude::Reader};

pub mod args;

pub fn read_action(client: &mut Context, args: args::ReadArgs) {
    match args.function {
        args::ReadFuncs::Coils(args) => {
            client.read_coils(args.address, args.quantity);
        },
        _ => (),
    };
    ()
}