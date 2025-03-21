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

> **NOTE: This feature is not ready yet.**

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

This `version` command could became a crate from clap, like clap-version, which could give the same
result.

### Print very detailed version information in JSON format (full vergen)

This is probably overkil. This command is only here to see the full features from vergen.
I might add some of these fields to the regular `version -o json` command.

```bash
./target/debug/myfirstclap version --output full | jq
```

```json
{
  "build_date": "2025-03-21",
  "build_timestamp": "2025-03-21T13:46:51.609678254Z",
  "cargo_debug": "true",
  "cargo_dependencies": "anyhow 1.0.97,serde 1.0.219,serde_json 1.0.140,vergen-git2 1.0.5",
  "cargo_features": "",
  "cargo_opt_level": "0",
  "cargo_target_triple": "x86_64-unknown-linux-gnu",
  "git_branch": "add-flags",
  "git_commit_author_email": "18116277+nicop311@users.noreply.github.com",
  "git_commit_author_name": "nicop311",
  "git_commit_count": "5",
  "git_commit_date": "2025-03-21",
  "git_commit_message": "remove main from root project dir and change the name of the binary produced by the cli crate to use the name of the project insteadSigned-off-by: nicop311 <18116277+nicop311@users.noreply.github.com>",
  "git_commit_timestamp": "2025-03-21T13:43:50.000000000Z",
  "git_describe": "04c8644",
  "git_dirty": "false",
  "git_sha": "04c86446f921d24ae6a10988dd807676d2cb94dc",
  "rustc_channel": "stable",
  "rustc_commit_date": "2025-02-17",
  "rustc_commit_hash": "4d91de4e48198da2e33413efdcd9cd2cc0c46688",
  "rustc_host_triple": "x86_64-unknown-linux-gnu",
  "rustc_llvm_version": "19.1",
  "rustc_semver": "1.85.0",
  "sysinfo_cpu_brand": "Intel(R) Core(TM) Ultra 7 155U",
  "sysinfo_cpu_core_count": "12",
  "sysinfo_cpu_frequency": "1700",
  "sysinfo_cpu_name": "cpu0,cpu1,cpu2,cpu3,cpu4,cpu5,cpu6,cpu7,cpu8,cpu9,cpu10,cpu11,cpu12,cpu13",
  "sysinfo_cpu_vendor": "GenuineIntel",
  "sysinfo_memory": "Unknown",
  "sysinfo_name": "Debian GNU/Linux",
  "sysinfo_os_version": "Linux (Debian GNU/Linux 12)",
  "sysinfo_user": "root"
}
```

### Log level

The root CLI support setting the log level.

```bash
./target/debug/myfirstclap --log-level=trace  serve hello
```

```log
Listening on http://127.0.0.1:3000
 2025-03-21T13:53:36.571Z INFO  serve::hello > such information
 2025-03-21T13:53:36.571Z DEBUG serve::hello > This is a debug message
 2025-03-21T13:53:36.571Z TRACE serve::hello > This trace will be hard to wash
 2025-03-21T13:53:36.571Z WARN  serve::hello > o_O
 2025-03-21T13:53:36.571Z ERROR serve::hello > much error
```

### Build terminal completion scripts

```
 myfirstclap completion -h
Completion scripts for various terminals

Usage: myfirstclap completion [OPTIONS]

Options:
  -s, --shell <SHELL>  Supported values: bash. [default: bash] [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help           Print help
```

Example with fish

```bash
myfirstclap completion -s fish > ~/.config/fish/completions/myfirstclap.fish
```