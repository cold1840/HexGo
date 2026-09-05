**English** | [简体中文](ROADMAP.zh-CN.md)

> English is normative. If the translations differ, this document takes precedence.

# HexGo Roadmap

This roadmap expresses direction, not release dates. A phase is complete only when its acceptance criteria are met. Priorities may change through reviewed proposals, but rule changes require explicit approval and synchronized rule documents.

## Phase 0: Project Foundation

Goal: make the repository understandable and safe for multilingual, human, and AI-assisted collaboration.

- Publish normative English documentation and official Simplified Chinese translations.
- Define contribution, security, conduct, AI, and protected-branch workflows.
- Preserve formatting, linting, build, and test gates in CI.
- Configure GitHub branch protection according to `BRANCH_PROTECTION.md`.

Acceptance criteria:

- A new contributor can set up, run, validate, and propose a change using repository documentation alone.
- Direct pushes to `main` are blocked by repository settings.
- All documentation links and language pairs are valid.

## Phase 1: Rules Engine

Goal: implement the full HexGo rules as deterministic Rust logic independent from Bevy.

- Build and validate the vertex-edge board graph.
- Represent game configuration, occupancy, turns, actions, and results.
- Implement groups, unique liberties, captures, suicide prevention, and positional superko.
- Implement pass, resignation, game completion, empty regions, area scoring, and komi.
- Add small graph fixtures and comprehensive unit and integration tests for every normative rule.

Acceptance criteria:

- All behavior in `RULES.md` is covered by automated tests.
- Illegal actions leave the state unchanged.
- The rules engine can run headlessly and does not depend on Bevy.

## Phase 2: Local Playable Game

Goal: deliver a complete local two-player game through Bevy.

- Render the board graph, playable vertices, and stones clearly.
- Support point selection, placement, pass, resignation, restart, and game configuration.
- Display the current player, invalid-action feedback, consecutive passes, game result, and score breakdown.
- Keep rendering synchronized from domain state and provide accessible input and readable visual states.

Acceptance criteria:

- Two people can complete a rules-correct game from launch through result.
- UI actions cannot bypass domain validation.
- Core flows receive focused automated checks and a documented manual smoke test.

## Phase 3: Persistence, Replay, and Release Readiness

Goal: make games reproducible and prepare a dependable distributable build.

- Define a versioned save and replay format without coupling it to render state.
- Support loading, replay navigation, and clear errors for invalid or incompatible files.
- Improve settings, onboarding, accessibility, diagnostics, and platform packaging.
- Establish release notes, versioning, supported-platform checks, and release artifacts.

Acceptance criteria:

- A completed game can be saved, loaded, and replayed deterministically.
- Invalid data fails safely without corrupting current state.
- Supported builds pass documented release checks.

## Future Candidates

The following are intentionally uncommitted and require separate proposals:

- AI opponents, analysis, and self-play for komi research;
- Online multiplayer and spectator support;
- Matchmaking, accounts, rankings, or hosted services;
- Additional board presets and competitive rule profiles.

Future AI and network clients must use the same validated game-command interface as local players.
