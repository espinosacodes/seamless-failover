# Constraints

Scope: netfailoverd/, config/, iphone/
Do not touch: N/A, applies wherever those scopes are edited
Last updated: 2026-09-23

## Apple and iOS constraints

1. **No public iOS API to configure Personal Hotspot.** Apple Developer
   Relations states iOS has no API for configuring Personal Hotspot. A
   third-party app cannot turn it on. See ADR-002.
2. **Shortcuts may expose a "Set Personal Hotspot" action.** One community
   writeup claims iOS 16 Shortcuts has this action with a Turn On value. This is
   unverified on the installed iOS 26. Treat as a Phase 2 task, not a fact. See
   open questions.
3. **MDM can enable Personal Hotspot** (`enable_personal_hotspot`), but only on
   a supervised device enrolled in MDM. Heavy; a fallback, not the default.
4. **Private APIs / jailbreak** are rejected. Not shippable, not supported.
5. **Instant Hotspot** lets a Mac join the iPhone hotspot without typing a
   password when both share an Apple ID, but the hotspot must already be on.

## macOS constraints

1. **No reachability-based failover.** macOS moves between services on link
   state, not on "link up but no internet". This is the reason the project
   exists.
2. **Root is required** to change service order or routes. Hence a
   LaunchDaemon, installed once with the admin password.
3. **Per-service DNS.** A bare default-route change leaves DNS pointing at the
   dead network. Always re-scope DNS on switch (ADR-003).
4. **ICMP is unreliable.** The observed corporate network (172.30.0.0/16)
   blocks ping to its gateway while the internet still works. Probes must be
   HTTP (ADR-005).
5. **VPN `utun` interfaces can own the default route.** Multiple `utun` devices
   were observed. A failover that ignores the VPN may appear to do nothing.
6. **Service names, not just interfaces.** `networksetup` addresses services by
   display name ("Wi-Fi", "iPhone USB"). Names can change; config should also
   record the hardware port and interface to detect drift.

## Observed environment (2026-09-23)

- macOS 26.6.2 (build 25G83).
- Wi-Fi primary: service "Wi-Fi", interface en0, network 172.30.0.0/16,
  gateway 172.30.0.1 (ICMP blocked).
- iPhone tethering service present: "iPhone USB", interface en8.
- Several VPN / bridge interfaces present (`utun0..5`, `bridge0`, docker
  `feth`). Plan for a VPN-aware switch.
- `sudo` needs a password (no passwordless sudo). One-time entry at install is
  acceptable to the operator.

## Repo constraints

- No secrets: no Wi-Fi or hotspot passwords in the repo or config.
- Specs use ASCII hyphens only; no em or en dashes.
- Do not build past the current phase in the roadmap.

## Verification gates

- Probes must not depend on ICMP or on the gateway.
- A failover and a revert must each be reproducible with a `pf`-based upstream
  block, in dry-run and live modes.
- Hysteresis logic (the state machine) must have a runnable selfcheck that
  feeds it synthetic probe outcomes and asserts no flapping.
