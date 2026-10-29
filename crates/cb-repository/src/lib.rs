use cb_errors::{BastionError, Result};
use cb_rvf::Archive;
use cb_types::SessionView;
use std::path::{Path, PathBuf};
fn path(root: &Path, id: &str) -> PathBuf {
    root.join(format!("{id}.rvf"))
}
pub fn save(root: &Path, view: &SessionView) -> Result<()> {
    let p = path(root, &view.id);
    let id = Archive::read_all(&p)?.len() as u64 + 1;
    let bytes = serde_json::to_vec(view).map_err(|e| BastionError::Persistence(e.to_string()))?;
    Archive::append(&p, id, &bytes)
}
pub fn load(root: &Path, id: &str) -> Result<SessionView> {
    let all = Archive::read_all(&path(root, id))?;
    let bytes = all
        .last()
        .ok_or_else(|| BastionError::NotFound(id.into()))?
        .1
        .as_slice();
    serde_json::from_slice(bytes).map_err(|e| BastionError::Persistence(e.to_string()))
}
pub fn recover(root: &Path) -> Result<Vec<SessionView>> {
    if !root.exists() {
        return Ok(vec![]);
    }
    let mut out = vec![];
    for e in std::fs::read_dir(root).map_err(|e| BastionError::Persistence(e.to_string()))? {
        let p = e
            .map_err(|e| BastionError::Persistence(e.to_string()))?
            .path();
        if p.extension().and_then(|v| v.to_str()) == Some("rvf") {
            if let Some(id) = p.file_stem().and_then(|v| v.to_str()) {
                out.push(load(root, id)?);
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_session_is_explicit() {
        let root = std::env::temp_dir().join(format!("cb-repository-{}", std::process::id()));
        let error = load(&root, "absent").expect_err("missing");
        assert!(matches!(error, BastionError::NotFound(_)));
    }
}
