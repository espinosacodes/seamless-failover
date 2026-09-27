# Decisions (ADRs)

Scope: whole project
Do not touch: N/A
Last updated: 2026-09-26

Living ADRs. Append new ones; do not rewrite history.

Format:

```markdown
## ADR-00N: Title
Date: YYYY-MM-DD
Scope: <folders>
Status: accepted | superseded by ADR-00X
### Context
### Decision
### Why
```

---

## ADR-005: HTTP probe, not ICMP

Date: 2026-09-23
Scope: netfailoverd/
Status: accepted

### Context

The primary network blocks ICMP to its own gateway (observed: 2 packets to
172.30.0.1 lost, while 1.1.1.1 answered). An ICMP-based "is there internet"
check would be wrong on this network and many corporate ones.

### Decision

Reachability is HTTP based: request a small stable endpoint and assert expected
content, so captive portals are rejected. Never use ping as the deciding probe.

### Why

Correct on the observed network, resistant to captive portals, and independent
of gateway policy.

---

## ADR-004: USB tethering is the primary failover path

Date: 2026-09-23
Scope: netfailoverd/, iphone/
Status: accepted

### Context

Two hotspot transports exist: USB ("iPhone USB", en8) and wireless Instant
Hotspot. USB appears as a normal service with its own DNS and does not require
the Wi-Fi radio to reassociate or a password. Wireless needs the Wi-Fi radio,
Instant Hotspot discovery, and can collide with the dead network's auto-join.

### Decision

Phase 1 failover targets USB tethering. Wireless Instant Hotspot is Phase 3.

### Why

USB is the most reliable, fully scriptable path and removes a whole class of
Wi-Fi race conditions from the MVP.

---

## ADR-003: Switch by network service order, not raw route

Date: 2026-09-23
Scope: netfailoverd/
Status: accepted

### Context

Changing only the default route with `route change` leaves the resolver and
per-service scoping on the old interface, which breaks DNS for agents even
though packets would flow.

### Decision

Switch using `networksetup -ordernetworkservices`, then flush DNS
(`dscacheutil -flushcache`, `killall -HUP mDNSResponder`), then verify.

### Why

Keeps DNS, IPv6 scoping, and service metadata consistent with the active path.
`route` is only a fallback if service reordering cannot achieve the switch.

---

## ADR-002: No third-party mobile app to force Personal Hotspot

Date: 2026-09-23
Scope: iphone/
Status: accepted

### Context

The operator asked for a mobile app that forces the iPhone to share data.
Apple documents that iOS has no API for configuring Personal Hotspot, and App
Review rejects private-API use. A normal app cannot do this.

### Decision

Do not build an app for this. Enable the hotspot through, in order: a manual
toggle (Phase 1), an iPhone Shortcuts personal automation with a Personal
Hotspot action if available (Phase 2), or an MDM command on a supervised device
(Phase 4). Record the unverified Shortcuts action as an open question.

### Why

Only sanctioned mechanisms actually ship. It avoids wasted work on an app that
cannot be approved or would break on every iOS update.

---

## ADR-001: Failover runs as a root LaunchDaemon on the Mac

Date: 2026-09-23
Scope: netfailoverd/
Status: accepted

### Context

The operator is fine entering the admin password once. Route and service-order
changes require root. Failover must survive logout and run without a GUI.

### Decision

Implement `netfailoverd` as a `LaunchDaemon` under `/Library/LaunchDaemons`
with `KeepAlive`, installed once. Privileged actions live only there.

### Why

Reliable across login sessions and reboots, standard macOS pattern, single
place that touches the network stack.

---

## ADR-006: Rust for netfailoverd

Date: 2026-09-26
Scope: netfailoverd/
Status: accepted

### Context

The operator chose Rust for the daemon. The daemon is always on, runs as root,
and must be reliable across sleep, reboot, and flaky networks. It shells out to
`networksetup`, `dscacheutil`, and `mDNSResponder` control, and it needs an HTTP
probe, TOML config, file logging, and a hysteresis selfcheck.

### Decision

Implement `netfailoverd` and `netfailoverctl` in Rust. Ship a single binary
with no runtime dependency. Keep all privileged actions in the daemon. Talk to
the CLI over a root owned control socket. Use a small HTTP client, TOML config
parsing, and structured logging to `os_log` plus `/var/log/netfailover.log`.

### Why

Memory safety for an always on root process, predictable builds across macOS
versions, and good test support for the state machine. No interpreted runtime
to install or break.

---

## ADR-007: Wireless first UX, cable validated core

Date: 2026-09-26
Scope: whole project
Status: accepted

### Context

The goal is a tool other people can install and forget, with no cable in daily
use. iOS still exposes no public API to turn on Personal Hotspot, and wireless
failover adds Wi-Fi reassociation and Instant Hotspot handling on top of the
core probe and switch logic.

### Decision

Keep the Phase 1 core transport agnostic and validate it with cable plus a
`pf` upstream block, then ship wireless as the primary daily UX. Product work
targets simple install, clear status, and safe defaults so the tool is useful
beyond this machine.

### Why

Cable validation is deterministic and proves hysteresis, DNS rescoping, and
revert without Wi-Fi races. Wireless is what people want day to day. One core
serves both, so we do not skip safety gates to reach wireless.
