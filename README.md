# ray-dashboard-sdk

A simple Rust client for the [Ray]( https://docs.ray.io/en/latest/index.html#) Dashboard REST API.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/ray-dashboard-sdk.svg
[crates-url]: https://crates.io/crates/ray-dashboard-sdk
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/sfriedowitz/ray-dashboard-sdk/blob/main/LICENSE
[actions-badge]: https://github.com/sfriedowitz/ray-dashboard-sdk/workflows/CI/badge.svg
[actions-url]: https://github.com/sfriedowitz/ray-dashboard-sdk/actions/workflows/ci.yml

## Overview

A Rust SDK for the Ray Dashboard REST API. Currently supports the Jobs API for submitting and managing Ray jobs.

See [examples/](examples/) for usage examples.

## Contributing

### Running Tests

Integration tests require a running Ray cluster:

```bash
# Start Ray cluster
docker compose up -d

# Run tests
cargo test

# Stop Ray cluster
docker compose down
```

### Running Examples

Examples also require the Ray cluster:

```bash
docker compose up -d
cargo run --example simple
cargo run --example axum
```

## TODO

- Expand unit tests for schemas
- Implement Serve API bindings