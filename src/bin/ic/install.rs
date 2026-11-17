//! Locate installed data files relative to the `ic` executable.
//!
//! The macOS package installs binaries in `<prefix>/bin` and data in
//! `<prefix>/share/cryptocli`. A source checkout keeps the same relative
//! layout under the repository root. Resolving in one place keeps `ic fixed`
//! and `ic ecc2k130` from drifting apart when the install layout changes.
use std::path::PathBuf;

/// Find `relative` under the installed share tree, an extracted archive, or
/// the source checkout, in that order.
pub fn share_file(relative: &str) -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("IC_SHARE_DIR") {
        let candidate = PathBuf::from(path).join(relative);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    let mut candidates = Vec::new();
    if let Ok(executable) = std::env::current_exe() {
        if let Some(directory) = executable.parent() {
            candidates.push(directory.join("../share/cryptocli").join(relative));
            candidates.push(directory.join("share/cryptocli").join(relative));
            candidates.push(directory.join(relative));
        }
    }
    candidates.push(source_path(relative));
    candidates.into_iter().find(|path| path.is_file())
}

/// Like [`share_file`], but falls back to the source path so callers can
/// produce a useful error rather than an `Option`-shaped one.
pub fn share_file_or_source(relative: &str) -> PathBuf {
    share_file(relative).unwrap_or_else(|| source_path(relative))
}

fn source_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_fallback_finds_the_ecc2k130_profile() {
        let path = share_file("docs/ic/params/ecc2k130-fixed.json");
        assert!(path.is_some(), "source checkout should provide the profile");
        assert!(path.unwrap().is_file());
    }

    #[test]
    fn missing_files_are_none() {
        assert!(share_file("docs/ic/params/does-not-exist.json").is_none());
    }
}
