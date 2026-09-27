use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub primary_service: String,
    pub fallback_service: String,
    pub primary_interface: String,
    pub fallback_interface: String,
    pub probe_url: String,
    pub expected_content: String,
    pub probe_interval_secs: u64,
    pub probe_timeout_secs: u64,
    pub fail_threshold: u32,
    pub recover_threshold: u32,
    pub cooldown_secs: u64,
    pub dry_run: bool,
    pub log_path: String,
    pub disable_lock_path: String,
    pub state_path: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            primary_service: "Wi-Fi".to_string(),
            fallback_service: "iPhone USB".to_string(),
            primary_interface: "en0".to_string(),
            fallback_interface: "en8".to_string(),
            probe_url: "http://connectivitycheck.gstatic.com/generate_204".to_string(),
            expected_content: "".to_string(),
            probe_interval_secs: 5,
            probe_timeout_secs: 5,
            fail_threshold: 3,
            recover_threshold: 3,
            cooldown_secs: 30,
            dry_run: true,
            log_path: "/var/log/netfailover.log".to_string(),
            disable_lock_path: "/var/run/netfailover.disabled".to_string(),
            state_path: "/var/run/netfailover.state".to_string(),
        }
    }
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = fs::read_to_string(path)
            .with_context(|| format!("read config {}", path.display()))?;
        let cfg: Self =
            toml::from_str(&text).with_context(|| format!("parse config {}", path.display()))?;
        cfg.validate()?;
        Ok(cfg)
    }

    fn validate(&self) -> Result<()> {
        anyhow::ensure!(!self.primary_service.is_empty(), "primary_service is empty");
        anyhow::ensure!(!self.fallback_service.is_empty(), "fallback_service is empty");
        anyhow::ensure!(!self.probe_url.is_empty(), "probe_url is empty");
        anyhow::ensure!(self.fail_threshold >= 2, "fail_threshold must be >= 2");
        anyhow::ensure!(self.recover_threshold >= 2, "recover_threshold must be >= 2");
        Ok(())
    }

    pub fn cooldown_ticks(&self) -> u32 {
        let interval = self.probe_interval_secs.max(1);
        ((self.cooldown_secs + interval - 1) / interval).max(1) as u32
    }
}
