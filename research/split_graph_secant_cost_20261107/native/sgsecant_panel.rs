//! Prospective panel driver (frozen by PROTOCOL.md): cells x instances x runs x arms,
//! full ledgers, paired ratios with bootstrap intervals, decision flags.
use crate::ec::*;
use crate::ops::*;
use crate::run::*;
use crate::sys::*;
use std::collections::BTreeMap;
use std::fmt::Write as _;

pub const DOMAIN: &str = "sgsecant/v1";
pub const FROZEN_SEED: u64 = 3141592653; // never used in any development run (dev runs use --seed 424242)

#[derive(Clone, Debug)]
pub struct Cfg {
    pub seed: u64,
    pub cells: Vec<(u64, usize)>,
    pub instances: usize,
    pub runs: usize,
    pub tmax: u64,
    pub budget: u64,
    pub curve_cap: u64,
    pub graph_cap: u64,
    pub boot: usize,
    pub threads: usize,
    pub name: String,
}
impl Cfg {
    pub fn frozen() -> Cfg {
        Cfg {
            seed: FROZEN_SEED,
            cells: vec![(151, 3), (227, 4), (337, 5), (491, 6), (673, 7), (991, 8)],
            instances: 4,
            runs: 30,
            tmax: 400,
            budget: 2_000_000_000,
            curve_cap: 1024,
            graph_cap: 40_000,
            boot: 4000,
            threads: 4,
            name: "frozen".into(),
        }
    }
    pub fn smoke() -> Cfg {
        Cfg { seed: 424242, cells: vec![(151, 3), (227, 4)], instances: 1, runs: 2, tmax: 120, budget: 500_000_000, boot: 200, threads: 2, name: "smoke".into(), ..Cfg::frozen() }
    }
    pub fn describe(&self) -> String {
        format!(
            "name={} domain={} seed={} cells={:?} instances={} runs={} tmax={} budget={} curve_cap={} graph_cap={} boot={}",
            self.name, DOMAIN, self.seed, self.cells, self.instances, self.runs, self.tmax, self.budget, self.curve_cap, self.graph_cap, self.boot
        )
    }
}

pub const ARMS: [(&str, &str, Pres); 10] = [
    ("cs2", "graph", Pres::Secant2),
    ("cs1", "graph", Pres::Secant1),
    ("cd", "graph", Pres::Dense),
    ("ls2", "low-x", Pres::Secant2),
    ("ls1", "low-x", Pres::Secant1),
    ("ld", "low-x", Pres::Dense),
    ("hs2", "hash-x", Pres::Secant2),
    ("hs1", "hash-x", Pres::Secant1),
    ("hd", "hash-x", Pres::Dense),
    ("hs2b", "hash-xB", Pres::Secant2),
];

#[derive(Clone, Debug)]
pub struct Rec {
    pub p: u64,
    pub d: usize,
    pub inst: usize,
    pub run: usize,
    pub arm: String,
    pub rr: RunRec,
    pub construct: Led,
    pub rho: Led,
    pub rho_steps: u64,
    pub rho_ok: bool,
    pub n: u64,
    pub secs: f64,
}
#[derive(Clone, Debug, Default)]
pub struct InstFail {
    pub p: u64,
    pub d: usize,
    pub inst: usize,
    pub reason: String,
    pub construct: Led,
}

fn led_json(l: &Led) -> String {
    format!("[{},{},{},{}]", l.mul, l.add, l.inv, l.native)
}
fn rec_json(r: &Rec) -> String {
    let mut s = String::new();
    let ph: Vec<String> = r.rr.led.iter().map(led_json).collect();
    let _ = write!(
        s,
        "{{\"p\":{},\"d\":{},\"inst\":{},\"run\":{},\"arm\":\"{}\",\"n\":{},\"completed\":{},\"rank\":{},\"targets\":{},\"targets_with_rel\":{},\"distinct_rel\":{},\"censored_systems\":{},\"nonzero_dim_systems\":{},\"replay_fail\":{},\"unresolved\":{},\"wrong_lambda\":{},\"inconsistent\":{},\"exact_mismatch\":{},\"handled_degree_max\":{},\"dim_sum\":{},\"solves\":{},\"fallbacks\":{},\"phases\":{:?},\"phase_ledger_mul_add_inv_native\":[{}],\"construct\":{},\"rho\":{},\"rho_steps\":{},\"rho_ok\":{},\"secs_L0\":{:.3}}}",
        r.p, r.d, r.inst, r.run, r.arm, r.n, r.rr.completed, r.rr.rank, r.rr.targets, r.rr.targets_with_rel, r.rr.distinct_rel, r.rr.censored_systems,
        r.rr.nonzero_dim_systems, r.rr.replay_fail, r.rr.unresolved, r.rr.wrong_lambda, r.rr.inconsistent, r.rr.exact_mismatch, r.rr.handled_degree_max,
        r.rr.dim_sum, r.rr.solves, r.rr.fallbacks, PHASES, ph.join(","), led_json(&r.construct), led_json(&r.rho), r.rho_steps, r.rho_ok, r.secs
    );
    s
}

fn seed_for(master: u64, p: u64, d: usize, inst: usize) -> u64 {
    let h = hash_words("sgsecant/v1/instance-seed", &[master, p, d as u64, inst as u64]);
    word(&h[..8])
}

fn run_instance(cfg: &Cfg, p: u64, d: usize, inst: usize) -> (Vec<Rec>, Option<InstFail>) {
    reset();
    phase(PH_CONSTRUCT);
    let spec = Spec { domain: DOMAIN.into(), seed: seed_for(cfg.seed, p, d, inst), curve_cap: cfg.curve_cap, graph_cap: cfg.graph_cap };
    let s0 = snapshot();
    let cs = select_curve(&spec, p, "curve");
    let s1 = snapshot();
    let Some(c) = cs.curve else {
        return (vec![], Some(InstFail { p, d, inst, reason: "curve_cap".into(), construct: total(&diff(&s1, &s0)) }));
    };
    let gs = graph_search(&spec, &c, d, "forced");
    let s2 = snapshot();
    let lowx = control_base(&spec, &c, d, "low-x", "control");
    let s3 = snapshot();
    let hashx = control_base(&spec, &c, d, "hash-x", "control");
    let s4 = snapshot();
    let hashxb = control_base(&spec, &c, d, "hash-x", "controlB");
    let s5 = snapshot();
    let Some(graph) = gs.base else {
        return (vec![], Some(InstFail { p, d, inst, reason: "graph_cap".into(), construct: total(&diff(&s2, &s1)) }));
    };
    let curve_l = total(&diff(&s1, &s0));
    let base_l = |a: &Snap, b: &Snap| total(&diff(b, a));
    let bases: Vec<(&str, &Base, Led)> = vec![
        ("graph", &graph, curve_l.plus(&base_l(&s1, &s2))),
        ("low-x", &lowx, curve_l.plus(&base_l(&s2, &s3))),
        ("hash-x", &hashx, curve_l.plus(&base_l(&s3, &s4))),
        ("hash-xB", &hashxb, curve_l.plus(&base_l(&s4, &s5))),
    ];
    let g = c.points[0];
    let mut out = Vec::new();
    for run in 0..cfg.runs {
        let hh = hash_words("sgsecant/v1/lambda", &[spec.seed, run as u64]);
        let lambda = 2 + word(&hh[..8]) % (c.n - 2);
        let h = smul(&c, lambda, g).unwrap();
        let inst_ = Inst { c: &c, g, h, lambda };
        let (rl, rsteps, rok) = rho(&c, g, h, lambda, &[spec.seed, run as u64]);
        for (arm, bname, pres) in ARMS.iter() {
            let (_, b, cl) = bases.iter().find(|x| x.0 == *bname).unwrap();
            let t0 = std::time::Instant::now();
            let rr = run_arm(&inst_, b, *pres, &[spec.seed, run as u64], cfg.tmax, false, true, cfg.budget);
            out.push(Rec { p, d, inst, run, arm: arm.to_string(), rr, construct: *cl, rho: rl, rho_steps: rsteps, rho_ok: rok, n: c.n, secs: t0.elapsed().as_secs_f64() });
            // positive control: the cs2 arm solved twice per target (must cost ~2x in solve phases)
            if *arm == "cs2" && inst == 0 {
                let t0 = std::time::Instant::now();
                let rr = run_arm(&inst_, b, *pres, &[spec.seed, run as u64], cfg.tmax, true, false, cfg.budget);
                out.push(Rec { p, d, inst, run, arm: "pc2x".into(), rr, construct: *cl, rho: rl, rho_steps: rsteps, rho_ok: rok, n: c.n, secs: t0.elapsed().as_secs_f64() });
            }
        }
    }
    (out, None)
}

// ----------------------------------------------------------- statistics
struct Sm(u64);
impl Sm {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}
#[derive(Clone, Copy)]
pub enum Metric {
    Field,
    Mul,
    InvX10,
    W(f64),
}
impl Metric {
    pub fn name(&self) -> String {
        match self {
            Metric::Field => "field".into(),
            Metric::Mul => "mul_only".into(),
            Metric::InvX10 => "inv_x10".into(),
            Metric::W(w) => format!("field+{}native", w),
        }
    }
    fn of(&self, l: &Led) -> f64 {
        match self {
            Metric::Field => l.field() as f64,
            Metric::Mul => l.mul as f64,
            Metric::InvX10 => (l.mul + l.add + 10 * l.inv) as f64,
            Metric::W(w) => l.field() as f64 + w * l.native as f64,
        }
    }
}
pub fn metrics() -> Vec<Metric> {
    vec![Metric::Field, Metric::Mul, Metric::InvX10, Metric::W(0.1), Metric::W(1.0)]
}
fn cost(r: &Rec, m: Metric, incl_construct: bool) -> f64 {
    let t = total(&r.rr.led.clone().try_into().unwrap());
    let mut c = m.of(&t);
    if incl_construct {
        c += m.of(&r.construct);
    }
    c
}
fn gm_ci(ratios: &[f64], boot: usize, seed: u64) -> (f64, f64, f64) {
    if ratios.is_empty() {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    let logs: Vec<f64> = ratios.iter().map(|x| x.ln()).collect();
    let mean = logs.iter().sum::<f64>() / logs.len() as f64;
    let mut rng = Sm(seed);
    let mut bs = Vec::with_capacity(boot);
    for _ in 0..boot {
        let mut s = 0.0;
        for _ in 0..logs.len() {
            s += logs[(rng.next() % logs.len() as u64) as usize];
        }
        bs.push(s / logs.len() as f64);
    }
    bs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let lo = bs[(0.025 * boot as f64) as usize];
    let hi = bs[((0.975 * boot as f64) as usize).min(boot - 1)];
    (mean.exp(), lo.exp(), hi.exp())
}
fn median(v: &mut Vec<f64>) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = v.len() / 2;
    if v.len() % 2 == 1 { v[m] } else { (v[m - 1] + v[m]) / 2.0 }
}

pub const COMPARISONS: [(&str, &str, &str); 10] = [
    ("PRIMARY cs2/cd", "cs2", "cd"),
    ("sec cs1/cd", "cs1", "cd"),
    ("ctl cs2/ls2", "cs2", "ls2"),
    ("ctl cs2/hs2", "cs2", "hs2"),
    ("ctl cs1/ls1", "cs1", "ls1"),
    ("ctl cs1/hs1", "cs1", "hs1"),
    ("same-base dense ctl ld/hd", "ld", "hd"),
    ("A/A hs2/hs2b", "hs2", "hs2b"),
    ("pos-ctl pc2x/cs2", "pc2x", "cs2"),
    ("self cs2/cs2", "cs2", "cs2"),
];

pub const MARGIN: f64 = 0.05; // materiality margin frozen in PROTOCOL.md

fn verdict_of(n: usize, lo: f64, hi: f64) -> &'static str {
    if n == 0 {
        "unmeasured"
    } else if hi < 1.0 - MARGIN {
        "lower"
    } else if lo > 1.0 + MARGIN {
        "higher"
    } else {
        "indistinct"
    }
}
struct Stat {
    n: usize,
    unmeasured: usize,
    gm: f64,
    lo: f64,
    hi: f64,
    verdict: &'static str,
}
fn pair_stat(cfg: &Cfg, rs: &[&Rec], a: &str, b: &str, incl: bool, m: Metric, seed: u64) -> Stat {
    let get = |arm: &str, inst: usize, run: usize| rs.iter().find(|r| r.arm == arm && r.inst == inst && r.run == run).copied();
    let mut ratios = Vec::new();
    let mut unmeasured = 0;
    for inst in 0..cfg.instances {
        for run in 0..cfg.runs {
            if let (Some(ra), Some(rb)) = (get(a, inst, run), get(b, inst, run)) {
                let ok = |r: &Rec| r.rr.completed && r.rr.censored_systems == 0 && r.rr.unresolved == 0 && r.rr.nonzero_dim_systems == 0;
                if ok(ra) && ok(rb) {
                    ratios.push(cost(ra, m, incl) / cost(rb, m, incl));
                } else {
                    unmeasured += 1;
                }
            }
        }
    }
    let (gm, lo, hi) = gm_ci(&ratios, cfg.boot, seed);
    Stat { n: ratios.len(), unmeasured, gm, lo, hi, verdict: verdict_of(ratios.len(), lo, hi) }
}
/// Solve-phase operations (build, groebner, fglm, roots, replay, verify): doubled exactly by pc2x.
fn solve_phase_field(r: &Rec) -> f64 {
    [PH_BUILD, PH_GB, PH_FGLM, PH_ROOTS, PH_REPLAY, PH_VERIFY].iter().map(|&i| r.rr.led[i].field() as f64).sum()
}

pub fn summarize(cfg: &Cfg, recs: &[Rec], fails: &[InstFail]) -> (String, String) {
    let mut txt = String::new();
    let mut js = String::from("{\"config\":\"");
    js.push_str(&cfg.describe());
    js.push_str("\",\"cells\":[");
    let _ = writeln!(txt, "# sgsecant panel summary ({})", cfg.describe());
    for f in fails {
        let _ = writeln!(txt, "INSTANCE FAILED (unmeasured): p={} d={} inst={} reason={}", f.p, f.d, f.inst, f.reason);
    }
    // decision bookkeeping
    let mut integrity_ok = true;
    let mut ctl_ok = true;
    let mut cells_ok: Vec<(u64, usize)> = Vec::new();
    let mut dec: BTreeMap<&str, (usize, usize, usize, usize, usize)> = BTreeMap::new(); // cand -> (advancing cells, higher in field ops, not lower in field ops, weight/convention dependent, clean cells)
    let mut first = true;
    for (ci, &(p, d)) in cfg.cells.iter().enumerate() {
        let rs: Vec<&Rec> = recs.iter().filter(|r| r.p == p && r.d == d).collect();
        let nominal = cfg.instances * cfg.runs;
        let seed = cfg.seed ^ (ci as u64 * 7919 + 13);
        if !first {
            js.push(',');
        }
        first = false;
        let _ = write!(js, "{{\"p\":{},\"d\":{},\"arms\":{{", p, d);
        let _ = writeln!(txt, "\n## cell p={} d={}  (K={} abscissae, rank target {}, nominal paired units {})", p, d, 2 * d, 2 * d + 1, nominal);
        let mut afirst = true;
        let mut cell_clean = !rs.is_empty() && !fails.iter().any(|f| f.p == p && f.d == d);
        for arm in ARMS.iter().map(|a| a.0).chain(std::iter::once("pc2x")) {
            let ar: Vec<&&Rec> = rs.iter().filter(|r| r.arm == arm).collect();
            if ar.is_empty() {
                continue;
            }
            let done = ar.iter().filter(|r| r.rr.completed && r.rr.censored_systems == 0).count();
            let mut perrel: Vec<f64> = ar.iter().filter(|r| r.rr.completed).map(|r| cost(r, Metric::Field, true) / (2 * d + 1) as f64).collect();
            let mut perrel_x: Vec<f64> = ar.iter().filter(|r| r.rr.completed).map(|r| cost(r, Metric::Field, false) / (2 * d + 1) as f64).collect();
            let mut nat: Vec<f64> = ar.iter().filter(|r| r.rr.completed).map(|r| total(&r.rr.led.clone().try_into().unwrap()).native as f64 / (2 * d + 1) as f64).collect();
            let mut tg: Vec<f64> = ar.iter().map(|r| r.rr.targets as f64).collect();
            let mut hd: Vec<f64> = ar.iter().map(|r| r.rr.handled_degree_max as f64).collect();
            let mut share: Vec<f64> = ar.iter().filter(|r| r.rr.completed).map(|r| { let c = Metric::Field.of(&r.construct); c / cost(r, Metric::Field, true) }).collect();
            let mism: u64 = ar.iter().map(|r| r.rr.exact_mismatch).sum();
            let rf: u64 = ar.iter().map(|r| r.rr.replay_fail).sum();
            let cens: u64 = ar.iter().map(|r| r.rr.censored_systems).sum();
            let unres: u64 = ar.iter().map(|r| r.rr.unresolved + r.rr.nonzero_dim_systems).sum();
            let wrong = ar.iter().filter(|r| r.rr.wrong_lambda || r.rr.inconsistent).count();
            if mism + rf + unres > 0 || wrong > 0 {
                integrity_ok = false;
            }
            if cens > 0 || done < ar.len() {
                cell_clean = false;
            }
            let (mp, mx, mn, mh, mt, ms) = (median(&mut perrel), median(&mut perrel_x), median(&mut nat), median(&mut hd), median(&mut tg), median(&mut share));
            let _ = writeln!(
                txt,
                "arm {:<5} runs={} completed={} median field/relation incl.construct={:.0} excl={:.0} native/relation={:.0} construct share={:.2} median targets={:.0} handled_degree(median)={:.0} exact_mismatch={} replay_fail={} unresolved/nonzero-dim={} censored={} wrong_lambda={}",
                arm, ar.len(), done, mp, mx, mn, ms, mt, mh, mism, rf, unres, cens, wrong
            );
            if !afirst {
                js.push(',');
            }
            afirst = false;
            let _ = write!(
                js,
                "\"{}\":{{\"runs\":{},\"completed\":{},\"median_field_per_relation_incl_construct\":{:.3},\"median_field_per_relation_excl_construct\":{:.3},\"median_native_per_relation\":{:.3},\"construct_share\":{:.4},\"median_targets\":{:.1},\"median_handled_degree\":{:.1},\"exact_mismatch\":{},\"replay_fail\":{},\"unresolved_or_nonzero_dim\":{},\"censored_systems\":{},\"wrong_lambda\":{}}}",
                arm, ar.len(), done, mp, mx, mn, ms, mt, mh, mism, rf, unres, cens, wrong
            );
        }
        js.push_str("},\"comparisons\":[");
        let mut cfirst = true;
        for (label, a, b) in COMPARISONS.iter() {
            for incl in [true, false] {
                for m in metrics() {
                    let st = pair_stat(cfg, &rs, a, b, incl, m, seed);
                    if (matches!(m, Metric::Field) && incl) || label.starts_with("PRIMARY") || label.starts_with("sec cs1") {
                        let _ = writeln!(txt, "  {:<28} {:<18} incl_construct={:<5} n={} GM={:.4} CI95=[{:.4},{:.4}] unmeasured={} -> {}", label, m.name(), incl, st.n, st.gm, st.lo, st.hi, st.unmeasured, st.verdict);
                    }
                    if !cfirst {
                        js.push(',');
                    }
                    cfirst = false;
                    let _ = write!(
                        js,
                        "{{\"comparison\":\"{}\",\"a\":\"{}\",\"b\":\"{}\",\"metric\":\"{}\",\"incl_construct\":{},\"n\":{},\"unmeasured\":{},\"gm_ratio\":{:.6},\"ci_lo\":{:.6},\"ci_hi\":{:.6},\"verdict\":\"{}\"}}",
                        label, a, b, m.name(), incl, st.n, st.unmeasured, st.gm, st.lo, st.hi, st.verdict
                    );
                }
            }
        }
        js.push(']');
        // controls
        let selfst = pair_stat(cfg, &rs, "cs2", "cs2", true, Metric::Field, seed);
        let mut pcr = Vec::new();
        for r in rs.iter().filter(|r| r.arm == "pc2x") {
            if let Some(b) = rs.iter().find(|x| x.arm == "cs2" && x.inst == r.inst && x.run == r.run) {
                pcr.push(solve_phase_field(r) / solve_phase_field(b));
            }
        }
        let pc_ok = !pcr.is_empty() && pcr.iter().all(|x| (x - 2.0).abs() < 1e-9);
        let aa = pair_stat(cfg, &rs, "hs2", "hs2b", true, Metric::Field, seed);
        let aa_ok = aa.n > 0 && aa.lo <= 1.0 + MARGIN && aa.hi >= 1.0 - MARGIN;
        let self_ok = selfst.n > 0 && (selfst.gm - 1.0).abs() < 1e-12;
        if !(pc_ok && aa_ok && self_ok) {
            ctl_ok = false;
        }
        let _ = writeln!(txt, "  controls: self-ratio==1: {}  positive control (solve-phase ratio exactly 2.0): {} (n={})  A/A hs2/hs2b CI95=[{:.3},{:.3}] contains the +-{:.0}% margin band: {}", self_ok, pc_ok, pcr.len(), aa.lo, aa.hi, MARGIN * 100.0, aa_ok);
        // same-base decision statistics, excl. construct (construction is identical for the pair)
        for cand in ["cs1", "cs2"] {
            let mut vs = Vec::new();
            let mut field_v = "unmeasured";
            let mut other_higher = false;
            let mut other_lower = false;
            for m in metrics() {
                let st = pair_stat(cfg, &rs, cand, "cd", false, m, seed);
                vs.push((m.name(), st.verdict, st.gm));
                if matches!(m, Metric::Field) {
                    field_v = st.verdict;
                } else {
                    if st.verdict == "higher" {
                        other_higher = true;
                    }
                    if st.verdict == "lower" {
                        other_lower = true;
                    }
                }
            }
            let _ = writeln!(txt, "  same-base {cand}/cd (excl. identical construction): {}", vs.iter().map(|(n, v, g)| format!("{n}:{v}({g:.3})")).collect::<Vec<_>>().join("  "));
            let e = dec.entry(cand).or_insert((0, 0, 0, 0, 0));
            if cell_clean {
                e.4 += 1;
                if field_v == "lower" && !other_higher {
                    e.0 += 1; // advancing cell
                }
                if field_v == "higher" {
                    e.1 += 1;
                }
                if field_v != "lower" {
                    e.2 += 1; // not lower on field ops
                }
                if (field_v != "lower" && other_lower) || (field_v == "lower" && other_higher) {
                    e.3 += 1; // weight/convention dependent
                }
            }
        }
        if cell_clean {
            cells_ok.push((p, d));
        }
        let mut gaps = Vec::new();
        for r in rs.iter().filter(|r| r.arm == "cs2" && r.rr.completed) {
            gaps.push(cost(r, Metric::Field, true) / r.rho.field() as f64);
        }
        let mg = median(&mut gaps);
        let mut gaps1 = Vec::new();
        for r in rs.iter().filter(|r| r.arm == "cs1" && r.rr.completed) {
            gaps1.push(cost(r, Metric::Field, true) / r.rho.field() as f64);
        }
        let mg1 = median(&mut gaps1);
        let mut gd = Vec::new();
        for r in rs.iter().filter(|r| r.arm == "cd" && r.rr.completed) {
            gd.push(cost(r, Metric::Field, true) / r.rho.field() as f64);
        }
        let mgd = median(&mut gd);
        let _ = writeln!(txt, "  distance to internal rho reference (field ops, complete cost / rho cost, median, incl. construction): cs2 = {:.1}x, cs1 = {:.1}x, cd = {:.1}x (distance remaining, not a gate; the matched ecbench rho with the section 5a band is a separate required row)", mg, mg1, mgd);
        let _ = write!(js, ",\"clean\":{},\"gap_to_internal_rho_median\":{{\"cs2\":{:.3},\"cs1\":{:.3},\"cd\":{:.3}}}}}", cell_clean, mg, mg1, mgd);
    }
    js.push_str("],\"decision\":{");
    let _ = writeln!(txt, "\n## PROTOCOL DECISION (rule frozen in PROTOCOL.md)");
    let _ = writeln!(txt, "clean (uncensored) cells: {} of {}  integrity (exact root sets, replay, no unresolved): {}  controls: {}", cells_ok.len(), cfg.cells.len(), integrity_ok, ctl_ok);
    let mut dfirst = true;
    for (cand, (adv, higher, notlower, wdep, cells)) in dec.iter() {
        let outcome = if !integrity_ok || !ctl_ok || *cells < 3 {
            "NOT CLOSED (integrity, control or fewer than 3 clean cells: unmeasured)".to_string()
        } else if *adv >= 3 && *higher == 0 {
            "ADVANCES (>=3 clean cells lower on field ops by more than the margin, none higher, no other counting convention higher)".to_string()
        } else if *notlower >= 3 {
            let mut s = String::from("REJECTED on complete same-base field operations at the tested scale (lower degree, not lower cost)");
            if *wdep > 0 {
                s.push_str("; native-weighted or counting-convention order differs in some cells, so under section 5a the pair is UNRANKED across the band, not ranked in the candidate's favour");
            }
            s
        } else {
            "INCONCLUSIVE".to_string()
        };
        let _ = writeln!(txt, "candidate {cand}: advancing cells = {adv}, cells higher in field ops = {higher}, cells not lower in field ops = {notlower}, cells with convention-dependent order = {wdep}, clean cells = {cells} -> {outcome}");
        if !dfirst {
            js.push(',');
        }
        dfirst = false;
        let _ = write!(js, "\"{}\":{{\"advancing_cells\":{},\"higher_field_cells\":{},\"not_lower_field_cells\":{},\"convention_dependent_cells\":{},\"clean_cells\":{},\"outcome\":\"{}\"}}", cand, adv, higher, notlower, wdep, cells, outcome);
    }
    js.push_str("}}");
    (txt, js)
}

pub fn panel_main(args: &[String]) {
    let mut cfg = Cfg::frozen();
    let mut out = String::from("panel_out");
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--smoke" => cfg = Cfg::smoke(),
            "--out" => {
                i += 1;
                out = args[i].clone();
            }
            "--threads" => {
                i += 1;
                cfg.threads = args[i].parse().unwrap();
            }
            "--seed" => {
                i += 1;
                cfg.seed = args[i].parse().unwrap();
                cfg.name = format!("dev-seed-{}", cfg.seed);
            }
            "--cells" => {
                i += 1;
                cfg.cells = args[i].split(',').map(|c| { let mut it = c.split(':'); (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap()) }).collect();
                cfg.name = format!("{}-cells", cfg.name);
            }
            "--instances" => {
                i += 1;
                cfg.instances = args[i].parse().unwrap();
            }
            "--runs" => {
                i += 1;
                cfg.runs = args[i].parse().unwrap();
            }
            "--describe" => {
                println!("{}", cfg.describe());
                return;
            }
            a => panic!("unknown argument {a}"),
        }
        i += 1;
    }
    std::fs::create_dir_all(&out).unwrap();
    println!("{}", cfg.describe());
    let jobs: Vec<(u64, usize, usize)> = cfg.cells.iter().flat_map(|&(p, d)| (0..cfg.instances).map(move |i| (p, d, i))).collect();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let results = std::sync::Mutex::new(Vec::<(Vec<Rec>, Option<InstFail>, f64)>::new());
    std::thread::scope(|sc| {
        for _ in 0..cfg.threads {
            sc.spawn(|| loop {
                let k = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if k >= jobs.len() {
                    break;
                }
                let (p, d, inst) = jobs[k];
                let t0 = std::time::Instant::now();
                let (r, f) = run_instance(&cfg, p, d, inst);
                let secs = t0.elapsed().as_secs_f64();
                eprintln!("job p={p} d={d} inst={inst} done in {secs:.1}s ({} records)", r.len());
                results.lock().unwrap().push((r, f, secs));
            });
        }
    });
    let mut all: Vec<Rec> = Vec::new();
    let mut fails = Vec::new();
    for (r, f, _) in results.into_inner().unwrap() {
        all.extend(r);
        if let Some(f) = f {
            fails.push(f);
        }
    }
    all.sort_by(|a, b| (a.p, a.inst, a.run, a.arm.clone()).cmp(&(b.p, b.inst, b.run, b.arm.clone())));
    let mut lines = String::new();
    for r in &all {
        lines.push_str(&rec_json(r));
        lines.push('\n');
    }
    std::fs::write(format!("{out}/records.jsonl"), lines).unwrap();
    std::fs::write(format!("{out}/config.txt"), cfg.describe() + "\n").unwrap();
    let (txt, js) = summarize(&cfg, &all, &fails);
    std::fs::write(format!("{out}/summary.txt"), &txt).unwrap();
    std::fs::write(format!("{out}/summary.json"), &js).unwrap();
    println!("{txt}");
    let _ = BTreeMap::<u8, u8>::new();
}
