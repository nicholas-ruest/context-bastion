use cb_errors::{BastionError, Result};
use cb_types::Analogue;
use ruvector_core::{
    types::{DbOptions, DistanceMetric, SearchQuery, VectorEntry},
    VectorDB,
};
use std::collections::HashMap;
pub struct ClaimIndex {
    db: VectorDB,
    len: usize,
}
impl ClaimIndex {
    pub fn new() -> Result<Self> {
        let db = VectorDB::new(DbOptions {
            dimensions: cb_embedding::DIMENSION,
            distance_metric: DistanceMetric::Euclidean,
            storage_path: "memory://context-bastion".into(),
            hnsw_config: None,
            quantization: None,
        })
        .map_err(|e| BastionError::Dependency(format!("RuVector create: {e}")))?;
        Ok(Self { db, len: 0 })
    }
    pub fn insert(&mut self, id: &str, vector: Vec<f32>) -> Result<()> {
        let mut m = HashMap::new();
        m.insert("kind".into(), serde_json::json!("context-claim"));
        self.db
            .insert(VectorEntry {
                id: Some(id.into()),
                vector,
                metadata: Some(m),
            })
            .map_err(|e| BastionError::Dependency(format!("RuVector insert: {e}")))?;
        self.len += 1;
        Ok(())
    }
    pub fn search(&self, vector: &[f32], k: usize) -> Result<Vec<Analogue>> {
        if self.len == 0 {
            return Ok(vec![]);
        }
        self.db
            .search(SearchQuery {
                vector: vector.to_vec(),
                k: k.min(self.len),
                filter: None,
                ef_search: None,
            })
            .map_err(|e| BastionError::Dependency(format!("RuVector search: {e}")))
            .map(|v| {
                v.into_iter()
                    .map(|h| Analogue {
                        claim_id: h.id,
                        distance: f64::from(h.score),
                    })
                    .collect()
            })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_index_search() {
        let mut i = ClaimIndex::new().unwrap();
        i.insert("p", vec![0.0; cb_embedding::DIMENSION]).unwrap();
        assert_eq!(
            i.search(&[0.1; cb_embedding::DIMENSION], 1).unwrap()[0].claim_id,
            "p"
        );
    }
}
