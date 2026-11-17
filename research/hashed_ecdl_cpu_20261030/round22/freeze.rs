use pencil_native_canonical_export::{load, protocol_hash, validate_protocol, write_exclusive};
use serde_json::json;
use std::path::Path;
fn main() -> Result<(), String> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 6 || a[5] != "--before-data" {
        return Err("usage: freeze DRAFT NEW_OUTPUT UTC_TIMESTAMP COMMIT --before-data".into());
    }
    let path = Path::new(&a[1]);
    let mut p = load(path)?;
    if p["freeze"]["status"] != "draft" {
        return Err("only an unexecuted draft may be frozen".into());
    }
    validate_protocol(&p, path.parent())?;
    p["freeze"] = json!({"status":"frozen","protocol_sha256":protocol_hash(&p)?,"commit":a[4],"frozen_at":a[3],"filed_before_affected_data":true});
    validate_protocol(&p, path.parent())?;
    write_exclusive(Path::new(&a[2]), &p)?;
    println!(
        "{}",
        json!({"protocol_sha256":p["freeze"]["protocol_sha256"],"launch_authorized":false})
    );
    Ok(())
}
