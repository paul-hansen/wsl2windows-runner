# wsl2windows-runner

A [cargo target runner](https://nexte.st/book/target-runners.html) for use on Windows to build in WSL2 and run the output .exe in Windows.

Once it's setup you can just run `cargo run` while in WSL. It will build in WSL2 as usual but then run in Windows.

[Get started](#first-time-setup)

## Why?
Rust compilation on Windows is slower than on linux even when building exe files with the msvc toolchain, 
and it's a pain (impossible?) to get some graphical applications like [Bevy](https://bevyengine.org) to run in WSL2. 
This allows you to have some of the build speed improvements of building on linux while still being able to have the output .exe run in Windows automatically. Works great with RustRover/Clion (or other IDEs that support using Rust with WSL and expect to be able to use cargo run).

## Performance comparison

These numbers will vary greatly and should only be used to get a rough idea of the performance gain you _might_ get. The numbers varied greatly even between runs on the same machine in some cases. I recommend comparing with your own setup to make sure it's benefiting you with your setup and project.

### Clean Build

```shell
cargo clean
cargo run --features bevy/dynamic_linking
```

| Windows | WSL    |
|---------|--------|
| 2m 35s  | 2m 10s |

### Incremental Build

Made a single line change between each run.

```shell
cargo run --features bevy/dynamic_linking
```

| Windows | WSL   |
|---------|-------|
| 5.71s   | 4.05s |

### Clippy

Made a single line change between each run.

```shell
cargo clippy --features bevy/dynamic_linking
```

| Windows | WSL   |
|---------|-------|
| 2.96s   | 2.69s |

#### Test machine
- AMD Ryzen 9 7950X (32 threads 5.20 GHz all cores)
- 64 GB DDR5
- 4 TB NVME SSD 4900/3500 MB/s

#### Test project
- [Bevy](https://bevyengine.org) 0.11.0
- 15,131 lines of Rust in the project 
- Installed in WSL for WSL tests e.g. `\\wsl.localhost\Ubuntu\home\paul\project\`
- Installed in Windows for Windows tests e.g. `C:\Users\paul\project\`
- Exclusions for the project directory in Windows Defender for both projects
- Closed source sorry

# First Time Setup

## Install WSL2 and Rust

```shell
wsl --install
wsl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add x86_64-pc-windows-msvc
```

## Install wsl2windows-runner

While still in wsl:

```shell
cargo install --git https://github.com/paul-hansen/wsl2windows-runner.git
```

## Install Windows SDK for MSVC inside WSL2 

See https://bevy-cheatbook.github.io/setup/cross/linux-windows.html for less condensed instructions.

Replace `/home/me/.xwin` in the next command with the path you want to install the SDK to. Remember this path for the next step.
```shell
cargo install xwin
xwin --accept-license splat --output /home/me/.xwin
```

Add this to your project's `.cargo/config.toml` (or create it) replacing `/home/me/.xwin` with the path you installed to in the previous step.
```toml
[build]
target = "x86_64-pc-windows-msvc"

[target.x86_64-pc-windows-msvc]
runner = "wsl2windows-runner"
linker = "rust-lld"
rustflags = [
    "-Lnative=/home/me/.xwin/crt/lib/x86_64",
    "-Lnative=/home/me/.xwin/sdk/lib/um/x86_64",
    "-Lnative=/home/me/.xwin/sdk/lib/ucrt/x86_64"
]
```

I recommend adding `./cargo/config.toml` to your .gitignore file if you are collaborating as not everyone will want to use this.

## Other/additional ways to reduce compile times

If you are looking at this project these resources may be useful to you as well:
- https://davidlattimore.github.io/working-on-rust-iteration-time.html
- https://benw.is/posts/how-i-improved-my-rust-compile-times-by-seventy-five-percent
- https://matklad.github.io/2021/09/04/fast-rust-builds.html

## Alternatives

- [cross](https://github.com/cross-rs/cross) Building in docker containers. Didn't fit my needs because it uses `cross run` instead of `cargo run` and Clion doesn't play well with that.
