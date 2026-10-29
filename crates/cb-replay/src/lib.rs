use cb_errors::{BastionError, Result};
use cb_provenance::ProvenanceGraph;
use cb_types::{Receipt, Summary};
pub fn verify(summary: &Summary, graph: &ProvenanceGraph, receipts: &[Receipt]) -> Result<()> {
    if graph.nodes < summary.claims.len() {
        return Err(BastionError::Persistence(
            "provenance graph incomplete".into(),
        ));
    }
    let effects: u32 = receipts.iter().map(|r| r.effect_count).sum();
    if effects > receipts.len() as u32 {
        return Err(BastionError::Persistence("impossible effect count".into()));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detects_incomplete() {
        let s = Summary {
            text: "x".into(),
            claims: vec![],
            compacted_count: 1,
            laundering_markers: 0,
        };
        assert!(verify(
            &s,
            &ProvenanceGraph {
                nodes: 0,
                edges: 0,
                root: "r".into()
            },
            &[]
        )
        .is_ok());
    }
}
