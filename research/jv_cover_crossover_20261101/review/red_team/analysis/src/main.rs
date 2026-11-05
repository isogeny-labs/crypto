//! Red-team analysis for research/jv_cover_crossover_20261101 (Rust only, no Python number).
//!
//! Usage: jv_crossover_red_team <study results dir> <predecessor results dir> <controls dir> <out dir>
//!
//! Reads the study's frozen records (never written), the predecessor's rho_e samples, and the
//! red-team control records; writes analysis.json and analysis.txt into <out dir>.
//! Every number is labelled measured (read from a record), derived (computed here) or model.

use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 11
    }
    fn idx(&mut self, n: usize) -> usize {
        (self.next() as usize) % n
    }
}
fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn sd(v: &[f64]) -> f64 {
    let m = mean(v);
    (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0)).sqrt()
}
fn se(v: &[f64]) -> f64 {
    sd(v) / (v.len() as f64).sqrt()
}
fn gmean(v: &[f64]) -> f64 {
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}
fn pct(v: &[f64], q: f64) -> f64 {
    let mut w = v.to_vec();
    w.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let k = ((w.len() as f64 - 1.0) * q).round() as usize;
    w[k]
}
/// Welch: (difference of means, its standard error, Satterthwaite df, t)
fn welch(a: &[f64], b: &[f64]) -> (f64, f64, f64, f64) {
    let (va, vb) = (
        sd(a).powi(2) / a.len() as f64,
        sd(b).powi(2) / b.len() as f64,
    );
    let s = (va + vb).sqrt();
    let df = (va + vb).powi(2)
        / (va.powi(2) / (a.len() as f64 - 1.0) + vb.powi(2) / (b.len() as f64 - 1.0));
    (mean(a) - mean(b), s, df, (mean(a) - mean(b)) / s)
}
fn rows(path: &Path) -> Vec<Value> {
    let t = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    match serde_json::from_str::<Value>(&t).unwrap() {
        Value::Array(a) => a,
        v => vec![v],
    }
}
fn files_with(dir: &str, prefix: &str) -> Vec<PathBuf> {
    let Ok(rd) = fs::read_dir(dir) else { return vec![] };
    let mut v: Vec<PathBuf> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name().unwrap().to_str().unwrap().starts_with(prefix)
                && p.extension().is_some_and(|e| e == "json")
        })
        .collect();
    v.sort();
    v
}
fn f(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or_else(|| panic!("missing {k}"))
}
fn u(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap_or_else(|| panic!("missing {k}"))
}

/// A reference arm: run samples grouped by instance (resampled within group, pooled mean).
#[derive(Clone, Default)]
struct Arm {
    groups: Vec<Vec<f64>>,
}
impl Arm {
    fn all(&self) -> Vec<f64> {
        self.groups.iter().flatten().cloned().collect()
    }
    fn n(&self) -> usize {
        self.groups.iter().map(|g| g.len()).sum()
    }
    fn mean(&self) -> f64 {
        mean(&self.all())
    }
    fn boot_mean(&self, rng: &mut Lcg) -> f64 {
        let (mut tot, mut n) = (0.0, 0.0);
        for g in &self.groups {
            for _ in 0..g.len() {
                tot += g[rng.idx(g.len())];
                n += 1.0;
            }
        }
        tot / n
    }
    fn merged(arms: &[&Arm]) -> Arm {
        Arm { groups: arms.iter().flat_map(|a| a.groups.clone()).collect() }
    }
}
fn verdict(lo: f64, hi: f64) -> &'static str {
    if hi < 1.0 {
        "below"
    } else if lo > 1.0 {
        "above"
    } else {
        "not resolved"
    }
}
/// The study's frozen rule: geometric mean of the cover seeds over the reference mean, percentile
/// bootstrap (20000) over seeds and over reference runs within instance. `fixed` = a constant reference.
fn ratio_verdict(ss: &[f64], arm: Option<&Arm>, fixed: Option<f64>, rng: &mut Lcg) -> Value {
    let r = match (arm, fixed) {
        (Some(a), _) => a.mean(),
        (None, Some(c)) => c,
        _ => unreachable!(),
    };
    let point = gmean(ss) / r;
    let mut bs = Vec::with_capacity(20000);
    for _ in 0..20000 {
        let g: Vec<f64> = (0..ss.len()).map(|_| ss[rng.idx(ss.len())]).collect();
        let d = match arm {
            Some(a) => a.boot_mean(rng),
            None => r,
        };
        bs.push(gmean(&g) / d);
    }
    let (lo, hi) = (pct(&bs, 0.025), pct(&bs, 0.975));
    json!({"reference_mean": r, "reference_n": arm.map_or(0, |a| a.n()), "ratio_geo_mean": point,
           "ci95_boot": [lo, hi], "verdict": verdict(lo, hi),
           "per_seed_ratio": ss.iter().map(|s| s / r).collect::<Vec<_>>()})
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (rdir, pdir, cdir, out) = (&a[1], &a[2], &a[3], &a[4]);
    let mut txt = String::new();
    macro_rules! say { ($($t:tt)*) => {{ let s = format!($($t)*); println!("{s}"); txt += &s; txt += "\n"; }} }
    let mut rng = Lcg(20261101_7);
    let mut report = json!({});

    // ---------------- cover arm (study, measured) ----------------
    #[derive(Clone)]
    struct Cover {
        p: f64,
        l: f64,
        c_e: f64,
        total_muls: u64,
        s: f64,
        f4_native: u64,
        residual_ratio: f64,
        base: u64,
        cross_checked: u64,
        f4_muls: u64,
        lin_muls: u64,
        la_muls: u64,
    }
    let mut cover: BTreeMap<u64, Vec<Cover>> = BTreeMap::new();
    for pth in files_with(rdir, "dlp_p") {
        for r in rows(&pth) {
            let c = Cover {
                p: f(&r, "p"),
                l: f(&r, "l"),
                c_e: f(&r, "c_add_e"),
                total_muls: u(&r, "total_muls"),
                s: f(&r, "s"),
                f4_native: u(&r, "f4_native_ops"),
                residual_ratio: f(&r, "residual_ratio"),
                base: u(&r, "base"),
                cross_checked: u(&r, "cross_checked"),
                f4_muls: u(&r, "f4_muls"),
                lin_muls: u(&r, "lin_muls"),
                la_muls: u(&r, "la_muls"),
            };
            assert!(r["solved"].as_bool().unwrap() && r["correct"].as_bool().unwrap());
            let s_check = c.total_muls as f64 / (c.c_e * c.l.sqrt());
            assert!((s_check - c.s).abs() < 1e-9, "S recomputation");
            cover.entry(c.p as u64).or_default().push(c);
        }
    }

    // ---------------- rhodp (study + calibration, measured) ----------------
    let mut rhodp: BTreeMap<u64, Arm> = BTreeMap::new(); // by prime, groups = instances
    let mut rhodp_threads: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    let mut rhodp_sqrt_l: BTreeMap<u64, f64> = BTreeMap::new();
    for prefix in ["rhodp_p", "calib_p"] {
        for pth in files_with(rdir, prefix) {
            let rs = rows(&pth);
            let p = u(&rs[0], "p");
            rhodp_sqrt_l.insert(p, f(&rs[0], "l").sqrt());
            let g: Vec<f64> = rs
                .iter()
                .map(|r| {
                    assert!(r["correct"].as_bool().unwrap());
                    rhodp_threads.entry(p).or_default().push(u(r, "threads"));
                    f(r, "s")
                })
                .collect();
            rhodp.entry(p).or_default().groups.push(g);
        }
    }
    // ---------------- rhostrong (study, measured) ----------------
    #[derive(Default, Clone)]
    struct Strong {
        steps: Arm,
        harness: Arm,
        tuned: Arm,
        native: Arm,
        fp_adds_per_step: Vec<f64>,
    }
    let mut strong: BTreeMap<u64, Strong> = BTreeMap::new();
    let mut strong_rows: BTreeMap<(u64, u64), Vec<Value>> = BTreeMap::new();
    for pth in files_with(rdir, "rhostrong_p") {
        for r in rows(&pth) {
            assert!(r["correct"].as_bool().unwrap() && !r["capped"].as_bool().unwrap());
            strong_rows.entry((u(&r, "p"), u(&r, "seed"))).or_default().push(r);
        }
    }
    let native_of = |r: &Value| -> f64 {
        ["canon_cmp", "hash", "dp_check", "scalar_ops", "table_ops", "cycle_cmp"]
            .iter()
            .map(|k| f(&r["ops"], k))
            .sum()
    };
    for ((p, _seed), rs) in &strong_rows {
        let st = strong.entry(*p).or_default();
        st.steps.groups.push(rs.iter().map(|r| f(r, "steps") / f(r, "l").sqrt()).collect());
        st.harness.groups.push(rs.iter().map(|r| f(r, "s_harness")).collect());
        st.tuned.groups.push(rs.iter().map(|r| f(r, "s_tuned")).collect());
        st.native.groups.push(rs.iter().map(|r| native_of(r) / f(r, "l").sqrt()).collect());
        st.fp_adds_per_step
            .extend(rs.iter().map(|r| f(&r["ops"], "fp_adds") / f(r, "steps")));
    }

    // ---------------- predecessor rho_e at 251 (measured) ----------------
    let mut rho_e_251 = Arm::default();
    for s in [1u64, 2] {
        let r = &rows(&Path::new(pdir).join(format!("dlp_p251_s{s}.json")))[0];
        rho_e_251
            .groups
            .push(r["rho"].as_array().unwrap().iter().map(|x| f(x, "s")).collect());
    }
    let pred_1009: Option<(u64, u64)> = {
        let r = &rows(&Path::new(pdir).join("dlp_p1009_s1.json"))[0];
        Some((u(r, "total_muls"), u(r, "lin_muls")))
    };

    // ---------------- controls (red team, measured) ----------------
    let mut ctrl = json!({});
    // F4 counter check
    let f4c: Vec<Value> = ["f4check_t1", "f4check_t2"]
        .iter()
        .filter_map(|n| {
            let p = Path::new(cdir).join(format!("{n}.json"));
            p.exists().then(|| {
                let r = &rows(&p)[0];
                json!({"label": n, "f4_muls": u(r,"f4_muls"), "f4_muls_per_test_sum": u(r,"f4_muls_per_test_sum"),
                       "total_muls": u(r,"total_muls"), "lin_muls": u(r,"lin_muls"), "relations": u(r,"relations"),
                       "relation_digest": u(r,"relation_digest"), "per_test_over_global": u(r,"f4_muls_per_test_sum") as f64 / u(r,"f4_muls") as f64})
            })
        })
        .collect();
    if f4c.len() == 2 {
        say!("F4 COUNTER CHECK (p=53 seed 2): 1 thread f4_muls {} per-test sum {} (ratio {:.4}); 2 threads f4_muls {} per-test sum {} (ratio {:.4}); global f4_muls thread-independent: {}; total_muls equal: {}; relation digest equal: {}",
            f4c[0]["f4_muls"], f4c[0]["f4_muls_per_test_sum"], f4c[0]["per_test_over_global"].as_f64().unwrap(),
            f4c[1]["f4_muls"], f4c[1]["f4_muls_per_test_sum"], f4c[1]["per_test_over_global"].as_f64().unwrap(),
            f4c[0]["f4_muls"] == f4c[1]["f4_muls"], f4c[0]["total_muls"] == f4c[1]["total_muls"], f4c[0]["relation_digest"] == f4c[1]["relation_digest"]);
    }
    ctrl["f4_counter_check"] = json!(f4c);
    // reproduction with JV_LIN=sparse
    let rp = Path::new(cdir).join("repro_sparse_p1009_s1.json");
    if rp.exists() {
        let r = &rows(&rp)[0];
        let (pt, pl) = pred_1009.unwrap();
        let (nt, nl) = (u(r, "total_muls"), u(r, "lin_muls"));
        say!("REPRODUCTION with JV_LIN=sparse (p=1009 seed 1, 2 rayon threads): total_muls {nt} vs predecessor {pt} identical: {}; lin_muls {nl} vs predecessor {pl} identical: {}; solved {} correct {}",
            nt == pt, nl == pl, r["solved"], r["correct"]);
        ctrl["reproduction_sparse"] = json!({"total_muls": nt, "predecessor_total_muls": pt, "identical": nt == pt,
            "lin_muls": nl, "predecessor_lin_muls": pl, "lin_identical": nl == pl, "solved": r["solved"], "correct": r["correct"]});
    }
    // rhodp null at 251
    let mut null_t2 = Arm::default();
    let mut null_t1 = Arm::default();
    for s in [1u64, 2] {
        let p = Path::new(cdir).join(format!("rhodp_null_p251_s{s}.json"));
        if p.exists() {
            let rs = rows(&p);
            null_t2.groups.push(
                rs.iter()
                    .map(|r| {
                        assert!(r["correct"].as_bool().unwrap());
                        assert_eq!(u(r, "threads"), 2);
                        f(r, "s")
                    })
                    .collect(),
            );
        }
    }
    {
        let p = Path::new(cdir).join("rhodp_null_p251_s1_t1.json");
        if p.exists() {
            let rs = rows(&p);
            null_t1.groups.push(
                rs.iter()
                    .map(|r| {
                        assert!(r["correct"].as_bool().unwrap());
                        assert_eq!(u(r, "threads"), 1);
                        f(r, "s")
                    })
                    .collect(),
            );
        }
    }
    // strong rho at 251: negation arm and no-negation arm
    let mut neg251 = Strong::default();
    let mut noneg251 = Strong::default();
    for (prefix, st) in [("rhostrong_neg_p251_run", &mut neg251), ("rhostrong_noneg_p251_run", &mut noneg251)] {
        let mut by_seed: BTreeMap<u64, Vec<Value>> = BTreeMap::new();
        for pth in files_with(cdir, prefix) {
            for r in rows(&pth) {
                assert!(r["correct"].as_bool().unwrap() && !r["capped"].as_bool().unwrap());
                by_seed.entry(u(&r, "seed")).or_default().push(r);
            }
        }
        for (_s, rs) in &by_seed {
            st.steps.groups.push(rs.iter().map(|r| f(r, "steps") / f(r, "l").sqrt()).collect());
            st.harness.groups.push(rs.iter().map(|r| f(r, "s_harness")).collect());
            st.tuned.groups.push(rs.iter().map(|r| f(r, "s_tuned")).collect());
            st.native.groups.push(rs.iter().map(|r| native_of(r) / f(r, "l").sqrt()).collect());
            st.fp_adds_per_step.extend(rs.iter().map(|r| f(&r["ops"], "fp_adds") / f(r, "steps")));
        }
    }

    // ---------------- rho instrument statistics ----------------
    // Expected S for a random walk without negation: sqrt(pi/2) = 1.2533; r-adding correction 1/sqrt(1-1/r) (model).
    let exp_plain = (std::f64::consts::PI / 2.0).sqrt();
    let exp_plain_r32 = exp_plain / (1.0f64 - 1.0 / 32.0).sqrt();
    let exp_neg = (std::f64::consts::PI / 4.0).sqrt();
    let exp_neg_r256 = exp_neg / (1.0f64 - 1.0 / 256.0).sqrt();
    say!("MODEL expectations (random-walk theory): plain rho S = sqrt(pi/2) = {exp_plain:.4}, r=32 adding {exp_plain_r32:.4}; negation rho S = sqrt(pi/4) = {exp_neg:.4}, r=256 adding {exp_neg_r256:.4}");
    let new_primes = [1301u64, 1451, 1601];
    let rhodp_new = Arm::merged(&new_primes.iter().map(|p| &rhodp[p]).collect::<Vec<_>>());
    let rhodp_all = Arm::merged(&rhodp.values().collect::<Vec<_>>());
    let mut rho_stats = json!({});
    // Stop-flag overshoot (model, read from rho_e_dp): each non-finding walker polls `stop` every 2^20
    // steps, so it overshoots by 2^19 steps on average; charged into steps and ops.
    let overshoot = |threads: f64, sqrt_l: f64| (threads - 1.0) * (1u64 << 19) as f64 / sqrt_l;
    let s251 = rhodp_sqrt_l[&251];
    let s_new: f64 = new_primes.iter().map(|p| rhodp_sqrt_l[p]).sum::<f64>() / 3.0;
    say!("MODEL stop-flag overshoot of rho_e_dp, (threads-1)*2^19/sqrt(l): p=251 4 threads {:.3}, 2 threads {:.3}, 1 thread 0; new primes 4 threads {:.4}",
        overshoot(4.0, s251), overshoot(2.0, s251), overshoot(4.0, s_new));
    for (label, arm, ovs) in [
        ("rhodp study new primes (27 runs, 4 threads)", &rhodp_new, overshoot(4.0, s_new)),
        ("rhodp study calibration p=251 (16 runs, 4 threads)", &rhodp[&251], overshoot(4.0, s251)),
        ("rhodp study pooled (43 runs, 4 threads)", &rhodp_all, (27.0 * overshoot(4.0, s_new) + 16.0 * overshoot(4.0, s251)) / 43.0),
        ("rhodp red-team null p=251 (2 threads)", &null_t2, overshoot(2.0, s251)),
        ("rhodp red-team null p=251 (1 thread)", &null_t1, 0.0),
        ("rho_e predecessor p=251 (32 runs, stored table)", &rho_e_251, 0.0),
    ] {
        if arm.n() == 0 {
            continue;
        }
        let v = arm.all();
        let (m, s, e) = (mean(&v), sd(&v), se(&v));
        say!("RHO (no negation) {label}: n {} mean {m:.4} sd {s:.3} se {e:.3}; z vs {exp_plain_r32:.4}: {:+.2}; mean/expected {:.3}; overshoot model {ovs:.3} -> corrected mean {:.4} (z {:+.2})",
            v.len(), (m - exp_plain_r32) / e, m / exp_plain_r32, m - ovs, (m - ovs - exp_plain_r32) / e);
        rho_stats[label] = json!({"n": v.len(), "mean": m, "sd": s, "se": e, "z_vs_r32_theory": (m - exp_plain_r32) / e, "ratio_to_theory": m / exp_plain_r32,
            "overshoot_model": ovs, "corrected_mean": m - ovs, "corrected_z": (m - ovs - exp_plain_r32) / e});
    }
    if null_t1.n() > 0 && null_t2.n() > 0 {
        let both = Arm::merged(&[&null_t1, &null_t2]);
        let v = both.all();
        say!("RHO (no negation) rhodp red-team null pooled 1+2 threads: n {} mean {:.4} se {:.3}", v.len(), mean(&v), se(&v));
    }
    {
        let (d, s, df, t) = welch(&rhodp[&251].all(), &rho_e_251.all());
        say!("  Welch study calibration: rhodp(4 threads) - rho_e at 251: diff {d:+.3} se {s:.3} df {df:.1} t {t:+.2}; after overshoot correction diff {:+.3} t {:+.2}", d - overshoot(4.0, s251), (d - overshoot(4.0, s251)) / s);
        rho_stats["calibration_welch"] = json!({"diff": d, "se": s, "df": df, "t": t, "diff_corrected": d - overshoot(4.0, s251)});
    }
    if null_t2.n() > 0 {
        let (d, s, df, t) = welch(&null_t2.all(), &rho_e_251.all());
        say!("  Welch rhodp null (2 threads) - rho_e predecessor at 251: diff {d:+.3} se {s:.3} df {df:.1} t {t:+.2}");
        let (d, s, df, t) = welch(&rhodp_all.all(), &null_t2.all());
        say!("  Welch rhodp study pooled (4 threads, 43) - rhodp null (2 threads): diff {d:+.3} se {s:.3} df {df:.1} t {t:+.2}");
        rho_stats["welch_null_vs_rho_e"] = json!({"diff": d, "se": s, "df": df, "t": t});
    }
    // negation arms
    let strong_new = Strong {
        steps: Arm::merged(&new_primes.iter().map(|p| &strong[p].steps).collect::<Vec<_>>()),
        harness: Arm::merged(&new_primes.iter().map(|p| &strong[p].harness).collect::<Vec<_>>()),
        tuned: Arm::merged(&new_primes.iter().map(|p| &strong[p].tuned).collect::<Vec<_>>()),
        native: Arm::merged(&new_primes.iter().map(|p| &strong[p].native).collect::<Vec<_>>()),
        fp_adds_per_step: new_primes.iter().flat_map(|p| strong[p].fp_adds_per_step.clone()).collect(),
    };
    let mut strong_pool = Strong::default();
    for st in [&strong_new, &neg251] {
        strong_pool.fp_adds_per_step.extend(st.fp_adds_per_step.clone());
        strong_pool.steps.groups.extend(st.steps.groups.clone());
        strong_pool.harness.groups.extend(st.harness.groups.clone());
        strong_pool.tuned.groups.extend(st.tuned.groups.clone());
        strong_pool.native.groups.extend(st.native.groups.clone());
    }
    for (label, st, expect) in [
        ("rhostrong negation, study new primes (18 runs)", &strong_new, exp_neg_r256),
        ("rhostrong negation, red-team p=251", &neg251, exp_neg_r256),
        ("rhostrong negation, pooled study + red-team", &strong_pool, exp_neg_r256),
        ("rhostrong NO negation, batch 1, r 32, red-team p=251", &noneg251, exp_plain_r32),
    ] {
        if st.steps.n() == 0 {
            continue;
        }
        let v = st.steps.all();
        let h = st.harness.all();
        let t = st.tuned.all();
        let nat = st.native.all();
        let adds = if st.fp_adds_per_step.is_empty() { f64::NAN } else { mean(&st.fp_adds_per_step) };
        say!("STRONG RHO {label}: n {} steps/sqrt(l) mean {:.4} sd {:.3} se {:.3} (z vs {expect:.4}: {:+.2}); s_harness mean {:.4}; s_tuned mean {:.4} (tuned/harness {:.3}); native/sqrt(l) mean {:.2}; F_p adds per step {:.1}",
            v.len(), mean(&v), sd(&v), se(&v), (mean(&v) - expect) / se(&v), mean(&h), mean(&t), mean(&t) / mean(&h), mean(&nat), adds);
        rho_stats[label] = json!({"n": v.len(), "steps_mean": mean(&v), "steps_sd": sd(&v), "steps_se": se(&v), "s_harness_mean": mean(&h), "s_tuned_mean": mean(&t), "native_per_sqrt_l": mean(&nat), "fp_adds_per_step": mean(&st.fp_adds_per_step)});
    }
    report["rho_instruments"] = rho_stats;

    // ---------------- per-prime verdicts under alternative references ----------------
    say!("");
    say!("PER-PRIME RATIOS under alternative references (study rule: geo-mean over seeds / reference mean, 20000-resample percentile bootstrap; 'fixed' references carry no run uncertainty)");
    let mut per_prime = vec![];
    for p in new_primes {
        let cs = &cover[&p];
        let ss: Vec<f64> = cs.iter().map(|c| c.s).collect();
        let sqrt_l = cs[0].l.sqrt();
        let mut e = json!({"p": p, "cover_S_per_seed": ss, "cover_S_geo_mean": gmean(&ss),
            "cover_f4_native_per_sqrt_l": cs.iter().map(|c| c.f4_native as f64 / c.l.sqrt()).collect::<Vec<_>>(),
            "cover_residual_ratio": cs.iter().map(|c| c.residual_ratio).collect::<Vec<_>>(),
            "cover_base": cs.iter().map(|c| c.base).collect::<Vec<_>>(),
            "cover_cross_checked": cs.iter().map(|c| c.cross_checked).collect::<Vec<_>>(),
            "sqrt_l": sqrt_l});
        let refs: Vec<(&str, Option<&Arm>, Option<f64>)> = vec![
            ("R1 rhodp this prime (study primary)", Some(&rhodp[&p]), None),
            ("R2 rhodp pooled 43 runs (study, all primes incl. calibration)", Some(&rhodp_all), None),
            ("R2b rhodp pooled 27 runs (study, new primes only)", Some(&rhodp_new), None),
            ("R3 plain rho theory 1.2733 (fixed, model)", None, Some(exp_plain_r32)),
            ("R4 rhostrong this prime, steps/sqrt(l) (study secondary)", Some(&strong[&p].steps), None),
            ("R5 rhostrong negation pooled (study 18 + red-team 251), steps/sqrt(l)", Some(&strong_pool.steps), None),
            ("R6 negation rho theory 0.8880 (fixed, model)", None, Some(exp_neg_r256)),
            ("R7 rhostrong negation pooled, TUNED F_p-multiplication unit (s_tuned)", Some(&strong_pool.tuned), None),
        ];
        let mut rv = json!({});
        for (name, arm, fixed) in refs {
            if let Some(a) = arm {
                if a.n() == 0 {
                    continue;
                }
            }
            let v = ratio_verdict(&ss, arm, fixed, &mut rng);
            say!("p={p} {name}: ref mean {:.4} (n {}) ratio {:.3} [{:.3}, {:.3}] -> {}",
                v["reference_mean"].as_f64().unwrap(), v["reference_n"], v["ratio_geo_mean"].as_f64().unwrap(),
                v["ci95_boot"][0].as_f64().unwrap(), v["ci95_boot"][1].as_f64().unwrap(), v["verdict"].as_str().unwrap());
            rv[name] = v;
        }
        e["references"] = rv;
        // ---- section 5a band with the counters that exist ----
        // cover native: f4_native_ops (emitted; a lower bound: pair selection, assembly, sorting, F_p additions not counted)
        // rhodp native: derived 1 hash per step + DP table ops per 2^18 steps (study's derivation)
        // rhostrong native: emitted (canon_cmp + hash + dp_check + scalar_ops + table_ops + cycle_cmp)
        let cover_native = mean(&cs.iter().map(|c| c.f4_native as f64 / c.l.sqrt()).collect::<Vec<_>>());
        let cover_s = gmean(&ss);
        let dp_s = rhodp[&p].mean();
        let dp_native = dp_s * (1.0 + 1.0 / (1u64 << 18) as f64);
        let dp_native4 = dp_s * (4.0 + 1.0 / (1u64 << 18) as f64);
        let st_s = strong[&p].harness.mean();
        let st_native = strong[&p].native.mean();
        let mut band = vec![];
        for w in [0.0, 0.1, 1.0] {
            let c = cover_s + w * cover_native;
            let d = dp_s + w * dp_native;
            let d4 = dp_s + w * dp_native4;
            let s = st_s + w * st_native;
            say!("p={p} 5a band w={w}: cover S_w {c:.3} (native {cover_native:.2}/sqrt l, lower bound) | rhodp S_w {d:.3} (4/step convention {d4:.3}) | rhostrong S_w {s:.3} | cover/rhodp {:.3} ({:.3}) | cover/rhostrong {:.3}", c / d, c / d4, c / s);
            band.push(json!({"w": w, "cover": c, "rhodp": d, "rhodp_native4": d4, "rhostrong": s, "cover_over_rhodp": c / d, "cover_over_rhodp_native4": c / d4, "cover_over_rhostrong": c / s}));
        }
        e["band_5a"] = json!(band);
        // ---- fair-reference crossover extrapolation (model): slope -2.10 (study fit A) anchored at this prime
        let slope = -2.098;
        let mut cross = json!({});
        for (name, r) in [
            ("rhodp pooled", rhodp_all.mean()),
            ("plain theory", exp_plain_r32),
            ("negation pooled steps", if strong_pool.steps.n() > 0 { strong_pool.steps.mean() } else { strong_new.steps.mean() }),
            ("negation tuned unit", if strong_pool.tuned.n() > 0 { strong_pool.tuned.mean() } else { strong_new.tuned.mean() }),
        ] {
            let ratio = cover_s / r;
            let pstar = p as f64 * ratio.powf(-1.0 / slope);
            cross[name] = json!({"ratio_at_p": ratio, "p_star_model": pstar});
            say!("p={p} crossover MODEL vs {name} (ref {r:.3}): ratio {ratio:.3} -> p* = {pstar:.0} (slope {slope}, extrapolation)");
        }
        e["crossover_model"] = cross;
        say!("");
        per_prime.push(e);
    }
    report["per_prime"] = json!(per_prime);

    // ---------------- cover variance: S against relation yield ----------------
    let all: Vec<&Cover> = cover.values().flatten().filter(|c| new_primes.contains(&(c.p as u64))).collect();
    let xs: Vec<f64> = all.iter().map(|c| c.residual_ratio.ln()).collect();
    let ys: Vec<f64> = all.iter().map(|c| (c.s * (c.p / 1451.0).powi(2)).ln()).collect(); // p^2-normalised S
    let (mx, my) = (mean(&xs), mean(&ys));
    let cov: f64 = xs.iter().zip(&ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let corr = cov / (xs.iter().map(|x| (x - mx).powi(2)).sum::<f64>() * ys.iter().map(|y| (y - my).powi(2)).sum::<f64>()).sqrt();
    say!("COVER VARIANCE: 9 new-prime runs, corr(ln residual_ratio, ln S*(p/1451)^2) = {corr:.3}; residual_ratio range {:.3}-{:.3}; S range {:.3}-{:.3}; cross_checked all zero: {}",
        all.iter().map(|c| c.residual_ratio).fold(f64::INFINITY, f64::min), all.iter().map(|c| c.residual_ratio).fold(0.0, f64::max),
        all.iter().map(|c| c.s).fold(f64::INFINITY, f64::min), all.iter().map(|c| c.s).fold(0.0, f64::max),
        all.iter().all(|c| c.cross_checked == 0));
    let shares: Vec<(f64, f64, f64)> = all.iter().map(|c| (c.f4_muls as f64 / c.total_muls as f64, c.lin_muls as f64 / c.total_muls as f64, c.la_muls as f64 / c.total_muls as f64)).collect();
    say!("COVER PHASE SHARES (new primes): f4 {:.3}, lin {:.3}, LA {:.5} of total_muls (means)", mean(&shares.iter().map(|s| s.0).collect::<Vec<_>>()), mean(&shares.iter().map(|s| s.1).collect::<Vec<_>>()), mean(&shares.iter().map(|s| s.2).collect::<Vec<_>>()));
    report["cover_variance"] = json!({"corr_ln_residual_ratio_ln_S_p2": corr, "n": all.len()});
    report["controls"] = ctrl;
    report["labels"] = json!("measured: study and control records; derived: means, ratios, bootstrap intervals, Welch; model: random-walk expectations, crossover extrapolations");
    fs::create_dir_all(out).unwrap();
    fs::write(Path::new(out).join("analysis.json"), serde_json::to_string_pretty(&report).unwrap()).unwrap();
    fs::write(Path::new(out).join("analysis.txt"), txt).unwrap();
}
