pub mod api;
pub mod argument;
pub mod command;
pub mod entity;
pub mod film;
pub mod server;
pub mod utility;

use argument::Arguments;
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn setup_normal_logging() {
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
    );
    pretty_env_logger::env_logger::init();
}

fn setup_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    println!("using input arguments: {}", args);

    if args.mode.to_lowercase() == "server" {
        setup_tracing();
        let res = server::run(args.clone()).await;
        if let Err(e) = res {
            tracing::error!("failed to run in server mode, cause: {}", e);
        }
        return;
    }

    setup_normal_logging();
    command::run(args).await;
}
