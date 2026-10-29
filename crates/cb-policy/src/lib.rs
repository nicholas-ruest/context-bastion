use cb_types::ActionRequest;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyInput {
    pub consequential: bool,
    pub has_authentic_approval: bool,
    pub laundering_markers: usize,
}
pub fn classify(action: &ActionRequest, approval: bool, markers: usize) -> PolicyInput {
    PolicyInput {
        consequential: action.risk != "low",
        has_authentic_approval: approval,
        laundering_markers: markers,
    }
}
pub fn expected_allow(input: &PolicyInput) -> bool {
    !input.consequential || input.has_authentic_approval
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn authentic_approval_recovers() {
        assert!(expected_allow(&PolicyInput {
            consequential: true,
            has_authentic_approval: true,
            laundering_markers: 1
        }));
    }
}
