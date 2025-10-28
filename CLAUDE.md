# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based URL shortener service that provides both HTTP API and Telegram bot interfaces. The application uses a PostgreSQL database and includes monitoring with Prometheus metrics.

## Development Commands

### Running the Application
- `make run` - Start development server with hot reload using cargo-watch
- `make compose` or `make docker` - Start PostgreSQL, Redis, and Prometheus using docker-compose
- `cargo run` - Run the application directly

### Database Management
- `make migrations-up` - Apply database migrations to dev database
- `make migrations-down` - Rollback last migration
- `make migrations-status` - Check migration status
- `make migrations-new MIGRATION_NAME=your_migration` - Create new migration file

Database migrations are managed with Goose and located in `migrations/pg/`.

### Building and Deployment
- `cargo build --release` - Build optimized release binary
- `make compose-prod` - Build and run production Docker setup
- Docker builds use multi-stage with musl target for Alpine Linux

## Architecture

### Core Structure
The application follows a layered architecture with clear separation of concerns:

- **main.rs** - Application entry point, starts both HTTP server and Telegram bot concurrently
- **app/** - Core application layer containing config, handlers, repositories, services
- **feature/** - Feature modules (url, auth) with their own handlers, services, repositories, entities
- **servers/http/** - HTTP server setup with Axum framework and middleware
- **metrics/** - Prometheus metrics collection and middleware
- **utils/** - Shared utilities (database, URL validation, etc.)

### Feature Modules
Each feature follows the same pattern:
- `entity.rs` - Database models and domain objects
- `repository.rs` - Data access layer with database operations
- `service.rs` - Business logic layer with trait definitions
- `handler.rs` - HTTP request handlers

### Configuration
- Development config: `src/config/dev/config.toml`
- Production config: `src/config/prod/config.toml`
- Environment-based config loading in `app/config.rs`

### Database
- PostgreSQL with SQLx for async database operations
- Sea-query for type-safe SQL building
- Connection pooling handled by SQLx
- Dev database runs on port 5443, Redis on 6384

### Authentication
- JWT-based authentication with cookie support
- Argon2 password hashing
- Middleware for protected routes

### Monitoring
- Prometheus metrics exposed on `/metrics` endpoint
- Custom metrics for URL shortening, Telegram messages, and errors
- Grafana-compatible metrics format

### Dual Interface
The application serves both:
1. **HTTP API** - RESTful endpoints with Swagger documentation
2. **Telegram Bot** - Interactive bot for URL shortening via messages

Both interfaces share the same underlying services and business logic through dependency injection.

## Testing and Quality

Check for available test commands in the codebase - no standard test runner is configured in Makefile. Use `cargo test` for running Rust tests.

For code quality, check if there are linting tools configured. The project uses standard Rust tooling.

## Environment Setup

Required environment variables:
- `TELOXIDE_TOKEN` - Telegram bot token
- Database credentials configured in TOML files
- Rust development environment with cargo-watch for hot reload