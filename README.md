# Rust for R developers @ Cascadia R Conference 2025


## Setting up

Before we can start, we need to get our house in order. We need to install:

- [ ] Positron
- [ ] Rust
- [ ] [Rust analyzer VS Code extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [ ] [Even Better TOML VS Code extension](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)


## Install Positron

Download the appropriate Positron installer from the [downloads page](https://positron.posit.co/download.html).

Open the extensions pane (or press `shift + cmd + x`).

- search for `rust analyzer` and install
- search for `even better toml` and install

## Install rust

To install Rust, please use [rustup](https://rustup.rs/). If you use a system installation via brew, apt, dnf, etc you will likely run into issues. I will be able to help debug these.

For installing Rust on Mac / Unix / Linux please run:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

If using Windows download the appropriate installer. Then, once your installation is complete, from your command prompt run:

```shell
rustup target add x86_64-pc-windows-gnu
```

This is a compilation target that is required for building extendr packages on Windows.
