# Hello Cargo!

By running the command

```sh
cargo new hello_cargo
```

We get a new directory called `hello_cargo` with the following contents:

```sh
hello_cargo
├── Cargo.toml
└── src
    └── main.rs
```

The `Cargo.toml` file is the manifest that describes the project. The `src` directory contains the Rust code. The `main.rs` file is the entry point for the program.

## Building and Running a Cargo Project

To build and run the project, we can use the `cargo run` command:

```sh
cd hello_cargo
cargo run
```

This will compile the project and then run the resulting binary. The output should be `Hello, world!`. The first time we run `cargo run`, it will download and compile the Rust standard library, which may take a few minutes.

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/hello_cargo`
Hello, world!
```

We can also use the `cargo build` command to compile the project without running it. This will create a binary in the `target/debug` directory.

```sh
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
```

To run the binary, we can use the `./target/debug/hello_cargo` command.

```sh
$ ./target/debug/hello_cargo
Hello, world!
```

When we're ready to release the project, we can use the `cargo build --release` command to compile the project with optimizations. This will create a binary in the `target/release` directory.

```sh
$ cargo build --release
   Compiling hello_cargo v0.1.0 (<PWD>/hello_cargo)
    Finished release [optimized] target(s) in 0.40s
```

## Checking the Code with `cargo check`

The `cargo check` command is a useful tool to check if the code compiles without producing an executable. This is faster than `cargo build` because it skips the code generation step.

```sh
$ cargo check
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
```
