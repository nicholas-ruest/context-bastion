use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
    Tool,
    System,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionEvent {
    pub id: String,
    pub session_id: String,
    pub sequence: u64,
    pub role: Role,
    pub source: String,
    pub content: String,
    pub authenticated: bool,
    pub digest: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claim {
    pub id: String,
    pub text: String,
    pub event_ids: Vec<String>,
    pub roles: Vec<Role>,
    pub trusted: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Summary {
    pub text: String,
    pub claims: Vec<Claim>,
    pub compacted_count: usize,
    pub laundering_markers: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Approval {
    pub id: String,
    pub action_id: String,
    pub event_id: String,
    pub actor: String,
    pub authenticated: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActionRequest {
    pub id: String,
    pub session_id: String,
    pub verb: String,
    pub target: String,
    pub risk: String,
    pub approval_event_id: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Decision {
    pub allowed: bool,
    pub reason: String,
    pub policy_id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Receipt {
    pub id: String,
    pub action_id: String,
    pub effect_count: u32,
    pub digest: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Analogue {
    pub claim_id: String,
    pub distance: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionView {
    pub id: String,
    pub name: String,
    pub events: Vec<SessionEvent>,
    pub summary: Option<Summary>,
    pub decisions: Vec<Decision>,
    pub receipts: Vec<Receipt>,
    pub analogues: Vec<Analogue>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub name: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddEventRequest {
    pub role: Role,
    pub source: String,
    pub content: String,
    pub authenticated: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub verb: String,
    pub target: String,
    pub risk: String,
    pub approval_event_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn role_serialization_is_stable() {
        assert_eq!(serde_json::to_string(&Role::Tool).unwrap(), "\"tool\"");
    }
}
