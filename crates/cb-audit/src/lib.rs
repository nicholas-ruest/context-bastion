use cb_provenance::ProvenanceGraph;
use cb_types::{Decision, Receipt, Summary};
pub fn correlation_json(
    session: &str,
    summary: &Summary,
    graph: &ProvenanceGraph,
    decision: &Decision,
    receipt: &Receipt,
) -> String {
    serde_json::json!({"session_id":session,"claims":summary.claims.len(),"provenance_root":graph.root,"allowed":decision.allowed,"effect_count":receipt.effect_count,"receipt":receipt.id}).to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contains_session() {
        let s = Summary {
            text: "x".into(),
            claims: vec![],
            compacted_count: 0,
            laundering_markers: 0,
        };
        let g = ProvenanceGraph {
            nodes: 0,
            edges: 0,
            root: "r".into(),
        };
        let d = Decision {
            allowed: true,
            reason: "ok".into(),
            policy_id: "p".into(),
        };
        let r = Receipt {
            id: "r".into(),
            action_id: "a".into(),
            effect_count: 1,
            digest: "d".into(),
        };
        assert!(correlation_json("s", &s, &g, &d, &r).contains("\"session_id\":\"s\""));
    }
}
