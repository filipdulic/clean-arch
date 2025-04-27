# Clean Architecture Rust Demo
## Overview
This is a demo project exploring Clean Architecture, Domain-Driven Design, and
general Rust development.

## Todo:
### Documentation
- [ ] Code
- [ ] Wiki
### Design
- [x] Typestate state machine
- [x] Database transactions
- [ ] Outbox pattern - for publishing messages
### Authentication
- [x] JWT Token claim pack/extract
### Authorization
- [x] Role Based Access
- [x] Object Owner Access
### Interfaces
- [ ] Terminal
    - [x] CLI (string) server
    - [ ] TUI/ncurses client
    - [ ] Desktop client
- [ ] Web
    - [ ] Actix server
    - [x] poem-openapi server
    - [ ] Axum server
    - [ ] Yew frontend
    - [ ] Seed frontend
- [ ] WebSocket
- [ ] gRPC
### Databases
- [x] SQLite/SQLX
- [ ] InMemory (HashMap)
- [ ] PostgreSQL/Diesel
- [ ] LMDB/Heed
- [ ] Wide-column DB/DynamoDB
### Message Brokers/Queues
- [ ] RabbitMQ (RMQ)
- [ ] Kafka
- [ ] ZeroMQ (ZMQ)
## Note
Heavily influenced by [clean-architecture-with-rust](https://github.com/flosse/clean-architecture-with-rust) project.
