// Correctness lives in tests/harness.rs. A lib-unit-test build would activate
// the pinned modules' repository-wide `cfg(test)` suites, which require the
// full parent crate rather than this minimal compatibility crate.
#![cfg(not(test))]
#![recursion_limit = "256"]
// The pinned source modules predate these style lints; their bodies must remain byte-exact.
#![allow(
    clippy::manual_is_multiple_of,
    clippy::needless_range_loop,
    clippy::too_many_arguments
)]

use std::collections::HashSet;
use std::fmt::Write as _;
use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub mod ecc;

pub mod cryptanalysis {
    pub mod residual_walk {
        #[inline]
        fn mul_mod(a: u64, b: u64, n: u64) -> u64 {
            ((a as u128 * b as u128) % n as u128) as u64
        }

        fn pow_mod(mut a: u64, mut e: u64, n: u64) -> u64 {
            let mut out = 1u64;
            while e > 0 {
                if e & 1 == 1 {
                    out = mul_mod(out, a, n);
                }
                a = mul_mod(a, a, n);
                e >>= 1;
            }
            out
        }

        pub fn is_prime_u64(n: u64) -> bool {
            if n < 2 {
                return false;
            }
            for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
                if n == p {
                    return true;
                }
                if n.is_multiple_of(p) {
                    return false;
                }
            }
            let s = (n - 1).trailing_zeros();
            let d = (n - 1) >> s;
            for a in [2u64, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022] {
                let a = a % n;
                if a == 0 {
                    continue;
                }
                let mut x = pow_mod(a, d, n);
                if x == 1 || x == n - 1 {
                    continue;
                }
                let mut witness = true;
                for _ in 1..s {
                    x = mul_mod(x, x, n);
                    if x == n - 1 {
                        witness = false;
                        break;
                    }
                }
                if witness {
                    return false;
                }
            }
            true
        }
    }
}

#[path = "../../../../src/cryptanalysis/bsgs_fast.rs"]
pub mod bsgs_fast;
#[path = "../../../../src/cryptanalysis/prime_factor_base_controls.rs"]
pub mod prime_factor_base_controls;
#[path = "../../../../src/cryptanalysis/prime_fast.rs"]
pub mod prime_fast;

use bsgs_fast::{BsgsFast, BsgsFastPlan, StepKernel};
use ecc::curve::CurveParams;
use prime_fast::{rho_parallel, RhoConfig, RhoOpCounts};

const STUDY_ID: &str = "blocked-bsgs-rho-wall/v1";
const MASTER_HEX: &str = "1aaea7ece2a35976043741b3a869b6ddc9a3798b18596b0635ee52829e3bba38";

const BSGS_SHA256: &str = "ebe890abcbdcfb2e719eff99e046847f4f1efb0864f627ba2ad2a073a5f4a7bb";
const RHO_SHA256: &str = "6a9dc015b1558045935912688beebf141a6b4161c045c821faac81fc3a430bc1";
const SEED_MANIFEST_SHA256: &str =
    "45fa0e04de6c91c567dea346849fc74c12d9537814124086649dc47ade8e6c5f";
const PROTOCOL_MD_SHA256: &str = "53548103cbf898dab3f39bf0de8bebb5ced07184855baa82556079395e58c773";
const PROTOCOL_ADDENDUM_1_SHA256: &str =
    "45cf548ae2c1fa71e55f713e933f74b743052bb403fd1846767026eef4082801";
const PROTOCOL_ADDENDUM_2_SHA256: &str =
    "d581de654830d1730eac4a3019e323752f5dc98cbb4d7276e9943c954191a1c1";
const PROTOCOL_JSON_BODY_SHA256: &str =
    "a6c0cf1afac40deda80081b5c9eeedf94b0d8884b1f679c8a2f412d995f888bc";
const CARGO_LOCK_SHA256: &str = "cd56d62407c6ef09b5fbd0cdf187b2fd684c609a1b2df42b301a8eb67c699b17";
const COMPACT_SEED_LIMIT_BYTES: usize = 1 << 20;

#[derive(Clone, Copy)]
struct PanelSpec {
    name: &'static str,
    replicates: u64,
    items: u64,
    has_rho: bool,
    aa_schedule: bool,
}

const PANEL_SPECS: [PanelSpec; 5] = [
    PanelSpec {
        name: "warmup",
        replicates: 4,
        items: 1,
        has_rho: true,
        aa_schedule: false,
    },
    PanelSpec {
        name: "cold-paired",
        replicates: 512,
        items: 1,
        has_rho: true,
        aa_schedule: false,
    },
    PanelSpec {
        name: "bsgs-aa",
        replicates: 64,
        items: 1,
        has_rho: false,
        aa_schedule: true,
    },
    PanelSpec {
        name: "rho-aa",
        replicates: 64,
        items: 1,
        has_rho: true,
        aa_schedule: true,
    },
    PanelSpec {
        name: "amortized-B64",
        replicates: 16,
        items: 64,
        has_rho: true,
        aa_schedule: false,
    },
];

const BOOTSTRAP_SPECS: [(&str, u64); 4] = [
    ("bootstrap-primary", 512),
    ("bootstrap-bsgs-aa", 64),
    ("bootstrap-rho-aa", 64),
    ("bootstrap-amortized", 16),
];

#[derive(Clone, Copy, Debug)]
pub struct Fixture {
    pub slug: &'static str,
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub gx: u64,
    pub gy: u64,
    pub n: u64,
}

impl Fixture {
    pub fn params(self) -> CurveParams {
        CurveParams {
            name: self.slug,
            p: BigUint::from(self.p),
            a: BigUint::from(self.a),
            b: BigUint::from(self.b),
            gx: BigUint::from(self.gx),
            gy: BigUint::from(self.gy),
            n: BigUint::from(self.n),
            h: 1,
        }
    }
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

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|e| format!("open {}: {e}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 1 << 16];
    loop {
        let n = file
            .read(&mut buf)
            .map_err(|e| format!("read {}: {e}", path.display()))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}

fn canonical_json(value: &Value) -> Result<Vec<u8>, String> {
    fn emit(value: &Value, output: &mut Vec<u8>) -> Result<(), String> {
        match value {
            Value::Object(map) => {
                output.push(b'{');
                let mut keys: Vec<_> = map.keys().collect();
                keys.sort_unstable();
                for (index, key) in keys.into_iter().enumerate() {
                    if index != 0 {
                        output.push(b',');
                    }
                    output.extend(serde_json::to_vec(key).map_err(|error| error.to_string())?);
                    output.push(b':');
                    emit(&map[key], output)?;
                }
                output.push(b'}');
            }
            Value::Array(items) => {
                output.push(b'[');
                for (index, item) in items.iter().enumerate() {
                    if index != 0 {
                        output.push(b',');
                    }
                    emit(item, output)?;
                }
                output.push(b']');
            }
            _ => output.extend(
                serde_json::to_vec(value).map_err(|error| format!("canonical JSON: {error}"))?,
            ),
        }
        Ok(())
    }

    let mut output = Vec::new();
    emit(value, &mut output)?;
    Ok(output)
}

fn protocol_body_sha256(protocol: &Value) -> Result<String, String> {
    let mut body = protocol.clone();
    body.as_object_mut()
        .ok_or_else(|| "protocol.json is not an object".to_owned())?
        .remove("freeze");
    Ok(hex::encode(Sha256::digest(canonical_json(&body)?)))
}

pub fn verify_machine_protocol_value(protocol: &Value) -> Result<(), String> {
    if protocol["schema"] != "experiment.protocol/v1" {
        return Err("protocol.json schema mismatch".into());
    }
    let freeze = &protocol["freeze"];
    if freeze["status"] != "frozen" || freeze["filed_before_affected_data"] != true {
        return Err("protocol.json is not a prospectively filed freeze".into());
    }
    let commit = freeze["commit"]
        .as_str()
        .ok_or_else(|| "protocol.json freeze commit is missing".to_owned())?;
    if commit.len() != 40
        || !commit
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err("protocol.json freeze commit is not a lowercase Git OID".into());
    }
    let computed = protocol_body_sha256(protocol)?;
    let declared = freeze["protocol_sha256"]
        .as_str()
        .ok_or_else(|| "protocol.json canonical body hash is missing".to_owned())?;
    if declared != computed {
        return Err(format!(
            "protocol.json canonical body hash mismatch: {declared} != {computed}"
        ));
    }
    if computed != PROTOCOL_JSON_BODY_SHA256 {
        return Err(format!(
            "protocol.json body is not the compiled freeze: {computed} != {PROTOCOL_JSON_BODY_SHA256}"
        ));
    }
    Ok(())
}

pub fn verify_source_hashes() -> Result<(), String> {
    let root = manifest_dir().join("../../..");
    for (rel, want) in [
        ("src/cryptanalysis/bsgs_fast.rs", BSGS_SHA256),
        ("src/cryptanalysis/prime_fast.rs", RHO_SHA256),
        (
            "research/blocked_bsgs_rho_wall_20261126/PROTOCOL.md",
            PROTOCOL_MD_SHA256,
        ),
        (
            "research/blocked_bsgs_rho_wall_20261126/PROTOCOL-ADDENDUM-1.md",
            PROTOCOL_ADDENDUM_1_SHA256,
        ),
        (
            "research/blocked_bsgs_rho_wall_20261126/PROTOCOL-ADDENDUM-2.md",
            PROTOCOL_ADDENDUM_2_SHA256,
        ),
        (
            "research/blocked_bsgs_rho_wall_20261126/seed-manifest.json",
            SEED_MANIFEST_SHA256,
        ),
        (
            "research/blocked_bsgs_rho_wall_20261126/native/Cargo.lock",
            CARGO_LOCK_SHA256,
        ),
    ] {
        let path = root.join(rel);
        let got = sha256_file(&path)?;
        if got != want {
            return Err(format!(
                "frozen custody hash mismatch for {rel}: {got} != {want}"
            ));
        }
    }
    Ok(())
}

fn master_bytes() -> [u8; 32] {
    let decoded = hex::decode(MASTER_HEX).expect("frozen master seed is hex");
    decoded.try_into().expect("frozen master seed is 32 bytes")
}

pub fn derive_u64(
    curve_slug: &str,
    panel: &str,
    replicate: u64,
    item: u64,
    stream_label: &str,
) -> u64 {
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
    hasher.update(stream_label.as_bytes());
    hasher.update([0]);
    hasher.update(master_bytes());
    let digest = hasher.finalize();
    u64::from_le_bytes(digest[..8].try_into().expect("eight bytes"))
}

pub fn target_scalar(fixture: Fixture, panel: &str, replicate: u64, item: u64) -> u64 {
    1 + derive_u64(fixture.slug, panel, replicate, item, "target") % (fixture.n - 1)
}

pub fn rho_seed(fixture: Fixture, panel: &str, replicate: u64, item: u64) -> u64 {
    derive_u64(fixture.slug, panel, replicate, item, "rho-walk")
}

fn ceil_sqrt(n: u64) -> u64 {
    let mut lo = 0u64;
    let mut hi = 1u64 << 32;
    while lo + 1 < hi {
        let mid = lo + (hi - lo) / 2;
        if (mid as u128) * (mid as u128) >= n as u128 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    hi
}

fn target_pair(
    fixture: Fixture,
    panel: &str,
    replicate: u64,
    item: u64,
) -> Result<
    (
        u64,
        bsgs_fast::FastCurve,
        bsgs_fast::FastPoint,
        prime_fast::FastCurve,
        prime_fast::Pt,
    ),
    String,
> {
    let params = fixture.params();
    let bcurve = bsgs_fast::FastCurve::from_params(&params)
        .ok_or_else(|| format!("BSGS rejects {}", fixture.slug))?;
    let pcurve = prime_fast::FastCurve::from_params(&params)
        .ok_or_else(|| format!("rho rejects {}", fixture.slug))?;
    let pg = pcurve
        .from_textbook(&params.generator())
        .ok_or_else(|| format!("rho generator is infinity for {}", fixture.slug))?;
    let k = target_scalar(fixture, panel, replicate, item);
    let bq = bcurve.scalar_mul(bcurve.g, k);
    let pq = pcurve
        .mul(Some(pg), k)
        .ok_or_else(|| format!("target is infinity for {} k={k}", fixture.slug))?;
    if bcurve.lower(bq) != pcurve.to_textbook(Some(pq)) {
        return Err(format!(
            "cross-representation target mismatch for {}",
            fixture.slug
        ));
    }
    Ok((k, bcurve, bq, pcurve, pq))
}

fn bsgs_plan_value(plan: &BsgsFastPlan) -> Value {
    json!({
        "neg_map": plan.neg_map,
        "width": plan.width,
        "x0": plan.x0,
        "m": plan.m,
        "stride": plan.stride,
        "giant_count": plan.giant_count,
        "table_bits": plan.table_bits,
        "workers": plan.workers,
        "chains_per_worker": plan.chains_per_worker,
        "baby_len": plan.baby_len,
        "giant_len": plan.giant_len,
        "kernel": plan.kernel.name(),
        "pair_k": plan.pair_k
    })
}

fn rho_config_value(cfg: &RhoConfig) -> Value {
    json!({
        "threads": cfg.threads,
        "walkers": cfg.walkers,
        "table_bits": cfg.table_bits,
        "dp_bits": cfg.dp_bits,
        "negation": cfg.negation,
        "seed": cfg.seed,
        "max_steps": cfg.max_steps,
        "look_ahead": cfg.look_ahead,
        "cycle_window": cfg.cycle_window,
        "coeff_bits": cfg.coeff_bits,
        "stride_starts": cfg.stride_starts
    })
}

fn process_memory_kib() -> (Option<u64>, Option<u64>) {
    let Ok(status) = fs::read_to_string("/proc/self/status") else {
        return (None, None);
    };
    let field = |name: &str| {
        status.lines().find_map(|line| {
            let rest = line.strip_prefix(name)?.trim();
            rest.split_ascii_whitespace().next()?.parse::<u64>().ok()
        })
    };
    (field("VmHWM:"), field("VmRSS:"))
}

fn outcome(got: Option<u64>, correct: bool) -> (&'static str, bool, Option<&'static str>) {
    if got.is_none() {
        ("solver_none_or_cap", true, Some("solver_none_or_cap"))
    } else if !correct {
        ("incorrect", false, Some("evaluator_mismatch"))
    } else {
        ("ok", false, None)
    }
}

fn bsgs_run(fixture: Fixture, bcurve: bsgs_fast::FastCurve, q: bsgs_fast::FastPoint) -> Value {
    let plan = BsgsFastPlan::new(fixture.n, 0, true, 1, 8).with_kernel(StepKernel::Paired, 32);
    let table_bytes = plan.table_bytes();
    let plan_record = bsgs_plan_value(&plan);
    let total_start = Instant::now();
    let build_start = Instant::now();
    let mut solver = BsgsFast::build(bcurve, plan);
    let build_ns = build_start.elapsed().as_nanos() as u64;
    let search_start = Instant::now();
    let got = solver.solve(&q);
    let search_ns = search_start.elapsed().as_nanos() as u64;
    let verify_start = Instant::now();
    let correct = got
        .map(|log| bcurve.scalar_mul(bcurve.g, log) == q)
        .unwrap_or(false);
    let verify_ns = verify_start.elapsed().as_nanos() as u64;
    let stats = solver.stats();
    let drop_start = Instant::now();
    drop(solver);
    let drop_ns = drop_start.elapsed().as_nanos() as u64;
    let total_ns = total_start.elapsed().as_nanos() as u64;
    let attributed_ns = build_ns
        .saturating_add(search_ns)
        .saturating_add(verify_ns)
        .saturating_add(drop_ns);
    let unattributed_ns = total_ns
        .checked_sub(attributed_ns)
        .expect("BSGS phase timers must be enclosed by total timer");
    let (process_hwm_kib, process_rss_kib) = process_memory_kib();
    let (status, censored, error) = outcome(got, correct);
    json!({
        "status": status,
        "censored": censored,
        "error": error,
        "log": got,
        "correct": correct,
        "build_ns": build_ns,
        "search_ns": search_ns,
        "evaluator_verify_ns": verify_ns,
        "drop_ns": drop_ns,
        "unattributed_ns": unattributed_ns,
        "total_ns": total_ns,
        "phase_sum_ns": attributed_ns.saturating_add(unattributed_ns),
        "plan": plan_record,
        "table_bytes": table_bytes,
        "process_hwm_kib": process_hwm_kib,
        "process_rss_kib": process_rss_kib,
        "table_entries": stats.table_entries,
        "baby_steps": stats.baby_steps,
        "giant_steps": stats.giant_steps,
        "candidates": stats.candidates,
        "false_candidates": stats.false_candidates
    })
}

fn rho_ops_value(ops: RhoOpCounts) -> Value {
    json!({
        "table_adds": ops.table_adds,
        "table_doubles": ops.table_doubles,
        "initial_start_adds": ops.initial_start_adds,
        "initial_start_doubles": ops.initial_start_doubles,
        "restart_adds": ops.restart_adds,
        "restart_doubles": ops.restart_doubles,
        "escape_doubles": ops.escape_doubles,
        "collision_checks": ops.collision_checks,
        "look_ahead_adds": ops.look_ahead_adds,
        "cycles_2": ops.cycles_2,
        "cycles_4": ops.cycles_4,
        "cycles_other": ops.cycles_other,
        "abandoned": ops.abandoned,
        "capped": ops.capped,
        "restarts": ops.restarts,
        "dp_trail_steps": ops.dp_trail_steps,
        "lost_steps": ops.lost_steps,
        "in_flight_steps": ops.in_flight_steps
    })
}

#[derive(Clone, Copy)]
struct RhoResultSnapshot {
    log: u64,
    total_steps: u64,
    distinguished_points: u64,
    wall_secs: f64,
    steps_per_sec: f64,
    threads: usize,
    walkers_per_thread: usize,
    dp_bits: u32,
    ops: RhoOpCounts,
}

struct RhoMeasurement {
    correct: bool,
    solver_ns: u64,
    evaluator_verify_ns: u64,
    drop_ns: u64,
    unattributed_ns: u64,
    total_ns: u64,
    config: RhoConfig,
    result: Option<RhoResultSnapshot>,
}

fn fixed_rho_config(fixture: Fixture, seed: u64) -> RhoConfig {
    let mut config = RhoConfig::tuned(fixture.n);
    config.threads = 1;
    config.seed = seed;
    config.max_steps = 64u64.saturating_mul(ceil_sqrt(fixture.n));
    config
}

#[allow(clippy::drop_non_drop)] // Deliberately time destruction of the returned solver state.
fn measure_rho_with_config(
    pcurve: &prime_fast::FastCurve,
    g: prime_fast::Pt,
    q: prime_fast::Pt,
    config: RhoConfig,
) -> RhoMeasurement {
    let total_start = Instant::now();
    let solver_start = Instant::now();
    let got = rho_parallel(pcurve, g, q, &config);
    let solver_ns = solver_start.elapsed().as_nanos() as u64;
    let verify_start = Instant::now();
    let correct = got
        .as_ref()
        .map(|result| pcurve.mul(Some(g), result.log) == Some(q))
        .unwrap_or(false);
    let evaluator_verify_ns = verify_start.elapsed().as_nanos() as u64;
    let result = got.as_ref().map(|result| RhoResultSnapshot {
        log: result.log,
        total_steps: result.total_steps,
        distinguished_points: result.distinguished_points,
        wall_secs: result.wall_secs,
        steps_per_sec: result.steps_per_sec,
        threads: result.threads,
        walkers_per_thread: result.walkers_per_thread,
        dp_bits: result.dp_bits,
        ops: result.ops,
    });
    let drop_start = Instant::now();
    drop(got);
    let drop_ns = drop_start.elapsed().as_nanos() as u64;
    let total_ns = total_start.elapsed().as_nanos() as u64;
    let attributed_ns = solver_ns
        .saturating_add(evaluator_verify_ns)
        .saturating_add(drop_ns);
    let unattributed_ns = total_ns
        .checked_sub(attributed_ns)
        .expect("rho phase timers must be enclosed by total timer");
    RhoMeasurement {
        correct,
        solver_ns,
        evaluator_verify_ns,
        drop_ns,
        unattributed_ns,
        total_ns,
        config,
        result,
    }
}

fn measure_rho(
    fixture: Fixture,
    pcurve: &prime_fast::FastCurve,
    q: prime_fast::Pt,
    seed: u64,
) -> RhoMeasurement {
    let params = fixture.params();
    let g = pcurve
        .from_textbook(&params.generator())
        .expect("fixed generator is affine");
    measure_rho_with_config(pcurve, g, q, fixed_rho_config(fixture, seed))
}

fn rho_measurement_value(
    measurement: &RhoMeasurement,
    process_hwm_kib: Option<u64>,
    process_rss_kib: Option<u64>,
) -> Value {
    let result = measurement.result.map(|result| {
        json!({
            "log": result.log,
            "total_steps": result.total_steps,
            "distinguished_points": result.distinguished_points,
            "inner_wall_ns": (result.wall_secs * 1_000_000_000.0).round() as u64,
            "steps_per_sec": result.steps_per_sec,
            "threads": result.threads,
            "walkers_per_thread": result.walkers_per_thread,
            "dp_bits": result.dp_bits,
            "ops": rho_ops_value(result.ops)
        })
    });
    let recovered = measurement.result.map(|result| result.log);
    let (status, censored, error) = outcome(recovered, measurement.correct);
    let phase_sum_ns = measurement
        .solver_ns
        .saturating_add(measurement.evaluator_verify_ns)
        .saturating_add(measurement.drop_ns)
        .saturating_add(measurement.unattributed_ns);
    json!({
        "status": status,
        "censored": censored,
        "error": error,
        "correct": measurement.correct,
        "solver_ns": measurement.solver_ns,
        "evaluator_verify_ns": measurement.evaluator_verify_ns,
        "drop_ns": measurement.drop_ns,
        "unattributed_ns": measurement.unattributed_ns,
        "total_ns": measurement.total_ns,
        "phase_sum_ns": phase_sum_ns,
        "config": rho_config_value(&measurement.config),
        "process_hwm_kib": process_hwm_kib,
        "process_rss_kib": process_rss_kib,
        "result": result
    })
}

fn rho_run(
    fixture: Fixture,
    pcurve: &prime_fast::FastCurve,
    q: prime_fast::Pt,
    seed: u64,
) -> Value {
    let measurement = measure_rho(fixture, pcurve, q, seed);
    let (process_hwm_kib, process_rss_kib) = process_memory_kib();
    rho_measurement_value(&measurement, process_hwm_kib, process_rss_kib)
}

fn ensure_one_rayon_thread() -> Result<(), String> {
    if rayon::current_num_threads() != 1 {
        return Err(format!(
            "RAYON thread count is {}, expected 1",
            rayon::current_num_threads()
        ));
    }
    Ok(())
}

fn ensure_timing_environment() -> Result<(), String> {
    ensure_one_rayon_thread()?;
    if cfg!(debug_assertions) {
        return Err("timing subcommands require a --release build".into());
    }
    Ok(())
}

pub fn validate_git_revision(
    build_head: &str,
    build_dirty: bool,
    runtime_head: &str,
    runtime_status: &str,
) -> Result<(), String> {
    let valid_oid = |value: &str| {
        matches!(value.len(), 40 | 64)
            && value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    };
    if !valid_oid(build_head) {
        return Err(format!("invalid build git revision {build_head:?}"));
    }
    if build_dirty {
        return Err("timing refused: binary was built from a dirty worktree".into());
    }
    if !valid_oid(runtime_head) {
        return Err(format!("invalid runtime git revision {runtime_head:?}"));
    }
    if build_head != runtime_head {
        return Err(format!(
            "build/runtime git revision mismatch: {build_head} != {runtime_head}"
        ));
    }
    if !runtime_status.trim().is_empty() {
        return Err(format!(
            "timing refused: runtime worktree is dirty: {}",
            runtime_status.replace('\n', " | ")
        ));
    }
    Ok(())
}

fn git_stdout(root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|error| format!("run git {}: {error}", args.join(" ")))?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

#[derive(Clone, Debug)]
struct GitCustody {
    build_head: &'static str,
    build_dirty: bool,
    build_status: &'static str,
    runtime_head: String,
    runtime_status: String,
}

fn verify_git_custody() -> Result<GitCustody, String> {
    let build_head = env!("HARNESS_BUILD_GIT_HEAD");
    let build_dirty = match env!("HARNESS_BUILD_GIT_DIRTY") {
        "true" => true,
        "false" => false,
        value => return Err(format!("invalid build git dirty flag {value:?}")),
    };
    let build_status = env!("HARNESS_BUILD_GIT_STATUS");
    if build_dirty != !build_status.trim().is_empty() {
        return Err("inconsistent embedded build git status".into());
    }
    let root = manifest_dir().join("../../..");
    let runtime_head = git_stdout(&root, &["rev-parse", "--verify", "HEAD"])?;
    let runtime_status = git_stdout(
        &root,
        &[
            "status",
            "--porcelain=v1",
            "--untracked-files=all",
            "--",
            ".",
            ":(exclude)research/blocked_bsgs_rho_wall_20261126/raw/**",
        ],
    )?;
    validate_git_revision(build_head, build_dirty, &runtime_head, &runtime_status)?;
    Ok(GitCustody {
        build_head,
        build_dirty,
        build_status,
        runtime_head,
        runtime_status,
    })
}

pub fn verify_fixtures() -> Result<Value, String> {
    verify_source_hashes()?;
    ensure_one_rayon_thread()?;
    let mut checks = Vec::new();
    for fixture in FIXTURES {
        if !cryptanalysis::residual_walk::is_prime_u64(fixture.p)
            || !cryptanalysis::residual_walk::is_prime_u64(fixture.n)
        {
            return Err(format!(
                "non-prime fixture modulus/order for {}",
                fixture.slug
            ));
        }
        let params = fixture.params();
        let bc = bsgs_fast::FastCurve::from_params(&params)
            .ok_or_else(|| format!("BSGS rejects {}", fixture.slug))?;
        let pc = prime_fast::FastCurve::from_params(&params)
            .ok_or_else(|| format!("rho rejects {}", fixture.slug))?;
        let pg = pc
            .from_textbook(&params.generator())
            .expect("affine generator");
        if !bc.on_curve(bc.g) || !pc.is_on_curve(pg) {
            return Err(format!("generator off curve for {}", fixture.slug));
        }
        if !bc.scalar_mul(bc.g, fixture.n).infinity || pc.mul(Some(pg), fixture.n).is_some() {
            return Err(format!("generator order check failed for {}", fixture.slug));
        }
        let (k, _, _, _, _) = target_pair(fixture, "cold-paired", 0, 0)?;
        checks.push(json!({"curve_slug": fixture.slug, "target_scalar": k, "order_check": true}));
    }

    let fixture = FIXTURES[0];
    let (secret, bc, bq, pc, pq) = target_pair(fixture, "cold-paired", 0, 0)?;
    let b = bsgs_run(fixture, bc, bq);
    let r = rho_run(fixture, &pc, pq, rho_seed(fixture, "cold-paired", 0, 0));
    if b["log"].as_u64() != Some(secret)
        || r["result"]["log"].as_u64() != Some(secret)
        || b["correct"] != true
        || r["correct"] != true
    {
        return Err("fixed 24-bit solver smoke did not recover the planted scalar".into());
    }
    Ok(json!({
        "schema": "blocked-bsgs-rho-wall/fixture-check/v1",
        "source_hashes_ok": true,
        "rayon_threads": rayon::current_num_threads(),
        "fixtures": checks,
        "fixed_solver_smoke": {"curve_slug": fixture.slug, "secret": secret, "bsgs": b, "rho": r}
    }))
}

fn write_json_line(writer: &mut BufWriter<File>, value: &Value) -> Result<(), String> {
    serde_json::to_writer(&mut *writer, value).map_err(|e| e.to_string())?;
    writer.write_all(b"\n").map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.to_string())
}

fn fresh_writer(path: &Path) -> Result<BufWriter<File>, String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()))?;
    }
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|e| format!("refuse replacement/open {}: {e}", path.display()))?;
    Ok(BufWriter::new(file))
}

fn read_trimmed(path: &str) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .map(|value| value.trim().to_owned())
}

fn colon_field<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    text.lines()
        .find_map(|line| line.strip_prefix(name).map(str::trim))
}

fn command_version(program: &str, args: &[&str]) -> Value {
    match Command::new(program).args(args).output() {
        Ok(output) => json!({
            "program": program,
            "args": args,
            "success": output.status.success(),
            "exit_code": output.status.code(),
            "stdout": String::from_utf8_lossy(&output.stdout).trim(),
            "stderr": String::from_utf8_lossy(&output.stderr).trim()
        }),
        Err(error) => json!({
            "program": program,
            "args": args,
            "success": false,
            "error": error.to_string()
        }),
    }
}

fn cpu_envelope() -> Value {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").ok();
    let first_cpu = cpuinfo
        .as_deref()
        .and_then(|value| value.split("\n\n").next());
    json!({
        "model_name": first_cpu.and_then(|value| colon_field(value, "model name\t:")),
        "vendor_id": first_cpu.and_then(|value| colon_field(value, "vendor_id\t:")),
        "cpu_family": first_cpu.and_then(|value| colon_field(value, "cpu family\t:")),
        "model": first_cpu.and_then(|value| colon_field(value, "model\t\t:")),
        "stepping": first_cpu.and_then(|value| colon_field(value, "stepping\t:")),
        "microcode": first_cpu.and_then(|value| colon_field(value, "microcode\t:")),
        "flags": first_cpu
            .and_then(|value| colon_field(value, "flags\t\t:"))
            .or_else(|| first_cpu.and_then(|value| colon_field(value, "Features\t:"))),
        "logical_cpu_blocks": cpuinfo
            .as_deref()
            .map(|value| value.split("\n\n").filter(|block| !block.trim().is_empty()).count())
    })
}

fn process_counter_envelope(status: Option<&str>) -> Value {
    let stat = fs::read_to_string("/proc/self/stat").ok();
    let stat_fields = stat.as_deref().and_then(|value| {
        let end = value.rfind(')')?;
        Some(
            value[end + 1..]
                .split_ascii_whitespace()
                .collect::<Vec<_>>(),
        )
    });
    let stat_u64 = |index: usize| {
        stat_fields
            .as_ref()
            .and_then(|fields| fields.get(index))
            .and_then(|value| value.parse::<u64>().ok())
    };
    let status_u64 = |name: &str| {
        status
            .and_then(|value| colon_field(value, name))
            .and_then(|value| value.split_ascii_whitespace().next())
            .and_then(|value| value.parse::<u64>().ok())
    };
    json!({
        "voluntary_context_switches": status_u64("voluntary_ctxt_switches:"),
        "nonvoluntary_context_switches": status_u64("nonvoluntary_ctxt_switches:"),
        "minor_faults": stat_u64(7),
        "child_minor_faults": stat_u64(8),
        "major_faults": stat_u64(9),
        "child_major_faults": stat_u64(10)
    })
}

fn host_envelope() -> Value {
    let status = fs::read_to_string("/proc/self/status").ok();
    let status_ref = status.as_deref();
    json!({
        "captured_before_warmups": true,
        "hostname": read_trimmed("/etc/hostname"),
        "os_release": read_trimmed("/etc/os-release"),
        "proc_version": read_trimmed("/proc/version"),
        "cpu": cpu_envelope(),
        "affinity": {
            "cpus_allowed_hex": status_ref.and_then(|value| colon_field(value, "Cpus_allowed:")),
            "cpus_allowed_list": status_ref.and_then(|value| colon_field(value, "Cpus_allowed_list:")),
            "mems_allowed_hex": status_ref.and_then(|value| colon_field(value, "Mems_allowed:")),
            "mems_allowed_list": status_ref.and_then(|value| colon_field(value, "Mems_allowed_list:"))
        },
        "cgroup": {
            "membership": read_trimmed("/proc/self/cgroup"),
            "cpu_max": read_trimmed("/sys/fs/cgroup/cpu.max"),
            "cpuset_cpus_effective": read_trimmed("/sys/fs/cgroup/cpuset.cpus.effective"),
            "memory_max": read_trimmed("/sys/fs/cgroup/memory.max"),
            "memory_current": read_trimmed("/sys/fs/cgroup/memory.current"),
            "pids_max": read_trimmed("/sys/fs/cgroup/pids.max")
        },
        "loadavg": read_trimmed("/proc/loadavg"),
        "psi": {
            "cpu": read_trimmed("/proc/pressure/cpu"),
            "memory": read_trimmed("/proc/pressure/memory"),
            "io": read_trimmed("/proc/pressure/io")
        },
        "process_counters": process_counter_envelope(status_ref),
        "contention": {
            "external_status": std::env::var("ECBENCH_CONTENTION_STATUS").ok(),
            "harness_assessment": "not_inferred; use admitted runner evidence",
            "observation_only": true
        }
    })
}

fn write_run_manifest(
    writer: &mut BufWriter<File>,
    command: &str,
    panels: &[&str],
    seed_custody: &SeedArtifactCustody,
    git_custody: &GitCustody,
) -> Result<(), String> {
    let binary = std::env::current_exe().map_err(|error| error.to_string())?;
    let binary_sha256 = sha256_file(&binary)?;
    let protocol_json_path = manifest_dir().join("../protocol.json");
    let protocol_json_sha256 = sha256_file(&protocol_json_path)?;
    let (process_hwm_kib, process_rss_kib) = process_memory_kib();
    write_json_line(
        writer,
        &json!({
            "schema": "blocked-bsgs-rho-wall/run-manifest/v1",
            "event": "run_manifest",
            "study_id": STUDY_ID,
            "command": command,
            "panels": panels,
            "phase_unit": "ns",
            "pid": std::process::id(),
            "rayon_threads": rayon::current_num_threads(),
            "rayon_num_threads_env": std::env::var("RAYON_NUM_THREADS").ok(),
            "debug_assertions": cfg!(debug_assertions),
            "target_arch": std::env::consts::ARCH,
            "target_os": std::env::consts::OS,
            "package_version": env!("CARGO_PKG_VERSION"),
            "binary_path": binary.display().to_string(),
            "binary_sha256": binary_sha256,
            "process_hwm_kib": process_hwm_kib,
            "process_rss_kib": process_rss_kib,
            "source": {
                "repository_base": "a2c2c400",
                "study_git_head": git_custody.build_head,
                "build_git_dirty": git_custody.build_dirty,
                "build_git_status": git_custody.build_status,
                "runtime_git_head": git_custody.runtime_head,
                "runtime_git_status": git_custody.runtime_status,
                "runtime_git_status_scope": "all tracked/untracked paths except authorized raw timing outputs under research/blocked_bsgs_rho_wall_20261126/raw/",
                "bsgs_path": "src/cryptanalysis/bsgs_fast.rs",
                "bsgs_sha256": BSGS_SHA256,
                "rho_path": "src/cryptanalysis/prime_fast.rs",
                "rho_sha256": RHO_SHA256,
                "protocol_json_path": "research/blocked_bsgs_rho_wall_20261126/protocol.json",
                "protocol_json_sha256": protocol_json_sha256,
                "protocol_md_path": "research/blocked_bsgs_rho_wall_20261126/PROTOCOL.md",
                "protocol_md_sha256": PROTOCOL_MD_SHA256,
                "protocol_addendum_1_path": "research/blocked_bsgs_rho_wall_20261126/PROTOCOL-ADDENDUM-1.md",
                "protocol_addendum_1_sha256": PROTOCOL_ADDENDUM_1_SHA256,
                "protocol_addendum_2_path": "research/blocked_bsgs_rho_wall_20261126/PROTOCOL-ADDENDUM-2.md",
                "protocol_addendum_2_sha256": PROTOCOL_ADDENDUM_2_SHA256,
                "seed_manifest_path": "research/blocked_bsgs_rho_wall_20261126/seed-manifest.json",
                "seed_manifest_sha256": SEED_MANIFEST_SHA256,
                "cargo_lock_path": "research/blocked_bsgs_rho_wall_20261126/native/Cargo.lock",
                "cargo_lock_sha256": CARGO_LOCK_SHA256,
                "derived_seed_mapping_path": seed_custody.relative_path,
                "derived_seed_mapping_sha256": seed_custody.sha256,
                "derived_seed_mapping_bytes": seed_custody.bytes
            },
            "toolchain": {
                "build_rustc": env!("HARNESS_BUILD_RUSTC"),
                "build_cargo": env!("HARNESS_BUILD_CARGO"),
                "runtime_rustc_observation": command_version("rustc", &["--version", "--verbose"]),
                "runtime_cargo_observation": command_version("cargo", &["--version", "--verbose"])
            },
            "host": host_envelope(),
            "curve_slugs": FIXTURES.iter().map(|fixture| fixture.slug).collect::<Vec<_>>()
        }),
    )
}

#[derive(Clone, Copy)]
struct EventKey<'a> {
    curve_slug: &'a str,
    panel: &'a str,
    replicate: u64,
    item: u64,
    arm: &'a str,
    copy: u64,
}

fn intent_event(
    key: EventKey<'_>,
    target_seed: u64,
    rho_seed_value: Option<u64>,
    target_coords: Option<(u64, u64)>,
    schedule_position: u64,
    invocation_label: &str,
    scope: &str,
) -> Value {
    let (qx, qy) = target_coords
        .map(|(x, y)| (Some(x), Some(y)))
        .unwrap_or((None, None));
    json!({
        "schema": "blocked-bsgs-rho-wall/event/v1",
        "event": "intent",
        "scope": scope,
        "curve_slug": key.curve_slug,
        "panel": key.panel,
        "replicate": key.replicate,
        "item": key.item,
        "arm": key.arm,
        "copy": key.copy,
        "target_seed": target_seed,
        "rho_seed": rho_seed_value,
        "qx": qx,
        "qy": qy,
        "schedule_position": schedule_position,
        "invocation_label": invocation_label
    })
}

fn result_event(key: EventKey<'_>, scope: &str, payload: Value) -> Value {
    let status = payload["measurement"]["status"]
        .as_str()
        .unwrap_or("incorrect")
        .to_owned();
    let censored = payload["measurement"]["censored"].as_bool().unwrap_or(true);
    let error = payload["measurement"]["error"].clone();
    let qx = payload["qx"].clone();
    let qy = payload["qy"].clone();
    json!({
        "schema": "blocked-bsgs-rho-wall/event/v1",
        "event": "result",
        "scope": scope,
        "curve_slug": key.curve_slug,
        "panel": key.panel,
        "replicate": key.replicate,
        "item": key.item,
        "arm": key.arm,
        "copy": key.copy,
        "status": status,
        "censored": censored,
        "error": error,
        "qx": qx,
        "qy": qy,
        "payload": payload
    })
}

fn preflight_seed_uniqueness() -> Result<(), String> {
    let mut seen = HashSet::new();
    for fixture in FIXTURES {
        for panel in PANEL_SPECS {
            for replicate in 0..panel.replicates {
                for item in 0..panel.items {
                    for stream in ["target", "rho-walk"] {
                        if stream == "rho-walk" && !panel.has_rho {
                            continue;
                        }
                        let seed = derive_u64(fixture.slug, panel.name, replicate, item, stream);
                        if !seen.insert(seed) {
                            return Err(format!(
                                "duplicate derived seed {seed:016x} at {}/{}/{replicate}/{item}/{stream}",
                                fixture.slug, panel.name
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn push_hex_word(output: &mut String, word: u64) {
    write!(output, "{word:016x}").expect("writing to String cannot fail");
}

fn bootstrap_digest(
    fixture: Fixture,
    stream: &str,
    observations: u64,
    bootstrap_replicates: u64,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    for bootstrap_replicate in 0..bootstrap_replicates {
        for draw in 0..observations {
            let seed = derive_u64(fixture.slug, "analysis", bootstrap_replicate, draw, stream);
            digest.update((seed % observations).to_le_bytes());
        }
    }
    digest.finalize().into()
}

/// Build the lossless indexed seed artifact in memory. The CLI fixes
/// `bootstrap_replicates` to 10,000; the argument lets correctness tests
/// exercise the complete codec using a small deterministic bootstrap prefix.
pub fn build_compact_seed_manifest(bootstrap_replicates: u64) -> Result<Value, String> {
    preflight_seed_uniqueness()?;
    if bootstrap_replicates == 0 {
        return Err("bootstrap replicate count must be nonzero".into());
    }

    let mut target_seeds = String::new();
    let mut target_scalars = String::new();
    let mut rho_seeds = String::new();
    let mut schedule_codes = String::new();
    for fixture in FIXTURES {
        for panel in PANEL_SPECS {
            for replicate in 0..panel.replicates {
                for item in 0..panel.items {
                    let target = derive_u64(fixture.slug, panel.name, replicate, item, "target");
                    push_hex_word(&mut target_seeds, target);
                    push_hex_word(&mut target_scalars, 1 + target % (fixture.n - 1));
                    if panel.has_rho {
                        push_hex_word(
                            &mut rho_seeds,
                            derive_u64(fixture.slug, panel.name, replicate, item, "rho-walk"),
                        );
                    }
                }
                schedule_codes.push(match (panel.aa_schedule, replicate % 2) {
                    (false, 0) => '0',
                    (false, _) => '1',
                    (true, 0) => '2',
                    (true, _) => '3',
                });
            }
        }
    }

    let mut bootstrap_digests = String::new();
    for fixture in FIXTURES {
        for (stream, observations) in BOOTSTRAP_SPECS {
            bootstrap_digests.push_str(&hex::encode(bootstrap_digest(
                fixture,
                stream,
                observations,
                bootstrap_replicates,
            )));
        }
    }

    let mut target_offset = 0u64;
    let mut rho_offset = 0u64;
    let mut schedule_offset = 0u64;
    let mut panel_dimensions = Vec::with_capacity(PANEL_SPECS.len());
    for panel in PANEL_SPECS {
        let target_count = panel.replicates * panel.items;
        panel_dimensions.push(json!({
            "panel": panel.name,
            "replicates": panel.replicates,
            "items_per_replicate": panel.items,
            "target_offset_per_curve": target_offset,
            "target_count_per_curve": target_count,
            "rho_offset_per_curve": if panel.has_rho { Some(rho_offset) } else { None },
            "rho_count_per_curve": if panel.has_rho { Some(target_count) } else { None },
            "schedule_offset_per_curve": schedule_offset,
            "schedule_count_per_curve": panel.replicates
        }));
        target_offset += target_count;
        if panel.has_rho {
            rho_offset += target_count;
        }
        schedule_offset += panel.replicates;
    }

    let subgroup_orders_hex = FIXTURES.iter().fold(String::new(), |mut output, fixture| {
        push_hex_word(&mut output, fixture.n);
        output
    });
    let bootstrap_dimensions = BOOTSTRAP_SPECS
        .iter()
        .enumerate()
        .map(|(offset, (stream, observations))| {
            json!({
                "stream": stream,
                "observations": observations,
                "digest_offset_per_curve": offset,
                "draws_per_digest": bootstrap_replicates * observations
            })
        })
        .collect::<Vec<_>>();

    Ok(json!({
        "schema": "blocked-bsgs-rho-wall/derived-seeds/v2",
        "study_id": STUDY_ID,
        "seed_manifest_sha256": SEED_MANIFEST_SHA256,
        "master_seed_hex": MASTER_HEX,
        "encoding": {
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
        },
        "dimensions": {
            "curve_count": FIXTURES.len(),
            "curve_order": FIXTURES.iter().map(|fixture| fixture.slug).collect::<Vec<_>>(),
            "subgroup_orders_u64_hex": subgroup_orders_hex,
            "panels": panel_dimensions,
            "target_count_per_curve": target_offset,
            "rho_count_per_curve": rho_offset,
            "schedule_count_per_curve": schedule_offset,
            "bootstrap_replicates": bootstrap_replicates,
            "bootstrap_stream_count": BOOTSTRAP_SPECS.len(),
            "bootstrap_streams": bootstrap_dimensions,
            "logical_counts": {
                "target_seeds": target_offset * FIXTURES.len() as u64,
                "target_scalars": target_offset * FIXTURES.len() as u64,
                "rho_seeds": rho_offset * FIXTURES.len() as u64,
                "schedule_codes": schedule_offset * FIXTURES.len() as u64,
                "bootstrap_digests": BOOTSTRAP_SPECS.len() * FIXTURES.len(),
                "superseded_jsonl_rows": (target_offset + rho_offset + schedule_offset)
                    * FIXTURES.len() as u64
                    + (BOOTSTRAP_SPECS.len() * FIXTURES.len()) as u64
            }
        },
        "schedule_codebook": {
            "0": ["bsgs", "rho"],
            "1": ["rho", "bsgs"],
            "2": ["A", "A-copy"],
            "3": ["A-copy", "A"]
        },
        "data": {
            "target_derived_u64_hex": target_seeds,
            "target_scalar_u64_hex": target_scalars,
            "rho_derived_u64_hex": rho_seeds,
            "schedule_codes": schedule_codes,
            "bootstrap_selection_sha256_hex": bootstrap_digests
        }
    }))
}

pub fn canonical_compact_seed_bytes(bootstrap_replicates: u64) -> Result<Vec<u8>, String> {
    let artifact = build_compact_seed_manifest(bootstrap_replicates)?;
    let mut encoded = serde_json::to_vec(&artifact).map_err(|error| error.to_string())?;
    encoded.push(b'\n');
    if encoded.len() >= COMPACT_SEED_LIMIT_BYTES {
        return Err(format!(
            "compact seed artifact is {} bytes; limit is strictly below {}",
            encoded.len(),
            COMPACT_SEED_LIMIT_BYTES
        ));
    }
    Ok(encoded)
}

pub fn verify_compact_seed_artifact_bytes(
    actual: &[u8],
    bootstrap_replicates: u64,
) -> Result<String, String> {
    let canonical = canonical_compact_seed_bytes(bootstrap_replicates)?;
    if actual != canonical {
        let mismatch = actual
            .iter()
            .zip(&canonical)
            .position(|(left, right)| left != right)
            .unwrap_or_else(|| actual.len().min(canonical.len()));
        return Err(format!(
            "derived-seeds.json is noncanonical at byte {mismatch}: actual length {}, expected {}",
            actual.len(),
            canonical.len()
        ));
    }
    Ok(hex::encode(Sha256::digest(actual)))
}

#[derive(Clone, Debug)]
struct SeedArtifactCustody {
    relative_path: &'static str,
    sha256: String,
    bytes: u64,
}

fn verify_committed_seed_artifact() -> Result<SeedArtifactCustody, String> {
    const RELATIVE_PATH: &str = "research/blocked_bsgs_rho_wall_20261126/derived-seeds.json";
    let path = manifest_dir().join("../derived-seeds.json");
    let metadata = fs::metadata(&path)
        .map_err(|error| format!("required seed artifact {}: {error}", path.display()))?;
    if metadata.len() >= COMPACT_SEED_LIMIT_BYTES as u64 {
        return Err(format!(
            "seed artifact {} is {} bytes; must be below {}",
            path.display(),
            metadata.len(),
            COMPACT_SEED_LIMIT_BYTES
        ));
    }
    let actual = fs::read(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let sha256 = verify_compact_seed_artifact_bytes(&actual, 10_000)?;

    let protocol_path = manifest_dir().join("../protocol.json");
    let protocol_file = File::open(&protocol_path)
        .map_err(|error| format!("open {}: {error}", protocol_path.display()))?;
    let protocol: Value = serde_json::from_reader(protocol_file)
        .map_err(|error| format!("parse {}: {error}", protocol_path.display()))?;
    verify_machine_protocol_value(&protocol)?;
    let anchors = &protocol["extensions"]["protocol_anchors"];
    for (field, expected) in [
        ("human_protocol_sha256", PROTOCOL_MD_SHA256),
        ("cpu_envelope_addendum_sha256", PROTOCOL_ADDENDUM_1_SHA256),
        ("seed_custody_addendum_sha256", PROTOCOL_ADDENDUM_2_SHA256),
        ("seed_manifest_sha256", SEED_MANIFEST_SHA256),
        ("bsgs_source_sha256", BSGS_SHA256),
        ("rho_source_sha256", RHO_SHA256),
    ] {
        if anchors[field].as_str() != Some(expected) {
            return Err(format!("protocol.json anchor mismatch for {field}"));
        }
    }
    let custody = &protocol["extensions"]["seed_custody"];
    if custody["path"] != RELATIVE_PATH
        || custody["schema"] != "blocked-bsgs-rho-wall/derived-seeds/v2"
        || custody["logical_records"] != 19_680
        || custody["target_derived_u64_values"] != 8_340
        || custody["target_scalars"] != 8_340
        || custody["rho_walk_derived_u64_values"] != 8_020
        || custody["schedule_codes"] != 3_300
        || custody["bootstrap_stream_digests"] != 20
    {
        return Err("protocol.json seed-custody dimensions mismatch".into());
    }
    let pinned = custody["canonical_file_sha256"].as_str().ok_or_else(|| {
        "protocol.json does not independently hash-pin derived-seeds.json; timing refused"
            .to_owned()
    })?;
    if pinned != sha256 {
        return Err(format!(
            "derived seed artifact hash {sha256} does not match protocol pin {pinned}"
        ));
    }
    Ok(SeedArtifactCustody {
        relative_path: RELATIVE_PATH,
        sha256,
        bytes: metadata.len(),
    })
}

pub fn emit_seeds() -> Result<PathBuf, String> {
    verify_source_hashes()?;
    let encoded = canonical_compact_seed_bytes(10_000)?;
    let path = manifest_dir().join("../derived-seeds.json");
    let mut writer = fresh_writer(&path)?;
    writer
        .write_all(&encoded)
        .map_err(|error| error.to_string())?;
    writer.flush().map_err(|error| error.to_string())?;
    Ok(path)
}

fn run_warmups(fixture: Fixture) -> Result<(), String> {
    for replicate in 0..4u64 {
        let (_, bc, bq, pc, pq) = target_pair(fixture, "warmup", replicate, 0)?;
        let seed = rho_seed(fixture, "warmup", replicate, 0);
        let (b, r) = if replicate % 2 == 0 {
            (bsgs_run(fixture, bc, bq), rho_run(fixture, &pc, pq, seed))
        } else {
            let r = rho_run(fixture, &pc, pq, seed);
            let b = bsgs_run(fixture, bc, bq);
            (b, r)
        };
        if b["correct"] != true || r["correct"] != true {
            return Err(format!(
                "warmup failed for {} replicate {replicate}",
                fixture.slug
            ));
        }
    }
    Ok(())
}

pub fn run_cold() -> Result<PathBuf, String> {
    verify_source_hashes()?;
    ensure_timing_environment()?;
    let git_custody = verify_git_custody()?;
    let seed_custody = verify_committed_seed_artifact()?;
    let path = manifest_dir().join("../raw/cold.jsonl");
    let mut writer = fresh_writer(&path)?;
    write_run_manifest(
        &mut writer,
        "run-cold",
        &["cold-paired"],
        &seed_custody,
        &git_custody,
    )?;
    for fixture in FIXTURES {
        run_warmups(fixture)?;
        for replicate in 0..512u64 {
            let (secret, bc, bq, pc, pq) = target_pair(fixture, "cold-paired", replicate, 0)?;
            let target_seed_value = derive_u64(fixture.slug, "cold-paired", replicate, 0, "target");
            let seed = rho_seed(fixture, "cold-paired", replicate, 0);
            let target_coords = pc.canonical(pq);
            let order = if replicate % 2 == 0 {
                ["bsgs", "rho"]
            } else {
                ["rho", "bsgs"]
            };
            let mut bc = Some(bc);
            let mut bq = Some(bq);
            let pc = Some(pc);
            let mut pq = Some(pq);
            for (schedule_position, arm) in order.into_iter().enumerate() {
                let key = EventKey {
                    curve_slug: fixture.slug,
                    panel: "cold-paired",
                    replicate,
                    item: 0,
                    arm,
                    copy: 0,
                };
                write_json_line(
                    &mut writer,
                    &intent_event(
                        key,
                        target_seed_value,
                        (arm == "rho").then_some(seed),
                        Some(target_coords),
                        schedule_position as u64,
                        arm,
                        "arm",
                    ),
                )?;
                let payload = if arm == "bsgs" {
                    bsgs_run(
                        fixture,
                        bc.take().expect("one BSGS run"),
                        bq.take().unwrap(),
                    )
                } else {
                    rho_run(
                        fixture,
                        pc.as_ref().expect("one rho run"),
                        pq.take().unwrap(),
                        seed,
                    )
                };
                write_json_line(
                    &mut writer,
                    &result_event(
                        key,
                        "arm",
                        json!({
                            "target_scalar": secret,
                            "qx": target_coords.0,
                            "qy": target_coords.1,
                            "measurement": payload
                        }),
                    ),
                )?;
            }
        }
    }
    Ok(path)
}

pub fn run_aa() -> Result<PathBuf, String> {
    verify_source_hashes()?;
    ensure_timing_environment()?;
    let git_custody = verify_git_custody()?;
    let seed_custody = verify_committed_seed_artifact()?;
    let path = manifest_dir().join("../raw/aa.jsonl");
    let mut writer = fresh_writer(&path)?;
    write_run_manifest(
        &mut writer,
        "run-aa",
        &["bsgs-aa", "rho-aa"],
        &seed_custody,
        &git_custody,
    )?;
    for fixture in FIXTURES {
        run_warmups(fixture)?;
        for replicate in 0..64u64 {
            for (panel, arm) in [("bsgs-aa", "bsgs"), ("rho-aa", "rho")] {
                let target_seed_value = derive_u64(fixture.slug, panel, replicate, 0, "target");
                let seed = rho_seed(fixture, panel, replicate, 0);
                let (secret, bc, bq, pc, pq) = target_pair(fixture, panel, replicate, 0)?;
                let target_coords = pc.canonical(pq);
                let labels = if replicate % 2 == 0 {
                    ["A", "A-copy"]
                } else {
                    ["A-copy", "A"]
                };
                for (schedule_position, invocation_label) in labels.into_iter().enumerate() {
                    let copy = if invocation_label == "A" { 0 } else { 1 };
                    let key = EventKey {
                        curve_slug: fixture.slug,
                        panel,
                        replicate,
                        item: 0,
                        arm,
                        copy,
                    };
                    write_json_line(
                        &mut writer,
                        &intent_event(
                            key,
                            target_seed_value,
                            (arm == "rho").then_some(seed),
                            Some(target_coords),
                            schedule_position as u64,
                            invocation_label,
                            "arm",
                        ),
                    )?;
                    let measurement = if arm == "bsgs" {
                        bsgs_run(fixture, bc, bq)
                    } else {
                        rho_run(fixture, &pc, pq, seed)
                    };
                    write_json_line(
                        &mut writer,
                        &result_event(
                            key,
                            "arm",
                            json!({
                                "target_scalar": secret,
                                "qx": target_coords.0,
                                "qy": target_coords.1,
                                "measurement": measurement
                            }),
                        ),
                    )?;
                }
            }
        }
    }
    Ok(path)
}

fn digest_logs(logs: &[Option<u64>]) -> String {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for log in logs {
        h = (h ^ log.unwrap_or(u64::MAX)).wrapping_mul(0x100_0000_01b3);
    }
    format!("{h:016x}")
}

#[derive(Clone, Copy)]
struct BsgsItemMeasurement {
    log: Option<u64>,
    correct: bool,
    search_ns: u64,
    evaluator_verify_ns: u64,
    unattributed_ns: u64,
    total_ns: u64,
}

fn amortized_bsgs(
    fixture: Fixture,
    bc: bsgs_fast::FastCurve,
    bqs: &[bsgs_fast::FastPoint],
) -> (Value, Vec<Value>) {
    let plan = BsgsFastPlan::new(fixture.n, 0, true, 1, 8).with_kernel(StepKernel::Paired, 32);
    let table_bytes = plan.table_bytes();
    let plan_record = bsgs_plan_value(&plan);
    let mut logs = Vec::with_capacity(bqs.len());
    let mut measurements = Vec::with_capacity(bqs.len());
    let total_start = Instant::now();
    let build_start = Instant::now();
    let mut solver = BsgsFast::build(bc, plan);
    let build_ns = build_start.elapsed().as_nanos() as u64;
    let mut search_ns_sum = 0u64;
    let mut verify_ns_sum = 0u64;
    for &q in bqs {
        let item_start = Instant::now();
        let search_start = Instant::now();
        let log = solver.solve(&q);
        let search_ns = search_start.elapsed().as_nanos() as u64;
        let verify_start = Instant::now();
        let correct = log.map(|k| bc.scalar_mul(bc.g, k) == q).unwrap_or(false);
        let verify_ns = verify_start.elapsed().as_nanos() as u64;
        let item_total_ns = item_start.elapsed().as_nanos() as u64;
        let item_attributed_ns = search_ns.saturating_add(verify_ns);
        let item_unattributed_ns = item_total_ns
            .checked_sub(item_attributed_ns)
            .expect("BSGS item phase timers must be enclosed by item timer");
        search_ns_sum = search_ns_sum.saturating_add(search_ns);
        verify_ns_sum = verify_ns_sum.saturating_add(verify_ns);
        logs.push(log);
        measurements.push(BsgsItemMeasurement {
            log,
            correct,
            search_ns,
            evaluator_verify_ns: verify_ns,
            unattributed_ns: item_unattributed_ns,
            total_ns: item_total_ns,
        });
    }
    let stats = solver.stats();
    let drop_start = Instant::now();
    drop(solver);
    let drop_ns = drop_start.elapsed().as_nanos() as u64;
    let total_ns = total_start.elapsed().as_nanos() as u64;
    let attributed_ns = build_ns
        .saturating_add(search_ns_sum)
        .saturating_add(verify_ns_sum)
        .saturating_add(drop_ns);
    let unattributed_ns = total_ns
        .checked_sub(attributed_ns)
        .expect("BSGS batch phase timers must be enclosed by batch timer");
    let correct = measurements.iter().all(|item| item.correct);
    let any_incorrect = measurements
        .iter()
        .any(|item| item.log.is_some() && !item.correct);
    let any_censored = measurements.iter().any(|item| item.log.is_none());
    let (status, error) = if any_incorrect {
        ("incorrect", Some("evaluator_mismatch"))
    } else if any_censored {
        ("solver_none_or_cap", Some("solver_none_or_cap"))
    } else {
        ("ok", None)
    };
    let (process_hwm_kib, process_rss_kib) = process_memory_kib();
    let items = measurements
        .iter()
        .map(|item| {
            let (status, censored, error) = outcome(item.log, item.correct);
            json!({
                "status": status,
                "censored": censored,
                "error": error,
                "log": item.log,
                "correct": item.correct,
                "search_ns": item.search_ns,
                "evaluator_verify_ns": item.evaluator_verify_ns,
                "unattributed_ns": item.unattributed_ns,
                "total_ns": item.total_ns,
                "phase_sum_ns": item.search_ns
                    .saturating_add(item.evaluator_verify_ns)
                    .saturating_add(item.unattributed_ns)
            })
        })
        .collect();
    let batch = json!({
        "status": status,
        "censored": any_censored,
        "error": error,
        "correct": correct,
        "logs": logs,
        "answers_digest": digest_logs(&logs),
        "build_ns": build_ns,
        "search_ns_sum": search_ns_sum,
        "evaluator_verify_ns_sum": verify_ns_sum,
        "drop_ns": drop_ns,
        "unattributed_ns": unattributed_ns,
        "total_ns": total_ns,
        "phase_sum_ns": attributed_ns.saturating_add(unattributed_ns),
        "plan": plan_record,
        "table_bytes": table_bytes,
        "process_hwm_kib": process_hwm_kib,
        "process_rss_kib": process_rss_kib,
        "table_entries": stats.table_entries,
        "baby_steps": stats.baby_steps,
        "giant_steps": stats.giant_steps,
        "candidates": stats.candidates,
        "false_candidates": stats.false_candidates
    });
    (batch, items)
}

fn amortized_rho(
    fixture: Fixture,
    pc: &prime_fast::FastCurve,
    pqs: &[prime_fast::Pt],
    replicate: u64,
) -> (Value, Vec<Value>) {
    let params = fixture.params();
    let g = pc
        .from_textbook(&params.generator())
        .expect("fixed generator is affine");
    let configs = (0..pqs.len())
        .map(|item| {
            fixed_rho_config(
                fixture,
                rho_seed(fixture, "amortized-B64", replicate, item as u64),
            )
        })
        .collect::<Vec<_>>();
    let mut measurements = Vec::with_capacity(pqs.len());
    let total_start = Instant::now();
    for (&q, config) in pqs.iter().zip(configs) {
        measurements.push(measure_rho_with_config(pc, g, q, config));
    }
    let total_ns = total_start.elapsed().as_nanos() as u64;
    let (process_hwm_kib, process_rss_kib) = process_memory_kib();

    let mut solver_ns_sum = 0u64;
    let mut verify_ns_sum = 0u64;
    let mut total_steps = 0u64;
    let mut distinguished_points = 0u64;
    let mut censored_count = 0u64;
    let mut incorrect_count = 0u64;
    let mut item_total_ns_sum = 0u64;
    let mut drop_ns_sum = 0u64;
    let mut unattributed_ns_sum = 0u64;
    let mut logs = Vec::with_capacity(measurements.len());
    for measurement in &measurements {
        solver_ns_sum = solver_ns_sum.saturating_add(measurement.solver_ns);
        verify_ns_sum = verify_ns_sum.saturating_add(measurement.evaluator_verify_ns);
        item_total_ns_sum = item_total_ns_sum.saturating_add(measurement.total_ns);
        drop_ns_sum = drop_ns_sum.saturating_add(measurement.drop_ns);
        unattributed_ns_sum = unattributed_ns_sum.saturating_add(measurement.unattributed_ns);
        let log = measurement.result.map(|result| result.log);
        if let Some(result) = measurement.result {
            total_steps = total_steps.saturating_add(result.total_steps);
            distinguished_points = distinguished_points.saturating_add(result.distinguished_points);
        }
        if log.is_none() {
            censored_count += 1;
        } else if !measurement.correct {
            incorrect_count += 1;
        }
        logs.push(log);
    }
    let batch_unattributed_ns = total_ns
        .checked_sub(item_total_ns_sum)
        .expect("rho item timers must be enclosed by batch timer");
    let correct = censored_count == 0 && incorrect_count == 0;
    let (status, error) = if incorrect_count != 0 {
        ("incorrect", Some("evaluator_mismatch"))
    } else if censored_count != 0 {
        ("solver_none_or_cap", Some("solver_none_or_cap"))
    } else {
        ("ok", None)
    };
    let rows = measurements
        .iter()
        .map(|measurement| rho_measurement_value(measurement, process_hwm_kib, process_rss_kib))
        .collect();
    let batch = json!({
        "status": status,
        "correct": correct,
        "censored": censored_count != 0,
        "error": error,
        "censored_count": censored_count,
        "incorrect_count": incorrect_count,
        "logs": logs,
        "answers_digest": digest_logs(&logs),
        "solver_ns_sum": solver_ns_sum,
        "evaluator_verify_ns_sum": verify_ns_sum,
        "drop_ns_sum": drop_ns_sum,
        "item_unattributed_ns_sum": unattributed_ns_sum,
        "item_total_ns_sum": item_total_ns_sum,
        "batch_unattributed_ns": batch_unattributed_ns,
        "total_ns": total_ns,
        "phase_sum_ns": item_total_ns_sum.saturating_add(batch_unattributed_ns),
        "process_hwm_kib": process_hwm_kib,
        "process_rss_kib": process_rss_kib,
        "total_steps": total_steps,
        "distinguished_points": distinguished_points
    });
    (batch, rows)
}

pub fn run_amortized() -> Result<PathBuf, String> {
    verify_source_hashes()?;
    ensure_timing_environment()?;
    let git_custody = verify_git_custody()?;
    let seed_custody = verify_committed_seed_artifact()?;
    let path = manifest_dir().join("../raw/amortized-B64.jsonl");
    let mut writer = fresh_writer(&path)?;
    write_run_manifest(
        &mut writer,
        "run-amortized",
        &["amortized-B64"],
        &seed_custody,
        &git_custody,
    )?;
    for fixture in FIXTURES {
        run_warmups(fixture)?;
        for replicate in 0..16u64 {
            let params = fixture.params();
            let bc = bsgs_fast::FastCurve::from_params(&params).expect("fixed BSGS curve");
            let pc = prime_fast::FastCurve::from_params(&params).expect("fixed rho curve");
            let pg = pc
                .from_textbook(&params.generator())
                .expect("fixed generator");
            let mut bqs = Vec::with_capacity(64);
            let mut pqs = Vec::with_capacity(64);
            let mut secrets = Vec::with_capacity(64);
            let mut target_coords = Vec::with_capacity(64);
            for item in 0..64u64 {
                let k = target_scalar(fixture, "amortized-B64", replicate, item);
                let bq = bc.scalar_mul(bc.g, k);
                let pq = pc.mul(Some(pg), k).expect("nonzero target");
                if bc.lower(bq) != pc.to_textbook(Some(pq)) {
                    return Err(format!(
                        "batch target mismatch {} {replicate}/{item}",
                        fixture.slug
                    ));
                }
                secrets.push(k);
                bqs.push(bq);
                pqs.push(pq);
                target_coords.push(pc.canonical(pq));
            }
            let mut target_points_digest = Sha256::new();
            for &(qx, qy) in &target_coords {
                target_points_digest.update(qx.to_le_bytes());
                target_points_digest.update(qy.to_le_bytes());
            }
            let target_points_sha256 = hex::encode(target_points_digest.finalize());
            let first_target_seed =
                derive_u64(fixture.slug, "amortized-B64", replicate, 0, "target");
            let order = if replicate % 2 == 0 {
                ["bsgs", "rho"]
            } else {
                ["rho", "bsgs"]
            };
            let mut bc = Some(bc);
            for (schedule_position, arm) in order.into_iter().enumerate() {
                let batch_key = EventKey {
                    curve_slug: fixture.slug,
                    panel: "amortized-B64",
                    replicate,
                    item: 64,
                    arm,
                    copy: 0,
                };
                let mut intent = intent_event(
                    batch_key,
                    first_target_seed,
                    (arm == "rho").then_some(rho_seed(fixture, "amortized-B64", replicate, 0)),
                    None,
                    schedule_position as u64,
                    arm,
                    "batch",
                );
                intent["target_seed_identity"] = json!({
                    "mapping": "derived-seeds.json",
                    "panel": "amortized-B64",
                    "replicate": replicate,
                    "items": "0..63",
                    "target_points_sha256": target_points_sha256,
                    "target_points_encoding": "ordered qx-u64-le || qy-u64-le"
                });
                intent["targets"] = Value::Array(
                    target_coords
                        .iter()
                        .enumerate()
                        .map(|(item, &(qx, qy))| {
                            json!({
                                "item": item,
                                "target_seed": derive_u64(
                                    fixture.slug,
                                    "amortized-B64",
                                    replicate,
                                    item as u64,
                                    "target"
                                ),
                                "rho_seed": if arm == "rho" {
                                    Some(rho_seed(
                                        fixture,
                                        "amortized-B64",
                                        replicate,
                                        item as u64
                                    ))
                                } else {
                                    None
                                },
                                "qx": qx,
                                "qy": qy
                            })
                        })
                        .collect(),
                );
                write_json_line(&mut writer, &intent)?;
                let (batch, items) = if arm == "bsgs" {
                    amortized_bsgs(fixture, bc.take().expect("one BSGS batch"), &bqs)
                } else {
                    amortized_rho(fixture, &pc, &pqs, replicate)
                };
                for (item, payload) in items.into_iter().enumerate() {
                    let item_key = EventKey {
                        item: item as u64,
                        ..batch_key
                    };
                    write_json_line(
                        &mut writer,
                        &result_event(
                            item_key,
                            "item",
                            json!({
                                "target_scalar": secrets[item],
                                "qx": target_coords[item].0,
                                "qy": target_coords[item].1,
                                "target_seed": derive_u64(
                                    fixture.slug,
                                    "amortized-B64",
                                    replicate,
                                    item as u64,
                                    "target"
                                ),
                                "rho_seed": if arm == "rho" {
                                    Some(rho_seed(
                                        fixture,
                                        "amortized-B64",
                                        replicate,
                                        item as u64
                                    ))
                                } else {
                                    None
                                },
                                "measurement": payload
                            }),
                        ),
                    )?;
                }
                write_json_line(
                    &mut writer,
                    &result_event(
                        batch_key,
                        "batch",
                        json!({
                            "batch_size": 64,
                            "target_points_sha256": target_points_sha256,
                            "measurement": batch
                        }),
                    ),
                )?;
            }
        }
    }
    Ok(path)
}

pub fn dispatch(command: &str) -> Result<Value, String> {
    match command {
        "verify-fixtures" => verify_fixtures(),
        "emit-seeds" => emit_seeds().map(|path| json!({"written": path})),
        "run-cold" => run_cold().map(|path| json!({"written": path})),
        "run-aa" => run_aa().map(|path| json!({"written": path})),
        "run-amortized" => run_amortized().map(|path| json!({"written": path})),
        _ => Err(
            "usage: blocked-bsgs-rho-wall <verify-fixtures|emit-seeds|run-cold|run-aa|run-amortized>"
                .to_owned(),
        ),
    }
}
