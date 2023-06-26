# Embedding Servo

This repository showcases the basic setup for compiling servo as dependency in another rust project.

- Option 1: Using `libsimpleservo`
- Option 2: Using `libservo`

## Option 1: Using `libsimpleservo`

To use `libsimpleservo`, the following steps assumes that you already have `Servo project` built locally. If not, please check out the [Servo repository](https://github.com/servo/servo) for instructions on how to set it up locally.


#### Steps:
1. Clone this project
3. Delete the `cargo.lock` file and the `rust-toolchain` file in your local `servo-embedding` project.
3. Copy the `cargo.lock` file and the `rust-toolchain` file from the `Servo repository` and paste them into your local `servo-embedding` project


## Option 2: Using `libservo`

#### Steps:
1. Clone this project
2. In `Cargo.toml` file, under [dependencies] delete the `simpleservo` line(L9) and add:
    ```
    libservo = {git = "https://github.com/servo/servo", rev="XXXXXXX", features = ["layout-2013", "media-gstreamer"]}
    ```
    Note: you will need to get the latest `rev` from servo [repository](https://github.com/servo/servo)

3. Delete the `cargo.lock` file and the `rust-toolchain` file in your local `servo-embedding` project.
4. Copy the `cargo.lock` file and the `rust-toolchain` file from the `Servo repository` and paste them into your local `servo-embedding` project
5. Replace the code in `main.rs` file with following:

    ```
    extern crate servo;

    fn main() {
        // do something
    }
    ```

## Building and running locally:

To build:
```
cargo build --release
```

To run:
```
cargo run --release
```
