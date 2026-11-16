//! Independent general-field replay of the N83 m=2/m=3 constructed controls.
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use flate2::read::GzDecoder;
use serde_json::{json, Value};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const U_OBJECT: &str = "1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d";
const D_OBJECT: &str = "e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682";
const U_SET: &str = "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0";
const D_SET: &str = "8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0";
const K2048_OBJECT: &str = "82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851";
const D12_OBJECT: &str = "aea278aff2d79a592d5164d7437ddde8f8e9377fa481058dfc16afa176a31389";
const K2048_SET: &str = "d528f5828a05a00add4252212bc909ece481f33093e4074ff7dd8c6a51d3c97e";
const D12_SET: &str = "fb8ea2ecfeb245a90fecd5506dfc01592916b4ebec756d5fea7cbdd0c25537ce";

fn point(value: &Value) -> Result<BinaryPoint> {
    let words = value.as_array().ok_or("point words absent")?;
    if words.len() != 2 {
        return Err("point width differs".into());
    }
    let x: u128 = words[0].as_str().ok_or("point x absent")?.parse()?;
    let y: u128 = words[1].as_str().ok_or("point y absent")?.parse()?;
    if x >= 1u128 << 83 || y >= 1u128 << 83 {
        return Err("point outside field".into());
    }
    Ok(BinaryPoint::Affine {
        x: F2mElement::from_hex(&format!("{x:x}"), 83),
        y: F2mElement::from_hex(&format!("{y:x}"), 83),
    })
}

fn replay(base_root: &Path, receipt_path: &Path, output: &Path) -> Result<()> {
    let receipt_bytes = fs::read(receipt_path)?;
    let receipt: Value = serde_json::from_slice(&receipt_bytes)?;
    let arm = receipt["arm"].as_str().ok_or("arm absent")?;
    let m = receipt["m"].as_u64().ok_or("m absent")? as usize;
    if receipt["schema"] != "n83.m23-control/v1"
        || receipt["status"] != "PASS"
        || ![2, 3].contains(&m)
        || receipt["rank"] != 1
        || receipt["changed_target_rejected"] != true
        || receipt["changed_row_rejected"] != true
    {
        return Err("control receipt contract differs".into());
    }
    let (object, set, columns, points, manifest_schema, header_schema) = match arm {
        "U" => (
            U_OBJECT,
            U_SET,
            16_627,
            2_760_082,
            "n83.factor-base-panel/v2-size-frontier",
            "n83.factor-base/v1",
        ),
        "D" => (
            D_OBJECT,
            D_SET,
            16_627,
            2_760_082,
            "n83.direct-subgroup-orbit-export/v1",
            "n83.factor-base/v1",
        ),
        "K2048" => (
            K2048_OBJECT,
            K2048_SET,
            2048,
            339_968,
            "n83.factor-base-panel/v2-size-frontier",
            "n83.factor-base/v1",
        ),
        "D12" => (
            D12_OBJECT,
            D12_SET,
            2001,
            332_166,
            "n83.structured-orbit-panel/v1",
            "n83.structured-orbit-base/v1",
        ),
        _ => return Err("arm outside frozen bases".into()),
    };
    if receipt["object_blake3"] != object || receipt["point_set_blake3"] != set {
        return Err("control base identity differs".into());
    }
    let manifest: Value = serde_json::from_slice(&fs::read(base_root.join("manifest.json"))?)?;
    if manifest["schema"] != manifest_schema {
        return Err("manifest schema differs".into());
    }
    let selected = if matches!(arm, "U" | "K2048") {
        manifest["bases"]
            .as_array()
            .ok_or("U bases absent")?
            .iter()
            .find(|row| {
                row["columns"] == columns
                    && row["policy"] == "public_x_hash"
                    && row["seed"] == 2026100801u64
            })
            .ok_or("U row absent")?
    } else {
        &manifest
    };
    if selected["object"] != format!("objects/{object}.jsonl.gz")
        || selected["point_set_blake3"] != set
    {
        return Err("manifest identity differs".into());
    }
    let compressed = fs::read(base_root.join(format!("objects/{object}.jsonl.gz")))?;
    if blake3::hash(&compressed).to_hex().as_str() != object {
        return Err("archive digest differs".into());
    }
    let mut reader = BufReader::new(GzDecoder::new(compressed.as_slice()));
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let header: Value = serde_json::from_str(&line)?;
    if header["schema"] != header_schema
        || header["curve_slug"] != CURVE
        || header["orbit_columns"] != columns
        || header["point_count"] != points
    {
        return Err("archive header differs".into());
    }
    let indices: Vec<usize> = receipt["point_indices"]
        .as_array()
        .ok_or("indices absent")?
        .iter()
        .map(|v| v.as_u64().map(|x| x as usize).ok_or("index malformed"))
        .collect::<std::result::Result<_, _>>()?;
    if indices != (0..m).map(|i| i * 166).collect::<Vec<_>>() {
        return Err("control selectors differ".into());
    }
    let mut selected_points = Vec::new();
    let mut changed_first = None;
    for ordinal in 0..=(m - 1) * 166 {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            return Err("archive row missing".into());
        }
        if indices.contains(&ordinal) || ordinal == 1 {
            let row: Value = serde_json::from_str(&line)?;
            let p = point(&row["point"])?;
            if indices.contains(&ordinal) {
                if row["column"] != ordinal / 166
                    || row["phase"] != 0
                    || row["negative"] != false
                    || row["coefficient"] != "1"
                {
                    return Err("selected row label differs".into());
                }
                selected_points.push(p);
            } else {
                changed_first = Some(p);
            }
        }
    }
    let kc = KoblitzCurve::known_n83_k0().ok_or("N83 constructor absent")?;
    if kc.label() != CURVE {
        return Err("curve identity differs".into());
    }
    let mut sum = BinaryPoint::Infinity;
    for p in &selected_points {
        if !kc.curve.is_on_curve(p) || kc.mul(p, &kc.subgroup_order) != BinaryPoint::Infinity {
            return Err("selected point not on subgroup curve".into());
        }
        sum = kc.add(&sum, p);
    }
    let target = point(&receipt["target"])?;
    if sum != target || !kc.curve.is_on_curve(&target) {
        return Err("general-field group equation differs".into());
    }
    let changed_target = kc.add(&target, kc.generator());
    if sum == changed_target {
        return Err("changed target accepted".into());
    }
    let mut changed_sum = changed_first.ok_or("tampered row absent")?;
    for p in selected_points.iter().skip(1) {
        changed_sum = kc.add(&changed_sum, p);
    }
    if changed_sum == target {
        return Err("changed row accepted".into());
    }
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(output)?;
    serde_json::to_writer_pretty(
        &mut file,
        &json!({
            "schema":"n83.m23-control-general-replay/v1","status":"PASS","arm":arm,"m":m,
            "curve_id":CURVE,"rows_checked":m,"rank_lower_bound":1,
            "changed_target_rejected":true,"changed_row_rejected":true,
            "control_blake3":blake3::hash(&receipt_bytes).to_hex().to_string(),
            "object_blake3":object,"point_set_blake3":set,
            "candidate_id":Value::Null,"selected_best_total_runtime":Value::Null
        }),
    )?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: n83_m23_control_replay BASE_DIR CONTROL_JSON NEW_REPLAY_JSON".into());
    }
    replay(
        Path::new(&args[1]),
        Path::new(&args[2]),
        Path::new(&args[3]),
    )
}
