//! The curve catalog: the rows of `docs/curves/registry.json` that
//! `docs/curves/cover-catalog.json` names.
//!
//! The registry grows whenever research output is swept into it, and a file
//! derived from its whole bytes goes stale each time.  The native catalogs
//! (`jacobian-transfers.json`, `traits.json`) are derived from the catalog
//! instead.  [`restrict`] returns the named rows as a registry document in a
//! canonical form (registry order, sorted keys, compact), and the digest of
//! that text is the `catalog_rows_sha256` those files record.  A registry row
//! the manifest does not name changes nothing; an edited named row, or a named
//! slug the registry lacks, is an error or a changed digest.

use serde_json::{json, Value};
use std::collections::HashSet;

/// Repository-relative path of the manifest.
pub const MANIFEST_PATH: &str = "docs/curves/cover-catalog.json";
/// The manifest's `schema_version`.
pub const MANIFEST_SCHEMA: &str = "curve-cover-catalog/v1";

/// The registry rows the manifest names, in registry order, as registry
/// JSON text (`{"curves": [...], "schema_version": ...}`, sorted keys,
/// compact).  Hash this text with [`rows_sha256`].
pub fn restrict(registry_text: &str, manifest_text: &str) -> Result<String, String> {
    let manifest: Value =
        serde_json::from_str(manifest_text).map_err(|e| format!("{MANIFEST_PATH}: {e}"))?;
    if manifest["schema_version"] != MANIFEST_SCHEMA {
        return Err(format!("{MANIFEST_PATH}: unsupported schema"));
    }
    let slugs = manifest["slugs"]
        .as_array()
        .ok_or_else(|| format!("{MANIFEST_PATH}: missing slugs"))?;
    let mut wanted = HashSet::new();
    for slug in slugs {
        let slug = slug
            .as_str()
            .ok_or_else(|| format!("{MANIFEST_PATH}: a slug is not a string"))?;
        if !wanted.insert(slug) {
            return Err(format!("{MANIFEST_PATH}: duplicate slug {slug}"));
        }
    }
    if manifest["count"].as_u64() != Some(wanted.len() as u64) {
        return Err(format!("{MANIFEST_PATH}: count does not match its slugs"));
    }
    let registry: Value =
        serde_json::from_str(registry_text).map_err(|e| format!("registry JSON: {e}"))?;
    let rows: Vec<Value> = registry["curves"]
        .as_array()
        .ok_or("registry has no curves")?
        .iter()
        .filter(|row| row["slug"].as_str().is_some_and(|s| wanted.contains(s)))
        .cloned()
        .collect();
    if rows.len() != wanted.len() {
        let held: HashSet<&str> = rows.iter().filter_map(|r| r["slug"].as_str()).collect();
        let mut missing: Vec<&str> = wanted
            .iter()
            .copied()
            .filter(|s| !held.contains(s))
            .collect();
        missing.sort_unstable();
        missing.truncate(5);
        return Err(format!(
            "{MANIFEST_PATH} names slugs the registry does not hold: {missing:?}"
        ));
    }
    Ok(json!({"schema_version": registry["schema_version"], "curves": rows}).to_string())
}

/// Lower-case hex SHA-256 of [`restrict`]'s text: the `catalog_rows_sha256`.
pub fn rows_sha256(restricted: &str) -> String {
    crate::hash::sha256::sha256(restricted.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry(slugs: &[&str]) -> String {
        let curves: Vec<Value> = slugs
            .iter()
            .map(|s| json!({"slug": s, "order": "6", "model_json": "{}"}))
            .collect();
        json!({"schema_version": 1, "curves": curves}).to_string()
    }

    fn manifest(slugs: &[&str]) -> String {
        json!({"schema_version": MANIFEST_SCHEMA, "count": slugs.len(), "slugs": slugs}).to_string()
    }

    #[test]
    fn rows_outside_the_catalog_do_not_change_the_digest() {
        let m = manifest(&["a", "c"]);
        let before = restrict(&registry(&["a", "c"]), &m).unwrap();
        // A census row anywhere in the registry leaves the rows, and so the digest, alone.
        let after = restrict(&registry(&["x", "a", "y", "c", "z"]), &m).unwrap();
        assert_eq!(before, after);
        assert_eq!(rows_sha256(&before), rows_sha256(&after));
    }

    #[test]
    fn an_edited_catalog_row_changes_the_digest() {
        let m = manifest(&["a", "c"]);
        let plain = restrict(&registry(&["a", "c"]), &m).unwrap();
        let edited = registry(&["a", "c"]).replace("\"order\":\"6\"", "\"order\":\"7\"");
        let changed = restrict(&edited, &m).unwrap();
        assert_ne!(rows_sha256(&plain), rows_sha256(&changed));
    }

    #[test]
    fn membership_is_part_of_the_digest() {
        let reg = registry(&["a", "b", "c"]);
        let ab = restrict(&reg, &manifest(&["a", "b"])).unwrap();
        let abc = restrict(&reg, &manifest(&["a", "b", "c"])).unwrap();
        assert_ne!(rows_sha256(&ab), rows_sha256(&abc));
    }

    #[test]
    fn a_bad_manifest_is_refused() {
        let reg = registry(&["a", "b"]);
        assert!(restrict(&reg, &manifest(&["a", "absent"])).is_err());
        assert!(restrict(&reg, &manifest(&["a", "a"])).is_err());
        let miscount = json!({"schema_version": MANIFEST_SCHEMA, "count": 3, "slugs": ["a"]});
        assert!(restrict(&reg, &miscount.to_string()).is_err());
        let wrong = json!({"schema_version": "other", "count": 1, "slugs": ["a"]});
        assert!(restrict(&reg, &wrong.to_string()).is_err());
    }
}
