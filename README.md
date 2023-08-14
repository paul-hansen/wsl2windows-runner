# wsl2windows-runner

A cargo target runner for use on Windows to build in WSL2 and run the output .exe in Windows.

Once it's setup you can just run `cargo run` while in WSL. It will build in WSL2 as usual but then run in Windows.

[Get started](#first-time-setup)

## Why?
Rust compilation on Windows is slower than on linux even when building exe files with the msvc toolchain, 
and it's a pain (impossible?) to get some graphical applications like [Bevy](https://bevyengine.org) to run in WSL2. 
This allows you to have some of the build speed improvements of building on linux while still being able to have the output .exe run in Windows automatically. This works great with Clion.

## Performance comparison

These numbers will vary greatly and should only be used to get a rough idea of the performance gain you _might_ get.

### Clean Build

```shell
cargo clean
cargo run --features bevy/dynamic_linking
```

| Windows | WSL    |
|---------|--------|
| 2m 35s  | 2m 10s |
| 2m 32s  | 1m 56s |

### Incremental Build

Made a single line change between each run.

```shell
cargo run --features bevy/dynamic_linking
```

| Windows | WSL   |
|---------|-------|
| 5.65s   | 4.08s |
| 5.71s   | 4.05s |
| 5.69s   | 3.93s |

### Clippy

Made a single line change between each run.

```shell
cargo clippy --features bevy/dynamic_linking
```

| Windows | WSL   |
|---------|-------|
| 3.89s   | 2.79s |
| 2.98s   | 3.33s |
| 2.96s   | 2.69s |
| 3.06s   | 2.64s |

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

## Install WSL2 and install Rust in it

```shell
wsl --install
wsl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add x86_64-pc-windows-msvc
```

## Install wsl2windows-runner

While still in wsl:

```shell
git clone https://gitlab.com/paul-hansen/wsl2windows-runner.git
cd wsl2windows-runner
cargo install --path .
```

## Install Windows SDK for MSVC inside WSL2 

See https://bevy-cheatbook.github.io/setup/cross/linux-windows.html for more about this.

Replace /home/me/.xwin with the path you want to install the SDK to. Remember this path for the next step.
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