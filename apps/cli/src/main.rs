use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[command(name = "ghostmesh", version, about = "Privacy-first peer-to-peer messaging")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print client and protocol status.
    Status,
    /// Identity commands; key generation is not implemented yet.
    Identity {
        #[command(subcommand)]
        action: IdentityCommand,
    },
}

#[derive(Debug, Subcommand)]
enum IdentityCommand {
    /// Show identity implementation status.
    Status,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    match Cli::parse().command {
        Command::Status => {
            println!("GhostMesh {}", env!("CARGO_PKG_VERSION"));
            println!("Protocol schema: {}", ghostmesh_protocol::PROTOCOL_VERSION);
            println!("Transport: not configured (development scaffold)");
        }
        Command::Identity { action: IdentityCommand::Status } => {
            println!("Identity key management is not implemented yet.");
            println!("No keys have been generated or stored.");
        }
    }
    Ok(())
}
