# Spec index

Scope: root
Do not touch: N/A (map only)
Last updated: 2026-09-23

## Purpose

Map of living specs for AI operators. This project is **spec-first**: read
before building, and build only the current phase.

## Read order

1. [../AGENTS.md](../AGENTS.md)
2. This file
3. [03-constraints.md](03-constraints.md): hard limits
4. [05-roadmap.md](05-roadmap.md): what may be built now
5. [04-decisions.md](04-decisions.md): ADRs
6. [01-product.md](01-product.md) / [02-architecture.md](02-architecture.md)
7. [06-open-questions.md](06-open-questions.md): check before deciding

## Spec catalog

| File | Role |
|------|------|
| [01-product.md](01-product.md) | Problem, user stories, non-goals, success metrics |
| [02-architecture.md](02-architecture.md) | Components, state machine, switch primitives, flows |
| [03-constraints.md](03-constraints.md) | Apple and macOS hard limits, observed environment |
| [04-decisions.md](04-decisions.md) | Living ADRs |
| [05-roadmap.md](05-roadmap.md) | Phases, deliverables, exit criteria |
| [06-open-questions.md](06-open-questions.md) | Unresolved items to verify |

## Folder → primary specs

| Surface | Open these |
|---------|------------|
| `specs/` | 03, 04, 05 |
| `netfailoverd/` (future) | 02, 03, 04 |
| `iphone/` (Shortcuts, MDM) | 01, 03, 04, 06 |
| `config/` | 02, 03 |

## Header convention

Every file under `specs/` must start with:

```markdown
# Title
Scope: <folders>
Do not touch: <folders out of scope>
Last updated: YYYY-MM-DD
```
