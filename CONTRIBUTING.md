**English** | [简体中文](CONTRIBUTING.zh-CN.md)

> English is normative. If the translations differ, this document takes precedence.

# Contributing to HexGo

Thank you for helping build HexGo. The project welcomes human-authored and AI-assisted contributions under the same standards of correctness, review, and accountability.

## Before You Start

1. Read the [rules](docs/RULES.md), [architecture](docs/ARCHITECTURE.md), and [roadmap](docs/ROADMAP.md).
2. Check existing issues and pull requests to avoid duplicate work.
3. For a substantial feature, rule change, new dependency, or architectural change, open a proposal before implementation.
4. Confirm that your working tree does not contain unrelated changes:

```bash
git status --short --branch
```

## Branch Workflow

Never commit directly to `main`. Create a branch from an up-to-date `main` before editing tracked files:

```bash
git switch main
git pull --ff-only
git switch -c docs/short-description
```

Use lowercase kebab-case after one of these prefixes:

- `feat/` for new behavior
- `fix/` for bug fixes
- `docs/` for documentation
- `refactor/` for behavior-preserving restructuring
- `test/` for tests
- `chore/` for maintenance

Keep a branch and pull request focused on one concern.

## Development Standards

- Use stable Rust, edition 2024, standard `rustfmt`, and idiomatic Rust naming.
- Keep the domain rules independent from Bevy and other presentation concerns.
- Write all source code, identifiers, comments, Rust doc comments, commit messages, logs, and technical diagnostics in English.
- Add tests for new behavior and regressions. Rule behavior should be deterministic and testable without rendering a window.
- Do not add a dependency, introduce `unsafe`, or change public behavior without prior agreement.
- Never edit `Cargo.lock` manually.

Comments should explain intent, invariants, or a non-obvious decision rather than restating code.

## Rules and Translations

- [English rules](docs/RULES.md) are normative; the [Chinese rules](docs/RULES.zh-CN.md) are an official translation.
- Do not change a game rule incidentally. Rule changes require a focused proposal and explicit maintainer approval.
- Update both rules documents in the same pull request, preserving section order, examples, formulas, and terminology.
- Other paired English and `.zh-CN.md` documents must also be updated together.
- Keep the language switcher at the top of every paired document.

If two language versions disagree, report the discrepancy in the pull request. Do not silently reinterpret the rule.

## Local Validation

Run the complete local quality gate before opening a code pull request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --all-features
cargo test --all-features
```

For documentation-only changes, also run:

```bash
git diff --check
```

Review all changed relative links and confirm that paired translations remain synchronized. Optional pre-commit hooks can be installed with `pre-commit install`.

## Commits

Write commit messages in English using Conventional Commit style:

```text
feat(board): generate the playable vertex graph
fix(rules): reject repeated board positions
docs(rules): clarify boundary liberties
```

Use an imperative, concise subject. Do not mix unrelated changes in one commit. Do not bypass hooks with `--no-verify`.

## Pull Requests

- Use a Conventional Commit style English title suitable for squash merging.
- Explain the problem, approach, user-visible behavior, and any rule or architecture impact.
- Link related issues and include screenshots for visible UI changes.
- List the exact validation commands run and any checks not run.
- Confirm that relevant English and Chinese documents were updated together.
- Resolve review conversations and obtain at least one human approval.
- Wait for all required CI checks before merging.

### AI-assisted contributions

AI assistance is allowed, but the pull request must disclose:

- Which tools or agents were used and what work they performed;
- What the human contributor reviewed or changed;
- Which tests and checks were run;
- Any remaining uncertainty or risk.

Do not add AI attribution to every commit unless a maintainer requests it. The human contributor submitting the pull request is responsible for correctness, security, licensing, and compliance with these guidelines.

## Reporting Problems

- Use the bug report form for reproducible defects.
- Use the feature proposal form for new behavior or rule discussions.
- Follow [SECURITY.md](SECURITY.md) for vulnerabilities; never publish sensitive security details in an issue.
- Follow [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) in all project spaces.
