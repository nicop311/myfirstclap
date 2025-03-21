use clap::{Parser, Subcommand};
use anyhow::Result;
use std::env;
use pretty_env_logger;

#[derive(Parser)]
#[command(name = "myfirstclap")]
#[command(version = version::get_describe())]
struct Cli {
    #[arg(long, default_value = "info", help = "Set the log verbosity level (error, warn, info, debug, trace)")]
    log_level: String,

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

pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    // Initialize logging and set log level
    env::set_var("RUST_LOG", &cli.log_level);
    pretty_env_logger::init_timed();

    match cli.command {
        Commands::Version { output } => version::run(output)?,
        Commands::Serve { command } => match command {
            ServeCommands::Hello => serve::hello::run()?,
        },
    }
    Ok(())
}
