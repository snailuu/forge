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
        /// Deploy user name (creates user if provided)
        #[arg(long)]
        user: Option<String>,

        /// Project name (creates project directory if provided)
        #[arg(long)]
        project: Option<String>,

        /// Site domain or IP address
        #[arg(long, default_value = "localhost")]
        domain: String,

        /// SSH public key for deploy user
        #[arg(long)]
        ssh_key: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { user, project, domain, ssh_key } => {
            commands::init::run(user, project, domain, ssh_key)
        }
    }
}
