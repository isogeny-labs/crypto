//! fresh-scaling-v2 design-time check of the amended exponent rule (critic F1)
//! and the amended runtime estimate (F4/F8 arms and ladder). std only, no crates.
//! Every number it prints is a model output, [pcc:unverified]. It reads no
//! measured data except the v1 lane's `estimate.csv` model rows.
//!
//! usage: rule_check <v1 estimate.csv> <out estimate-v2.csv>
//!
//! Part A  re-runs the v1 `estimate.csv` IC rows through the v1 rule (global OLS
//!         of the total) and the amended v2 rule (largest per-phase exponent and
//!         the top-pair local slope), at point values.
//! Part B  Monte Carlo of both rules with the protocol's noise model (n = 32
//!         targets per size, NB(2, pi) descent, Rayleigh rho, stratified
//!         bootstrap) on seven cost scenarios and both ladders.
//! Part C  runtime/CPU-hour estimate for the v2 arms on the v2 ladder.

use std::f64::consts::PI;
use std::fmt::Write as _;

const DELTA: f64 = 0.05; // preregistered exponent margin (draft; owner decides)
const ALPHA_DERIVED: f64 = 1.0 / 3.0; // full factor base: LA ~ p^2 = l^(1/3)
const C_RES: f64 = 0.0038; // s per descent residual (v1 model, Run50 Runpod host)
const PI_ACC: f64 = 1.0 / 1100.0; // descent acceptance rate per residual (v1 model)
const R_MEAN: f64 = 2.0 / PI_ACC; // E[residuals] per run = 2 / pi
const ADD_WALK: f64 = 1.00196; // r = 256 adding-walk factor (rho-fold)
const N_TARGETS: usize = 32;
const REPS: usize = 200;
const BOOT: usize = 400;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn unif(&mut self) -> f64 {
        ((self.next() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
    }
    fn normal(&mut self) -> f64 {
        let (u, v) = (self.unif(), self.unif());
        (-2.0 * u.ln()).sqrt() * (2.0 * PI * v).cos()
    }
    fn lognormal(&mut self, sigma: f64) -> f64 {
        (sigma * self.normal() - 0.5 * sigma * sigma).exp()
    }
    fn geometric(&mut self, p: f64) -> f64 {
        (self.unif().ln() / (1.0 - p).ln()).floor() + 1.0
    }
    fn rayleigh_mean(&mut self, mean: f64) -> f64 {
        let sigma = mean / (PI / 2.0).sqrt();
        sigma * (-2.0 * self.unif().ln()).sqrt()
    }
    fn below(&mut self, n: usize) -> usize {
        (self.unif() * n as f64) as usize % n
    }
}

fn ols(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len() as f64;
    let (mx, my) = (x.iter().sum::<f64>() / n, y.iter().sum::<f64>() / n);
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let sxx: f64 = x.iter().map(|a| (a - mx) * (a - mx)).sum();
    sxy / sxx
}

fn ell(p: f64) -> f64 {
    p.powi(6) / 4.0
}

fn pct(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let i = ((v.len() - 1) as f64 * q).round() as usize;
    v[i]
}

/// Verdict of the amended rule from intervals. `phase_lo/up` are the per-phase
/// exponent bounds, `top_lo/up` the M2 top-pair bounds, `theta` = alpha_rho - delta.
fn v2_verdict(gcov: bool, phase_lo: &[f64], phase_up: &[f64], top_lo: f64, top_up: f64, theta: f64) -> &'static str {
    let max_lo = phase_lo.iter().cloned().fold(f64::MIN, f64::max);
    let max_up = phase_up.iter().cloned().fold(f64::MIN, f64::max);
    if top_lo >= theta || (gcov && max_lo >= theta) {
        return "FAIL";
    }
    if gcov && max_up < theta && top_up < theta {
        if max_up < ALPHA_DERIVED {
            return "HELD";
        }
        return "PASS";
    }
    "UNRESOLVED"
}

fn part_a(csv: &str, out: &mut String) {
    writeln!(out, "## Part A: v1 estimate.csv rows through the v1 and the amended rule (point values)").unwrap();
    // p -> (l, ic e2, ic e3, rho neg-b16, rho min)
    let mut rows: Vec<(f64, f64, f64, f64, f64)> = Vec::new();
    for line in csv.lines().skip(1) {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(',').collect();
        let p: f64 = f[0].parse().unwrap();
        let b: f64 = f[1].parse().unwrap();
        let s: f64 = f[4].parse().unwrap();
        let idx = match rows.iter().position(|r| r.0 == p) {
            Some(i) => i,
            None => {
                rows.push((p, b, 0.0, 0.0, f64::MAX));
                rows.len() - 1
            }
        };
        let r = &mut rows[idx];
        match f[2] {
            "ic-graph(e=2)" => r.2 = s,
            "ic-graph(e=3)" => r.3 = s,
            a if a.starts_with("rho-") && !a.contains("AA") => r.4 = r.4.min(s),
            _ => {}
        }
    }
    let lnl: Vec<f64> = rows.iter().map(|r| r.1 * 2f64.ln()).collect();
    let rho: Vec<f64> = rows.iter().map(|r| r.4.ln()).collect();
    let a_rho = ols(&lnl, &rho);
    let theta = a_rho - DELTA;
    writeln!(out, "sizes p = {:?}", rows.iter().map(|r| r.0 as u64).collect::<Vec<_>>()).unwrap();
    writeln!(out, "rho_ref (cheapest per size, as v1) global OLS alpha = {a_rho:.3}; theta = alpha_rho - delta = {theta:.3} (delta = {DELTA})").unwrap();
    let n = rows.len();
    for (k, e) in [(2usize, 2.0f64), (3, 3.0)] {
        let tot: Vec<f64> = rows.iter().map(|r| if k == 2 { r.2 } else { r.3 }.ln()).collect();
        let a_tot = ols(&lnl, &tot);
        let top = (tot[n - 1] - tot[n - 2]) / (lnl[n - 1] - lnl[n - 2]);
        // phases of the v1 model: non-descent T_nd (p/1009)^e and descent R c_res
        let nd: Vec<f64> = rows.iter().map(|r| (13.0 * (r.0 / 1009.0).powf(e)).ln()).collect();
        let de: Vec<f64> = rows.iter().map(|_| (2000.0 * C_RES).ln()).collect();
        let (a_nd, a_de) = (ols(&lnl, &nd), ols(&lnl, &de));
        let a_star = a_nd.max(a_de);
        let v1 = if a_tot - a_rho < 0.0 { "PASS (smaller exponent)" } else { "not smaller" };
        let v2 = v2_verdict(true, &[a_nd, a_de], &[a_nd, a_de], top, top, theta);
        let v2n = v2_verdict(false, &[a_nd, a_de], &[a_nd, a_de], top, top, theta);
        writeln!(out, "IC model e={e}: v1 global alpha_total = {a_tot:.3} (alpha_total - alpha_rho = {:.3}) -> v1 rule: {v1}", a_tot - a_rho).unwrap();
        writeln!(out, "  amended: per-phase alpha non-descent = {a_nd:.3}, descent = {a_de:.3}; alpha* = {a_star:.3}; M2 top-pair local slope = {top:.3}").unwrap();
        writeln!(out, "  amended rule verdict (G-cov passed) = {v2}; (G-cov failed) = {v2n}").unwrap();
    }
}

#[derive(Clone)]
struct Scenario {
    name: &'static str,
    phases: Vec<(f64, f64)>, // (seconds at p = 1009, exponent in l)
    descent: bool,
}

struct Tally {
    v1: [usize; 4],
    v2: [usize; 4],
    v2n: [usize; 4],
    a_tot: Vec<f64>,
    top: Vec<f64>,
    star: Vec<f64>,
}

fn idx(v: &str) -> usize {
    match v {
        "PASS" => 0,
        "FAIL" => 1,
        "HELD" => 2,
        _ => 3,
    }
}

fn simulate(sc: &Scenario, ladder: &[f64], seed: u64) -> Tally {
    let mut rng = Rng(seed);
    let l_ref = ell(1009.0);
    let lnl: Vec<f64> = ladder.iter().map(|&p| ell(p).ln()).collect();
    let s = ladder.len();
    let nph = sc.phases.len() + usize::from(sc.descent);
    let mut t = Tally { v1: [0; 4], v2: [0; 4], v2n: [0; 4], a_tot: vec![], top: vec![], star: vec![] };
    for _ in 0..REPS {
        // data[size][target] = (phase costs, rho cost)
        let mut data: Vec<Vec<(Vec<f64>, f64)>> = Vec::with_capacity(s);
        for &p in ladder {
            let l = ell(p);
            let e_iter = (PI * l / 4.0).sqrt() * ADD_WALK;
            let mut col = Vec::with_capacity(N_TARGETS);
            for _ in 0..N_TARGETS {
                let mut ph: Vec<f64> = sc
                    .phases
                    .iter()
                    .map(|&(c0, a)| c0 * (l / l_ref).powf(a) * rng.lognormal(0.05))
                    .collect();
                if sc.descent {
                    let r = rng.geometric(PI_ACC) + rng.geometric(PI_ACC);
                    ph.push(r * C_RES * rng.lognormal(0.03));
                }
                let rho = rng.rayleigh_mean(e_iter) * 166.75e-9;
                col.push((ph, rho));
            }
            data.push(col);
        }
        let stats = |sel: &dyn Fn(usize) -> usize| -> (Vec<f64>, f64, f64, f64) {
            // per-phase slopes, total slope, top-pair slope, rho slope
            let mut mph = vec![vec![0.0; s]; nph];
            let mut mt = vec![0.0; s];
            let mut mr = vec![0.0; s];
            for (k, col) in data.iter().enumerate() {
                for i in 0..N_TARGETS {
                    let (ph, rho) = &col[sel(k * N_TARGETS + i) % N_TARGETS];
                    for (j, c) in ph.iter().enumerate() {
                        mph[j][k] += c;
                        mt[k] += c;
                    }
                    mr[k] += rho;
                }
            }
            let lg = |v: &Vec<f64>| v.iter().map(|x| (x / N_TARGETS as f64).ln()).collect::<Vec<f64>>();
            let a_ph: Vec<f64> = mph.iter().map(|v| ols(&lnl, &lg(v))).collect();
            let lt = lg(&mt);
            let a_t = ols(&lnl, &lt);
            let top = (lt[s - 1] - lt[s - 2]) / (lnl[s - 1] - lnl[s - 2]);
            (a_ph, a_t, top, ols(&lnl, &lg(&mr)))
        };
        let (pa, pt, ptop, prho) = stats(&|i| i % N_TARGETS);
        let mut bph = vec![Vec::with_capacity(BOOT); nph];
        let (mut bdiff, mut btop) = (Vec::with_capacity(BOOT), Vec::with_capacity(BOOT));
        for _ in 0..BOOT {
            let draws: Vec<usize> = (0..s * N_TARGETS).map(|_| rng.below(N_TARGETS)).collect();
            let (a, at, tp, ar) = stats(&|i| draws[i]);
            for j in 0..nph {
                bph[j].push(a[j]);
            }
            bdiff.push(at - ar);
            btop.push(tp);
        }
        let lo: Vec<f64> = bph.iter_mut().map(|v| pct(v, 0.025)).collect();
        let up: Vec<f64> = bph.iter_mut().map(|v| pct(v, 0.975)).collect();
        let (dlo, dup) = (pct(&mut bdiff, 0.025), pct(&mut bdiff, 0.975));
        let (tlo, tup) = (pct(&mut btop, 0.025), pct(&mut btop, 0.975));
        let theta = prho - DELTA;
        let v1 = if dup < 0.0 { "PASS" } else if dlo > 0.0 { "FAIL" } else { "UNRESOLVED" };
        t.v1[idx(v1)] += 1;
        t.v2[idx(v2_verdict(true, &lo, &up, tlo, tup, theta))] += 1;
        t.v2n[idx(v2_verdict(false, &lo, &up, tlo, tup, theta))] += 1;
        t.a_tot.push(pt);
        t.top.push(ptop);
        t.star.push(pa.iter().cloned().fold(f64::MIN, f64::max));
    }
    t
}

fn part_b(out: &mut String) {
    writeln!(out, "\n## Part B: Monte Carlo of both rules ({REPS} simulated studies x {BOOT} stratified bootstrap resamples, n = {N_TARGETS} per size)").unwrap();
    writeln!(out, "noise: non-descent phases lognormal sigma 0.05; descent R ~ NB(2, 1/1100) x {C_RES} s x lognormal 0.03; rho = rho-neg-b16, Rayleigh steps with mean sqrt(pi l/4) x {ADD_WALK}, 166.75 ns/step").unwrap();
    writeln!(out, "v1 rule: PASS iff the 95% interval of alpha_total - alpha_rho lies below 0 (global OLS of the total).").unwrap();
    writeln!(out, "v2 rule: FAIL iff M2 top-pair lower >= theta or (G-cov) some phase lower >= theta; PASS iff G-cov and every phase upper and the top-pair upper < theta, HELD if additionally max phase upper < alpha_derived = {ALPHA_DERIVED:.3}; else UNRESOLVED. theta = alpha_rho_fit - {DELTA}.").unwrap();
    let l2 = 1.0 / 3.0;
    let s3_b = {
        // steep phase with a 1% share of phase A at the smallest v2 size p = 101
        let r = (ell(101.0) / ell(1009.0)).powf(0.25 - 0.75);
        0.01 * 14.85 * r
    };
    let scen = vec![
        Scenario { name: "S1 deferred T_nd 14.85 s, e=2 (phase alpha 1/3) + NB descent", phases: vec![(14.85, l2)], descent: true },
        Scenario { name: "S2 deferred T_nd 14.85 s, e=3 (phase alpha 1/2) + NB descent", phases: vec![(14.85, 0.5)], descent: true },
        Scenario { name: "S3 table T_nd 2.14 s, e=2 + NB descent", phases: vec![(2.14, l2)], descent: true },
        Scenario { name: "S4 table T_nd 2.14 s, e=3 + NB descent", phases: vec![(2.14, 0.5)], descent: true },
        Scenario { name: "S5 lesson L-eda09: phase alpha 0.25 + 1%-share phase alpha 0.75 + NB descent", phases: vec![(14.85, 0.25), (s3_b, 0.75)], descent: true },
        Scenario { name: "S6 fixed cost: constant 10 s phase + NB descent (alpha 0)", phases: vec![(10.0, 0.0)], descent: true },
        Scenario { name: "S7 null: every phase alpha 1/2 (IC proportional to rho)", phases: vec![(14.85, 0.5), (8.4, 0.5)], descent: false },
    ];
    let ladders: [(&str, Vec<f64>); 2] = [
        ("v1 ladder 127..2017", vec![127.0, 223.0, 383.0, 661.0, 1153.0, 2017.0]),
        ("v2 ladder 101..1511", vec![101.0, 127.0, 223.0, 383.0, 661.0, 1153.0, 1511.0]),
    ];
    writeln!(out, "| scenario | ladder | median alpha_total (v1 stat) | median top-pair | median alpha* (max phase) | v1 PASS/FAIL/UNRES | v2 G-cov passed PASS/FAIL/HELD/UNRES | v2 G-cov failed PASS/FAIL/HELD/UNRES |").unwrap();
    writeln!(out, "|---|---|---:|---:|---:|---|---|---|").unwrap();
    for (k, sc) in scen.iter().enumerate() {
        for (j, (lname, lad)) in ladders.iter().enumerate() {
            let mut t = simulate(sc, lad, 0x5EED_0000 + (k as u64) * 16 + j as u64);
            let (m1, m2, m3) = (pct(&mut t.a_tot, 0.5), pct(&mut t.top, 0.5), pct(&mut t.star, 0.5));
            writeln!(
                out,
                "| {} | {} | {m1:.3} | {m2:.3} | {m3:.3} | {}/{}/{} | {}/{}/{}/{} | {}/{}/{}/{} |",
                sc.name, lname, t.v1[0], t.v1[1], t.v1[3], t.v2[0], t.v2[1], t.v2[2], t.v2[3], t.v2n[0], t.v2n[1], t.v2n[2], t.v2n[3]
            )
            .unwrap();
        }
    }
}

fn part_c(path: &str, out: &mut String) {
    writeln!(out, "\n## Part C: v2 runtime estimate (model, n = {N_TARGETS} public targets per size)").unwrap();
    let ladder = [101.0f64, 127.0, 223.0, 383.0, 661.0, 1153.0, 1511.0];
    // seconds/step at p1009 on the Run50 Runpod host (process real_s / steps), one walk each
    let rho: [(&str, f64, f64); 4] = [
        ("rho-neg-b1 (M1 rho_ref)", 117.565 / 535_450_837.0, 4.0),
        ("rho-neg-b16 (M2 rho_ref)", 102.924 / 617_229_160.0, 4.0),
        ("rho-plain-b16 (negative control)", 36.908 / 249_417_689.0, 2.0),
        ("rho-aa (neg-b16 independent seed)", 102.924 / 617_229_160.0, 4.0),
    ];
    // IC non-descent seconds at p1009: Run50 process seconds minus 3415 residuals x C_RES
    let ic: [(&str, f64); 3] = [
        ("ic-table", 15.116 - 3415.0 * C_RES),
        ("ic-deferred", 27.829 - 3415.0 * C_RES),
        ("ic-table-aa", 15.116 - 3415.0 * C_RES),
    ];
    let mut csv = String::from("p,b_log2_l,arm,model,seconds_per_target_e2,seconds_per_target_e3,cpu_hours_n32_e2,cpu_hours_n32_e3\n");
    let n = N_TARGETS as f64;
    let (mut tot2, mut tot3, mut rho_tot, mut pilot) = (0.0, 0.0, 0.0, 0.0);
    for &p in &ladder {
        let l = ell(p);
        let b = l.log2();
        let mut size2 = 0.0;
        let mut size3 = 0.0;
        for (name, sps, div) in rho {
            let s = (PI * l / div).sqrt() * ADD_WALK * sps;
            size2 += s;
            size3 += s;
            rho_tot += s * n;
            writeln!(csv, "{p},{b:.2},{name},E[iter] x s/step,{s:.2},{s:.2},{:.3},{:.3}", s * n / 3600.0, s * n / 3600.0).unwrap();
        }
        writeln!(csv, "{p},{b:.2},rho-neg-b1-karatsuba-priced (M1 only),derived from rho-neg-b1 steps; no run,0,0,0,0").unwrap();
        let mut ic_pilot = 0.0;
        for (name, tnd) in ic {
            let s2 = tnd * (p / 1009.0).powi(2) + R_MEAN * C_RES;
            let s3 = tnd * (p / 1009.0).powi(3) + R_MEAN * C_RES;
            size2 += s2;
            size3 += s3;
            if !name.ends_with("-aa") {
                ic_pilot += s3;
            }
            writeln!(csv, "{p},{b:.2},{name},T_nd (p/1009)^e + (2/pi) c_res,{s2:.2},{s3:.2},{:.3},{:.3}", s2 * n / 3600.0, s3 * n / 3600.0).unwrap();
        }
        // pilot: 2 planted targets x (ic-table, ic-deferred, rho-neg-b16), overflow-checks build at 1.5x
        let rho16 = (PI * l / 4.0).sqrt() * ADD_WALK * rho[1].1;
        pilot += 2.0 * 1.5 * (ic_pilot + rho16);
        tot2 += size2 * n;
        tot3 += size3 * n;
        writeln!(out, "p = {p}: b = {b:.2}; all arms x 32 = {:.2} CPU-h (e=2), {:.2} CPU-h (e=3)", size2 * n / 3600.0, size3 * n / 3600.0).unwrap();
    }
    // bridge fixture: Run50 p1009 public target once per arm per host (7 arms, Run50 process seconds)
    let bridge = 15.116 * 2.0 + 27.829 + 117.565 + 102.924 * 2.0 + 36.908;
    writeln!(csv, "# totals: measured arms n=32 {:.1} CPU-h (e=2) / {:.1} CPU-h (e=3); rho share {:.1} CPU-h; pilot (overflow-checks build, 2 planted per size) {:.2} CPU-h; bridge {:.3} CPU-h per host", tot2 / 3600.0, tot3 / 3600.0, rho_tot / 3600.0, pilot / 3600.0, bridge / 3600.0).unwrap();
    std::fs::write(path, &csv).unwrap();
    writeln!(out, "TOTAL measured arms: {:.1} CPU-h (e=2), {:.1} CPU-h (e=3); rho share {:.1} CPU-h", tot2 / 3600.0, tot3 / 3600.0, rho_tot / 3600.0).unwrap();
    writeln!(out, "pilot (planted, overflow-checks build at 1.5x, e=3): {:.2} CPU-h; bridge fixture per host: {:.3} CPU-h", pilot / 3600.0, bridge / 3600.0).unwrap();
    let opt_kara = ladder.iter().map(|&p| (PI * ell(p) / 4.0).sqrt() * ADD_WALK * rho[0].1 * 0.632).sum::<f64>() * n / 3600.0;
    writeln!(out, "optional measured Karatsuba rho arm (only if the opt-in Arith lands; 0.632 x neg-b1 per-step bitops used as a wall proxy, unmeasured): {opt_kara:.1} CPU-h").unwrap();
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let csv = std::fs::read_to_string(&a[1]).expect("v1 estimate.csv");
    let mut out = String::from("# fresh-scaling-v2 rule check (model only, [pcc:unverified])\n");
    part_a(&csv, &mut out);
    part_b(&mut out);
    part_c(&a[2], &mut out);
    print!("{out}");
}
