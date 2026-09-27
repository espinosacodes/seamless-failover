# AGENTS.md: seamless-failover

Entry point for any AI operator (Cursor, Claude Code, OpenCode, Codex).

## What this is

Keep the Mac online when the current network loses its upstream. A root
LaunchDaemon walks the Mac from Wi-Fi to iPhone data (and back) automatically.
The iPhone side is nudged through Shortcuts automations (and optionally MDM),
never through a third-party "force hotspot" app, because iOS does not allow it.

Status: **specs only**. No runtime code yet. Do not build ahead of the roadmap.

## Source of truth

| Surface | Path | Role |
|---------|------|------|
| Specs | [specs/00-index.md](specs/00-index.md) | Read order and folder map |
| Roadmap | [specs/05-roadmap.md](specs/05-roadmap.md) | Phase gate; do not skip |
| Constraints | [specs/03-constraints.md](specs/03-constraints.md) | Hard limits, verify before coding |
| Decisions | [specs/04-decisions.md](specs/04-decisions.md) | ADRs, append only |
| Open questions | [specs/06-open-questions.md](specs/06-open-questions.md) | Unresolved; check before deciding |

## Read order (every session)

1. This file
2. [specs/00-index.md](specs/00-index.md)
3. [specs/03-constraints.md](specs/03-constraints.md)
4. [specs/05-roadmap.md](specs/05-roadmap.md)
5. Only the specs whose `Scope` matches your task

## Hard vetoes

- No third-party mobile app to toggle Personal Hotspot. iOS exposes no such
  API; it would be rejected or require private APIs. See ADR-002.
- No secrets, Wi-Fi passwords, or hotspot passwords committed. Config holds
  service and interface names only.
- Do not assume ICMP works. Corporate networks (observed: 172.30.0.0/16)
  block ping to the gateway. Probes are HTTP based. See ADR-005.
- Do not change the default route without also fixing DNS for the new service.
  See ADR-003.
- Do not ship Phase N+1 work while Phase N exit criteria are unmet.

## Verify before finishing code changes

Until code exists, the gate is spec consistency:

```bash
ls specs && grep -L "Last updated:" specs/*.md
```

Once `netfailoverd` exists, add: dry-run mode, a hysteresis selfcheck, and a
`pf`-based failover simulation. See [specs/05-roadmap.md](specs/05-roadmap.md).

## Memory updates

Changed a decision or resolved an open question? Update the matching spec and
[specs/04-decisions.md](specs/04-decisions.md) before closing the session.
