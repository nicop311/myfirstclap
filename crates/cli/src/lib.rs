use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "myfirstclap")]
#[command(version = version::get_describe())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Version {
        #[arg(short, long, default_value = "text")]
        output: String,
    },
    Serve {
        #[command(subcommand)]
        command: ServeCommands,
    },
}

#[derive(Subcommand)]
enum ServeCommands {
    Hello,
}

pub fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Version { output } => version::run(output)?,
        Commands::Serve { command } => match command {
            ServeCommands::Hello => serve::hello::run()?,
        },
    }
    Ok(())
}
