pub mod api;
pub mod argument;
pub mod command;
pub mod entity;
pub mod film;
pub mod server;
pub mod utility;

use argument::Arguments;
use clap::Parser;
use log::{error, info};

fn set_default_log_level() {
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
    );
    pretty_env_logger::env_logger::init();
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    set_default_log_level();
    info!("using input arguments: {:?}", args);

    if args.mode.to_lowercase() == "server" {
        let res = server::run(args.clone()).await;
        if let Err(e) = res {
            error!("failed to run in server mode, cause: {}", e);
            return;
        }
    }
    command::run(args);
}
