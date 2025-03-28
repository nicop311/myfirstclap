use anyhow::Result;
use clap_complete::{generate, Shell};
use clap::{Parser, Subcommand, CommandFactory, ValueEnum};
use pretty_env_logger;
use std::env;
use std::io;

use log::info;

/// a local enum that mirrors the log::Level enum and derives the necessary traits
#[derive(ValueEnum, Clone)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// implement the ToString trait for the VersionOutputFormats enum
impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Error => "error".to_string(),
            LogLevel::Warn =>  "warn".to_string(),
            LogLevel::Info =>  "info".to_string(),
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Trace => "trace".to_string(),
        }
    }
}

/// Root CLI of myfirstclap.
#[derive(Parser)]
#[command(name = "myfirstclap")]
#[command(version = version::get_describe())]
pub struct Cli {
    #[arg(long, 
        default_value = "info",
        env = "MYFIRSTCLAP_LOG_LEVEL",
        value_enum,
        help = "Set the log verbosity level" )]
        log_level: LogLevel,

    #[command(subcommand)]
    command: Commands,
}

/// Supported output formats for the version CLI command
#[derive(ValueEnum, Clone)]
enum VersionOutputFormat {
    Text,
    Json,
    Full,
}

/// implement the ToString trait for the VersionOutputFormats enum
impl ToString for VersionOutputFormat {
    fn to_string(&self) -> String {
        match self {
            VersionOutputFormat::Text => "text".to_string(),
            VersionOutputFormat::Json => "json".to_string(),
            VersionOutputFormat::Full => "full".to_string(),
        }
    }
}

/// Level 1 subcommands for myfirstclap.
#[derive(Subcommand)]
enum Commands {
    /// A more detailed version command with information from the build.
    Version {
        #[arg(short,
            long,
            default_value = "text",
            value_enum,
            help = "Output format in STDOUT.")]
        output: VersionOutputFormat,
        #[arg(long,
            default_value_t = false,
            help = "Do not pretty print JSON.")]
        no_pretty: bool,
    },
    /// A collection of several trivial servers
    Serve {
        #[command(subcommand)]
        command: ServeCommands,
    },
    /// Completion scripts for various terminals.
    Completion {
        #[arg(short,
            long,
            default_value = "bash",
            value_enum,
            help = "Target shell.")]
        shell: Shell,
    },
    /// NOT WORKING YET Generate a man page for this application
    Man {
        #[arg(short, long, default_value = "myfirstclap.1")]
        output: String,
    },
}

/// Level 2 subcommands of serve
#[derive(Subcommand)]
enum ServeCommands {
    Hello,
}

/// Main entry point for the CLI application.
///
/// This function parses the command-line arguments, initializes the logging mechanism,
/// and executes the appropriate subcommand based on the user's input. The supported
/// commands include:
///
/// - `Version`: Outputs version information in the specified format (text, json, or full).
/// - `Serve`: Runs a collection of trivial servers, specifically a "Hello" server in this case.
/// - `Completion`: Generates shell completion scripts for various terminals.
/// - `Man`: Generates a man page for the application.
///
/// # Returns
///
/// Returns a `Result` indicating success or failure. Any errors in subcommand execution
/// will be propagated upwards.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    // Initialize logging and set log level
    env::set_var("RUST_LOG", &cli.log_level.to_string());
    pretty_env_logger::init_timed();

    match cli.command {
        Commands::Version { output, no_pretty } => version::run(output.to_string(), !no_pretty)?,
        Commands::Serve { command } => match command {
            ServeCommands::Hello => serve::hello::run()?,
        },
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "myfirstclap", &mut io::stdout());
        }
        Commands::Man { output } => {
            info!("Man page feature is not ready yet. The man page file {} was not generated.", output);
        }
    }
    Ok(())
}
