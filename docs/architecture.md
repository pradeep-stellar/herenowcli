# Architecture

## Goals

`herenow` is a Rust CLI for here.now's public API. It should be useful in three
contexts:

- Humans publishing local sites from a terminal.
- CI jobs that pass `HERENOW_API_KEY`.
- Agents that need predictable JSON output, durable local state, and safe
  credential handling.

The first implementation prioritizes Sites publishing and account operations,
while keeping the command surface aligned with the documented OpenAPI routes.

## API Surface

The CLI follows the stable public API described by here.now docs and OpenAPI:

- `auth`: request and verify one-time email codes.
- `publish`: create or update a Site by scanning local files, uploading changed
  content to presigned URLs, and finalizing the version.
- `sites`: list, inspect, delete, claim, duplicate/future metadata controls.
- `drives`: list, create, inspect, file upload/download/list/delete/move, and
  scoped token management.
- `domains`, `handle`, `links`: public routing controls.
- `variables`: server-side service variables for proxy routes.
- `wallet`: payment wallet helper endpoints.

Initial Drive support intentionally covers direct file operations only: list,
read, upload/finalize, delete, move, and scoped token creation. Batch
commit/import/export can be layered on later without changing the public module
layout.

## Module Layout

```text
src/
  main.rs          async entry point and top-level error rendering
  cli.rs           clap command definitions and argument parsing
  config.rs        API base URL, client attribution, and credential loading
  error.rs         API error model and response mapping
  http.rs          reqwest wrapper with auth, JSON, and upload helpers
  files.rs         local publish file discovery, MIME guessing, SHA-256 hashing
  output.rs        human vs JSON output helpers
  state.rs         .herenow/state.json read/write for anonymous claim tokens
  api/
    mod.rs         API module exports
    models.rs      serde request/response structs shared across APIs
    auth.rs        agent login endpoints
    sites.rs       Site publish/update/finalize/list/detail/metadata flows
    drives.rs      Drive file and token endpoints
    domains.rs     domain, handle, and link endpoints
    variables.rs   service variable endpoints
    wallet.rs      wallet/payment helper endpoints
```

## Key Choices

### Hand-Written Client Over Generated Code

The OpenAPI spec is the source of truth for route coverage, but the CLI uses a
hand-written typed client. Generated clients tend to expose route-shaped
functions but do not help with the important CLI behavior: walking directories,
normalizing publish paths, hashing files for incremental deploys, refreshing
state, uploading to presigned URLs, and saving credentials securely.

### Async Runtime

The CLI uses `tokio` and `reqwest`. Publishing naturally benefits from parallel
uploads, and the same client can later support streaming Drive downloads or paid
site fetch sessions.

### Credential Handling

The credential lookup order matches the here.now helper behavior:

1. Explicit `--api-key`.
2. `HERENOW_API_KEY`.
3. `~/.herenow/credentials`.

The `auth login` command writes verified API keys to `~/.herenow/credentials`
with `0600` permissions on Unix.

### Local State

Anonymous Site updates require a `claimToken`. The CLI stores anonymous publish
metadata under `.herenow/state.json` in the current working directory. That file
is treated as a cache, not as authority for remote state.

### Output

Commands default to concise human output. `--json` emits structured JSON for
automation and agent workflows. Errors preserve the server's stable `code`,
`message`, `retry_after`, and `docs_url` fields when present.

## Open Questions

- Whether the first release should include full Drive batch commit support or
  start with direct file upload/finalize operations only. Decision: start with
  direct file operations.
- Whether destructive commands such as `sites delete` should require an
  interactive confirmation by default, or rely on explicit flags in CI.
  Decision: require confirmation unless `--yes` is passed.
- Whether to add shell completions in the initial version.
