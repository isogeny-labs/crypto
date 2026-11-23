//! Write-once output and exact resume identities for the conjugate-pair pilot.
use serde_json::Value;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

pub fn chart_stream_seed(offset: u64, curve: usize, target: &str, arm: &str) -> u64 {
    0x9150u64
        .wrapping_add(offset)
        .wrapping_add((curve as u64).wrapping_mul(31))
        .wrapping_add(target.len() as u64)
        .wrapping_add(if arm == "conj" { 0 } else { 7 })
}

pub fn key(v: &Value) -> Result<String, String> {
    let number = |name: &str| {
        v[name]
            .as_u64()
            .ok_or_else(|| format!("missing integer {name}"))
    };
    let string = |name: &str| {
        v[name]
            .as_str()
            .ok_or_else(|| format!("missing string {name}"))
    };
    let transform = v["transform"].as_bool().ok_or("missing transform")?;
    let md = match v.get("macaulay_degree") {
        Some(n) => n.as_u64().ok_or("invalid macaulay_degree")?,
        None => 29, // historical R1/R2/R3 records
    };
    let seed = match v.get("chart_seed") {
        Some(n) => n.as_u64().ok_or("invalid chart_seed")?,
        None => 0, // historical stream
    };
    Ok(serde_json::json!([
        number("p")?,
        number("curve")?,
        string("arm")?,
        string("target")?,
        string("set")?,
        transform,
        md,
        if transform { seed } else { 0 }
    ])
    .to_string())
}

/// Parse completely before opening an output. Refuse malformed/truncated data,
/// ambiguous duplicates, and accidental replacement of an existing output.
pub fn open_output(out: &Path, resume: Option<&Path>) -> io::Result<(File, HashSet<String>)> {
    let invalid = |e: String| io::Error::new(io::ErrorKind::InvalidData, e);
    let mut keys = HashSet::new();
    let mut raw = Vec::new();
    if let Some(path) = resume {
        raw = std::fs::read(path)?;
        if !raw.is_empty() && !raw.ends_with(b"\n") {
            return Err(invalid(
                "resume file has an unterminated final record".into(),
            ));
        }
        let text = std::str::from_utf8(&raw).map_err(|e| invalid(e.to_string()))?;
        for line in text.lines() {
            let v: Value = serde_json::from_str(line).map_err(|e| invalid(e.to_string()))?;
            if v["kind"] == "inf" && !keys.insert(key(&v).map_err(invalid)?) {
                return Err(invalid("duplicate solve identity in resume input".into()));
            }
        }
        // Canonical paths catch different spellings and symlinks, before writes.
        if out.exists() && std::fs::canonicalize(out)? == std::fs::canonicalize(path)? {
            return Ok((OpenOptions::new().append(true).open(out)?, keys));
        }
    }
    let mut file = OpenOptions::new().write(true).create_new(true).open(out)?;
    file.write_all(&raw)?;
    file.flush()?;
    Ok((file, keys))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Dir(std::path::PathBuf);
    impl Dir {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "conjpair-resume-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&p).unwrap();
            Self(p)
        }
    }
    impl Drop for Dir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    fn row() -> Value {
        serde_json::json!({"kind":"inf","p":29,"curve":1,"arm":"conj","target":"common0","set":"R2","transform":true})
    }
    #[test]
    fn identity_separates_field_degree_and_chart() {
        let old = row();
        let mut explicit = old.clone();
        explicit["macaulay_degree"] = 29.into();
        explicit["chart_seed"] = 0.into();
        assert_eq!(key(&old), key(&explicit));
        for (field, n) in [("p", 37), ("macaulay_degree", 30), ("chart_seed", 1)] {
            let mut v = old.clone();
            v[field] = n.into();
            assert_ne!(key(&old), key(&v));
        }
        let mut bad = old.clone();
        bad["chart_seed"] = "1".into();
        assert!(key(&bad).is_err());
    }

    #[test]
    fn chart_seed_zero_preserves_the_legacy_stream() {
        assert_eq!(chart_stream_seed(0, 1, "common0", "conj"), 0x9150 + 31 + 7);
        assert_ne!(
            chart_stream_seed(1, 1, "common0", "conj"),
            chart_stream_seed(0, 1, "common0", "conj")
        );
        let _ = chart_stream_seed(u64::MAX, usize::MAX, "common0", "std");
    }
    #[test]
    fn same_file_resume_preserves_bytes_and_appends() {
        let dir = Dir::new();
        let p = dir.0.join("rows.jsonl");
        let before = format!("{}\n", row());
        std::fs::write(&p, &before).unwrap();
        let (mut f, keys) = open_output(&p, Some(&p)).unwrap();
        assert_eq!(keys.len(), 1);
        writeln!(f, "{{\"kind\":\"control\"}}").unwrap();
        drop(f);
        assert!(std::fs::read_to_string(&p).unwrap().starts_with(&before));
    }
    #[test]
    fn separate_resume_copies_history_and_refuses_overwrite() {
        let dir = Dir::new();
        let p = dir.0.join("old");
        let q = dir.0.join("new");
        let before = format!("{}\n", row());
        std::fs::write(&p, &before).unwrap();
        open_output(&q, Some(&p)).unwrap();
        assert_eq!(std::fs::read(&p).unwrap(), std::fs::read(&q).unwrap());
        assert!(open_output(&q, None).is_err());
        assert!(open_output(&q, Some(&p)).is_err());
    }
    #[test]
    fn corrupt_or_duplicate_resume_is_never_truncated() {
        let dir = Dir::new();
        let p = dir.0.join("rows");
        for text in [
            format!("{}", row()),
            "not json\n".into(),
            format!("{}\n{}\n", row(), row()),
        ] {
            std::fs::write(&p, &text).unwrap();
            assert!(open_output(&p, Some(&p)).is_err());
            assert_eq!(std::fs::read_to_string(&p).unwrap(), text);
        }
        assert!(open_output(&p, Some(&dir.0.join("missing"))).is_err());
    }
}
