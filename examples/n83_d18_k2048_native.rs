//! Native verification for the primary N83 D18 K2048 construction.
use flate2::read::GzDecoder;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const ORDER: &str = "2417851639230796216685689";
const U_BLAKE3: &str = "82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851";
const D_BLAKE3: &str = "a3c06a35db07b7aa0d9e7f45e8b747d5c136f2ea1a0fdc0e8822ffed84659f68";
const U_POLICY: &str = "public_x_hash";
const D_POLICY: &str = "direct_subgroup_x_d18_prefix2048";
const BUCKET: &str = "crypto-autoresearcher";
const PREFIX: &str =
    "factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-direct-subgroup-k2048-v1/a0";
const POINTS: usize = 339_968;
const COLUMNS: usize = 2_048;

fn sha256(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}

fn hashes(data: &[u8]) -> Value {
    json!({"sha256":sha256(data),"blake3":blake3::hash(data).to_hex().to_string(),"bytes":data.len()})
}

fn must(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn write_new(path: &Path, value: &Value) -> Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!("{}", serde_json::to_string(value)?);
    Ok(())
}

fn source_binary_identity() -> Result<Value> {
    Ok(json!({
        "verifier_source_blake3": blake3::hash(include_bytes!("n83_d18_k2048_native.rs")).to_hex().to_string(),
        "verifier_binary_sha256": sha256(&fs::read(std::env::current_exe()?)?),
    }))
}

fn load_points(arm: &str, path: &Path) -> Result<(HashSet<(u128, u128)>, Value)> {
    let (expected_digest, policy) = match arm {
        "U" => (U_BLAKE3, U_POLICY),
        "D" => (D_BLAKE3, D_POLICY),
        _ => return Err("unknown arm".into()),
    };
    let data = fs::read(path)?;
    must(
        blake3::hash(&data).to_hex().as_str() == expected_digest,
        "compressed archive BLAKE3 differs",
    )?;
    let mut reader = BufReader::new(GzDecoder::new(data.as_slice()));
    let mut line = String::new();
    must(reader.read_line(&mut line)? > 0, "archive header absent")?;
    let header: Value = serde_json::from_str(&line)?;
    must(
        header["curve_slug"] == CURVE
            && header["policy"] == policy
            && header["orbit_columns"] == COLUMNS
            && header["point_count"] == POINTS
            && header["subgroup_order"] == ORDER
            && header["closure"] == "signed_frobenius",
        "archive header identity differs",
    )?;
    let mut points = HashSet::with_capacity(POINTS);
    let mut columns = vec![false; COLUMNS];
    let mut rows = 0;
    line.clear();
    while reader.read_line(&mut line)? > 0 {
        let row: Value = serde_json::from_str(&line)?;
        let xy = row["point"].as_array().ok_or("point pair absent")?;
        must(xy.len() == 2, "point pair has wrong length")?;
        let x: u128 = xy[0].as_str().ok_or("x not decimal")?.parse()?;
        let y: u128 = xy[1].as_str().ok_or("y not decimal")?.parse()?;
        let column = row["column"].as_u64().ok_or("column absent")? as usize;
        must(
            x < (1_u128 << 83) && y < (1_u128 << 83),
            "point outside field",
        )?;
        must(column < COLUMNS, "column outside range")?;
        must(points.insert((x, y)), "duplicate point")?;
        columns[column] = true;
        rows += 1;
        line.clear();
    }
    must(
        rows == POINTS && columns.iter().all(|&seen| seen),
        "row count or column coverage differs",
    )?;
    let distinct_points = points.len();
    Ok((
        points,
        json!({"policy":policy,"rows":rows,"distinct_points":distinct_points}),
    ))
}

fn overlap(u_path: &Path, d_path: &Path) -> Result<Value> {
    let (u, u_meta) = load_points("U", u_path)?;
    let (d, d_meta) = load_points("D", d_path)?;
    let common = u.intersection(&d).count();
    must(common % 166 == 0, "partial signed orbit overlap")?;
    Ok(json!({
        "schema":"n83.d18-k2048-structural-overlap-native/v1",
        "status":"PASS", "curve_id":CURVE,
        "arms":{"U":u_meta,"D":d_meta},
        "u_object_blake3":U_BLAKE3,"d_object_blake3":D_BLAKE3,
        "shared_points":common,"shared_signed_orbits":common/166,
        "unique_u_points":u.len()-common,"unique_d_points":d.len()-common,
        "selected_best_total_runtime":Value::Null,
        "verifier":source_binary_identity()?,
    }))
}

fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn custody(root: &Path) -> Result<Value> {
    let base = root.join("base");
    let fresh = root.join("fresh");
    let manifest_path = base.join("manifest.json");
    let replay_path = base.join("replay.json");
    let manifest = read_json(&manifest_path)?;
    let replay = read_json(&replay_path)?;
    let export_guard = read_json(&root.join("export-guard.json"))?;
    let replay_guard = read_json(&root.join("replay-guard.json"))?;
    must(
        manifest["schema"] == "n83.direct-subgroup-orbit-export-k2048/v1"
            && manifest["curve_id"] == CURVE
            && manifest["policy"] == D_POLICY
            && manifest["columns"] == COLUMNS
            && manifest["points"] == POINTS
            && manifest["selected_best_total_runtime"].is_null()
            && replay["schema"] == "n83.direct-subgroup-orbit-replay-k2048/v1"
            && replay["status"] == "PASS"
            && replay["representatives_checked"] == COLUMNS
            && replay["points_checked"] == POINTS
            && replay["point_set_blake3"] == manifest["point_set_blake3"]
            && replay["compressed_sha256"] == manifest["compressed_sha256"]
            && export_guard["status"] == "PASS"
            && replay_guard["status"] == "PASS"
            && export_guard["source_head_sha"] == manifest["source_commit"]
            && replay_guard["source_head_sha"] == manifest["source_commit"]
            && export_guard["binary_sha256"] == replay_guard["binary_sha256"],
        "export/replay/guard identity differs",
    )?;
    let object_name = format!(
        "{}.jsonl.gz",
        manifest["compressed_blake3"]
            .as_str()
            .ok_or("object digest absent")?
    );
    let object_rel = format!("objects/{object_name}");
    must(
        manifest["object"] == object_rel,
        "object name not content addressed",
    )?;
    let expected = json!({
        "sha256":manifest["compressed_sha256"],
        "blake3":manifest["compressed_blake3"],
        "bytes":manifest["bytes"],
    });
    must(
        hashes(&fs::read(base.join(&object_rel))?) == expected,
        "local object hash differs",
    )?;
    must(
        hashes(&fs::read(fresh.join(format!("0-{object_name}")))?) == expected,
        "fresh object hash differs",
    )?;
    must(
        fs::read(fresh.join("1-manifest.json"))? == fs::read(&manifest_path)?,
        "fresh manifest differs",
    )?;
    must(
        fs::read(fresh.join("2-replay.json"))? == fs::read(&replay_path)?,
        "fresh replay differs",
    )?;
    let key = format!("{PREFIX}/{object_rel}");
    let head = Command::new("aws")
        .args([
            "s3api",
            "head-object",
            "--bucket",
            BUCKET,
            "--key",
            &key,
            "--region",
            "us-west-2",
            "--output",
            "json",
        ])
        .output()?;
    must(head.status.success(), "S3 HEAD failed")?;
    let metadata: Value = serde_json::from_slice(&head.stdout)?;
    must(
        metadata["ContentLength"] == manifest["bytes"]
            && metadata["ServerSideEncryption"] == "AES256",
        "S3 HEAD length or encryption differs",
    )?;
    Ok(json!({
        "schema":"n83.d18-k2048-s3-custody-native/v1", "status":"PASS",
        "curve_id":CURVE,"policy":D_POLICY,
        "source_commit":manifest["source_commit"],
        "binary_sha256":export_guard["binary_sha256"],
        "s3_uri":format!("s3://{BUCKET}/{key}"),
        "object":expected,"point_set_blake3":manifest["point_set_blake3"],
        "columns":COLUMNS,"points":POINTS,
        "manifest_sha256":sha256(&fs::read(&manifest_path)?),
        "replay_sha256":sha256(&fs::read(&replay_path)?),
        "head_content_length":metadata["ContentLength"],
        "server_side_encryption":metadata["ServerSideEncryption"],
        "selected_best_total_runtime":Value::Null,
        "verifier":source_binary_identity()?,
    }))
}

fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    match args.as_slice() {
        [_, mode, u, d, output] if mode == "overlap" => {
            write_new(Path::new(output), &overlap(Path::new(u), Path::new(d))?)
        }
        [_, mode, root, output] if mode == "custody" => {
            write_new(Path::new(output), &custody(Path::new(root))?)
        }
        _ => Err(
            "usage: native overlap U_OBJECT D_OBJECT OUTPUT | native custody RUN_DIR OUTPUT".into(),
        ),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("n83 K2048 native verifier: {error}");
        std::process::exit(1);
    }
}
