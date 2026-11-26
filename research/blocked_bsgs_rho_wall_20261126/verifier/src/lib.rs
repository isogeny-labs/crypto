use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const STUDY_ID: &str = "blocked-bsgs-rho-wall/v1";
pub const MASTER_HEX: &str = "1aaea7ece2a35976043741b3a869b6ddc9a3798b18596b0635ee52829e3bba38";
pub const BSGS_SHA256: &str = "ebe890abcbdcfb2e719eff99e046847f4f1efb0864f627ba2ad2a073a5f4a7bb";
pub const RHO_SHA256: &str = "6a9dc015b1558045935912688beebf141a6b4161c045c821faac81fc3a430bc1";
pub const SEED_MANIFEST_SHA256: &str =
    "45fa0e04de6c91c567dea346849fc74c12d9537814124086649dc47ade8e6c5f";
pub const PROTOCOL_MD_SHA256: &str =
    "53548103cbf898dab3f39bf0de8bebb5ced07184855baa82556079395e58c773";
pub const PROTOCOL_ADDENDUM_1_SHA256: &str =
    "45cf548ae2c1fa71e55f713e933f74b743052bb403fd1846767026eef4082801";
pub const PROTOCOL_ADDENDUM_2_SHA256: &str =
    "d581de654830d1730eac4a3019e323752f5dc98cbb4d7276e9943c954191a1c1";
pub const NATIVE_CARGO_LOCK_SHA256: &str =
    "cd56d62407c6ef09b5fbd0cdf187b2fd684c609a1b2df42b301a8eb67c699b17";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fixture {
    pub slug: &'static str,
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub gx: u64,
    pub gy: u64,
    pub n: u64,
}

pub const FIXTURES: [Fixture; 5] = [
    Fixture {
        slug: "icv1-fp24-tm5099-a6f137c6",
        p: 16_777_199,
        a: 16_777_196,
        b: 31,
        gx: 2,
        gy: 5_412_647,
        n: 16_782_299,
    },
    Fixture {
        slug: "icv1-fp28-t28201-65099091",
        p: 268_435_399,
        a: 268_435_396,
        b: 3,
        gx: 1,
        gy: 1,
        n: 268_407_199,
    },
    Fixture {
        slug: "icv1-fp32-t7319-90da43e9",
        p: 4_294_967_291,
        a: 4_294_967_288,
        b: 31,
        gx: 2,
        gy: 2_120_051_858,
        n: 4_294_959_973,
    },
    Fixture {
        slug: "icv1-fp36-t454193-44a20818",
        p: 68_719_476_731,
        a: 68_719_476_728,
        b: 33,
        gx: 2,
        gy: 51_741_851_039,
        n: 68_719_022_539,
    },
    Fixture {
        slug: "icv1-fp40-tm506303-75549727",
        p: 649_522_587_953,
        a: 552_810_359_288,
        b: 555_863_716_954,
        gx: 549_251_065_028,
        gy: 609_044_633_619,
        n: 649_523_094_257,
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Point {
    Infinity,
    Affine { x: u64, y: u64 },
}

fn fixture(slug: &str) -> Result<Fixture, String> {
    FIXTURES
        .iter()
        .copied()
        .find(|candidate| candidate.slug == slug)
        .ok_or_else(|| format!("unregistered curve slug {slug:?}"))
}

#[inline]
fn add_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}

#[inline]
fn sub_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + p as u128 - b as u128) % p as u128) as u64
}

#[inline]
fn mul_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn pow_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    while exponent != 0 {
        if exponent & 1 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exponent >>= 1;
    }
    result
}

fn inv_mod(value: u64, p: u64) -> Result<u64, String> {
    if value == 0 {
        Err("attempted inverse of zero".into())
    } else {
        Ok(pow_mod(value, p - 2, p))
    }
}

pub fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for small in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == small {
            return true;
        }
        if n.is_multiple_of(small) {
            return false;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'witness: for base in [2u64, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022] {
        let base = base % n;
        if base == 0 {
            continue;
        }
        let mut x = pow_mod(base, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

pub fn add_points(curve: Fixture, left: Point, right: Point) -> Result<Point, String> {
    match (left, right) {
        (Point::Infinity, point) | (point, Point::Infinity) => Ok(point),
        (Point::Affine { x: x1, y: y1 }, Point::Affine { x: x2, y: y2 }) => {
            if x1 >= curve.p || y1 >= curve.p || x2 >= curve.p || y2 >= curve.p {
                return Err("non-canonical point coordinate".into());
            }
            if x1 == x2 && add_mod(y1, y2, curve.p) == 0 {
                return Ok(Point::Infinity);
            }
            let slope = if x1 == x2 && y1 == y2 {
                if y1 == 0 {
                    return Ok(Point::Infinity);
                }
                let numerator = add_mod(
                    mul_mod(3, mul_mod(x1, x1, curve.p), curve.p),
                    curve.a,
                    curve.p,
                );
                mul_mod(
                    numerator,
                    inv_mod(mul_mod(2, y1, curve.p), curve.p)?,
                    curve.p,
                )
            } else {
                mul_mod(
                    sub_mod(y2, y1, curve.p),
                    inv_mod(sub_mod(x2, x1, curve.p), curve.p)?,
                    curve.p,
                )
            };
            let x3 = sub_mod(
                sub_mod(mul_mod(slope, slope, curve.p), x1, curve.p),
                x2,
                curve.p,
            );
            let y3 = sub_mod(
                mul_mod(slope, sub_mod(x1, x3, curve.p), curve.p),
                y1,
                curve.p,
            );
            Ok(Point::Affine { x: x3, y: y3 })
        }
    }
}

pub fn scalar_mul(curve: Fixture, mut scalar: u64, point: Point) -> Result<Point, String> {
    let mut result = Point::Infinity;
    let mut base = point;
    while scalar != 0 {
        if scalar & 1 == 1 {
            result = add_points(curve, result, base)?;
        }
        scalar >>= 1;
        if scalar != 0 {
            base = add_points(curve, base, base)?;
        }
    }
    Ok(result)
}

fn on_curve(curve: Fixture, point: Point) -> bool {
    match point {
        Point::Infinity => true,
        Point::Affine { x, y } if x < curve.p && y < curve.p => {
            let lhs = mul_mod(y, y, curve.p);
            let x3 = mul_mod(mul_mod(x, x, curve.p), x, curve.p);
            let rhs = add_mod(
                add_mod(x3, mul_mod(curve.a, x, curve.p), curve.p),
                curve.b,
                curve.p,
            );
            lhs == rhs
        }
        _ => false,
    }
}

fn generator(curve: Fixture) -> Point {
    Point::Affine {
        x: curve.gx,
        y: curve.gy,
    }
}

fn nonsingular(curve: Fixture) -> bool {
    let a3 = mul_mod(mul_mod(curve.a, curve.a, curve.p), curve.a, curve.p);
    let b2 = mul_mod(curve.b, curve.b, curve.p);
    add_mod(mul_mod(4, a3, curve.p), mul_mod(27, b2, curve.p), curve.p) != 0
}

fn master_bytes() -> [u8; 32] {
    hex::decode(MASTER_HEX)
        .expect("frozen master seed is valid hex")
        .try_into()
        .expect("frozen master seed is 32 bytes")
}

pub fn derive_u64(curve_slug: &str, panel: &str, replicate: u64, item: u64, stream: &str) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(STUDY_ID.as_bytes());
    hasher.update([0]);
    hasher.update(curve_slug.as_bytes());
    hasher.update([0]);
    hasher.update(panel.as_bytes());
    hasher.update([0]);
    hasher.update(replicate.to_le_bytes());
    hasher.update([0]);
    hasher.update(item.to_le_bytes());
    hasher.update([0]);
    hasher.update(stream.as_bytes());
    hasher.update([0]);
    hasher.update(master_bytes());
    let digest = hasher.finalize();
    u64::from_le_bytes(digest[..8].try_into().unwrap())
}

pub fn target_scalar(curve: Fixture, panel: &str, replicate: u64, item: u64) -> u64 {
    1 + derive_u64(curve.slug, panel, replicate, item, "target") % (curve.n - 1)
}

fn expected_target(
    curve: Fixture,
    panel: &str,
    replicate: u64,
    item: u64,
) -> Result<Point, String> {
    scalar_mul(
        curve,
        target_scalar(curve, panel, replicate, item),
        generator(curve),
    )
}

fn point_json(point: Point) -> Value {
    match point {
        Point::Infinity => Value::Null,
        Point::Affine { x, y } => json!({"x": x, "y": y}),
    }
}

pub fn verify_fixtures() -> Result<Value, String> {
    let mut records = Vec::new();
    for curve in FIXTURES {
        if !is_prime_u64(curve.p) {
            return Err(format!("{} has composite p", curve.slug));
        }
        if !is_prime_u64(curve.n) {
            return Err(format!("{} has composite n", curve.slug));
        }
        if !nonsingular(curve) {
            return Err(format!("{} is singular", curve.slug));
        }
        let g = generator(curve);
        if !on_curve(curve, g) {
            return Err(format!("{} generator is off curve", curve.slug));
        }
        if scalar_mul(curve, curve.n, g)? != Point::Infinity {
            return Err(format!(
                "{} generator does not have declared order",
                curve.slug
            ));
        }
        records.push(json!({
            "curve_slug": curve.slug,
            "p_prime": true,
            "n_prime": true,
            "nonsingular": true,
            "generator_on_curve": true,
            "order_check": true,
            "sample_target": point_json(expected_target(curve, "cold-paired", 0, 0)?)
        }));
    }
    Ok(json!({
        "schema": "blocked-bsgs-rho-wall/independent-verifier/v1",
        "study_id": STUDY_ID,
        "status": "ok",
        "fixtures": records
    }))
}

fn read_jsonl(path: &Path) -> Result<Vec<Value>, String> {
    let file = File::open(path).map_err(|e| format!("open {}: {e}", path.display()))?;
    BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let line = line.map_err(|e| format!("read {}:{}: {e}", path.display(), index + 1))?;
            if line.trim().is_empty() {
                return Err(format!("blank line at {}:{}", path.display(), index + 1));
            }
            serde_json::from_str(&line)
                .map_err(|e| format!("parse {}:{}: {e}", path.display(), index + 1))
        })
        .collect()
}

fn sha256_path(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

fn required_u64(value: &Value, field: &str) -> Result<u64, String> {
    value[field]
        .as_u64()
        .ok_or_else(|| format!("missing/integer field {field}"))
}

fn required_str<'a>(value: &'a Value, field: &str) -> Result<&'a str, String> {
    value[field]
        .as_str()
        .ok_or_else(|| format!("missing/string field {field}"))
}

#[derive(Clone, Copy)]
struct PanelSpec {
    name: &'static str,
    replicates: u64,
    items: u64,
    has_rho_seed: bool,
}

const PANELS: [PanelSpec; 5] = [
    PanelSpec {
        name: "warmup",
        replicates: 4,
        items: 1,
        has_rho_seed: true,
    },
    PanelSpec {
        name: "cold-paired",
        replicates: 512,
        items: 1,
        has_rho_seed: true,
    },
    PanelSpec {
        name: "bsgs-aa",
        replicates: 64,
        items: 1,
        has_rho_seed: false,
    },
    PanelSpec {
        name: "rho-aa",
        replicates: 64,
        items: 1,
        has_rho_seed: true,
    },
    PanelSpec {
        name: "amortized-B64",
        replicates: 16,
        items: 64,
        has_rho_seed: true,
    },
];

fn expected_schedule(panel: &str, replicate: u64) -> [&'static str; 2] {
    match panel {
        "warmup" | "cold-paired" | "amortized-B64" => {
            if replicate.is_multiple_of(2) {
                ["bsgs", "rho"]
            } else {
                ["rho", "bsgs"]
            }
        }
        "bsgs-aa" | "rho-aa" => {
            if replicate.is_multiple_of(2) {
                ["A", "A-copy"]
            } else {
                ["A-copy", "A"]
            }
        }
        _ => unreachable!("fixed panel"),
    }
}

fn bootstrap_digest(curve: Fixture, stream: &str, observations: u64, reps: u64) -> String {
    let mut digest = Sha256::new();
    for replicate in 0..reps {
        for draw in 0..observations {
            let seed = derive_u64(curve.slug, "analysis", replicate, draw, stream);
            digest.update((seed % observations).to_le_bytes());
        }
    }
    hex::encode(digest.finalize())
}

pub fn verify_seed_mapping(path: &Path) -> Result<Value, String> {
    let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let artifact: Value =
        serde_json::from_slice(&bytes).map_err(|e| format!("parse {}: {e}", path.display()))?;
    let mut canonical = serde_json::to_vec(&artifact).map_err(|e| e.to_string())?;
    canonical.push(b'\n');
    if bytes != canonical {
        return Err("derived seed artifact is not canonical compact JSON plus newline".into());
    }
    if artifact["schema"].as_str() != Some("blocked-bsgs-rho-wall/derived-seeds/v2")
        || artifact["study_id"].as_str() != Some(STUDY_ID)
        || artifact["seed_manifest_sha256"].as_str() != Some(SEED_MANIFEST_SHA256)
        || artifact["master_seed_hex"].as_str() != Some(MASTER_HEX)
    {
        return Err("derived seed artifact identity mismatch".into());
    }
    let expected_encoding = json!({
        "u64_arrays": "concatenated lowercase fixed-width 16-hex-digit words",
        "u64_word_chars": 16,
        "digest_arrays": "concatenated lowercase fixed-width SHA-256 hex digests",
        "digest_chars": 64,
        "array_order": "curve-major, then panel-major, replicate-major, item-major",
        "bootstrap_order": "curve-major, then bootstrap-stream-major",
        "target_flat_index": "curve_index * target_count_per_curve + panel.target_offset_per_curve + replicate * panel.items_per_replicate + item",
        "rho_flat_index": "curve_index * rho_count_per_curve + panel.rho_offset_per_curve + replicate * panel.items_per_replicate + item",
        "schedule_flat_index": "curve_index * schedule_count_per_curve + panel.schedule_offset_per_curve + replicate",
        "bootstrap_digest_flat_index": "curve_index * bootstrap_stream_count + stream.digest_offset_per_curve"
    });
    if artifact["encoding"] != expected_encoding {
        return Err("derived seed encoding contract mismatch".into());
    }

    let mut target_seeds = String::new();
    let mut target_scalars = String::new();
    let mut rho_seeds = String::new();
    let mut schedule_codes = String::new();
    let mut bootstrap_digests = String::new();
    let mut subgroup_orders = String::new();
    let mut seen = HashSet::new();
    for curve in FIXTURES {
        subgroup_orders.push_str(&format!("{:016x}", curve.n));
        for panel in PANELS {
            for replicate in 0..panel.replicates {
                for item in 0..panel.items {
                    let target = derive_u64(curve.slug, panel.name, replicate, item, "target");
                    if !seen.insert(target) {
                        return Err(format!("duplicate target-derived u64 {target:016x}"));
                    }
                    target_seeds.push_str(&format!("{target:016x}"));
                    target_scalars.push_str(&format!("{:016x}", 1 + target % (curve.n - 1)));
                    if panel.has_rho_seed {
                        let rho = derive_u64(curve.slug, panel.name, replicate, item, "rho-walk");
                        if !seen.insert(rho) {
                            return Err(format!("duplicate rho-derived u64 {rho:016x}"));
                        }
                        rho_seeds.push_str(&format!("{rho:016x}"));
                    }
                }
                let code = match (panel.name.ends_with("-aa"), replicate % 2) {
                    (false, 0) => '0',
                    (false, _) => '1',
                    (true, 0) => '2',
                    (true, _) => '3',
                };
                schedule_codes.push(code);
            }
        }
        for (stream, observations) in [
            ("bootstrap-primary", 512u64),
            ("bootstrap-bsgs-aa", 64),
            ("bootstrap-rho-aa", 64),
            ("bootstrap-amortized", 16),
        ] {
            bootstrap_digests.push_str(&bootstrap_digest(curve, stream, observations, 10_000));
        }
    }

    let expected_data = json!({
        "target_derived_u64_hex": target_seeds,
        "target_scalar_u64_hex": target_scalars,
        "rho_derived_u64_hex": rho_seeds,
        "schedule_codes": schedule_codes,
        "bootstrap_selection_sha256_hex": bootstrap_digests
    });
    if artifact["data"] != expected_data {
        return Err("derived seed data differs from independent expansion".into());
    }
    let mut target_offset = 0u64;
    let mut rho_offset = 0u64;
    let mut schedule_offset = 0u64;
    let mut panel_dimensions = Vec::new();
    for panel in PANELS {
        let count = panel.replicates * panel.items;
        panel_dimensions.push(json!({
            "panel": panel.name,
            "replicates": panel.replicates,
            "items_per_replicate": panel.items,
            "target_offset_per_curve": target_offset,
            "target_count_per_curve": count,
            "rho_offset_per_curve": if panel.has_rho_seed { Some(rho_offset) } else { None },
            "rho_count_per_curve": if panel.has_rho_seed { Some(count) } else { None },
            "schedule_offset_per_curve": schedule_offset,
            "schedule_count_per_curve": panel.replicates
        }));
        target_offset += count;
        if panel.has_rho_seed {
            rho_offset += count;
        }
        schedule_offset += panel.replicates;
    }
    let bootstrap_dimensions = [
        ("bootstrap-primary", 512u64),
        ("bootstrap-bsgs-aa", 64),
        ("bootstrap-rho-aa", 64),
        ("bootstrap-amortized", 16),
    ]
    .into_iter()
    .enumerate()
    .map(|(offset, (stream, observations))| {
        json!({
            "stream": stream,
            "observations": observations,
            "digest_offset_per_curve": offset,
            "draws_per_digest": 10_000 * observations
        })
    })
    .collect::<Vec<_>>();
    let expected_dimensions = json!({
        "curve_count": FIXTURES.len(),
        "curve_order": FIXTURES.iter().map(|curve| curve.slug).collect::<Vec<_>>(),
        "subgroup_orders_u64_hex": subgroup_orders,
        "panels": panel_dimensions,
        "target_count_per_curve": target_offset,
        "rho_count_per_curve": rho_offset,
        "schedule_count_per_curve": schedule_offset,
        "bootstrap_replicates": 10_000,
        "bootstrap_stream_count": 4,
        "bootstrap_streams": bootstrap_dimensions,
        "logical_counts": {
            "target_seeds": target_offset * FIXTURES.len() as u64,
            "target_scalars": target_offset * FIXTURES.len() as u64,
            "rho_seeds": rho_offset * FIXTURES.len() as u64,
            "schedule_codes": schedule_offset * FIXTURES.len() as u64,
            "bootstrap_digests": 4 * FIXTURES.len(),
            "superseded_jsonl_rows": (target_offset + rho_offset + schedule_offset)
                * FIXTURES.len() as u64 + 4 * FIXTURES.len() as u64
        }
    });
    if artifact["dimensions"] != expected_dimensions {
        return Err("derived seed dimensions differ from frozen indexing".into());
    }
    if artifact["schedule_codebook"]
        != json!({
            "0": ["bsgs", "rho"],
            "1": ["rho", "bsgs"],
            "2": ["A", "A-copy"],
            "3": ["A-copy", "A"]
        })
    {
        return Err("derived seed schedule codebook mismatch".into());
    }
    let digest = hex::encode(Sha256::digest(&bytes));
    Ok(json!({
        "schema": "blocked-bsgs-rho-wall/seed-verification/v2",
        "status": "ok",
        "artifact_sha256": digest,
        "artifact_bytes": bytes.len(),
        "target_records": target_offset * FIXTURES.len() as u64,
        "rho_records": rho_offset * FIXTURES.len() as u64,
        "schedule_records": schedule_offset * FIXTURES.len() as u64,
        "bootstrap_records": 4 * FIXTURES.len(),
        "unique_derived_seeds": seen.len()
    }))
}

fn floor_sqrt(n: u64) -> u64 {
    let mut low = 0u64;
    let mut high = 1u64 << 32;
    while low + 1 < high {
        let middle = low + (high - low) / 2;
        if (middle as u128) * (middle as u128) <= n as u128 {
            low = middle;
        } else {
            high = middle;
        }
    }
    low
}

fn ceil_sqrt(n: u64) -> u64 {
    let floor = floor_sqrt(n);
    if floor * floor == n {
        floor
    } else {
        floor + 1
    }
}

fn expected_bsgs_plan(curve: Fixture) -> Value {
    let ceil_div = |a: u64, b: u64| a.div_ceil(b);
    let m = ceil_div(floor_sqrt(curve.n), 2).max(1);
    let stride = 2 * m - 1;
    let giant_count = (curve.n - 1) / stride + 2;
    let mut table_bits = 4u32;
    while (1u64 << table_bits) < 2 * m {
        table_bits += 1;
    }
    json!({
        "neg_map": true,
        "width": curve.n,
        "x0": 0,
        "m": m,
        "stride": stride,
        "giant_count": giant_count,
        "table_bits": table_bits,
        "workers": 1,
        "chains_per_worker": 8,
        "baby_len": ceil_div(m, 8),
        "giant_len": ceil_div(giant_count, 8),
        "kernel": "paired",
        "pair_k": 32
    })
}

fn expected_rho_config(curve: Fixture, seed: u64) -> Value {
    let bits = 64 - curve.n.leading_zeros();
    let expected = (std::f64::consts::PI * curve.n as f64 / 4.0).sqrt();
    let table_bits = match bits {
        0..=15 => 2u32,
        16..=22 => 3,
        _ => 4,
    };
    let walkers = if bits <= 30 { 32u64 } else { 64 };
    let dp_bits = (0.02 * expected / walkers as f64)
        .log2()
        .floor()
        .clamp(0.0, 24.0) as u32;
    json!({
        "threads": 1,
        "walkers": walkers,
        "table_bits": table_bits,
        "dp_bits": dp_bits,
        "negation": true,
        "seed": seed,
        "max_steps": 64u64.saturating_mul(ceil_sqrt(curve.n)),
        "look_ahead": true,
        "cycle_window": 16,
        "coeff_bits": (3 * table_bits).clamp(6, 12),
        "stride_starts": true
    })
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct EventKey {
    curve: String,
    panel: String,
    replicate: u64,
    item: u64,
    arm: String,
    copy: u64,
}

impl EventKey {
    fn parse(row: &Value) -> Result<Self, String> {
        Ok(Self {
            curve: required_str(row, "curve_slug")?.to_owned(),
            panel: required_str(row, "panel")?.to_owned(),
            replicate: required_u64(row, "replicate")?,
            item: required_u64(row, "item")?,
            arm: required_str(row, "arm")?.to_owned(),
            copy: required_u64(row, "copy")?,
        })
    }
}

#[derive(Default)]
struct EventSet {
    manifest: Option<Value>,
    intents: BTreeMap<EventKey, Value>,
    results: BTreeMap<EventKey, Value>,
}

fn check_manifest(row: &Value, command: &str, panels: &[&str]) -> Result<(), String> {
    if row["schema"].as_str() != Some("blocked-bsgs-rho-wall/run-manifest/v1")
        || row["event"].as_str() != Some("run_manifest")
        || row["study_id"].as_str() != Some(STUDY_ID)
        || row["command"].as_str() != Some(command)
        || row["phase_unit"].as_str() != Some("ns")
        || row["rayon_threads"].as_u64() != Some(1)
        || row["rayon_num_threads_env"].as_str() != Some("1")
        || row["debug_assertions"].as_bool() != Some(false)
    {
        return Err(format!("invalid run manifest for {command}"));
    }
    let got_panels = row["panels"]
        .as_array()
        .ok_or_else(|| "run manifest panels is not an array".to_string())?
        .iter()
        .map(|value| value.as_str().unwrap_or(""))
        .collect::<Vec<_>>();
    if got_panels != panels {
        return Err(format!("manifest panel mismatch for {command}"));
    }
    let source = &row["source"];
    if source["repository_base"].as_str() != Some("a2c2c400")
        || source["bsgs_path"].as_str() != Some("src/cryptanalysis/bsgs_fast.rs")
        || source["bsgs_sha256"].as_str() != Some(BSGS_SHA256)
        || source["rho_path"].as_str() != Some("src/cryptanalysis/prime_fast.rs")
        || source["rho_sha256"].as_str() != Some(RHO_SHA256)
        || source["protocol_json_path"].as_str()
            != Some("research/blocked_bsgs_rho_wall_20261126/protocol.json")
        || source["protocol_md_path"].as_str()
            != Some("research/blocked_bsgs_rho_wall_20261126/PROTOCOL.md")
        || source["protocol_md_sha256"].as_str() != Some(PROTOCOL_MD_SHA256)
        || source["protocol_addendum_1_path"].as_str()
            != Some("research/blocked_bsgs_rho_wall_20261126/PROTOCOL-ADDENDUM-1.md")
        || source["protocol_addendum_1_sha256"].as_str() != Some(PROTOCOL_ADDENDUM_1_SHA256)
        || source["protocol_addendum_2_path"].as_str()
            != Some("research/blocked_bsgs_rho_wall_20261126/PROTOCOL-ADDENDUM-2.md")
        || source["protocol_addendum_2_sha256"].as_str() != Some(PROTOCOL_ADDENDUM_2_SHA256)
        || source["seed_manifest_path"].as_str()
            != Some("research/blocked_bsgs_rho_wall_20261126/seed-manifest.json")
        || source["seed_manifest_sha256"].as_str() != Some(SEED_MANIFEST_SHA256)
        || source["cargo_lock_path"].as_str()
            != Some("research/blocked_bsgs_rho_wall_20261126/native/Cargo.lock")
        || source["cargo_lock_sha256"].as_str() != Some(NATIVE_CARGO_LOCK_SHA256)
        || source["derived_seed_mapping_path"].as_str()
            != Some("research/blocked_bsgs_rho_wall_20261126/derived-seeds.json")
    {
        return Err(format!("source envelope mismatch for {command}"));
    }
    for field in ["protocol_json_sha256", "derived_seed_mapping_sha256"] {
        if source[field]
            .as_str()
            .filter(|digest| digest.len() == 64 && digest.chars().all(|c| c.is_ascii_hexdigit()))
            .is_none()
        {
            return Err(format!("missing source digest {field} for {command}"));
        }
    }
    if source["derived_seed_mapping_bytes"]
        .as_u64()
        .filter(|bytes| *bytes > 0 && *bytes < (1 << 20))
        .is_none()
    {
        return Err(format!("invalid seed artifact size for {command}"));
    }
    let slugs = row["curve_slugs"]
        .as_array()
        .ok_or_else(|| "manifest curve_slugs is not an array".to_string())?;
    let want = FIXTURES.iter().map(|curve| curve.slug).collect::<Vec<_>>();
    let got = slugs
        .iter()
        .map(|value| value.as_str().unwrap_or(""))
        .collect::<Vec<_>>();
    if got != want {
        return Err(format!("curve envelope mismatch for {command}"));
    }
    if row["binary_sha256"]
        .as_str()
        .filter(|digest| digest.len() == 64 && digest.chars().all(|c| c.is_ascii_hexdigit()))
        .is_none()
    {
        return Err(format!("missing binary digest for {command}"));
    }
    for field in ["build_rustc", "build_cargo"] {
        if row["toolchain"][field]
            .as_str()
            .filter(|value| !value.is_empty() && value.contains("success=true"))
            .is_none()
        {
            return Err(format!("invalid {field} build record for {command}"));
        }
    }
    for (field, tool) in [
        ("runtime_rustc_observation", "rustc"),
        ("runtime_cargo_observation", "cargo"),
    ] {
        let record = &row["toolchain"][field];
        if record["program"].as_str() != Some(tool)
            || record["success"].as_bool() != Some(true)
            || record["stdout"]
                .as_str()
                .filter(|value| !value.is_empty())
                .is_none()
        {
            return Err(format!("invalid {tool} toolchain record for {command}"));
        }
    }
    let host = &row["host"];
    if host["captured_before_warmups"].as_bool() != Some(true)
        || !host["cpu"].is_object()
        || !host["affinity"].is_object()
        || !host["cgroup"].is_object()
        || !host["psi"].is_object()
        || !host["process_counters"].is_object()
        || host["contention"]["harness_assessment"].as_str()
            != Some("not_inferred; use admitted runner evidence")
        || host["contention"]["observation_only"].as_bool() != Some(true)
    {
        return Err(format!("incomplete host envelope for {command}"));
    }
    Ok(())
}

fn parse_event_file(path: &Path, command: &str, panels: &[&str]) -> Result<EventSet, String> {
    let rows = read_jsonl(path)?;
    let mut manifest_count = 0usize;
    let mut events = EventSet::default();
    for (index, row) in rows.into_iter().enumerate() {
        match row["schema"].as_str() {
            Some("blocked-bsgs-rho-wall/run-manifest/v1") => {
                if index != 0 {
                    return Err(format!(
                        "{}:{}: run manifest must be the first row",
                        path.display(),
                        index + 1
                    ));
                }
                manifest_count += 1;
                check_manifest(&row, command, panels)
                    .map_err(|e| format!("{}:{}: {e}", path.display(), index + 1))?;
                events.manifest = Some(row);
            }
            Some("blocked-bsgs-rho-wall/event/v1") => {
                let key = EventKey::parse(&row)
                    .map_err(|e| format!("{}:{}: {e}", path.display(), index + 1))?;
                fixture(&key.curve)?;
                if !panels.contains(&key.panel.as_str()) {
                    return Err(format!(
                        "{}:{}: panel {:?} does not belong to file",
                        path.display(),
                        index + 1,
                        key.panel
                    ));
                }
                match row["event"].as_str() {
                    Some("intent") => {
                        if events.intents.insert(key, row).is_some() {
                            return Err(format!(
                                "{}:{}: duplicate intent",
                                path.display(),
                                index + 1
                            ));
                        }
                    }
                    Some("result") => {
                        if events.results.insert(key, row).is_some() {
                            return Err(format!(
                                "{}:{}: duplicate result",
                                path.display(),
                                index + 1
                            ));
                        }
                    }
                    _ => return Err(format!("{}:{}: unknown event", path.display(), index + 1)),
                }
            }
            other => {
                return Err(format!(
                    "{}:{}: unexpected schema {other:?}",
                    path.display(),
                    index + 1
                ))
            }
        }
    }
    if manifest_count != 1 {
        return Err(format!(
            "{} has {manifest_count} manifests, expected one",
            path.display()
        ));
    }
    Ok(events)
}

fn key(curve: Fixture, panel: &str, replicate: u64, item: u64, arm: &str, copy: u64) -> EventKey {
    EventKey {
        curve: curve.slug.to_owned(),
        panel: panel.to_owned(),
        replicate,
        item,
        arm: arm.to_owned(),
        copy,
    }
}

fn expected_cold_keys() -> BTreeSet<EventKey> {
    let mut keys = BTreeSet::new();
    for curve in FIXTURES {
        for replicate in 0..512 {
            for arm in ["bsgs", "rho"] {
                keys.insert(key(curve, "cold-paired", replicate, 0, arm, 0));
            }
        }
    }
    keys
}

fn expected_aa_keys() -> BTreeSet<EventKey> {
    let mut keys = BTreeSet::new();
    for curve in FIXTURES {
        for replicate in 0..64 {
            for (panel, arm) in [("bsgs-aa", "bsgs"), ("rho-aa", "rho")] {
                for copy in 0..2 {
                    keys.insert(key(curve, panel, replicate, 0, arm, copy));
                }
            }
        }
    }
    keys
}

fn expected_amortized_keys() -> (BTreeSet<EventKey>, BTreeSet<EventKey>) {
    let mut intents = BTreeSet::new();
    let mut results = BTreeSet::new();
    for curve in FIXTURES {
        for replicate in 0..16 {
            for arm in ["bsgs", "rho"] {
                let batch = key(curve, "amortized-B64", replicate, 64, arm, 0);
                intents.insert(batch.clone());
                results.insert(batch);
                for item in 0..64 {
                    results.insert(key(curve, "amortized-B64", replicate, item, arm, 0));
                }
            }
        }
    }
    (intents, results)
}

fn affine_coords(point: Point) -> Result<(u64, u64), String> {
    match point {
        Point::Affine { x, y } => Ok((x, y)),
        Point::Infinity => Err("registered nonzero target became infinity".into()),
    }
}

fn exact_point(
    row: &Value,
    curve: Fixture,
    panel: &str,
    replicate: u64,
    item: u64,
) -> Result<(), String> {
    let (want_x, want_y) = affine_coords(expected_target(curve, panel, replicate, item)?)?;
    if row["qx"].as_u64() != Some(want_x) || row["qy"].as_u64() != Some(want_y) {
        return Err(format!(
            "target coordinate mismatch for {}/{panel}/{replicate}/{item}",
            curve.slug
        ));
    }
    Ok(())
}

fn validate_intent(event_key: &EventKey, row: &Value) -> Result<(), String> {
    let curve = fixture(&event_key.curve)?;
    let expected_seed = derive_u64(
        curve.slug,
        &event_key.panel,
        event_key.replicate,
        0,
        "target",
    );
    if row["target_seed"].as_u64() != Some(expected_seed) {
        return Err(format!("target seed mismatch for {event_key:?}"));
    }
    if event_key.panel == "amortized-B64" {
        if row["scope"].as_str() != Some("batch")
            || event_key.item != 64
            || !row["qx"].is_null()
            || !row["qy"].is_null()
        {
            return Err(format!("invalid amortized batch intent {event_key:?}"));
        }
        let targets = row["targets"]
            .as_array()
            .ok_or_else(|| format!("missing target array for {event_key:?}"))?;
        if targets.len() != 64 {
            return Err(format!("batch target count != 64 for {event_key:?}"));
        }
        let mut points_digest = Sha256::new();
        for (item, target) in targets.iter().enumerate() {
            let item = item as u64;
            if target["item"].as_u64() != Some(item)
                || target["target_seed"].as_u64()
                    != Some(derive_u64(
                        curve.slug,
                        "amortized-B64",
                        event_key.replicate,
                        item,
                        "target",
                    ))
            {
                return Err(format!(
                    "batch target seed mismatch for {event_key:?}/{item}"
                ));
            }
            exact_point(target, curve, "amortized-B64", event_key.replicate, item)?;
            let qx = required_u64(target, "qx")?;
            let qy = required_u64(target, "qy")?;
            points_digest.update(qx.to_le_bytes());
            points_digest.update(qy.to_le_bytes());
            let expected_rho_seed = derive_u64(
                curve.slug,
                "amortized-B64",
                event_key.replicate,
                item,
                "rho-walk",
            );
            if event_key.arm == "rho" {
                if target["rho_seed"].as_u64() != Some(expected_rho_seed) {
                    return Err(format!("batch rho seed mismatch for {event_key:?}/{item}"));
                }
            } else if !target["rho_seed"].is_null() {
                return Err(format!(
                    "BSGS batch carries rho seed for {event_key:?}/{item}"
                ));
            }
        }
        let digest = hex::encode(points_digest.finalize());
        let identity = &row["target_seed_identity"];
        if identity["mapping"].as_str() != Some("derived-seeds.json")
            || identity["panel"].as_str() != Some("amortized-B64")
            || identity["replicate"].as_u64() != Some(event_key.replicate)
            || identity["items"].as_str() != Some("0..63")
            || identity["target_points_encoding"].as_str() != Some("ordered qx-u64-le || qy-u64-le")
            || identity["target_points_sha256"].as_str() != Some(&digest)
        {
            return Err(format!("batch target digest mismatch for {event_key:?}"));
        }
    } else {
        if row["scope"].as_str() != Some("arm") || event_key.item != 0 {
            return Err(format!("invalid arm intent {event_key:?}"));
        }
        exact_point(row, curve, &event_key.panel, event_key.replicate, 0)?;
    }

    let expected_walk = derive_u64(
        curve.slug,
        &event_key.panel,
        event_key.replicate,
        0,
        "rho-walk",
    );
    if event_key.arm == "rho" {
        if row["rho_seed"].as_u64() != Some(expected_walk) {
            return Err(format!("rho seed mismatch for {event_key:?}"));
        }
    } else if event_key.arm == "bsgs" {
        if !row["rho_seed"].is_null() {
            return Err(format!("BSGS intent carries rho seed for {event_key:?}"));
        }
    } else {
        return Err(format!("unknown arm for {event_key:?}"));
    }

    if event_key.panel.ends_with("-aa") {
        let label = if event_key.copy == 0 { "A" } else { "A-copy" };
        if row["invocation_label"].as_str() != Some(label) {
            return Err(format!("A/A label mismatch for {event_key:?}"));
        }
        let order = expected_schedule(&event_key.panel, event_key.replicate);
        let position = if order[0] == label { 0 } else { 1 };
        if row["schedule_position"].as_u64() != Some(position) {
            return Err(format!("A/A schedule mismatch for {event_key:?}"));
        }
    } else {
        if event_key.copy != 0 || row["invocation_label"].as_str() != Some(&event_key.arm) {
            return Err(format!("invocation label mismatch for {event_key:?}"));
        }
        let order = expected_schedule(&event_key.panel, event_key.replicate);
        let position = if order[0] == event_key.arm { 0 } else { 1 };
        if row["schedule_position"].as_u64() != Some(position) {
            return Err(format!("schedule mismatch for {event_key:?}"));
        }
    }
    Ok(())
}

fn sum_fields(value: &Value, fields: &[&str]) -> Result<u64, String> {
    fields.iter().try_fold(0u64, |sum, field| {
        let next = required_u64(value, field)?;
        sum.checked_add(next)
            .ok_or_else(|| format!("phase sum overflow at {field}"))
    })
}

fn validate_phase_sum(
    measurement: &Value,
    event_key: &EventKey,
    scope: &str,
) -> Result<(), String> {
    let total = required_u64(measurement, "total_ns")?;
    if measurement["phase_sum_ns"].as_u64() != Some(total) {
        return Err(format!("declared phase sum mismatch for {event_key:?}"));
    }
    let fields: &[&str] = match (scope, event_key.arm.as_str()) {
        ("arm", "bsgs") => &[
            "build_ns",
            "search_ns",
            "evaluator_verify_ns",
            "drop_ns",
            "unattributed_ns",
        ],
        ("arm", "rho") | ("item", "rho") => &[
            "solver_ns",
            "evaluator_verify_ns",
            "drop_ns",
            "unattributed_ns",
        ],
        ("item", "bsgs") => &["search_ns", "evaluator_verify_ns", "unattributed_ns"],
        ("batch", "bsgs") => &[
            "build_ns",
            "search_ns_sum",
            "evaluator_verify_ns_sum",
            "drop_ns",
            "unattributed_ns",
        ],
        ("batch", "rho") => &["item_total_ns_sum", "batch_unattributed_ns"],
        _ => return Err(format!("invalid scope/arm for {event_key:?}")),
    };
    if sum_fields(measurement, fields)? != total {
        return Err(format!("recomputed phase sum mismatch for {event_key:?}"));
    }
    Ok(())
}

fn validate_status(row: &Value, measurement: &Value, event_key: &EventKey) -> Result<(), String> {
    let status = required_str(measurement, "status")?;
    if !matches!(status, "ok" | "solver_none_or_cap" | "incorrect") {
        return Err(format!("unknown status {status:?} for {event_key:?}"));
    }
    let censored = measurement["censored"]
        .as_bool()
        .ok_or_else(|| format!("missing censored flag for {event_key:?}"))?;
    let correct = measurement["correct"]
        .as_bool()
        .ok_or_else(|| format!("missing correct flag for {event_key:?}"))?;
    if row["status"].as_str() != Some(status) || row["censored"].as_bool() != Some(censored) {
        return Err(format!("top-level status mismatch for {event_key:?}"));
    }
    if row["error"] != measurement["error"] {
        return Err(format!("top-level error mismatch for {event_key:?}"));
    }
    match status {
        "ok" if !censored && correct && measurement["error"].is_null() => {}
        "solver_none_or_cap"
            if censored
                && !correct
                && measurement["error"].as_str() == Some("solver_none_or_cap") => {}
        "incorrect"
            if !censored
                && !correct
                && measurement["error"].as_str() == Some("evaluator_mismatch") => {}
        _ => return Err(format!("inconsistent status fields for {event_key:?}")),
    }
    Ok(())
}

fn validate_bsgs_measurement(
    curve: Fixture,
    event_key: &EventKey,
    scope: &str,
    measurement: &Value,
) -> Result<(), String> {
    if scope != "item" {
        if measurement["plan"] != expected_bsgs_plan(curve) {
            return Err(format!("BSGS plan mismatch for {event_key:?}"));
        }
        let table_bits = required_u64(&measurement["plan"], "table_bits")?;
        if measurement["table_bytes"].as_u64() != Some(8u64 << table_bits) {
            return Err(format!("BSGS table bytes mismatch for {event_key:?}"));
        }
        for field in [
            "table_entries",
            "baby_steps",
            "giant_steps",
            "candidates",
            "false_candidates",
        ] {
            required_u64(measurement, field)
                .map_err(|e| format!("BSGS stats for {event_key:?}: {e}"))?;
        }
    }
    Ok(())
}

fn validate_rho_measurement(
    curve: Fixture,
    event_key: &EventKey,
    scope: &str,
    measurement: &Value,
) -> Result<(), String> {
    if scope == "batch" {
        for field in [
            "censored_count",
            "incorrect_count",
            "solver_ns_sum",
            "evaluator_verify_ns_sum",
            "drop_ns_sum",
            "item_unattributed_ns_sum",
            "item_total_ns_sum",
            "total_steps",
            "distinguished_points",
        ] {
            required_u64(measurement, field)
                .map_err(|e| format!("rho batch for {event_key:?}: {e}"))?;
        }
        return Ok(());
    }
    let seed = derive_u64(
        curve.slug,
        &event_key.panel,
        event_key.replicate,
        event_key.item,
        "rho-walk",
    );
    if measurement["config"] != expected_rho_config(curve, seed) {
        return Err(format!("rho config mismatch for {event_key:?}"));
    }
    if let Some(result) = measurement["result"].as_object() {
        let result = Value::Object(result.clone());
        for field in [
            "log",
            "total_steps",
            "distinguished_points",
            "inner_wall_ns",
            "threads",
            "walkers_per_thread",
            "dp_bits",
        ] {
            required_u64(&result, field)
                .map_err(|e| format!("rho result for {event_key:?}: {e}"))?;
        }
        if result["threads"].as_u64() != Some(1)
            || result["walkers_per_thread"] != measurement["config"]["walkers"]
            || result["dp_bits"] != measurement["config"]["dp_bits"]
            || result["steps_per_sec"]
                .as_f64()
                .filter(|value| value.is_finite() && *value >= 0.0)
                .is_none()
        {
            return Err(format!("rho result/config mismatch for {event_key:?}"));
        }
        let ops = &result["ops"];
        for field in [
            "table_adds",
            "table_doubles",
            "initial_start_adds",
            "initial_start_doubles",
            "restart_adds",
            "restart_doubles",
            "escape_doubles",
            "collision_checks",
            "look_ahead_adds",
            "cycles_2",
            "cycles_4",
            "cycles_other",
            "abandoned",
            "capped",
            "restarts",
            "dp_trail_steps",
            "lost_steps",
            "in_flight_steps",
        ] {
            required_u64(ops, field).map_err(|e| format!("rho ops for {event_key:?}: {e}"))?;
        }
    } else if !measurement["result"].is_null() {
        return Err(format!("rho result has invalid type for {event_key:?}"));
    }
    Ok(())
}

fn returned_log(measurement: &Value, arm: &str) -> Option<u64> {
    if arm == "bsgs" {
        measurement["log"].as_u64()
    } else {
        measurement["result"]["log"].as_u64()
    }
}

fn validate_result(event_key: &EventKey, row: &Value) -> Result<(), String> {
    let curve = fixture(&event_key.curve)?;
    let scope = required_str(row, "scope")?;
    let payload = &row["payload"];
    let measurement = &payload["measurement"];
    validate_status(row, measurement, event_key)?;
    validate_phase_sum(measurement, event_key, scope)?;
    match event_key.arm.as_str() {
        "bsgs" => validate_bsgs_measurement(curve, event_key, scope, measurement)?,
        "rho" => validate_rho_measurement(curve, event_key, scope, measurement)?,
        _ => return Err(format!("unknown arm for {event_key:?}")),
    }

    if scope == "batch" {
        if event_key.panel != "amortized-B64"
            || event_key.item != 64
            || event_key.copy != 0
            || payload["batch_size"].as_u64() != Some(64)
            || !row["qx"].is_null()
            || !row["qy"].is_null()
        {
            return Err(format!("invalid batch result for {event_key:?}"));
        }
        let logs = measurement["logs"]
            .as_array()
            .ok_or_else(|| format!("missing batch logs for {event_key:?}"))?;
        if logs.len() != 64 {
            return Err(format!("batch log count != 64 for {event_key:?}"));
        }
        return Ok(());
    }

    if !matches!(scope, "arm" | "item") {
        return Err(format!("invalid result scope for {event_key:?}"));
    }
    if scope == "arm" && event_key.item != 0 {
        return Err(format!("arm result has nonzero item for {event_key:?}"));
    }
    if scope == "item"
        && (event_key.panel != "amortized-B64" || event_key.item >= 64 || event_key.copy != 0)
    {
        return Err(format!("invalid item result for {event_key:?}"));
    }
    let expected_scalar =
        target_scalar(curve, &event_key.panel, event_key.replicate, event_key.item);
    if payload["target_scalar"].as_u64() != Some(expected_scalar) {
        return Err(format!("target scalar mismatch for {event_key:?}"));
    }
    exact_point(
        payload,
        curve,
        &event_key.panel,
        event_key.replicate,
        event_key.item,
    )?;
    exact_point(
        row,
        curve,
        &event_key.panel,
        event_key.replicate,
        event_key.item,
    )?;
    if row["qx"] != payload["qx"] || row["qy"] != payload["qy"] {
        return Err(format!("top/payload target mismatch for {event_key:?}"));
    }
    if scope == "item" {
        let target_seed = derive_u64(
            curve.slug,
            &event_key.panel,
            event_key.replicate,
            event_key.item,
            "target",
        );
        if payload["target_seed"].as_u64() != Some(target_seed) {
            return Err(format!("item target seed mismatch for {event_key:?}"));
        }
        if event_key.arm == "rho" {
            let seed = derive_u64(
                curve.slug,
                &event_key.panel,
                event_key.replicate,
                event_key.item,
                "rho-walk",
            );
            if payload["rho_seed"].as_u64() != Some(seed) {
                return Err(format!("item rho seed mismatch for {event_key:?}"));
            }
        } else if !payload["rho_seed"].is_null() {
            return Err(format!("BSGS item carries rho seed for {event_key:?}"));
        }
    }
    let log = returned_log(measurement, &event_key.arm);
    if let Some(log) = log {
        if log >= curve.n {
            return Err(format!("returned scalar out of range for {event_key:?}"));
        }
        let expected = Point::Affine {
            x: required_u64(payload, "qx")?,
            y: required_u64(payload, "qy")?,
        };
        let replay_matches = scalar_mul(curve, log, generator(curve))? == expected;
        match measurement["status"].as_str() {
            Some("ok") if !replay_matches => {
                return Err(format!(
                    "returned scalar fails affine replay for {event_key:?}"
                ))
            }
            Some("incorrect") if replay_matches => {
                return Err(format!(
                    "incorrect status contradicts affine replay for {event_key:?}"
                ))
            }
            Some("solver_none_or_cap") => {
                return Err(format!("solver-none row carries a log for {event_key:?}"))
            }
            _ => {}
        }
    } else if measurement["status"].as_str() != Some("solver_none_or_cap") {
        return Err(format!(
            "non-cap result lacks returned log for {event_key:?}"
        ));
    }
    Ok(())
}

fn answers_digest(logs: &[Option<u64>]) -> String {
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for log in logs {
        digest = (digest ^ log.unwrap_or(u64::MAX)).wrapping_mul(0x100_0000_01b3);
    }
    format!("{digest:016x}")
}

fn validate_batch(events: &EventSet, batch_key: &EventKey) -> Result<(), String> {
    let intent = events
        .intents
        .get(batch_key)
        .ok_or_else(|| format!("batch result lacks intent for {batch_key:?}"))?;
    let batch_row = events
        .results
        .get(batch_key)
        .ok_or_else(|| format!("missing batch result for {batch_key:?}"))?;
    if intent["target_seed_identity"]["target_points_sha256"]
        != batch_row["payload"]["target_points_sha256"]
    {
        return Err(format!(
            "batch target digest disagreement for {batch_key:?}"
        ));
    }
    let measurement = &batch_row["payload"]["measurement"];
    let mut logs = Vec::with_capacity(64);
    let mut all_correct = true;
    let mut any_censored = false;
    let mut incorrect_count = 0u64;
    let mut censored_count = 0u64;
    let mut search_sum = 0u64;
    let mut verify_sum = 0u64;
    let mut solver_sum = 0u64;
    let mut drop_sum = 0u64;
    let mut unattributed_sum = 0u64;
    let mut item_total_sum = 0u64;
    let mut total_steps = 0u64;
    let mut distinguished_points = 0u64;
    for item in 0..64 {
        let item_key = EventKey {
            item,
            ..batch_key.clone()
        };
        let row = events
            .results
            .get(&item_key)
            .ok_or_else(|| format!("batch aggregate lacks item result for {item_key:?}"))?;
        let item_measurement = &row["payload"]["measurement"];
        let log = returned_log(item_measurement, &batch_key.arm);
        logs.push(log);
        let correct = item_measurement["correct"].as_bool() == Some(true);
        let censored = item_measurement["censored"].as_bool() == Some(true);
        all_correct &= correct;
        any_censored |= censored;
        censored_count +=
            u64::from(item_measurement["status"].as_str() == Some("solver_none_or_cap"));
        incorrect_count += u64::from(item_measurement["status"].as_str() == Some("incorrect"));
        verify_sum = verify_sum
            .checked_add(required_u64(item_measurement, "evaluator_verify_ns")?)
            .ok_or_else(|| "batch verification sum overflow".to_string())?;
        unattributed_sum = unattributed_sum
            .checked_add(required_u64(item_measurement, "unattributed_ns")?)
            .ok_or_else(|| "batch unattributed sum overflow".to_string())?;
        item_total_sum = item_total_sum
            .checked_add(required_u64(item_measurement, "total_ns")?)
            .ok_or_else(|| "batch item total sum overflow".to_string())?;
        if batch_key.arm == "bsgs" {
            search_sum = search_sum
                .checked_add(required_u64(item_measurement, "search_ns")?)
                .ok_or_else(|| "batch search sum overflow".to_string())?;
        } else {
            solver_sum = solver_sum
                .checked_add(required_u64(item_measurement, "solver_ns")?)
                .ok_or_else(|| "batch solver sum overflow".to_string())?;
            drop_sum = drop_sum
                .checked_add(required_u64(item_measurement, "drop_ns")?)
                .ok_or_else(|| "batch drop sum overflow".to_string())?;
            total_steps = total_steps
                .checked_add(
                    item_measurement["result"]["total_steps"]
                        .as_u64()
                        .unwrap_or(0),
                )
                .ok_or_else(|| "batch steps sum overflow".to_string())?;
            distinguished_points = distinguished_points
                .checked_add(
                    item_measurement["result"]["distinguished_points"]
                        .as_u64()
                        .unwrap_or(0),
                )
                .ok_or_else(|| "batch DP sum overflow".to_string())?;
        }
    }
    let recorded_logs = measurement["logs"]
        .as_array()
        .ok_or_else(|| format!("batch lacks log vector for {batch_key:?}"))?;
    let got_logs = recorded_logs
        .iter()
        .map(Value::as_u64)
        .collect::<Vec<Option<u64>>>();
    if got_logs != logs || measurement["answers_digest"].as_str() != Some(&answers_digest(&logs)) {
        return Err(format!("batch/item log disagreement for {batch_key:?}"));
    }
    if measurement["correct"].as_bool() != Some(all_correct)
        || measurement["censored"].as_bool() != Some(any_censored)
    {
        return Err(format!("batch/item status disagreement for {batch_key:?}"));
    }
    if batch_key.arm == "bsgs" {
        if measurement["search_ns_sum"].as_u64() != Some(search_sum)
            || measurement["evaluator_verify_ns_sum"].as_u64() != Some(verify_sum)
        {
            return Err(format!(
                "BSGS batch phase aggregate mismatch for {batch_key:?}"
            ));
        }
    } else if measurement["solver_ns_sum"].as_u64() != Some(solver_sum)
        || measurement["evaluator_verify_ns_sum"].as_u64() != Some(verify_sum)
        || measurement["drop_ns_sum"].as_u64() != Some(drop_sum)
        || measurement["item_unattributed_ns_sum"].as_u64() != Some(unattributed_sum)
        || measurement["item_total_ns_sum"].as_u64() != Some(item_total_sum)
        || measurement["total_steps"].as_u64() != Some(total_steps)
        || measurement["distinguished_points"].as_u64() != Some(distinguished_points)
        || measurement["censored_count"].as_u64() != Some(censored_count)
        || measurement["incorrect_count"].as_u64() != Some(incorrect_count)
    {
        return Err(format!(
            "rho batch phase aggregate mismatch for {batch_key:?}"
        ));
    }
    Ok(())
}

fn validate_event_set(
    events: &EventSet,
    expected_intents: &BTreeSet<EventKey>,
    expected_results: &BTreeSet<EventKey>,
    amortized: bool,
) -> Result<Value, String> {
    let actual_intents = events.intents.keys().cloned().collect::<BTreeSet<_>>();
    let actual_results = events.results.keys().cloned().collect::<BTreeSet<_>>();
    if let Some(unexpected) = actual_intents.difference(expected_intents).next() {
        return Err(format!("unexpected intent key {unexpected:?}"));
    }
    if let Some(unexpected) = actual_results.difference(expected_results).next() {
        return Err(format!("unexpected result key {unexpected:?}"));
    }
    for (event_key, row) in &events.intents {
        validate_intent(event_key, row)?;
    }
    for (event_key, row) in &events.results {
        validate_result(event_key, row)?;
        let parent = if amortized && event_key.item < 64 {
            EventKey {
                item: 64,
                ..event_key.clone()
            }
        } else {
            event_key.clone()
        };
        if !events.intents.contains_key(&parent) {
            return Err(format!("result lacks its flushed intent {event_key:?}"));
        }
    }
    if amortized {
        for batch_key in events
            .results
            .keys()
            .filter(|event_key| event_key.item == 64)
        {
            validate_batch(events, batch_key)?;
        }
    }
    let missing_intents = expected_intents.difference(&actual_intents).count();
    let missing_results = expected_results.difference(&actual_results).count();
    let unmatched_intents = actual_intents
        .iter()
        .filter(|event_key| !actual_results.contains(*event_key))
        .count();
    let censored_results = events
        .results
        .values()
        .filter(|row| row["censored"].as_bool() == Some(true))
        .count();
    let incorrect_results = events
        .results
        .values()
        .filter(|row| row["status"].as_str() == Some("incorrect"))
        .count();
    Ok(json!({
        "expected_intents": expected_intents.len(),
        "observed_intents": actual_intents.len(),
        "expected_results": expected_results.len(),
        "observed_results": actual_results.len(),
        "missing_intents": missing_intents,
        "missing_results": missing_results,
        "unmatched_intents": unmatched_intents,
        "censored_results": censored_results,
        "incorrect_results": incorrect_results,
        "complete": missing_intents == 0 && missing_results == 0
            && unmatched_intents == 0 && censored_results == 0 && incorrect_results == 0
    }))
}

fn ok_total(events: &EventSet, event_key: &EventKey) -> Option<u64> {
    let row = events.results.get(event_key)?;
    if row["status"].as_str() != Some("ok") || row["censored"].as_bool() != Some(false) {
        return None;
    }
    row["payload"]["measurement"]["total_ns"].as_u64()
}

fn ratio_of_sums(pairs: &[(u64, u64)], indices: impl Iterator<Item = usize>) -> f64 {
    let mut numerator = 0f64;
    let mut denominator = 0f64;
    for index in indices {
        denominator += pairs[index].0 as f64;
        numerator += pairs[index].1 as f64;
    }
    numerator / denominator
}

fn paired_summary(curve: Fixture, stream: &str, pairs: &[(u64, u64)]) -> Value {
    let ratio = ratio_of_sums(pairs, 0..pairs.len());
    let first_mean = pairs.iter().map(|pair| pair.0 as f64).sum::<f64>() / pairs.len() as f64;
    let second_mean = pairs.iter().map(|pair| pair.1 as f64).sum::<f64>() / pairs.len() as f64;
    let mean_difference = second_mean - first_mean;
    let mut samples = Vec::with_capacity(10_000);
    for bootstrap_replicate in 0..10_000u64 {
        let indices = (0..pairs.len()).map(|draw| {
            derive_u64(
                curve.slug,
                "analysis",
                bootstrap_replicate,
                draw as u64,
                stream,
            ) as usize
                % pairs.len()
        });
        samples.push(ratio_of_sums(pairs, indices));
    }
    samples.sort_by(f64::total_cmp);
    json!({
        "observations": pairs.len(),
        "ratio_of_sums": ratio,
        "mean_first_ns": first_mean,
        "mean_second_ns": second_mean,
        "mean_second_minus_first_ns": mean_difference,
        "ci95_type1": [samples[249], samples[9749]],
        "bootstrap_replicates": 10_000,
        "quantile_indices_zero_based": [249, 9749]
    })
}

fn collect_cold(events: &EventSet, curve: Fixture) -> Option<Vec<(u64, u64)>> {
    let mut pairs = Vec::with_capacity(512);
    for replicate in 0..512 {
        let bsgs = ok_total(events, &key(curve, "cold-paired", replicate, 0, "bsgs", 0))?;
        let rho = ok_total(events, &key(curve, "cold-paired", replicate, 0, "rho", 0))?;
        pairs.push((bsgs, rho));
    }
    Some(pairs)
}

fn collect_aa(
    events: &EventSet,
    curve: Fixture,
    panel: &str,
    arm: &str,
) -> Option<Vec<(u64, u64)>> {
    let mut pairs = Vec::with_capacity(64);
    for replicate in 0..64 {
        let copy0 = ok_total(events, &key(curve, panel, replicate, 0, arm, 0))?;
        let copy1 = ok_total(events, &key(curve, panel, replicate, 0, arm, 1))?;
        pairs.push((copy0, copy1));
    }
    Some(pairs)
}

fn collect_amortized(events: &EventSet, curve: Fixture) -> Option<Vec<(u64, u64)>> {
    let mut pairs = Vec::with_capacity(16);
    for replicate in 0..16 {
        let bsgs = ok_total(
            events,
            &key(curve, "amortized-B64", replicate, 64, "bsgs", 0),
        )?;
        let rho = ok_total(
            events,
            &key(curve, "amortized-B64", replicate, 64, "rho", 0),
        )?;
        pairs.push((bsgs, rho));
    }
    Some(pairs)
}

fn interval_contains_one(summary: &Value) -> bool {
    let interval = summary["ci95_type1"].as_array().expect("summary interval");
    interval[0].as_f64().unwrap() <= 1.0 && interval[1].as_f64().unwrap() >= 1.0
}

fn analyze(cold: &EventSet, aa: &EventSet, amortized: &EventSet) -> Value {
    let mut curves = Vec::new();
    for curve in FIXTURES {
        let cold_pairs = collect_cold(cold, curve);
        let bsgs_aa_pairs = collect_aa(aa, curve, "bsgs-aa", "bsgs");
        let rho_aa_pairs = collect_aa(aa, curve, "rho-aa", "rho");
        let amortized_pairs = collect_amortized(amortized, curve);
        let cold_summary = cold_pairs
            .as_ref()
            .map(|pairs| paired_summary(curve, "bootstrap-primary", pairs));
        let bsgs_aa = bsgs_aa_pairs
            .as_ref()
            .map(|pairs| paired_summary(curve, "bootstrap-bsgs-aa", pairs));
        let rho_aa = rho_aa_pairs
            .as_ref()
            .map(|pairs| paired_summary(curve, "bootstrap-rho-aa", pairs));
        let amortized_summary = amortized_pairs
            .as_ref()
            .map(|pairs| paired_summary(curve, "bootstrap-amortized", pairs));
        let decision = match (&cold_summary, &bsgs_aa, &rho_aa) {
            (Some(primary), Some(bsgs), Some(rho))
                if interval_contains_one(bsgs) && interval_contains_one(rho) =>
            {
                let interval = primary["ci95_type1"].as_array().unwrap();
                let lower = interval[0].as_f64().unwrap();
                let upper = interval[1].as_f64().unwrap();
                if lower > 1.10 {
                    "supported-at-least-10-percent"
                } else if lower > 1.0 {
                    "supported-positive"
                } else if upper < 1.0 {
                    "bsgs-loss"
                } else {
                    "unresolved-primary-interval"
                }
            }
            (Some(_), Some(_), Some(_)) => "unresolved-aa-gate",
            _ => "unresolved-censored-or-incomplete",
        };
        let normalized = cold_summary.as_ref().map(|summary| {
            let sqrt_n = (curve.n as f64).sqrt();
            json!({
                "bsgs_mean_ns_per_sqrt_n": summary["mean_first_ns"].as_f64().unwrap() / sqrt_n,
                "rho_mean_ns_per_sqrt_n": summary["mean_second_ns"].as_f64().unwrap() / sqrt_n
            })
        });
        curves.push(json!({
            "curve_slug": curve.slug,
            "primary_cold": cold_summary,
            "bsgs_aa_copy1_over_copy0": bsgs_aa,
            "rho_aa_copy1_over_copy0": rho_aa,
            "amortized_B64_secondary": amortized_summary,
            "cold_normalized": normalized,
            "decision": decision
        }));
    }
    json!({
        "schema": "blocked-bsgs-rho-wall/independent-analysis/v1",
        "estimand": "sum(rho_total_ns)/sum(bsgs_total_ns)",
        "aa_orientation": "copy1/copy0",
        "curves": curves
    })
}

fn parse_optional(path: &Path, command: &str, panels: &[&str]) -> Result<(EventSet, bool), String> {
    if path.exists() {
        Ok((parse_event_file(path, command, panels)?, true))
    } else {
        Ok((EventSet::default(), false))
    }
}

fn stable_host_envelope(manifest: &Value) -> Value {
    let host = &manifest["host"];
    json!({
        "hostname": host["hostname"],
        "os_release": host["os_release"],
        "proc_version": host["proc_version"],
        "cpu": host["cpu"],
        "affinity": host["affinity"],
        "cgroup": {
            "membership": host["cgroup"]["membership"],
            "cpu_max": host["cgroup"]["cpu_max"],
            "cpuset_cpus_effective": host["cgroup"]["cpuset_cpus_effective"],
            "memory_max": host["cgroup"]["memory_max"],
            "pids_max": host["cgroup"]["pids_max"]
        },
        "contention_external_status": host["contention"]["external_status"],
        "contention_assessment": host["contention"]["harness_assessment"]
    })
}

fn compare_manifest_identity(manifests: &[&Value]) -> Result<(), String> {
    let Some(first) = manifests.first() else {
        return Ok(());
    };
    let first_host = stable_host_envelope(first);
    for manifest in manifests.iter().skip(1) {
        for field in [
            "binary_sha256",
            "debug_assertions",
            "target_arch",
            "target_os",
            "package_version",
            "rayon_threads",
            "rayon_num_threads_env",
        ] {
            if manifest[field] != first[field] {
                return Err(format!("cross-run manifest mismatch in {field}"));
            }
        }
        if manifest["source"] != first["source"] {
            return Err("cross-run source/protocol/custody mismatch".into());
        }
        if manifest["toolchain"] != first["toolchain"] {
            return Err("cross-run compiler/build identity mismatch".into());
        }
        if stable_host_envelope(manifest) != first_host {
            return Err("cross-run admitted host/isolation envelope mismatch".into());
        }
    }
    Ok(())
}

fn verify_manifest_coherence(study_dir: &Path, event_sets: &[&EventSet]) -> Result<Value, String> {
    for (relative, expected) in [
        ("PROTOCOL.md", PROTOCOL_MD_SHA256),
        ("PROTOCOL-ADDENDUM-1.md", PROTOCOL_ADDENDUM_1_SHA256),
        ("PROTOCOL-ADDENDUM-2.md", PROTOCOL_ADDENDUM_2_SHA256),
        ("seed-manifest.json", SEED_MANIFEST_SHA256),
        ("native/Cargo.lock", NATIVE_CARGO_LOCK_SHA256),
    ] {
        let got = sha256_path(&study_dir.join(relative))?;
        if got != expected {
            return Err(format!(
                "current custody hash mismatch for {relative}: {got}"
            ));
        }
    }
    let protocol_json_sha256 = sha256_path(&study_dir.join("protocol.json"))?;
    let seed_path = study_dir.join("derived-seeds.json");
    let seed_sha256 = sha256_path(&seed_path)?;
    let seed_bytes = fs::metadata(&seed_path)
        .map_err(|e| format!("stat {}: {e}", seed_path.display()))?
        .len();
    let manifests = event_sets
        .iter()
        .filter_map(|events| events.manifest.as_ref())
        .collect::<Vec<_>>();
    for manifest in &manifests {
        let source = &manifest["source"];
        if source["protocol_json_sha256"].as_str() != Some(&protocol_json_sha256)
            || source["derived_seed_mapping_sha256"].as_str() != Some(&seed_sha256)
            || source["derived_seed_mapping_bytes"].as_u64() != Some(seed_bytes)
        {
            return Err("run manifest does not match current protocol/seed custody".into());
        }
    }
    compare_manifest_identity(&manifests)?;
    Ok(json!({
        "present_manifests": manifests.len(),
        "binary_sha256": manifests.first().map(|manifest| manifest["binary_sha256"].clone()),
        "protocol_json_sha256": protocol_json_sha256,
        "derived_seed_mapping_sha256": seed_sha256,
        "derived_seed_mapping_bytes": seed_bytes,
        "cross_run_identity_match": true
    }))
}

pub fn verify_all(study_dir: &Path) -> Result<Value, String> {
    let fixture_report = verify_fixtures()?;
    let seed_report = verify_seed_mapping(&study_dir.join("derived-seeds.json"))?;
    let raw = study_dir.join("raw");
    let (cold, cold_present) =
        parse_optional(&raw.join("cold.jsonl"), "run-cold", &["cold-paired"])?;
    let (aa, aa_present) = parse_optional(&raw.join("aa.jsonl"), "run-aa", &["bsgs-aa", "rho-aa"])?;
    let (amortized, amortized_present) = parse_optional(
        &raw.join("amortized-B64.jsonl"),
        "run-amortized",
        &["amortized-B64"],
    )?;
    let cold_expected = expected_cold_keys();
    let aa_expected = expected_aa_keys();
    let (amortized_intents, amortized_results) = expected_amortized_keys();
    let cold_completeness = validate_event_set(&cold, &cold_expected, &cold_expected, false)?;
    let aa_completeness = validate_event_set(&aa, &aa_expected, &aa_expected, false)?;
    let amortized_completeness =
        validate_event_set(&amortized, &amortized_intents, &amortized_results, true)?;
    let manifest_coherence = verify_manifest_coherence(study_dir, &[&cold, &aa, &amortized])?;
    let any_raw = cold_present || aa_present || amortized_present;
    Ok(json!({
        "schema": "blocked-bsgs-rho-wall/independent-verification-report/v1",
        "study_id": STUDY_ID,
        "status": if any_raw { "validated-present-data" } else { "pre-data" },
        "fixtures": fixture_report,
        "seed_mapping": seed_report,
        "raw_files_present": {
            "cold": cold_present,
            "aa": aa_present,
            "amortized_B64": amortized_present
        },
        "event_completeness": {
            "cold": cold_completeness,
            "aa": aa_completeness,
            "amortized_B64": amortized_completeness
        },
        "manifest_coherence": manifest_coherence,
        "analysis": analyze(&cold, &aa, &amortized)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cold_key(arm: &str) -> EventKey {
        key(FIXTURES[0], "cold-paired", 0, 0, arm, 0)
    }

    fn target_payload(measurement: Value) -> Value {
        let curve = FIXTURES[0];
        let scalar = target_scalar(curve, "cold-paired", 0, 0);
        let (qx, qy) = affine_coords(expected_target(curve, "cold-paired", 0, 0).unwrap()).unwrap();
        json!({
            "target_scalar": scalar,
            "qx": qx,
            "qy": qy,
            "measurement": measurement
        })
    }

    fn result_row(event_key: &EventKey, payload: Value) -> Value {
        json!({
            "schema": "blocked-bsgs-rho-wall/event/v1",
            "event": "result",
            "scope": "arm",
            "curve_slug": event_key.curve,
            "panel": event_key.panel,
            "replicate": event_key.replicate,
            "item": event_key.item,
            "arm": event_key.arm,
            "copy": event_key.copy,
            "status": payload["measurement"]["status"],
            "censored": payload["measurement"]["censored"],
            "error": payload["measurement"]["error"],
            "qx": payload["qx"],
            "qy": payload["qy"],
            "payload": payload
        })
    }

    fn valid_bsgs_row() -> Value {
        let curve = FIXTURES[0];
        let log = target_scalar(curve, "cold-paired", 0, 0);
        let plan = expected_bsgs_plan(curve);
        let table_bytes = 8u64 << plan["table_bits"].as_u64().unwrap();
        result_row(
            &cold_key("bsgs"),
            target_payload(json!({
                "status": "ok",
                "censored": false,
                "error": null,
                "correct": true,
                "log": log,
                "build_ns": 10,
                "search_ns": 20,
                "evaluator_verify_ns": 3,
                "drop_ns": 1,
                "unattributed_ns": 2,
                "total_ns": 36,
                "phase_sum_ns": 36,
                "plan": plan,
                "table_bytes": table_bytes,
                "table_entries": 1,
                "baby_steps": 2,
                "giant_steps": 3,
                "candidates": 1,
                "false_candidates": 0
            })),
        )
    }

    fn valid_censored_rho_row() -> Value {
        let curve = FIXTURES[0];
        let seed = derive_u64(curve.slug, "cold-paired", 0, 0, "rho-walk");
        result_row(
            &cold_key("rho"),
            target_payload(json!({
                "status": "solver_none_or_cap",
                "censored": true,
                "error": "solver_none_or_cap",
                "correct": false,
                "solver_ns": 10,
                "evaluator_verify_ns": 2,
                "drop_ns": 1,
                "unattributed_ns": 1,
                "total_ns": 14,
                "phase_sum_ns": 14,
                "config": expected_rho_config(curve, seed),
                "result": null
            })),
        )
    }

    #[test]
    fn deterministic_miller_rabin_handles_controls() {
        for prime in [2, 3, 37, 65_537, 4_294_967_291] {
            assert!(is_prime_u64(prime), "{prime}");
        }
        for composite in [0, 1, 4, 9, 561, 1_105, 3_215_031_751] {
            assert!(!is_prime_u64(composite), "{composite}");
        }
    }

    #[test]
    fn all_registered_fixtures_pass_independent_group_checks() {
        let report = verify_fixtures().unwrap();
        assert_eq!(report["status"], "ok");
        assert_eq!(report["fixtures"].as_array().unwrap().len(), 5);
    }

    #[test]
    fn seed_derivation_matches_frozen_vector() {
        assert_eq!(
            derive_u64("icv1-fp24-tm5099-a6f137c6", "cold-paired", 0, 0, "target"),
            0xff54_8ce9_2b1d_e5ae
        );
        assert_ne!(
            derive_u64("icv1-fp24-tm5099-a6f137c6", "cold-paired", 0, 0, "target"),
            derive_u64("icv1-fp24-tm5099-a6f137c6", "cold-paired", 0, 0, "rho-walk")
        );
    }

    #[test]
    fn correct_result_replays_and_scalar_mutation_fails() {
        let event_key = cold_key("bsgs");
        let mut row = valid_bsgs_row();
        validate_result(&event_key, &row).unwrap();
        let wrong = row["payload"]["measurement"]["log"].as_u64().unwrap() + 1;
        row["payload"]["measurement"]["log"] = json!(wrong);
        assert!(validate_result(&event_key, &row).is_err());
    }

    #[test]
    fn incorrect_result_is_retained_and_independently_confirmed() {
        let event_key = cold_key("bsgs");
        let mut row = valid_bsgs_row();
        let wrong = row["payload"]["measurement"]["log"].as_u64().unwrap() + 1;
        row["payload"]["measurement"]["log"] = json!(wrong);
        row["payload"]["measurement"]["status"] = json!("incorrect");
        row["payload"]["measurement"]["censored"] = json!(false);
        row["payload"]["measurement"]["error"] = json!("evaluator_mismatch");
        row["payload"]["measurement"]["correct"] = json!(false);
        row["status"] = json!("incorrect");
        row["censored"] = json!(false);
        row["error"] = json!("evaluator_mismatch");
        validate_result(&event_key, &row).unwrap();
    }

    #[test]
    fn phase_and_plan_mutations_fail() {
        let event_key = cold_key("bsgs");
        let mut phase = valid_bsgs_row();
        phase["payload"]["measurement"]["phase_sum_ns"] = json!(35);
        assert!(validate_result(&event_key, &phase).is_err());

        let mut plan = valid_bsgs_row();
        plan["payload"]["measurement"]["plan"]["pair_k"] = json!(31);
        assert!(validate_result(&event_key, &plan).is_err());
    }

    #[test]
    fn rho_config_is_fully_bound() {
        let event_key = cold_key("rho");
        let mut row = valid_censored_rho_row();
        validate_result(&event_key, &row).unwrap();
        row["payload"]["measurement"]["config"]["threads"] = json!(2);
        assert!(validate_result(&event_key, &row).is_err());
    }

    #[test]
    fn unmatched_intent_is_reported_as_incomplete_not_dropped() {
        let event_key = cold_key("bsgs");
        let curve = FIXTURES[0];
        let (qx, qy) = affine_coords(expected_target(curve, "cold-paired", 0, 0).unwrap()).unwrap();
        let intent = json!({
            "schema": "blocked-bsgs-rho-wall/event/v1",
            "event": "intent",
            "scope": "arm",
            "curve_slug": curve.slug,
            "panel": "cold-paired",
            "replicate": 0,
            "item": 0,
            "arm": "bsgs",
            "copy": 0,
            "target_seed": derive_u64(curve.slug, "cold-paired", 0, 0, "target"),
            "rho_seed": null,
            "qx": qx,
            "qy": qy,
            "schedule_position": 0,
            "invocation_label": "bsgs"
        });
        let mut events = EventSet::default();
        events.intents.insert(event_key.clone(), intent);
        let expected = BTreeSet::from([event_key]);
        let report = validate_event_set(&events, &expected, &expected, false).unwrap();
        assert_eq!(report["unmatched_intents"], 1);
        assert_eq!(report["complete"], false);
    }

    #[test]
    fn fixed_interface_rejects_unknown_curves() {
        assert!(fixture("external-curve").is_err());
    }

    #[test]
    fn bootstrap_is_deterministic_on_small_control() {
        let digest = bootstrap_digest(FIXTURES[0], "bootstrap-primary", 7, 3);
        assert_eq!(digest.len(), 64);
        assert_eq!(
            digest,
            bootstrap_digest(FIXTURES[0], "bootstrap-primary", 7, 3)
        );
    }

    #[test]
    fn ratio_of_sums_and_bootstrap_orientation_are_fixed() {
        let summary = paired_summary(FIXTURES[0], "bootstrap-primary", &[(10, 20), (20, 40)]);
        assert_eq!(summary["ratio_of_sums"].as_f64(), Some(2.0));
        assert_eq!(summary["mean_second_minus_first_ns"].as_f64(), Some(15.0));
        assert_eq!(summary["ci95_type1"][0].as_f64(), Some(2.0));
        assert_eq!(summary["ci95_type1"][1].as_f64(), Some(2.0));
    }

    #[test]
    fn cross_run_identity_rejects_binary_and_host_drift() {
        let base = json!({
            "binary_sha256": "a",
            "debug_assertions": false,
            "target_arch": "x86_64",
            "target_os": "linux",
            "package_version": "0.1.0",
            "rayon_threads": 1,
            "rayon_num_threads_env": "1",
            "source": {"pinned": true},
            "toolchain": {"build": "same"},
            "host": {
                "hostname": "host",
                "os_release": "os",
                "proc_version": "kernel",
                "cpu": {"model": "cpu"},
                "affinity": {"cpus_allowed_list": "3"},
                "cgroup": {
                    "membership": "group",
                    "cpu_max": "100000 100000",
                    "cpuset_cpus_effective": "3",
                    "memory_max": "4294967296",
                    "pids_max": "64"
                },
                "contention": {
                    "external_status": "isolated",
                    "harness_assessment": "not_inferred; use admitted runner evidence"
                }
            }
        });
        let same = base.clone();
        compare_manifest_identity(&[&base, &same]).unwrap();

        let mut binary_drift = base.clone();
        binary_drift["binary_sha256"] = json!("b");
        assert!(compare_manifest_identity(&[&base, &binary_drift]).is_err());

        let mut host_drift = base.clone();
        host_drift["host"]["affinity"]["cpus_allowed_list"] = json!("4");
        assert!(compare_manifest_identity(&[&base, &host_drift]).is_err());
    }
}
