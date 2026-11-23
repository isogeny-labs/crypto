//! Driver for the N4 plane-quartic strong-rho control.
//!
//! Build: `rustc --edition 2021 -O main.rs -o ghs_strong_rho`

#![allow(dead_code, static_mut_refs)]

#[path = "../quartic_n4_20261019/common.rs"]
mod common;
#[path = "../quartic_n4_20261019/ghs.rs"]
mod ghs;
#[path = "../../src/cryptanalysis/plane_quartic_pic.rs"]
mod pic;

mod ic2 {
    use crate::common::{bump_ladd, bump_lmul};

    #[inline(always)]
    pub fn la2(a: u64, b: u64, ell: u64) -> u64 {
        bump_ladd(1);
        let (s, overflow) = a.overflowing_add(b);
        if overflow || s >= ell {
            s.wrapping_sub(ell)
        } else {
            s
        }
    }

    #[inline(always)]
    pub fn ls2(a: u64, b: u64, ell: u64) -> u64 {
        bump_ladd(1);
        if a >= b {
            a - b
        } else {
            a + (ell - b)
        }
    }

    #[inline(always)]
    pub fn lm2(a: u64, b: u64, ell: u64) -> u64 {
        bump_lmul(1);
        ((a as u128 * b as u128) % ell as u128) as u64
    }
}

#[path = "../quartic_n4v2_20261020/rho2.rs"]
mod rho2;
mod strong_rho;

use common::*;
use ghs::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use strong_rho::{
    hex_digest, sha256, strong_rho, strong_rho_bare, strong_rho_parallel_check,
    strong_rho_uncounted, FieldCounts, NativeCounts, ParallelCheckOpts, ParallelDpRecord,
    StrongOpts,
};

#[derive(Clone, Copy)]
enum CountMode {
    Counted,
    FieldCounterFree,
    Bare,
}

impl CountMode {
    fn field_authoritative(self) -> bool {
        matches!(self, Self::Counted)
    }

    fn native_authoritative(self) -> bool {
        matches!(self, Self::Counted | Self::FieldCounterFree)
    }
}

#[derive(Clone, Copy)]
enum SetupMode {
    BatchedJumps,
    BatchedJumpsAndWalkers,
    Legacy,
}

#[derive(Clone, Copy)]
struct ArmMode {
    width: usize,
    count_mode: CountMode,
    setup_mode: SetupMode,
    use_mul5: bool,
}

fn parse_strong_arm(arm: &str) -> ArmMode {
    let (rest, count_mode) = if let Some(rest) = arm.strip_prefix("strong-b") {
        (rest, CountMode::Counted)
    } else if let Some(rest) = arm.strip_prefix("fast-b") {
        (rest, CountMode::FieldCounterFree)
    } else if let Some(rest) = arm.strip_prefix("bare-b") {
        (rest, CountMode::Bare)
    } else {
        panic!("unknown arm {arm}")
    };
    let (width_text, setup_mode, use_mul5) = if let Some(width) = rest.strip_suffix("-legacy-setup")
    {
        (width, SetupMode::Legacy, false)
    } else if let Some(width) = rest.strip_suffix("-mul5-walker-table") {
        (width, SetupMode::BatchedJumpsAndWalkers, true)
    } else if let Some(width) = rest.strip_suffix("-mul6-walker-table") {
        (width, SetupMode::BatchedJumpsAndWalkers, false)
    } else if let Some(width) = rest.strip_suffix("-walker-table") {
        (width, SetupMode::BatchedJumpsAndWalkers, false)
    } else if let Some(width) = rest.strip_suffix("-mul5") {
        (width, SetupMode::BatchedJumps, true)
    } else if let Some(width) = rest.strip_suffix("-mul6") {
        (width, SetupMode::BatchedJumps, false)
    } else {
        (rest, SetupMode::BatchedJumps, false)
    };
    let width: usize = width_text
        .parse()
        .unwrap_or_else(|_| panic!("invalid batch width in arm {arm}"));
    assert!(width > 0, "batch width must be positive in arm {arm}");
    ArmMode {
        width,
        count_mode,
        setup_mode,
        use_mul5,
    }
}

fn selftest_arm_parser() {
    let counted = parse_strong_arm("strong-b8");
    assert_eq!(counted.width, 8);
    assert!(matches!(counted.count_mode, CountMode::Counted));
    assert!(matches!(counted.setup_mode, SetupMode::BatchedJumps));
    assert!(!counted.use_mul5);
    assert!(counted.count_mode.field_authoritative());
    assert!(counted.count_mode.native_authoritative());

    let field_counter_free = parse_strong_arm("fast-b8");
    assert_eq!(field_counter_free.width, 8);
    assert!(matches!(
        field_counter_free.count_mode,
        CountMode::FieldCounterFree
    ));
    assert!(matches!(
        field_counter_free.setup_mode,
        SetupMode::BatchedJumps
    ));
    assert!(!field_counter_free.use_mul5);
    assert!(!field_counter_free.count_mode.field_authoritative());
    assert!(field_counter_free.count_mode.native_authoritative());

    let plain = parse_strong_arm("bare-b8");
    assert_eq!(plain.width, 8);
    assert!(matches!(plain.count_mode, CountMode::Bare));
    assert!(matches!(plain.setup_mode, SetupMode::BatchedJumps));
    assert!(!plain.use_mul5);
    assert!(!plain.count_mode.field_authoritative());
    assert!(!plain.count_mode.native_authoritative());

    let combined = parse_strong_arm("bare-b16-mul5-walker-table");
    assert_eq!(combined.width, 16);
    assert!(matches!(combined.count_mode, CountMode::Bare));
    assert!(matches!(
        combined.setup_mode,
        SetupMode::BatchedJumpsAndWalkers
    ));
    assert!(combined.use_mul5);

    let explicit_control = parse_strong_arm("strong-b8-mul6");
    assert_eq!(explicit_control.width, 8);
    assert!(matches!(
        explicit_control.setup_mode,
        SetupMode::BatchedJumps
    ));
    assert!(!explicit_control.use_mul5);

    let explicit_control_table = parse_strong_arm("strong-b8-mul6-walker-table");
    assert_eq!(explicit_control_table.width, 8);
    assert!(matches!(
        explicit_control_table.setup_mode,
        SetupMode::BatchedJumpsAndWalkers
    ));
    assert!(!explicit_control_table.use_mul5);

    let legacy = parse_strong_arm("bare-b4-legacy-setup");
    assert_eq!(legacy.width, 4);
    assert!(matches!(legacy.count_mode, CountMode::Bare));
    assert!(matches!(legacy.setup_mode, SetupMode::Legacy));
    assert!(!legacy.use_mul5);
    eprintln!("selftest: bare arm parser and authority split match schema");
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

#[derive(Clone, Debug)]
struct CalibrationSample {
    mul_s: f64,
    inv_s: f64,
    lmul_s: f64,
    r_inv: f64,
    r_lmul: f64,
}

#[derive(Clone, Debug)]
struct CalibrationResult {
    r_inv: f64,
    r_lmul: f64,
    mul_ns_median: f64,
    mul_ns_min: f64,
    samples: Vec<CalibrationSample>,
}

/// Interleaved same-host calibration for the two priced scalar operations.
/// This is a conversion factor, not a throughput result.
fn calibration_full(q: u64, ell: u64, triples: usize) -> CalibrationResult {
    use std::hint::black_box;
    use std::time::Instant;
    assert!(triples > 0);
    let (mut x, mut z, mut w) = (12_345 % q, 3u64, 12_345 % ell);
    let (y, yl) = (54_321 % q, 54_321 % ell);
    let mut ri = Vec::with_capacity(triples);
    let mut rl = Vec::with_capacity(triples);
    let mut tm = Vec::with_capacity(triples);
    let mut samples = Vec::with_capacity(triples);
    for _ in 0..triples {
        let t = Instant::now();
        for _ in 0..20_000 {
            // Match the frozen N4-v2 calibration kernel exactly.  q < 2^31,
            // so this product cannot overflow u64.
            x = black_box(x * y % q);
            x ^= 1;
        }
        let mul_s = t.elapsed().as_secs_f64() / 20_000.0;
        let t = Instant::now();
        for _ in 0..2_000 {
            z = black_box(inv_mod_u64(z % (q - 1) + 1, q));
        }
        let inv_s = t.elapsed().as_secs_f64() / 2_000.0;
        let t = Instant::now();
        for _ in 0..20_000 {
            w = black_box(((w as u128 * yl as u128) % ell as u128) as u64);
            w ^= 1;
        }
        let lmul_s = t.elapsed().as_secs_f64() / 20_000.0;
        ri.push(inv_s / mul_s);
        rl.push(lmul_s / mul_s);
        tm.push(mul_s * 1e9);
        samples.push(CalibrationSample {
            mul_s,
            inv_s,
            lmul_s,
            r_inv: inv_s / mul_s,
            r_lmul: lmul_s / mul_s,
        });
    }
    black_box((x, z, w));
    let min_ns = tm.iter().copied().fold(f64::INFINITY, f64::min);
    CalibrationResult {
        r_inv: median(&mut ri),
        r_lmul: median(&mut rl),
        mul_ns_median: median(&mut tm),
        mul_ns_min: min_ns,
        samples,
    }
}

fn calibration(q: u64, ell: u64, triples: usize) -> (f64, f64, f64, f64) {
    let result = calibration_full(q, ell, triples);
    (
        result.r_inv,
        result.r_lmul,
        result.mul_ns_median,
        result.mul_ns_min,
    )
}

fn target_stream(inst: &Inst, seed: u64) -> (u64, Rng) {
    // Match N4-v2's preamble so paired runs use its public synthetic targets.
    let mut rng = Rng::new(seed ^ 0x1234);
    let p = inst.ec.rand_point(&mut rng);
    let p2 = inst.ec.rand_point(&mut rng);
    let mut acc = p;
    for _ in 0..100 {
        acc = inst.ec.add(&acc, &p2);
    }
    std::hint::black_box(acc);
    let ic_target = rng.below(inst.ell - 1) + 1;
    (ic_target, rng)
}

fn target_at(inst: &Inst, seed: u64, index: usize, policy: &str) -> u64 {
    assert!(matches!(policy, "ic-repeat" | "sequence"));
    let (ic_target, mut rng) = target_stream(inst, seed);
    if policy == "ic-repeat" || index == 0 {
        return ic_target;
    }
    let mut value = ic_target;
    for _ in 1..=index {
        value = rng.below(inst.ell - 1) + 1;
    }
    value
}

fn named_args(args: &[String]) -> Result<BTreeMap<String, String>, String> {
    let mut out = BTreeMap::new();
    let mut i = 0;
    while i < args.len() {
        let Some(key) = args[i].strip_prefix("--") else {
            return Err(format!("expected --name, got {}", args[i]));
        };
        if i + 1 == args.len() || args[i + 1].starts_with("--") {
            return Err(format!("missing value for --{key}"));
        }
        if out.insert(key.to_owned(), args[i + 1].clone()).is_some() {
            return Err(format!("duplicate --{key}"));
        }
        i += 2;
    }
    Ok(out)
}

fn required<'a>(args: &'a BTreeMap<String, String>, name: &str) -> Result<&'a str, String> {
    args.get(name)
        .map(String::as_str)
        .ok_or_else(|| format!("missing --{name}"))
}

fn parse_num<T: std::str::FromStr>(
    args: &BTreeMap<String, String>,
    name: &str,
) -> Result<T, String> {
    required(args, name)?
        .parse()
        .map_err(|_| format!("invalid --{name}"))
}

fn expand_cpu_list(text: &str) -> Result<BTreeSet<usize>, String> {
    let mut out = BTreeSet::new();
    for part in text.split(',') {
        if part.is_empty() {
            return Err("empty CPU-list element".into());
        }
        if let Some((lo, hi)) = part.split_once('-') {
            let lo: usize = lo.parse().map_err(|_| "invalid CPU range")?;
            let hi: usize = hi.parse().map_err(|_| "invalid CPU range")?;
            if lo > hi {
                return Err("descending CPU range".into());
            }
            out.extend(lo..=hi);
        } else {
            out.insert(part.parse().map_err(|_| "invalid CPU")?);
        }
    }
    Ok(out)
}

fn process_allowed_cpu_text() -> Result<String, String> {
    let status = fs::read_to_string("/proc/self/status")
        .map_err(|e| format!("read /proc/self/status: {e}"))?;
    status
        .lines()
        .find_map(|line| line.strip_prefix("Cpus_allowed_list:\t"))
        .map(str::trim)
        .map(str::to_owned)
        .ok_or_else(|| "Cpus_allowed_list missing".into())
}

fn validate_process_placement(cpu_text: &str, numa_node: usize) -> Result<String, String> {
    let requested = expand_cpu_list(cpu_text)?;
    if requested.is_empty() {
        return Err("worker CPU list is empty".into());
    }
    let observed_text = process_allowed_cpu_text()?;
    let observed = expand_cpu_list(&observed_text)?;
    if !requested.is_subset(&observed) {
        return Err(format!(
            "requested CPUs {cpu_text} are not a subset of process affinity {observed_text}"
        ));
    }
    let node_path = format!("/sys/devices/system/node/node{numa_node}/cpulist");
    let node_text = fs::read_to_string(&node_path).map_err(|e| format!("read {node_path}: {e}"))?;
    let node_cpus = expand_cpu_list(node_text.trim())?;
    if !requested.is_subset(&node_cpus) {
        return Err(format!(
            "requested CPUs {cpu_text} are not all on NUMA node {numa_node} ({})",
            node_text.trim()
        ));
    }
    Ok(observed_text)
}

fn json_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("create {}: {e}", path.display()))?;
    file.write_all(bytes)
        .map_err(|e| format!("write {}: {e}", path.display()))?;
    file.sync_all()
        .map_err(|e| format!("sync {}: {e}", path.display()))
}

fn point_digest(point: &EP) -> String {
    let mut bytes = Vec::new();
    match point {
        EP::Inf => bytes.push(0),
        EP::A(x, y) => {
            bytes.push(1);
            for limb in x.iter().chain(y.iter()) {
                bytes.extend_from_slice(&limb.to_be_bytes());
            }
        }
    }
    hex_digest(&sha256(&bytes))
}

fn array_u64(values: &[u64]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn array_usize(values: &[usize]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn is_lower_hex(text: &str, width: usize) -> bool {
    text.len() == width
        && text
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn dp_record_json(record: &ParallelDpRecord) -> String {
    format!(
        "{{\"worker\":{},\"lane\":{},\"walker_index\":{},\"step\":{},\"x\":[{},{},{}],\"y\":[{},{},{}],\"a\":{},\"b\":{},\"injected\":{}}}",
        record.worker,
        record.lane,
        record.walker_index,
        record.step,
        record.x[0],
        record.x[1],
        record.x[2],
        record.y[0],
        record.y[1],
        record.y[2],
        record.a,
        record.b,
        record.injected,
    )
}

fn peak_rss_bytes() -> Option<u64> {
    let status = fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find(|line| line.starts_with("VmHWM:"))?;
    let kib: u64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some(kib * 1024)
}

fn flat_field(prefix: &str, c: FieldCounts) -> String {
    format!(
        "\"{p}.fq_mul\":{},\"{p}.fq_inv\":{},\"{p}.fq_add\":{},\"{p}.fq_small_div\":{},\"{p}.f3_mul\":{},\"{p}.f3_sq\":{},\"{p}.f3_inv\":{},\"{p}.batch_inv\":{},\"{p}.group_add\":{}",
        c.fq_mul, c.fq_inv, c.fq_add, c.fq_small_div, c.f3_mul, c.f3_sq, c.f3_inv, c.batch_inv, c.group_add,
        p = prefix,
    )
}

fn flat_native(prefix: &str, c: NativeCounts) -> String {
    format!(
        "\"{p}.rng\":{},\"{p}.hash\":{},\"{p}.canon\":{},\"{p}.lookahead_test\":{},\"{p}.dp\":{},\"{p}.table\":{},\"{p}.cycle_cmp\":{},\"{p}.scalar_add\":{},\"{p}.scalar_mul\":{},\"{p}.scalar_inv\":{},\"{p}.band_ops\":{}",
        c.rng_draws, c.hashes, c.canonicalisations, c.lookahead_tests, c.dp_tests, c.table_ops, c.cycle_compares,
        c.scalar_adds, c.scalar_muls, c.scalar_invs, c.band_ops(), p = prefix,
    )
}

fn old_counts(c: &Cnt) -> String {
    format!(
        "\"c.mul\":{},\"c.inv\":{},\"c.add\":{},\"c.lmul\":{},\"c.ladd\":{},\"c.hash\":{},\"c.cmp\":{}",
        c.mul, c.inv, c.add, c.lmul, c.ladd, c.hash, c.cmp,
    )
}

fn run(q: u64, seed: u64, i0: usize, i1: usize, target_policy: &str, arms: &str) {
    assert!(matches!(target_policy, "ic-repeat" | "sequence"));
    let inst = Inst::search(q, seed, 200_000).expect("no GHS instance");
    let (ic_target, mut targets) = target_stream(&inst, seed);
    let triples = std::env::var("GHS_CALIB_TRIPLES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(600usize);
    let (r_inv, r_lmul, mul_ns_median, mul_ns_min) = calibration(q, inst.ell, triples);
    println!(
        "{{\"rec\":\"instance\",\"q\":{},\"seed\":{},\"ell\":{},\"order\":{},\"tries\":{},\"field_c\":{},\"ic_target\":{},\"target_policy\":\"{}\",\"source_revision\":\"{}\"}}",
        q,
        seed,
        inst.ell,
        inst.order,
        inst.tries,
        inst.kf.c,
        ic_target,
        target_policy,
        option_env!("GHS_SOURCE_REVISION").unwrap_or("unbound"),
    );
    println!(
        "{{\"rec\":\"calibration\",\"q\":{},\"seed\":{},\"triples\":{},\"r_inv\":{:.9},\"r_lmul\":{:.9},\"mul_ns_median\":{:.9},\"mul_ns_min\":{:.9},\"isolation\":\"{}\"}}",
        q,
        seed,
        triples,
        r_inv,
        r_lmul,
        mul_ns_median,
        mul_ns_min,
        std::env::var("GHS_ISOLATION").unwrap_or_else(|_| "development".into()),
    );
    for i in 0..i1 {
        let sequence_target = if i == 0 {
            ic_target
        } else {
            targets.below(inst.ell - 1) + 1
        };
        let (k, target_kind) = if target_policy == "ic-repeat" {
            (ic_target, "ic")
        } else if i == 0 {
            (sequence_target, "ic")
        } else {
            (sequence_target, "holdout")
        };
        if i < i0 {
            continue;
        }
        for arm in arms.split(',') {
            if arm == "old-neg" {
                let out = rho2::rho_neg(&inst, k, seed * 1000 + i as u64);
                println!(
                    "{{\"rec\":\"rho\",\"arm\":\"old-neg\",\"q\":{},\"seed\":{},\"i\":{},\"target_kind\":\"{}\",\"ell\":{},\"k\":{},\"ok\":{},\"steps\":{},\"dps\":{},\"restarts\":{},\"cycles\":{},\"lookahead\":{},\"wall_s\":{:.9},{}}}",
                    q, seed, i, target_kind, inst.ell, k, out.ok, out.steps, out.dps, out.restarts, out.cycles,
                    out.lookahead, out.wall, old_counts(&out.cnt),
                );
                continue;
            }
            let mode = parse_strong_arm(arm);
            let field_count_authoritative = mode.count_mode.field_authoritative();
            let native_count_authoritative = mode.count_mode.native_authoritative();
            let count_authoritative = field_count_authoritative && native_count_authoritative;
            let mut opts = StrongOpts::for_instance(&inst, mode.width);
            opts.batch_setup = !matches!(mode.setup_mode, SetupMode::Legacy);
            opts.reuse_setup_table_for_walkers =
                matches!(mode.setup_mode, SetupMode::BatchedJumpsAndWalkers);
            opts.use_mul5 = mode.use_mul5;
            let effective_mul5 = mode.use_mul5 && inst.q % 2 != 0 && inst.q % 3 != 0;
            let out = match mode.count_mode {
                CountMode::Counted => strong_rho(&inst, k, seed * 1000 + i as u64, opts),
                CountMode::FieldCounterFree => {
                    strong_rho_uncounted(&inst, k, seed * 1000 + i as u64, opts)
                }
                CountMode::Bare => strong_rho_bare(&inst, k, seed * 1000 + i as u64, opts),
            };
            println!(
                "{{\"rec\":\"rho\",\"arm\":\"{}\",\"batch\":{},\"setup_mode\":\"{}\",\"field_kernel\":\"{}\",\"count_authoritative\":{},\"field_count_authoritative\":{},\"native_count_authoritative\":{},\"q\":{},\"seed\":{},\"i\":{},\"target_kind\":\"{}\",\"ell\":{},\"k\":{},\"k_found\":{},\"ok\":{},\"steps\":{},\"dps\":{},\"restarts\":{},\"lookahead\":{},\"cycles\":{},\"cycle_adds\":{},\"setup.reciprocal_div\":{},\"wall_s\":{:.9},{},{},{},{},{}}}",
                arm,
                mode.width,
                match mode.setup_mode {
                    SetupMode::Legacy => "legacy",
                    SetupMode::BatchedJumps => "batched-window2-jumps",
                    SetupMode::BatchedJumpsAndWalkers => "batched-window2-jumps-walkers",
                },
                if effective_mul5 { "evaluation5" } else { "karatsuba6" },
                count_authoritative,
                field_count_authoritative,
                native_count_authoritative,
                q,
                seed,
                i,
                target_kind,
                inst.ell,
                k,
                out.found.map(|v| v.to_string()).unwrap_or_else(|| "null".into()),
                out.ok,
                out.steps,
                out.dps,
                out.restarts,
                out.lookahead,
                out.cycles,
                out.cycle_adds,
                out.setup_reciprocal_div,
                out.wall_s,
                flat_field("setup", out.setup_field),
                flat_field("walk", out.walk_field),
                flat_field("verify", out.verify_field),
                flat_native("setup_native", out.setup_native),
                flat_native("walk_native", out.walk_native),
            );
        }
    }
}

fn check_parallel_command(argv: &[String]) -> Result<(), String> {
    let args = named_args(argv)?;
    const ALLOWED: &[&str] = &[
        "q",
        "instance-seed",
        "target-index",
        "target-policy",
        "arm",
        "batch-width",
        "partitions",
        "workers",
        "worker-start",
        "dp-bits",
        "dp-shards",
        "queue-depth",
        "stop-check-steps",
        "stop-delay-checks",
        "worker-cpus",
        "numa-node",
        "inject-tail",
        "inject-collision-step",
        "output",
    ];
    for key in args.keys() {
        if !ALLOWED.contains(&key.as_str()) {
            return Err(format!("unknown --{key}"));
        }
    }
    let q: u64 = parse_num(&args, "q")?;
    let seed: u64 = parse_num(&args, "instance-seed")?;
    let target_index: usize = parse_num(&args, "target-index")?;
    let target_policy = required(&args, "target-policy")?;
    if !matches!(target_policy, "ic-repeat" | "sequence") {
        return Err("--target-policy must be ic-repeat or sequence".into());
    }
    let arm = required(&args, "arm")?;
    let mode = parse_strong_arm(arm);
    if !matches!(mode.count_mode, CountMode::Counted) {
        return Err("check-parallel currently admits counted strong-* arms only".into());
    }
    let batch: usize = parse_num(&args, "batch-width")?;
    if batch != mode.width {
        return Err(format!(
            "arm width {} disagrees with --batch-width {batch}",
            mode.width
        ));
    }
    let partitions: usize = parse_num(&args, "partitions")?;
    let workers: usize = parse_num(&args, "workers")?;
    if required(&args, "worker-start")? != "indexed-sha256-v1" {
        return Err("only --worker-start indexed-sha256-v1 is implemented".into());
    }
    let dp_shards: usize = parse_num(&args, "dp-shards")?;
    let queue_depth: usize = parse_num(&args, "queue-depth")?;
    let stop_check_steps: u64 = parse_num(&args, "stop-check-steps")?;
    let stop_delay_checks: u64 = parse_num(&args, "stop-delay-checks")?;
    let inject_collision_step: u64 = parse_num(&args, "inject-collision-step")?;
    let cpu_text = required(&args, "worker-cpus")?;
    if expand_cpu_list(cpu_text)?.len() != workers {
        return Err("--worker-cpus must name exactly one CPU per worker".into());
    }
    let numa_node: usize = parse_num(&args, "numa-node")?;
    let observed_affinity = validate_process_placement(cpu_text, numa_node)?;
    let (active_text, total_text) = required(&args, "inject-tail")?
        .split_once('/')
        .ok_or_else(|| "--inject-tail must be ACTIVE/TOTAL".to_owned())?;
    let active_lanes: usize = active_text
        .parse()
        .map_err(|_| "invalid active tail length")?;
    let total_lanes: usize = total_text
        .parse()
        .map_err(|_| "invalid total tail length")?;
    if total_lanes != batch || active_lanes > total_lanes {
        return Err("--inject-tail must satisfy ACTIVE<=TOTAL=batch-width".into());
    }
    let output = PathBuf::from(required(&args, "output")?);
    let transcript_path = PathBuf::from(format!("{}.dps.jsonl", output.display()));
    if output.exists() || transcript_path.exists() {
        return Err("output or DP transcript already exists".into());
    }
    let inst = Inst::search(q, seed, 200_000).ok_or_else(|| "no GHS instance".to_owned())?;
    let k = target_at(&inst, seed, target_index, target_policy);
    let target_index_u64 =
        u64::try_from(target_index).map_err(|_| "--target-index does not fit u64".to_owned())?;
    let walk_seed = seed
        .checked_mul(1000)
        .and_then(|value| value.checked_add(target_index_u64))
        .ok_or_else(|| "walk seed derivation overflow".to_owned())?;
    let mut strong = StrongOpts::for_instance(&inst, batch);
    strong.r = partitions;
    strong.dp_bits = match required(&args, "dp-bits")? {
        "auto" => strong.dp_bits,
        explicit => explicit.parse().map_err(|_| "invalid --dp-bits")?,
    };
    if !(1..=44).contains(&strong.dp_bits) {
        return Err("--dp-bits must be in 1..=44".into());
    }
    strong.batch_setup = !matches!(mode.setup_mode, SetupMode::Legacy);
    strong.reuse_setup_table_for_walkers =
        matches!(mode.setup_mode, SetupMode::BatchedJumpsAndWalkers);
    strong.use_mul5 = mode.use_mul5;
    let opts = ParallelCheckOpts {
        strong,
        workers,
        dp_shards,
        queue_depth,
        stop_check_steps,
        stop_delay_checks,
        active_lanes,
        inject_collision_step,
    };
    let out = strong_rho_parallel_check(&inst, k, walk_seed, opts);
    if out.total_steps != out.useful_steps + out.overshoot_steps {
        return Err("total_steps != useful_steps + overshoot_steps".into());
    }
    if out.per_worker_total_steps.iter().sum::<u64>() != out.total_steps {
        return Err("per-worker steps do not sum to total_steps".into());
    }
    if out.queue_submitted != out.queue_processed {
        return Err("bounded queue did not drain exactly".into());
    }
    if out.rejected_records != 0 {
        return Err(format!("{} DP records rejected", out.rejected_records));
    }
    if !out.ok {
        return Err("parallel deterministic check did not recover/verify the target".into());
    }
    let target = inst.ec.mul(&inst.g, k as u128);
    let target_digest = point_digest(&target);
    let recovered_scalar_check = out.found == Some(k);
    let source_revision = option_env!("GHS_SOURCE_REVISION").unwrap_or("unbound");
    let source_bound = is_lower_hex(source_revision, 40);
    let binary = std::env::current_exe().map_err(|error| format!("current_exe: {error}"))?;
    let binary_bytes = fs::read(&binary).map_err(|error| format!("read binary: {error}"))?;
    let binary_sha256 = hex_digest(&sha256(&binary_bytes));
    let job_text = format!(
        concat!(
            "ghs-rho/check-parallel/v2|source={source_revision}|binary={binary_sha256}",
            "|q={q}|ell={ell}|instance_seed={seed}|walk_seed={walk_seed}",
            "|target_index={target_index}|target_policy={target_policy}|target={target_digest}",
            "|arm={arm}|batch={batch}|active_lanes={active_lanes}|partitions={partitions}",
            "|dp_bits={dp_bits}|workers={workers}|worker_cpus={cpu_text}|numa_node={numa_node}",
            "|dp_shards={dp_shards}|queue_depth={queue_depth}",
            "|stop_check_steps={stop_check_steps}|stop_delay_checks={stop_delay_checks}",
            "|inject_collision_step={inject_collision_step}|transition={transition_digest}",
            "|jump={jump_digest}|worker_start={worker_start_digest}"
        ),
        source_revision = source_revision,
        binary_sha256 = binary_sha256,
        q = q,
        ell = inst.ell,
        seed = seed,
        walk_seed = walk_seed,
        target_index = target_index,
        target_policy = target_policy,
        target_digest = target_digest,
        arm = arm,
        batch = batch,
        active_lanes = active_lanes,
        partitions = partitions,
        dp_bits = strong.dp_bits,
        workers = workers,
        cpu_text = cpu_text,
        numa_node = numa_node,
        dp_shards = dp_shards,
        queue_depth = queue_depth,
        stop_check_steps = stop_check_steps,
        stop_delay_checks = stop_delay_checks,
        inject_collision_step = inject_collision_step,
        transition_digest = out.transition_digest,
        jump_digest = out.jump_digest,
        worker_start_digest = out.worker_start_digest,
    );
    let job_id = hex_digest(&sha256(job_text.as_bytes()));
    let table_entries: usize = out.shard_occupancy.iter().sum();
    let table_entry_payload_bytes =
        table_entries.saturating_mul(std::mem::size_of::<([u64; 3], ParallelDpRecord)>());
    // The non-timing control must replay byte-for-byte. Process high-water RSS
    // is intentionally unavailable here because allocator/host state can vary;
    // the admitted `run-parallel` record must populate it.
    let peak_rss = "null";
    let mut transcript = String::new();
    for dp in &out.transcript {
        transcript.push_str(&dp_record_json(dp));
        transcript.push('\n');
    }
    let transcript_sha256 = hex_digest(&sha256(transcript.as_bytes()));
    let transcript_bytes = transcript.len();
    let record = format!(
        concat!(
            "{{\"schema\":\"ghs-rho-parallel-check/v1\",\"job_id\":{},",
            "\"source_revision\":{},\"source_bound\":{},\"binary_sha256\":{},",
            "\"curve_id\":\"ghs-e0-fq3-q{}-ell{}\",",
            "\"q\":{},\"ell\":{},\"instance_seed\":{},\"walk_seed\":{},\"target_index\":{},",
            "\"target_policy\":{},\"target_digest\":{},\"k\":{},\"k_found\":{},\"ok\":{},",
            "\"control_only\":{},\"timing_observations\":0,\"performance_claims\":0,",
            "\"threaded\":false,\"scheduler\":\"deterministic-round-robin\",",
            "\"arm\":{},\"batch_width\":{},\"active_lanes\":{},\"partitions\":{},\"dp_bits\":{},",
            "\"workers\":{},\"worker_count_authority\":\"logical-control-only\",",
            "\"worker_cpu_map\":{},\"worker_cpu_map_authoritative\":false,\"affinity_requested\":{},",
            "\"affinity_observed\":{},\"affinity_scope\":\"process-control-only\",",
            "\"numa_requested\":{},\"numa_memory_policy_observed\":false,",
            "\"worker_start_scheme\":\"indexed-sha256-v1\",",
            "\"worker_start_digest\":{},\"transition_digest\":{},\"jump_digest\":{},",
            "\"dp_shards\":{},\"queue_depth\":{},\"stop_check_steps\":{},",
            "\"stop_delay_checks\":{},\"inject_collision_step\":{},",
            "\"stop_policy\":\"checked-aggregate-v1\",",
            "\"useful_steps\":{},\"overshoot_steps\":{},\"total_steps\":{},",
            "\"per_worker_total_steps\":{},\"dp_tests\":{},\"dp_emitted\":{},",
            "\"dp_inserted\":{},\"dp_duplicates\":{},\"sterile_collisions\":{},",
            "\"usable_collisions\":{},\"rejected_records\":{},\"queue_submitted\":{},",
            "\"queue_processed\":{},\"queue_peak_depth\":{},\"queue_block_events\":{},",
            "\"shard_occupancy\":{},\"table_entry_payload_bytes\":{},",
            "\"table_bytes_authoritative\":false,\"peak_rss\":{},",
            "\"restarts\":{},\"lookahead\":{},\"cycles\":{},\"cycle_adds\":{},",
            "{},{},{},{},{},\"dp_semantic_digest_sha256\":{},",
            "\"dp_transcript_sha256\":{},\"dp_transcript_bytes\":{},",
            "\"recovered_scalar_check\":{},\"exit_status\":\"ok\"}}\n"
        ),
        json_string(&job_id),
        json_string(source_revision),
        source_bound,
        json_string(&binary_sha256),
        q,
        inst.ell,
        q,
        inst.ell,
        seed,
        walk_seed,
        target_index,
        json_string(target_policy),
        json_string(&target_digest),
        k,
        out.found
            .map(|v| v.to_string())
            .unwrap_or_else(|| "null".into()),
        out.ok,
        out.control_only,
        json_string(arm),
        batch,
        active_lanes,
        partitions,
        strong.dp_bits,
        workers,
        json_string(cpu_text),
        json_string(cpu_text),
        json_string(&observed_affinity),
        numa_node,
        json_string(&out.worker_start_digest),
        json_string(&out.transition_digest),
        json_string(&out.jump_digest),
        dp_shards,
        queue_depth,
        stop_check_steps,
        stop_delay_checks,
        inject_collision_step,
        out.useful_steps,
        out.overshoot_steps,
        out.total_steps,
        array_u64(&out.per_worker_total_steps),
        out.dp_tests,
        out.dp_emitted,
        out.dp_inserted,
        out.dp_duplicates,
        out.sterile_collisions,
        out.usable_collisions,
        out.rejected_records,
        out.queue_submitted,
        out.queue_processed,
        out.queue_peak_depth,
        out.queue_block_events,
        array_usize(&out.shard_occupancy),
        table_entry_payload_bytes,
        peak_rss,
        out.restarts,
        out.lookahead,
        out.cycles,
        out.cycle_adds,
        flat_field("setup", out.setup_field),
        flat_field("walk", out.walk_field),
        flat_field("verify", out.verify_field),
        flat_native("setup_native", out.setup_native),
        flat_native("walk_native", out.walk_native),
        json_string(&out.dp_transcript_digest),
        json_string(&transcript_sha256),
        transcript_bytes,
        recovered_scalar_check,
    );
    write_new(&output, record.as_bytes())?;
    if let Err(error) = write_new(&transcript_path, transcript.as_bytes()) {
        let _ = fs::remove_file(&output);
        return Err(error);
    }
    println!("CHECK-PARALLEL OK {}", output.display());
    println!("DP-TRANSCRIPT {}", transcript_path.display());
    Ok(())
}

fn calibrate_command(argv: &[String]) -> Result<(), String> {
    let args = named_args(argv)?;
    const ALLOWED: &[&str] = &[
        "q",
        "instance-seed",
        "triples",
        "worker-cpus",
        "numa-node",
        "output",
    ];
    for key in args.keys() {
        if !ALLOWED.contains(&key.as_str()) {
            return Err(format!("unknown --{key}"));
        }
    }
    let benchrec = std::env::var("BENCHREC")
        .map_err(|_| "calibrate requires a BENCHREC reference".to_owned())?;
    if benchrec.trim().is_empty() {
        return Err("BENCHREC is empty".into());
    }
    let q: u64 = parse_num(&args, "q")?;
    let seed: u64 = parse_num(&args, "instance-seed")?;
    let triples: usize = parse_num(&args, "triples")?;
    if triples == 0 {
        return Err("--triples must be positive".into());
    }
    let cpu_text = required(&args, "worker-cpus")?;
    let numa_node: usize = parse_num(&args, "numa-node")?;
    let observed_affinity = validate_process_placement(cpu_text, numa_node)?;
    let output = PathBuf::from(required(&args, "output")?);
    if output.exists() {
        return Err("calibration output already exists".into());
    }
    let inst = Inst::search(q, seed, 200_000).ok_or_else(|| "no GHS instance".to_owned())?;
    let result = calibration_full(q, inst.ell, triples);
    let binary = std::env::current_exe().map_err(|e| format!("current_exe: {e}"))?;
    let binary_bytes = fs::read(&binary).map_err(|e| format!("read binary: {e}"))?;
    let binary_sha256 = hex_digest(&sha256(&binary_bytes));
    let samples = result
        .samples
        .iter()
        .enumerate()
        .map(|(i, s)| {
            format!(
                "{{\"i\":{i},\"mul_s\":{:.12},\"inv_s\":{:.12},\"lmul_s\":{:.12},\"r_inv\":{:.12},\"r_lmul\":{:.12}}}",
                s.mul_s, s.inv_s, s.lmul_s, s.r_inv, s.r_lmul
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let record = format!(
        concat!(
            "{{\"schema\":\"ghs-rho-calibration-development/v1\",\"q\":{},\"ell\":{},",
            "\"instance_seed\":{},\"triples\":{},\"r_inv\":{:.12},",
            "\"r_lmul\":{:.12},\"mul_ns_median\":{:.12},\"mul_ns_min\":{:.12},",
            "\"worker_cpus\":{},\"affinity_observed\":{},\"affinity_scope\":\"process-control-only\",",
            "\"placement_pinned\":false,\"numa_node\":{},\"numa_memory_policy_observed\":false,",
            "\"source_revision\":{},\"source_bound\":{},\"compiler\":{},",
            "\"binary_sha256\":{},\"benchrec_reference\":{},",
            "\"isolation\":\"ungraded-process-control-only\",",
            "\"admission_status\":\"diagnostic_only\",\"timing_admissible\":false,",
            "\"samples\":[{}]}}\n"
        ),
        q,
        inst.ell,
        seed,
        triples,
        result.r_inv,
        result.r_lmul,
        result.mul_ns_median,
        result.mul_ns_min,
        json_string(cpu_text),
        json_string(&observed_affinity),
        numa_node,
        json_string(option_env!("GHS_SOURCE_REVISION").unwrap_or("unbound")),
        option_env!("GHS_SOURCE_REVISION").is_some_and(|value| is_lower_hex(value, 40)),
        json_string(option_env!("GHS_COMPILER").unwrap_or("unbound")),
        json_string(&binary_sha256),
        json_string(&benchrec),
        samples,
    );
    write_new(&output, record.as_bytes())?;
    println!("CALIBRATION OK {}", output.display());
    Ok(())
}

fn usage() -> ! {
    eprintln!(
        "usage: ghs_strong_rho selftest | run <q> <seed> <i0> <i1> <ic-repeat|sequence> <arms> | check-parallel --q Q ... --output PATH | calibrate --q Q ... --output PATH\nrun-parallel is intentionally unavailable until the threaded affinity/NUMA and timing implementation passes deterministic controls"
    );
    std::process::exit(2)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 2 && args[1] == "selftest" {
        selftest_arm_parser();
        strong_rho::selftest();
        return;
    }
    if args.len() >= 2 && args[1] == "check-parallel" {
        if let Err(error) = check_parallel_command(&args[2..]) {
            eprintln!("CHECK-PARALLEL FAILED: {error}");
            std::process::exit(1);
        }
        return;
    }
    if args.len() >= 2 && args[1] == "calibrate" {
        if let Err(error) = calibrate_command(&args[2..]) {
            eprintln!("CALIBRATION FAILED: {error}");
            std::process::exit(1);
        }
        return;
    }
    if args.len() >= 2 && args[1] == "run-parallel" {
        eprintln!(
            "RUN-PARALLEL UNAVAILABLE: threaded worker affinity/NUMA readback and admitted timing are not implemented"
        );
        std::process::exit(3);
    }
    if args.len() != 8 || args[1] != "run" {
        usage();
    }
    run(
        args[2].parse().unwrap(),
        args[3].parse().unwrap(),
        args[4].parse().unwrap(),
        args[5].parse().unwrap(),
        &args[6],
        &args[7],
    );
}
