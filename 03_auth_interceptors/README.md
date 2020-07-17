# Authentication with Interceptors

## Concept

To demonstrate a simple token based authentication using gRPC.

Service definitions can be found in [./protos/auth.proto](./protos/auth.proto)

In this example we are passing static credentials as metadata attached to the client call.

We intercept this call on the server side and perform crude validation that this is a value
we expect and return the secret message.

We then try making a call without the expected credentials and return an error.

We are using an `Interceptor` on the server in order to do this, by registering the Interceptor
we are able to intercept the call to `request_response` and either progress to the code call, or
raise an error status and abort the call.

## Instructions

* Run the server in one terminal:

```shell
cargo run --bin auth-server
```

* Run the client in another terminal:

```shell
cargo run --bin auth-client
```

* Receive a response and celebrate

