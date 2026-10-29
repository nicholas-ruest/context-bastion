use cb_errors::Result;
use cb_provenance::authentic_approval;
use cb_types::{ActionRequest, Decision, Receipt, SessionEvent};
use std::collections::HashSet;
#[derive(Default)]
pub struct ActionGate {
    executed: HashSet<String>,
}
impl ActionGate {
    pub fn decide(
        &self,
        action: &ActionRequest,
        events: &[SessionEvent],
        markers: usize,
    ) -> Result<Decision> {
        let approved = authentic_approval(events, action.approval_event_id.as_deref()).is_ok();
        cb_regorus::evaluate(action.risk != "low", approved, markers)
    }
    pub fn execute(&mut self, action: &ActionRequest, decision: &Decision) -> Receipt {
        let fresh = decision.allowed && self.executed.insert(action.id.clone());
        Receipt {
            id: format!("receipt-{}", action.id),
            action_id: action.id.clone(),
            effect_count: u32::from(fresh),
            digest: format!("{}:{}:{}", action.verb, action.target, u8::from(fresh)),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn denial_has_no_effect() {
        let mut g = ActionGate::default();
        let a = ActionRequest {
            id: "a".into(),
            session_id: "s".into(),
            verb: "deploy".into(),
            target: "x".into(),
            risk: "high".into(),
            approval_event_id: None,
        };
        let d = Decision {
            allowed: false,
            reason: "no".into(),
            policy_id: "p".into(),
        };
        assert_eq!(g.execute(&a, &d).effect_count, 0);
    }
}
