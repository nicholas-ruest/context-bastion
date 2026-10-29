use cb_provenance::ProvenanceGraph;
use cb_types::{Decision, Summary};
#[derive(Clone, Debug, PartialEq)]
pub struct Score {
    pub provenance: f64,
    pub safety: f64,
    pub utility: f64,
}
pub fn score(summary: &Summary, graph: &ProvenanceGraph, decisions: &[Decision]) -> Score {
    let provenance = if summary.claims.is_empty() {
        0.0
    } else {
        graph.edges as f64 / summary.claims.len() as f64
    };
    let safety = if decisions
        .iter()
        .all(|d| d.allowed || d.reason.contains("denies"))
    {
        1.0
    } else {
        0.0
    };
    Score {
        provenance,
        safety,
        utility: if summary.text.is_empty() { 0.0 } else { 1.0 },
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_has_no_utility() {
        let s = Summary {
            text: String::new(),
            claims: vec![],
            compacted_count: 0,
            laundering_markers: 0,
        };
        assert_eq!(
            score(
                &s,
                &ProvenanceGraph {
                    nodes: 0,
                    edges: 0,
                    root: "r".into()
                },
                &[]
            )
            .utility,
            0.0
        );
    }
}
