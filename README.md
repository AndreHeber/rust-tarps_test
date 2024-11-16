# Tarpc Example

This project demonstrates a simple client-server application using the `tarpc` library for RPC (Remote Procedure Call) in Rust. The server provides a service that returns a greeting message, and the client sends requests to the server to receive these greetings.

## Dependencies

The project uses the following dependencies:

- `anyhow`: For error handling.
- `clap`: For command-line argument parsing.
- `futures`: For asynchronous programming.
- `log`: For logging.
- `opentelemetry`: For OpenTelemetry tracing.
- `opentelemetry-jaeger`: For Jaeger backend support in OpenTelemetry.
- `rand`: For random number generation.
- `tarpc`: For RPC.
- `tokio`: For asynchronous runtime.
- `tracing`: For structured logging.
- `tracing-opentelemetry`: For integrating tracing with OpenTelemetry.
- `tracing-subscriber`: For subscribing to tracing events.

## Building and Running

To build the project, run:

```sh
cargo build
```

## Running the Server

To run the server, use:

```sh
cargo run --bin server -- --port <PORT>
```

Replace <PORT> with the port number you want the server to listen on.

## Running the Client

To run the client, use:

```sh
cargo run --bin client -- --server_addr <SERVER_ADDR> --name <NAME>
```

Replace <SERVER_ADDR> with the server's address (e.g., 127.0.0.1:8080) and <NAME> with the name you want to send to the server.

## Example

Start the server:

```sh
cargo run --bin server -- --port 8080
```

In another terminal, start the client:

```sh
cargo run --bin client -- --server_addr 127.0.0.1:8080 --name Alice
```

The client will send requests to the server and print the responses.

## License

This project is licensed under the MIT License.