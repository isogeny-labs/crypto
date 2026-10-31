//! Exact export and independent generic replay of the frozen primary N83 d18 orbit base.
//! Usage: n83_d18_subgroup_orbit_export export NEW_DIR | replay EXPORTED_DIR
use crypto_lib::binary_ecc::curve::point_neg;
use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::koblitz_fast_arith::FastBinaryCurve128;
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::Command;
use std::time::Instant;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const SOURCE: &[u8] = include_bytes!("n83_d18_subgroup_orbit_export.rs");
const STUDY: &str = "koblitz_n83_factor_base_sweep_20261008";
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const POLICY: &str = "direct_subgroup_x_d18";
const COLUMNS: usize = 16_627;
const POINTS: usize = COLUMNS * 166;
const CSV: &str = "research/n83_d18_subgroup_orbit_census_20261018/run-20261019T0142Z/selected-representatives.csv";
const CSV_SHA256: &str = "534e58ced979e05ec747eb462aa1d63135d8004813d094c522a4d7160c03c4a4";
const CSV_BLAKE3: &str = "2d5e7b6a5ce1574183d0b236c601f5c2ac5d47008c520b923338fd52e6dc6a44";
const CENSUS_SHA256: &str = "bedf8f59acf5fbed1bf2d2afdf38e992e08fb4f695d6645e5f3c83735b40ba4f";

type Words = (u128, u128);

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn read_selected() -> Result<Vec<(u128, u128, u32)>> {
    let source = fs::read(CSV)?;
    if sha256(&source) != CSV_SHA256 {
        return Err("frozen selected CSV SHA-256 differs".into());
    }
    let content = std::str::from_utf8(&source)?;
    let mut lines = content.lines();
    if lines.next() != Some("orbit_key_decimal,source_x_decimal,source_x_multiplicity") {
        return Err("selected CSV header differs".into());
    }
    let mut selected = Vec::with_capacity(COLUMNS);
    let mut digest = blake3::Hasher::new();
    for line in lines {
        let fields: Vec<_> = line.split(',').collect();
        if fields.len() != 3 {
            return Err("malformed selected CSV row".into());
        }
        let key: u128 = fields[0].parse()?;
        let x: u128 = fields[1].parse()?;
        let multiplicity: u32 = fields[2].parse()?;
        if x >= 1 << 18
            || x <= 1
            || multiplicity == 0
            || multiplicity > 3
            || selected
                .last()
                .is_some_and(|&(previous, _, _)| key <= previous)
        {
            return Err("selected key, x, multiplicity or order differs".into());
        }
        selected.push((key, x, multiplicity));
        digest.update(line.as_bytes());
        digest.update(b"\n");
    }
    if selected.len() != COLUMNS || digest.finalize().to_hex().as_str() != CSV_BLAKE3 {
        return Err("selected CSV count or BLAKE3 differs".into());
    }
    let census = fs::read(
        "research/n83_d18_subgroup_orbit_census_20261018/run-20261019T0142Z/summary.json",
    )?;
    if sha256(&census) != CENSUS_SHA256 {
        return Err("frozen census summary SHA-256 differs".into());
    }
    Ok(selected)
}

fn committed_source() -> Result<String> {
    let head = Command::new("git").args(["rev-parse", "HEAD"]).output()?;
    if !head.status.success() {
        return Err("source commit unavailable".into());
    }
    let commit = String::from_utf8(head.stdout)?.trim().to_owned();
    let show = Command::new("git")
        .args([
            "show",
            &format!("{commit}:examples/n83_d18_subgroup_orbit_export.rs"),
        ])
        .output()?;
    if !show.status.success() || show.stdout != SOURCE {
        return Err("exporter source does not match the clean committed bytes".into());
    }
    Ok(commit)
}

fn words(point: &BinaryPoint) -> Result<Words> {
    let BinaryPoint::Affine { x, y } = point else {
        return Err("identity in factor base".into());
    };
    Ok((
        x.to_biguint().to_u128().ok_or("x outside packed field")?,
        y.to_biguint().to_u128().ok_or("y outside packed field")?,
    ))
}

fn point_json(point: Words) -> Value {
    json!([point.0.to_string(), point.1.to_string()])
}

fn point_from_words(fast: &FastBinaryCurve128, point: Words) -> BinaryPoint {
    BinaryPoint::Affine {
        x: fast.element(point.0),
        y: fast.element(point.1),
    }
}

fn orbit_key(fast: &FastBinaryCurve128, x: u128) -> Result<u128> {
    let mut current = x;
    let mut least = x;
    for phase in 1..=83 {
        current = fast.gf.sqr(current);
        least = least.min(current);
        if current == x {
            if phase != 83 {
                return Err("short Frobenius orbit".into());
            }
            return Ok(least);
        }
    }
    Err("Frobenius orbit failed to close".into())
}

fn point_set_hash(mut points: Vec<Words>) -> String {
    points.sort_unstable();
    let mut hash = blake3::Hasher::new();
    hash.update(b"n83.sorted-polynomial-points/v1");
    for (x, y) in points {
        hash.update(&x.to_le_bytes());
        hash.update(&y.to_le_bytes());
    }
    hash.finalize().to_hex().to_string()
}

struct PlainHashWriter<W> {
    inner: W,
    blake: blake3::Hasher,
    sha: Sha256,
}
impl<W: Write> Write for PlainHashWriter<W> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let n = self.inner.write(bytes)?;
        self.blake.update(&bytes[..n]);
        self.sha.update(&bytes[..n]);
        Ok(n)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

fn compressed_hashes(path: &Path) -> Result<(String, String, u64)> {
    let mut file = File::open(path)?;
    let mut blake = blake3::Hasher::new();
    let mut sha = Sha256::new();
    let mut count = 0u64;
    let mut block = [0u8; 1 << 20];
    loop {
        let n = file.read(&mut block)?;
        if n == 0 {
            break;
        }
        count += n as u64;
        blake.update(&block[..n]);
        sha.update(&block[..n]);
    }
    Ok((
        blake.finalize().to_hex().to_string(),
        format!("{:x}", sha.finalize()),
        count,
    ))
}

fn plain_hashes(path: &Path) -> Result<(String, String, u64)> {
    let mut stream = GzDecoder::new(File::open(path)?);
    let mut blake = blake3::Hasher::new();
    let mut sha = Sha256::new();
    let mut count = 0u64;
    let mut block = [0u8; 1 << 20];
    loop {
        let n = stream.read(&mut block)?;
        if n == 0 {
            break;
        }
        count += n as u64;
        blake.update(&block[..n]);
        sha.update(&block[..n]);
    }
    Ok((
        blake.finalize().to_hex().to_string(),
        format!("{:x}", sha.finalize()),
        count,
    ))
}

fn write_new_json(path: &Path, value: &Value) -> Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn export(root: &Path) -> Result<()> {
    let commit = committed_source()?;
    let selected = read_selected()?;
    let kc = KoblitzCurve::known_n83_k0().ok_or("primary curve unavailable")?;
    if kc.label() != CURVE
        || kc.subgroup_order.to_string() != "2417851639230796216685689"
        || kc.cofactor != BigUint::from(4u8)
        || kc.n != 83
        || !kc.frobenius_is_endomorphism
    {
        return Err("primary curve identity differs".into());
    }
    let fast =
        FastBinaryCurve128::new(&kc.curve.irreducible, 0).ok_or("packed field unavailable")?;
    let started = Instant::now();
    fs::create_dir(root)?;
    fs::create_dir(root.join("objects"))?;
    let partial = root.join("objects/.partial.jsonl.gz");
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&partial)?;
    let mut writer = PlainHashWriter {
        inner: GzEncoder::new(file, Compression::default()),
        blake: blake3::Hasher::new(),
        sha: Sha256::new(),
    };
    let mut reps = Vec::with_capacity(COLUMNS);
    let mut keys = HashSet::with_capacity(COLUMNS);
    for &(key, source_x, _) in &selected {
        if orbit_key(&fast, source_x)? != key || !keys.insert(key) {
            return Err("selected source x or orbit key differs".into());
        }
        let rep = fast
            .points_with_x(1, source_x)
            .first()
            .copied()
            .ok_or("selected x failed to lift")?
            .ok_or("identity representative")?;
        if rep.0 != source_x || fast.scalar_mul(Some(rep), &kc.subgroup_order).is_some() {
            return Err("selected representative is outside direct subgroup".into());
        }
        reps.push(rep);
    }
    let header = json!({
        "schema":"n83.factor-base/v1", "study":STUDY, "curve_slug":CURVE,
        "n":83, "curve_a":0, "curve_b":"1", "modulus_low_terms":[0,1,2,45],
        "field_encoding":"decimal u128 polynomial word; bit i is coefficient of z^i",
        "subgroup_order":kc.subgroup_order.to_string(), "cofactor":kc.cofactor.to_string(),
        "frobenius_lambda":kc.lambda.to_string(), "generator":point_json(words(kc.generator())?),
        "policy":POLICY, "seed":0, "orbit_columns":COLUMNS, "point_count":POINTS,
        "closure":"signed_frobenius", "subspace_membership_after_projection":false,
        "representatives":reps.iter().copied().map(point_json).collect::<Vec<_>>(),
        "selected_source_x":selected.iter().map(|&(_,x,_)|x.to_string()).collect::<Vec<_>>(),
        "selected_orbit_keys":selected.iter().map(|&(key,_,_)|key.to_string()).collect::<Vec<_>>(),
        "selected_csv_sha256":CSV_SHA256, "selected_csv_blake3":CSV_BLAKE3,
        "census_summary_sha256":CENSUS_SHA256, "source_commit":commit,
        "source_blake3":blake3::hash(SOURCE).to_hex().to_string()
    });
    serde_json::to_writer(&mut writer, &header)?;
    writer.write_all(b"\n")?;
    let mut point_set = Vec::with_capacity(POINTS);
    for (column, &(mut x, mut y)) in reps.iter().enumerate() {
        let original = (x, y);
        let mut coefficient = BigUint::from(1u8);
        for phase in 0..83 {
            for negative in [false, true] {
                let point = (x, if negative { x ^ y } else { y });
                let label = if negative {
                    &kc.subgroup_order - &coefficient
                } else {
                    coefficient.clone()
                };
                serde_json::to_writer(
                    &mut writer,
                    &json!({
                        "point":point_json(point), "column":column, "phase":phase,
                        "negative":negative, "coefficient":label.to_string()
                    }),
                )?;
                writer.write_all(b"\n")?;
                point_set.push(point);
            }
            x = fast.gf.sqr(x);
            y = fast.gf.sqr(y);
            coefficient = (&coefficient * &kc.lambda) % &kc.subgroup_order;
        }
        if (x, y) != original {
            return Err("representative orbit failed to close".into());
        }
    }
    if point_set.len() != POINTS {
        return Err("point count differs".into());
    }
    let set_hash = point_set_hash(point_set);
    let PlainHashWriter { inner, blake, sha } = writer;
    let plain_blake3 = blake.finalize().to_hex().to_string();
    let plain_sha256 = format!("{:x}", sha.finalize());
    inner.finish()?.sync_all()?;
    let (compressed_blake3, compressed_sha256, bytes) = compressed_hashes(&partial)?;
    let object = format!("objects/{compressed_blake3}.jsonl.gz");
    fs::rename(&partial, root.join(&object))?;
    let manifest = json!({
        "schema":"n83.direct-subgroup-orbit-export/v1", "status":"EXPORTED_UNREPLAYED",
        "study":STUDY, "curve_id":CURVE, "policy":POLICY, "source_commit":commit,
        "source_blake3":blake3::hash(SOURCE).to_hex().to_string(),
        "selected_csv_sha256":CSV_SHA256, "selected_csv_blake3":CSV_BLAKE3,
        "census_summary_sha256":CENSUS_SHA256, "columns":COLUMNS, "points":POINTS,
        "point_set_blake3":set_hash, "object":object, "plain_blake3":plain_blake3,
        "plain_sha256":plain_sha256, "compressed_blake3":compressed_blake3,
        "compressed_sha256":compressed_sha256, "bytes":bytes,
        "construction_elapsed_ms_informational_L0":started.elapsed().as_secs_f64()*1000.0,
        "relation_yield":Value::Null, "matrix_rank":Value::Null,
        "total_index_calculus_runtime_ms":Value::Null, "selected_best_total_runtime":Value::Null
    });
    write_new_json(&root.join("manifest.json"), &manifest)?;
    println!("{}", serde_json::to_string(&manifest)?);
    Ok(())
}

fn replay(root: &Path) -> Result<()> {
    let selected = read_selected()?;
    let manifest: Value = serde_json::from_slice(&fs::read(root.join("manifest.json"))?)?;
    if manifest["schema"] != "n83.direct-subgroup-orbit-export/v1"
        || manifest["status"] != "EXPORTED_UNREPLAYED"
        || manifest["curve_id"] != CURVE
        || manifest["policy"] != POLICY
        || manifest["columns"] != COLUMNS
        || manifest["points"] != POINTS
        || manifest["selected_csv_sha256"] != CSV_SHA256
        || manifest["selected_csv_blake3"] != CSV_BLAKE3
        || manifest["census_summary_sha256"] != CENSUS_SHA256
        || manifest["selected_best_total_runtime"] != Value::Null
    {
        return Err("manifest identity differs".into());
    }
    let object = manifest["object"].as_str().ok_or("object path absent")?;
    if object
        != format!(
            "objects/{}.jsonl.gz",
            manifest["compressed_blake3"]
                .as_str()
                .ok_or("compressed hash absent")?
        )
    {
        return Err("content-addressed object name differs".into());
    }
    let path = root.join(object);
    let source_commit = manifest["source_commit"]
        .as_str()
        .ok_or("source commit absent")?;
    let source = Command::new("git")
        .args([
            "show",
            &format!("{source_commit}:examples/n83_d18_subgroup_orbit_export.rs"),
        ])
        .output()?;
    if !source.status.success()
        || blake3::hash(&source.stdout).to_hex().as_str()
            != manifest["source_blake3"]
                .as_str()
                .ok_or("source hash absent")?
    {
        return Err("committed producer source hash differs".into());
    }
    let (plain_blake, plain_sha, _) = plain_hashes(&path)?;
    if manifest["plain_blake3"] != plain_blake || manifest["plain_sha256"] != plain_sha {
        return Err("plain object hash differs".into());
    }
    let (blake, sha, bytes) = compressed_hashes(&path)?;
    if manifest["compressed_blake3"] != blake
        || manifest["compressed_sha256"] != sha
        || manifest["bytes"] != bytes
    {
        return Err("compressed object hash or length differs".into());
    }
    let kc = KoblitzCurve::known_n83_k0().ok_or("primary curve unavailable")?;
    let fast =
        FastBinaryCurve128::new(&kc.curve.irreducible, 0).ok_or("packed field unavailable")?;
    let mut reader = BufReader::new(GzDecoder::new(File::open(&path)?));
    let mut line = String::new();
    if reader.read_line(&mut line)? == 0 {
        return Err("object header absent".into());
    }
    let header: Value = serde_json::from_str(&line)?;
    if header["schema"] != "n83.factor-base/v1"
        || header["study"] != STUDY
        || header["curve_slug"] != CURVE
        || header["n"] != 83
        || header["curve_a"] != 0
        || header["curve_b"] != "1"
        || header["modulus_low_terms"] != json!([0, 1, 2, 45])
        || header["subgroup_order"] != kc.subgroup_order.to_string()
        || header["cofactor"] != kc.cofactor.to_string()
        || header["frobenius_lambda"] != kc.lambda.to_string()
        || header["generator"] != point_json(words(kc.generator())?)
        || header["policy"] != POLICY
        || header["seed"] != 0
        || header["orbit_columns"] != COLUMNS
        || header["point_count"] != POINTS
        || header["closure"] != "signed_frobenius"
        || header["selected_csv_sha256"] != CSV_SHA256
        || header["selected_csv_blake3"] != CSV_BLAKE3
        || header["census_summary_sha256"] != CENSUS_SHA256
        || header["source_commit"] != manifest["source_commit"]
        || header["source_blake3"] != manifest["source_blake3"]
    {
        return Err("object header or curve identity differs".into());
    }
    let reps = header["representatives"]
        .as_array()
        .ok_or("representative array absent")?;
    let xs = header["selected_source_x"]
        .as_array()
        .ok_or("source x array absent")?;
    let keys = header["selected_orbit_keys"]
        .as_array()
        .ok_or("orbit key array absent")?;
    if reps.len() != COLUMNS || xs.len() != COLUMNS || keys.len() != COLUMNS {
        return Err("header selection count differs".into());
    }
    let mut seen = HashSet::with_capacity(POINTS);
    let mut points = Vec::with_capacity(POINTS);
    let started = Instant::now();
    for (column, &(key, source_x, _)) in selected.iter().enumerate() {
        if xs[column] != source_x.to_string()
            || keys[column] != key.to_string()
            || orbit_key(&fast, source_x)? != key
        {
            return Err("source x selection differs".into());
        }
        let rep = reps[column]
            .as_array()
            .ok_or("representative point absent")?;
        if rep.len() != 2 {
            return Err("representative shape differs".into());
        }
        let x: u128 = rep[0].as_str().ok_or("representative x absent")?.parse()?;
        let y: u128 = rep[1].as_str().ok_or("representative y absent")?.parse()?;
        if x != source_x
            || x >= 1u128 << 83
            || y >= 1u128 << 83
            || fast.points_with_x(1, source_x).first().copied() != Some(Some((x, y)))
        {
            return Err("representative source x, lift or field bound differs".into());
        }
        let original = point_from_words(&fast, (x, y));
        if !kc.curve.is_on_curve(&original)
            || kc.mul(&original, &kc.subgroup_order) != BinaryPoint::Infinity
            || kc.mul(&original, &kc.lambda) != kc.frobenius(&original)
        {
            return Err("generic representative subgroup or eigenvalue check failed".into());
        }
        let mut current = original.clone();
        let mut coefficient = BigUint::from(1u8);
        for phase in 0..83 {
            for negative in [false, true] {
                line.clear();
                if reader.read_line(&mut line)? == 0 {
                    return Err("point row missing".into());
                }
                let row: Value = serde_json::from_str(&line)?;
                let point = if negative {
                    point_neg(&current)
                } else {
                    current.clone()
                };
                let words = words(&point)?;
                let label = if negative {
                    &kc.subgroup_order - &coefficient
                } else {
                    coefficient.clone()
                };
                if row["point"] != point_json(words)
                    || row["column"] != column
                    || row["phase"] != phase
                    || row["negative"] != negative
                    || row["coefficient"] != label.to_string()
                    || !seen.insert(words)
                {
                    return Err("generic row point, label or uniqueness check failed".into());
                }
                points.push(words);
            }
            current = kc.frobenius(&current);
            coefficient = (&coefficient * &kc.lambda) % &kc.subgroup_order;
        }
        if current != original {
            return Err("generic orbit closure failed".into());
        }
    }
    line.clear();
    if reader.read_line(&mut line)? != 0
        || seen.len() != POINTS
        || point_set_hash(points) != manifest["point_set_blake3"]
    {
        return Err("trailing row, point count or set hash differs".into());
    }
    let result = json!({
        "schema":"n83.direct-subgroup-orbit-replay/v1", "status":"PASS",
        "object":object, "compressed_sha256":sha, "compressed_blake3":blake,
        "selected_csv_sha256":CSV_SHA256, "representatives_checked":COLUMNS,
        "points_checked":POINTS, "point_set_blake3":manifest["point_set_blake3"],
        "generic_backend":"binary_ecc multi-limb curve, subgroup and Frobenius",
        "replay_elapsed_ms_informational_L0":started.elapsed().as_secs_f64()*1000.0,
        "selected_best_total_runtime":Value::Null
    });
    write_new_json(&root.join("replay.json"), &result)?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err(
            "usage: n83_d18_subgroup_orbit_export export NEW_DIR | replay EXPORTED_DIR".into(),
        );
    }
    match args[1].as_str() {
        "export" => export(Path::new(&args[2])),
        "replay" => replay(Path::new(&args[2])),
        _ => Err("mode must be export or replay".into()),
    }
}
