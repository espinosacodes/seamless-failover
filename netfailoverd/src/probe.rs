use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProbeOutcome {
    pub ok: bool,
    pub detail: String,
}

pub fn http_probe(url: &str, expected_content: &str, timeout_secs: u64) -> ProbeOutcome {
    let timeout = Duration::from_secs(timeout_secs.max(1));
    let agent: ureq::Agent = ureq::AgentBuilder::new()
        .timeout(timeout)
        .redirects(0)
        .build();
    match agent.get(url).call() {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.into_string().unwrap_or_default();
            // A captive portal may return 200 with a login page, so require
            // expected content when configured. Empty expected content means
            // any 2xx response counts.
            let content_ok =
                expected_content.is_empty() || body.contains(expected_content);
            let ok = (200..300).contains(&status) && content_ok;
            ProbeOutcome {
                ok,
                detail: format!("http {status} content_ok={content_ok}"),
            }
        }
        Err(ureq::Error::Status(status, _)) => ProbeOutcome {
            ok: false,
            detail: format!("http status {status}"),
        },
        Err(err) => ProbeOutcome {
            ok: false,
            detail: format!("request error: {err}"),
        },
    }
}
