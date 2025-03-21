// This is using the CLI workspace tool crate
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    cli::run()
}
