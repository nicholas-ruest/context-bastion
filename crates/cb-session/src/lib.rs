use cb_errors::{BastionError, Result};
use cb_types::SessionEvent;
#[derive(Clone, Debug)]
pub struct Session {
    pub id: String,
    pub name: String,
    events: Vec<SessionEvent>,
}
impl Session {
    pub fn new(id: String, name: String) -> Result<Self> {
        if name.trim().len() < 3 {
            return Err(BastionError::Validation("session name too short".into()));
        }
        Ok(Self {
            id,
            name,
            events: vec![],
        })
    }
    pub fn append(&mut self, event: SessionEvent) -> Result<()> {
        if event.session_id != self.id || event.sequence != self.events.len() as u64 + 1 {
            return Err(BastionError::Validation(
                "event sequence or session mismatch".into(),
            ));
        }
        self.events.push(event);
        Ok(())
    }
    pub fn events(&self) -> &[SessionEvent] {
        &self.events
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_name() {
        assert!(Session::new("s".into(), "x".into()).is_err());
    }
}
