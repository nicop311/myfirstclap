use anyhow::Result;
use vergen_git2::{
    BuildBuilder, CargoBuilder, Emitter, Git2Builder, RustcBuilder, SysinfoBuilder,
};

fn main() -> Result<()> {
    // vergen_git2 builder hardcodes the git describe options --tags and --dirty to false
    // This allow to set options --tags and --dirty to true
    // See https://github.com/rustyhorde/vergen/issues/408#issuecomment-2790357035
    let mut git2_builder = Git2Builder::default();
    _ = git2_builder.all().describe(true, true, None);
    let git2 = git2_builder.build()?;

    // Emit other build information using vergen
    Emitter::default()
        .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(&CargoBuilder::all_cargo()?)?
        .add_instructions(&Git2Builder::all_git()?)?
        .add_instructions(&RustcBuilder::all_rustc()?)?
        .add_instructions(&SysinfoBuilder::all_sysinfo()?)?
        .add_instructions(&git2)? // needed to get git describe --tags --dirty
        .emit()?;

    Ok(())
}
