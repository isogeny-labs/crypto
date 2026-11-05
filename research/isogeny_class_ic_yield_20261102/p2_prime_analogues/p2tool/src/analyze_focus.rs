//! `p2tool analyze-focus`: the frozen statistics and decision rule of P2 follow-up 1
//! (FOLLOWUP_c32A_m3.md sections 6 to 8). One pre-specified test (T1, curve 80 against the
//! pooled comparators), the iso-model tests T2 and T3, the positive and negative controls,
//! and the mechanical classification. Counted metrics only (relations / attempts).
//!
//! Groups of main-arm cells: P = primary (workload 0), C = the 15 comparators (workloads 1..=15),
//! I = the 8 iso models (workloads 16..=23). Warm-up rows are excluded exactly as in `analyze.rs`
//! (`warmup: true`), curves are identified by `(a, b)` of `workload.curve_spec`, arms by name.
//! `analyze.rs` is frozen and untouched; the splitmix64 generator is reproduced here.
use crate::arg;
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub const PERMS: usize = 20000;
pub const SEED: u64 = 20261102;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// One cell: relations found and attempts (targets_tried) of one (curve, round, target) run.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub rel: f64,
    pub att: f64,
}

/// Two-sided normal p-value 2(1 - Phi(|z|)) = erfc(|z| / sqrt 2).
pub fn p_two_sided(z: f64) -> f64 {
    erfc(z.abs() / std::f64::consts::SQRT_2)
}
/// erfc(x) for x >= 0: Maclaurin series of erf below 2.5 (absolute error ~1e-12), Lentz
/// continued fraction above.
fn erfc(x: f64) -> f64 {
    if x < 2.5 {
        let mut term = x;
        let mut sum = x;
        let mut n = 0.0;
        while term.abs() > 1e-18 * sum.abs().max(1e-300) && n < 200.0 {
            n += 1.0;
            term *= -x * x / n;
            sum += term / (2.0 * n + 1.0);
        }
        (1.0 - 2.0 / std::f64::consts::PI.sqrt() * sum).max(0.0)
    } else {
        // erfc(x) = exp(-x^2)/sqrt(pi) * 1/(x + (1/2)/(x + 1/(x + (3/2)/(x + 2/(x + ...)))))
        let mut f = 0.0;
        for k in (1..=120).rev() {
            f = (k as f64 / 2.0) / (x + f);
        }
        (-x * x).exp() / std::f64::consts::PI.sqrt() / (x + f)
    }
}

fn sums(cells: &[Cell]) -> (f64, f64) {
    (cells.iter().map(|c| c.rel).sum(), cells.iter().map(|c| c.att).sum())
}

/// log-ratio test of group x against group y: l = ln(y_x/y_y), SE = sqrt(1/K_x + 1/K_y).
fn ratio_test(x: &[Cell], y: &[Cell]) -> Value {
    let (kx, ax) = sums(x);
    let (ky, ay) = sums(y);
    let l = ((kx / ax) / (ky / ay)).ln();
    let se = (1.0 / kx + 1.0 / ky).sqrt();
    let z = l / se;
    json!({"K_x": kx, "A_x": ax, "K_y": ky, "A_y": ay, "cells_x": x.len(), "cells_y": y.len(),
           "y_x": kx / ax, "y_y": ky / ay, "ratio": l.exp(), "log_ratio": l, "se": se, "z": z, "p_z": p_two_sided(z),
           "ci95": [(l - 1.96 * se).exp(), (l + 1.96 * se).exp()]})
}
fn f(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or(f64::NAN)
}

/// Permutation p of T1: 20000 reassignments of the |P| curve-80 labels among the |P| + |C| cells
/// (Fisher-Yates, as `permuted` in analyze.rs), statistic |l|, add-one p.
fn perm_p(p: &[Cell], c: &[Cell], perms: usize, seed: u64) -> f64 {
    let all: Vec<Cell> = p.iter().chain(c.iter()).cloned().collect();
    let l_of = |pp: &[Cell], cc: &[Cell]| {
        let (kp, ap) = sums(pp);
        let (kc, ac) = sums(cc);
        ((kp / ap) / (kc / ac)).ln().abs()
    };
    let obs = l_of(p, c);
    let mut rng = Rng(seed);
    let mut v = all.clone();
    let mut hit = 0usize;
    for _ in 0..perms {
        for i in (1..v.len()).rev() {
            let j = rng.below(i + 1);
            v.swap(i, j);
        }
        // tolerance: exactly-equal statistics (e.g. identical labels) count as >=
        if l_of(&v[..p.len()], &v[p.len()..]) >= obs - 1e-12 {
            hit += 1;
        }
    }
    (hit as f64 + 1.0) / (perms as f64 + 1.0)
}

/// Verdict codes of section 8.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    NotRun,
    NotInformative,
    NotReplicated,
    CurveEffect,
    PresentationEffect,
    Inconclusive,
}
impl Verdict {
    pub fn code(&self) -> &'static str {
        match self {
            Verdict::NotRun => "not_run_incomplete",
            Verdict::NotInformative => "not_informative_or_inconclusive_overdispersed",
            Verdict::NotReplicated => "not_replicated_chance_winners_curse",
            Verdict::CurveEffect => "curve_effect_escalate_to_mechanism_test",
            Verdict::PresentationEffect => "factor_base_presentation_effect",
            Verdict::Inconclusive => "inconclusive",
        }
    }
}

/// Inputs of the mechanical classification (section 8, steps 1 to 5).
pub struct DecisionInputs {
    pub pc_ratio: f64,
    pub pc_p_z: f64,
    pub d: f64,
    pub r_p: f64,
    pub decision_p: f64,
    pub r_i: f64,
    pub p_z_i: f64,
    pub t3_p_z: f64,
    pub y_p_gt_y_i: bool,
}
/// Applied in the order of section 8; returns the verdict and the step that decided it.
pub fn classify(x: &DecisionInputs) -> (Verdict, u32) {
    let pc_ok = x.pc_ratio >= 1.10 && x.pc_p_z < 0.05;
    if !pc_ok || !(x.d <= 2.0) {
        return (Verdict::NotInformative, 1);
    }
    let replicated = x.r_p >= 1.05 && x.decision_p < 0.05;
    if !replicated {
        return (Verdict::NotReplicated, 2);
    }
    if x.r_i >= 1.05 && x.p_z_i < 0.05 {
        return (Verdict::CurveEffect, 3);
    }
    if x.r_i < 1.05 && x.t3_p_z < 0.05 && x.y_p_gt_y_i {
        return (Verdict::PresentationEffect, 4);
    }
    (Verdict::Inconclusive, 5)
}

/// All statistics from the cells. `main[w]` = main-arm cells of workload w (0 primary, 1..=15
/// comparators, 16..=23 iso); `pc` = pc-arm cells of the primary. `expected_cells` (targets x
/// rounds) marks a session incomplete when any used curve has another count.
pub fn analyse(main: &[Vec<Cell>], pc: &[Cell], n_comp: usize, n_iso: usize, expected_cells: Option<usize>, perms: usize, seed: u64) -> Value {
    assert_eq!(main.len(), 1 + n_comp + n_iso);
    let p = &main[0];
    let comp: Vec<&Vec<Cell>> = main[1..=n_comp].iter().collect();
    let iso: Vec<&Vec<Cell>> = main[1 + n_comp..].iter().collect();
    let flat = |v: &[&Vec<Cell>]| -> Vec<Cell> { v.iter().flat_map(|x| x.iter().cloned()).collect() };
    let c_all = flat(&comp);
    let i_all = flat(&iso);
    let counts: Vec<usize> = main.iter().map(|v| v.len()).collect();
    let mut incomplete: Vec<String> = vec![];
    if main.iter().any(|v| v.is_empty()) || pc.is_empty() {
        incomplete.push("a workload has no main cells or the primary has no pc cells".into());
    } else if let Some(e) = expected_cells {
        for (w, &k) in counts.iter().enumerate() {
            if k != e {
                incomplete.push(format!("workload {w}: {k} main cells, expected {e}"));
            }
        }
        if pc.len() != e {
            incomplete.push(format!("primary pc arm: {} cells, expected {e}", pc.len()));
        }
    }
    let mut out = json!({"cells_per_workload_main": counts, "pc_cells_primary": pc.len(), "expected_cells_per_curve": expected_cells,
                         "incomplete": incomplete, "permutations": perms, "seed": seed});
    let usable = !main.iter().any(|v| v.is_empty() || sums(v).0 <= 0.0) && !pc.is_empty() && sums(pc).0 > 0.0;
    if !usable {
        out["verdict"] = json!(Verdict::NotRun.code());
        out["decision"] = json!({"verdict": Verdict::NotRun.code(), "step": 0, "reason": "a group has no data or zero relations"});
        return out;
    }
    // T1
    let mut t1 = ratio_test(p, &c_all);
    let pp = perm_p(p, &c_all, perms, seed);
    let dec_p = f(&t1, "p_z").max(pp);
    t1["p_perm"] = json!(pp);
    t1["decision_p"] = json!(dec_p);
    t1["replicated"] = json!(f(&t1, "ratio") >= 1.05 && dec_p < 0.05);
    out["T1_primary_vs_comparators"] = t1.clone();
    // T2, T3
    let mut t2 = ratio_test(&i_all, &c_all);
    t2["present_in_iso"] = json!(f(&t2, "ratio") >= 1.05 && f(&t2, "p_z") < 0.05);
    out["T2_iso_vs_comparators"] = t2.clone();
    let t3 = ratio_test(p, &i_all);
    out["T3_primary_vs_iso"] = t3.clone();
    // per-iso-model descriptive
    let yp = sums(p).0 / sums(p).1;
    let per_iso: Vec<Value> = iso
        .iter()
        .enumerate()
        .map(|(k, v)| {
            let t = ratio_test(v, &c_all);
            let y = sums(v).0 / sums(v).1;
            json!({"k": k, "relations": sums(v).0, "attempts": sums(v).1, "ratio_to_C": t["ratio"], "z_vs_C": t["z"], "log_ratio_to_primary": (y / yp).ln()})
        })
        .collect();
    let rms = (iso.iter().map(|v| ((sums(v).0 / sums(v).1) / yp).ln().powi(2)).sum::<f64>() / iso.len() as f64).sqrt();
    out["iso_models"] = json!({"per_model": per_iso, "rms_log_ratio_iso_to_primary": rms});
    // positive control: pc cells of primary against main cells of C
    let pcr = ratio_test(pc, &c_all);
    let pc_met = f(&pcr, "ratio") >= 1.10 && f(&pcr, "p_z") < 0.05;
    out["positive_control"] = json!({"test": pcr, "met": pc_met, "rule": "ratio >= 1.10 and p_z < 0.05"});
    // negative control: each comparator against the other 14 pooled
    let mut zs = vec![];
    let mut rows = vec![];
    for (i, v) in comp.iter().enumerate() {
        let rest: Vec<Cell> = comp.iter().enumerate().filter(|(j, _)| *j != i).flat_map(|(_, x)| x.iter().cloned()).collect();
        let t = ratio_test(v, &rest);
        zs.push(f(&t, "z"));
        rows.push(json!({"comparator_position": i, "ratio": t["ratio"], "z": t["z"], "p_z": t["p_z"]}));
    }
    let d = zs.iter().map(|z| z * z).sum::<f64>() / zs.len() as f64;
    let n_sig = zs.iter().filter(|z| z.abs() > 1.96).count();
    out["negative_control"] = json!({"leave_one_out": rows, "n_abs_z_gt_1.96": n_sig, "expected_n": 0.05 * zs.len() as f64,
        "max_abs_z": zs.iter().fold(0.0f64, |a, z| a.max(z.abs())), "D_mean_z2": d, "informative": d <= 2.0, "rule": "D <= 2.0"});
    // decision
    let di = DecisionInputs {
        pc_ratio: f(&pcr, "ratio"), pc_p_z: f(&pcr, "p_z"), d, r_p: f(&t1, "ratio"), decision_p: dec_p,
        r_i: f(&t2, "ratio"), p_z_i: f(&t2, "p_z"), t3_p_z: f(&t3, "p_z"), y_p_gt_y_i: f(&t3, "log_ratio") > 0.0,
    };
    let (v, step) = classify(&di);
    let v = if incomplete.is_empty() { v } else { Verdict::NotRun };
    out["decision"] = json!({"verdict": v.code(), "step": step, "incomplete_session_overrides": !incomplete.is_empty(),
        "inputs": {"pc_ratio": di.pc_ratio, "pc_p_z": di.pc_p_z, "D": di.d, "r_P": di.r_p, "decision_p": di.decision_p, "r_I": di.r_i,
                   "p_z_I": di.p_z_i, "T3_p_z": di.t3_p_z, "y_P_gt_y_I": di.y_p_gt_y_i}});
    out["verdict"] = json!(v.code());
    out
}

pub fn summary(r: &Value) -> String {
    let mut s = String::new();
    s += &format!("verdict: {}\n", r["verdict"].as_str().unwrap_or("?"));
    if let Some(inc) = r["incomplete"].as_array() {
        if !inc.is_empty() {
            s += &format!("INCOMPLETE: {}\n", inc.iter().map(|x| x.as_str().unwrap_or("")).collect::<Vec<_>>().join("; "));
        }
    }
    if r.get("T1_primary_vs_comparators").is_none() {
        return s;
    }
    let t = |k: &str| &r[k];
    let t1 = t("T1_primary_vs_comparators");
    s += &format!("T1 curve 80 vs C: ratio {:.4} (95% {:.4}..{:.4}) z {:.2} p_z {:.3e} p_perm {:.3e} decision p {:.3e} replicated {}\n",
        f(t1, "ratio"), t1["ci95"][0].as_f64().unwrap(), t1["ci95"][1].as_f64().unwrap(), f(t1, "z"), f(t1, "p_z"), f(t1, "p_perm"), f(t1, "decision_p"), t1["replicated"]);
    let t2 = t("T2_iso_vs_comparators");
    s += &format!("T2 iso vs C: ratio {:.4} (95% {:.4}..{:.4}) z {:.2} p_z {:.3e} present {}\n",
        f(t2, "ratio"), t2["ci95"][0].as_f64().unwrap(), t2["ci95"][1].as_f64().unwrap(), f(t2, "z"), f(t2, "p_z"), t2["present_in_iso"]);
    let t3 = t("T3_primary_vs_iso");
    s += &format!("T3 curve 80 vs iso: ratio {:.4} z {:.2} p_z {:.3e}\n", f(t3, "ratio"), f(t3, "z"), f(t3, "p_z"));
    let pc = &t("positive_control")["test"];
    s += &format!("positive control (pc vs C): ratio {:.4} p_z {:.3e} met {}\n", f(pc, "ratio"), f(pc, "p_z"), t("positive_control")["met"]);
    let n = t("negative_control");
    s += &format!("negative control: D {:.3} max|z| {:.2} n|z|>1.96 {} informative {}\n", f(n, "D_mean_z2"), f(n, "max_abs_z"), n["n_abs_z_gt_1.96"], n["informative"]);
    s += &format!("decision step {} -> {}\n", r["decision"]["step"], r["verdict"].as_str().unwrap_or("?"));
    s
}

fn cell_key(r: &Value) -> Option<(u64, u64)> {
    let cs = &r["workload"]["curve_spec"];
    Some((cs["a"].as_u64()?, cs["b"].as_u64()?))
}

pub fn run(args: &[String]) {
    let spec_path = arg(args, "--spec").expect("--spec spec.json");
    let focus_path = arg(args, "--focus").expect("--focus followup1_focus.json");
    let classes_path = arg(args, "--classes").expect("--classes classes.json");
    let out = arg(args, "--out").expect("--out focus_analysis.json");
    let sessions: Vec<String> = args.iter().enumerate().filter(|(_, a)| *a == "--session").filter_map(|(i, _)| args.get(i + 1).cloned()).collect();
    assert!(!sessions.is_empty(), "--session DIR (repeatable)");
    let rd = |p: &str| -> Value { serde_json::from_str(&std::fs::read_to_string(p).unwrap_or_else(|e| panic!("{p}: {e}"))).unwrap_or_else(|e| panic!("{p}: {e}")) };
    let (spec, focus, classes) = (rd(&spec_path), rd(&focus_path), rd(&classes_path));
    let cid = focus["class_id"].as_str().expect("focus class_id");
    let m = focus["m"].as_u64().expect("focus m");
    let cls = classes["classes"].as_array().unwrap().iter().find(|c| c["id"] == cid).expect("class in classes.json");
    // expected (a,b) per workload, from the focus file and classes.json, cross-checked with the spec's list
    let ab = |v: &Value| (v["a"].as_u64().unwrap(), v["b"].as_u64().unwrap());
    let mut want = vec![ab(&focus["primary"])];
    let comps = focus["comparator_indices"].as_array().unwrap();
    for i in comps {
        want.push(ab(&cls["curves"][i.as_u64().unwrap() as usize]));
    }
    for im in focus["iso_models"].as_array().unwrap() {
        want.push(ab(im));
    }
    let (n_comp, n_iso) = (comps.len(), focus["iso_models"].as_array().unwrap().len());
    let spec_curves: Vec<(u64, u64)> = spec["workloads"]["curves"].as_array().expect("spec curves").iter().map(ab).collect();
    if spec_curves != want {
        eprintln!("error: spec workloads.curves differ from the focus design (order or content)");
        std::process::exit(1);
    }
    let mut widx: BTreeMap<(u64, u64), usize> = BTreeMap::new();
    for (i, k) in want.iter().enumerate() {
        if widx.insert(*k, i).is_some() {
            eprintln!("error: duplicate (a,b) among the workloads");
            std::process::exit(1);
        }
    }
    let main_arm = format!("m{m}-main");
    let pc_arm = format!("m{m}-pc");
    let mut main: Vec<Vec<Cell>> = vec![vec![]; want.len()];
    let mut pc: Vec<Cell> = vec![];
    let mut levels: BTreeMap<String, u64> = BTreeMap::new();
    let mut status: BTreeMap<String, u64> = BTreeMap::new();
    for dir in &sessions {
        let text = std::fs::read_to_string(format!("{dir}/records.jsonl")).unwrap_or_else(|e| panic!("{dir}/records.jsonl: {e}"));
        // (round, target, index) keys keep a deterministic cell order for the permutation test
        let mut rows: Vec<(u64, u64, usize, bool, Cell)> = vec![];
        for line in text.lines() {
            let r: Value = serde_json::from_str(line).unwrap();
            if r["warmup"].as_bool().unwrap_or(false) {
                continue;
            }
            let is_main = r["arm"] == main_arm.as_str();
            if !is_main && r["arm"] != pc_arm.as_str() {
                continue;
            }
            let w = match cell_key(&r).and_then(|k| widx.get(&k)) {
                Some(&w) => w,
                None => continue,
            };
            if !is_main && w != 0 {
                continue; // the positive control uses the pc cells of the primary only
            }
            *levels.entry(r["isolation"]["level"].as_str().unwrap_or("?").to_string()).or_default() += 1;
            *status.entry(r["outcome"]["status"].as_str().unwrap_or("?").to_string()).or_default() += 1;
            let c = Cell { rel: r["counters"]["relations_found"].as_f64().unwrap(), att: r["counters"]["targets_tried"].as_f64().unwrap() };
            rows.push((r["round"].as_u64().unwrap(), r["workload"]["target_index"].as_u64().unwrap(), w, is_main, c));
        }
        rows.sort_by_key(|x| (x.0, x.1));
        for (_, _, w, is_main, c) in rows {
            if is_main { main[w].push(c) } else { pc.push(c) }
        }
    }
    let t_per = spec["workloads"]["targets_per_curve"].as_u64().unwrap();
    let rounds = spec["measurement"]["rounds"].as_u64().unwrap();
    let mut rep = analyse(&main, &pc, n_comp, n_iso, Some((t_per * rounds) as usize), PERMS, SEED);
    rep["schema"] = json!("p2.focus_analysis/v1");
    rep["class"] = json!(cid);
    rep["m"] = json!(m);
    rep["sessions"] = json!(sessions);
    rep["spec_target_seed"] = spec["workloads"]["target_seed"].clone();
    rep["spec_measurement_seed"] = spec["measurement"]["seed"].clone();
    rep["isolation_levels_of_rows"] = json!(levels);
    rep["outcome_status_of_rows"] = json!(status);
    rep["groups"] = json!({"P": "curve 80 (workload 0)", "C": format!("{n_comp} comparators"), "I": format!("{n_iso} iso models of curve 80")});
    std::fs::write(&out, serde_json::to_string_pretty(&rep).unwrap()).unwrap();
    print!("{}", summary(&rep));
    eprintln!("wrote {out}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: f64 = 5964.0;
    const Y0: f64 = 0.033006;
    fn cells(y: f64, n: usize) -> Vec<Cell> {
        (0..n).map(|_| Cell { rel: (A * y).round(), att: A }).collect()
    }
    /// Deterministic scenario: relative yield multipliers, comparators with a small fixed spread.
    fn scenario(p: f64, iso: f64, pc: f64, comp_spread: f64) -> Value {
        let mut main = vec![cells(Y0 * p, 48)];
        for i in 0..15 {
            // alternating +-spread multiplies the per-curve yield; mean is ~1
            let s = if i % 2 == 0 { 1.0 + comp_spread } else { 1.0 - comp_spread };
            main.push(cells(Y0 * s, 48));
        }
        for _ in 0..8 {
            main.push(cells(Y0 * iso, 48));
        }
        analyse(&main, &cells(Y0 * pc, 48), 15, 8, Some(48), 2000, SEED)
    }
    fn verdict(v: &Value) -> &str {
        v["verdict"].as_str().unwrap()
    }

    #[test]
    fn normal_tail_values() {
        assert!((p_two_sided(1.96) - 0.04999579).abs() < 1e-7);
        assert!((p_two_sided(0.0) - 1.0).abs() < 1e-12);
        assert!((p_two_sided(3.0) - 0.0026997961).abs() < 1e-9);
        // continued-fraction branch (x >= 2.5 i.e. |z| >= 3.54)
        assert!((p_two_sided(5.0) / 5.733031438e-7 - 1.0).abs() < 1e-6);
        assert!((p_two_sided(4.0) / 6.334248366e-5 - 1.0).abs() < 1e-6);
    }
    #[test]
    fn branch1_positive_control_missed() {
        // effect present everywhere but pc not planted: not informative whatever T1 says
        let v = scenario(1.10, 1.10, 1.0, 0.0);
        assert_eq!(verdict(&v), Verdict::NotInformative.code());
        assert_eq!(v["decision"]["step"], 1);
    }
    #[test]
    fn branch1_overdispersed() {
        // comparator spread of +-4% (about 4.4 binomial SD each way) gives D far above 2
        let v = scenario(1.0, 1.0, 1.2, 0.04);
        assert!(v["negative_control"]["D_mean_z2"].as_f64().unwrap() > 2.0);
        assert_eq!(verdict(&v), Verdict::NotInformative.code());
    }
    #[test]
    fn branch2_no_effect() {
        let v = scenario(1.0, 1.0, 1.2, 0.0);
        assert_eq!(verdict(&v), Verdict::NotReplicated.code());
        assert_eq!(v["decision"]["step"], 2);
        assert_eq!(v["T1_primary_vs_comparators"]["replicated"], false);
    }
    #[test]
    fn branch2_significant_but_below_floor() {
        // +3%: p < 0.05 alone but under the 1.05 floor
        let v = scenario(1.03, 1.0, 1.2, 0.0);
        assert!(v["T1_primary_vs_comparators"]["decision_p"].as_f64().unwrap() < 0.05);
        assert_eq!(verdict(&v), Verdict::NotReplicated.code());
    }
    #[test]
    fn branch3_curve_effect() {
        let v = scenario(1.10, 1.10, 1.2, 0.0);
        assert_eq!(verdict(&v), Verdict::CurveEffect.code());
        assert_eq!(v["decision"]["step"], 3);
    }
    #[test]
    fn branch4_presentation() {
        let v = scenario(1.07, 1.0, 1.2, 0.0);
        assert_eq!(verdict(&v), Verdict::PresentationEffect.code());
        assert_eq!(v["decision"]["step"], 4);
    }
    #[test]
    fn branch5_inconclusive() {
        // replicated (P +6%) but iso +4% (below the floor, T3 not significant)
        let v = scenario(1.06, 1.04, 1.2, 0.0);
        assert_eq!(verdict(&v), Verdict::Inconclusive.code());
        assert_eq!(v["decision"]["step"], 5);
    }
    #[test]
    fn branch5_p_below_iso() {
        // replicated, iso not present, T3 significant but y_P < y_I: not rule 4, so inconclusive
        let x = DecisionInputs { pc_ratio: 1.2, pc_p_z: 0.0, d: 1.0, r_p: 1.06, decision_p: 0.001, r_i: 1.04, p_z_i: 0.001, t3_p_z: 0.001, y_p_gt_y_i: false };
        assert_eq!(classify(&x), (Verdict::Inconclusive, 5));
    }
    #[test]
    fn classify_order_and_edges() {
        let base = DecisionInputs { pc_ratio: 1.2, pc_p_z: 0.0, d: 1.0, r_p: 1.1, decision_p: 0.001, r_i: 1.1, p_z_i: 0.001, t3_p_z: 0.5, y_p_gt_y_i: true };
        assert_eq!(classify(&base), (Verdict::CurveEffect, 3));
        // step 1 precedes everything, including D exactly 2.0 (allowed) vs just above
        assert_eq!(classify(&DecisionInputs { d: 2.0, ..base }).0, Verdict::CurveEffect);
        assert_eq!(classify(&DecisionInputs { d: 2.0001, ..base }).0, Verdict::NotInformative);
        assert_eq!(classify(&DecisionInputs { d: f64::NAN, ..base }).0, Verdict::NotInformative);
        assert_eq!(classify(&DecisionInputs { pc_ratio: 1.0999, ..base }).0, Verdict::NotInformative);
        assert_eq!(classify(&DecisionInputs { pc_p_z: 0.05, ..base }).0, Verdict::NotInformative);
        // floor and p thresholds of step 2
        assert_eq!(classify(&DecisionInputs { r_p: 1.0499, ..base }).0, Verdict::NotReplicated);
        assert_eq!(classify(&DecisionInputs { r_p: 1.05, ..base }).0, Verdict::CurveEffect);
        assert_eq!(classify(&DecisionInputs { decision_p: 0.05, ..base }).0, Verdict::NotReplicated);
        // step 3 vs 4 boundary: r_I exactly 1.05 with p<0.05 is present
        assert_eq!(classify(&DecisionInputs { r_i: 1.05, ..base }).0, Verdict::CurveEffect);
        assert_eq!(classify(&DecisionInputs { r_i: 1.0499, t3_p_z: 0.01, ..base }).0, Verdict::PresentationEffect);
        // r_I >= 1.05 but p_z_I >= 0.05 is neither 3 nor 4: inconclusive
        assert_eq!(classify(&DecisionInputs { p_z_i: 0.2, ..base }).0, Verdict::Inconclusive);
    }
    #[test]
    fn incomplete_and_empty_sessions_are_not_run() {
        let mut main = vec![cells(Y0, 48)];
        for _ in 0..23 {
            main.push(cells(Y0, 48));
        }
        main[5].pop();
        let v = analyse(&main, &cells(Y0 * 1.2, 48), 15, 8, Some(48), 200, SEED);
        assert_eq!(verdict(&v), Verdict::NotRun.code());
        main[5] = vec![];
        let v = analyse(&main, &cells(Y0 * 1.2, 48), 15, 8, Some(48), 200, SEED);
        assert_eq!(verdict(&v), Verdict::NotRun.code());
        assert!(v.get("T1_primary_vs_comparators").is_none());
    }
    #[test]
    fn t1_numbers_by_hand() {
        // exact counts: P yields 1.10 x, C 1 x: K and SE follow the stated formulas
        let v = scenario(1.10, 1.10, 1.2, 0.0);
        let t1 = &v["T1_primary_vs_comparators"];
        let (kp, kc) = (t1["K_x"].as_f64().unwrap(), t1["K_y"].as_f64().unwrap());
        assert_eq!(t1["cells_x"], 48);
        assert_eq!(t1["cells_y"], 720);
        assert!((t1["se"].as_f64().unwrap() - (1.0 / kp + 1.0 / kc).sqrt()).abs() < 1e-15);
        assert!((t1["ratio"].as_f64().unwrap() - (kp / (48.0 * A)) / (kc / (720.0 * A))).abs() < 1e-12);
        let p_perm = t1["p_perm"].as_f64().unwrap();
        assert!(p_perm >= 1.0 / 2001.0 && p_perm < 0.01);
    }
    #[test]
    fn permutation_p_is_reproducible_and_null_is_uniformish() {
        let mut rng = Rng(7);
        // noisy null data: P and C from the same distribution
        let mk = |rng: &mut Rng, n: usize| -> Vec<Cell> { (0..n).map(|_| Cell { rel: 150.0 + (rng.next() % 100) as f64, att: A }).collect() };
        let p = mk(&mut rng, 48);
        let c = mk(&mut rng, 720);
        let a = perm_p(&p, &c, 500, SEED);
        assert_eq!(a, perm_p(&p, &c, 500, SEED));
        assert!(a > 0.0 && a <= 1.0);
    }
}
