use cb_types::Claim;
use sha2::{Digest, Sha256};
pub const DIMENSION: usize = 16;
pub fn embed(text: &str) -> Vec<f32> {
    let d = Sha256::digest(text.as_bytes());
    (0..DIMENSION).map(|i| f32::from(d[i]) / 255.0).collect()
}
pub fn claim_vector(claim: &Claim) -> Vec<f32> {
    embed(&claim.text)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stable() {
        assert_eq!(embed("x"), embed("x"));
    }
}
