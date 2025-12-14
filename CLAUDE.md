# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust SDK for the Ray Dashboard REST API, providing a client to interact with Ray job submission and management endpoints. The project is in active development and currently focuses on the Jobs API, with plans to expand to other Ray Dashboard APIs (like Serve).

## Build & Test Commands

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Lint with clippy
cargo clippy -- -D warnings

# Build
cargo build

# Run unit tests only (tests in src/ modules)
cargo test --lib

# Run integration tests (requires Ray cluster - see below)
cargo test --test '*'

# Run specific integration test file
cargo test --test jobs

# Start local Ray cluster for integration tests (REQUIRED before running integration tests)
docker compose up -d

# Stop Ray cluster
docker compose down
```

## Architecture

### Client Design Pattern

The codebase uses a **trait-based API pattern** where:
- `RayDashboardClient` is the main HTTP client struct that manages the base URL and reqwest client
- API functionality is implemented as **traits** (e.g., `JobSubmissionAPI`)
- The client implements these traits to provide the actual API methods
- This allows for clean separation of concerns and easy mocking/testing

Key implementation detail in [src/client/mod.rs:38](src/client/mod.rs#L38): Ray Dashboard requires a `User-Agent` header on all requests or it returns 500 errors. The SDK sets this automatically via `SDK_USER_AGENT` constant.

### Module Structure

```
src/
├── client/          # API client implementations
│   ├── mod.rs      # Base RayDashboardClient struct with common request building
│   └── jobs.rs     # JobSubmissionAPI trait and implementation
├── schemas/         # Request/response types for API endpoints
│   ├── common.rs   # Shared schemas (e.g., version info)
│   ├── jobs.rs     # Job-related schemas with builder pattern for JobSubmitRequest
│   └── env.rs      # Runtime environment schemas
├── error.rs         # Error types using thiserror
├── constants.rs     # SDK constants (User-Agent, etc.)
└── lib.rs          # Public exports
```

### Schema Design

Schemas use serde for JSON serialization with:
- Builder pattern for request types (see `JobSubmitRequest::new()` and `with_*` methods in [src/schemas/jobs.rs](src/schemas/jobs.rs))
- Optional fields use `#[serde(skip_serializing_if = "Option::is_none")]` to omit from JSON when None
- Enums like `JobStatus` have helper methods (e.g., `is_terminal()`) for common checks

### Testing Strategy

- **Unit tests**: Embedded in schema modules (e.g., `src/schemas/jobs.rs`) to test serialization, builders, and helpers
- **Integration tests**: In `tests/` directory, require a running Ray cluster at `http://127.0.0.1:8265`
  - **IMPORTANT**: You must manually start the Ray cluster with `docker compose up -d` before running integration tests
  - The tests will fail if the Ray cluster is not running
  - Use `docker compose down` to clean up after testing
- Test utilities in `tests/common/mod.rs` define shared constants like `RAY_DASHBOARD_URL`
- Integration tests use `uuid::v4()` to generate unique submission IDs to avoid conflicts

## Development Conventions

### Code Style

- Format with `rustfmt` using project settings: max line width 110, field init shorthand enabled
- All clippy warnings must be resolved (CI enforces `-D warnings`)
- Import ordering is enforced by rustfmt

### Adding New API Endpoints

When adding new Dashboard APIs (e.g., Serve API):

1. Create new trait in `src/client/` (e.g., `serve.rs`)
2. Define request/response schemas in `src/schemas/`
3. Implement trait for `RayDashboardClient`
4. Export trait from `src/lib.rs`
5. Add integration tests in `tests/`
6. Follow the existing pattern from `JobSubmissionAPI` as reference

### Error Handling

The SDK uses a custom `Result<T>` type alias with a `thiserror`-based `Error` enum. All API methods return `crate::Result<T>`. Add new error variants to `src/error.rs` as needed.

## CI/CD

GitHub Actions runs on PRs and main branch:
- Formatting check (`cargo fmt -- --check`)
- Clippy lints (`cargo clippy -- -D warnings`)
- Unit tests (`cargo test --lib`)
- Integration tests with Docker Compose Ray cluster

Auto-publishes to crates.io on main branch pushes.
