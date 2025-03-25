use clap::{Parser, Subcommand, CommandFactory, ValueEnum};
use clap_complete::{generate, Shell};
use anyhow::Result;
use std::env;
use std::io;
use pretty_env_logger;

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

#[derive(Parser)]
#[command(name = "myfirstclap")]
#[command(version = version::get_describe())]
struct Cli {
    #[arg(long, 
        default_value = "info",
        value_enum,
        help = format!("Set the log verbosity level ({}, {}, {}, {}, {})", 
                        LogLevel::Error.to_string(),
                        LogLevel::Warn.to_string(),
                        LogLevel::Info.to_string(),
                        LogLevel::Debug.to_string(),
                        LogLevel::Trace.to_string() ))]
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
            default_value = "true",
            help = "Pretty print JSON.")]
        pretty: bool,
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
            help = "Supported values: bash, elvish, fish, powershell, zsh.")]
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
    env::set_var("RUST_LOG", &cli.log_level.to_string());
    pretty_env_logger::init_timed();

    match cli.command {
        Commands::Version { output, pretty } => version::run(output.to_string(), pretty)?,
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
