**English** | [简体中文](README.zh-CN.md)

> English is the normative project language. If the translations differ, the English document takes precedence.

# HexGo

HexGo is a new Go-like board game played on the vertices of a regular hexagonal tiling. A typical interior point has three neighbors instead of the four found on a standard Go board, producing a distinct topology for connections, liberties, captures, life, and territory.

The project is in its initial development stage. The game rules are specified, while the playable implementation currently starts a minimal Bevy application.

## Core Rules at a Glance

- Black and White alternate placing stones on connected board vertices; Black moves first.
- Connected stones of one color form a group, and adjacent empty vertices are its liberties.
- A group with no liberties is captured and removed.
- Suicide is forbidden after opposing captures have been resolved.
- Positional superko forbids a move that recreates any earlier board position.
- Passing is always legal; two consecutive passes end the game.
- Area scoring counts living stones and exclusively surrounded empty points. White also receives configurable komi.

Read the complete [English rules](docs/RULES.md) or [Simplified Chinese rules](docs/RULES.zh-CN.md).

## Technology and Status

- Language: Rust 2024 edition
- Game engine: Bevy 0.19.1
- Toolchain: stable Rust with `rustfmt` and Clippy
- License: GNU General Public License v3.0

Current priorities are documented in the [roadmap](docs/ROADMAP.md). The intended subsystem boundaries are described in the [architecture](docs/ARCHITECTURE.md).

## Getting Started

Install [Rust](https://www.rust-lang.org/tools/install) with `rustup`. The repository's `rust-toolchain.toml` selects the required stable toolchain and components automatically.

On Linux, Bevy may require native audio, input, Wayland, and X11 development packages. See the package list in `.github/workflows/ci.yml` as a tested Ubuntu reference.

Clone the repository, create a working branch, and run the application:

```bash
git clone https://github.com/cold1840/HexGo.git
cd HexGo
git switch -c feat/your-change
cargo run
```

Do not commit directly to `main`.

## Quality Checks

Run the same checks used by CI before opening a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --all-features
cargo test --all-features
```

Optional local hooks are configured through [pre-commit](https://pre-commit.com/):

```bash
pre-commit install
pre-commit run --all-files
```

## Documentation

- [Game rules](docs/RULES.md) ([中文](docs/RULES.zh-CN.md))
- [Architecture](docs/ARCHITECTURE.md) ([中文](docs/ARCHITECTURE.zh-CN.md))
- [Roadmap](docs/ROADMAP.md) ([中文](docs/ROADMAP.zh-CN.md))
- [Contributing](CONTRIBUTING.md) ([中文](CONTRIBUTING.zh-CN.md))
- [Code of Conduct](CODE_OF_CONDUCT.md) ([中文](CODE_OF_CONDUCT.zh-CN.md))
- [Security policy](SECURITY.md) ([中文](SECURITY.zh-CN.md))
- [Branch protection setup](docs/BRANCH_PROTECTION.md) ([中文](docs/BRANCH_PROTECTION.zh-CN.md))

AI coding agents must also follow [AGENTS.md](AGENTS.md).

## Contributing

Human and AI-assisted contributions are welcome. Every change must be made on a non-`main` branch and merged through a reviewed pull request. Read [CONTRIBUTING.md](CONTRIBUTING.md) before starting.

## License

HexGo is licensed under the [GNU General Public License v3.0](LICENSE).
