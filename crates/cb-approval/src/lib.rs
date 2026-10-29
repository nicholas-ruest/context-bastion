use cb_errors::{BastionError, Result};
use cb_types::{Approval, SessionEvent};
pub fn record(action_id: &str, event: &SessionEvent) -> Result<Approval> {
    if !event.authenticated {
        return Err(BastionError::Authorization(
            "approval event not authenticated".into(),
        ));
    }
    Ok(Approval {
        id: format!("approval-{}-{}", action_id, event.sequence),
        action_id: action_id.into(),
        event_id: event.id.clone(),
        actor: event.source.clone(),
        authenticated: true,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unauthenticated_denied() {
        let e = SessionEvent {
            id: "e".into(),
            session_id: "s".into(),
            sequence: 1,
            role: cb_types::Role::User,
            source: "u".into(),
            content: "approve".into(),
            authenticated: false,
            digest: "d".into(),
        };
        assert!(record("a", &e).is_err());
    }
}
