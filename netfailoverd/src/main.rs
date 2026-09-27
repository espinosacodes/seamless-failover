mod config;
mod probe;
mod state;
mod switch;

use anyhow::Result;
use clap::Parser;
use config::AppConfig;
use state::{Action, StateMachine};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use switch::{SwitchExecutor, SystemRunner};

#[derive(Parser, Debug)]
#[command(name = "netfailoverd", about = "Mac upstream failover daemon")]
struct Args {
    #[arg(long, default_value = "/usr/local/etc/netfailover.conf")]
    config: PathBuf,
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    once: bool,
    #[arg(long)]
    selfcheck: bool,
}

fn log_line(cfg: &AppConfig, line: &str) {
    let enriched = format!("{line}\n");
    print!("{enriched}");
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&cfg.log_path) {
        let _ = f.write_all(enriched.as_bytes());
    }
}

fn write_state(cfg: &AppConfig, text: &str) {
    if let Some(parent) = std::path::Path::new(&cfg.state_path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(&cfg.state_path, text);
}

fn disabled(cfg: &AppConfig) -> bool {
    std::path::Path::new(&cfg.disable_lock_path).exists()
}

fn run_selfcheck() -> Result<()> {
    // Synthetic hysteresis check without touching the network.
    let mut m = StateMachine::new(3, 3, 6);
    for _ in 0..20 {
        m.tick(true, true);
        m.tick(false, true);
    }
    anyhow::ensure!(m.state == state::State::Healthy, "flapping caused a switch");
    let mut m = StateMachine::new(3, 3, 6);
    m.tick(false, true);
    m.tick(false, true);
    m.tick(false, true);
    anyhow::ensure!(m.state == state::State::Degraded, "no degraded after N fails");
    println!("selfcheck ok: hysteresis holds, no flapping");
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.selfcheck {
        return run_selfcheck();
    }
    let mut cfg = AppConfig::load(&args.config).unwrap_or_default();
    if args.dry_run {
        cfg.dry_run = true;
    }
    let mut machine = StateMachine::new(
        cfg.fail_threshold,
        cfg.recover_threshold,
        cfg.cooldown_ticks(),
    );
    log_line(
        &cfg,
        &format!(
            "netfailoverd start primary={} fallback={} dry_run={}",
            cfg.primary_service, cfg.fallback_service, cfg.dry_run
        ),
    );
    loop {
        if disabled(&cfg) {
            log_line(&cfg, "disabled lock present, skipping probe cycle");
            write_state(&cfg, "disabled\n");
        } else {
            let primary = probe::http_probe(
                &cfg.probe_url,
                &cfg.expected_content,
                cfg.probe_timeout_secs,
            );
            // Phase 1 probes the fallback over the current route as well.
            // Source bound probing per service is a later hardening step.
            let fallback = probe::http_probe(
                &cfg.probe_url,
                &cfg.expected_content,
                cfg.probe_timeout_secs,
            );
            let action = machine.tick(primary.ok, fallback.ok);
            write_state(
                &cfg,
                &format!(
                    "state={:?} primary_ok={} fallback_ok={} primary={} fallback={}\n",
                    machine.state, primary.ok, fallback.ok, primary.detail, fallback.detail
                ),
            );
            match action {
                Action::None => {}
                Action::SwitchToFallback => {
                    let ex = SwitchExecutor {
                        runner: SystemRunner,
                        dry_run: cfg.dry_run,
                        primary_service: cfg.primary_service.clone(),
                        fallback_service: cfg.fallback_service.clone(),
                    };
                    match ex.failover() {
                        Ok(msg) => log_line(&cfg, &format!("failover: {msg}")),
                        Err(e) => log_line(&cfg, &format!("failover failed: {e:#}")),
                    }
                }
                Action::RevertToPrimary => {
                    let ex = SwitchExecutor {
                        runner: SystemRunner,
                        dry_run: cfg.dry_run,
                        primary_service: cfg.primary_service.clone(),
                        fallback_service: cfg.fallback_service.clone(),
                    };
                    match ex.revert() {
                        Ok(msg) => log_line(&cfg, &format!("revert: {msg}")),
                        Err(e) => log_line(&cfg, &format!("revert failed: {e:#}")),
                    }
                }
            }
        }
        if args.once {
            break;
        }
        thread::sleep(Duration::from_secs(cfg.probe_interval_secs.max(1)));
    }
    Ok(())
}
