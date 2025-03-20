# `myfirstclap`: Testing Rust, Clap & Vergen

This repository contains a sample Rust project that demonstrates the usage of the Clap and Vergen crates for command-line argument parsing and build-time information generation, respectively.

The project showcases how to define and parse command-line arguments using Clap, and how to embed build-time information such as the current git commit hash and build timestamp using Vergen.

**The only purpose of this project is training and learning.**

## Build the project

### Build With Cargo

> Note: If you do not have Rust installed, use this podman command to start a rust container to 
> build from inside the container.
> 
> ```bash
> podman run -it -v $PWD:/pwd --workdir /pwd docker.io/library/rust:bookworm
> ```

```
rustc --version 
rustc 1.85.0 (4d91de4e4 2025-02-17) (Arch Linux rust 1:1.85.0-1)
```

```
cargo build --workspace
```

### Build With `goreleaser`

> NOTE: This feature is not ready yet.

See the Goreleaser Rust documentation here:
* https://goreleaser.com/customization/builds/rust/
* https://github.com/goreleaser/example-rust/tree/main

#### You do not have goreleaser installed ? Use this method to create a temp container

If you do not have Rust or Goreleaser installed, use this podman command to start a rust container
to build from inside the container. Then install Goreleaser inside the container.

```bash
podman run -it -v $PWD:/pwd --workdir /pwd docker.io/library/rust:bookworm
```

Install Goreleaser inside the container: read this https://goreleaser.com/install/#apt-repository
```bash
apt update
echo 'deb [trusted=yes] https://repo.goreleaser.com/apt/ /' | tee /etc/apt/sources.list.d/goreleaser.list
apt update
apt install goreleaser
```

Then build the project using Goreleaser:

```bash
goreleaser release --snapshot --clean --skip sign,publish,validate,ko,sbom
```

## Usage

### Print help

```bash
 ./target/debug/myfirstclap 
```

```bash
Usage: myfirstclap <COMMAND>

Commands:
  version  
  serve    
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Print simple oneliner version information

```bash
./target/debug/myfirstclap version
4a9f71d
```

### Print detailed version information in JSON format

```bash
./target/debug/myfirstclap version --output json
```
or 
```bash
./target/debug/myfirstclap version -o json
```

```json
{
  "build_date": "2025-03-21",
  "build_timestamp": "2025-03-21T09:14:47.122011316Z",
  "git_commit_date": "2025-03-20",
  "git_describe": "4a9f71d",
  "git_sha": "4a9f71d19be9aecc01dfe39f58d0675c86289597",
  "os_version": "Linux (Debian GNU/Linux 12)"
}
```