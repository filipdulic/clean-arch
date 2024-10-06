//! # Rust Clean Architecture
//!
//! This is a simple example of a Rust project using the Clean Architecture pattern.
//! The project is a simple user signup application that allows users to sign up for an account
//! and complete the signup process.

/// The adapter module contains the implementation of the adapters that connect the application to the outside world.
pub mod adapter;
/// The application module contains the use cases of the application.
pub mod application;
/// The cli module contains the command-line interface for the application.
pub mod cli;
/// The db module contains the database implementation.
pub mod db;
/// The domain module contains the domain entities and business rules.
pub mod domain;
/// The infrastructure module contains the implementation of the infrastructure components.
pub mod infrastructure;
