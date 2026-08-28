# Clean Architecture Rust Demo

This project explores Clean Architecture, Domain-Driven Design, and Rust. It
models a user signup flow with typestate, authorization, and persisted state
history. The same use cases are exposed through CLI and Poem OpenAPI interfaces.

## Structure

- `domain` contains entities, value objects, and signup state transitions.
- `application` contains use cases and gateway traits.
- `adapter` coordinates input, authorization, execution, and presentation.
- `infrastructure` provides CLI, HTTP, JWT, file email, and SQLite adapters.
- `src/bin` assembles complete applications from those components.

## Requirements

Rust 1.94 or newer is required.

## Run

Start the HTTP API and Swagger UI:

```console
cargo run
```

Run the CLI with an isolated data directory:

```console
cargo run --bin clean-arch-cli-sqlx-sqlite -- --data-dir ./data --help
```

## Roadmap

### Documentation

- [ ] Code
- [ ] Wiki

### Design

- [x] Typestate state machine
- [x] Database transactions
- [ ] Outbox pattern

### Authentication and authorization

- [x] JWT claim packing and extraction
- [x] Role-based access
- [x] Object-owner access

### Interfaces

- [x] CLI with string boundaries
- [x] Poem OpenAPI server
- [ ] TUI client
- [ ] Desktop client
- [ ] Actix server
- [ ] Axum server
- [ ] Web frontend
- [ ] WebSocket
- [ ] gRPC

### Databases

- [x] SQLite with SQLx
- [ ] In-memory HashMap
- [ ] PostgreSQL with Diesel
- [ ] LMDB with Heed
- [ ] DynamoDB

### Message brokers

- [ ] RabbitMQ
- [ ] Kafka
- [ ] ZeroMQ

The project was influenced by
[clean-architecture-with-rust](https://github.com/flosse/clean-architecture-with-rust).
