use anyhow::Result;
use serde::Serialize;
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

pub fn get_describe() -> &'static str {
    env!("VERGEN_GIT_DESCRIBE")
}

pub fn run(output: String) -> Result<()> {
    let version_info = get_version();

    match output.as_str() {
        "json" => {
            // Output the full version info as formatted JSON
            println!(
                "{{
  \"build_date\": \"{}\",
  \"build_timestamp\": \"{}\",
  \"git_commit_date\": \"{}\",
  \"git_describe\": \"{}\",
  \"git_sha\": \"{}\",
  \"os_version\": \"{}\"
}}",
                version_info.build_date,
                version_info.build_timestamp,
                version_info.git_commit_date,
                version_info.git_describe,
                version_info.git_sha,
                version_info.os_version,
            );
        }
        _ => {
            // Default output: print just the git_describe
            println!("{}", version_info.git_describe);
        }
    }

    Ok(())
}
