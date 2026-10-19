//! Frozen same-Q full-lane comparison for serial and batched collaborative rho.
//! The caller pins the process, alternates modes, and applies host noise gates.

use std::fs;
use std::sync::Mutex;
use std::time::Instant;

use base64::Engine;
use crypto_lib::cryptanalysis::pollard_collab::job::JobSpec;
use crypto_lib::cryptanalysis::pollard_collab::state::SharedState;
use crypto_lib::cryptanalysis::pollard_collab::worker::{run_lane, LaneOptions};
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::field::FieldElement;
use crypto_lib::ecc::point::Point;
use crypto_lib::hash::sha256;
use num_bigint::BigUint;
use serde_json::{json, Value};

const MANIFEST_SHA256: &str = "f46ad43788b7f9241927904541cbeb03ea70a96ca95aa84d527a9b22ba8d12b8";
const SEED: u64 = 0x71cc_2026_1017;

fn hex_big(s: &str) -> BigUint {
    BigUint::parse_bytes(s.as_bytes(), 16).expect("valid hex integer")
}

fn required<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key]
        .as_str()
        .unwrap_or_else(|| panic!("missing {key}"))
}

fn peak_rss_kib() -> Option<u64> {
    let status = fs::read_to_string("/proc/self/status").ok()?;
    let value = status.lines().find(|line| line.starts_with("VmHWM:"))?;
    value.split_whitespace().nth(1)?.parse().ok()
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert!(
        matches!(args.len(), 8 | 9 | 10),
        "usage: rho_worker_batch_benchmark MANIFEST Q_INDEX serial|batch WALKERS DP_BITS STEP_CAP BATCH_WIDTH [CHECKIN_EVERY] [--strict-output]"
    );
    let manifest_bytes = fs::read(&args[1]).expect("read frozen Q manifest");
    let manifest_sha256 = hex::encode(sha256(&manifest_bytes));
    assert_eq!(manifest_sha256, MANIFEST_SHA256, "Q manifest changed");
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("valid Q manifest");
    assert_eq!(required(&manifest, "curve"), "secp256k1");
    let fixtures = manifest["fixtures"].as_array().expect("fixture array");
    assert_eq!(fixtures.len(), 32, "frozen Q count");
    let q_index: usize = args[2].parse().expect("Q index");
    let fixture = fixtures.get(q_index).expect("Q index in range");
    let mode = args[3].as_str();
    assert!(matches!(mode, "serial" | "batch"));
    let walkers: u64 = args[4].parse().expect("walkers");
    let dp_bits: u8 = args[5].parse().expect("DP bits");
    let step_cap: u64 = args[6].parse().expect("step cap");
    let batch_width: usize = args[7].parse().expect("batch width");
    let checkin_every: u64 = args
        .get(8)
        .map_or(8, |word| word.parse().expect("checkin every"));
    let strict_output = args.get(9).is_some_and(|word| word == "--strict-output");
    assert!(args.len() != 10 || strict_output, "unknown output format");
    assert!(walkers > 0 && walkers <= 256 && step_cap > 0);
    assert!((2..32).contains(&dp_bits));
    assert!(batch_width > 0 && batch_width <= walkers as usize);
    assert!(checkin_every > 0 && checkin_every <= walkers);
    assert!(mode == "batch" || batch_width == 1);

    let curve = CurveParams::secp256k1();
    let q_hex = required(fixture, "q_hex");
    let (qx, qy) = q_hex.split_once(':').expect("affine Q encoding");
    let q = Point::Affine {
        x: FieldElement::new(hex_big(qx), curve.p.clone()),
        y: FieldElement::new(hex_big(qy), curve.p.clone()),
    };
    assert!(curve.is_on_curve(&q));
    let q_scalar = hex_big(required(fixture, "q_scalar_hex"));
    assert_eq!(
        curve
            .generator()
            .scalar_mul_vartime(&q_scalar, &curve.a_fe()),
        q,
        "frozen public Q scalar replay"
    );

    // Fixture loading and its known-scalar replay are outside both arms.
    // Q-dependent job construction and local worker state are charged.
    let setup_started = Instant::now();
    let mut spec = JobSpec::new(&curve, &q, "rho-worker-batch-v1", SEED).expect("valid job");
    spec.dp_bits = dp_bits;
    spec.num_branches = 32;
    spec.negation_map = false;
    spec.unit_size = walkers;
    spec.max_steps_per_walker = step_cap;
    let ctx = spec.build().expect("validated job context");
    let state = Mutex::new(SharedState::new(&ctx));
    let setup_ns = setup_started.elapsed().as_nanos();

    let options = LaneOptions {
        checkin_every,
        claim_window: 1,
        max_units: 1,
        max_walkers: walkers,
        batch_width,
        ..LaneOptions::new("bench.0")
    };
    let mut checkins = Vec::new();
    let walk_started = Instant::now();
    let summary = run_lane(
        &ctx,
        &state,
        &options,
        &mut |ci| checkins.push(ci.clone()),
        &|| false,
    );
    let walk_ns = walk_started.elapsed().as_nanos();
    let peak_rss_kib = peak_rss_kib();

    let verify_started = Instant::now();
    let mut verified_dps = 0u64;
    for ci in &mut checkins {
        ci.time = 0;
        for dp in &ci.dps {
            dp.verify(&ctx).expect("independent DP record replay");
            verified_dps += 1;
        }
    }
    let verify_ns = verify_started.elapsed().as_nanos();
    assert_eq!(verified_dps, summary.dps);
    let checkin_blake3 = blake3::hash(&serde_json::to_vec(&checkins).expect("check-in JSON"))
        .to_hex()
        .to_string();
    let status = if summary.walkers == walkers && summary.solution.is_none() {
        "complete_frozen_work"
    } else {
        "early_stop_or_solution"
    };
    let report = json!({
        "schema": "rho-worker-batch-benchmark-v1",
        "status": status,
        "tier": "unclassified_host_receipt_required",
        "curve": "secp256k1",
        "fixture_manifest_sha256": manifest_sha256,
        "q_index": q_index,
        "q_hex": q_hex,
        "mode": mode,
        "batch_width": batch_width,
        "job_id": ctx.job_id,
        "seed": SEED,
        "walkers_requested": walkers,
        "dp_bits": dp_bits,
        "step_cap": step_cap,
        "checkin_every": options.checkin_every,
        "setup_ns": setup_ns,
        "walk_ns": walk_ns,
        "total_charged_ns": setup_ns + walk_ns,
        "verification_ns_uncharged": verify_ns,
        "peak_rss_kib": peak_rss_kib,
        "walkers_done": summary.walkers,
        "walk_steps": summary.steps,
        "distinguished_points": summary.dps,
        "verified_distinguished_points": verified_dps,
        "dead_trails": summary.dead_trails,
        "units_completed": summary.units_completed,
        "checkins": summary.checkins,
        "checkin_blake3_time_normalized": checkin_blake3,
        "solution_hex": summary.solution.map(|v| format!("{v:x}")),
    });
    if strict_output {
        assert_eq!(status, "complete_frozen_work");
        let raw_b64 = base64::engine::general_purpose::URL_SAFE.encode(report.to_string());
        println!(
            "job_id={} checkin_blake3_time_normalized={} walkers_done={} walk_steps={} distinguished_points={} dead_trails={} units_completed={} checkins={} q_index={} q_hex={} dp_bits={} step_cap={} checkin_every={} verified=1 online_ms={:.6} peak_rss_kib={} raw_b64={}",
            report["job_id"].as_str().unwrap(),
            report["checkin_blake3_time_normalized"].as_str().unwrap(),
            summary.walkers,
            summary.steps,
            summary.dps,
            summary.dead_trails,
            summary.units_completed,
            summary.checkins,
            q_index,
            q_hex,
            dp_bits,
            step_cap,
            options.checkin_every,
            (setup_ns + walk_ns) as f64 / 1_000_000.0,
            peak_rss_kib.expect("Linux peak RSS"),
            raw_b64,
        );
    } else {
        println!("{report}");
    }
}
