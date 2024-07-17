# Introduction
This repository contains rust "helloworld" for ARM64.
The sample code is from following link and I did some minor changes,
  https://lowenware.com/blog/aarch64-bare-metal-program-in-rust/
And
  https://github.com/sammyne/arm64-bare-metal-program-in-rust

## Install Rust Toolchain.
### Install rustup.
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

### Install nightly Toolchain. (cargo, rust-std, rustc, etc).
> rustup default nightly
### Install rust-src
> rustup component add rust-src

### Install Qemu for ARM
> sudo apt install qemu-system-arm

## Build
The --release is needed as the debug version will add some section before .text.boot which make the code break.
> cargo build --release

## Run with Qemu
### Run "run.sh"
> /run.sh
### Or Qemu Command
> qemu-system-aarch64 -machine virt \
  -m 1024M \
  -cpu cortex-a53 \
  -nographic \
  -kernel target/aarch64-unknown-none/release/aarch64-bare-metal

### Output
  String "AArch64 Bare Metal" will appear in the console.
