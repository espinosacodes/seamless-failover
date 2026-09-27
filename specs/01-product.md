# Product

Scope: whole project
Do not touch: N/A
Last updated: 2026-09-26

## Problem

macOS chooses a network service by link state, not by whether that link has a
working upstream. When Wi-Fi stays associated but the router, ISP, or VPN
upstream dies, the Mac keeps using Wi-Fi and every local agent (opencode
server, watchers, keepers) loses internet. There is no built-in "fail over
when the link is up but the internet is not" behavior.

The iPhone can provide a second path, but two things get in the way:

1. The Mac will not move to it while the Wi-Fi link is technically up.
2. Personal Hotspot has to be on. iOS gives no public API to turn it on.

## Goal

When the primary network loses internet, the Mac moves itself to iPhone data
within seconds, and moves back when the primary recovers. The operator touches
nothing. Long term this is a distributable tool: simple install, wireless first
in daily use, safe defaults, so other people get the same recovery.

## User stories

- As an operator, when my Wi-Fi loses upstream, the Mac is back online on
  iPhone data within about 15 seconds, so my agents recover on their own.
- As an operator, when Wi-Fi returns, the Mac switches back within about 60
  seconds and stops burning mobile data.
- As an operator, Personal Hotspot is already on when the Mac needs it, without
  me opening Settings.
- As an operator, I can see current state and force or disable failover.

## Success metrics

- Failover latency: at most 15s from upstream loss to working route.
- Revert latency: at most 60s from upstream recovery.
- Flap rate: zero unintended switches per day on a healthy link.
- Idle cost: zero mobile data used while the primary is healthy.

## Non-goals

- Keeping open TCP connections alive across a switch. Sockets break; agents
  reconnect. Failover is at the IP layer, not session layer.
- Load balancing or using both links at once. One active path at a time.
- Toggling Personal Hotspot from a third-party app. iOS forbids it (ADR-002).
- Working before login on a clean boot with no saved Wi-Fi. Out of scope for
  the first phases.
- Captive portal login automation. Detection is enough to avoid choosing a
  portal as "internet".

## Phase 1 MVP

Mac only, cable only, hotspot turned on by hand once: the daemon moves to
"iPhone USB" when Wi-Fi has no internet, and back when it does. No iPhone
automation yet.
