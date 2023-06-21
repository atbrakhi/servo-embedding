# Embedding servo

This repository shocase the basic setup needed to setup a `hello world` equivalent program to build and run servo.

Note: As I am using `libsimpleservo` in this repository, for next steps I am assuming you already servo running locally. If not please checkout servo [repository](https://github.com/servo/servo) on how to set it up locally.

## Steps:
1. Clone this project
3. Delete `cargo.lock` file and `rust-toolchain` file in your local `servo-embedding` project
3. Copy `cargo.lock` file and `rust-toolchain` file from `servo's repository` and paste it in your local `servo-embedding` project

## Building and running locally:

To build:
```
cargo build --release
```

To run:
```
cargo run --release
```