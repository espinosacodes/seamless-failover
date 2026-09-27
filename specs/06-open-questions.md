# Open questions

Scope: whole project
Do not touch: N/A
Last updated: 2026-09-23

Resolve or mark blocked before deciding anything that depends on them.

## OQ-001: Does iOS 26 Shortcuts have a "Set Personal Hotspot" action?

Why it matters: Phase 2 depends on it. If it does not exist, the only
sanctioned automation paths are MDM (Phase 4) or manual tapping.

How to check: on the iPhone, Shortcuts, new shortcut, search "Set Personal
Hotspot". Confirm it actually toggles the hotspot and is not just a settings
deep link. Record the exact iOS version.

## OQ-002: What triggers the iPhone automation reliably?

Candidates and doubts:
- Bluetooth: does the iPhone maintain a dependable Bluetooth connection to this
  Mac so a connect trigger fires?
- Wi-Fi disconnect from the dead network: does it fire when the Mac is still on
  that network but the iPhone is on cellular?
- Time based: reliable but wasteful.

How to check: build the Phase 2 automation with each trigger and measure how
often it fires without a manual nudge.

## OQ-003: Is the iPhone supervised and MDM-eligible?

Why it matters: decides whether Phase 4 is on the table at all.

How to check: whether the iPhone can be enrolled in an MDM (for example via
Apple Configurator) and whether the operator is willing to supervise it.

## OQ-004: Wireless hotspot credentials

Why it matters: Phase 3 reassociation may need the hotspot password, and
Instant Hotspot avoidance of the password depends on shared Apple ID.

How to check: confirm both devices share an Apple ID and that Instant Hotspot
appears without entering a password.

## OQ-005: VPN interaction

Why it matters: several `utun` interfaces can own the default route. A switch
may not change effective connectivity if a VPN pins traffic.

How to check: reproduce a primary outage with the VPN up and observe whether
the failover actually restores the probe.

## OQ-006: Data guardrail

Why it matters: failover to mobile data can burn a plan quickly if the primary
flaps.

How to check: decide whether to cap time on mobile data, notify the operator,
or require confirmation after a threshold.
