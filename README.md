# herenow

Command-line client for the here.now Sites and Drives API.

`herenow` publishes static sites, manages here.now Sites and Drives, and exposes
account helpers for auth, domains, handles, links, variables, and wallet
settings. It is written in Rust and uses structured JSON output when requested
for CI and agent workflows.

## Install

Install the latest release with Homebrew:

```bash
brew install pradeep-stellar/tap/herenow
```

Or build the local binary from source with Cargo:

```bash
cargo build --release
```

The binary is written to `target/release/herenow`.

During development, you can run commands without installing:

```bash
cargo run -- publish ./dist
```

## Authentication

Credentials are loaded in this order:

1. `--api-key`
2. `HERENOW_API_KEY`
3. `~/.herenow/credentials`

Log in with a one-time email code:

```bash
cargo run -- auth login user@example.com
```

Anonymous publishing is supported when no API key is available. Anonymous claim
metadata is stored in `.herenow/state.json` in the current working directory.

## Usage

Publish a directory:

```bash
cargo run -- publish ./dist
cargo run -- publish ./dist --slug my-site --title "My Site" --spa
```

Work with Sites:

```bash
cargo run -- sites list
cargo run -- sites get my-site
cargo run -- sites metadata my-site --title "New title"
```

Work with Drives:

```bash
cargo run -- drives list
cargo run -- drives create project-files
cargo run -- drives files DRIVE_ID --prefix assets/
cargo run -- drives put DRIVE_ID assets/logo.png --from ./logo.png
```

Use `--json` for machine-readable output:

```bash
cargo run -- --json sites list
```

Destructive commands prompt for confirmation. Pass `--yes` for CI or scripted
use:

```bash
cargo run -- sites delete my-site --yes
```

## Commands

- `auth request-code <email>`: request a login code.
- `auth login <email>`: verify a login code and save credentials.
- `publish <path>`: publish a local site directory.
- `sites`: list, inspect, delete, claim, update metadata, or publish from a
  Drive.
- `drives`: list, create, inspect, upload, download, move, delete, and share
  Drive files.
- `domains`: list, add, inspect, and delete custom domains.
- `handle`: get, create, update, or delete the account handle.
- `links`: list, create, inspect, update, or delete routing links.
- `variables`: list, set, or delete service variables.
- `wallet`: get, set, or clear the wallet address.

Run `herenow --help` or `cargo run -- --help` for the full command reference.

## Development

Common tasks are available through `make`:

```bash
make build
make release
make test
make fmt
make lint
make check
```

See [docs/architecture.md](docs/architecture.md) for module layout and design
notes.
