**English** | [简体中文](ARCHITECTURE.zh-CN.md)

> English is normative. If the translations differ, this document takes precedence.

# HexGo Architecture

## Purpose

This document defines the intended boundaries for the first playable HexGo implementation. It guides future code organization; it does not claim that the current minimal application already implements these components.

The central architectural rule is:

> The complete game rules must be implemented as deterministic, platform-independent Rust logic. Bevy consumes that logic but does not define it.

## Layers

### 1. Domain layer

The domain layer owns all rule concepts and has no dependency on Bevy, rendering, input devices, operating-system services, or wall-clock time.

Its responsibilities are:

- Represent the board as an explicit graph of vertices and edges;
- Represent empty, Black, and White vertex states;
- Track the player to move, consecutive passes, komi, game status, and board-position history;
- Find groups, unique liberties, connected empty regions, and territory;
- Validate and apply stone placements in the exact order specified by the rules;
- Apply captures, suicide checks, and positional superko;
- Apply passes and resignations;
- Produce final area scores and results.

Domain operations should return structured outcomes and errors rather than display text or trigger UI effects. An illegal action must leave the state unchanged. Board topology and rules should support deterministic equality and hashing for position history.

### 2. Application layer

The application layer coordinates a game session without owning rule definitions. It translates user intentions into domain actions and publishes outcomes for presentation.

Its responsibilities are:

- Create a game from validated configuration;
- Submit place, pass, and resign commands;
- Expose legal outcomes, captures, turn changes, scores, and game-over state;
- Coordinate new game, restart, save, and replay workflows when those features are implemented.

This layer must not duplicate capture, scoring, or legality logic.

### 3. Presentation layer

The Bevy presentation layer owns rendering and interaction:

- Draw the hexagonal tiling, playable vertices, stones, highlights, and UI;
- Convert pointer or keyboard input into application commands;
- Display legal-move feedback, turn status, pass state, score, and result;
- Animate domain outcomes without changing their meaning.

Bevy ECS components may reference stable domain identifiers, but domain types must not require Bevy ECS traits to function.

### 4. Adapters

Persistence, replay formats, analytics, and future AI or network integrations are adapters around the application and domain layers. They must use the same public game commands as a local human player and may not bypass legality checks.

## Data Flow

```text
Player input
    ↓
Bevy presentation
    ↓
Application command
    ↓
Domain validation and state transition
    ↓
Structured outcome or error
    ↓
Application event
    ↓
Bevy presentation update
```

Rendering never serves as the source of truth for occupancy, adjacency, turn order, or scoring.

## Rule Invariants

- Every edge connects two existing vertices, and adjacency is symmetric.
- A vertex has exactly one state: empty, Black, or White.
- Groups, liberties, and regions are derived from graph edges only.
- Opposing zero-liberty groups are removed before checking the newly placed stone's group.
- A rejected action does not mutate state or consume a turn.
- Pass bypasses superko; a stone placement resets the consecutive-pass count.
- Position history used for positional superko contains board occupancy only, not the player to move.
- Stones remaining after two consecutive passes are treated as alive for area scoring.

## Testing Strategy

Domain tests should use small, explicit graph fixtures rather than Bevy scenes. Tests must cover graph boundaries, merged groups, shared liberties, simultaneous captures, capture-before-suicide, rejected suicide, positional superko, pass behavior, resignation, empty-region ownership, neutral regions, seki scoring consequences, komi, and atomic rejection of illegal actions.

Application tests should verify command sequencing and emitted outcomes. Presentation behavior may use focused Bevy system tests and end-to-end interaction checks after a playable UI exists.

## Dependency Direction

Dependencies point inward:

```text
Presentation and adapters → Application → Domain
```

The domain layer must not import outward layers. New dependencies require explicit approval and should not be added when the standard library or existing dependencies are sufficient.

## Deferred Decisions

Concrete module names, serialized file formats, board-size presets, network protocols, and AI algorithms will be specified when their roadmap phase begins. They must preserve the boundaries and invariants in this document.
