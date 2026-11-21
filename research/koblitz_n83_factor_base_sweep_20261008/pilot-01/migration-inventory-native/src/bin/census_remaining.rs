use flate2::read::GzDecoder;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const PREFIX: &str = "factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/";
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const REMOTE: &str = "s3://setec-astronomy-artifacts/store";

fn ensure(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.to_owned().into())
    }
}

fn string<'a>(row: &'a Value, key: &str) -> Result<&'a str> {
    row[key]
        .as_str()
        .ok_or_else(|| format!("missing string {key}").into())
}

fn number(row: &Value, key: &str) -> Result<u64> {
    row[key]
        .as_u64()
        .ok_or_else(|| format!("missing integer {key}").into())
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn hashes(mut reader: impl Read) -> Result<(u64, String, String)> {
    let mut blake = blake3::Hasher::new();
    let mut sha = Sha256::new();
    let mut size = 0u64;
    let mut buffer = [0u8; 65536];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        size += n as u64;
        blake.update(&buffer[..n]);
        sha.update(&buffer[..n]);
    }
    Ok((
        size,
        blake.finalize().to_hex().to_string(),
        format!("{:x}", sha.finalize()),
    ))
}

fn seen_from_inventory(
    path: &Path,
    expected_schema: &str,
    expected_count: usize,
) -> Result<(HashSet<String>, u64)> {
    let inventory: Value = serde_json::from_slice(&fs::read(path)?)?;
    ensure(
        inventory["schema"] == expected_schema
            && inventory["publication_status"] == "not_uploaded"
            && inventory["object_count"] == expected_count,
        "predecessor inventory identity mismatch",
    )?;
    let rows = inventory["objects"]
        .as_array()
        .ok_or("predecessor objects")?;
    ensure(rows.len() == expected_count, "predecessor count mismatch")?;
    let bytes = rows.iter().try_fold(0u64, |acc, row| {
        Ok::<u64, Box<dyn std::error::Error>>(acc + number(row, "bytes")?)
    })?;
    ensure(
        inventory["compressed_bytes"] == bytes,
        "predecessor byte total mismatch",
    )?;
    let uris = rows
        .iter()
        .map(|row| Ok(string(row, "old_s3_uri")?.to_owned()))
        .collect::<Result<HashSet<_>>>()?;
    Ok((uris, bytes))
}

fn source_manifest(source_dir: &Path, index: &Value, source: &str) -> Result<(Value, String)> {
    let entries = index.as_array().ok_or("census index")?;
    let entry = entries
        .iter()
        .find(|entry| entry["key"] == source)
        .ok_or("manifest absent from census index")?;
    let ordinal = number(entry, "index")?;
    ensure(ordinal < 23, "manifest index out of range")?;
    let bytes = fs::read(source_dir.join(format!("{ordinal:02}.json")))?;
    let sha = digest(&bytes);
    ensure(
        sha == string(entry, "sha256")? && bytes.len() as u64 == number(entry, "bytes")?,
        "manifest snapshot differs from census index",
    )?;
    Ok((serde_json::from_slice(&bytes)?, sha))
}

fn run(
    source_dir: &Path,
    pilot: &Path,
    historical: &Path,
    blobs: &Path,
    output: &Path,
) -> Result<()> {
    ensure(!output.exists(), "inventory already exists")?;
    let (mut previous, pilot_bytes) =
        seen_from_inventory(pilot, "n83.factor-base-store-migration-inventory/v1", 54)?;
    ensure(previous.len() == 54, "duplicate pilot URI")?;
    let (historical_uris, historical_bytes) = seen_from_inventory(
        historical,
        "n83.factor-base-store-migration-inventory-historical/v1",
        5,
    )?;
    for uri in historical_uris {
        ensure(previous.insert(uri), "duplicate predecessor URI")?;
    }
    ensure(previous.len() == 59, "wrong predecessor count")?;
    let catalog_bytes = fs::read(source_dir.join("catalog.json"))?;
    let catalog: Value = serde_json::from_slice(&catalog_bytes)?;
    ensure(
        catalog["schema"] == "n83.live-s3-manifest-census/v1"
            && catalog["status"] == "PASS_77_BASES_RECONCILED"
            && catalog["content_addressed_objects"] == 77,
        "census catalog identity mismatch",
    )?;
    let index_bytes = fs::read(source_dir.join("index.json"))?;
    let index: Value = serde_json::from_slice(&index_bytes)?;
    ensure(
        index.as_array().is_some_and(|items| items.len() == 23),
        "wrong census index size",
    )?;
    let catalog_rows = catalog["records"].as_array().ok_or("catalog records")?;
    ensure(catalog_rows.len() == 77, "wrong catalog record count")?;
    let direct_custody_bytes = fs::read(source_dir.join("direct-s3-custody.json"))?;
    let direct_custody: Value = serde_json::from_slice(&direct_custody_bytes)?;
    let direct_manifest_bytes = fs::read(source_dir.join("direct-manifest.json"))?;
    let direct_manifest: Value = serde_json::from_slice(&direct_manifest_bytes)?;
    ensure(
        direct_manifest["schema"] == "n83.direct-subgroup-orbit-export/v1"
            && direct_manifest["curve_id"] == CURVE,
        "direct manifest identity mismatch",
    )?;
    let mut catalog_keys = HashSet::new();
    let mut results = Vec::new();
    let mut total_bytes = 0u64;
    let mut families = [0usize; 3];
    for record in catalog_rows {
        let key = string(record, "key")?;
        ensure(key.starts_with(PREFIX), "outside frozen N83 prefix")?;
        ensure(catalog_keys.insert(key), "duplicate census key")?;
        let uri = format!("s3://crypto-autoresearcher/{key}");
        if previous.contains(&uri) {
            continue;
        }
        ensure(
            record["curve_arm"] == "a0",
            "remaining object is not primary curve",
        )?;
        let family = string(record, "family")?;
        let (manifest, manifest_sha, family_index) = if family == "d18-direct-subgroup-v1/a0" {
            ensure(
                record["manifest_source"]
                    == "git:46c86f47c5530f309453d94ee7246957b03544af:export-v1/manifest.json"
                    && record["independent_full_row_replay"] == true,
                "direct D18 source or replay mismatch",
            )?;
            (direct_manifest.clone(), digest(&direct_manifest_bytes), 0)
        } else if family == "standard-subspace/a0" || family == "v2-size-frontier/a0" {
            let source = string(record, "manifest_source")?;
            let (manifest, sha) = source_manifest(source_dir, &index, source)?;
            (
                manifest,
                sha,
                if family == "standard-subspace/a0" {
                    1
                } else {
                    2
                },
            )
        } else {
            return Err(format!("unexpected remaining family {family}").into());
        };
        let row = if family_index == 2 {
            ensure(
                manifest["schema"] == "n83.factor-base-panel/v2-size-frontier"
                    && manifest["status"] == "completed_factor_base_object",
                "v2 manifest identity mismatch",
            )?;
            let bases = manifest["bases"].as_array().ok_or("v2 bases")?;
            ensure(bases.len() == 1 && bases[0]["a"] == 0, "v2 curve mismatch")?;
            &bases[0]
        } else {
            if family_index == 1 {
                ensure(
                    manifest["schema"] == "n83.standard-subspace-factor-base-manifest/v1"
                        && manifest["curve_slug"] == CURVE,
                    "standard manifest identity mismatch",
                )?;
            }
            &manifest
        };
        let expected_blake = string(record, "compressed_blake3")?;
        ensure(
            expected_blake.len() == 64 && expected_blake.bytes().all(|c| c.is_ascii_hexdigit()),
            "invalid catalog digest",
        )?;
        ensure(
            key == format!("{PREFIX}{family}/objects/{expected_blake}.jsonl.gz")
                && row["object"] == format!("objects/{expected_blake}.jsonl.gz"),
            "noncanonical object key",
        )?;
        let expected_bytes = number(record, "compressed_bytes")?;
        ensure(
            row["compressed_blake3"] == expected_blake
                && (if family_index == 0 {
                    number(row, "bytes")?
                } else if family_index == 2 {
                    number(row, "bytes")?
                } else {
                    number(row, "compressed_bytes")?
                }) == expected_bytes,
            "manifest/catalog compressed mismatch",
        )?;
        if family_index != 0 {
            ensure(row["s3_uri"] == uri, "manifest URI mismatch")?;
        } else {
            ensure(
                direct_custody["schema"] == "n83.d18-s3-custody/v1"
                    && direct_custody["status"] == "PASS"
                    && direct_custody["curve_id"] == CURVE
                    && direct_custody["s3_uri"] == uri
                    && direct_custody["head_content_length"] == expected_bytes
                    && direct_custody["manifest_expected"]["bytes"] == expected_bytes
                    && direct_custody["manifest_expected"]["blake3"] == expected_blake
                    && direct_custody["manifest_expected"]["sha256"] == record["compressed_sha256"]
                    && direct_custody["fresh_get"] == direct_custody["manifest_expected"],
                "direct D18 custody mismatch",
            )?;
        }
        let blob = blobs.join(format!("{expected_blake}.jsonl.gz"));
        let (actual_bytes, actual_blake, compressed_sha) = hashes(File::open(&blob)?)?;
        ensure(
            actual_bytes == expected_bytes
                && actual_blake == expected_blake
                && record["compressed_sha256"] == compressed_sha,
            "compressed object differs from frozen source",
        )?;
        let (_, plain_blake, plain_sha) = hashes(GzDecoder::new(File::open(&blob)?))?;
        ensure(
            plain_blake == string(row, "plain_blake3")?,
            "plain object differs from manifest",
        )?;
        if family_index == 0 {
            ensure(
                row["compressed_sha256"] == compressed_sha && row["plain_sha256"] == plain_sha,
                "direct D18 SHA256 mismatch",
            )?;
        }
        let store_key = format!("sha256/{}/{}", &compressed_sha[..2], compressed_sha);
        results.push(json!({
            "family": family,
            "old_s3_uri": uri,
            "bytes": expected_bytes,
            "compressed_blake3": expected_blake,
            "plain_blake3": plain_blake,
            "compressed_sha256": compressed_sha,
            "plain_sha256": plain_sha,
            "source_manifest": record["manifest_source"],
            "source_manifest_sha256": manifest_sha,
            "proposed_store_key": store_key,
            "proposed_s3_uri": format!("{REMOTE}/{store_key}"),
            "proposed_store_name": format!("factor-bases/{CURVE}/{family}/{expected_blake}.jsonl.gz"),
        }));
        total_bytes += expected_bytes;
        families[family_index] += 1;
    }
    ensure(
        catalog_keys.len() == 77 && results.len() == 19 && families == [1, 16, 2],
        "unexpected remaining census partition",
    )?;
    ensure(
        catalog_rows.iter().all(|row| {
            let uri = format!(
                "s3://crypto-autoresearcher/{}",
                row["key"].as_str().unwrap_or("")
            );
            previous.contains(&uri) || results.iter().any(|out| out["old_s3_uri"] == uri)
        }),
        "census coverage gap",
    )?;
    results.sort_by(|a, b| a["old_s3_uri"].as_str().cmp(&b["old_s3_uri"].as_str()));
    let output_value = json!({
        "schema": "n83.factor-base-store-migration-inventory-census-remaining/v1",
        "status": "verified_local_source_proposed_destination",
        "publication_status": "not_uploaded",
        "curve_id": CURVE,
        "source_bucket": "crypto-autoresearcher",
        "proposed_remote": REMOTE,
        "catalog_sha256": digest(&catalog_bytes),
        "index_sha256": digest(&index_bytes),
        "direct_custody_sha256": digest(&direct_custody_bytes),
        "object_count": results.len(),
        "family_counts": {"direct_d18": families[0], "standard_subspace": families[1], "size_frontier_v2": families[2]},
        "compressed_bytes": total_bytes,
        "combined_checked_object_count": previous.len() + results.len(),
        "combined_checked_compressed_bytes": pilot_bytes + historical_bytes + total_bytes,
        "objects": results,
    });
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer_pretty(&mut file, &output_value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!("verified 19 remaining census objects, {total_bytes} compressed bytes; no upload");
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    ensure(
        args.len() == 6,
        "usage: census_remaining CENSUS_SOURCE_DIR PILOT_INVENTORY HISTORICAL_INVENTORY DOWNLOADED_OBJECTS_DIR NEW_OUTPUT_JSON",
    )?;
    run(
        Path::new(&args[1]),
        Path::new(&args[2]),
        Path::new(&args[3]),
        Path::new(&args[4]),
        Path::new(&args[5]),
    )
}
