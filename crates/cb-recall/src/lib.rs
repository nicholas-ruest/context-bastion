use cb_errors::Result;
use cb_ruvector::ClaimIndex;
use cb_types::{Analogue, Claim};
pub fn index_and_recall(claims: &[Claim], query: &str) -> Result<Vec<Analogue>> {
    let mut index = ClaimIndex::new()?;
    for claim in claims {
        index.insert(&claim.id, cb_embedding::claim_vector(claim))?;
    }
    index.search(&cb_embedding::embed(query), 3)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_is_empty() {
        assert!(index_and_recall(&[], "x").unwrap().is_empty());
    }
}
