# `myfirstclap`: Testing Rust, Clap & Vergen

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[![Licence](https://img.shields.io/github/license/Ileriayo/markdown-badges?style=for-the-badge)](./LICENSE)
[![goreleaser](https://github.com/nicop311/myfirstclap/actions/workflows/release.yml/badge.svg)](https://github.com/nicop311/myfirstclap/actions/workflows/release.yml)
<hr>
<hr>

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

> **NOTE: Tested only on Podman running on x86 Arch linux**

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

#### Use `goreleaser-rust-cross`

> **NOTE: Tested only on Podman running on x86 Arch linux**

Check this project: https://github.com/goreleaser/goreleaser-rust-cross

```bash
podman pull ghcr.io/goreleaser/goreleaser-rust-cross:v2.8.0
```

```bash
podman run -it --rm -v $PWD:/pwd --workdir /pwd ghcr.io/goreleaser/goreleaser-rust-cross:v2.8.0 release --snapshot --clean --skip sign,publish,validate,ko,sbom
```

```
tree dist/
dist/
├── artifacts.json
├── config.yaml
├── linux-builds_x86_64-unknown-linux-gnu
│   └── myfirstclap
├── metadata.json
├── myfirstclap-0.2.1SNAPSHOT_196deb7-1-x86_64.pkg.tar.zst
├── myfirstclap-0.2.1~SNAPSHOT_196deb7-1.x86_64.rpm
├── myfirstclap_0.2.1~SNAPSHOT-196deb7_amd64.deb
├── myfirstclap_0.2.1-SNAPSHOT-196deb7_checksums.txt
├── myfirstclap_0.2.1_SNAPSHOT-196deb7_x86_64.apk
├── myfirstclap_Linux_x86_64.tar.zst
├── myfirstclap_Windows_arm64.zip
├── myfirstclap_Windows_x86_64.zip
├── win-msvc-builds_aarch64-pc-windows-msvc
│   └── myfirstclap.exe
└── win-msvc-builds_x86_64-pc-windows-msvc
    └── myfirstclap.exe
```

## Usage

### Print help message

```bash
myfirstclap 
```

```bash
Root CLI of myfirstclap

Usage: myfirstclap [OPTIONS] <COMMAND>

Commands:
  version     A more detailed version command with information from the build
  serve       A collection of several trivial servers
  completion  Completion scripts for various terminals
  man         NOT WORKING YET Generate a man page for this application
  help        Print this message or the help of the given subcommand(s)

Options:
      --log-level <LOG_LEVEL>  Set the log verbosity level [default: info] [possible values: error, warn, info, debug, trace]
  -h, --help                   Print help
  -V, --version                Print version
```

### Print simple oneliner version information

Both `version` and `-V` use the `git2` crate to extract the version information from the current Git repository, and not
from the cargo version.

The git equivalent to `git2` crate is: 

```bash
git describe --tags --always --dirty
v0.4.0
```

Which gives this in the CLI:

```bash
root@0057b779a97b:/# myfirstclap version                      
v0.4.0
```

This is equivalent to:

```bash
root@0057b779a97b:/# myfirstclap -V      
myfirstclap v0.4.0
```

### Print detailed version information in JSON format

This is using `vergen-git2` from https://github.com/rustyhorde/vergen, and also
using the regular `git2` crate (for the `git describe --tags --always --dirty`)

```bash
myfirstclap version --output json
```

or 

```bash
myfirstclap version -o json
```

```json
{
  "build_date": "2025-03-26",
  "build_timestamp": "2025-03-26T11:56:46.924681571Z",
  "git_commit_date": "2025-03-26",
  "git_describe": "65c7711",
  "git_describe_tags": "v0.4.0",
  "git_sha": "65c7711dc5785b874de7352688fe0dd7f608d4ad",
  "os_version": "Linux (Ubuntu 24.04)"
}
```

> Note: Notice both `git_describe` from vergen and `git_describe_tags` from git2. For some reasons,
> vergen hardcode the `describe` options `tags` and `dirty` to false:
> https://github.com/rustyhorde/vergen/blob/50bebaf2546930c9b011e2efb2b0449d522d8506/vergen-git2/src/git2/mod.rs#L214
> Therefore, I use the `git2` crate.
> Maybe it is worth asking the vergen maintainers to make these options configurable.

This `version` command could became a crate from clap, like clap-version, which could give the same
result.

### Print very detailed version information in JSON format (full vergen)

This is probably overkil. This command is only here to see the full features from vergen.
I might add some of these fields to the regular `version -o json` command.

> Note: Notice both `git_describe` from vergen and `git_describe_tags` from git2.
> It is like the regular `version -o json` command.

```bash
myfirstclap version --output full | jq
```

```json
myfirstclap version --output full
{
  "build_date": "2025-03-26",
  "build_timestamp": "2025-03-26T11:56:46.924681571Z",
  "cargo_debug": "false",
  "cargo_dependencies": "anyhow 1.0.97,git2 0.20.1,serde 1.0.219,serde_json 1.0.140,vergen-git2 1.0.5",
  "cargo_features": "",
  "cargo_opt_level": "3",
  "cargo_target_triple": "x86_64-unknown-linux-gnu",
  "git_branch": "HEAD",
  "git_commit_author_email": "18116277+nicop311@users.noreply.github.com",
  "git_commit_author_name": "nicop311",
  "git_commit_count": "29",
  "git_commit_date": "2025-03-26",
  "git_commit_message": "remove the log level init since this is done in the root CLISigned-off-by: nicop311 <18116277+nicop311@users.noreply.github.com>",
  "git_commit_timestamp": "2025-03-26T11:48:58.000000000Z",
  "git_describe": "65c7711",
  "git_describe_tags": "v0.4.0",
  "git_dirty": "false",
  "git_sha": "65c7711dc5785b874de7352688fe0dd7f608d4ad",
  "rustc_channel": "stable",
  "rustc_commit_date": "2025-02-17",
  "rustc_commit_hash": "4d91de4e48198da2e33413efdcd9cd2cc0c46688",
  "rustc_host_triple": "x86_64-unknown-linux-gnu",
  "rustc_llvm_version": "19.1",
  "rustc_semver": "1.85.0",
  "sysinfo_cpu_brand": "AMD EPYC 7763 64-Core Processor",
  "sysinfo_cpu_core_count": "2",
  "sysinfo_cpu_frequency": "2445",
  "sysinfo_cpu_name": "cpu0,cpu1,cpu2,cpu3",
  "sysinfo_cpu_vendor": "AuthenticAMD",
  "sysinfo_memory": "Unknown",
  "sysinfo_name": "Ubuntu",
  "sysinfo_os_version": "Linux (Ubuntu 24.04)",
  "sysinfo_user": "runner"
}
```

### Log level

The root CLI support setting the log level.

```bash
myfirstclap --log-level=trace serve hello
```

```log
 2025-03-26T11:59:37.709Z INFO  serve::hello > Listening on http://127.0.0.1:3000
 2025-03-26T11:59:37.709Z INFO  serve::hello > such information
 2025-03-26T11:59:37.709Z DEBUG serve::hello > This is a debug message
 2025-03-26T11:59:37.709Z TRACE serve::hello > This trace will be hard to wash
 2025-03-26T11:59:37.709Z WARN  serve::hello > o_O
 2025-03-26T11:59:37.709Z ERROR serve::hello > much error
```

### Build terminal completion scripts

```bash
myfirstclap completion -h
```
```
Completion scripts for various terminals

Usage: myfirstclap completion [OPTIONS]

Options:
  -s, --shell <SHELL>  Target shell. [default: bash] [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help           Print help
```

#### Example with fish

```bash
myfirstclap completion -s fish > ~/.config/fish/completions/myfirstclap.fish
```