use crate::{arithmetic::*, recipe::*};
use anyhow::{ensure, Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

pub fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
pub fn canonical<T: Serialize>(v: &T) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec(&serde_json::to_value(v)?)?)
}
pub fn source_digest() -> String {
    digest(
        [
            include_str!("lib.rs"),
            include_str!("arithmetic.rs"),
            include_str!("recipe.rs"),
            include_str!("store.rs"),
            include_str!("evaluate.rs"),
            include_str!("main.rs"),
            include_str!("../Cargo.toml"),
            include_str!("../Cargo.lock"),
        ]
        .join("\n")
        .as_bytes(),
    )
}
pub fn valid_id(id: &str) -> Result<()> {
    ensure!(
        id.len() == 64
            && id
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "expected lowercase SHA-256 identifier"
    );
    Ok(())
}
pub fn read_limited(path: &Path, limit: u64) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    fs::File::open(path)?
        .take(limit + 1)
        .read_to_end(&mut data)?;
    ensure!(data.len() as u64 <= limit, "file exceeds size limit");
    Ok(data)
}
pub fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    Ok(serde_json::from_slice(&read_limited(
        path,
        32 * 1024 * 1024,
    )?)?)
}
pub fn write_once(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::create_dir_all(path.parent().context("no parent")?)?;
    if path.exists() {
        ensure!(
            read_limited(path, bytes.len() as u64)? == bytes,
            "immutable path has different content: {}",
            path.display()
        );
        return Ok(());
    }
    let mut counter = 0;
    let (tmp, mut f) = loop {
        let tmp = path.with_extension(format!("tmp-{}-{counter}", std::process::id()));
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&tmp)
        {
            Ok(f) => break (tmp, f),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => counter += 1,
            Err(e) => return Err(e.into()),
        }
    };
    let result = (|| {
        f.write_all(bytes)?;
        f.sync_all()?;
        drop(f);
        match fs::hard_link(&tmp, path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                ensure!(
                    read_limited(path, bytes.len() as u64)? == bytes,
                    "immutable object conflict"
                );
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    })();
    let _ = fs::remove_file(tmp);
    result
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Object {
    pub sha256: String,
    pub bytes: usize,
    pub points: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: String,
    pub curve_sha256: String,
    pub recipe_sha256: String,
    pub source_sha256: String,
    pub config: Config,
    pub requirements: Value,
    pub structure: Value,
    pub point_count: usize,
    pub objects: Vec<Object>,
}
pub fn object_path(root: &Path, id: &str) -> Result<PathBuf> {
    valid_id(id)?;
    Ok(root.join("objects").join(format!("{id}.jsonl")))
}
pub fn manifest_path(root: &Path, id: &str) -> Result<PathBuf> {
    valid_id(id)?;
    Ok(root.join("manifests").join(format!("{id}.json")))
}
pub fn build(config: &Config, root: &Path) -> Result<String> {
    let g = generate(config)?;
    let mut objects = Vec::new();
    for chunk in g.points.chunks(config.shard_points) {
        let mut bytes = Vec::new();
        for p in chunk {
            bytes.extend(canonical(&p.spec())?);
            bytes.push(b'\n');
        }
        let id = digest(&bytes);
        write_once(&object_path(root, &id)?, &bytes)?;
        objects.push(Object {
            sha256: id,
            bytes: bytes.len(),
            points: chunk.len(),
        });
    }
    let mut config = config.clone();
    config.curve = g.curve.spec.clone();
    config.recipe = g.recipe;
    let m = Manifest {
        schema: "factor-base-manifest/v1".into(),
        curve_sha256: digest(&canonical(&config.curve)?),
        recipe_sha256: digest(&canonical(&config.recipe)?),
        source_sha256: source_digest(),
        config,
        requirements: g.curve.requirements(),
        structure: g.structure,
        point_count: g.points.len(),
        objects,
    };
    let bytes = canonical(&m)?;
    let id = digest(&bytes);
    write_once(&manifest_path(root, &id)?, &bytes)?;
    Ok(id)
}
pub fn load(root: &Path, id: &str) -> Result<(Manifest, Vec<Point>)> {
    let data = read_limited(&manifest_path(root, id)?, 32 * 1024 * 1024)?;
    ensure!(digest(&data) == id, "manifest hash mismatch");
    let m: Manifest = serde_json::from_slice(&data)?;
    ensure!(
        m.schema == "factor-base-manifest/v1",
        "unknown archive schema"
    );
    ensure!(
        m.point_count <= 2_000_000 && m.objects.len() <= 2_000_000,
        "archive exceeds supported limits"
    );
    let totals = m
        .objects
        .iter()
        .try_fold((0usize, 0usize), |(points, bytes), o| {
            Some((points.checked_add(o.points)?, bytes.checked_add(o.bytes)?))
        })
        .context("archive totals overflow")?;
    ensure!(
        totals.0 == m.point_count && totals.1 <= 2_000_000_000,
        "shard totals disagree with manifest or exceed archive cap"
    );
    valid_id(&m.source_sha256)?;
    ensure!(
        m.curve_sha256 == digest(&canonical(&m.config.curve)?)
            && m.recipe_sha256 == digest(&canonical(&m.config.recipe)?),
        "identity digest mismatch"
    );
    let mut points = Vec::new();
    for obj in &m.objects {
        ensure!(
            obj.points > 0 && obj.points <= 65536 && obj.bytes <= 32 * 1024 * 1024,
            "invalid shard limits"
        );
        let bytes = read_limited(&object_path(root, &obj.sha256)?, obj.bytes as u64)?;
        ensure!(
            bytes.len() == obj.bytes && digest(&bytes) == obj.sha256,
            "shard hash or size mismatch"
        );
        let start = points.len();
        for line in bytes.split(|b| *b == b'\n').filter(|l| !l.is_empty()) {
            points.push(Point::from_spec(&serde_json::from_slice::<PointSpec>(
                line,
            )?)?);
        }
        ensure!(points.len() - start == obj.points, "shard count mismatch");
    }
    ensure!(
        points.len() == m.point_count && points.windows(2).all(|p| p[0] < p[1]),
        "point count/order/uniqueness mismatch"
    );
    Ok((m, points))
}
pub fn verify(root: &Path, id: &str) -> Result<(Manifest, Generated)> {
    let (m, points) = load(root, id)?;
    let g = generate(&m.config)?;
    ensure!(
        g.points == points,
        "recipe replay differs from stored points"
    );
    ensure!(
        g.structure == m.structure && g.curve.requirements() == m.requirements,
        "structural or validation metadata mismatch"
    );
    Ok((m, g))
}
pub fn plan(root: &Path, id: &str, prefix: &str) -> Result<Value> {
    ensure!(
        !prefix.starts_with('/')
            && !prefix.contains("..")
            && prefix
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"/-_".contains(&b)),
        "invalid S3 prefix"
    );
    let (m, _) = verify(root, id)?;
    let prefix = prefix.trim_end_matches('/');
    let key = |s: String| {
        if prefix.is_empty() {
            s
        } else {
            format!("{prefix}/{s}")
        }
    };
    let mut objects = Vec::new();
    for o in &m.objects {
        objects.push(json!({"key":key(format!("objects/{}.jsonl",o.sha256)),"local_path":object_path(root,&o.sha256)?,"sha256":o.sha256,"bytes":o.bytes}));
    }
    let bytes = read_limited(&manifest_path(root, id)?, 32 * 1024 * 1024)?;
    objects.push(json!({"key":key(format!("manifests/{id}.json")),"local_path":manifest_path(root,id)?,"sha256":id,"bytes":bytes.len()}));
    Ok(
        json!({"schema":"factor-base-upload-plan/v1","manifest_sha256":id,"commit_order":"objects_then_manifest","if_none_match":"*","checksum_algorithm":"SHA256","objects":objects}),
    )
}
pub fn publish(root: &Path, id: &str, bucket: &str, prefix: &str) -> Result<Value> {
    publish_with(root, id, bucket, prefix, Path::new("aws"))
}
pub fn publish_with(
    root: &Path,
    id: &str,
    bucket: &str,
    prefix: &str,
    aws: &Path,
) -> Result<Value> {
    ensure!(
        (3..=63).contains(&bucket.len())
            && bucket
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b".-".contains(&b)),
        "invalid S3 bucket"
    );
    let plan = plan(root, id, prefix)?;
    let mut receipts = Vec::new();
    for obj in plan["objects"].as_array().unwrap() {
        let path = Path::new(obj["local_path"].as_str().unwrap());
        let data = read_limited(path, 32 * 1024 * 1024)?;
        ensure!(
            digest(&data) == obj["sha256"].as_str().unwrap(),
            "local object changed during publication"
        );
        let checksum = STANDARD.encode(Sha256::digest(&data));
        let key = obj["key"].as_str().unwrap();
        let out = Command::new(aws)
            .args([
                "s3api",
                "put-object",
                "--bucket",
                bucket,
                "--key",
                key,
                "--body",
            ])
            .arg(path)
            .args([
                "--if-none-match",
                "*",
                "--checksum-algorithm",
                "SHA256",
                "--checksum-sha256",
                &checksum,
                "--server-side-encryption",
                "AES256",
                "--output",
                "json",
                "--no-cli-pager",
            ])
            .output()
            .context("AWS CLI v2 required on trusted upload controller")?;
        if !out.status.success() {
            let error = String::from_utf8_lossy(&out.stderr);
            if error.contains("PreconditionFailed") || error.contains("(412)") {
                let head = Command::new(aws)
                    .args([
                        "s3api",
                        "head-object",
                        "--bucket",
                        bucket,
                        "--key",
                        key,
                        "--checksum-mode",
                        "ENABLED",
                        "--output",
                        "json",
                        "--no-cli-pager",
                    ])
                    .output()?;
                ensure!(
                    head.status.success(),
                    "existing S3 object could not be checked"
                );
                let meta: Value = serde_json::from_slice(&head.stdout)?;
                ensure!(
                    meta["ChecksumSHA256"] == checksum
                        && meta["ContentLength"].as_u64() == Some(data.len() as u64),
                    "existing object checksum/size mismatch or checksum unavailable"
                );
                receipts.push(json!({"key":key,"status":"already_present_checksum_verified","version_id":meta["VersionId"],"sha256":obj["sha256"]}));
                continue;
            }
            anyhow::bail!("S3 PUT failed for {key}: {error}");
        }
        let meta: Value = serde_json::from_slice(&out.stdout)?;
        ensure!(
            meta["ChecksumSHA256"] == checksum,
            "S3 response did not confirm SHA256"
        );
        receipts.push(json!({"key":key,"status":"created","version_id":meta["VersionId"],"etag":meta["ETag"],"sha256":obj["sha256"]}));
    }
    Ok(
        json!({"schema":"factor-base-publication/v1","bucket":bucket,"manifest_sha256":id,"objects":receipts}),
    )
}

pub fn fetch(root: &Path, id: &str, bucket: &str, prefix: &str) -> Result<Value> {
    fetch_with(root, id, bucket, prefix, Path::new("aws"))
}
pub fn fetch_with(root: &Path, id: &str, bucket: &str, prefix: &str, aws: &Path) -> Result<Value> {
    valid_id(id)?;
    ensure!(
        (3..=63).contains(&bucket.len())
            && bucket
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b".-".contains(&b)),
        "invalid S3 bucket"
    );
    ensure!(
        !prefix.starts_with('/')
            && !prefix.contains("..")
            && prefix
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"/-_".contains(&b)),
        "invalid S3 prefix"
    );
    let key = |suffix: String| {
        if prefix.is_empty() {
            suffix
        } else {
            format!("{}/{suffix}", prefix.trim_end_matches('/'))
        }
    };
    let manifest = manifest_path(root, id)?;
    download(
        aws,
        &manifest,
        &key(format!("manifests/{id}.json")),
        bucket,
        id,
        32 * 1024 * 1024,
    )?;
    let m: Manifest = read_json(&manifest)?;
    ensure!(
        m.schema == "factor-base-manifest/v1" && m.objects.len() <= 2_000_000,
        "unsupported manifest"
    );
    let total: usize = m
        .objects
        .iter()
        .try_fold(0usize, |sum, o| sum.checked_add(o.bytes))
        .context("archive size overflow")?;
    ensure!(total <= 2_000_000_000, "download exceeds 2GB archive cap");
    for o in &m.objects {
        ensure!(o.bytes <= 32 * 1024 * 1024, "shard exceeds size cap");
        download(
            aws,
            &object_path(root, &o.sha256)?,
            &key(format!("objects/{}.jsonl", o.sha256)),
            bucket,
            &o.sha256,
            o.bytes as u64,
        )?;
    }
    let (_, g) = verify(root, id)?;
    Ok(
        json!({"status":"downloaded_and_replayed","manifest_sha256":id,"points":g.points.len(),"downloaded_object_bytes":total,"bucket":bucket}),
    )
}
fn download(
    aws: &Path,
    destination: &Path,
    key: &str,
    bucket: &str,
    id: &str,
    max: u64,
) -> Result<()> {
    if destination.exists() {
        let data = read_limited(destination, max)?;
        ensure!(digest(&data) == id, "corrupt local cache object");
        return Ok(());
    }
    let head = Command::new(aws)
        .args([
            "s3api",
            "head-object",
            "--bucket",
            bucket,
            "--key",
            key,
            "--output",
            "json",
            "--no-cli-pager",
        ])
        .output()?;
    ensure!(head.status.success(), "S3 HEAD failed for {key}");
    let meta: Value = serde_json::from_slice(&head.stdout)?;
    ensure!(
        meta["ContentLength"].as_u64().is_some_and(|n| n <= max),
        "remote object exceeds size cap"
    );
    let etag = meta["ETag"].as_str().context("missing S3 ETag")?;
    fs::create_dir_all(destination.parent().unwrap())?;
    let mut counter = 0;
    let temp = loop {
        let p = destination.with_extension(format!("fetch-{}-{counter}", std::process::id()));
        match fs::OpenOptions::new().create_new(true).write(true).open(&p) {
            Ok(_) => break p,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => counter += 1,
            Err(e) => return Err(e.into()),
        }
    };
    let result = (|| {
        let mut command = Command::new(aws);
        command.args([
            "s3api",
            "get-object",
            "--bucket",
            bucket,
            "--key",
            key,
            "--if-match",
            etag,
            "--output",
            "json",
            "--no-cli-pager",
        ]);
        if let Some(v) = meta["VersionId"].as_str() {
            command.args(["--version-id", v]);
        }
        let out = command.arg(&temp).output()?;
        ensure!(out.status.success(), "S3 GET failed for {key}");
        let data = read_limited(&temp, max)?;
        ensure!(digest(&data) == id, "download checksum mismatch");
        write_once(destination, &data)
    })();
    let _ = fs::remove_file(temp);
    result
}
