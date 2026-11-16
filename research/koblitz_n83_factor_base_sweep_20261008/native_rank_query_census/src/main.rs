//! Exact necessary rank-query screen over retained N83 manifest claims.
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

const CATALOG_SHA256: &str = "1117ead0dfcc9164a73d83d5ca709ec538528f7042e070ca8d72a5e188487ade";
const D18_SHA256: &str = "1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce";
const HISTORICAL_SHA256: &str = "3b0d9424a2809ee865b67614460a348bf70ae884d426b1732296638cec052276";
const PRIMARY_ORDER: &str = "2417851639230796216685689";
const DIAGNOSTIC_ORDER: &str = "8569786107849059";
const PRIMARY_CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";

fn checked_json(path: &Path, digest: &str) -> Result<Value, String> {
    let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let actual = format!("{:x}", Sha256::digest(&bytes));
    if actual != digest {
        return Err(format!(
            "{}: source SHA-256 changed: {actual}",
            path.display()
        ));
    }
    serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display()))
}

fn string<'a>(value: &'a Value, field: &str) -> Result<&'a str, String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing {field}"))
}
fn number(value: &Value, field: &str) -> Result<u64, String> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing {field}"))
}
fn big(s: &str) -> BigUint {
    BigUint::parse_bytes(s.as_bytes(), 10).expect("fixed positive order")
}
fn ceil_div(n: &BigUint, d: &BigUint) -> BigUint {
    (n + d - 1u8) / d
}
fn multiset_count(base_points: u64, summands: u64) -> BigUint {
    let mut result = BigUint::from(1u8);
    for i in 1..=summands {
        result = result * BigUint::from(base_points + i - 1) / BigUint::from(i);
    }
    result
}
fn query_floor(
    points: u64,
    columns: u64,
    summands: u64,
    order: &BigUint,
) -> (BigUint, BigUint, BigUint) {
    let multisets = multiset_count(points, summands);
    let capped = std::cmp::min(multisets.clone(), order.clone());
    let rank = BigUint::from(columns);
    let threshold = std::cmp::max(
        rank.clone(),
        ceil_div(&(&rank * order), &(BigUint::from(2u8) * &capped)),
    );
    (multisets, capped, threshold)
}
fn threshold_check(floor: &BigUint, columns: u64, capped: &BigUint, order: &BigUint) -> bool {
    let k = BigUint::from(columns);
    let q_before = floor - 1u8;
    let bound_half_before = q_before < k || BigUint::from(2u8) * &q_before * capped < &k * order;
    let bound_half_at = floor >= &k && BigUint::from(2u8) * floor * capped >= &k * order;
    bound_half_before && bound_half_at
}

fn run(root: &Path) -> Result<Value, String> {
    let catalog = checked_json(
        &root.join("../verification/live-s3-manifest-census-20261025/evidence-01/catalog.json"),
        CATALOG_SHA256,
    )?;
    let d18 = checked_json(
        &root.join("../verification/d18-full-orbit-20261025/manifest.json"),
        D18_SHA256,
    )?;
    let historical = checked_json(
        &root.join("../pilot-01/rank-query-screen.json"),
        HISTORICAL_SHA256,
    )?;
    if number(&catalog, "content_addressed_objects")? != 77
        || catalog["curve_arm_object_counts"] != json!({"a0":50,"a1":27})
    {
        return Err("October catalog count/arm mismatch".into());
    }
    let records = catalog["records"]
        .as_array()
        .ok_or("missing catalog records")?;
    if records.len() != 77 {
        return Err("October catalog record length mismatch".into());
    }
    if string(&d18, "curve_id")? != PRIMARY_CURVE
        || string(&d18, "status")? != "EXPORTED_UNREPLAYED"
    {
        return Err("full-D18 identity/status mismatch".into());
    }
    let mut bases = Vec::new();
    let mut ids = HashSet::new();
    let mut keys = HashSet::new();
    let mut arms = [0usize; 2];
    for record in records {
        let id = string(record, "compressed_blake3")?;
        let key = string(record, "key")?;
        let arm = string(record, "curve_arm")?;
        let ai = match arm {
            "a0" => 0,
            "a1" => 1,
            _ => return Err(format!("invalid curve arm {arm}")),
        };
        let points = number(record, "point_count_manifest_claim")?;
        let columns = number(record, "orbit_columns_manifest_claim")?;
        if points == 0 || columns == 0 || !ids.insert(id.to_owned()) || !keys.insert(key.to_owned())
        {
            return Err("zero size or duplicate catalog object/key".into());
        }
        if !record["selected_best_total_runtime"].is_null() {
            return Err("catalog already claims winner".into());
        }
        arms[ai] += 1;
        bases.push((
            id.to_owned(),
            key.to_owned(),
            arm.to_owned(),
            points,
            columns,
        ));
    }
    if arms != [50, 27] {
        return Err("wrong arm distribution".into());
    }
    let d18_id = string(&d18, "compressed_blake3")?.to_owned();
    if !ids.insert(d18_id.clone()) {
        return Err("duplicate full-D18 object".into());
    }
    if !d18["selected_best_total_runtime"].is_null()
        || !d18["total_index_calculus_runtime_ms"].is_null()
    {
        return Err("full-D18 manifest already claims runtime".into());
    }
    bases.push((
        d18_id,
        string(&d18, "object")?.to_owned(),
        "a0".into(),
        number(&d18, "points")?,
        number(&d18, "columns")?,
    ));
    bases.sort_by(|a, b| a.0.cmp(&b.0));
    let mut rows = Vec::new();
    let mut overlapping: BTreeMap<(u8, u64, u64, u64), String> = BTreeMap::new();
    for (id, key, arm, points, columns) in bases {
        let (curve_a, order) = if arm == "a0" {
            (0u8, big(PRIMARY_ORDER))
        } else {
            (1u8, big(DIAGNOSTIC_ORDER))
        };
        for m in 2..=6 {
            let (multisets, capped, floor) = query_floor(points, columns, m, &order);
            if !threshold_check(&floor, columns, &capped, &order) {
                return Err(format!("threshold failed for {id} m={m}"));
            }
            overlapping.insert((curve_a, points, columns, m), floor.to_string());
            rows.push(json!({
                "object_blake3":id, "source_object_key":key, "curve_arm":arm,
                "point_count_manifest_claim":points, "orbit_columns_manifest_claim":columns,
                "summands":m, "unordered_multisets_with_repetition":multisets.to_string(),
                "one_query_hit_ceiling_numerator":capped.to_string(),
                "one_query_hit_ceiling_denominator":order.to_string(),
                "minimum_queries_before_half_rank_probability_is_possible":floor.to_string(),
                "total_index_calculus_runtime_ms":Value::Null,
            }));
        }
    }
    if rows.len() != 390 {
        return Err("wrong output row count".into());
    }
    let old = historical["cases"]
        .as_array()
        .ok_or("missing historical cases")?;
    if old.len() != 30 {
        return Err("wrong historical case count".into());
    }
    for row in old {
        let key = (
            number(row, "curve_a")? as u8,
            number(row, "point_count")?,
            number(row, "orbit_columns")?,
            number(row, "summands")?,
        );
        let expected = string(
            row,
            "minimum_queries_before_half_rank_probability_is_possible",
        )?;
        if overlapping.get(&key).map(String::as_str) != Some(expected) {
            return Err(format!("historical regression mismatch for {key:?}"));
        }
    }
    Ok(json!({
        "schema":"n83.native-rank-query-census/v1", "protocol":"N83-RQC-20261115-v1",
        "catalog_sha256":CATALOG_SHA256, "full_d18_manifest_sha256":D18_SHA256,
        "historical_screen_sha256":HISTORICAL_SHA256,
        "model":"uniform whole-subgroup query targets; at most one independent full-smooth row per query; correlated queries permitted",
        "scope_limit":"manifest-claimed base sizes; no large-prime partials, biased queries, multiple rows per query, solver cost, or total runtime inference",
        "base_objects":78, "rows":rows, "selected_best_total_runtime":Value::Null,
    }))
}
fn main() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output = root.join("results.json");
    let encoded = serde_json::to_vec_pretty(&run(&root)?).map_err(|e| e.to_string())?;
    let mut encoded = encoded;
    encoded.push(b'\n');
    if output.exists() {
        if fs::read(&output).map_err(|e| e.to_string())? != encoded {
            return Err("write-once result differs".into());
        }
    } else {
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&output)
            .map_err(|e| e.to_string())?;
        f.write_all(&encoded).map_err(|e| e.to_string())?;
    }
    println!("{}", output.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_multisets_and_thresholds() {
        assert_eq!(multiset_count(3, 2), BigUint::from(6u8));
        assert_eq!(multiset_count(3, 3), BigUint::from(10u8));
        let r = BigUint::from(101u8);
        let (_, c, q) = query_floor(3, 4, 2, &r);
        assert_eq!(q, BigUint::from(34u8));
        assert!(threshold_check(&q, 4, &c, &r));
    }
    #[test]
    fn frozen_sources_and_regression() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let result = run(&root).expect("pinned sources and historical cases");
        assert_eq!(result["rows"].as_array().unwrap().len(), 390);
        assert!(result["selected_best_total_runtime"].is_null());
    }
    #[test]
    fn changed_source_rejected() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let source =
            root.join("../verification/live-s3-manifest-census-20261025/evidence-01/catalog.json");
        let mut changed = fs::read(&source).unwrap();
        changed[0] ^= 1;
        let path = std::env::temp_dir().join(format!(
            "n83-rqc-{}-changed-catalog.json",
            std::process::id()
        ));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        file.write_all(&changed).unwrap();
        assert!(checked_json(&path, CATALOG_SHA256).is_err());
        fs::remove_file(path).unwrap();
    }
}
