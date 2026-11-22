use flate2::read::GzDecoder;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const STUDY: &str = "koblitz_n83_factor_base_sweep_20261008";
const OLD_PREFIX: &str =
    "s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008";
const PROPOSED_REMOTE: &str = "s3://setec-astronomy-artifacts/store";

fn string<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    value[key]
        .as_str()
        .ok_or_else(|| format!("missing string {key}").into())
}

fn number(value: &Value, key: &str) -> Result<u64> {
    value[key]
        .as_u64()
        .ok_or_else(|| format!("missing integer {key}").into())
}

fn ensure(test: bool, message: &str) -> Result<()> {
    if test {
        Ok(())
    } else {
        Err(message.to_string().into())
    }
}

fn digest_sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn run(study: &Path, objects: &Path, output: &Path) -> Result<()> {
    ensure(!output.exists(), "inventory already exists")?;
    let manifest_bytes = fs::read(study.join("manifest.json"))?;
    let receipt_bytes = fs::read(study.join("upload-receipt.json"))?;
    let manifest: Value = serde_json::from_slice(&manifest_bytes)?;
    let receipt: Value = serde_json::from_slice(&receipt_bytes)?;
    ensure(
        manifest["schema"] == "n83.factor-base-panel/v1"
            && manifest["status"] == "completed_factor_base_panel"
            && manifest["study"] == STUDY
            && manifest["expected_base_count"] == 54
            && manifest["completed_base_count"] == 54,
        "unexpected frozen manifest identity or completion status",
    )?;
    ensure(
        receipt["schema"] == "n83.factor-base-storage/v1"
            && receipt["status"] == "PASS"
            && receipt["bucket"] == "crypto-autoresearcher"
            && receipt["manifest_blake3"] == blake3::hash(&manifest_bytes).to_hex().to_string(),
        "upload receipt does not bind the frozen manifest",
    )?;
    let bases = manifest["bases"].as_array().ok_or("manifest bases")?;
    let uploaded = receipt["objects"].as_array().ok_or("receipt objects")?;
    ensure(
        bases.len() == 54 && uploaded.len() == 54,
        "wrong panel size",
    )?;

    let mut seen_uris = HashSet::new();
    let mut checked_receipts = HashSet::new();
    let mut results = Vec::with_capacity(bases.len());
    let mut total_bytes = 0u64;
    let mut counts = [0usize; 2];
    for row in bases {
        let a = usize::try_from(number(row, "a")?)?;
        ensure(a <= 1, "unexpected curve a")?;
        let compressed_blake3 = string(row, "compressed_blake3")?;
        ensure(
            compressed_blake3.len() == 64
                && compressed_blake3.bytes().all(|b| b.is_ascii_hexdigit()),
            "invalid compressed digest",
        )?;
        let object = string(row, "object")?;
        ensure(
            object == format!("objects/{compressed_blake3}.jsonl.gz"),
            "unsafe or noncanonical object name",
        )?;
        let old_uri = string(row, "s3_uri")?;
        ensure(
            old_uri == format!("{OLD_PREFIX}/a{a}/{object}"),
            "old S3 URI differs from frozen convention",
        )?;
        ensure(seen_uris.insert(old_uri), "duplicate old object URI")?;
        let old_receipt = uploaded
            .iter()
            .find(|value| value["s3_uri"] == old_uri)
            .ok_or("missing upload receipt")?;
        ensure(
            old_receipt["compressed_blake3"] == compressed_blake3
                && old_receipt["downloaded_hash_matches"] == true,
            "old upload receipt mismatch",
        )?;
        ensure(
            checked_receipts.insert(old_uri),
            "duplicate receipt matched",
        )?;

        let bytes = fs::read(objects.join(format!("a{a}")).join(object))?;
        let expected_len = number(row, "bytes")?;
        ensure(
            bytes.len() as u64 == expected_len,
            "compressed size mismatch",
        )?;
        ensure(
            blake3::hash(&bytes).to_hex().as_str() == compressed_blake3,
            "compressed BLAKE3 mismatch",
        )?;
        let mut decoder = GzDecoder::new(&bytes[..]);
        let mut plain = Vec::new();
        decoder.read_to_end(&mut plain)?;
        ensure(
            blake3::hash(&plain).to_hex().as_str() == string(row, "plain_blake3")?,
            "plain BLAKE3 mismatch",
        )?;
        let sha256 = digest_sha256(&bytes);
        let curve = if a == 0 {
            "icv1-f2m83-tm6151469093347-debefd74"
        } else {
            "icv1-f2m83-t6151469093347-cdcc5432"
        };
        let store_key = format!("sha256/{}/{}", &sha256[..2], sha256);
        results.push(json!({
            "a": a,
            "bytes": expected_len,
            "compressed_blake3": compressed_blake3,
            "plain_blake3": string(row, "plain_blake3")?,
            "sha256": sha256,
            "old_s3_uri": old_uri,
            "proposed_store_name": format!("factor-bases/{curve}/pilot-01/{compressed_blake3}.jsonl.gz"),
            "proposed_store_key": store_key,
            "proposed_s3_uri": format!("{PROPOSED_REMOTE}/{store_key}"),
            "curve_slug": curve,
            "policy": string(row, "policy")?,
            "columns": number(row, "columns")?,
            "seed": number(row, "seed")?,
        }));
        total_bytes += expected_len;
        counts[a] += 1;
    }
    ensure(counts == [27, 27], "curve partition mismatch")?;
    ensure(
        uploaded.iter().all(|value| {
            value["s3_uri"]
                .as_str()
                .is_some_and(|uri| checked_receipts.contains(uri))
        }),
        "receipt has an unmatched object",
    )?;
    results.sort_by(|left, right| {
        left["old_s3_uri"]
            .as_str()
            .cmp(&right["old_s3_uri"].as_str())
    });
    let inventory = json!({
        "schema": "n83.factor-base-store-migration-inventory/v1",
        "status": "verified_local_source_proposed_destination",
        "publication_status": "not_uploaded",
        "manifest_sha256": digest_sha256(&manifest_bytes),
        "manifest_blake3": blake3::hash(&manifest_bytes).to_hex().to_string(),
        "upload_receipt_sha256": digest_sha256(&receipt_bytes),
        "source_bucket": "crypto-autoresearcher",
        "proposed_remote": PROPOSED_REMOTE,
        "object_count": results.len(),
        "curve_counts": {"a0": counts[0], "a1": counts[1]},
        "compressed_bytes": total_bytes,
        "objects": results,
    });
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer_pretty(&mut file, &inventory)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!(
        "verified {} objects, {} compressed bytes; no upload",
        bases.len(),
        total_bytes
    );
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    ensure(
        args.len() == 4,
        "usage: n83-pilot-migration-inventory STUDY_DIR DOWNLOADED_OBJECTS_DIR NEW_OUTPUT_JSON",
    )?;
    run(
        &PathBuf::from(&args[1]),
        &PathBuf::from(&args[2]),
        &PathBuf::from(&args[3]),
    )
}
