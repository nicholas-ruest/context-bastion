use cb_errors::{BastionError, Result};
#[derive(Clone, Debug)]
pub struct Config {
    pub bind: String,
    pub data_dir: String,
    pub namespace: String,
    pub test_mode: bool,
}
impl Config {
    pub fn from_env() -> Result<Self> {
        let data_dir =
            std::env::var("BASTION_DATA_DIR").unwrap_or_else(|_| "/tmp/context-bastion".into());
        if data_dir.trim().is_empty() {
            return Err(BastionError::Validation("empty data directory".into()));
        }
        Ok(Self {
            bind: std::env::var("BASTION_BIND").unwrap_or_else(|_| "127.0.0.1:8787".into()),
            data_dir,
            namespace: std::env::var("BASTION_NAMESPACE")
                .unwrap_or_else(|_| "dream-machine-test".into()),
            test_mode: std::env::var("BASTION_TEST_MODE").is_ok_and(|v| v == "1"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults_are_runnable() {
        let config = Config::from_env().expect("defaults");
        assert!(!config.bind.is_empty());
        assert!(!config.data_dir.is_empty());
        assert!(!config.namespace.is_empty());
    }
}
