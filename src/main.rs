use clap::{Parser, Subcommand};

mod commands;
mod nginx;
mod system;
mod user;
mod template;

#[derive(Parser)]
#[command(name = "forge")]
#[command(about = "Server initialization and deployment tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize server (nginx + deploy user)
    Init {
        /// SSH public key for deploy user
        #[arg(long)]
        ssh_key: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { ssh_key } => commands::init::run(ssh_key),
    }
}
