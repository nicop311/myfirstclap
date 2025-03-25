use anyhow::Result;
use vergen_git2::{
    BuildBuilder, CargoBuilder, Emitter, Git2Builder, RustcBuilder, SysinfoBuilder,
};
use git2::{DescribeOptions, DescribeFormatOptions, Repository};

fn main() -> Result<()> {
    // Step 1: Get Git Describe information using git2
    let repo = Repository::discover(".")?;
    let mut describe_opts = DescribeOptions::new();
    describe_opts
        .describe_tags()                         // Equivalent to --tags
        .show_commit_oid_as_fallback(true);      // Equivalent to --always

    let describe = repo.describe(&describe_opts)?;

    // Configure format options (e.g., dirty suffix)
    let mut format_opts = DescribeFormatOptions::new();
    format_opts
        .dirty_suffix("-dirty")                  // Equivalent to --dirty
        .abbreviated_size(7);                    // Optional: adjust size like git describe

    let describe_result = describe.format(Some(&format_opts))?;

    // Step 2: Set the describe result as an environment variable for access during build
    println!("cargo:rustc-env=GIT_DESCRIBE_TAGS={}", describe_result);

    // Step 3: Emit other build information using vergen
    // vergen hardcode describe options tags and dirty to false
    Emitter::default()
        .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(&CargoBuilder::all_cargo()?)?
        .add_instructions(&Git2Builder::all_git()?)?
        .add_instructions(&RustcBuilder::all_rustc()?)?
        .add_instructions(&SysinfoBuilder::all_sysinfo()?)?
        .emit()?;

    Ok(())
}
