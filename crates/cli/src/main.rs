use clap::{Parser, Subcommand, CommandFactory};
use clap_complete::{generate, Shell};
use anyhow::Result;
use std::env;
use std::io;
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
    /// A more detailed version command with information from the build.
    Version {
        #[arg(short, long, default_value = "text", help = "Supported values: text, json, full.")]
        output: String,
    },
    /// A collection of several trivial servers
    Serve {
        #[command(subcommand)]
        command: ServeCommands,
    },
    /// Completion scripts for various terminals.
    Completion {
        #[arg(short, long, default_value = "bash", help = "Supported values: bash.")]
        shell: Shell,
    },
}

#[derive(Subcommand)]
enum ServeCommands {
    Hello,
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    // Initialize logging and set log level
    env::set_var("RUST_LOG", &cli.log_level);
    pretty_env_logger::init_timed();

    match cli.command {
        Commands::Version { output } => version::run(output)?,
        Commands::Serve { command } => match command {
            ServeCommands::Hello => serve::hello::run()?,
        },
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "myfirstclap", &mut io::stdout());
        }
    }
    Ok(())
}
