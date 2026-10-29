use cb_errors::{BastionError, Result};
use cb_types::{Claim, SessionEvent, Summary};
pub fn summarize(events: &[SessionEvent], markers: usize) -> Result<Summary> {
    if events.is_empty() {
        return Err(BastionError::Validation(
            "cannot summarize empty session".into(),
        ));
    }
    let claims = events
        .iter()
        .map(|e| Claim {
            id: format!("claim-{}", e.id),
            text: e.content.clone(),
            event_ids: vec![e.id.clone()],
            roles: vec![e.role.clone()],
            trusted: e.authenticated,
        })
        .collect::<Vec<_>>();
    let text = claims
        .iter()
        .map(|c| c.text.as_str())
        .collect::<Vec<_>>()
        .join(" | ");
    Ok(Summary {
        text,
        claims,
        compacted_count: events.len(),
        laundering_markers: markers,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_rejected() {
        assert!(summarize(&[], 0).is_err());
    }
}
