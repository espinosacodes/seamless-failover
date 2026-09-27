# Architecture

Scope: netfailoverd/, config/, iphone/
Do not touch: N/A
Last updated: 2026-09-23

## Components

### Mac side

- `netfailoverd`: root LaunchDaemon, `KeepAlive`. Owns the state machine,
  probes, and switch/revert actions.
- Config file: `/usr/local/etc/netfailover.conf`. Service names, interfaces,
  probe endpoints, thresholds, cooldowns. No secrets.
- Log: `os_log` plus a plain file under `/var/log/netfailover.log`.
- `netfailoverctl`: optional CLI (and later a menu bar item) for
  `status`, `force <service>`, `disable`, `resume`.

### iPhone side

- Transport: USB tethering ("iPhone USB", observed as en8) is primary.
  Wireless Instant Hotspot is a later phase.
- Hotspot enablement, in order of preference:
  1. Manual toggle in Control Center (Phase 1).
  2. Shortcuts personal automation with a "Set Personal Hotspot" action, if that
     action exists on the installed iOS (Phase 2, see open questions).
  3. MDM command `enable_personal_hotspot` from the Mac to a supervised iPhone
     (Phase 4).

## State machine

```
HEALTHY --(N consecutive probe failures)--> DEGRADED
DEGRADED --(fallback has internet)--> FAILOVER
DEGRADED --(primary recovers before switch)--> HEALTHY
FAILOVER --(M consecutive primary successes)--> RECOVERING
RECOVERING --(primary stable for cooldown)--> HEALTHY
RECOVERING --(primary fails again)--> FAILOVER
```

- `N` (fail threshold) and `M` (recover threshold) provide hysteresis and stop
  flapping. Suggested N=3, M=3, probe interval 5s, cooldown 30s.
- Every transition is logged with the reason and the observed probe results.

## Reachability probe

- HTTP based, not ICMP (ADR-005). Example: fetch a small, stable endpoint and
  assert expected content, so a captive portal that returns 200 with a login
  page is not treated as internet.
- Probe the current default path first, then each candidate fallback that has a
  routable address, before deciding to switch.

## Switch primitive

Preferred: change network service order with
`networksetup -ordernetworkservices`, then flush DNS. This keeps per-service
DNS and IPv6 scoping correct, which a raw `route change` does not (ADR-003).

Order of operations for failover:

1. Confirm fallback service is active and has internet.
2. Move fallback service above the primary in service order.
3. `dscacheutil -flushcache` and `killall -HUP mDNSResponder`.
4. Verify new default path reaches the probe endpoint.
5. If verification fails, restore the previous order and log.

Revert does the inverse (primary back on top), then DNS flush, then verify.

## Flows

Failover:

```
Wi-Fi upstream dies
  -> netfailoverd probes fail N times
  -> fallback (iPhone USB) probed and has internet
  -> service order: iPhone USB before Wi-Fi
  -> DNS flush, verify
  -> FAILOVER, agents reconnect
```

Revert:

```
Wi-Fi upstream returns
  -> primary probed and succeeds M times
  -> cooldown
  -> service order: Wi-Fi before iPhone USB
  -> DNS flush, verify
  -> HEALTHY
```

## Failure handling

- Fallback not present or without internet: stay DEGRADED, keep probing, do not
  switch into a black hole.
- In-flight agent connections drop on switch. Expected, documented in product.
- VPN `utun` default routes: detect and report; do not fight the VPN blindly
  (see open questions).
- Daemon crash: `KeepAlive` restarts it; state is re-derived from live probes,
  never trusted from disk.

## Safety

- Dry-run mode that logs intended actions without applying them.
- `netfailoverctl disable` writes a lock file the daemon honors.
- All privileged actions live only in `netfailoverd`; the CLI talks to it via a
  root-owned control socket, not by running `networksetup` itself.
