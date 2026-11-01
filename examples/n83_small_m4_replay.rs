//! Independent general-field replay for the explicit frozen N83 m=4 policies.
use crypto_lib::binary_ecc::curve::point_neg;
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use flate2::read::GzDecoder;
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const CORPUS: &str = "2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5";

fn point(v: &Value) -> Result<BinaryPoint> {
    let a = v.as_array().ok_or("point pair absent")?;
    if a.len() != 2 {
        return Err("point pair width".into());
    }
    let parse = |v: &Value| -> Result<BigUint> {
        let s = v.as_str().ok_or("coordinate string absent")?;
        let n: BigUint = s.parse()?;
        if n.bits() > 83 || n.to_string() != s {
            return Err("noncanonical coordinate".into());
        }
        Ok(n)
    };
    Ok(BinaryPoint::Affine {
        x: F2mElement::from_biguint(&parse(&a[0])?, 83),
        y: F2mElement::from_biguint(&parse(&a[1])?, 83),
    })
}

fn contract(receipt: &Value) -> Result<(&str, &str, usize, usize)> {
    let mode = receipt["mode"].as_str().ok_or("mode absent")?;
    let arm = receipt["arm"].as_str().ok_or("arm absent")?;
    let expected = match mode {
        "positive" => "PASS_CONSTRUCTED_PENDING_REPLAY",
        "natural" => "PASS_NATURAL_PENDING_REPLAY",
        _ => return Err("unsupported mode".into()),
    };
    let target_allowed = match receipt.get("experiment_policy") {
        None => matches!(arm, "K2048" | "D12") && receipt["target_index"] == 0,
        Some(p) if p == "full-m4-common-public/v1" => {
            matches!(arm, "U" | "D")
                && receipt["xor_decided_cache"].is_boolean()
                && (mode == "positive" || receipt["xor_decided_cache"] == true)
                && (mode != "positive" || receipt["target_index"] == 0)
                && receipt["conflict_budget"] == 10000
                && receipt["target_index"].as_u64().is_some_and(|n| n < 8)
        }
        Some(p) if p == "d18-shared-root-m4/v1" => {
            matches!(arm, "U" | "D")
                && (mode != "natural" || arm == "D")
                && (mode != "positive" || receipt["target_index"] == 0)
                && receipt["xor_decided_cache"] == true
                && receipt["conflict_budget"] == 10000
                && receipt["target_index"].as_u64().is_some_and(|n| n < 8)
                && matches!(
                    receipt["root_encoding"].as_str(),
                    Some("per-representative" | "shared-bits")
                )
                && receipt["root_representatives_blake3"]
                    .as_str()
                    .is_some_and(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit()))
                && receipt["source_encoding_sha256"]
                    .as_str()
                    .is_some_and(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit()))
                && receipt["root_shared_bit_count"]
                    .as_u64()
                    .is_some_and(|n| n < 83)
        }
        Some(p) => {
            p == "small-m4-xor-cache/v1"
                && matches!(arm, "K2048" | "D12")
                && receipt["xor_decided_cache"].is_boolean()
                && receipt["conflict_budget"] == 10000
                && receipt["target_index"].as_u64().is_some_and(|n| n <= 1)
        }
    };
    if receipt["schema"] != "n83.small-m4-worker/v1"
        || receipt["status"] != expected
        || receipt["curve_id"] != CURVE
        || receipt["m"] != 4
        || !target_allowed
        || receipt["rank"] != 1
        || receipt["group_verified"] != true
        || receipt["changed_target_rejected"] != true
        || receipt["changed_row_rejected"] != true
        || receipt["public_corpus_sha256"] != CORPUS
    {
        return Err("worker contract differs".into());
    }
    let (columns, points) = match arm {
        "K2048" => (2048, 339968),
        "D12" => (2001, 332166),
        "U" | "D" => (16627, 2760082),
        _ => return Err("arm outside frozen policy".into()),
    };
    if receipt["columns"] != columns || receipt["points"] != points {
        return Err("base dimensions differ".into());
    }
    Ok((mode, arm, columns, points))
}

fn replay(base: &Path, corpus: &Path, receipt_path: &Path, output: &Path) -> Result<()> {
    let bytes = fs::read(receipt_path)?;
    let receipt: Value = serde_json::from_slice(&bytes)?;
    let (mode, arm, columns, points) = contract(&receipt)?;
    let (object, set, sha) = match arm {
        "K2048" => (
            "82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851",
            "d528f5828a05a00add4252212bc909ece481f33093e4074ff7dd8c6a51d3c97e",
            "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a",
        ),
        "D12" => (
            "aea278aff2d79a592d5164d7437ddde8f8e9377fa481058dfc16afa176a31389",
            "fb8ea2ecfeb245a90fecd5506dfc01592916b4ebec756d5fea7cbdd0c25537ce",
            "c5de0b26e84ad80bfb7be6a214b89c9682e356effb6079c7abf28cd1151ffb03",
        ),
        "U" => (
            "1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d",
            "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0",
            "2457a977ea9c05a11cc60878bdb3b29f6d905dc8db9aa26e52913a702d308369",
        ),
        "D" => (
            "e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682",
            "8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0",
            "347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7",
        ),
        _ => return Err("base outside replay policy".into()),
    };
    if receipt["object_blake3"] != object || receipt["point_set_blake3"] != set {
        return Err("object or set identity differs".into());
    }
    let public_bytes = fs::read(corpus)?;
    if format!("{:x}", Sha256::digest(&public_bytes)) != CORPUS {
        return Err("corpus hash differs".into());
    }
    let public: Value = serde_json::from_slice(&public_bytes)?;
    let target = point(&receipt["target"])?;
    let target_index = receipt["target_index"].as_u64().ok_or("target index")? as usize;
    if mode == "natural" && target != point(&public["points"][target_index])? {
        return Err("natural target differs from corpus".into());
    }
    let indices: Vec<usize> = receipt["point_indices"]
        .as_array()
        .ok_or("indices absent")?
        .iter()
        .map(|v| {
            v.as_u64()
                .and_then(|n| usize::try_from(n).ok())
                .ok_or("index malformed")
        })
        .collect::<std::result::Result<_, _>>()?;
    if indices.len() != 4
        || indices.windows(2).any(|p| p[0] > p[1])
        || indices.iter().any(|&i| i >= points)
        || (mode == "positive" && indices != [0, 166, 332, 498])
    {
        return Err("indices outside receipt contract".into());
    }
    let compressed = fs::read(base.join(format!("objects/{object}.jsonl.gz")))?;
    if blake3::hash(&compressed).to_hex().as_str() != object
        || format!("{:x}", Sha256::digest(&compressed)) != sha
    {
        return Err("archive digest differs".into());
    }
    let mut reader = BufReader::new(GzDecoder::new(compressed.as_slice()));
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let header: Value = serde_json::from_str(&line)?;
    if header["curve_slug"] != CURVE
        || header["orbit_columns"] != columns
        || header["point_count"] != points
    {
        return Err("archive header differs".into());
    }
    let mut needed: BTreeSet<usize> = indices.iter().copied().collect();
    for &i in &indices {
        needed.insert((i / 166) * 166);
    }
    let mut rows = BTreeMap::new();
    let shared_policy = receipt["experiment_policy"] == "d18-shared-root-m4/v1";
    let last = if shared_policy {
        points - 1
    } else {
        *needed.last().ok_or("no selected rows")?
    };
    let mut roots = Vec::new();
    for ordinal in 0..=last {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            return Err("archive truncated".into());
        }
        if needed.contains(&ordinal) || (shared_policy && ordinal % 166 == 0) {
            let row = serde_json::from_str::<Value>(&line)?;
            if shared_policy && ordinal % 166 == 0 {
                if row["column"] != ordinal / 166
                    || row["phase"] != 0
                    || row["negative"] != false
                    || row["coefficient"] != "1"
                {
                    return Err("root row labels differ".into());
                }
                let text = row["point"][0].as_str().ok_or("root coordinate absent")?;
                let x: u128 = text.parse()?;
                if x >= 1 << 83 || x.to_string() != text {
                    return Err("root coordinate outside canonical field".into());
                }
                roots.push(x);
            }
            if needed.contains(&ordinal) {
                rows.insert(ordinal, row);
            }
        }
    }
    if shared_policy {
        check_root_inventory(&roots, &receipt)?;
    }
    let kc = KoblitzCurve::known_n83_k0().ok_or("curve absent")?;
    if kc.label() != CURVE || kc.subgroup_order.to_string() != "2417851639230796216685689" {
        return Err("subgroup identity differs".into());
    }
    let mut sum = BinaryPoint::Infinity;
    let mut sparse = BTreeMap::<usize, BigUint>::new();
    let mut selected = Vec::new();
    for &i in &indices {
        let row = &rows[&i];
        let column = i / 166;
        let phase = (i % 166) / 2;
        let negative = i % 2 == 1;
        if row["column"] != column || row["phase"] != phase || row["negative"] != negative {
            return Err("row labels differ".into());
        }
        let p = point(&row["point"])?;
        let representative = point(&rows[&(column * 166)]["point"])?;
        if !kc.curve.is_on_curve(&p)
            || !kc.curve.is_on_curve(&representative)
            || kc.mul(&p, &kc.subgroup_order) != BinaryPoint::Infinity
        {
            return Err("point not on subgroup curve".into());
        }
        let mut expected = representative;
        for _ in 0..phase {
            expected = kc.frobenius(&expected);
        }
        if negative {
            expected = point_neg(&expected);
        }
        if p != expected {
            return Err("point phase or sign differs".into());
        }
        let mut coefficient = kc.lambda.modpow(&BigUint::from(phase), &kc.subgroup_order);
        if negative {
            coefficient = &kc.subgroup_order - coefficient;
        }
        if row["coefficient"] != coefficient.to_string() {
            return Err("coefficient label differs".into());
        }
        let entry = sparse.entry(column).or_default();
        *entry = (&*entry + coefficient) % &kc.subgroup_order;
        sum = kc.add(&sum, &p);
        selected.push(p);
    }
    sparse.retain(|_, v| *v != BigUint::from(0u8));
    let canonical: Vec<_> = sparse.iter().map(|(&c, v)| (c, v.to_string())).collect();
    if sparse.is_empty()
        || receipt["sparse_row"] != json!(canonical)
        || receipt["sparse_row_blake3"]
            != blake3::hash(&serde_json::to_vec(&canonical)?)
                .to_hex()
                .as_str()
    {
        return Err("sparse row or rank differs".into());
    }
    if sum != target || !kc.curve.is_on_curve(&target) {
        return Err("group sum differs".into());
    }
    let changed_target = kc.add(&target, kc.generator());
    let mut changed_sum = point_neg(&selected[0]);
    for p in selected.iter().skip(1) {
        changed_sum = kc.add(&changed_sum, p);
    }
    if sum == changed_target || changed_sum == target {
        return Err("tamper control accepted".into());
    }
    let mut changed_row = canonical.clone();
    changed_row[0].1 = ((&sparse[&changed_row[0].0] + 1u8) % &kc.subgroup_order).to_string();
    if json!(changed_row) == receipt["sparse_row"] {
        return Err("coefficient tamper accepted".into());
    }
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(output)?;
    serde_json::to_writer_pretty(
        &mut file,
        &json!({"schema":"n83.small-m4-general-replay/v1","status":"PASS",
        "curve_id":CURVE,"arm":arm,"mode":mode,"worker_blake3":blake3::hash(&bytes).to_hex().to_string(),
        "object_blake3":object,"public_corpus_sha256":CORPUS,"point_indices":indices,
        "rank_lower_bound":1,"rows_checked":4,"changed_target_rejected":true,"changed_point_rejected":true,
        "changed_coefficient_rejected":true,"candidate_id":Value::Null,"selected_best_total_runtime":Value::Null}),
    )?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn check_root_inventory(roots: &[u128], receipt: &Value) -> Result<()> {
    if roots.len() != 16_627 || roots.iter().any(|&x| x >= 1 << 83) {
        return Err("root inventory dimensions differ".into());
    }
    let mut digest = blake3::Hasher::new();
    digest.update(b"n83.root-representatives/v1");
    let mut varying = 0u128;
    for &x in roots {
        digest.update(&x.to_le_bytes());
        varying |= x ^ roots[0];
    }
    let shared = 83 - varying.count_ones();
    let removed = if receipt["root_encoding"] == "shared-bits" {
        4 * 16_626 * shared as usize
    } else {
        0
    };
    if receipt["root_representatives_blake3"] != digest.finalize().to_hex().as_str()
        || receipt["root_shared_bit_count"] != shared
        || receipt["sat_variables"] != 201_598
        || receipt["sat_xor_rows"] != 3_901
        || receipt["sat_clauses"] != 5_915_238usize - removed
    {
        return Err("root inventory or construction counts differ".into());
    }
    Ok(())
}

fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 5 {
        return Err(
            "usage: n83_small_m4_replay BASE_DIR PUBLIC_JSON WORKER_JSON NEW_REPLAY_JSON".into(),
        );
    }
    replay(
        Path::new(&a[1]),
        Path::new(&a[2]),
        Path::new(&a[3]),
        Path::new(&a[4]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capped_control_and_wrong_mode_cannot_pass_replay_contract() {
        let mut r = json!({"schema":"n83.small-m4-worker/v1","status":"PASS_CONSTRUCTED_PENDING_REPLAY",
            "mode":"positive","curve_id":CURVE,"arm":"D12","m":4,"target_index":0,"rank":1,
            "group_verified":true,"changed_target_rejected":true,"changed_row_rejected":true,
            "public_corpus_sha256":CORPUS,"columns":2001,"points":332166});
        assert!(contract(&r).is_ok());
        r["status"] = json!("UNKNOWN_budget");
        assert!(contract(&r).is_err());
        r["status"] = json!("PASS_NATURAL_PENDING_REPLAY");
        r["mode"] = json!("natural");
        r["target_index"] = json!(1);
        assert!(contract(&r).is_err());
        r["experiment_policy"] = json!("small-m4-xor-cache/v1");
        r["xor_decided_cache"] = json!(true);
        r["conflict_budget"] = json!(10000);
        assert!(contract(&r).is_ok());
        r["target_index"] = json!(2);
        assert!(contract(&r).is_err());
        r["experiment_policy"] = json!("full-m4-common-public/v1");
        assert!(contract(&r).is_err());
        r["arm"] = json!("D");
        r["columns"] = json!(16627);
        r["points"] = json!(2760082);
        r["target_index"] = json!(7);
        assert!(contract(&r).is_ok());
        r["target_index"] = json!(8);
        assert!(contract(&r).is_err());
        r["target_index"] = json!(0);
        r["xor_decided_cache"] = json!(false);
        assert!(contract(&r).is_err());
        r["mode"] = json!("positive");
        r["status"] = json!("PASS_CONSTRUCTED_PENDING_REPLAY");
        assert!(contract(&r).is_ok());
        r["experiment_policy"] = json!("small-m4-xor-cache/v1");
        assert!(contract(&r).is_err());
        r["status"] = json!("PASS_CONSTRUCTED_PENDING_REPLAY");
        r["mode"] = json!("natural");
        assert!(contract(&r).is_err());
    }
    #[test]
    fn point_parser_rejects_noncanonical_and_outside_field_coordinates() {
        assert!(point(&json!(["00", "1"])).is_err());
        assert!(point(&json!([(BigUint::from(1u8) << 83usize).to_string(), "1"])).is_err());
        assert!(point(&json!(["0", "1"])).is_ok());
    }

    #[test]
    fn shared_root_replay_rejects_policy_and_inventory_tampering() {
        let roots: Vec<_> = (0..16_627u128).collect();
        let mut hash = blake3::Hasher::new();
        hash.update(b"n83.root-representatives/v1");
        for x in &roots {
            hash.update(&x.to_le_bytes());
        }
        let mut r = json!({"schema":"n83.small-m4-worker/v1","status":"PASS_NATURAL_PENDING_REPLAY",
            "mode":"natural","curve_id":CURVE,"arm":"D","m":4,"target_index":7,"rank":1,
            "group_verified":true,"changed_target_rejected":true,"changed_row_rejected":true,
            "public_corpus_sha256":CORPUS,"columns":16627,"points":2760082,
            "experiment_policy":"d18-shared-root-m4/v1","root_encoding":"shared-bits",
            "xor_decided_cache":true,"conflict_budget":10000,
            "root_representatives_blake3":hash.finalize().to_hex().to_string(),
            "source_encoding_sha256":"1".repeat(64),"root_shared_bit_count":68,
            "sat_variables":201598,"sat_xor_rows":3901,"sat_clauses":5915238-4*16626*68});
        assert!(contract(&r).is_ok());
        assert!(check_root_inventory(&roots, &r).is_ok());
        for (key, value) in [
            ("root_encoding", json!("auto")),
            ("arm", json!("U")),
            ("xor_decided_cache", json!(false)),
            ("target_index", json!(8)),
            ("source_encoding_sha256", json!("missing")),
        ] {
            let mut bad = r.clone();
            bad[key] = value;
            assert!(contract(&bad).is_err(), "{key}");
        }
        for (key, value) in [
            ("root_representatives_blake3", json!("0".repeat(64))),
            ("root_shared_bit_count", json!(67)),
            ("sat_clauses", json!(5915238)),
            ("sat_variables", json!(201597)),
            ("sat_xor_rows", json!(3900)),
        ] {
            let mut bad = r.clone();
            bad[key] = value;
            assert!(check_root_inventory(&roots, &bad).is_err(), "{key}");
        }
        r["root_encoding"] = json!("per-representative");
        r["sat_clauses"] = json!(5915238);
        assert!(contract(&r).is_ok());
        assert!(check_root_inventory(&roots, &r).is_ok());
    }
}
