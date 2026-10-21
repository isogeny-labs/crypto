//! E5b of research/generator_effect_ic_20261019/PLAN.md: a cost *model* of
//! the known-log random-scalar base, the abscissa baseline, BSGS at equal
//! memory and rho, projected to the group orders of P-192 and P-224.
//!
//! `cargo run --release --example generator_effect_e5b -- DRYRUN_RECORDS_DIR OUT.json`
//!
//! The constants come from dry run 2's phase records (sessions
//! ECBS1h0331ce29276e and ECBS1hd98aace0ae34, 24-bit curves, measured counts);
//! the structure (pair table `B(B+1)`, hit rate `2B(B+1)/r`, relations
//! `f_R·B`, BSGS giant stride `2M+1`) is derived in E5.md.  Nothing here is a
//! measurement at 192 or 224 bits, and nothing is a feasibility claim.
use crypto_lib::ecc::curve::CurveParams;
use serde_json::{json, Value};
use std::io::BufRead;

const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];
/// Kuhn–Struik (2001): T keys by parallel collision search with shared
/// distinguished points cost about `sqrt(2 r T)` steps against
/// `sqrt(pi r / 2)` for one, a factor `sqrt(4T/pi) = 1.128 sqrt(T)`.
const KS: f64 = std::f64::consts::FRAC_2_SQRT_PI;

#[derive(Default, Clone)]
struct Mean {
    sum: f64,
    n: f64,
}
impl Mean {
    fn add(&mut self, v: f64) {
        self.sum += v;
        self.n += 1.0;
    }
    fn get(&self) -> f64 {
        self.sum / self.n
    }
}

fn phase<'a>(rec: &'a Value, name: &str) -> &'a Value {
    rec["phases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"] == name)
        .unwrap_or_else(|| panic!("phase {name}"))
}

fn num(v: &Value) -> f64 {
    v.as_f64().unwrap_or(0.0)
}

/// Constants calibrated on the dry-run records (both curves pooled).
#[derive(Debug, Clone)]
struct Calib {
    /// GAE per scalar multiplication per bit of r.
    c_sm: f64,
    /// Per-key fixed overhead of a relation walk (jump table, starts), in scalar-multiplication units.
    f_sm: f64,
    /// Unpriced operations (lookups + target-guard probes) per trial.
    unpriced_per_trial: f64,
    /// Group additions per trial (walk step).
    adds_per_trial: f64,
    /// Verify GAE per key, in scalar-multiplication units.
    verify_sm: f64,
    /// Abscissa base: relations collected per base element.
    f_r: f64,
    /// Abscissa base: LA row operations per (B · relations).
    k_la: f64,
    /// Abscissa base: native ops (Legendre + square roots) per base element.
    fb_native_per_b: f64,
    /// Measured hit rate over the model hit rate 2B(B+1)/r, random-scalar base.
    hit_calibration: f64,
    /// Rho-neg S at each weight.
    s_rho: [f64; 3],
    /// bsgs-neg-B: measured giant steps over the model mean r/(2(2M+1)).
    giant_calibration: f64,
    runs: usize,
}

fn calibrate(dir: &str) -> (Calib, Vec<Value>) {
    let mut c_sm = Mean::default();
    let mut f_sm = Mean::default();
    let mut upt = Mean::default();
    let mut apt = Mean::default();
    let mut vsm = Mean::default();
    let mut f_r = Mean::default();
    let mut k_la = Mean::default();
    let mut fbn = Mean::default();
    let mut rho: [Mean; 3] = Default::default();
    let mut giant = Mean::default();
    let mut runs = 0usize;
    let mut sources = Vec::new();
    let mut base_size: Vec<(f64, f64)> = Vec::new();
    for c in ["c1", "c2"] {
        let path = format!("{dir}/d2-{c}-phase-records.jsonl");
        let f = std::fs::File::open(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        let mut n_here = 0usize;
        for line in std::io::BufReader::new(f).lines() {
            let rec: Value = serde_json::from_str(&line.unwrap()).unwrap();
            if rec["warmup"] == true || rec["outcome"]["status"] != "verified" {
                continue;
            }
            n_here += 1;
            let r = num(&rec["workload"]["curve_spec"]["r"]);
            let lr = r.log2();
            let sqrt_r = r.sqrt();
            match rec["arm"].as_str().unwrap() {
                "ic-knownlog-random" => {
                    let fb = phase(&rec, "factor_base");
                    let b = num(&fb["scalar_mults"]);
                    if !base_size.iter().any(|(rr, _)| *rr == r) {
                        base_size.push((r, b));
                    }
                    let sm = num(&fb["gae"]) / b;
                    c_sm.add(sm / lr);
                    let rel = phase(&rec, "relations");
                    let nat = &rel["native"];
                    let trials = num(&nat["trials"]);
                    let walk = num(&nat["walk_steps"]);
                    f_sm.add((num(&rel["gae"]) - walk) / sm);
                    apt.add(walk / trials);
                    upt.add((num(&nat["lookups"]) + num(&nat["target_guard_probes"])) / trials);
                    vsm.add(num(&phase(&rec, "verify")["gae"]) / sm);
                }
                "ic-abscissa" => {
                    let b: f64 = rec["method"]["params"]["factor_base"]
                        .as_str()
                        .unwrap()
                        .rsplit('=')
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();
                    let rel = num(&phase(&rec, "relations")["native"]["relations"]);
                    f_r.add(rel / b);
                    k_la.add(num(&phase(&rec, "linear_algebra")["native"]["row_ops"]) / (b * rel));
                    let n = &phase(&rec, "factor_base")["native"];
                    fbn.add((num(&n["legendre_symbols"]) + num(&n["sqrt_solves"])) / b);
                }
                "bsgs-neg-B" => {
                    let m: f64 = rec["method"]["params"]["baby"]
                        .as_str()
                        .unwrap()
                        .parse()
                        .unwrap();
                    giant.add(num(&rec["counters"]["giant_steps"]) / (r / (2.0 * (2.0 * m + 1.0))));
                }
                "rho-neg" => {
                    let g = num(&rec["cost"]["total_gae"]);
                    let can = num(&rec["counters"]["canonicalisations_uncharged"]);
                    for (k, w) in WEIGHTS.iter().enumerate() {
                        rho[k].add((g + w * can) / sqrt_r);
                    }
                }
                _ => {}
            }
        }
        runs += n_here;
        sources.push(json!({"path": path, "measured_runs": n_here}));
    }
    // Hit-rate calibration from summary2.json (yield block, 3000 targets per curve).
    let s2: Value =
        serde_json::from_str(&std::fs::read_to_string(format!("{dir}/../summary2.json")).unwrap())
            .unwrap();
    let mut hit = Mean::default();
    for cv in s2["curves"].as_array().unwrap() {
        let r = num(&cv["r"]);
        let y = &cv["yield_block"]["ic-knownlog-random"];
        let bsz = base_size
            .iter()
            .find(|(rr, _)| *rr == r)
            .expect("B for this r")
            .1;
        hit.add(num(&y["yield"]) / (2.0 * bsz * (bsz + 1.0) / r));
    }
    sources.push(json!({"path": format!("{dir}/../summary2.json"), "use": "hit-rate calibration"}));
    (
        Calib {
            c_sm: c_sm.get(),
            f_sm: f_sm.get(),
            unpriced_per_trial: upt.get(),
            adds_per_trial: apt.get(),
            verify_sm: vsm.get(),
            f_r: f_r.get(),
            k_la: k_la.get(),
            fb_native_per_b: fbn.get(),
            hit_calibration: hit.get(),
            s_rho: [rho[0].get(), rho[1].get(), rho[2].get()],
            giant_calibration: giant.get(),
            runs,
        },
        sources,
    )
}

struct Model<'a> {
    c: &'a Calib,
    r: f64,
    lr: f64,
    w: f64,
    wi: usize,
}

impl Model<'_> {
    fn sm(&self) -> f64 {
        self.c.c_sm * self.lr
    }
    fn hit(&self, b: f64) -> f64 {
        self.c.hit_calibration * 2.0 * b * (b + 1.0) / self.r
    }
    fn trial(&self) -> f64 {
        self.c.adds_per_trial + self.w * self.c.unpriced_per_trial
    }
    fn table(&self, b: f64) -> f64 {
        // B(B+1) additions; inserts priced at w as BSGS's are (dry run 2 left IC inserts uncounted).
        b * (b + 1.0) * (1.0 + self.w)
    }
    /// Online GAE of one further key under a known-log or solved base.
    fn per_key(&self, b: f64) -> f64 {
        (self.c.f_sm + self.c.verify_sm) * self.sm() + self.trial() / self.hit(b)
    }
    fn knownlog_setup(&self, b: f64) -> f64 {
        b * self.sm() + self.table(b)
    }
    fn knownlog(&self, b: f64, t: f64) -> f64 {
        self.knownlog_setup(b) + t * self.per_key(b)
    }
    fn abscissa_setup(&self, b: f64) -> f64 {
        let rel = self.c.f_r * b;
        self.w * self.c.fb_native_per_b * b
            + self.table(b)
            + (rel - 1.0) * self.trial() / self.hit(b)
            + self.w * self.c.k_la * b * rel
    }
    fn abscissa(&self, b: f64, t: f64) -> f64 {
        self.abscissa_setup(b) + t * self.per_key(b)
    }
    /// bsgs.negation_table with M baby classes: stride 2M+1, mean r/(2(2M+1)) giant steps.
    fn bsgs(&self, m: f64, t: f64) -> f64 {
        self.sm() + m * (1.0 + self.w) + t * self.bsgs_per_key(m)
    }
    fn bsgs_per_key(&self, m: f64) -> f64 {
        self.sm() + self.c.giant_calibration * (1.0 + self.w) * self.r / (2.0 * (2.0 * m + 1.0))
    }
    fn rho_one(&self) -> f64 {
        self.c.s_rho[self.wi] * self.r.sqrt()
    }
    fn rho_indep(&self, t: f64) -> f64 {
        t * self.rho_one()
    }
    fn rho_ks(&self, t: f64) -> f64 {
        if t <= 1.0 {
            self.rho_one()
        } else {
            KS * t.sqrt() * self.rho_one()
        }
    }
}

/// Golden-section minimum of `f` over log2 B in [lo, hi].
fn argmin_log2(f: impl Fn(f64) -> f64, lo: f64, hi: f64) -> f64 {
    let (mut a, mut b) = (lo, hi);
    let g = (5f64.sqrt() - 1.0) / 2.0;
    for _ in 0..200 {
        let c = b - g * (b - a);
        let d = a + g * (b - a);
        if f(c) < f(d) {
            b = d;
        } else {
            a = c;
        }
    }
    (a + b) / 2.0
}

/// Smallest T (log2, continuous) with `arm(T)/T ≤ rho(T)/T`, searched in [0, 256].
fn break_even(arm: impl Fn(f64) -> f64, rho: impl Fn(f64) -> f64) -> Option<f64> {
    let ok = |lt: f64| {
        let t = 2f64.powf(lt);
        arm(t) <= rho(t)
    };
    if ok(0.0) {
        return Some(0.0);
    }
    if !ok(256.0) {
        return None;
    }
    let (mut lo, mut hi) = (0.0, 256.0);
    for _ in 0..200 {
        let mid = (lo + hi) / 2.0;
        if ok(mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    Some(hi)
}

fn l2(x: f64) -> f64 {
    x.log2()
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (calib, sources) = calibrate(&a[0]);
    let t_grid: Vec<f64> = vec![0.0, 8.0, 16.0, 32.0, 48.0, 64.0];
    let mut curves = Vec::new();
    for cp in [CurveParams::p192(), CurveParams::p224()] {
        let r: f64 = cp.n.to_string().parse().unwrap();
        let lr = r.log2();
        let mut by_w = Vec::new();
        for (wi, &w) in WEIGHTS.iter().enumerate() {
            let m = Model {
                c: &calib,
                r,
                lr,
                w,
                wi,
            };
            let s = |gae: f64| gae / r.sqrt();
            // (a) the plan's equal B: ceil(r^(1/3)).
            let b = r.cbrt().ceil();
            let e = b * (b + 1.0);
            let kl1 = m.knownlog(b, 1.0);
            let ab1 = m.abscissa(b, 1.0);
            let bs1 = m.bsgs(e, 1.0);
            let equal_b = json!({
                "log2_B": l2(b),
                "log2_table_entries": l2(e),
                "log2_online_probes_per_key": {
                    "knownlog_random": l2(1.0 / m.hit(b)),
                    "abscissa_first_key": l2(calib.f_r * b / m.hit(b)),
                    "abscissa_further_key": l2(1.0 / m.hit(b)),
                    "bsgs_equal_memory": l2(calib.giant_calibration * r / (2.0 * (2.0 * e + 1.0))),
                },
                "log2_S_T1": {
                    "knownlog_random": l2(s(kl1)),
                    "abscissa": l2(s(ab1)),
                    "bsgs_equal_memory": l2(s(bs1)),
                    "rho_neg": l2(s(m.rho_one())),
                },
                "log2_distance_to_rho_T1": {
                    "knownlog_random": l2(kl1 / m.rho_one()),
                    "abscissa": l2(ab1 / m.rho_one()),
                    "bsgs_equal_memory": l2(bs1 / m.rho_one()),
                },
                "log2_break_even_T": {
                    "knownlog_random_vs_rho_indep": break_even(|t| m.knownlog(b, t), |t| m.rho_indep(t)),
                    "knownlog_random_vs_rho_ks": break_even(|t| m.knownlog(b, t), |t| m.rho_ks(t)),
                    "abscissa_vs_rho_indep": break_even(|t| m.abscissa(b, t), |t| m.rho_indep(t)),
                    "abscissa_vs_rho_ks": break_even(|t| m.abscissa(b, t), |t| m.rho_ks(t)),
                    "bsgs_equal_memory_vs_rho_indep": break_even(|t| m.bsgs(e, t), |t| m.rho_indep(t)),
                    "bsgs_equal_memory_vs_rho_ks": break_even(|t| m.bsgs(e, t), |t| m.rho_ks(t)),
                },
                // Same table; the known-log arm adds B scalar multiplications and probes more per key,
                // so it never catches BSGS at equal memory.  Totals near 2^128 cannot show the
                // difference in f64, so the per-key online costs are compared directly.
                "online_per_key_knownlog_over_bsgs_equal_memory":
                    m.per_key(b) / m.bsgs_per_key(e),
            });
            // (b) each arm at its own optimal size for T keys.
            let mut opt = Vec::new();
            for &lt in &t_grid {
                let t = 2f64.powf(lt);
                let lb_kl = argmin_log2(|x| m.knownlog(2f64.powf(x), t), 1.0, lr);
                let lb_ab = argmin_log2(|x| m.abscissa(2f64.powf(x), t), 1.0, lr);
                let lm_bs = argmin_log2(|x| m.bsgs(2f64.powf(x), t), 1.0, lr);
                let (bk, ba, mb) = (2f64.powf(lb_kl), 2f64.powf(lb_ab), 2f64.powf(lm_bs));
                let per = |tot: f64| tot / t;
                let kl = per(m.knownlog(bk, t));
                let ab = per(m.abscissa(ba, t));
                let bs = per(m.bsgs(mb, t));
                let ek = bk * (bk + 1.0);
                let bs_eq = per(m.bsgs(ek, t));
                let ri = per(m.rho_indep(t));
                let rk = per(m.rho_ks(t));
                opt.push(json!({
                    "log2_T": lt,
                    "ks_regime_ok": lt <= lr / 4.0,
                    "knownlog_random": {
                        "log2_B": lb_kl, "log2_table_entries": l2(ek),
                        "log2_online_probes_per_key": l2(1.0 / m.hit(bk)),
                        "log2_S_per_key": l2(s(kl)),
                    },
                    "abscissa": {
                        "log2_B": lb_ab, "log2_table_entries": l2(ba * (ba + 1.0)),
                        "log2_S_per_key": l2(s(ab)),
                    },
                    "bsgs_opt": {"log2_M": lm_bs, "log2_S_per_key": l2(s(bs))},
                    "bsgs_equal_memory_to_knownlog": {"log2_M": l2(ek), "log2_S_per_key": l2(s(bs_eq))},
                    "rho_indep": {"log2_S_per_key": l2(s(ri))},
                    "rho_ks": {"log2_S_per_key": l2(s(rk))},
                    "log2_distance": {
                        "knownlog_random_to_rho_indep": l2(kl / ri),
                        "knownlog_random_to_rho_ks": l2(kl / rk),
                        "abscissa_to_rho_ks": l2(ab / rk),
                        "knownlog_random_to_bsgs_equal_memory": l2(kl / bs_eq),
                        "bsgs_opt_to_rho_ks": l2(bs / rk),
                    },
                }));
            }
            // Break-even with each arm at its own optimal size for every T.
            let kl_opt = |t: f64| {
                let lb = argmin_log2(|x| m.knownlog(2f64.powf(x), t), 1.0, lr);
                m.knownlog(2f64.powf(lb), t)
            };
            let bs_opt = |t: f64| {
                let lm = argmin_log2(|x| m.bsgs(2f64.powf(x), t), 1.0, lr);
                m.bsgs(2f64.powf(lm), t)
            };
            let ab_opt = |t: f64| {
                let lb = argmin_log2(|x| m.abscissa(2f64.powf(x), t), 1.0, lr);
                m.abscissa(2f64.powf(lb), t)
            };
            let opt_break_even = json!({
                "knownlog_random_vs_rho_indep": break_even(kl_opt, |t| m.rho_indep(t)),
                "knownlog_random_vs_rho_ks": break_even(kl_opt, |t| m.rho_ks(t)),
                "abscissa_vs_rho_indep": break_even(ab_opt, |t| m.rho_indep(t)),
                "abscissa_vs_rho_ks": break_even(ab_opt, |t| m.rho_ks(t)),
                "bsgs_vs_rho_indep": break_even(bs_opt, |t| m.rho_indep(t)),
                "bsgs_vs_rho_ks": break_even(bs_opt, |t| m.rho_ks(t)),
            });
            by_w.push(json!({
                "w": w,
                "equal_B": equal_b,
                "optimal_B": opt,
                "optimal_B_log2_break_even_T": opt_break_even,
            }));
        }
        curves.push(json!({"curve": cp.name, "log2_r": lr, "by_w": by_w}));
    }
    let doc = json!({
        "schema": "generator_effect_e5b/v1",
        "status": "MODEL, not a measurement and not a feasibility claim; constants from a 24-bit scratch dry run (PROTOCOL.md not frozen)",
        "sources": sources,
        "calibration": {
            "measured_runs": calib.runs,
            "c_sm_gae_per_scalar_mult_per_bit": calib.c_sm,
            "walk_overhead_scalar_mults_per_key": calib.f_sm,
            "verify_scalar_mults_per_key": calib.verify_sm,
            "adds_per_trial": calib.adds_per_trial,
            "unpriced_ops_per_trial": calib.unpriced_per_trial,
            "abscissa_relations_per_base_element": calib.f_r,
            "abscissa_la_row_ops_per_B_times_relations": calib.k_la,
            "abscissa_fb_native_ops_per_B": calib.fb_native_per_b,
            "hit_rate_measured_over_model": calib.hit_calibration,
            "rho_neg_s_at_w": calib.s_rho,
            "bsgs_giant_steps_measured_over_model": calib.giant_calibration,
            "ks_factor": KS,
        },
        "weights": WEIGHTS,
        "curves": curves,
    });
    std::fs::write(&a[1], serde_json::to_string_pretty(&doc).unwrap() + "\n").unwrap();
}
