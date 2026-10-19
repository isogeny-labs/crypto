//! Native bounded relation-stage and setup accounting for two exact N83 orbit bases.
//! See research/n83_prebuild_cost_m23_20261019/PROTOCOL.md before running.
use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::koblitz_fast_arith::{FastBinaryCurve128, FastPoint128};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use flate2::read::GzDecoder;
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Words = (u128, u128);
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const ORDER: &str = "2417851639230796216685689";
const COLUMNS: usize = 16_627;
const POINTS: usize = COLUMNS * 166;
const U_OBJECT: &str = "1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d";
const D_OBJECT: &str = "e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682";
const U_POINT_SET: &str = "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0";
const D_POINT_SET: &str = "8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0";
const D_LABELED_ROWS: &str = "805d1cbc65cac0a74ebf49315b8c17b0d5229bdceba5725ba4349fb1cc6c9e9c";
const K2048_OBJECT: &str = "82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851";
const D12_OBJECT: &str = "aea278aff2d79a592d5164d7437ddde8f8e9377fa481058dfc16afa176a31389";
const K2048_SET: &str = "d528f5828a05a00add4252212bc909ece481f33093e4074ff7dd8c6a51d3c97e";
const D12_SET: &str = "fb8ea2ecfeb245a90fecd5506dfc01592916b4ebec756d5fea7cbdd0c25537ce";
const PUBLIC_SHA256: &str = "2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5";
const PUBLIC_0: Words = (1834913121178197242856238, 8797964190732846213981377);

#[derive(Clone)]
struct Row {
    point: Words,
    column: usize,
    coefficient: BigUint,
}

struct Base {
    rows: Vec<Row>,
    index: HashMap<Words, usize>,
    object_blake3: String,
    point_set_blake3: String,
}

fn curve() -> Result<(KoblitzCurve, FastBinaryCurve128)> {
    let kc = KoblitzCurve::known_n83_k0().ok_or("primary N83 fixture absent")?;
    if kc.label() != CURVE || kc.subgroup_order.to_string() != ORDER || kc.cofactor != 4u8.into() {
        return Err("curve identity differs".into());
    }
    let fast = FastBinaryCurve128::new(&kc.curve.irreducible, 0).ok_or("packed N83 unavailable")?;
    Ok((kc, fast))
}

fn parse_words(value: &Value) -> Result<Words> {
    let words = value.as_array().ok_or("point must be a two-word array")?;
    if words.len() != 2 {
        return Err("point word count differs".into());
    }
    let x: u128 = words[0]
        .as_str()
        .ok_or("point x must be decimal")?
        .parse()?;
    let y: u128 = words[1]
        .as_str()
        .ok_or("point y must be decimal")?
        .parse()?;
    if x >= (1u128 << 83) || y >= (1u128 << 83) {
        return Err("point outside GF(2^83)".into());
    }
    Ok((x, y))
}

fn point_set_hash(points: &mut [Words]) -> String {
    points.sort_unstable();
    let mut hash = blake3::Hasher::new();
    hash.update(b"n83.sorted-polynomial-points/v1");
    for (x, y) in points {
        hash.update(&x.to_le_bytes());
        hash.update(&y.to_le_bytes());
    }
    hash.finalize().to_hex().to_string()
}

fn load_base(arm: &str, root: &Path) -> Result<Base> {
    let (
        expected_object,
        expected_set,
        policy,
        seed,
        columns,
        point_count,
        manifest_schema,
        header_schema,
        expected_sha,
    ) = match arm {
        "U" => (
            U_OBJECT,
            U_POINT_SET,
            "public_x_hash",
            2026100801u64,
            COLUMNS,
            POINTS,
            "n83.factor-base-panel/v2-size-frontier",
            "n83.factor-base/v1",
            None,
        ),
        "D" => (
            D_OBJECT,
            D_POINT_SET,
            "direct_subgroup_x_d18",
            0u64,
            COLUMNS,
            POINTS,
            "n83.direct-subgroup-orbit-export/v1",
            "n83.factor-base/v1",
            Some("347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7"),
        ),
        "K2048" => (
            K2048_OBJECT,
            K2048_SET,
            "public_x_hash",
            2026100801u64,
            2048,
            339968,
            "n83.factor-base-panel/v2-size-frontier",
            "n83.factor-base/v1",
            Some("aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a"),
        ),
        "D12" => (
            D12_OBJECT,
            D12_SET,
            "",
            0u64,
            2001,
            332166,
            "n83.structured-orbit-panel/v1",
            "n83.structured-orbit-base/v1",
            Some("c5de0b26e84ad80bfb7be6a214b89c9682e356effb6079c7abf28cd1151ffb03"),
        ),
        _ => return Err("arm must be U, D, K2048 or D12".into()),
    };
    let manifest: Value = serde_json::from_slice(&fs::read(root.join("manifest.json"))?)?;
    let selected = if matches!(arm, "U" | "K2048") {
        if manifest["schema"] != manifest_schema {
            return Err("uniform panel schema differs".into());
        }
        let rows = manifest["bases"]
            .as_array()
            .ok_or("uniform panel rows absent")?;
        let matching: Vec<_> = rows
            .iter()
            .filter(|row| {
                row["columns"] == columns && row["policy"] == policy && row["seed"] == seed
            })
            .collect();
        if matching.len() != 1 {
            return Err("uniform panel selection is ambiguous".into());
        }
        matching[0]
    } else {
        if manifest["schema"] != manifest_schema {
            return Err("direct or structured manifest schema differs".into());
        }
        &manifest
    };
    let object = format!("objects/{expected_object}.jsonl.gz");
    if selected["object"] != object || selected["point_set_blake3"] != expected_set {
        return Err("manifest object or point-set identity differs".into());
    }
    let compressed = fs::read(root.join(&object))?;
    if blake3::hash(&compressed).to_hex().as_str() != expected_object {
        return Err("compressed object digest differs".into());
    }
    if let Some(expected_sha) = expected_sha {
        if format!("{:x}", Sha256::digest(&compressed)) != expected_sha {
            return Err("compressed SHA-256 differs".into());
        }
    }
    let mut reader = BufReader::new(GzDecoder::new(compressed.as_slice()));
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let header: Value = serde_json::from_str(&line)?;
    if header["schema"] != header_schema
        || header["curve_slug"] != CURVE
        || (arm != "D12" && header["policy"] != policy)
        || (arm != "D12" && header["seed"] != seed)
        || header["subgroup_order"] != ORDER
        || header["orbit_columns"] != columns
        || header["point_count"] != point_count
        || header["closure"] != "signed_frobenius"
    {
        return Err("factor-base header identity differs".into());
    }
    let mut rows = Vec::with_capacity(point_count);
    let mut index = HashMap::with_capacity(point_count);
    let mut points = Vec::with_capacity(point_count);
    let subgroup_order = BigUint::parse_bytes(ORDER.as_bytes(), 10).ok_or("subgroup order")?;
    for ordinal in 0..point_count {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            return Err("factor-base row missing".into());
        }
        let row: Value = serde_json::from_str(&line)?;
        let point = parse_words(&row["point"])?;
        let column = row["column"].as_u64().ok_or("column absent")? as usize;
        let phase = row["phase"].as_u64().ok_or("phase absent")? as usize;
        let negative = row["negative"].as_bool().ok_or("sign absent")?;
        if column != ordinal / 166 || phase != (ordinal % 166) / 2 || negative != (ordinal % 2 == 1)
        {
            return Err("row label order differs".into());
        }
        let coefficient = BigUint::parse_bytes(
            row["coefficient"]
                .as_str()
                .ok_or("coefficient absent")?
                .as_bytes(),
            10,
        )
        .ok_or("coefficient malformed")?;
        if coefficient == BigUint::from(0u8) || coefficient >= subgroup_order {
            return Err("coefficient outside subgroup".into());
        }
        if index.insert(point, ordinal).is_some() {
            return Err("duplicate factor-base point".into());
        }
        rows.push(Row {
            point,
            column,
            coefficient,
        });
        points.push(point);
    }
    line.clear();
    if reader.read_line(&mut line)? != 0 {
        return Err("extra factor-base row".into());
    }
    let set_hash = point_set_hash(&mut points);
    if set_hash != expected_set {
        return Err("point-set digest differs".into());
    }
    Ok(Base {
        rows,
        index,
        object_blake3: expected_object.into(),
        point_set_blake3: set_hash,
    })
}

fn direct_orbit_key(fast: &FastBinaryCurve128, x: u128) -> Result<u128> {
    let mut current = x;
    let mut least = x;
    for phase in 1..=83 {
        current = fast.gf.sqr(current);
        least = least.min(current);
        if current == x {
            if phase != 83 {
                return Err("direct source has short orbit".into());
            }
            return Ok(least);
        }
    }
    Err("direct source orbit did not close".into())
}

fn generate_d18() -> Result<(Base, f64, f64)> {
    let (kc, fast) = curve()?;
    let enumeration = Instant::now();
    let mut classes = BTreeMap::<u128, u128>::new();
    for x in 0..(1u128 << 18) {
        let Some(point) = fast.points_with_x(1, x).first().copied().flatten() else {
            continue;
        };
        if fast.scalar_mul(Some(point), &kc.subgroup_order).is_some() {
            continue;
        }
        let key = direct_orbit_key(&fast, x)?;
        classes
            .entry(key)
            .and_modify(|source| *source = (*source).min(x))
            .or_insert(x);
    }
    if classes.len() != 32_643 {
        return Err("direct source class count differs".into());
    }
    let enumeration_ms = enumeration.elapsed().as_secs_f64() * 1000.0;
    let closure = Instant::now();
    let mut rows = Vec::with_capacity(POINTS);
    let mut index = HashMap::with_capacity(POINTS);
    let mut points = Vec::with_capacity(POINTS);
    for (column, (_, source_x)) in classes.iter().take(COLUMNS).enumerate() {
        let Some((mut x, mut y)) = fast.points_with_x(1, *source_x).first().copied().flatten()
        else {
            return Err("selected direct source failed to lift".into());
        };
        let original = (x, y);
        let mut coefficient = BigUint::from(1u8);
        for _phase in 0..83 {
            for negative in [false, true] {
                let point = (x, if negative { x ^ y } else { y });
                let label = if negative {
                    &kc.subgroup_order - &coefficient
                } else {
                    coefficient.clone()
                };
                if index.insert(point, rows.len()).is_some() {
                    return Err("generated point duplicated".into());
                }
                rows.push(Row {
                    point,
                    column,
                    coefficient: label,
                });
                points.push(point);
            }
            x = fast.gf.sqr(x);
            y = fast.gf.sqr(y);
            coefficient = (&coefficient * &kc.lambda) % &kc.subgroup_order;
        }
        if (x, y) != original {
            return Err("generated orbit did not close".into());
        }
    }
    if rows.len() != POINTS {
        return Err("generated point count differs".into());
    }
    let set_hash = point_set_hash(&mut points);
    if set_hash != D_POINT_SET {
        return Err("generated point-set digest differs".into());
    }
    let closure_index_ms = closure.elapsed().as_secs_f64() * 1000.0;
    Ok((
        Base {
            rows,
            index,
            object_blake3: D_OBJECT.into(),
            point_set_blake3: set_hash,
        },
        enumeration_ms,
        closure_index_ms,
    ))
}

fn labeled_rows_hash(base: &Base) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"n83.labeled-orbit-rows/v1");
    for row in &base.rows {
        hasher.update(&row.point.0.to_le_bytes());
        hasher.update(&row.point.1.to_le_bytes());
        hasher.update(&(row.column as u64).to_le_bytes());
        let digits = row.coefficient.to_bytes_le();
        hasher.update(&(digits.len() as u32).to_le_bytes());
        hasher.update(&digits);
    }
    hasher.finalize().to_hex().to_string()
}

fn resource_usage() -> (Option<u64>, Option<f64>) {
    // SAFETY: getrusage fills this initialized plain-data structure for this process.
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut usage) } != 0 {
        return (None, None);
    }
    let rss = u64::try_from(usage.ru_maxrss).ok().map(|raw| {
        #[cfg(target_os = "macos")]
        {
            raw
        }
        #[cfg(not(target_os = "macos"))]
        {
            raw.saturating_mul(1024)
        }
    });
    let tv = |t: libc::timeval| t.tv_sec as f64 + t.tv_usec as f64 * 1e-6;
    (rss, Some(tv(usage.ru_utime) + tv(usage.ru_stime)))
}

fn setup(mode: &str, root: &Path, output: &Path) -> Result<()> {
    let started = Instant::now();
    let (base, enumeration_ms, closure_index_ms) = match mode {
        "prebuilt" => (load_base("D", root)?, 0.0, 0.0),
        "ondemand" => generate_d18()?,
        _ => return Err("setup mode must be prebuilt or ondemand".into()),
    };
    let setup_ms = started.elapsed().as_secs_f64() * 1000.0;
    let validation = Instant::now();
    let label_hash = labeled_rows_hash(&base);
    let validation_ms = validation.elapsed().as_secs_f64() * 1000.0;
    let (peak_rss_bytes, process_cpu_seconds) = resource_usage();
    write_new_json(
        output,
        &json!({"schema":"n83.prebuild-setup/v1","mode":mode,
        "status":"PASS","curve_id":CURVE,"columns":COLUMNS,"points":POINTS,
        "point_set_blake3":base.point_set_blake3,"labeled_rows_blake3":label_hash,
        "object_blake3":base.object_blake3,"enumeration_ms":enumeration_ms,
        "closure_index_ms":closure_index_ms,"setup_ms":setup_ms,
        "validation_ms":validation_ms,"process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
        "peak_rss_bytes":peak_rss_bytes,"process_cpu_seconds":process_cpu_seconds,
        "candidate_id":Value::Null,"selected_best_total_runtime":Value::Null}),
    )?;
    Ok(())
}

fn write_new_json(path: &Path, value: &Value) -> Result<()> {
    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn words(point: &BinaryPoint, fast: &FastBinaryCurve128) -> Result<Words> {
    let BinaryPoint::Affine { x, y } = point else {
        return Err("point is infinity".into());
    };
    Ok((fast.word(x), fast.word(y)))
}

fn targets(public: &Path, validation: &Path) -> Result<()> {
    let (kc, fast) = curve()?;
    let generator = words(kc.generator(), &fast)?;
    let mut points = vec![PUBLIC_0];
    let mut scalars = vec![Value::Null];
    let order = kc.subgroup_order.clone();
    for i in 1u32..8 {
        let mut bytes = Vec::from(&b"N83-M23-PUBLIC-v1"[..]);
        bytes.extend_from_slice(&20261019u64.to_le_bytes());
        bytes.extend_from_slice(&i.to_le_bytes());
        let mut scalar = BigUint::from_bytes_le(blake3::hash(&bytes).as_bytes()) % &order;
        if scalar == BigUint::from(0u8) {
            scalar = BigUint::from(1u8);
        }
        let point = fast
            .scalar_mul(Some(generator), &scalar)
            .ok_or("derived target is infinity")?;
        points.push(point);
        scalars.push(json!(scalar.to_string()));
    }
    for point in &points {
        if fast.scalar_mul(Some(*point), &order).is_some() {
            return Err("target outside subgroup".into());
        }
    }
    let public_value = json!({"schema":"n83.m23-public-targets/v1", "curve_id":CURVE,
        "seed":20261019, "points":points.iter().map(|&(x,y)|json!([x.to_string(),y.to_string()])).collect::<Vec<_>>()});
    write_new_json(public, &public_value)?;
    write_new_json(
        validation,
        &json!({"schema":"n83.m23-validation/v1", "public_blake3":
        blake3::hash(&fs::read(public)?).to_hex().to_string(), "scalars_decimal":scalars,
        "target_0_validation":"research/koblitz_n83_factor_base_sweep_20261008/pilot-01/probe-validation.json"}),
    )?;
    Ok(())
}

fn public_targets(path: &Path) -> Result<Vec<Words>> {
    let bytes = fs::read(path)?;
    if format!("{:x}", Sha256::digest(&bytes)) != PUBLIC_SHA256 {
        return Err("frozen public corpus SHA-256 differs".into());
    }
    let value: Value = serde_json::from_slice(&bytes)?;
    if value["schema"] != "n83.m23-public-targets/v1"
        || value["curve_id"] != CURVE
        || value["seed"] != 20261019
    {
        return Err("public corpus identity differs".into());
    }
    let points = value["points"].as_array().ok_or("target points absent")?;
    if points.len() != 8 || parse_words(&points[0])? != PUBLIC_0 {
        return Err("target count or fixture 0 differs".into());
    }
    let parsed: Vec<Words> = points.iter().map(parse_words).collect::<Result<_>>()?;
    let (kc, fast) = curve()?;
    for &point in &parsed {
        if !fast.points_with_x(1, point.0).contains(&Some(point))
            || fast.scalar_mul(Some(point), &kc.subgroup_order).is_some()
        {
            return Err("public target is not on the exact subgroup curve".into());
        }
    }
    Ok(parsed)
}

fn relation_row(rows: &[Row], indices: &[usize], order: &BigUint) -> BTreeMap<usize, BigUint> {
    let mut result = BTreeMap::new();
    for &index in indices {
        let row = &rows[index];
        let entry = result
            .entry(row.column)
            .or_insert_with(|| BigUint::from(0u8));
        *entry = (&*entry + &row.coefficient) % order;
    }
    result.retain(|_, value| *value != BigUint::from(0u8));
    result
}

struct RankTracker {
    pivots: BTreeMap<usize, BTreeMap<usize, BigUint>>,
    order: BigUint,
}
impl RankTracker {
    fn new(order: BigUint) -> Self {
        Self {
            pivots: BTreeMap::new(),
            order,
        }
    }
    fn insert(&mut self, mut row: BTreeMap<usize, BigUint>) -> bool {
        while let Some((&pivot, coefficient)) = row.first_key_value() {
            let coefficient = coefficient.clone();
            if let Some(existing) = self.pivots.get(&pivot) {
                for (&column, value) in existing {
                    let product = (&coefficient * value) % &self.order;
                    let current = row.remove(&column).unwrap_or_default();
                    let next = if current >= product {
                        current - product
                    } else {
                        &self.order - (product - current)
                    };
                    if next != BigUint::from(0u8) {
                        row.insert(column, next);
                    }
                }
            } else {
                let inverse = coefficient.modpow(&(&self.order - 2u8), &self.order);
                for value in row.values_mut() {
                    *value = (&*value * &inverse) % &self.order;
                }
                self.pivots.insert(pivot, row);
                return true;
            }
        }
        false
    }
    fn rank(&self) -> usize {
        self.pivots.len()
    }
}

fn verify_relation(
    fast: &FastBinaryCurve128,
    base: &Base,
    target: Words,
    indices: &[usize],
    rank: &mut RankTracker,
) -> Result<(bool, String)> {
    let mut sum: FastPoint128 = None;
    for &index in indices {
        sum = fast.add(sum, Some(base.rows[index].point));
    }
    if sum != Some(target) {
        return Err("returned relation fails group equation".into());
    }
    let row = relation_row(&base.rows, indices, &rank.order);
    let canonical: Vec<_> = row
        .iter()
        .map(|(&column, coefficient)| (column, coefficient.to_string()))
        .collect();
    let digest = blake3::hash(&serde_json::to_vec(&canonical)?)
        .to_hex()
        .to_string();
    Ok((rank.insert(row), digest))
}

fn pair_words(seed: u64, target: u32, ordinal: u64, point_count: usize) -> (usize, usize) {
    let mut input = Vec::from(&b"N83-M3-PAIRS-v1"[..]);
    input.extend_from_slice(&seed.to_le_bytes());
    input.extend_from_slice(&target.to_le_bytes());
    input.extend_from_slice(&ordinal.to_le_bytes());
    let mut xof = blake3::Hasher::new();
    xof.update(&input);
    let mut reader = xof.finalize_xof();
    let mut word = [0u8; 8];
    reader.fill(&mut word);
    let first = (u64::from_le_bytes(word) % point_count as u64) as usize;
    let second = loop {
        reader.fill(&mut word);
        let candidate = (u64::from_le_bytes(word) % point_count as u64) as usize;
        if candidate != first {
            break candidate;
        }
    };
    (first, second)
}

fn probe(
    prepare_mode: &str,
    arm: &str,
    root: &Path,
    corpus: &Path,
    target_index: usize,
    m: usize,
    output: &Path,
) -> Result<()> {
    if ![2, 3].contains(&m) || target_index >= 8 {
        return Err("m or target outside protocol".into());
    }
    if !matches!(prepare_mode, "prebuilt" | "ondemand")
        || (prepare_mode == "ondemand" && arm != "D")
    {
        return Err("base preparation mode outside protocol".into());
    }
    let started = Instant::now();
    let progress = PathBuf::from(format!("{}.progress.jsonl", output.display()));
    let relation_receipts = PathBuf::from(format!("{}.relations.jsonl", output.display()));
    let mut progress_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&progress)?;
    let mut relation_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&relation_receipts)?;
    serde_json::to_writer(
        &mut progress_file,
        &json!({
            "schema":"n83.m23-progress/v1","phase":if prepare_mode=="prebuilt" {"import"} else {"prepare"},
            "arm":arm,"m":m,"target_index":target_index,"completed":0,
            "base_prepare_mode":prepare_mode,
            "elapsed_ms":0.0
        }),
    )?;
    progress_file.write_all(b"\n")?;
    progress_file.sync_all()?;
    let (base, enumeration_ms, closure_index_ms) = if prepare_mode == "ondemand" {
        generate_d18()?
    } else {
        (load_base(arm, root)?, 0.0, 0.0)
    };
    let prepare_ms = started.elapsed().as_secs_f64() * 1000.0;
    let label_validation_started = Instant::now();
    let label_hash = labeled_rows_hash(&base);
    if arm == "D" && label_hash != D_LABELED_ROWS {
        return Err("D labeled-row digest differs".into());
    }
    let label_validation_ms = label_validation_started.elapsed().as_secs_f64() * 1000.0;
    let public_validation_started = Instant::now();
    let public = public_targets(corpus)?;
    let target = public[target_index];
    let (kc, fast) = curve()?;
    let public_validation_ms = public_validation_started.elapsed().as_secs_f64() * 1000.0;
    let mut rank = RankTracker::new(kc.subgroup_order.clone());
    let mut seen = HashSet::<Vec<usize>>::new();
    let mut checked = 0usize;
    let mut duplicate = 0usize;
    let mut relations = 0usize;
    let mut independent = 0usize;
    let total = if m == 2 { base.rows.len() } else { 1 << 20 };
    let mut last_flush = Instant::now();
    let (peak_rss_bytes, process_cpu_seconds) = resource_usage();
    serde_json::to_writer(
        &mut progress_file,
        &json!({
            "schema":"n83.m23-progress/v1","phase":"query",
            "arm":arm,"m":m,"target_index":target_index,"completed":0,
            "total":total,"lookups":0,"relations":0,"independent_rows":0,"rank":0,
            "elapsed_ms":started.elapsed().as_secs_f64()*1000.0,
            "peak_rss_bytes":peak_rss_bytes,"process_cpu_seconds":process_cpu_seconds
        }),
    )?;
    progress_file.write_all(b"\n")?;
    progress_file.sync_all()?;
    let query_started = Instant::now();
    for ordinal in 0..total {
        let pair = if m == 2 {
            None
        } else {
            Some(pair_words(
                20261019,
                target_index as u32,
                ordinal as u64,
                base.rows.len(),
            ))
        };
        let first = pair.map_or(ordinal, |value| value.0);
        let mut complement = fast.add(
            Some(target),
            FastBinaryCurve128::neg(Some(base.rows[first].point)),
        );
        let mut indices = vec![first];
        if let Some((_, second)) = pair {
            complement = fast.add(
                complement,
                FastBinaryCurve128::neg(Some(base.rows[second].point)),
            );
            indices.push(second);
        }
        if let Some(point) = complement {
            if let Some(&third) = base.index.get(&point) {
                indices.push(third);
                indices.sort_unstable();
                if seen.insert(indices.clone()) {
                    relations += 1;
                    let (added, row_digest) =
                        verify_relation(&fast, &base, target, &indices, &mut rank)?;
                    if added {
                        independent += 1;
                    }
                    serde_json::to_writer(
                        &mut relation_file,
                        &json!({
                            "schema":"n83.m23-relation/v1","arm":arm,"m":m,
                            "target_index":target_index,"attempt_ordinal":ordinal,
                            "point_indices":indices,"sparse_row_blake3":row_digest,
                            "rank_after_insert":rank.rank(),"independent":added,
                            "group_verified":true
                        }),
                    )?;
                    relation_file.write_all(b"\n")?;
                    relation_file.sync_all()?;
                } else {
                    duplicate += 1;
                }
            }
        }
        checked += 1;
        if last_flush.elapsed() >= Duration::from_secs(10) || checked == total {
            let (peak_rss_bytes, process_cpu_seconds) = resource_usage();
            let sample = json!({"schema":"n83.m23-progress/v1", "phase":"query",
                "arm":arm,"m":m,"target_index":target_index,"completed":checked,
                "total":total,"lookups":checked,
                "relations":relations,"independent_rows":independent,"rank":rank.rank(),
                "elapsed_ms":started.elapsed().as_secs_f64()*1000.0,
                "peak_rss_bytes":peak_rss_bytes,"process_cpu_seconds":process_cpu_seconds});
            serde_json::to_writer(&mut progress_file, &sample)?;
            progress_file.write_all(b"\n")?;
            progress_file.sync_all()?;
            last_flush = Instant::now();
        }
        if started.elapsed() >= Duration::from_secs(1790) {
            break;
        }
    }
    let status = if checked == total {
        "COMPLETE"
    } else {
        "UNKNOWN_budget"
    };
    let query_ms = query_started.elapsed().as_secs_f64() * 1000.0;
    let (peak_rss_bytes, process_cpu_seconds) = resource_usage();
    write_new_json(
        output,
        &json!({"schema":"n83.m23-probe/v1", "status":status,
        "arm":arm,"m":m,"target_index":target_index,"completed":checked,"total":total,
        "relations":relations,"duplicate_relation_hits":duplicate,
        "independent_rows":independent,"rank":rank.rank(),
        "base_prepare_mode":prepare_mode,"base_prepare_ms":prepare_ms,
        "base_import_index_ms":if prepare_mode=="prebuilt" {Some(prepare_ms)} else {None},
        "on_demand_enumeration_ms":enumeration_ms,"on_demand_closure_index_ms":closure_index_ms,
        "label_validation_ms":label_validation_ms,"labeled_rows_blake3":label_hash,
        "public_validation_ms":public_validation_ms,"query_ms":query_ms,
        "progress_file":progress.file_name().ok_or("progress name")?.to_string_lossy(),
        "relation_receipts_file":relation_receipts.file_name().ok_or("relation receipt name")?.to_string_lossy(),
        "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
        "peak_rss_bytes":peak_rss_bytes,"process_cpu_seconds":process_cpu_seconds,
        "object_blake3":base.object_blake3,"point_set_blake3":base.point_set_blake3,
        "public_corpus_blake3":blake3::hash(&fs::read(corpus)?).to_hex().to_string(),
        "candidate_id":Value::Null,"selected_best_total_runtime":Value::Null}),
    )?;
    Ok(())
}

fn replay(arm: &str, root: &Path, corpus: &Path, worker: &Path, output: &Path) -> Result<()> {
    let receipt: Value = serde_json::from_slice(&fs::read(worker)?)?;
    let target_index = receipt["target_index"]
        .as_u64()
        .ok_or("target index absent")? as usize;
    let m = receipt["m"].as_u64().ok_or("summand count absent")? as usize;
    if receipt["schema"] != "n83.m23-probe/v1"
        || receipt["arm"] != arm
        || ![2, 3].contains(&m)
        || target_index >= 8
        || !["COMPLETE", "UNKNOWN_budget"].contains(&receipt["status"].as_str().unwrap_or(""))
        || receipt["public_corpus_blake3"] != blake3::hash(&fs::read(corpus)?).to_hex().to_string()
    {
        return Err("worker receipt identity or status differs".into());
    }
    let prepare_mode = receipt["base_prepare_mode"].as_str().unwrap_or("prebuilt");
    if !matches!(prepare_mode, "prebuilt" | "ondemand")
        || (prepare_mode == "ondemand" && arm != "D")
    {
        return Err("worker base preparation mode differs".into());
    }
    let expected_points = match arm {
        "U" | "D" => POINTS,
        "K2048" => 339_968,
        "D12" => 332_166,
        _ => return Err("replay arm outside frozen bases".into()),
    };
    let expected_total = if m == 2 { expected_points } else { 1 << 20 };
    let completed = receipt["completed"]
        .as_u64()
        .ok_or("completed count absent")?;
    if receipt["total"] != expected_total
        || completed > expected_total as u64
        || (receipt["status"] == "COMPLETE") != (completed == expected_total as u64)
    {
        return Err("worker completion count or status differs".into());
    }
    let progress_path = PathBuf::from(format!("{}.progress.jsonl", worker.display()));
    let progress_lines: Vec<Value> = BufReader::new(std::fs::File::open(progress_path)?)
        .lines()
        .map(|line| Ok(serde_json::from_str::<Value>(&line?)?))
        .collect::<Result<_>>()?;
    if progress_lines.len() < 2
        || !["import", "prepare"].contains(&progress_lines[0]["phase"].as_str().unwrap_or(""))
        || progress_lines[1]["phase"] != "query"
        || progress_lines
            .last()
            .and_then(|sample| sample["completed"].as_u64())
            != Some(completed)
    {
        return Err("durable progress missing or incomplete".into());
    }
    let mut prior_progress = 0u64;
    for sample in &progress_lines {
        let current = sample["completed"]
            .as_u64()
            .ok_or("progress count absent")?;
        if sample["schema"] != "n83.m23-progress/v1"
            || sample["arm"] != arm
            || sample["m"] != m
            || sample["target_index"] != target_index
            || current < prior_progress
            || current > completed
        {
            return Err("progress identity or count differs".into());
        }
        prior_progress = current;
    }
    let base = load_base(arm, root)?;
    if receipt["object_blake3"] != base.object_blake3
        || receipt["point_set_blake3"] != base.point_set_blake3
    {
        return Err("worker base identity differs".into());
    }
    if receipt["labeled_rows_blake3"].is_string()
        && receipt["labeled_rows_blake3"] != labeled_rows_hash(&base)
    {
        return Err("worker labeled rows differ from independent archive".into());
    }
    let public = public_targets(corpus)?;
    let target = public[target_index];
    let (kc, fast) = curve()?;
    let generator = words(kc.generator(), &fast)?;
    let changed_target = fast
        .add(Some(target), Some(generator))
        .ok_or("changed target is infinity")?;
    let mut rank = RankTracker::new(kc.subgroup_order);
    let mut seen = HashSet::new();
    let mut relations = 0usize;
    let mut independent = 0usize;
    let mut prior_ordinal = None;
    let path = PathBuf::from(format!("{}.relations.jsonl", worker.display()));
    for line in BufReader::new(std::fs::File::open(&path)?).lines() {
        let entry: Value = serde_json::from_str(&line?)?;
        if entry["schema"] != "n83.m23-relation/v1"
            || entry["arm"] != arm
            || entry["m"] != m
            || entry["target_index"] != target_index
        {
            return Err("relation receipt identity differs".into());
        }
        let indices: Vec<usize> = entry["point_indices"]
            .as_array()
            .ok_or("indices absent")?
            .iter()
            .map(|value| value.as_u64().map(|x| x as usize).ok_or("index malformed"))
            .collect::<std::result::Result<_, _>>()?;
        let ordinal = entry["attempt_ordinal"]
            .as_u64()
            .ok_or("attempt ordinal absent")?;
        if ordinal >= completed || prior_ordinal.is_some_and(|prior| ordinal <= prior) {
            return Err("attempt ordinal outside completed prefix or out of order".into());
        }
        prior_ordinal = Some(ordinal);
        let expected = if m == 2 {
            vec![ordinal as usize]
        } else {
            let (first, second) =
                pair_words(20261019, target_index as u32, ordinal, expected_points);
            vec![first, second]
        };
        if indices.len() != m
            || indices.iter().any(|&index| index >= expected_points)
            || indices.windows(2).any(|pair| pair[0] > pair[1])
            || expected.iter().any(|index| !indices.contains(index))
            || !seen.insert(indices.clone())
        {
            return Err("relation indices invalid or repeated".into());
        }
        let (added, digest) = verify_relation(&fast, &base, target, &indices, &mut rank)?;
        if entry["sparse_row_blake3"] != digest
            || entry["independent"] != added
            || entry["rank_after_insert"] != rank.rank()
        {
            return Err("relation row digest or rank differs".into());
        }
        let mut disposable = RankTracker::new(rank.order.clone());
        if verify_relation(&fast, &base, changed_target, &indices, &mut disposable).is_ok() {
            return Err("changed target was accepted".into());
        }
        let mut changed_indices = indices.clone();
        changed_indices[0] = (changed_indices[0] + 1) % expected_points;
        if verify_relation(&fast, &base, target, &changed_indices, &mut disposable).is_ok() {
            return Err("changed row was accepted".into());
        }
        relations += 1;
        if added {
            independent += 1;
        }
    }
    if receipt["relations"] != relations
        || receipt["independent_rows"] != independent
        || receipt["rank"] != rank.rank()
    {
        return Err("worker relation or rank totals differ".into());
    }
    write_new_json(
        output,
        &json!({"schema":"n83.m23-replay/v1","status":"PASS",
        "arm":arm,"m":m,"target_index":target_index,"relations_checked":relations,
        "independent_rows":independent,"rank":rank.rank(),
        "changed_target_rejected":relations,"changed_row_rejected":relations,
        "worker_blake3":blake3::hash(&fs::read(worker)?).to_hex().to_string(),
        "candidate_id":Value::Null,"selected_best_total_runtime":Value::Null}),
    )?;
    Ok(())
}

fn control(arm: &str, root: &Path, m: usize, output: &Path) -> Result<()> {
    if ![2, 3].contains(&m) {
        return Err("control m must be 2 or 3".into());
    }
    let base = load_base(arm, root)?;
    let (kc, fast) = curve()?;
    let indices: Vec<usize> = (0..m).map(|i| i * 166).collect();
    let mut target: FastPoint128 = None;
    for &index in &indices {
        target = fast.add(target, Some(base.rows[index].point));
    }
    let target = target.ok_or("constructed target is infinity")?;
    let mut rank = RankTracker::new(kc.subgroup_order.clone());
    let (independent, row_digest) = verify_relation(&fast, &base, target, &indices, &mut rank)?;
    if !independent || rank.rank() != 1 {
        return Err("constructed relation has no useful rank".into());
    }
    let mut tampered = RankTracker::new(rank.order.clone());
    let changed_target = fast
        .add(Some(target), Some(words(kc.generator(), &fast)?))
        .ok_or("changed target is infinity")?;
    if verify_relation(&fast, &base, changed_target, &indices, &mut tampered).is_ok() {
        return Err("changed target was accepted".into());
    }
    let mut changed_indices = indices.clone();
    changed_indices[0] += 1;
    if verify_relation(&fast, &base, target, &changed_indices, &mut tampered).is_ok() {
        return Err("changed row was accepted".into());
    }
    write_new_json(
        output,
        &json!({
            "schema":"n83.m23-control/v1","status":"PASS","arm":arm,"m":m,
            "target":[target.0.to_string(),target.1.to_string()],
        "point_indices":indices,"sparse_row_blake3":row_digest,
            "rank":1,"changed_target_rejected":true,"changed_row_rejected":true,
            "object_blake3":base.object_blake3,"point_set_blake3":base.point_set_blake3,
            "candidate_id":Value::Null,"selected_best_total_runtime":Value::Null
        }),
    )
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("targets") if args.len()==4 => targets(Path::new(&args[2]),Path::new(&args[3])),
        Some("setup") if args.len()==5 => setup(&args[2],Path::new(&args[3]),Path::new(&args[4])),
        Some("probe") if args.len()==8 => probe("prebuilt",&args[2],Path::new(&args[3]),Path::new(&args[4]),
            args[5].parse()?,args[6].parse()?,Path::new(&args[7])),
        Some("probe-mode") if args.len()==9 => probe(&args[2],&args[3],Path::new(&args[4]),Path::new(&args[5]),
            args[6].parse()?,args[7].parse()?,Path::new(&args[8])),
        Some("replay") if args.len()==7 => replay(&args[2],Path::new(&args[3]),Path::new(&args[4]),Path::new(&args[5]),Path::new(&args[6])),
        Some("control") if args.len()==6 => control(&args[2],Path::new(&args[3]),args[4].parse()?,Path::new(&args[5])),
        _ => Err("usage: n83_prebuild_cost_m23 targets NEW_PUBLIC_JSON NEW_VALIDATION_JSON | setup prebuilt|ondemand D_BASE_DIR NEW_RESULT_JSON | probe U|D|K2048|D12 BASE_DIR PUBLIC_JSON TARGET_INDEX 2|3 NEW_RESULT_JSON | probe-mode prebuilt|ondemand D BASE_DIR PUBLIC_JSON TARGET_INDEX 2|3 NEW_RESULT_JSON | replay U|D|K2048|D12 BASE_DIR PUBLIC_JSON WORKER_JSON NEW_REPLAY_JSON | control U|D|K2048|D12 BASE_DIR 2|3 NEW_RESULT_JSON".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic_pairs_are_distinct_and_stable() {
        assert_eq!(
            pair_words(20261019, 0, 17, POINTS),
            pair_words(20261019, 0, 17, POINTS)
        );
        for size in [POINTS, 339_968, 332_166] {
            for ordinal in 0..1000 {
                let (a, b) = pair_words(20261019, 0, ordinal, size);
                assert_ne!(a, b);
                assert!(a < size && b < size);
            }
        }
    }
    #[test]
    fn rank_rejects_a_scaled_duplicate() {
        let order = BigUint::parse_bytes(ORDER.as_bytes(), 10).unwrap();
        let mut rank = RankTracker::new(order);
        assert!(rank.insert(BTreeMap::from([
            (1, BigUint::from(2u8)),
            (5, BigUint::from(3u8))
        ])));
        assert!(!rank.insert(BTreeMap::from([
            (1, BigUint::from(4u8)),
            (5, BigUint::from(6u8))
        ])));
        assert_eq!(rank.rank(), 1);
    }
}
