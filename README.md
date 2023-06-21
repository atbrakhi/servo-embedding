# Embedding Servo

This repository showcases the basic setup for an `Hello World` equivalent program to embed, build, and run Servo.

```
Note: Since I am using `libsimpleservo` in this repository, for the following steps, I assume that you already have `Servo project` built locally. If not, please check out the [ Servo repository](https://github.com/servo/servo) for instructions on how to set it up locally.
```

## Steps:
1. Clone this project
3. Delete the `cargo.lock` file and the `rust-toolchain` file in your local `servo-embedding` project.
3. Copy the `cargo.lock` file and the `rust-toolchain` file from the `Servo repository` and paste them into your local `servo-embedding` project

## Building and running locally:

To build:
```
cargo build --release
```

To run:
```
cargo run --release
```