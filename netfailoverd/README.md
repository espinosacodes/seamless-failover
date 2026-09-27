# netfailoverd

Phase 1 core in Rust. Probes the primary with HTTP, switches service order to
the fallback on sustained failure, reverts after recovery plus cooldown.

## Layout

* `src/config.rs`: TOML config, no secrets
* `src/probe.rs`: HTTP probe with captive portal check
* `src/state.rs`: hysteresis state machine plus tests
* `src/switch.rs`: service order switch plus DNS flush
* `src/main.rs`: daemon loop, dry run, disable lock, selfcheck
* `src/bin/netfailoverctl.rs`: status, disable, resume
* `config/example-netfailover.toml`: copy to `/usr/local/etc/netfailover.conf`
* `launchd/`: LaunchDaemon plist with KeepAlive

## Verify

```sh
cargo test
cargo run --bin netfailoverd -- --selfcheck
cargo run --bin netfailoverd -- --once --dry-run --config config/example-netfailover.toml
```

Failover simulation uses `scripts/pf-block.sh` and `scripts/pf-unblock.sh`.
