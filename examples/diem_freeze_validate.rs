//! Validate durable producer receipts before committing a complete frozen grid.
//! This checks identities, arithmetic constraints and recorded producer controls;
//! it does not independently replay point enumeration or prove the L-polynomial.
use crypto_lib::cryptanalysis::diem_quartic::{is_prime_u64, q_of_k, SplitMix64};
#[path = "diem_support/checkpoint.rs"]
mod checkpoint;
use serde_json::{json, Value};
use std::{fs, path::Path};

fn u(v: &Value, key: &str) -> Result<u64, String> {
    v[key]
        .as_u64()
        .ok_or_else(|| format!("missing unsigned {key}"))
}

fn canonical_field(q: u64) -> (u64, u64) {
    // When q = 2 mod 3, cubing permutes F_q. Every x^3+b then
    // has a root, so skip exactly that provably reducible a=0 family.
    // This preserves Kf::new's lexicographic choice without its q^2 scan.
    let (mut a, mut b) = if q % 3 == 2 { (1, 0) } else { (0, 1) };
    loop {
        let has_root = (0..q).any(|x| {
            let value =
                (x as u128 * x as u128 % q as u128) * x as u128 + a as u128 * x as u128 + b as u128;
            value.is_multiple_of(q as u128)
        });
        if !has_root {
            return (a, b);
        }
        b += 1;
        if b == q {
            b = 0;
            a += 1;
        }
    }
}

fn validate_record(v: &Value, k: u32, idx: u32) -> Result<(), String> {
    let require = |ok: bool, message: &str| if ok { Ok(()) } else { Err(message.to_owned()) };
    require(
        u(v, "k")? == k as u64 && u(v, "idx")? == idx as u64,
        "record key mismatch",
    )?;
    let q = q_of_k(k);
    require(u(v, "q")? == q, "field prime differs from protocol")?;
    let (ka, kb) = canonical_field(q);
    require(
        u(v, "ka")? == ka && u(v, "kb")? == kb,
        "field polynomial differs",
    )?;
    for (key, len) in [("ub", 3), ("uc", 3), ("quartic", 15)] {
        let a = v[key]
            .as_array()
            .ok_or_else(|| format!("missing array {key}"))?;
        require(
            a.len() == len && a.iter().all(|x| x.as_u64().is_some_and(|n| n < q)),
            "invalid field coefficients",
        )?;
    }
    require(
        v["quartic"]
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x.as_u64() != Some(0))
            .and_then(Value::as_u64)
            == Some(1),
        "quartic not normalized",
    )?;
    let (order, h, ell) = (u(v, "order")?, u(v, "h")?, u(v, "ell")?);
    require(
        (1..=16).contains(&h) && is_prime_u64(ell) && ell < (1 << 63),
        "invalid subgroup factors",
    )?;
    require(
        (h as u128) * (ell as u128) == order as u128,
        "order factorization differs",
    )?;
    require(
        (ell as u128).pow(2) > (q as u128).pow(5),
        "subgroup below registered size",
    )?;
    let trace = v["trace"].as_i64().ok_or("invalid trace")? as i128;
    require(
        trace == (q as i128).pow(3) + 1 - order as i128,
        "order and trace differ",
    )?;
    require(
        trace.unsigned_abs().pow(2) <= 4 * (q as u128).pow(3),
        "order outside Hasse interval",
    )?;
    let n1 = u(v, "n1")?;
    require(
        n1 == q + 1 && u(v, "n1_expected")? == n1 && u(v, "on_cover")? == n1,
        "point-count or cover control incomplete",
    )?;
    require(
        u(v, "cover_fail")? == 0 && u(v, "order_fail")? == 0,
        "producer control failed",
    )?;
    let checked = u(v, "order_checked")?;
    require(
        checked > 0 && checked <= n1 && (k > 14 || checked == n1),
        "order-check coverage invalid",
    )?;
    require(
        v["gen_wall_s"]
            .as_f64()
            .is_some_and(|x| x.is_finite() && x >= 0.0),
        "invalid timing receipt",
    )?;
    let candidates = u(v, "candidates")?;
    let rejects = v["rejects"]
        .as_array()
        .ok_or("missing rejection counters")?;
    let rejected = rejects
        .iter()
        .try_fold(0u64, |s, x| s.checked_add(x.as_u64()?));
    require(
        rejects.len() == 6 && rejected.and_then(|n| n.checked_add(1)) == Some(candidates),
        "candidate accounting differs",
    )?;
    // Stream replay checks the saved accepted candidate without repeating its
    // costly order search or changing the generator's live source identity.
    require(
        candidates > 0 && candidates <= 10_000_000,
        "candidate stream exceeds validation bound",
    )?;
    let mut rng = SplitMix64::new(0x5732_414C_0000_0000u64 ^ (q << 8) ^ idx as u64);
    let mut last = [0u64; 6];
    for _ in 0..candidates {
        for x in &mut last {
            *x = rng.below(q);
        }
    }
    for (j, key) in ["ub", "uc"].iter().enumerate() {
        for i in 0..3 {
            require(
                v[*key][i].as_u64() == Some(last[3 * j + i]),
                "saved candidate differs from seeded stream",
            )?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let partial = args.iter().any(|a| a == "--partial");
    let root = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .ok_or("usage: diem_freeze_validate RESULTS [--partial]")?;
    let root = Path::new(root);
    if !root.join("instance-checkpoints-v1/manifest.json").is_file() {
        return Err("missing existing manifest".into());
    }
    let source = json!({
        "schema": 1,
        "driver": include_str!("diem_quartic_study.rs"),
        "checkpoint": include_str!("diem_support/checkpoint.rs"),
        "generator": include_str!("../src/cryptanalysis/diem_quartic.rs"),
        "protocol": include_str!("../research/diem_quartic_relations_20261018/PROTOCOL.md"),
        "amendments": include_str!("../research/diem_quartic_relations_20261018/AMENDMENTS-20261018.md"),
    });
    let identity = checkpoint::digest(source.to_string().as_bytes());
    let grid: Vec<_> = (8..=21)
        .flat_map(|k| (0..if k <= 12 { 5 } else { 3 }).map(move |i| (k, i)))
        .collect();
    let store = checkpoint::Store::open(root, &identity, &grid)?;
    let mut rows = Vec::new();
    for &(k, idx) in &grid {
        if let Some(payload) = store.read(k, idx)? {
            let v: Value = serde_json::from_str(&payload)?;
            validate_record(&v, k, idx).map_err(|e| format!("({k},{idx}): {e}"))?;
            rows.push(
                json!({"k":k,"idx":idx,"payload_sha256":checkpoint::digest(payload.as_bytes())}),
            );
        }
    }
    let missing = store.missing()?;
    if !partial && !missing.is_empty() {
        return Err(format!("incomplete grid: {}/52", rows.len()).into());
    }
    let mut final_sha = None;
    if !partial {
        let body = fs::read(root.join("instances.jsonl"))?;
        let sha = checkpoint::digest(&body);
        if fs::read_to_string(root.join("FROZEN.sha256"))? != format!("{sha}  instances.jsonl\n") {
            return Err("frozen checksum mismatch".into());
        }
        final_sha = Some(sha);
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema":1,"status":if partial {"PARTIAL_RECEIPTS_VALID"} else {"COMPLETE_RECEIPTS_VALID"},
            "source_sha256":identity,"validated":rows.len(),"expected":52,"missing":missing,
            "instances_sha256":final_sha,"records":rows,
            "validation_scope":"checksums, exact grid/source/field identity, seeded candidate replay, order constraints and recorded producer controls",
            "independent_point_enumeration_replayed":false,"independent_lpolynomial_replayed":false
        }))?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canonical_field_matches_original_enumeration() {
        for q in [7, 11, 19, 23, 263, 523] {
            let kf = crypto_lib::cryptanalysis::diem_quartic::Kf::new(q);
            assert_eq!(canonical_field(q), (kf.a, kf.b));
        }
    }
    fn fixture() -> Value {
        serde_json::from_str(include_str!("../research/diem_quartic_relations_20261018/resume-validation-20261019/freeze-fixture.json")).unwrap()
    }
    #[test]
    fn accepts_saved_native_record() {
        validate_record(&fixture(), 8, 0).unwrap();
    }
    #[test]
    fn rejects_modified_controls_identity_order_and_candidate() {
        for key in [
            "q",
            "ka",
            "kb",
            "order",
            "h",
            "ell",
            "trace",
            "n1",
            "n1_expected",
            "on_cover",
            "order_checked",
            "candidates",
        ] {
            let mut v = fixture();
            v[key] = json!(v[key].as_i64().unwrap() + 1);
            assert!(validate_record(&v, 8, 0).is_err(), "{key}");
        }
        for key in ["cover_fail", "order_fail"] {
            let mut v = fixture();
            v[key] = json!(1);
            assert!(validate_record(&v, 8, 0).is_err(), "{key}");
        }
        let mut v = fixture();
        v["ub"][0] = json!((v["ub"][0].as_u64().unwrap() + 1) % v["q"].as_u64().unwrap());
        assert!(validate_record(&v, 8, 0).is_err());
    }
}
