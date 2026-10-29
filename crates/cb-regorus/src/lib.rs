use cb_errors::{BastionError, Result};
use cb_types::Decision;
use regorus::{Engine, Value};
use serde_json::json;
const POLICY: &str = r#"package context_bastion
default allow = false
allow if { not input.consequential }
allow if { input.consequential; input.has_authentic_approval }
"#;
pub fn evaluate(consequential: bool, approved: bool, markers: usize) -> Result<Decision> {
    let mut engine = Engine::new();
    engine
        .add_policy("context-bastion.rego".into(), POLICY.into())
        .map_err(|e| BastionError::Dependency(format!("Regorus policy: {e}")))?;
    let input=Value::from_json_str(&json!({"consequential":consequential,"has_authentic_approval":approved,"laundering_markers":markers}).to_string()).map_err(|e|BastionError::Dependency(format!("Regorus input: {e}")))?;
    engine.set_input(input);
    let value = engine
        .eval_rule("data.context_bastion.allow".into())
        .map_err(|e| BastionError::Dependency(format!("Regorus eval: {e}")))?;
    let allowed = value.as_bool().copied().unwrap_or(false);
    Ok(Decision {
        allowed,
        reason: if allowed {
            "policy permits typed source".into()
        } else {
            "policy denies unresolved or laundered approval".into()
        },
        policy_id: "microsoft-regorus/context-bastion-v1".into(),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rego_denies_forged() {
        assert!(!evaluate(true, false, 1).unwrap().allowed);
    }
}
