# MASH Stack - Claude Guide

## Commands
- Build: `cargo build`
- Run: `cargo run`
- Development: `cargo watch -x "run"`
- Lint: `cargo clippy`
- Format: `cargo fmt`
- Test: `cargo test`
- Single test: `cargo test test_name`

## Code Style
- **Structure**: Modular with handlers/ and views/ directories
- **Imports**: Group by source with blank lines between, external first
- **Formatting**: 4-space indents, semicolons for statements
- **Types**: Explicit annotations, derive common traits
- **Naming**: snake_case for functions/variables, CamelCase for types
- **Error Handling**: Use Result<T,E> with proper error propagation
- **State**: Thread-safe containers (Arc<Mutex<T>>) for shared state
- **Documentation**: Add doc comments for public functions/types
- **Async**: Use Tokio with async/await consistently

This project uses Maud for HTML templates, Axum for routing, SQLx for database, and HTMX for client interactions - follow existing patterns.