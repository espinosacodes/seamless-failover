#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Healthy,
    Degraded,
    Failover,
    Recovering,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    None,
    SwitchToFallback,
    RevertToPrimary,
}

pub struct StateMachine {
    pub state: State,
    fail_threshold: u32,
    recover_threshold: u32,
    cooldown_ticks: u32,
    consecutive_failures: u32,
    consecutive_successes: u32,
    cooldown_remaining: u32,
}

impl StateMachine {
    pub fn new(fail_threshold: u32, recover_threshold: u32, cooldown_ticks: u32) -> Self {
        Self {
            state: State::Healthy,
            fail_threshold: fail_threshold.max(2),
            recover_threshold: recover_threshold.max(2),
            cooldown_ticks: cooldown_ticks.max(1),
            consecutive_failures: 0,
            consecutive_successes: 0,
            cooldown_remaining: 0,
        }
    }

    pub fn tick(&mut self, primary_ok: bool, fallback_ok: bool) -> Action {
        match self.state {
            State::Healthy => {
                if primary_ok {
                    self.consecutive_failures = 0;
                    Action::None
                } else {
                    self.consecutive_failures += 1;
                    self.consecutive_successes = 0;
                    if self.consecutive_failures >= self.fail_threshold {
                        self.state = State::Degraded;
                    }
                    Action::None
                }
            }
            State::Degraded => {
                if primary_ok {
                    self.state = State::Healthy;
                    self.consecutive_failures = 0;
                    self.consecutive_successes = 0;
                    Action::None
                } else if fallback_ok {
                    self.state = State::Failover;
                    self.consecutive_successes = 0;
                    Action::SwitchToFallback
                } else {
                    // Stay degraded, do not switch into a black hole.
                    Action::None
                }
            }
            State::Failover => {
                if primary_ok {
                    self.consecutive_successes += 1;
                    if self.consecutive_successes >= self.recover_threshold {
                        self.state = State::Recovering;
                        self.cooldown_remaining = self.cooldown_ticks;
                    }
                    Action::None
                } else {
                    self.consecutive_successes = 0;
                    Action::None
                }
            }
            State::Recovering => {
                if !primary_ok {
                    self.state = State::Failover;
                    self.consecutive_successes = 0;
                    self.cooldown_remaining = 0;
                    Action::None
                } else {
                    self.cooldown_remaining = self.cooldown_remaining.saturating_sub(1);
                    if self.cooldown_remaining == 0 {
                        self.state = State::Healthy;
                        self.consecutive_failures = 0;
                        self.consecutive_successes = 0;
                        Action::RevertToPrimary
                    } else {
                        Action::None
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn machine() -> StateMachine {
        StateMachine::new(3, 3, 6)
    }

    #[test]
    fn flapping_noise_causes_no_switch() {
        let mut m = machine();
        // Alternate success and failure forever. Counters never reach threshold.
        for _ in 0..40 {
            assert_eq!(m.tick(true, true), Action::None);
            assert_eq!(m.tick(false, true), Action::None);
        }
        assert_eq!(m.state, State::Healthy);
    }

    #[test]
    fn sustained_failure_switches_then_recovers() {
        let mut m = machine();
        assert_eq!(m.tick(false, true), Action::None);
        assert_eq!(m.tick(false, true), Action::None);
        assert_eq!(m.tick(false, true), Action::None);
        assert_eq!(m.state, State::Degraded);
        assert_eq!(m.tick(false, true), Action::SwitchToFallback);
        assert_eq!(m.state, State::Failover);

        // Three primary successes move to recovering, no revert yet.
        assert_eq!(m.tick(true, true), Action::None);
        assert_eq!(m.tick(true, true), Action::None);
        assert_eq!(m.tick(true, true), Action::None);
        assert_eq!(m.state, State::Recovering);

        // Cooldown ticks then revert.
        for _ in 0..5 {
            assert_eq!(m.tick(true, true), Action::None);
        }
        assert_eq!(m.tick(true, true), Action::RevertToPrimary);
        assert_eq!(m.state, State::Healthy);
    }

    #[test]
    fn no_fallback_means_no_switch() {
        let mut m = machine();
        for _ in 0..6 {
            assert_eq!(m.tick(false, false), Action::None);
        }
        assert_eq!(m.state, State::Degraded);
    }

    #[test]
    fn recovering_failure_returns_to_failover() {
        let mut m = machine();
        for _ in 0..3 {
            m.tick(false, true);
        }
        m.tick(false, true);
        assert_eq!(m.state, State::Failover);
        for _ in 0..3 {
            m.tick(true, true);
        }
        assert_eq!(m.state, State::Recovering);
        assert_eq!(m.tick(false, true), Action::None);
        assert_eq!(m.state, State::Failover);
    }
}
