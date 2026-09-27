use anyhow::{Context, Result};
use std::process::Command;

pub trait CommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<String>;
}

pub struct SystemRunner;

impl CommandRunner for SystemRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<String> {
        let out = Command::new(program)
            .args(args)
            .output()
            .with_context(|| format!("run {program}"))?;
        if !out.status.success() {
            let err = String::from_utf8_lossy(&out.stderr).to_string();
            anyhow::bail!("{program} failed: {err}");
        }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

pub struct SwitchExecutor<R: CommandRunner> {
    pub runner: R,
    pub dry_run: bool,
    pub primary_service: String,
    pub fallback_service: String,
}

impl<R: CommandRunner> SwitchExecutor<R> {
    pub fn failover(&self) -> Result<String> {
        self.set_first_service(&self.fallback_service, "failover")
    }

    pub fn revert(&self) -> Result<String> {
        self.set_first_service(&self.primary_service, "revert")
    }

    fn set_first_service(&self, first: &str, reason: &str) -> Result<String> {
        let plan = format!("{reason}: order services so {first} is first");
        if self.dry_run {
            return Ok(format!("dry-run: {plan}"));
        }
        // Read current order so the change is auditable.
        let before = self
            .runner
            .run("networksetup", &["-listnetworkserviceorder"])
            .unwrap_or_else(|_| "unknown order".to_string());
        let other = if first == self.primary_service {
            &self.fallback_service
        } else {
            &self.primary_service
        };
        // Keep the remaining services after the two managed ones is out of
        // scope for Phase 1. Move the desired service above the other one.
        self.runner.run(
            "networksetup",
            &["-ordernetworkservices", first, other],
        )?;
        self.flush_dns()?;
        Ok(format!("applied: {plan}; before: {before}"))
    }

    fn flush_dns(&self) -> Result<()> {
        self.runner.run("dscacheutil", &["-flushcache"])?;
        self.runner.run("killall", &["-HUP", "mDNSResponder"])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeRunner {
        commands: std::cell::RefCell<Vec<String>>,
    }

    impl CommandRunner for FakeRunner {
        fn run(&self, program: &str, args: &[&str]) -> Result<String> {
            self.commands
                .borrow_mut()
                .push(format!("{program} {}", args.join(" ")));
            Ok("ok".to_string())
        }
    }

    #[test]
    fn dry_run_runs_nothing() {
        let ex = SwitchExecutor {
            runner: FakeRunner {
                commands: std::cell::RefCell::new(vec![]),
            },
            dry_run: true,
            primary_service: "Wi-Fi".to_string(),
            fallback_service: "iPhone USB".to_string(),
        };
        let msg = ex.failover().unwrap();
        assert!(msg.starts_with("dry-run"));
        assert!(ex.runner.commands.borrow().is_empty());
    }

    #[test]
    fn live_run_orders_and_flushes_dns() {
        let ex = SwitchExecutor {
            runner: FakeRunner {
                commands: std::cell::RefCell::new(vec![]),
            },
            dry_run: false,
            primary_service: "Wi-Fi".to_string(),
            fallback_service: "iPhone USB".to_string(),
        };
        ex.failover().unwrap();
        let cmds = ex.runner.commands.borrow().join("\n");
        assert!(cmds.contains("networksetup -ordernetworkservices iPhone USB Wi-Fi"));
        assert!(cmds.contains("dscacheutil -flushcache"));
        assert!(cmds.contains("killall -HUP mDNSResponder"));
    }
}
