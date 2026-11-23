//! Run50 per-phase recount with the recoverable accounting gaps closed, and
//! the AGENTS.md section 5a band (complete-accounting lane).
//!
//! A RECOUNT OF RECORDED DATA, not a new measurement and not an improvement:
//! no algorithm changes, every arm's recorded counts are kept and repriced
//! exactly, and each added term names its source. [pcc:unverified]
//!
//! Inputs (read-only):
//!   QUALIFIED  runpod-deferred-pair-20261117-50/qualified-original.json
//!   ORIGINAL   runpod-deferred-pair-20261117-50/original-ic.raw.json
//!   RECOUNT    runs/recount-seed389-n3415.json (linear_stage_recount)
//! Run: accounting_recount QUALIFIED ORIGINAL RECOUNT > recount.json
//!
//! Terms per arm:
//!   R  recorded observed primitive subtotal, by phase (repriced; must equal
//!      the recorded `diagnostic_priced_bitops` of every phase);
//!   U  the uniform samplers' separately priced native integer products and
//!      additions (`known_native_priced_bitops`, recorded, excluded from R);
//!   L  the descent linear stage's own F_p work, uncounted in R:
//!      graph arms - the per-primitive increment measured by replaying the
//!      reconstructed Run50 stream with the counted linear stage (RECOUNT);
//!      original IC - an ESTIMATE: its recorded legacy `descent_lin_muls`
//!      times the graph stream's increment per legacy linear-stage product;
//!   B  recorded unpriced native integer events (uniform sampler sub/clz/shl),
//!      priced at w GAE each for w in {0, 0.1, 1};
//!   N  named gaps with no emitted count (listed; they keep every total a
//!      lower bound and block any ranking under section 5a).
use serde_json::{json, Value};

const KINDS: [&str; 11] = [
    "field_add", "field_mul", "field_sqr", "field_inv", "field_sqrt", "legendre", "modr_add",
    "modr_mul", "modr_inv", "word_op", "hash_probe",
];

/// bitop/v1 (src/cryptanalysis/bitops.rs): int_add(w) = 5w,
/// int_mul(w) = w^2 + 5w(w-1); fp add = 2 int_add + 3b, mul = 2 int_mul + add,
/// inv = 2b add, sqrt = 2b mul, legendre = inv; word_op 64; hash_probe 4b;
/// mod-r arithmetic is prime-field arithmetic at r's width.
fn int_add(w: u128) -> u128 {
    5 * w
}
fn int_mul(w: u128) -> u128 {
    w * w + 5 * w * w.saturating_sub(1)
}
fn fp_add(b: u128) -> u128 {
    2 * int_add(b) + 3 * b
}
fn fp_mul(b: u128) -> u128 {
    2 * int_mul(b) + fp_add(b)
}
fn fp_inv(b: u128) -> u128 {
    2 * b * fp_add(b)
}
fn gates(kind: &str, b: u128, r: u128) -> u128 {
    match kind {
        "field_add" => fp_add(b),
        "field_mul" | "field_sqr" => fp_mul(b),
        "field_inv" | "legendre" => fp_inv(b),
        "field_sqrt" => 2 * b * fp_mul(b),
        "modr_add" => fp_add(r),
        "modr_mul" => fp_mul(r),
        "modr_inv" => fp_inv(r),
        "word_op" => 64,
        "hash_probe" => 4 * b,
        _ => panic!("unknown kind {kind}"),
    }
}

fn num(v: &Value) -> u128 {
    match v {
        Value::Number(n) => n.to_string().parse().expect("integer"),
        Value::String(s) => s.parse().expect("integer string"),
        Value::Null => 0,
        _ => panic!("not a number: {v}"),
    }
}

fn price(prims: &Value, b: u128, r: u128) -> u128 {
    KINDS.iter().map(|k| num(&prims[*k]) * gates(k, b, r)).sum()
}

fn load(path: &str) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}")))
        .unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// One affine addition on E(F_{p^6}) (bitops.rs PRIME_ADD: 1 I, 2 M, 1 S,
/// 6 A over the curve's field) with the F_{p^6} primitives of the rho
/// reference's own native tower calibration as recorded in Run50
/// (`native_calibration`: M = S = 55 field_mul + 44 field_add,
/// I = 80 field_mul + 42 field_add + 1 field_inv, A = 6 field_add).
fn gae_bitops(cal: &Value, b: u128, r: u128) -> (u128, Value) {
    let m = &cal["mul"];
    let s = &cal["square"];
    let i = &cal["inverse"];
    let mut v = serde_json::Map::new();
    for k in KINDS {
        let c = num(&i[k]) + 2 * num(&m[k]) + num(&s[k]) + if k == "field_add" { 36 } else { 0 };
        v.insert(k.to_string(), json!(c));
    }
    let v = Value::Object(v);
    (price(&v, b, r), v)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    assert_eq!(a.len(), 4, "usage: accounting_recount QUALIFIED ORIGINAL RECOUNT");
    let q = load(&a[1]);
    let orig = load(&a[2]);
    let rc = load(&a[3]);
    assert_eq!(rc["admitted"], json!(true), "recount not admitted");
    assert_eq!(num(&rc["conservation_failures"]), 0);
    let p = num(&q["public"]["p"]);
    let l = num(&q["public"]["l"]);
    let b = 128 - p.leading_zeros() as u128;
    let r = 128 - l.leading_zeros() as u128;
    assert_eq!(num(&rc["p"]), p);
    let outcomes = q["outcomes"].as_array().expect("outcomes");
    let rho_cal = outcomes
        .iter()
        .find(|o| o["mode"].as_str().unwrap().starts_with("rho"))
        .map(|o| o["proof"]["accounting"]["native_calibration"].clone())
        .expect("rho calibration");
    let (gae, gae_prims) = gae_bitops(&rho_cal, b, r);

    // L: per-primitive increment of the counted linear stage on the stream
    let with = &rc["descent_with_counted_linear_stage"]["primitives"];
    let pre = &rc["descent_pre_patch_hooks"]["primitives"];
    let mut inc = serde_json::Map::new();
    for k in KINDS {
        inc.insert(k.to_string(), json!((num(&with[k]) - num(&pre[k])).to_string()));
    }
    let inc = Value::Object(inc);
    let inc_bitops = price(&inc, b, r);
    assert_eq!(
        inc_bitops,
        num(&rc["descent_with_counted_linear_stage"]["bitops"]) - num(&rc["descent_pre_patch_hooks"]["bitops"])
    );
    let stream_legacy_lin = num(&rc["legacy_lin_muls"]);
    let orig_legacy_lin = num(&orig["work"]["descent_lin_muls"]);
    // estimate for original IC, rounded down
    let orig_l_estimate = inc_bitops * orig_legacy_lin / stream_legacy_lin;

    let weights: [(u128, u128, &str); 3] = [(0, 1, "0"), (1, 10, "0.1"), (1, 1, "1")];
    let mut arms = Vec::new();
    for o in outcomes {
        let mode = o["mode"].as_str().unwrap().to_string();
        let acc = &o["proof"]["accounting"];
        let rho = mode.starts_with("rho");
        let mut phases = Vec::new();
        let mut rsum = 0u128;
        for ph in acc["phases"].as_array().unwrap() {
            let rec = num(&ph["diagnostic_priced_bitops"]);
            let rep = price(&ph["primitives"], b, r);
            assert_eq!(rec, rep, "{mode} {} reprice", ph["phase"]);
            rsum += rep;
            if rep > 0 {
                phases.push(json!({"phase": ph["phase"], "recorded_bitops": rec.to_string()}));
            }
        }
        assert_eq!(rsum, num(&acc["observed_primitive_subtotal_bitops"]), "{mode} subtotal");
        let u = &acc["uniform_sampling"];
        let u_bitops = num(&u["known_native_priced_bitops"]);
        let unpriced_ops: u128 = if u.is_null() {
            0
        } else {
            ["integer_sub64", "integer_clz64", "integer_shl64"].iter().map(|k| num(&u["unpriced_native"][*k])).sum()
        };
        let graph = matches!(mode.as_str(), "quadratic-table" | "pair-discriminant" | "pair-deferred");
        let (l_bitops, l_kind) = if graph {
            (inc_bitops, "replay recount: counted linear stage on the reconstructed Run50 stream (seed 389, 3,415 residuals)")
        } else if mode == "original-ic" {
            (orig_l_estimate, "ESTIMATE: recorded descent_lin_muls x the stream's increment per legacy linear-stage product")
        } else {
            (0, "not applicable (no Groebner descent)")
        };
        let descent_rec = phases
            .iter()
            .find(|x| x["phase"] == "descent")
            .map(|x| num(&x["recorded_bitops"]))
            .unwrap_or(0);
        let base = rsum + u_bitops + l_bitops;
        let band: Vec<Value> = weights
            .iter()
            .map(|&(n, d, name)| {
                let add = unpriced_ops * gae * n / d;
                json!({"w_gae": name, "band_bitops": add.to_string(), "lower_bound_total_bitops": (base + add).to_string()})
            })
            .collect();
        let named_null: Vec<Value> = if rho {
            acc_rho_null()
        } else {
            vec![
                json!("native containers/normalization/scheduling (graph maps, witness storage, activation queues)"),
                json!("PRNG generator internals (ChaCha12 blocks behind every recorded draw)"),
                json!("incremental RREF scans / fixed-factor internals (linear algebra)"),
                json!("cohort hashing/serialization"),
                json!("descent linear stage: staircase setup copies, allocation; PolyRing root-splitting RNG draws"),
            ]
        };
        arms.push(json!({
            "mode": mode,
            "R_recorded_subtotal_bitops": rsum.to_string(),
            "R_descent_bitops": descent_rec.to_string(),
            "R_phases": phases,
            "U_uniform_known_native_priced_bitops": u_bitops.to_string(),
            "L_linear_stage_bitops": l_bitops.to_string(),
            "L_source": l_kind,
            "B_unpriced_native_ops": unpriced_ops.to_string(),
            "R_plus_U_plus_L_bitops": base.to_string(),
            "band": band,
            "N_named_gaps_without_counts": named_null,
            "complete": false,
        }));
    }
    // distance to matched rho: graph deferred vs each rho arm, per band point
    let total = |m: &str, w: usize| -> u128 {
        let x = arms.iter().find(|x| x["mode"] == m).unwrap();
        num(&x["band"][w]["lower_bound_total_bitops"])
    };
    let mut distance = Vec::new();
    for ic in ["original-ic", "quadratic-table", "pair-discriminant", "pair-deferred"] {
        for rho in ["rho-plain-b1", "rho-neg-b1", "rho-plain-b16", "rho-neg-b16"] {
            let rs: Vec<Value> = (0..3)
                .map(|w| {
                    json!({"w_gae": weights[w].2,
                        "ic_over_rho": format!("{:.6}", total(ic, w) as f64 / total(rho, w) as f64)})
                })
                .collect();
            distance.push(json!({"ic": ic, "rho": rho, "ratios": rs}));
        }
    }
    let out = json!({
        "schema": "complete-accounting.run50-recount/v1",
        "pcc": "unverified",
        "label": "recount of recorded Run50 data plus a replay-derived linear-stage term; not a new measurement, not an improvement, not a complete cost",
        "field_bits": b.to_string(), "r_bits": r.to_string(),
        "gae_definition": "one affine E(F_{p^6}) addition priced in bitop/v1 from the rho reference's recorded native tower calibration",
        "gae_primitives": gae_prims, "gae_bitops": gae.to_string(),
        "linear_stage_increment_primitives": inc,
        "linear_stage_increment_bitops": inc_bitops.to_string(),
        "stream_legacy_lin_muls": stream_legacy_lin.to_string(),
        "original_ic_legacy_lin_muls": orig_legacy_lin.to_string(),
        "arms": arms,
        "distance_to_rho_lower_bound_ratios": distance,
        "ranking": "none: every arm, rho included, has named native gaps without emitted counts (section 5a); ratios are between lower bounds",
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}

fn acc_rho_null() -> Vec<Value> {
    vec![
        json!("tuned_reference_bitop_pricing (count null in every rho raw)"),
        json!("rho_setup_and_native_bookkeeping_audit (count null)"),
        json!("rng_internal_work (count null; 514-544 draws recorded)"),
        json!("DP table container memory/insert internals beyond the counted hash_probe/word_op"),
    ]
}
