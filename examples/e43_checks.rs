//! **E43 controls** (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.58,
//! G2.259–G2.260): FV (`chain-fifo-profile`) and FW (`chain-fifo-tails`)
//! against F (`chain-fifo`) on E25's ten curves, base and precompute from
//! E25's stream, then the first `--calls` targets per curve of the stream
//! `StdRng(seed ^ salt)` (salt `E43C0001`), per-target seeds as E42's checks.
//!
//! Per call, every run from the same per-target seed; "identity against F"
//! is the solutions, degree, outcome, decomposition set, fallback flag, the
//! nine phase counts and the pivot digest of F's run.
//! - clean: FV = F and FW = F; FV's counter = its measured row lengths; FW's
//!   counter = final lengths + discarded buffers;
//! - PC1 (rule lost): V and W (A's sorted buckets on the same storage) must
//!   differ from F;
//! - PC2 (storage lost under F): FV with the lead cell dropped (E40's P1)
//!   and FW capped at the build end with the fill check off (E42's P1) must
//!   differ from F;
//! - PC3 (count lost under F): FV's and FW's counters short by one per row
//!   (E40's and E42's P2) must fail the measured-length checks;
//! - G0 under F: FW capped at the build end, fill check on, must equal F and
//!   pass the counter check (re-allocations recorded, descriptive);
//! - E31's perturbed coefficient must change FV's and FW's solution sets on
//!   every call with a non-empty clean set;
//! - F's digest against A counted's (a lost rule is visible on contested
//!   calls).
//!
//! ```text
//! RAYON_NUM_THREADS=1 ./e43_checks --calls 32 --json experiments/50_compose_fw/checks.json
//! ```

use std::collections::HashMap;

use crypto_lib::cryptanalysis::chain_pivot_rules::{ArrivalRule, RuleCheck};
use crypto_lib::cryptanalysis::gaudry_quartic::{PivotCheck, PivotRule, QuarticSolve};
use crypto_lib::cryptanalysis::klein_v4::{
    lift_back_v4, solve_weighted_chain_with, v4_system, Pivot, SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::macaulay_profile::{
    profile_measure_ordered, BucketOrder, ProfileCheck,
};
use crypto_lib::cryptanalysis::macaulay_tails::{tails_measure_ordered, TailsCheck};
use crypto_lib::cryptanalysis::quartic_folds::{generate_gls_klein_instance, klein_base_with};
use crypto_lib::cryptanalysis::residual_walk::mix64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

const SALT: u64 = 0xE43C_0001;
const FIFO: BucketOrder = BucketOrder::Fifo;

type Out = (
    Option<Vec<[u64; 4]>>,
    u16,
    String,
    Vec<[u64; 4]>,
    bool,
    [u64; 9],
    u64,
);

fn arg<T: std::str::FromStr>(args: &[String], k: &str, d: T) -> T {
    args.iter()
        .position(|a| a == k)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(d)
}

fn salt_arg(args: &[String]) -> u64 {
    args.iter()
        .position(|a| a == "--salt")
        .and_then(|i| args.get(i + 1))
        .map(|v| u64::from_str_radix(v.trim_start_matches("0x"), 16).expect("hex salt"))
        .unwrap_or(SALT)
}

/// Per-curve (and total) tallies, in this order.
const KEYS: [&str; 20] = [
    "calls",
    "fv_equals_f",
    "fw_equals_f",
    "fv_counter_ok",
    "fw_counter_ok",
    "pc1_v_detected",
    "pc1_w_detected",
    "pc2_fv_detected",
    "pc2_fw_detected",
    "pc3_fv_detected",
    "pc3_fw_detected",
    "g0f_equals_f",
    "g0f_counter_ok",
    "g0f_reallocs",
    "fw_reallocs",
    "w_reallocs",
    "nonempty",
    "perturbed_fv_changed",
    "perturbed_fw_changed",
    "f_digest_differs_from_a",
];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let per_curve: usize = arg(&args, "--calls", 32);
    let json_out: String = arg(&args, "--json", String::new());
    let salt = salt_arg(&args);
    let mut rows = Vec::new();
    let mut tot = [0u64; KEYS.len()];
    for p in [101u64, 149, 197, 269, 397] {
        for seed in [1u64, 2] {
            let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
            let inst = ki.inst;
            let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
            let _fb = klein_base_with(&ki, false, &mut rng).expect("base");
            let pcurve = inst.curve();
            let pre = SymmetrisedV4::precompute(&ki, &pcurve, &mut rng, 8).expect("precompute");
            let mut rng = StdRng::seed_from_u64(seed ^ salt);
            let sseed = seed ^ salt;
            let n = inst.n;
            let mut t = [0u64; KEYS.len()];
            let mut residuals = 0u64;
            'outer: loop {
                let batch: Vec<(u64, u64, u64)> = (0..8u64)
                    .map(|k| {
                        (
                            rng.gen_range(0..n),
                            rng.gen_range(1..n),
                            mix64(sseed ^ (residuals + k).wrapping_mul(0x9E37_79B9_7F4A_7C15)),
                        )
                    })
                    .collect();
                for (a, b, tseed) in batch {
                    if t[0] as usize >= per_curve {
                        break 'outer;
                    }
                    residuals += 1;
                    let c = inst.curve();
                    let r = c.add(&c.mul(&inst.g, a), &c.mul(&inst.q, b));
                    let Some(z_r) = ki.z_of_x(&c.f, &r.x) else {
                        continue;
                    };
                    let mut fp = 0u64;
                    let comps = v4_system(&pre, &c.f, &z_r, &mut fp);
                    t[0] += 1;
                    let run = |cs: &[HashMap<[u8; 4], u64>], pv: Pivot| -> Out {
                        let mut trng = StdRng::seed_from_u64(tseed);
                        let mut st = QuarticSolve::default();
                        let (s, d, info) = solve_weighted_chain_with(
                            cs,
                            MACAULAY_BOUND,
                            p,
                            &mut trng,
                            &mut st,
                            pv,
                        );
                        let lb = lift_back_v4(&ki, &c, s.as_deref().unwrap_or(&[]), &r, &mut trng);
                        let ph = [
                            st.weil_muls,
                            st.triangularise_muls,
                            st.echelon_muls,
                            st.nf_muls,
                            st.charpoly_muls,
                            st.roots_muls,
                            st.eigenvector_muls,
                            st.verify_muls,
                            st.total_muls,
                        ];
                        (
                            s,
                            d,
                            st.outcome,
                            lb.decompositions,
                            info.fell_back,
                            ph,
                            st.pivot_digest,
                        )
                    };
                    let fv_of = |chk| Pivot::ProfileOrdered(chk, FIFO);
                    let fw_of = |chk| Pivot::TailsOrdered(chk, FIFO);
                    let f = run(&comps, Pivot::Rule(ArrivalRule::Fifo, RuleCheck::default()));
                    let a_c = run(
                        &comps,
                        Pivot::Counted(PivotRule::Lowest, PivotCheck::default()),
                    );
                    let fv = run(&comps, fv_of(ProfileCheck::default()));
                    let fw = run(&comps, fw_of(TailsCheck::default()));
                    let v = run(&comps, Pivot::Profile(ProfileCheck::default()));
                    let w = run(&comps, Pivot::Tails(TailsCheck::default()));
                    let fv_p2 = run(
                        &comps,
                        fv_of(ProfileCheck {
                            drop_lead: true,
                            undercount: false,
                        }),
                    );
                    let fw_p2 = run(
                        &comps,
                        fw_of(TailsCheck {
                            exact_caps: true,
                            no_fill: true,
                            undercount: false,
                        }),
                    );
                    let g0c = TailsCheck {
                        exact_caps: true,
                        no_fill: false,
                        undercount: false,
                    };
                    let g0 = run(&comps, fw_of(g0c));
                    let pm = |chk| profile_measure_ordered(&comps, MACAULAY_BOUND, p, chk, FIFO);
                    let tm =
                        |chk, order| tails_measure_ordered(&comps, MACAULAY_BOUND, p, chk, order);
                    let tok = |m: &Option<
                        crypto_lib::cryptanalysis::macaulay_tails::OrderedTailsMeasure,
                    >| {
                        m.as_ref().is_some_and(|m| {
                            m.measure.counter == m.measure.final_lengths + m.measure.discarded
                        })
                    };
                    let realloc =
                        |m: &Option<
                            crypto_lib::cryptanalysis::macaulay_tails::OrderedTailsMeasure,
                        >| m.as_ref().map_or(0, |m| m.measure.reallocs);
                    let fv_m = pm(ProfileCheck::default());
                    let fv_m3 = pm(ProfileCheck {
                        drop_lead: false,
                        undercount: true,
                    });
                    let fw_m = tm(TailsCheck::default(), FIFO);
                    let w_m = tm(TailsCheck::default(), BucketOrder::Lowest);
                    let fw_m3 = tm(
                        TailsCheck {
                            exact_caps: false,
                            no_fill: false,
                            undercount: true,
                        },
                        FIFO,
                    );
                    let g0_m = tm(g0c, FIFO);
                    // E31's perturbed coefficient: the smallest key of the first
                    // non-empty equation, moved by one.
                    let mut bad = comps.clone();
                    if let Some(eq) = bad.iter_mut().find(|e| !e.is_empty()) {
                        let k = *eq.keys().min().unwrap();
                        let v0 = eq[&k];
                        eq.insert(k, (v0 + 1) % p);
                    }
                    let fvp = run(&bad, fv_of(ProfileCheck::default()));
                    let fwp = run(&bad, fw_of(TailsCheck::default()));
                    let ne = fv.0.as_ref().is_some_and(|s| !s.is_empty());
                    let hits = [
                        fv == f,
                        fw == f,
                        fv_m.is_some_and(|m| m.counter == m.measured),
                        tok(&fw_m),
                        v != f,
                        w != f,
                        fv_p2 != f,
                        fw_p2 != f,
                        fv_m3.is_some_and(|m| m.counter != m.measured),
                        !tok(&fw_m3),
                        g0 == f,
                        tok(&g0_m),
                    ];
                    for (i, h) in hits.iter().enumerate() {
                        t[1 + i] += u64::from(*h);
                    }
                    t[13] += realloc(&g0_m);
                    t[14] += realloc(&fw_m);
                    t[15] += realloc(&w_m);
                    t[16] += u64::from(ne);
                    t[17] += u64::from(ne && fvp.0 != fv.0);
                    t[18] += u64::from(ne && fwp.0 != fw.0);
                    t[19] += u64::from(f.6 != a_c.6);
                }
            }
            eprintln!(
                "p={p} seed={seed}: {}",
                KEYS.iter()
                    .zip(t.iter())
                    .map(|(k, v)| format!("{k} {v}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            let mut row = serde_json::Map::new();
            row.insert("p".into(), json!(p));
            row.insert("seed".into(), json!(seed));
            for (k, v) in KEYS.iter().zip(t.iter()) {
                row.insert((*k).into(), json!(v));
            }
            rows.push(serde_json::Value::Object(row));
            for (x, y) in tot.iter_mut().zip(t.iter()) {
                *x += y;
            }
        }
    }
    let get = |k: &str| tot[KEYS.iter().position(|x| *x == k).unwrap()];
    let calls = get("calls");
    let all = |k: &str| get(k) == calls;
    let g0_mean = get("g0f_reallocs") as f64 / calls.max(1) as f64;
    let g2_259 = all("fv_equals_f")
        && all("fw_equals_f")
        && all("fv_counter_ok")
        && all("fw_counter_ok")
        && all("pc1_v_detected")
        && all("pc1_w_detected")
        && all("pc2_fv_detected")
        && all("pc2_fw_detected")
        && all("pc3_fv_detected")
        && all("pc3_fw_detected")
        && all("g0f_equals_f")
        && all("g0f_counter_ok")
        && get("perturbed_fv_changed") == get("nonempty")
        && get("perturbed_fw_changed") == get("nonempty");
    let mut v = serde_json::Map::new();
    v.insert("experiment".into(), json!("E43 controls (§9.58)"));
    v.insert("stream_salt".into(), json!(salt));
    for k in KEYS {
        v.insert(k.into(), json!(get(k)));
    }
    v.insert("g0f_reallocs_per_call".into(), json!(g0_mean));
    v.insert(
        "fw_reallocs_per_call".into(),
        json!(get("fw_reallocs") as f64 / calls.max(1) as f64),
    );
    v.insert(
        "w_reallocs_per_call".into(),
        json!(get("w_reallocs") as f64 / calls.max(1) as f64),
    );
    v.insert("g2_259_e43_checks_holds".into(), json!(g2_259));
    v.insert(
        "g2_260_holds".into(),
        json!((150.0..=500.0).contains(&g0_mean)),
    );
    v.insert("curves".into(), json!(rows));
    let v = serde_json::Value::Object(v);
    eprintln!("total: {v}");
    if !json_out.is_empty() {
        std::fs::write(&json_out, serde_json::to_string_pretty(&v).unwrap()).unwrap();
    }
}
