use anyhow::Result;
use std::process::Command;
use vergen_git2::{
    BuildBuilder, CargoBuilder, Emitter, Git2Builder, RustcBuilder, SysinfoBuilder,
};

fn main() -> Result<()> {
    // Run `git describe --tags --always --dirty` to capture tag information
    let git_describe_tags = Command::new("git")
        .args(["describe", "--tags", "--always", "--dirty"])
        .output();

    match git_describe_tags {
        Ok(output) if output.status.success() => {
            let describe_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("cargo:rustc-env=GIT_DESCRIBE_TAGS={}", describe_str);
        }
        _ => {
            println!("cargo:rustc-env=GIT_DESCRIBE_TAGS=unknown");
        }
    }

    // Use vergen_git2 to emit other build information
    Emitter::default()
        .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(&CargoBuilder::all_cargo()?)?
        .add_instructions(&Git2Builder::all_git()?)?
        .add_instructions(&RustcBuilder::all_rustc()?)?
        .add_instructions(&SysinfoBuilder::all_sysinfo()?)?
        .emit()
}
