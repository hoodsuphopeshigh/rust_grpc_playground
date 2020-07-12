# Hello World

## Concept

To demonstrate a simple `Hello, World` using gRPC.

Service definitions can be found in [./protos/helloworld.proto](./protos/helloworld.proto)

We use a [build.rs](./build.rs) to automagically create our gRPC server and client stubs during
code compilation.

We define binary targets in the [Cargo](./Cargo.toml) to allow us to run the client and the server
independently.

## Instructions

* Run the server in one terminal:

```shell
cargo run --bin helloworld-server
```

* Run the client in another terminal:

```shell
cargo run --bin helloworld-client
```

* Receive a response and celebrate

