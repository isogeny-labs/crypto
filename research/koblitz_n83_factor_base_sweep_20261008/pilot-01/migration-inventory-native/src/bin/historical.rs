use flate2::read::GzDecoder;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const OLD_PREFIX: &str =
    "s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008";
const STUDY: &str = "koblitz_n83_factor_base_sweep_20261008";
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const PROPOSED_REMOTE: &str = "s3://setec-astronomy-artifacts/store";

struct Source {
    name: &'static str,
    source_path: &'static str,
    schema: &'static str,
    old_family: &'static str,
}

const SOURCES: [Source; 5] = [
    Source {
        name: "k1182",
        source_path: "verification/two-hour-extension-20261010/v2-object/manifest.json",
        schema: "n83.factor-base-panel/v2-size-frontier",
        old_family: "v2-size-frontier",
    },
    Source {
        name: "k2048",
        source_path: "verification/k2048-orbit-capacity-20261011/input/manifest.json",
        schema: "n83.factor-base-panel/v2-size-frontier",
        old_family: "v2-size-frontier",
    },
    Source {
        name: "d12",
        source_path: "verification/structured-d12-orbit-20261010/manifest.json",
        schema: "n83.structured-orbit-panel/v1",
        old_family: "v3-structured-orbit",
    },
    Source {
        name: "standard_d18",
        source_path: "verification/standard-d18-s3-20261015/manifest.json",
        schema: "n83.standard-subspace-factor-base-manifest/v1",
        old_family: "standard-subspace",
    },
    Source {
        name: "full_d18",
        source_path: "verification/d18-full-orbit-20261025/manifest.json",
        schema: "n83.direct-subgroup-orbit-export/v1",
        old_family: "d18-full-direct-subgroup-v1",
    },
];

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

fn stream_hashes(mut reader: impl Read) -> Result<(u64, String, String)> {
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

fn run(source_dir: &Path, blobs_dir: &Path, output: &Path) -> Result<()> {
    ensure(!output.exists(), "inventory already exists")?;
    let custody_bytes = fs::read(source_dir.join("full_d18_custody.json"))?;
    let custody: Value = serde_json::from_slice(&custody_bytes)?;
    ensure(
        custody["schema"] == "n83.d18-full-s3-custody/v1"
            && custody["status"] == "PASS_EXACT_FETCH"
            && custody["bucket"] == "crypto-autoresearcher",
        "full D18 custody receipt is not an exact successful fetch",
    )?;
    let mut seen = HashSet::new();
    let mut rows = Vec::new();
    let mut total_bytes = 0u64;
    for source in SOURCES {
        let manifest_bytes = fs::read(source_dir.join(format!("{}.json", source.name)))?;
        let manifest: Value = serde_json::from_slice(&manifest_bytes)?;
        ensure(
            manifest["schema"] == source.schema,
            "manifest schema mismatch",
        )?;
        if source.name != "standard_d18" {
            ensure(manifest["study"] == STUDY, "manifest study mismatch")?;
        }
        let row = if source.schema == "n83.factor-base-panel/v2-size-frontier" {
            ensure(
                manifest["status"] == "completed_factor_base_object"
                    && manifest["completed_base_count"] == 1
                    && manifest["expected_base_count"] == 1,
                "size-frontier object is not complete",
            )?;
            manifest["bases"]
                .as_array()
                .and_then(|bases| {
                    if bases.len() == 1 {
                        bases.first()
                    } else {
                        None
                    }
                })
                .ok_or("expected one size-frontier base")?
        } else {
            &manifest
        };
        if source.name == "d12" {
            ensure(
                manifest["status"] == "completed_factor_base_object",
                "structured D12 status mismatch",
            )?;
        }
        if source.name == "standard_d18" {
            ensure(
                manifest["curve_slug"] == CURVE,
                "standard D18 curve mismatch",
            )?;
        }
        if source.name == "full_d18" {
            ensure(
                manifest["curve_id"] == CURVE && manifest["status"] == "EXPORTED_UNREPLAYED",
                "full D18 manifest identity mismatch",
            )?;
        }
        if source.name == "k1182" || source.name == "k2048" {
            ensure(row["a"] == 0, "size-frontier curve mismatch")?;
        }
        let expected_bytes = if source.name == "full_d18" {
            number(row, "bytes")?
        } else if source.name == "k1182" || source.name == "k2048" {
            number(row, "bytes")?
        } else {
            number(row, "compressed_bytes")?
        };
        let expected_blake = string(row, "compressed_blake3")?;
        ensure(
            expected_blake.len() == 64 && expected_blake.bytes().all(|c| c.is_ascii_hexdigit()),
            "invalid compressed digest",
        )?;
        let old_key = format!(
            "factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/{}/a0/objects/{expected_blake}.jsonl.gz",
            source.old_family
        );
        ensure(
            row["object"] == format!("objects/{expected_blake}.jsonl.gz"),
            "noncanonical manifest object path",
        )?;
        let old_uri = format!("s3://crypto-autoresearcher/{old_key}");
        ensure(
            old_uri.starts_with(OLD_PREFIX),
            "old source prefix mismatch",
        )?;
        if source.name == "full_d18" {
            ensure(
                custody["key"] == old_key
                    && custody["bytes"] == expected_bytes
                    && custody["compressed_blake3"] == expected_blake
                    && custody["compressed_sha256"] == manifest["compressed_sha256"],
                "full D18 custody receipt mismatch",
            )?;
        } else {
            ensure(row["s3_uri"] == old_uri, "old S3 URI mismatch")?;
        }
        ensure(seen.insert(old_uri.clone()), "duplicate historical object")?;
        let blob = blobs_dir.join(format!("{}.jsonl.gz", source.name));
        let (actual_bytes, actual_blake, compressed_sha) = stream_hashes(File::open(&blob)?)?;
        ensure(
            actual_bytes == expected_bytes && actual_blake == expected_blake,
            "compressed object differs from frozen manifest",
        )?;
        let (_, plain_blake, plain_sha) = stream_hashes(GzDecoder::new(File::open(&blob)?))?;
        ensure(
            plain_blake == string(row, "plain_blake3")?,
            "decompressed object differs from frozen manifest",
        )?;
        if source.name == "full_d18" {
            ensure(
                manifest["compressed_sha256"] == compressed_sha
                    && manifest["plain_sha256"] == plain_sha,
                "full D18 SHA256 mismatch",
            )?;
        }
        let store_key = format!("sha256/{}/{}", &compressed_sha[..2], compressed_sha);
        rows.push(json!({
            "name": source.name,
            "source_manifest": format!("research/koblitz_n83_factor_base_sweep_20261008/{}", source.source_path),
            "source_manifest_sha256": format!("{:x}", Sha256::digest(&manifest_bytes)),
            "source_status": manifest["status"],
            "old_s3_uri": old_uri,
            "bytes": expected_bytes,
            "compressed_blake3": expected_blake,
            "plain_blake3": plain_blake,
            "compressed_sha256": compressed_sha,
            "plain_sha256": plain_sha,
            "proposed_store_name": format!("factor-bases/{CURVE}/{}/a0/{expected_blake}.jsonl.gz", source.old_family),
            "proposed_store_key": store_key,
            "proposed_s3_uri": format!("{PROPOSED_REMOTE}/{store_key}"),
        }));
        total_bytes += expected_bytes;
    }
    ensure(
        rows.len() == 5 && seen.len() == 5,
        "wrong historical inventory size",
    )?;
    let inventory = json!({
        "schema": "n83.factor-base-store-migration-inventory-historical/v1",
        "status": "verified_local_source_proposed_destination",
        "publication_status": "not_uploaded",
        "curve_id": CURVE,
        "source_bucket": "crypto-autoresearcher",
        "proposed_remote": PROPOSED_REMOTE,
        "object_count": rows.len(),
        "compressed_bytes": total_bytes,
        "full_d18_custody_sha256": format!("{:x}", Sha256::digest(&custody_bytes)),
        "objects": rows,
    });
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer_pretty(&mut file, &inventory)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!(
        "verified {} historical objects, {total_bytes} compressed bytes; no upload",
        seen.len()
    );
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    ensure(
        args.len() == 4,
        "usage: historical SOURCE_MANIFESTS_DIR DOWNLOADED_OBJECTS_DIR NEW_OUTPUT_JSON",
    )?;
    run(
        Path::new(&args[1]),
        Path::new(&args[2]),
        Path::new(&args[3]),
    )
}
