# wsl2windows-runner

A cargo target runner for use on Windows to build in WSL2 and run the output .exe in Windows.


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
cargo install wsl2windows-runner --path .
```

## Install Windows SDK for MSVC inside WSL2 

See https://bevy-cheatbook.github.io/setup/cross/linux-windows.html for more about this.

Replace /home/me/.xwin with the path you want to install the SDK to. Remember this path for the next step.
```shell
cargo install xwin
xwin --accept-license splat --output /home/me/.xwin
```

Add this to your .cargo/config.toml replacing /home/me/.xwin with the path you installed the SDK to.
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