use cb_errors::{BastionError, Result};
use cb_types::{Role, SessionEvent};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompactionPlan {
    pub retained: Vec<SessionEvent>,
    pub laundered_markers: usize,
}
pub fn plan(events: &[SessionEvent], max: usize) -> Result<CompactionPlan> {
    if max == 0 {
        return Err(BastionError::Validation("compaction budget is zero".into()));
    }
    let retained = events
        .iter()
        .rev()
        .take(max)
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    let laundered_markers = events
        .iter()
        .filter(|e| matches!(e.role, Role::Tool) && e.content.contains("[User]"))
        .count();
    Ok(CompactionPlan {
        retained,
        laundered_markers,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zero_rejected() {
        assert!(plan(&[], 0).is_err());
    }
}
