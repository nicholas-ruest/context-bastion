use cb_errors::{BastionError, Result};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Principal {
    pub subject: String,
    pub roles: Vec<String>,
}
pub fn authenticate(value: Option<&str>) -> Result<Principal> {
    let token = value.ok_or_else(|| BastionError::Authorization("missing bearer token".into()))?;
    if token != "Bearer operator-token" {
        return Err(BastionError::Authorization("invalid bearer token".into()));
    }
    Ok(Principal {
        subject: "operator".into(),
        roles: vec!["session:write".into(), "action:approve".into()],
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_unknown() {
        assert!(authenticate(Some("Bearer forged")).is_err());
    }
}
