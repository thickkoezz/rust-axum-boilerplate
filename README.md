# Rust Application Server Boilerplate using Axum framework and MongoDB! 🦀

This project is a boilerplate for building a Rust application server using the [Axum framework]("https://github.com/tokio-rs/axum") and MongoDB as the database. It provides a solid starting point for building your own Rust applications, with many common features already implemented.

## Features

- [x] Axum server: A modern and fast web framework with a focus on ergonomics and modularity.
- [x] MongoDB driver: A Rust driver for MongoDB, allowing you to interact with MongoDB collections.
- [x] Logging: Logging support using `tracing` and `tracing-subscriber` for async-compatible logging.
- [x] Error handler: Application error handling system.
- [x] Router: A router for mapping requests to handlers, cors, and static files.
- [x] Static: Static file serving using `tower-http`.
- [x] Extractors: Validation extractor for getting data from requests and validate with `validator` crate.
- [x] App config (figment): Load your application's configuration from a `config.toml` file.
- [x] Server Metrics with Prometheus using `axum-prometheus`. If you run in localhost, you can access the dashboard at [localhost:9091](http://localhost:9091).
- [x] Hashing: Password hashing
- [x] JWT utils: Utilities for working with JWTs.
- [x] Authentication: User authentication system. Uses both access token and refresh token to avoid unnecessary user re-login and refresh the access token when it is expired.

## Possible Planned Features
- [ ] Tests: Add tests for the application.

## Project Structure

The project is organized into several crates:

- `database`: Contains the MongoDB driver and user model and repository.
- `server`: Contains the main application server, including the API, router, and services.
- `utils`: Contains utility modules like config and errors.

## Getting Started

1. Clone the repository.
2. Install the Rust toolchain if you haven't already.
3. Run `cargo build` to build the project.
4. Run `cargo run` to start the server.

You can install cargo-watch to automatically recompile the project when changes are made:

```bash
cargo install cargo-watch
```

Then run `cargo watch -x run` to start the server.

## Contributing

Contributions are welcome!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
