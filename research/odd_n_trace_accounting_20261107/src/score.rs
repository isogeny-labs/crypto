//! Registered scoring (PROTOCOL.md "Predictions" and "Pass rules"), the
//! frozen interpretation rule, and the pcc validator report.
use crate::json::{self, J};
use crate::rng::{Rng, SEED};
use crate::ARMS;
use std::collections::BTreeMap;
use std::fmt::Write as _;

pub const B: usize = 2000;
const CLASSES: [&str; 3] = ["2E", "off", "odd"];

#[derive(Clone)]
struct Arm {
    m: f64,
    m0: f64,
    m1: f64,
    c: f64,
    brute_checked: f64,
    brute_mismatch: f64,
    ident: Option<(bool, bool, bool)>,
}

struct Unit {
    n: u32,
    d: u32,
    id: String,
    tr_a: u32,
    ne: f64,
    q: f64,
    census: bool,
    crit_mismatch: f64,
    odd_mismatch: f64,
    arms: Vec<Arm>,              // in ARMS order
    counts: Vec<Vec<Vec<f64>>>,  // [class][arm][target]
}

fn ai(name: &str) -> usize {
    ARMS.iter().position(|a| *a == name).unwrap()
}
fn ci(name: &str) -> usize {
    CLASSES.iter().position(|a| *a == name).unwrap()
}

fn load(dir: &str) -> Vec<Unit> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.file_name().unwrap().to_str().unwrap().starts_with("cell_") && p.extension().map(|x| x == "jsonl").unwrap_or(false))
        .collect();
    files.sort();
    let mut units: Vec<Unit> = vec![];
    for p in files {
        let txt = std::fs::read_to_string(&p).unwrap();
        for line in txt.lines().filter(|l| !l.trim().is_empty()) {
            let r = json::parse(line);
            let num = |k: &str| r.get(k).num().unwrap();
            if r.get("type").str() == "curve" {
                let arms = r
                    .get("arms")
                    .arr()
                    .iter()
                    .map(|a| {
                        let g = |k: &str| a.get(k).num().unwrap();
                        let ident = if matches!(a.get("identity_2E"), J::Null) {
                            None
                        } else {
                            Some((a.get("identity_2E").bool(), a.get("identity_off").bool(), a.get("identity_all").bool()))
                        };
                        Arm { m: g("m"), m0: g("m0"), m1: g("m1"), c: g("c"), brute_checked: g("brute_checked"), brute_mismatch: g("brute_mismatch"), ident }
                    })
                    .collect();
                for (i, a) in r.get("arms").arr().iter().enumerate() {
                    assert_eq!(a.get("arm").str(), ARMS[i]);
                }
                units.push(Unit {
                    n: num("n") as u32,
                    d: num("d") as u32,
                    id: r.get("id").str().to_string(),
                    tr_a: num("tr_a") as u32,
                    ne: num("nE"),
                    q: num("q"),
                    census: r.get("mode").str() == "census",
                    crit_mismatch: num("crit_mismatch"),
                    odd_mismatch: num("odd_mismatch"),
                    arms,
                    counts: vec![vec![]; 3],
                });
            } else {
                let u = units.last_mut().unwrap();
                assert_eq!(r.get("id").str(), u.id);
                let c = ci(r.get("class").str());
                u.counts[c] = ARMS.iter().map(|a| r.get("counts").get(a).nums()).collect();
            }
        }
    }
    units
}

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() { f64::NAN } else { v.iter().sum::<f64>() / v.len() as f64 }
}

/// means[class][arm]
type Means = Vec<Vec<f64>>;

impl Unit {
    fn label(&self) -> String {
        format!("n={} d={} {}", self.n, self.d, self.id)
    }
    fn means(&self) -> Means {
        self.counts.iter().map(|cl| cl.iter().map(|v| mean(v)).collect()).collect()
    }
    /// paired bootstrap over targets, classes resampled independently
    fn boot(&self) -> Vec<Means> {
        let mut rng = Rng::new(SEED, &format!("boot-n{}-d{}-{}", self.n, self.d, self.id));
        let mut out = Vec::with_capacity(B);
        for _ in 0..B {
            let mut mm: Means = vec![];
            for cl in &self.counts {
                let t = cl[0].len();
                let mut s = vec![0f64; cl.len()];
                for _ in 0..t {
                    let i = rng.below(t as u64) as usize;
                    for (a, v) in cl.iter().enumerate() {
                        s[a] += v[i];
                    }
                }
                mm.push(s.iter().map(|x| x / t as f64).collect());
            }
            out.push(mm);
        }
        out
    }
    fn pw(&self, x: f64) -> f64 {
        x.powi(self.n as i32)
    }
    /// exact coset mean over 2E (or off 2E) from the measured composition
    fn pred_mean(&self, arm: usize, class: usize) -> f64 {
        let a = &self.arms[arm];
        let (sp, sm) = (self.pw(a.m0 + a.m1), self.pw(a.m0 - a.m1));
        // for Tr(a) = 1 the roles of m0/m1 are as measured (m0 = points in 2E)
        match class {
            1 => (sp - sm) / self.ne,
            _ => (sp + sm) / self.ne,
        }
    }
    fn kappa(&self, mm: &Means, class: usize, arm: usize) -> f64 {
        mm[class][arm] * self.ne / self.pw(self.arms[arm].m)
    }
    fn kappa_pred(&self, class: usize, arm: usize) -> f64 {
        self.pred_mean(arm, class) * self.ne / self.pw(self.arms[arm].m)
    }
    fn eta(&self, mm: &Means, class: usize, arm: usize) -> f64 {
        mm[class][arm] / self.arms[arm].c
    }
    fn eta_pred(&self, class: usize, arm: usize) -> f64 {
        self.pred_mean(arm, class) / self.arms[arm].c
    }
}

type StatFn = Box<dyn Fn(&Unit, &Means) -> f64>;

struct Stat {
    name: &'static str,
    f: StatFn,
    pred: Box<dyn Fn(&Unit) -> f64>,
}

#[derive(Clone)]
pub struct Row {
    unit: String,
    obs: f64,
    lo: f64,
    hi: f64,
    pred: f64,
}
impl Row {
    fn in_ci(&self) -> bool {
        self.pred >= self.lo && self.pred <= self.hi
    }
}

fn pct(v: &mut [f64], p: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater));
    let i = ((v.len() as f64 - 1.0) * p).round() as usize;
    v[i]
}

fn eval(u: &Unit, mm: &Means, boots: &[Means], s: &Stat) -> Row {
    let obs = (s.f)(u, mm);
    let mut bs: Vec<f64> = boots.iter().map(|b| (s.f)(u, b)).map(|x| if x.is_nan() { f64::INFINITY } else { x }).collect();
    let lo = pct(&mut bs, 0.005);
    let hi = pct(&mut bs, 0.995);
    Row { unit: u.label(), obs, lo, hi, pred: (s.pred)(u) }
}

fn geo(v: &[f64]) -> f64 {
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}

/// registered rule: prediction inside the 99 % CI in all but max(1, floor(0.1 U)) units,
/// and pooled geometric mean of obs/pred in [0.9, 1.1]
fn rule(rows: &[Row]) -> (bool, usize, usize, f64) {
    let u = rows.len();
    let miss = rows.iter().filter(|r| !r.in_ci()).count();
    let allowed = std::cmp::max(1, u / 10);
    let pooled = geo(&rows.iter().map(|r| r.obs / r.pred).collect::<Vec<_>>());
    (miss <= allowed && (0.9..=1.1).contains(&pooled) && u > 0, miss, u, pooled)
}

pub struct Scored {
    pub text: String,
    pub headline: BTreeMap<String, f64>,
    pub verdicts: BTreeMap<String, String>,
}

fn stat(name: &'static str, f: impl Fn(&Unit, &Means) -> f64 + 'static, pred: impl Fn(&Unit) -> f64 + 'static) -> Stat {
    Stat { name, f: Box::new(f), pred: Box::new(pred) }
}

pub fn compute(dir: &str) -> Scored {
    let units = load(dir);
    let (s2, so, sd) = (ci("2E"), ci("off"), ci("odd"));
    let (std, tz, tw, twh, hz, mix) = (ai("std"), ai("tz"), ai("tw"), ai("twh"), ai("hz"), ai("mix"));
    let mut out = String::new();
    let mut head: BTreeMap<String, f64> = BTreeMap::new();
    let mut verd: BTreeMap<String, String> = BTreeMap::new();

    let stats: Vec<Stat> = vec![
        stat("P1_r_2E", move |_, m| m[s2][tw] / m[s2][std], move |u| u.pred_mean(tw, s2) / u.pred_mean(std, s2)),
        stat("P1_D_2E", move |u, m| u.kappa(m, s2, tw) / u.kappa(m, s2, std), move |u| u.kappa_pred(s2, tw) / u.kappa_pred(s2, std)),
        stat(
            "P2_r_all",
            move |_, m| (m[s2][tw] + m[so][tw]) / (m[s2][std] + m[so][std]),
            move |u| u.pw(u.arms[tw].m / u.arms[std].m),
        ),
        stat("P3_std_on_over_off", move |_, m| m[s2][std] / m[so][std], move |u| u.pred_mean(std, s2) / u.pred_mean(std, so)),
        stat("P3_kappa_tz_2E", move |u, m| u.kappa(m, s2, tz), move |u| u.kappa_pred(s2, tz)),
        stat("P3_kappa_tw_2E", move |u, m| u.kappa(m, s2, tw), move |u| u.kappa_pred(s2, tw)),
        stat("P4_kappa_std_2E", move |u, m| u.kappa(m, s2, std), move |u| u.kappa_pred(s2, std)),
        stat("P4_kappa_mix_2E", move |u, m| u.kappa(m, s2, mix), move |u| u.kappa_pred(s2, mix)),
        stat("P5_r_odd", move |_, m| m[sd][tw] / m[sd][std], move |u| u.pred_mean(tw, s2) / u.pred_mean(std, s2)),
        stat("P6_H_2E", move |u, m| u.kappa(m, s2, hz) / u.kappa(m, s2, std), move |u| u.kappa_pred(s2, hz) / u.kappa_pred(s2, std)),
        stat("P6_H_odd", move |u, m| u.kappa(m, sd, hz) / u.kappa(m, sd, std), move |u| u.kappa_pred(s2, hz) / u.kappa_pred(s2, std)),
        stat("P6_L_2E", move |u, m| u.kappa(m, s2, tw) / u.kappa(m, s2, hz), move |u| u.kappa_pred(s2, tw) / u.kappa_pred(s2, hz)),
        stat("P6_L_odd", move |u, m| u.kappa(m, sd, tw) / u.kappa(m, sd, hz), move |u| u.kappa_pred(s2, tw) / u.kappa_pred(s2, hz)),
        stat("P6_T_2E", move |u, m| u.eta(m, s2, tz) / u.eta(m, s2, std), move |u| u.eta_pred(s2, tz) / u.eta_pred(s2, std)),
        stat("P6_T_odd", move |u, m| u.eta(m, sd, tz) / u.eta(m, sd, std), move |u| u.eta_pred(s2, tz) / u.eta_pred(s2, std)),
        stat("P6_mix_2E", move |u, m| u.kappa(m, s2, mix) / u.kappa(m, s2, std), move |u| u.kappa_pred(s2, mix) / u.kappa_pred(s2, std)),
        // reported (no pass rule): relations per trial per column and the twh sub-base
        stat("R_eta_tw_over_std_odd", move |u, m| u.eta(m, sd, tw) / u.eta(m, sd, std), move |u| u.eta_pred(s2, tw) / u.eta_pred(s2, std)),
        stat("R_eta_tw_over_hz_odd", move |u, m| u.eta(m, sd, tw) / u.eta(m, sd, hz), move |u| u.eta_pred(s2, tw) / u.eta_pred(s2, hz)),
        stat("R_eta_hz_over_std_odd", move |u, m| u.eta(m, sd, hz) / u.eta(m, sd, std), move |u| u.eta_pred(s2, hz) / u.eta_pred(s2, std)),
        stat("R_yield_tw_over_hz_odd", move |_, m| m[sd][tw] / m[sd][hz], move |u| u.pred_mean(tw, s2) / u.pred_mean(hz, s2)),
        stat("R_kappa_twh_2E", move |u, m| u.kappa(m, s2, twh), move |u| u.kappa_pred(s2, twh)),
    ];

    // evaluate every stat on every Tr(a) = 0 unit
    let mut table: BTreeMap<&str, Vec<(u32, Row)>> = BTreeMap::new();
    let mut desc = String::new();
    for u in units.iter().filter(|u| u.tr_a == 0) {
        let mm = u.means();
        let boots = u.boot();
        for s in &stats {
            let odd = u.n % 2 == 1;
            let applies = match s.name {
                "P3_std_on_over_off" | "P3_kappa_tz_2E" | "P3_kappa_tw_2E" => odd,
                "P4_kappa_std_2E" | "P4_kappa_mix_2E" => !odd,
                n if n.starts_with("P6") => odd,
                _ => true,
            };
            if applies {
                table.entry(s.name).or_default().push((u.n, eval(u, &mm, &boots, s)));
            }
        }
        writeln!(desc, "  {} nE={} q={} mode={} T={:?}", u.label(), u.ne, u.q, if u.census { "census" } else { "sample" }, u.counts.iter().map(|c| c[0].len()).collect::<Vec<_>>()).unwrap();
        for (a, name) in ARMS.iter().enumerate() {
            let ar = &u.arms[a];
            let z = |c: usize| {
                let v = &u.counts[c][a];
                v.iter().filter(|&&x| x == 0.0).count() as f64 / v.len() as f64
            };
            writeln!(
                desc,
                "    {name:<4} m={:<4} m0={:<4} m1={:<4} c={:<4} mean 2E={:<10.4} off={:<10.4} odd={:<10.4} kappa 2E={:.3} odd={:.3} | zero frac 2E={:.3} odd={:.3}",
                ar.m, ar.m0, ar.m1, ar.c, mm[s2][a], mm[so][a], mm[sd][a],
                u.kappa(&mm, s2, a), u.kappa(&mm, sd, a), z(s2), z(sd)
            )
            .unwrap();
        }
    }

    let fmt_rows = |rows: &[(u32, Row)]| -> String {
        let mut s = String::new();
        for (_, r) in rows {
            writeln!(s, "    {:<14} obs {:>10.4} 99% CI [{:>10.4}, {:>10.4}] pred {:>10.4} {}", r.unit, r.obs, r.lo, r.hi, r.pred, if r.in_ci() { "in" } else { "MISS" }).unwrap();
        }
        s
    };
    let group = |name: &str, filt: &dyn Fn(u32) -> bool| -> Vec<Row> {
        table.get(name).map(|v| v.iter().filter(|(n, _)| filt(*n)).map(|(_, r)| r.clone()).collect()).unwrap_or_default()
    };
    let odd_f = |n: u32| n % 2 == 1;
    let even_f = |n: u32| n % 2 == 0;
    let any_f = |_: u32| true;
    let rule_line = |out: &mut String, head: &mut BTreeMap<String, f64>, key: &str, rows: &[Row]| -> bool {
        let (ok, miss, u, pooled) = rule(rows);
        writeln!(out, "  {key}: units {u}, misses {miss} (allowed {}), pooled obs/pred {pooled:.4} => {}", std::cmp::max(1, u / 10), if ok { "ok" } else { "FAIL" }).unwrap();
        head.insert(format!("{key}.misses"), miss as f64);
        head.insert(format!("{key}.pooled_obs_over_pred"), pooled);
        ok
    };

    // ---------- P8 instrument ----------
    let mut p8 = true;
    let mut lines = String::new();
    let crit: f64 = units.iter().map(|u| u.crit_mismatch).sum();
    let oddm: f64 = units.iter().map(|u| u.odd_mismatch).sum();
    let bchk: f64 = units.iter().flat_map(|u| u.arms.iter()).map(|a| a.brute_checked).sum();
    let bmis: f64 = units.iter().flat_map(|u| u.arms.iter()).map(|a| a.brute_mismatch).sum();
    let mut ident_bad = 0;
    let mut ident_n = 0;
    for u in &units {
        for a in &u.arms {
            if let Some((x, y, z)) = a.ident {
                ident_n += 1;
                if !(x && y && z) {
                    ident_bad += 1;
                }
            }
        }
    }
    let smallest_brute = units.iter().filter(|u| u.n == 3 && u.d == 3).all(|u| u.arms.iter().all(|a| a.m == 0.0 || a.brute_checked > 0.0));
    let mut alllift_bad = 0;
    for u in units.iter().filter(|u| u.tr_a == 0) {
        let t = &u.arms[tw];
        if t.m != 2.0 * (u.q - 1.0) || t.m1 != 0.0 || u.arms[hz].m1 != 0.0 || u.arms[twh].m1 != 0.0 || u.arms[tz].m1 != 0.0 {
            alllift_bad += 1;
        }
    }
    p8 &= crit == 0.0 && oddm == 0.0 && bmis == 0.0 && ident_bad == 0 && smallest_brute && alllift_bad == 0 && bchk > 0.0;
    writeln!(lines, "  2E criterion vs [nE/2]R = O: mismatches {crit}; odd-class [h]R = O: mismatches {oddm}").unwrap();
    writeln!(lines, "  brute force vs MITM: targets checked {bchk}, mismatches {bmis}; smallest cell fully checked: {smallest_brute}").unwrap();
    writeln!(lines, "  census identities (sum over 2E, off 2E, E): arm-units {ident_n}, failures {ident_bad}").unwrap();
    writeln!(lines, "  tw all-lift m = 2(q-1) and tw, twh, hz, tz inside 2E (Tr(a)=0 units): violations {alllift_bad}").unwrap();
    head.insert("p8_crit_mismatch".into(), crit);
    head.insert("p8_brute_checked".into(), bchk);
    head.insert("p8_brute_mismatch".into(), bmis);
    head.insert("p8_identity_failures".into(), ident_bad as f64);
    head.insert("p8_identity_armunits".into(), ident_n as f64);
    head.insert("p8_alllift_violations".into(), alllift_bad as f64);

    // ---------- P1 ----------
    writeln!(out, "P1 (2E targets: tw/std yield = composition-exact prediction; odd n carries the extra 2):").unwrap();
    let mut p1 = true;
    for (gname, f) in [("odd", &odd_f as &dyn Fn(u32) -> bool), ("even", &even_f)] {
        let r = group("P1_r_2E", f);
        write!(out, "{}", fmt_rows(&table["P1_r_2E"].iter().filter(|(n, _)| f(*n)).cloned().collect::<Vec<_>>())).unwrap();
        p1 &= rule_line(&mut out, &mut head, &format!("P1_r_2E.{gname}"), &r);
        let dd = group("P1_D_2E", f);
        write!(out, "{}", fmt_rows(&table["P1_D_2E"].iter().filter(|(n, _)| f(*n)).cloned().collect::<Vec<_>>())).unwrap();
        p1 &= rule_line(&mut out, &mut head, &format!("P1_D_2E.{gname}"), &dd);
        let pooled_d = geo(&dd.iter().map(|r| r.obs).collect::<Vec<_>>());
        head.insert(format!("P1_D_2E.{gname}.pooled"), pooled_d);
        let (band, lo_ok) = if gname == "odd" {
            let lo_ok = dd.iter().filter(|r| r.lo > 1.25).count();
            (pooled_d >= 1.7 && pooled_d <= 2.3 && lo_ok + std::cmp::max(1, dd.len() / 10) >= dd.len(), lo_ok)
        } else {
            ((0.85..=1.18).contains(&pooled_d), 0)
        };
        writeln!(out, "  P1_D_2E.{gname} pooled kappa ratio {pooled_d:.4} (band {}), units with CI low > 1.25: {lo_ok} => {}", if gname == "odd" { "[1.7, 2.3]" } else { "[0.85, 1.18]" }, if band { "ok" } else { "FAIL" }).unwrap();
        p1 &= band;
    }
    writeln!(out, "P1 => {}\n", pf(p1)).unwrap();
    verd.insert("P1".into(), pf(p1).into());

    // ---------- P2 ----------
    writeln!(out, "P2 (all targets: tw/std = (m_tw/m_std)^n):").unwrap();
    let r = group("P2_r_all", &any_f);
    write!(out, "{}", fmt_rows(&table["P2_r_all"])).unwrap();
    let p2 = rule_line(&mut out, &mut head, "P2_r_all", &r);
    writeln!(out, "P2 => {}\n", pf(p2)).unwrap();
    verd.insert("P2".into(), pf(p2).into());

    // ---------- P3 ----------
    writeln!(out, "P3 (odd n, Tr(a)=0: std equal on and off 2E; tz, tw, twh, hz zero off 2E, kappa 2 on 2E):").unwrap();
    let mut viol = 0f64;
    let mut pairs = 0f64;
    for u in units.iter().filter(|u| u.tr_a == 0 && u.n % 2 == 1) {
        for a in [tz, tw, twh, hz] {
            pairs += u.counts[so][a].len() as f64;
            viol += u.counts[so][a].iter().filter(|&&x| x != 0.0).count() as f64;
        }
    }
    writeln!(out, "  zero off 2E (tz, tw, twh, hz): (arm, target) pairs {pairs}, nonzero {viol}").unwrap();
    head.insert("p3_offE_pairs".into(), pairs);
    head.insert("p3_offE_nonzero".into(), viol);
    let mut p3 = viol == 0.0 && pairs > 0.0;
    for k in ["P3_std_on_over_off", "P3_kappa_tz_2E", "P3_kappa_tw_2E"] {
        let r = group(k, &odd_f);
        write!(out, "{}", fmt_rows(&table[k])).unwrap();
        p3 &= rule_line(&mut out, &mut head, k, &r);
    }
    writeln!(out, "P3 => {}\n", pf(p3)).unwrap();
    verd.insert("P3".into(), pf(p3).into());

    // ---------- P4 ----------
    writeln!(out, "P4 (n = 4 control: std inside 2E, kappa 2; tw/std = (m_tw/m_std)^4 under P1 even):").unwrap();
    let mut p4_exact = 0f64;
    let mut p4_pairs = 0f64;
    let mut p4_m1 = 0f64;
    for u in units.iter().filter(|u| u.tr_a == 0 && u.n % 2 == 0) {
        p4_m1 += u.arms[std].m1;
        p4_pairs += u.counts[so][std].len() as f64;
        p4_exact += u.counts[so][std].iter().filter(|&&x| x != 0.0).count() as f64;
    }
    writeln!(out, "  std points off 2E: {p4_m1}; std nonzero off 2E: {p4_exact} of {p4_pairs}").unwrap();
    head.insert("p4_std_m1".into(), p4_m1);
    head.insert("p4_std_offE_nonzero".into(), p4_exact);
    let mut p4 = p4_m1 == 0.0 && p4_exact == 0.0 && p4_pairs > 0.0;
    for k in ["P4_kappa_std_2E", "P4_kappa_mix_2E"] {
        let r = group(k, &even_f);
        write!(out, "{}", fmt_rows(&table[k])).unwrap();
        p4 &= rule_line(&mut out, &mut head, k, &r);
    }
    p4 &= verd["P1"] == "PASS" || {
        // P4 needs only the even half of P1
        head["P1_r_2E.even.misses"] <= 1.0 && (0.9..=1.1).contains(&head["P1_r_2E.even.pooled_obs_over_pred"])
    };
    writeln!(out, "P4 => {}\n", pf(p4)).unwrap();
    verd.insert("P4".into(), pf(p4).into());

    // ---------- P5 ----------
    writeln!(out, "P5 (odd-order targets: tw/std as on 2E):").unwrap();
    let r = group("P5_r_odd", &any_f);
    write!(out, "{}", fmt_rows(&table["P5_r_odd"])).unwrap();
    let p5 = rule_line(&mut out, &mut head, "P5_r_odd", &r);
    writeln!(out, "P5 => {}\n", pf(p5)).unwrap();
    verd.insert("P5".into(), pf(p5).into());

    // ---------- P6 ----------
    writeln!(out, "P6 (odd n: matched-size trace-homogeneous line hz, sub-base tz, generic line mix):").unwrap();
    let mut p6 = true;
    for k in ["P6_H_2E", "P6_H_odd", "P6_L_2E", "P6_L_odd", "P6_T_2E", "P6_T_odd", "P6_mix_2E"] {
        let r = group(k, &odd_f);
        write!(out, "{}", fmt_rows(&table[k])).unwrap();
        p6 &= rule_line(&mut out, &mut head, k, &r);
        head.insert(format!("{k}.pooled_obs"), geo(&r.iter().map(|x| x.obs).collect::<Vec<_>>()));
    }
    writeln!(out, "P6 => {}\n", pf(p6)).unwrap();
    verd.insert("P6".into(), pf(p6).into());

    // ---------- P7 ----------
    writeln!(out, "P7 (Tr(a) = 1 negative control):").unwrap();
    let mut p7 = true;
    let mut p7n = 0;
    for u in units.iter().filter(|u| u.tr_a == 1) {
        p7n += 1;
        let tw_m = u.arms[tw].m;
        let tz_off: usize = u.counts[so][tz].iter().filter(|&&x| x != 0.0).count();
        let (hz_bad, what) = if u.n % 2 == 1 {
            (
                u.counts[s2][hz].iter().chain(u.counts[sd][hz].iter()).filter(|&&x| x != 0.0).count(),
                "hz nonzero on 2E or odd targets",
            )
        } else {
            (u.counts[so][hz].iter().filter(|&&x| x != 0.0).count(), "hz nonzero off 2E")
        };
        let hz_in = u.arms[hz].m0;
        let ok = tw_m == 0.0 && tz_off == 0 && hz_bad == 0 && hz_in == 0.0 && u.arms[hz].m > 0.0;
        p7 &= ok;
        writeln!(out, "  {}: tw m = {tw_m}; tz nonzero off 2E {tz_off}; hz m = {} (in 2E {hz_in}); {what}: {hz_bad} => {}", u.label(), u.arms[hz].m, if ok { "ok" } else { "VIOLATION" }).unwrap();
    }
    p7 &= p7n > 0;
    writeln!(out, "P7 => {}\n", pf(p7)).unwrap();
    verd.insert("P7".into(), pf(p7).into());

    writeln!(out, "P8 (instrument):\n{lines}P8 => {}\n", pf(p8)).unwrap();
    verd.insert("P8".into(), pf(p8).into());

    // ---------- reported ----------
    writeln!(out, "Reported (no pass rule): relations per trial per column (eta) and yield on odd-order targets, odd n:").unwrap();
    for k in ["R_eta_tw_over_std_odd", "R_eta_tw_over_hz_odd", "R_eta_hz_over_std_odd", "R_yield_tw_over_hz_odd", "R_kappa_twh_2E"] {
        let r = group(k, &odd_f);
        write!(out, "  {k}:\n{}", fmt_rows(&table[k].iter().filter(|(n, _)| *n % 2 == 1).cloned().collect::<Vec<_>>())).unwrap();
        let pooled = geo(&r.iter().map(|x| x.obs).collect::<Vec<_>>());
        writeln!(out, "    pooled obs {pooled:.4}").unwrap();
        head.insert(format!("{k}.pooled_obs"), pooled);
    }

    // ---------- interpretation (frozen rule) ----------
    let hrows: Vec<Row> = group("P6_H_2E", &odd_f).into_iter().chain(group("P6_H_odd", &odd_f)).collect();
    let lrows: Vec<Row> = group("P6_L_2E", &odd_f).into_iter().chain(group("P6_L_odd", &odd_f)).collect();
    let h_pooled = geo(&hrows.iter().map(|r| r.obs).collect::<Vec<_>>());
    let l_pooled = geo(&lrows.iter().map(|r| r.obs).collect::<Vec<_>>());
    let h_lo1 = hrows.iter().filter(|r| r.lo > 1.0).count();
    let d_pooled = head["P1_D_2E.odd.pooled"];
    let need = hrows.len() - std::cmp::max(1, hrows.len() / 10);
    let interp = if (1.6..=2.5).contains(&h_pooled) && h_lo1 >= need && (0.85..=1.18).contains(&l_pooled) {
        "A"
    } else if h_pooled <= 1.25 && d_pooled >= 1.6 {
        "B"
    } else if d_pooled <= 1.25 {
        "C"
    } else {
        "inconclusive"
    };
    writeln!(
        out,
        "\nInterpretation rule (frozen): pooled H = kappa_hz/kappa_std over odd-n units, 2E and odd classes = {h_pooled:.4} (CI low > 1 in {h_lo1} of {}); pooled L = kappa_tw/kappa_hz = {l_pooled:.4}; pooled D (2E) = {d_pooled:.4} => outcome {interp}",
        hrows.len()
    )
    .unwrap();
    head.insert("interp.H_pooled".into(), h_pooled);
    head.insert("interp.L_pooled".into(), l_pooled);
    head.insert("interp.H_ci_low_gt1".into(), h_lo1 as f64);
    verd.insert("interpretation".into(), interp.into());

    let text = format!("Per-unit description (Tr(a) = 0 units):\n{desc}\n{out}");
    Scored { text, headline: head, verdicts: verd }
}

fn pf(b: bool) -> &'static str {
    if b { "PASS" } else { "FAIL" }
}

pub fn score(dir: &str) {
    let s = compute(dir);
    print!("{}", s.text);
    let mut js = String::from("{\n  \"verdicts\": {");
    js += &s.verdicts.iter().map(|(k, v)| format!("\"{k}\": \"{v}\"")).collect::<Vec<_>>().join(", ");
    js += "},\n  \"headline\": {\n";
    js += &s.headline.iter().map(|(k, v)| format!("    \"{k}\": {}", fnum(*v))).collect::<Vec<_>>().join(",\n");
    js += "\n  }\n}\n";
    std::fs::write(format!("{dir}/summary.json"), js).unwrap();
    std::fs::write(format!("{dir}/summary.txt"), &s.text).unwrap();
}

fn fnum(v: f64) -> String {
    if v.is_finite() { format!("{v:.6}") } else { "null".into() }
}

/// pcc `recompute` proof: recompute every claimed headline number from the
/// sha256-pinned raw rows and compare with claimed.json.
pub fn validate(dir: &str, claimed_path: &str, pins: &[String]) {
    let s = compute(dir);
    let claimed = json::parse(&std::fs::read_to_string(claimed_path).unwrap());
    let mut ok = true;
    let mut heads = vec![];
    let J::Obj(entries) = &claimed else { panic!("claimed.json must be an object") };
    for (name, c) in entries {
        let (cv, tol) = (c.get("value"), c.get("rel_tol").num().unwrap_or(0.0));
        let (claimed_s, rec_s, good) = if let Some(x) = cv.num() {
            let rv = *s.headline.get(name.as_str()).unwrap_or(&f64::NAN);
            let good = if tol == 0.0 { x == rv } else { ((x - rv) / x).abs() <= tol };
            (format!("{x}"), fnum(rv), good)
        } else {
            let want = cv.str();
            let got = s.verdicts.get(name.as_str()).cloned().unwrap_or_default();
            (format!("\"{want}\""), format!("\"{got}\""), want == got)
        };
        ok &= good;
        heads.push(format!("    {{\"name\": \"{name}\", \"claimed\": {claimed_s}, \"recomputed\": {rec_s}, \"rel_tol\": {tol}, \"match\": {good}}}"));
    }
    let raw: Vec<String> = pins
        .iter()
        .map(|p| {
            let (f, h) = p.split_once('=').unwrap();
            format!("    {{\"path\": \"{f}\", \"sha256\": \"{h}\"}}")
        })
        .collect();
    println!(
        "{{\n  \"schema\": \"validator-report\",\n  \"status\": \"{}\",\n  \"study\": \"research/odd_n_trace_accounting_20261107\",\n  \"recomputed_by\": {{\"tool\": \"research/odd_n_trace_accounting_20261107 (odd_n_trace validate)\", \"command\": \"odd_n_trace validate <run> claimed.json <file=sha256>...\"}},\n  \"note\": \"Producer-run recompute of README headline numbers and verdicts from the pinned raw rows; unsigned, so independence is not established.\",\n  \"raw_data\": [\n{}\n  ],\n  \"headline\": [\n{}\n  ]\n}}",
        if ok { "pass" } else { "fail" },
        raw.join(",\n"),
        heads.join(",\n")
    );
}
