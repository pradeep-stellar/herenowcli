# AGENTS.md

Guidance for agents working in this repository.

## Project

`herenow` is a Rust command-line client for the here.now Sites and Drives API.
The binary entrypoint is `src/main.rs`; command definitions live mainly in
`src/cli.rs`; API clients are under `src/api/`.

## Common Commands

- `make build`: build the debug binary.
- `make test`: run the test suite.
- `make fmt`: format all Rust code.
- `make lint`: run clippy with warnings denied.
- `make release`: build the optimized release binary.
- `make check`: run formatting, clippy, and tests.
- `cargo run -- --help`: inspect the CLI.

## Development Notes

- Prefer existing module boundaries and error handling patterns.
- Keep CLI output compatible with both human-readable and `--json` modes.
- Destructive commands should continue to honor confirmation prompts and `--yes`.
- Update `README.md` when command behavior or installation steps change.
- Do not commit build artifacts from `target/`.

## Release Notes

Releases are created by pushing version tags that match `v*`, for example
`v0.1.0`. The release workflow builds native archives for Linux, macOS, and
Windows, then publishes them on the GitHub Release for that tag.
