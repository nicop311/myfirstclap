use anyhow::Result;
use serde::Serialize;
use serde_json::json;
use std::env;

#[derive(Serialize)]
pub struct VersionInfo {
    build_date: &'static str,
    build_timestamp: &'static str,
    git_commit_date: &'static str,
    git_describe: &'static str,
    git_sha: &'static str,
    os_version: &'static str,
}

/// Get a reasonable amount of information from git and from build thanks to vergen.
pub fn get_version() -> VersionInfo {
    VersionInfo {
        build_date: env!("VERGEN_BUILD_DATE"),
        build_timestamp: env!("VERGEN_BUILD_TIMESTAMP"),
        git_commit_date: env!("VERGEN_GIT_COMMIT_DATE"),
        git_describe: env!("VERGEN_GIT_DESCRIBE"),
        git_sha: env!("VERGEN_GIT_SHA"),
        os_version: env!("VERGEN_SYSINFO_OS_VERSION"),
    }
}

// get only git describe from vergen
pub fn get_describe() -> &'static str {
    env!("VERGEN_GIT_DESCRIBE")
}

/// Retrieves comprehensive versioning and build-time information as a JSON object.
///
/// This function gathers and returns various metadata about the build environment,
/// Git repository state, Rust compiler, and system information. It includes details
/// such as build date and timestamp, Git commit information, Rust compiler version,
/// and system specifications like CPU and memory.
///
/// The returned JSON object contains the following fields:
/// - `build_date`: The date when the build was created.
/// - `build_timestamp`: The precise timestamp of the build.
/// - `cargo_debug`: Debug flag for the Cargo build.
/// - `cargo_dependencies`: The list of dependencies used in the build.
/// - `cargo_features`: Features enabled for the Cargo build.
/// - `cargo_opt_level`: Optimization level used in the Cargo build.
/// - `cargo_target_triple`: The target triple for which the build was compiled.
/// - `git_branch`: Current Git branch being built from.
/// - `git_commit_author_email`: Author's email of the commit.
/// - `git_commit_author_name`: Author's name of the commit.
/// - `git_commit_count`: Number of commits in the Git repository.
/// - `git_commit_date`: Date of the last commit.
/// - `git_commit_message`: Message of the last commit.
/// - `git_commit_timestamp`: Timestamp of the last commit.
/// - `git_describe`: Description string from Git.
/// - `git_dirty`: Indicates if there are uncommitted changes.
/// - `git_sha`: SHA-1 hash of the latest commit.
/// - `rustc_channel`: Rust compiler release channel used.
/// - `rustc_commit_date`: Commit date of the Rust compiler used.
/// - `rustc_commit_hash`: Commit hash of the Rust compiler used.
/// - `rustc_host_triple`: Host triple for the Rust compiler.
/// - `rustc_llvm_version`: LLVM version used by the Rust compiler.
/// - `rustc_semver`: Semantic versioning of the Rust compiler.
/// - `sysinfo_cpu_brand`: Brand of the CPU.
/// - `sysinfo_cpu_core_count`: Number of CPU cores.
/// - `sysinfo_cpu_frequency`: Frequency of the CPU.
/// - `sysinfo_cpu_name`: Name of the CPU.
/// - `sysinfo_cpu_vendor`: Vendor of the CPU.
/// - `sysinfo_memory`: Amount of system memory.
/// - `sysinfo_name`: Name of the operating system.
/// - `sysinfo_os_version`: Version of the operating system.
/// - `sysinfo_user`: Current user executing the build.
///
/// Some fields may be marked as "Unknown" if the information is not available.
pub fn get_full_version_info() -> serde_json::Value {
    json!({
        "build_date": env!("VERGEN_BUILD_DATE"),
        "build_timestamp": env!("VERGEN_BUILD_TIMESTAMP"),
        "cargo_debug": option_env!("VERGEN_CARGO_DEBUG").unwrap_or("Unknown"),
        "cargo_dependencies": option_env!("VERGEN_CARGO_DEPENDENCIES").unwrap_or("Unknown"),
        "cargo_features": option_env!("VERGEN_CARGO_FEATURES").unwrap_or("Unknown"),
        "cargo_opt_level": option_env!("VERGEN_CARGO_OPT_LEVEL").unwrap_or("Unknown"),
        "cargo_target_triple": option_env!("VERGEN_CARGO_TARGET_TRIPLE").unwrap_or("Unknown"),
        "git_branch": option_env!("VERGEN_GIT_BRANCH").unwrap_or("Unknown"),
        "git_commit_author_email": option_env!("VERGEN_GIT_COMMIT_AUTHOR_EMAIL").unwrap_or("Unknown"),
        "git_commit_author_name": option_env!("VERGEN_GIT_COMMIT_AUTHOR_NAME").unwrap_or("Unknown"),
        "git_commit_count": option_env!("VERGEN_GIT_COMMIT_COUNT").unwrap_or("Unknown"),
        "git_commit_date": env!("VERGEN_GIT_COMMIT_DATE"),
        "git_commit_message": option_env!("VERGEN_GIT_COMMIT_MESSAGE").unwrap_or("Unknown"),
        "git_commit_timestamp": option_env!("VERGEN_GIT_COMMIT_TIMESTAMP").unwrap_or("Unknown"),
        "git_describe": env!("VERGEN_GIT_DESCRIBE"),
        "git_dirty": option_env!("VERGEN_GIT_DIRTY").unwrap_or("Unknown"),
        "git_sha": env!("VERGEN_GIT_SHA"),
        "rustc_channel": option_env!("VERGEN_RUSTC_CHANNEL").unwrap_or("Unknown"),
        "rustc_commit_date": option_env!("VERGEN_RUSTC_COMMIT_DATE").unwrap_or("Unknown"),
        "rustc_commit_hash": option_env!("VERGEN_RUSTC_COMMIT_HASH").unwrap_or("Unknown"),
        "rustc_host_triple": option_env!("VERGEN_RUSTC_HOST_TRIPLE").unwrap_or("Unknown"),
        "rustc_llvm_version": option_env!("VERGEN_RUSTC_LLVM_VERSION").unwrap_or("Unknown"),
        "rustc_semver": option_env!("VERGEN_RUSTC_SEMVER").unwrap_or("Unknown"),
        "sysinfo_cpu_brand": option_env!("VERGEN_SYSINFO_CPU_BRAND").unwrap_or("Unknown"),
        "sysinfo_cpu_core_count": option_env!("VERGEN_SYSINFO_CPU_CORE_COUNT").unwrap_or("Unknown"),
        "sysinfo_cpu_frequency": option_env!("VERGEN_SYSINFO_CPU_FREQUENCY").unwrap_or("Unknown"),
        "sysinfo_cpu_name": option_env!("VERGEN_SYSINFO_CPU_NAME").unwrap_or("Unknown"),
        "sysinfo_cpu_vendor": option_env!("VERGEN_SYSINFO_CPU_VENDOR").unwrap_or("Unknown"),
        "sysinfo_memory": option_env!("VERGEN_SYSINFO_MEMORY").unwrap_or("Unknown"),
        "sysinfo_name": option_env!("VERGEN_SYSINFO_NAME").unwrap_or("Unknown"),
        "sysinfo_os_version": env!("VERGEN_SYSINFO_OS_VERSION"),
        "sysinfo_user": option_env!("VERGEN_SYSINFO_USER").unwrap_or("Unknown")
    })
}

pub fn run(output: String, pretty: bool) -> Result<()> {
    let version_info = get_version();
    let full_version_info = get_full_version_info();

    match output.as_str() {
        "json" => {
            // Use the serialized version info
            let version_info = get_version();
            if pretty {
                println!("{}", serde_json::to_string_pretty(&version_info)?);
            } else {
                println!("{}", serde_json::to_string(&version_info)?);
            }
        }
        "full" => {
            // Output all available version information in JSON
            if pretty {
                println!("{}", serde_json::to_string_pretty(&full_version_info)?);
            } else {
                println!("{}", full_version_info.to_string());
            }
        }
        _ => {
            // Default output: print just the git_describe
            println!("{}", version_info.git_describe);
        }
    }

    Ok(())
}
