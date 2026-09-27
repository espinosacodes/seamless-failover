# seamless-failover

Keep the Mac online when the current network loses its upstream.

macOS switches networks on link state, not on usable internet. When Wi-Fi stays
associated but the router, ISP, or VPN upstream dies, the Mac stays on the dead
path. This project moves the Mac to iPhone data automatically, then moves back
when the primary recovers.

## Idea

A small distributable tool for people who work from cafés, offices, and
rentals with fragile Wi-Fi:

1. A root daemon probes usable internet over HTTP, not ICMP.
2. On sustained failure it moves the iPhone path above Wi-Fi in service order,
   rescopes DNS, verifies, and logs the reason.
3. On sustained recovery plus cooldown it reverts to Wi-Fi to save mobile data.
4. Hysteresis prevents flapping. Dry run and disable are built in.

Daily use is wireless first. Cable USB tethering remains the deterministic
validation path because it avoids Wi-Fi races during testing.

## Status

Phase 0 specs are done. Phase 1 core exists in Rust under `netfailoverd/` with
config, HTTP probe, state machine, service order switch, LaunchDaemon plist,
control CLI, hysteresis selfcheck, and pf based simulation scripts.

See `specs/00-index.md` for the read order and `specs/05-roadmap.md` for gates.

## Quick verify

```sh
cd netfailoverd
cargo test
cargo run --bin netfailoverd -- --selfcheck
cargo run --bin netfailoverd -- --once --dry-run --config config/example-netfailover.toml
```

## Repo layout

* `specs/`: living specs, constraints, decisions, roadmap
* `netfailoverd/`: Rust daemon, CLI, config example, launchd plist, scripts
* `AGENTS.md`: operator rules for AI sessions
