use cb_auth::{authenticate, Principal};
use cb_errors::Result;
#[derive(Clone, Debug)]
pub struct RequestContext {
    pub principal: Principal,
    pub correlation_id: String,
}
pub fn context(auth: Option<&str>, incoming: Option<&str>) -> Result<RequestContext> {
    let principal = authenticate(auth)?;
    let correlation_id = incoming
        .filter(|v| !v.trim().is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    Ok(RequestContext {
        principal,
        correlation_id,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creates_id() {
        assert!(!context(Some("Bearer operator-token"), None)
            .unwrap()
            .correlation_id
            .is_empty());
    }
}
