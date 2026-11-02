//! E37 post-hoc diagnostic (not registered): why C_F diverges structurally.
//! Copied from `e37_checks.rs`.  E37 (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.44, G2.164–G2.165):
//! negative controls that corrupt state the solve reads, with a read probe
//! as their positive check, on E25's ten curves (32 calls each, a control
//! stream `StdRng(seed ^ 0xE37C_0001)` after E25's base and precompute).
//!
//! Per call and rule (F, S, H, L): the clean run (probe on) must return the
//! uncounted solver A's solutions, degree and decomposition set; F and L
//! charge no selection work; E31's perturbed coefficient must change every
//! non-empty solution set.  Controls, each aimed from the clean run's read
//! log of the same call:
//!
//! - C_SH (S, H): the inverse permutation maps the contents of `r₁` (the
//!   row of the first left-looking normal-form read) and `r₂` (the row of
//!   the last such read whose row and hash differ from `r₁`'s) to each other;
//! - C_F (F): at `c*`, the first contested column in the normal-form read
//!   log, the second arrival is filed by its next nonzero without its update;
//! - C_0 (F, S, H, L, calibration): one nonzero right of the pivot of the
//!   first normal-form-read row with such an entry moved by one.
//!
//! **Positive check:** the control run's read log first differs from the
//! clean run's at an entry of the same phase and column with a different row
//! hash.  **Detected:** solutions, degree, decomposition set or the fallback
//! flag differ from the clean run's.  Exits non-zero only when a clean run
//! differs from the reference (an identity failure); thresholds are judged
//! by `e37_tables`.
//!
//! ```text
//! RAYON_NUM_THREADS=1 ./e37_checks --json experiments/44_pivot_rules_confirm/checks.json
//! ```

use std::collections::{HashMap, HashSet};

use crypto_lib::cryptanalysis::chain_pivot_rules::{
    probe_begin, probe_end, take_diag, ArrivalRule, ReadProbe, RuleCheck,
};
use crypto_lib::cryptanalysis::gaudry_quartic::{PivotCheck, PivotRule, QuarticSolve};
use crypto_lib::cryptanalysis::klein_v4::{
    lift_back_v4, solve_weighted_chain_with, v4_system, Pivot, SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::quartic_folds::{generate_gls_klein_instance, klein_base_with};
use crypto_lib::cryptanalysis::residual_walk::mix64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;
use serde_json::json;

const SALT: u64 = 0xE37C_0001;
const CALLS_PER_CURVE: usize = 32;
const CURVES: [(u64, u64); 10] = [
    (101, 1),
    (101, 2),
    (149, 1),
    (149, 2),
    (197, 1),
    (197, 2),
    (269, 1),
    (269, 2),
    (397, 1),
    (397, 2),
];
const RULES: [(ArrivalRule, &str); 4] = [
    (ArrivalRule::Fifo, "F"),
    (ArrivalRule::Static, "S"),
    (ArrivalRule::FifoStatic, "H"),
    (ArrivalRule::Lifo, "L"),
];

/// `(solutions, degree, decomposition set, fell back, probe, state)`.
type Run = (
    Option<Vec<[u64; 4]>>,
    u16,
    Vec<[u64; 4]>,
    bool,
    ReadProbe,
    QuarticSolve,
);

/// One control's tallies.
#[derive(Default, Serialize, Clone)]
struct Tally {
    calls: usize,
    /// Calls where the control could be aimed (a target row or column found).
    aimed: usize,
    /// Calls where the corruption moved different contents (diagnostic).
    effective: usize,
    /// Positive check: first log difference at the same phase and column,
    /// different row hash.
    positive: usize,
    /// First log difference elsewhere (another column or phase, or one log
    /// a prefix of the other): structure changed, no same-column read.
    structural: usize,
    /// Logs identical.
    unread: usize,
    /// Detected (any call), and detected with a positive check.
    detected: usize,
    detected_positive: usize,
    /// Calls with a non-empty clean solution set, and those detected with a
    /// positive check.
    nonempty: usize,
    nonempty_detected_positive: usize,
    /// Detections that came through the fallback flag only.
    via_fallback: usize,
}

fn log_key(p: &ReadProbe) -> Vec<(u8, u32, u64)> {
    p.reads.iter().map(|r| (r.0, r.1, r.3)).collect()
}

/// 0 = positive (same phase/column, other hash), 1 = structural, 2 = equal.
fn first_difference(clean: &ReadProbe, bad: &ReadProbe) -> u8 {
    let (a, b) = (log_key(clean), log_key(bad));
    match a.iter().zip(&b).position(|(x, y)| x != y) {
        Some(k) => {
            if (a[k].0, a[k].1) == (b[k].0, b[k].1) {
                0
            } else {
                1
            }
        }
        None if a.len() != b.len() => 1,
        None => 2,
    }
}


fn main() {
    for (p, seed) in [(101u64, 1u64), (397, 2)] {
        let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
        let curve = ki.inst.curve();
        let f = &curve.f;
        let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
        let _fb = klein_base_with(&ki, false, &mut rng).expect("base");
        let pre = SymmetrisedV4::precompute(&ki, &curve, &mut rng, 8).expect("precompute");
        let mut rng = StdRng::seed_from_u64(seed ^ SALT);
        let sseed = seed ^ SALT;
        let n = ki.inst.n;
        let (mut done, mut residuals) = (0usize, 0u64);
        while done < 8 {
            let (a, b) = (rng.gen_range(0..n), rng.gen_range(1..n));
            let tseed = mix64(sseed ^ residuals.wrapping_mul(0x9E37_79B9_7F4A_7C15));
            residuals += 1;
            let r = curve.add(&curve.mul(&ki.inst.g, a), &curve.mul(&ki.inst.q, b));
            let Some(z_r) = ki.z_of_x(f, &r.x) else { continue };
            done += 1;
            let mut fp = 0u64;
            let comps = v4_system(&pre, f, &z_r, &mut fp);
            let run = |pivot: Pivot| -> (QuarticSolve, ReadProbe, bool) {
                let mut tr = StdRng::seed_from_u64(tseed);
                let mut st = QuarticSolve::default();
                probe_begin();
                let (_s, _d, info) = solve_weighted_chain_with(&comps, MACAULAY_BOUND, p, &mut tr, &mut st, pivot);
                (st, probe_end(), info.fell_back)
            };
            let c = run(Pivot::Rule(ArrivalRule::Fifo, RuleCheck::default()));
            let contested: HashSet<u32> = c.1.contested.iter().copied().collect();
            let cstar = c.1.reads.iter().find(|r| r.0 == 1 && contested.contains(&r.1)).map(|r| r.1);
            let bad = run(Pivot::Rule(ArrivalRule::Fifo, RuleCheck { misfile_at: cstar, ..Default::default() }));
            let (ka, kb) = (log_key(&c.1), log_key(&bad.1));
            let k = ka.iter().zip(&kb).position(|(x, y)| x != y);
            let first0 = c.1.reads.iter().find(|r| r.0 == 0).map(|r| r.1);
            println!(
                "p={p} s={seed} c*={cstar:?} first phase-0 col clean {first0:?} | pivots {} -> {} | zero rows {} -> {} | dim {} -> {} | reached {} -> {} | fell back {} -> {} | log len {} -> {} first diff at {k:?}: clean {:?} bad {:?}",
                c.0.pivots, bad.0.pivots, c.0.zero_rows, bad.0.zero_rows, c.0.dim, bad.0.dim,
                c.0.reached_pivots, bad.0.reached_pivots, c.2, bad.2, ka.len(), kb.len(),
                k.map(|k| ka.get(k)), k.map(|k| kb.get(k))
            );
            let _ = (HashMap::<u8, u8>::new(), take_diag(), PivotCheck::default(), PivotRule::Lowest, lift_back_v4 as usize, json!(0), CURVES, RULES, CALLS_PER_CURVE);
        }
    }
}
