# Roadmap

Scope: whole project
Do not touch: N/A
Last updated: 2026-09-23

Build one phase at a time. Do not start Phase N+1 until Phase N exit criteria
are met. Each phase is its own spec detail; this file is the gate.

## Phase 0: Specs (current)

Deliverable: this spec set.

Exit criteria:
- Index, product, architecture, constraints, decisions, roadmap, open questions
  all present and consistent.
- Open questions list is explicit about what blocks Phase 2.

## Phase 1: Mac-only, cable, manual hotspot

Deliverable:
- `netfailoverd` LaunchDaemon with the state machine, HTTP probe, dry-run mode,
  config file, logging.
- Switch and revert between "Wi-Fi" and "iPhone USB" via service order.
- `netfailoverctl status` and `disable`.

Exit criteria:
- With the cable attached and hotspot on, blocking the primary upstream with
  `pf` moves the Mac to iPhone USB within about 15s.
- Removing the block reverts within about 60s.
- Hysteresis selfcheck passes; no flapping under synthetic probe noise.
- `disable` holds state for at least one full probe cycle.

## Phase 2: iPhone Shortcuts automation

Deliverable:
- A documented Shortcuts personal automation that turns Personal Hotspot on,
  with `Ask Before Running` off, on a chosen trigger.
- Fallback doc: one-tap Control Center toggle if the action is unavailable.

Exit criteria:
- Hotspot is on when the cable is connected and the automation is enabled, with
  no manual step, verified over several cycles.
- Trigger choice documented with its reliability caveats.

## Phase 3: Wireless Instant Hotspot

Deliverable:
- Failover path that reassociates Wi-Fi to the iPhone hotspot when the dead
  network is the current association.
- Handling for the hotspot password or Instant Hotspot pairing.

Exit criteria:
- Wireless failover and revert work without the cable, within the latency
  budgets, without flapping between the dead network and the hotspot.

## Phase 4: MDM-backed hotspot forcing (optional)

Deliverable:
- If the iPhone is supervised, an MDM call from the Mac that enables Personal
  Hotspot on demand.

Exit criteria:
- Documented decision on whether MDM is worth it for this operator, with the
  provisioning cost stated.

## Phase 5: Hardening

Deliverable:
- VPN-aware switching, desktop notifications on failover/revert, menu bar item,
  mobile data usage guardrail.

Exit criteria:
- No spurious switches with a VPN active; operator sees every transition.
