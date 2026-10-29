use thiserror::Error;
#[derive(Debug, Error)]
pub enum BastionError {
    #[error("validation: {0}")]
    Validation(String),
    #[error("authorization: {0}")]
    Authorization(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("dependency: {0}")]
    Dependency(String),
    #[error("persistence: {0}")]
    Persistence(String),
}
pub type Result<T> = std::result::Result<T, BastionError>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn errors_keep_contract_category() {
        assert_eq!(
            BastionError::Authorization("denied".into()).to_string(),
            "authorization: denied"
        );
    }
}
