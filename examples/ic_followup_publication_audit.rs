//! Native retained-profile accounting and scalar checks for partial quartic runs.
use crypto_lib::binary_ecc::{
    curve::scalar_mul, BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly,
};
use crypto_lib::cryptanalysis::{
    curve_id,
    gaudry_quartic_bin_wide::{find_bin_curve, BinCurve, TowerBasis},
};
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path};

fn read(p: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap()
}
fn digest(b: &[u8]) -> String {
    hex::encode(Sha256::digest(b))
}
fn ir(text: &str) -> u64 {
    assert_eq!(
        text.lines().find(|l| l.starts_with("events:")),
        Some("events: Ir")
    );
    let numbers = |prefix: &str| {
        text.lines()
            .filter_map(|l| l.strip_prefix(prefix))
            .map(|n| n.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    };
    let summary = numbers("summary:");
    assert_eq!(summary.len(), 1);
    assert_eq!(summary, numbers("totals:"));
    summary[0]
}
fn constructor(study: &Path) -> Value {
    let mut arms = vec![];
    let mut outputs = vec![];
    for arm in ["before", "after"] {
        let dir = study.join(format!("profiles/narrow-constructor-{arm}-20261025"));
        let raw = std::fs::read(dir.join("callgrind.out")).unwrap();
        let total = ir(std::str::from_utf8(&raw).unwrap());
        let manifest = read(&dir.join("manifest.json"));
        assert_eq!(manifest["status"], "ok");
        assert_eq!(manifest["hotspots_method"], "callgrind");
        assert_eq!(read(&dir.join("hotspots.json"))["total_ir"], total);
        let bin = manifest["binary"]["path"].as_str().unwrap();
        let path = std::env::var_os("IC_PROFILE_BINARY_DIR").map_or_else(
            || Path::new(bin).to_path_buf(),
            |d| Path::new(&d).join(Path::new(bin).file_name().unwrap()),
        );
        assert_eq!(
            digest(&std::fs::read(path).unwrap()),
            manifest["binary"]["sha256"]
        );
        assert_eq!(manifest["command"], json!([bin, "narrow", "6"]));
        outputs.push(std::fs::read(dir.join("command.stdout")).unwrap());
        arms.push(json!({"arm":arm,"whole_process_ir":total,"raw_sha256":digest(&raw),
            "binary_sha256":manifest["binary"]["sha256"],"host":manifest["host"],"source_context":manifest["git"]}));
    }
    assert_eq!(outputs[0], outputs[1]);
    let mut controls = vec![];
    for m in 4..=10 {
        let before = study.join(format!(
            "pipeline-receipts/narrow-constructor-before-20261025/m{m}"
        ));
        let after = study.join(format!(
            "pipeline-receipts/narrow-constructor-after-20261025/m{m}"
        ));
        for dir in [&before, &after] {
            assert_eq!(read(&dir.join("status.json"))["exit_code"], 0);
        }
        let a = std::fs::read(before.join("output.json")).unwrap();
        let b = std::fs::read(after.join("output.json")).unwrap();
        assert_eq!(a, b);
        let control: Value = serde_json::from_slice(&b).unwrap();
        assert_eq!(control["m"], m);
        assert_eq!(control["control_parity_checked"], true);
        assert_eq!(
            read(&after.join("comparison-status.json"))["byte_comparison_exit_code"],
            0
        );
        controls.push(control);
    }
    for relative in [
        "narrow-constructor-build-20261025/status.json",
        "narrow-constructor-library-gate-20261025/status.json",
        "narrow-constructor-after-20261025/example-tests-status.json",
        "narrow-constructor-after-20261025/clippy-status.json",
    ] {
        assert_eq!(
            read(&study.join("pipeline-receipts").join(relative))["exit_code"],
            0
        );
    }
    let log = std::fs::read_to_string(
        study.join("pipeline-receipts/narrow-constructor-library-gate-20261025/test.log"),
    )
    .unwrap();
    assert!(log.lines().any(|l| l.starts_with(
        "test result: ok. 4635 passed; 0 failed; 119 ignored; 0 measured; 0 filtered out;"
    )));
    json!({"arms":arms,"field_product_controls":controls,"library_passed":4635,"library_ignored":119,
        "constructor_output_byte_identical":true,"status":"raw_profiles_outputs_controls_and_recorded_source_checks_agree",
        "scope":"simulated whole-process Ir, not native runtime, external fastest-tool evidence or complete IC price"})
}
fn hex_value(v: &Value) -> u128 {
    u128::from_str_radix(v.as_str().unwrap().trim_start_matches("0x"), 16).unwrap()
}
fn point(v: &Value, inst: &BinCurve, basis: &TowerBasis) -> BinaryPoint {
    let n = 4 * inst.curve.f.f.m;
    let x = inst.curve.f.unpack(hex_value(&v[0]));
    let y = inst.curve.f.unpack(hex_value(&v[1]));
    BinaryPoint::Affine {
        x: F2mElement::from_biguint(&BigUint::from(basis.coords(&inst.curve.f, &x)), n),
        y: F2mElement::from_biguint(&BigUint::from(basis.coords(&inst.curve.f, &y)), n),
    }
}
fn partial(study: &Path, m: u32) -> Value {
    let dir = study.join(format!(
        "pipeline-runs/quartic-m{m}-local-counts-20261025-01"
    ));
    let raw = std::fs::read(dir.join("records.jsonl")).unwrap();
    let records: Vec<Value> = std::str::from_utf8(&raw)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert!(records.len() < 24);
    let inst = find_bin_curve(m, 1, 16).unwrap();
    let basis = TowerBasis::new(&inst.curve.f);
    let n = 4 * m;
    let irr = IrreduciblePoly {
        degree: n,
        low_terms: (0..n).filter(|&i| basis.modulus >> i & 1 == 1).collect(),
    };
    let a = basis.coords(&inst.curve.f, &inst.curve.a);
    let b = basis.coords(&inst.curve.f, &inst.curve.b);
    let gx = basis.coords(&inst.curve.f, &inst.generator.x);
    let gy = basis.coords(&inst.curve.f, &inst.generator.y);
    let factory_generator = BinaryPoint::Affine {
        x: F2mElement::from_biguint(&BigUint::from(gx), n),
        y: F2mElement::from_biguint(&BigUint::from(gy), n),
    };
    let generator = point(&records[0]["workload"]["curve"]["generator"], &inst, &basis);
    let curve = BinaryCurve {
        m: n,
        irreducible: irr,
        a: F2mElement::from_biguint(&BigUint::from(a), n),
        b: F2mElement::from_biguint(&BigUint::from(b), n),
        generator: generator.clone(),
        order: BigUint::from(inst.r),
        cofactor: BigUint::from(inst.cofactor),
    };
    assert!(curve.is_on_curve(&generator));
    assert_eq!(
        scalar_mul(&curve, &generator, &BigUint::from(inst.r)),
        BinaryPoint::Infinity
    );
    let id = curve_id::binary(
        n,
        &BigUint::from(basis.modulus),
        &BigUint::from(a),
        &BigUint::from(b),
        &BigUint::from(inst.group_order),
        None,
    )
    .unwrap();
    let mut counts = BTreeMap::<String, BTreeMap<String, u64>>::new();
    let mut verified = vec![];
    for record in &records {
        assert_eq!(
            record["binary_sha256"],
            "efa25f209e57dd30fe191ddbd3a3c0d8465952cbe0a23b38a9dc7450f8051c9c"
        );
        let c = &record["workload"]["curve"];
        assert_eq!(c["icv1"], id.icv1);
        assert_eq!(
            c["group_order"].as_u64().map(u128::from),
            Some(inst.group_order)
        );
        assert_eq!(c["r"].as_u64().map(u128::from), Some(inst.r));
        assert_eq!(c["cofactor"].as_u64().map(u128::from), Some(inst.cofactor));
        assert_eq!(point(&c["generator"], &inst, &basis), generator);
        let arm = record["arm"].as_str().unwrap();
        let status = record["outcome"]["status"].as_str().unwrap();
        *counts
            .entry(arm.to_owned())
            .or_default()
            .entry(status.to_owned())
            .or_default() += 1;
        if status == "verified" {
            assert_eq!(record["outcome"]["matches_target"], true);
            let k = BigUint::parse_bytes(
                record["outcome"]["recovered"].as_str().unwrap().as_bytes(),
                10,
            )
            .unwrap();
            assert!(k < curve.order);
            let target = point(&record["workload"]["target"], &inst, &basis);
            assert!(curve.is_on_curve(&target));
            assert_eq!(scalar_mul(&curve, &generator, &k), target);
            verified.push(record["seq"].clone());
        }
    }
    let retained_audit = if m == 14 {
        let audit = read(
            &study
                .join("pipeline-receipts/quartic-m14-local-counts-20261025-01-launch/replay.json"),
        );
        assert_eq!(audit["ok"], false);
        assert_eq!(audit["records"], records.len());
        assert_eq!(audit["files"]["records.jsonl"], digest(&raw));
        assert_eq!(audit["verified_records"], verified.len());
        assert!(audit["replays"]
            .as_array()
            .unwrap()
            .iter()
            .all(|r| r["reproduced"] == true));
        json!({"ok":false,"session_status":audit["session_status"],"successful_record_replays":audit["replays"].as_array().unwrap().len(),"problems":audit["problems"]})
    } else {
        Value::Null
    };
    json!({"m":m,"n":n,"recorded":records.len(),"registered":24,"missing_registered_executions":24-records.len(),
        "counts":counts,"independent_scalar_checks":verified.len(),"verified_sequences":verified,"records_sha256":digest(&raw),
        "icv1":id.icv1,"curve_name":id.slug,"polynomial_modulus_hex":format!("0x{:x}",basis.modulus),
        "a_hex":format!("0x{a:x}"),"b_hex":format!("0x{b:x}"),"r_decimal":inst.r.to_string(),"order_decimal":inst.group_order.to_string(),
        "retained_original_audit":retained_audit,"complete_ic_cost":null,"ic_rho_ratio":null,
        "wide_factory_generator_matches_recorded":factory_generator==generator,
        "recorded_generator_tower":records[0]["workload"]["curve"]["generator"],
        "wide_factory_generator_polynomial":[format!("0x{gx:x}"),format!("0x{gy:x}")],
        "status":"partial_session_recorded_successful_answers_independently_verified",
        "scope":"successful scalar answers only; no new walks, counter replay or complete-session qualification"})
}
fn main() {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() == 2 && a[0] == "partial" {
        let study = Path::new(&a[1]);
        println!("{}", json!([14, 15].map(|m| partial(study, m))));
        return;
    }
    assert_eq!(a.len(), 1, "STUDY | partial STUDY");
    let study = Path::new(&a[0]);
    let profiles = constructor(study);
    let quartic = [14, 15].map(|m| partial(study, m));
    println!(
        "{}",
        json!({"schema":"ic-followup-publication-audit/v1","constructor":profiles,"partial_quartics":quartic,
        "status":"native_profile_accounting_and_partial_scalar_checks_pass","complete_ic_cost":null,"rho_crossover":null})
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn raw_profile_requires_matching_single_event_totals() {
        assert_eq!(ir("events: Ir\nsummary: 123\ntotals: 123\n"), 123);
        for text in [
            "events: Ir\nsummary: 123\ntotals: 124\n",
            "events: Ir Dr\nsummary: 123 1\ntotals: 123 1\n",
        ] {
            assert!(std::panic::catch_unwind(|| ir(text)).is_err());
        }
    }
}
