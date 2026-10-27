//! Native scoring of PROTOCOL.md sections 3-5 from rows.jsonl files.
//! usage: summarize <run dir>   (reads yield_k*.jsonl, cost_k*.jsonl, validate.jsonl)
//! writes summary.txt and summary.json. Written after the data, implements the
//! frozen decision rules of PROTOCOL.md verbatim.
#[path = "../json.rs"]
mod json;
use json::*;
use std::collections::BTreeMap;
use std::fmt::Write as _;

const SEED: u64 = 0xC1_7715_7ED_2610_27;
struct Rng(u64);
impl Rng {
    fn new(tag: &str) -> Rng {
        // same derivation as common.rs::Rng::new
        let mut h = SEED ^ 0xCBF2_9CE4_8422_2325;
        for b in tag.bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01B3);
        }
        let mut r = Rng(h);
        r.next();
        r
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, m: usize) -> usize {
        (self.next() % m as u64) as usize
    }
}

fn read_rows(dir: &str, prefix: &str) -> Vec<J> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.starts_with(prefix) && n.ends_with(".jsonl"))
        .collect();
    files.sort();
    let mut rows = vec![];
    for f in files {
        for line in std::fs::read_to_string(format!("{dir}/{f}")).unwrap().lines() {
            if !line.trim().is_empty() {
                rows.push(parse(line));
            }
        }
    }
    rows
}

fn fj(x: f64) -> String {
    if x.is_finite() { format!("{x:.5}") } else { "null".into() }
}
fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn sd(v: &[f64]) -> f64 {
    let m = mean(v);
    (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (v.len() as f64 - 1.0)).sqrt()
}
fn pct(sorted: &[f64], p: f64) -> f64 {
    let i = ((sorted.len() as f64 - 1.0) * p).round() as usize;
    sorted[i]
}

/// 95 % paired bootstrap interval of mean(a)/mean(b) over targets.
fn boot_ratio(a: &[f64], b: &[f64], rng: &mut Rng, reps: usize) -> (f64, f64) {
    let n = a.len();
    let mut rs = vec![];
    for _ in 0..reps {
        let (mut sa, mut sb) = (0.0, 0.0);
        for _ in 0..n {
            let i = rng.below(n);
            sa += a[i];
            sb += b[i];
        }
        if sb > 0.0 && sa >= 0.0 {
            rs.push(sa / sb);
        }
    }
    if rs.len() < reps / 2 {
        return (f64::NAN, f64::NAN);
    }
    rs.sort_by(|x, y| x.partial_cmp(y).unwrap());
    (pct(&rs, 0.025), pct(&rs, 0.975))
}

struct YRow {
    k: u32,
    curve: String,
    family: String,
    arm: String,
    lifting: f64,
    q1: f64,
    m: f64,
    order: Option<f64>,
    pred: Option<f64>,
    counts: Vec<f64>,
    c_o: f64,
    c_t: f64,
}

fn main() {
    let dir = std::env::args().nth(1).expect("run dir");
    let mut txt = String::new();
    let mut js = String::from("{");
    // ------------------------------------------------------------ yield
    let rows = read_rows(&dir, "yield_");
    let mut yr: Vec<YRow> = vec![];
    for r in &rows {
        yr.push(YRow {
            k: r.get("k").num().unwrap() as u32,
            curve: r.get("curve").str().to_string(),
            family: r.get("family").str().to_string(),
            arm: r.get("arm").str().to_string(),
            lifting: r.get("lifting").num().unwrap(),
            q1: r.get("q_minus_1").num().unwrap(),
            m: r.get("fb_points").num().unwrap(),
            order: r.get("order").num(),
            pred: r.get("pred_excl").num(),
            counts: r.get("counts").nums(),
            c_o: r.get("c_O").num().unwrap(),
            c_t: r.get("c_T").num().unwrap(),
        });
    }
    let mut rng = Rng::new("bootstrap");
    writeln!(txt, "== YIELD (exact ordered decompositions per target; PROTOCOL section 3) ==\n").unwrap();
    writeln!(txt, "-- H-Y1 rows: z = (mean - pred)/(s/sqrt T), empirical variance --").unwrap();
    writeln!(txt, "{:>2} {:<4} {:<22} {:>6} {:>6} {:>9} {:>10} {:>10} {:>7} {:>6}", "k", "curve", "arm", "FBpts", "T", "mean N", "pred", "sd", "z", "tested").unwrap();
    let (mut z_tested, mut z_pass, mut z_gt5, mut z_const) = (0, 0, 0, 0);
    let mut zs: Vec<f64> = vec![];
    let mut y1_rows = String::new();
    for r in &yr {
        if r.m == 0.0 {
            continue;
        }
        let t = r.counts.len() as f64;
        let mu = mean(&r.counts);
        let s = sd(&r.counts);
        let (zstr, tested, z) = match r.pred {
            Some(p) => {
                // Constant counts (s = 0) stay in the gate: z = 0 if the
                // constant equals pred, else +-inf (a failing row).
                let z = if s > 0.0 {
                    (mu - p) / (s / t.sqrt())
                } else if mu == p {
                    0.0
                } else {
                    (mu - p).signum() * f64::INFINITY
                };
                let tested = t * p >= 50.0;
                (format!("{z:7.2}"), tested, z)
            }
            None => ("     -".into(), false, f64::NAN),
        };
        if tested {
            z_tested += 1;
            if z.abs() <= 3.0 {
                z_pass += 1;
            }
            if z.abs() > 5.0 {
                z_gt5 += 1;
            }
            if z.is_finite() {
                zs.push(z);
            } else {
                z_const += 1;
            }
        }
        writeln!(txt, "{:>2} {:<4} {:<22} {:>6} {:>6} {:>9.4} {:>10} {:>10.4} {} {:>6}", r.k, r.curve, r.arm, r.m, t, mu, r.pred.map(|p| format!("{p:.4}")).unwrap_or("-".into()), s, zstr, tested).unwrap();
        write!(y1_rows, "{{\"k\":{},\"curve\":\"{}\",\"arm\":\"{}\",\"fb\":{},\"T\":{},\"mean\":{:.6},\"pred\":{},\"z\":{}}},", r.k, r.curve, r.arm, r.m, t, mu, r.pred.map(|p| format!("{p:.6}")).unwrap_or("null".into()), if z.is_finite() { format!("{z:.4}") } else { "null".into() }).unwrap();
    }
    let zm = if zs.is_empty() { f64::NAN } else { mean(&zs) };
    let zsd = if zs.len() > 1 { sd(&zs) } else { f64::NAN };
    let hy1 = z_tested > 0 && (z_pass as f64) >= 0.9 * z_tested as f64 && z_gt5 == 0;
    writeln!(txt, "\nH-Y1: tested rows {z_tested}, |z|<=3 on {z_pass}, |z|>5 on {z_gt5} ({z_const} of them constant counts != pred, z infinite); finite z mean {zm:.3} sd {zsd:.3}  => {}\n", if hy1 { "PASS" } else { "FAIL" }).unwrap();

    // ratios tw/std and tw_a/tw_b by cell
    let mut cells: BTreeMap<(u32, String), Vec<&YRow>> = BTreeMap::new();
    for r in &yr {
        cells.entry((r.k, r.curve.clone())).or_default().push(r);
    }
    writeln!(txt, "-- H-Y2/H-Y3 yield ratio tw/std on the same targets (95 % paired bootstrap, 2000) --").unwrap();
    writeln!(txt, "{:>2} {:<4} {:<22} {:>8} {:>8} {:>8} {:>10} {:>9} {:>7}", "k", "curve", "arm", "R_obs", "CI lo", "CI hi", "R_pred", "R_pred_src", "covers").unwrap();
    let mut y2: Vec<(u32, String, String, f64, f64, f64, f64, bool, bool)> = vec![]; // k,curve,arm,obs,lo,hi,pred,covers,exactpred
    let mut pooled: BTreeMap<String, (f64, f64, f64, f64)> = BTreeMap::new(); // set -> (sum mean tw, sum mean std, sum pred tw, sum pred std)
    for ((k, curve), rs) in &cells {
        if curve == "NEG" {
            continue;
        }
        let Some(std) = rs.iter().find(|r| r.arm == "std") else { continue };
        for tw in rs.iter().filter(|r| r.arm.starts_with("tw")) {
            let (lo, hi) = boot_ratio(&tw.counts, &std.counts, &mut rng, 2000);
            let obs = mean(&tw.counts) / mean(&std.counts);
            let (pred, src, exact) = match (tw.pred, std.pred) {
                (Some(a), Some(b)) => (a / b, "exact", true),
                _ => ((tw.m / std.m).powi(4), "m^4", false),
            };
            let covers = lo <= pred && pred <= hi;
            writeln!(txt, "{:>2} {:<4} {:<22} {:>8.3} {:>8.3} {:>8.3} {:>10.3} {:>9} {:>7}", k, curve, tw.arm, obs, lo, hi, pred, src, covers).unwrap();
            y2.push((*k, curve.clone(), tw.arm.clone(), obs, lo, hi, pred, covers, exact));
            if let (Some(a), Some(b)) = (tw.pred, std.pred) {
                let set = if curve == "A0" { "A0" } else { "tau1" };
                let e = pooled.entry(set.to_string()).or_insert((0.0, 0.0, 0.0, 0.0));
                e.0 += mean(&tw.counts);
                e.1 += mean(&std.counts);
                e.2 += a;
                e.3 += b;
            }
        }
    }
    for (set, label) in [("tau1", "H-Y2 (tau=1 curves A1 A2 B*)"), ("A0", "H-Y3 (A0, tau=0 control)")] {
        let sel: Vec<_> = y2.iter().filter(|x| (x.1 == "A0") == (set == "A0")).collect();
        let cov = sel.iter().filter(|x| x.7).count();
        let (pooled_obs, pooled_pred) = pooled.get(set).map(|p| ((p.0 / p.1) , (p.2 / p.3))).unwrap_or((f64::NAN, f64::NAN));
        let rr = pooled_obs / pooled_pred;
        let pass = !sel.is_empty() && (cov as f64) >= 0.8 * sel.len() as f64 && rr >= 0.9 && rr <= 1.1;
        writeln!(txt, "{label}: R_pred inside the interval in {cov}/{}; pooled (sum of means) obs {pooled_obs:.4} / pred {pooled_pred:.4} = {rr:.4}  => {}", sel.len(), if pass { "PASS" } else { "FAIL" }).unwrap();
    }
    // negative control
    writeln!(txt, "\n-- H-Y4 negative control NEG (Tr(a)=1) --").unwrap();
    let mut neg_ok = true;
    for r in yr.iter().filter(|r| r.curve == "NEG") {
        let tot: f64 = r.counts.iter().sum();
        writeln!(txt, "k={} {:<22} lifting abscissae {} of {}, FB points {}, sum of counts over {} targets {}", r.k, r.arm, r.lifting, r.q1, r.m, r.counts.len(), tot).unwrap();
        if r.arm.starts_with("tw") && (r.lifting != 0.0 || tot != 0.0) {
            neg_ok = false;
        }
    }
    writeln!(txt, "H-Y4: {}\n", if neg_ok { "PASS" } else { "FAIL" }).unwrap();
    // lifting counts of all arms
    writeln!(txt, "-- lifting abscissae on the line (of Q-1) --").unwrap();
    for ((k, curve), rs) in &cells {
        let s: Vec<String> = rs.iter().map(|r| format!("{}:{}", r.arm, r.lifting)).collect();
        writeln!(txt, "k={k} {curve:<4} Q-1={} {}", rs[0].q1, s.join("  ")).unwrap();
    }
    // A/A
    writeln!(txt, "\n-- H-Y5 A/A: tw_a / tw_b on the same targets --").unwrap();
    writeln!(txt, "{:>2} {:<4} {:>8} {:>8} {:>8} {:>10} {:>7}", "k", "curve", "R_obs", "CI lo", "CI hi", "R_pred", "has 1").unwrap();
    let mut aa_has1 = 0;
    let mut aa_n = 0;
    let (mut aa_sa, mut aa_sb) = (0.0, 0.0);
    let mut aa_rows = String::new();
    for ((k, curve), rs) in &cells {
        if curve == "NEG" {
            continue;
        }
        let a = rs.iter().find(|r| r.arm.starts_with("tw_a"));
        let b = rs.iter().find(|r| r.arm.starts_with("tw_b"));
        if let (Some(a), Some(b)) = (a, b) {
            let (lo, hi) = boot_ratio(&a.counts, &b.counts, &mut rng, 2000);
            let obs = mean(&a.counts) / mean(&b.counts);
            let pred = match (a.pred, b.pred) {
                (Some(x), Some(y)) => x / y,
                _ => 1.0,
            };
            let has1 = lo <= 1.0 && 1.0 <= hi;
            aa_n += 1;
            aa_has1 += has1 as usize;
            aa_sa += mean(&a.counts);
            aa_sb += mean(&b.counts);
            writeln!(txt, "{:>2} {:<4} {:>8.3} {:>8.3} {:>8.3} {:>10.4} {:>7}", k, curve, obs, lo, hi, pred, has1).unwrap();
            write!(aa_rows, "{{\"k\":{k},\"curve\":\"{curve}\",\"obs\":{},\"lo\":{},\"hi\":{}}},", fj(obs), fj(lo), fj(hi)).unwrap();
        }
    }
    let aa_pool = aa_sa / aa_sb;
    let aa_pass = aa_n > 0 && aa_has1 as f64 >= 0.8 * aa_n as f64 && (0.9..=1.1).contains(&aa_pool);
    writeln!(txt, "H-Y5: interval contains 1 in {aa_has1}/{aa_n}; pooled ratio {aa_pool:.4} => {}\n", if aa_pass { "PASS" } else { "FAIL" }).unwrap();

    // ------------------------------------------------------------ validation
    let vrows = read_rows(&dir, "validate");
    if !vrows.is_empty() {
        writeln!(txt, "== VALIDATION (k=5; PROTOCOL section 5) ==").unwrap();
        let (mut rel, mut viol, mut nin, mut mm) = (0.0, 0.0, 0.0, 0.0);
        for v in &vrows {
            rel += v.get("ordered_relations").num().unwrap();
            viol += v.get("equation_violations").num().unwrap();
            nin += v.get("abscissae_not_in_u_Fq").num().unwrap();
            mm += v.get("mitm_vs_enumeration_mismatches").num().unwrap();
        }
        writeln!(txt, "cells {}: ordered relations enumerated {rel}, equation violations {viol}, abscissae with x/u not in F_Q {nin}, pair-table vs enumeration mismatches {mm}\n", vrows.len()).unwrap();
    }

    // ------------------------------------------------------------ cost
    let crows = read_rows(&dir, "cost_");
    struct C {
        k: u32,
        curve: String,
        family: String,
        arm: String,
        target: u32,
        rank: f64,
        hf: f64,
        rows: f64,
        cols: f64,
        closed: bool,
        mults: f64,
        invs: f64,
        setup: f64,
        std_max: f64,
        bytes: f64,
    }
    let mut cs: Vec<C> = vec![];
    let mut lad: Vec<&J> = vec![];
    let mut linechecks = vec![];
    for r in &crows {
        match r.get("kind").str() {
            "cost" => cs.push(C {
                k: r.get("k").num().unwrap() as u32,
                curve: r.get("curve").str().to_string(),
                family: r.get("family").str().to_string(),
                arm: r.get("arm").str().to_string(),
                target: r.get("target").num().unwrap() as u32,
                rank: r.get("rank").num().unwrap(),
                hf: r.get("hf").num().unwrap(),
                rows: r.get("rows").num().unwrap(),
                cols: r.get("cols").num().unwrap(),
                closed: r.get("closed").bool(),
                mults: r.get("mults").num().unwrap(),
                invs: r.get("invs").num().unwrap(),
                setup: r.get("setup_muls").num().unwrap(),
                std_max: r.get("std_max_deg").num().unwrap(),
                bytes: r.get("matrix_bytes").num().unwrap(),
            }),
            "ladder" => lad.push(r),
            "linecheck" => linechecks.push(r),
            _ => {}
        }
    }
    writeln!(txt, "== COST (counted F_Q multiplications of the D=26 weighted Macaulay elimination; PROTOCOL section 4) ==\n").unwrap();
    // linecheck
    let lc_bad: Vec<String> = linechecks
        .iter()
        .filter(|r| {
            let curve = r.get("curve").str();
            let arm = r.get("arm").str();
            let l = r.get("lifting").num().unwrap();
            let n = r.get("sampled").num().unwrap();
            arm.starts_with("tw") && l != n && curve != "NEG"
        })
        .map(|r| format!("k={} {} {}", r.get("k").num().unwrap(), r.get("curve").str(), r.get("arm").str()))
        .collect();
    writeln!(txt, "linecheck: {} arm-cells sampled 64 abscissae each; twisted arms with a non-lifting sampled abscissa: {}", linechecks.len(), lc_bad.len()).unwrap();
    // ladder
    writeln!(txt, "\n-- matrix dimensions per degree step (target 0), cells A1 and B1 --").unwrap();
    writeln!(txt, "{:>2} {:<3} {:<22} {:>3} {:>6} {:>6} {:>6} {:>5} {:>7} {:>12}", "k", "cv", "arm", "D", "rows", "cols", "rank", "HF", "stdmax", "mults").unwrap();
    for r in &lad {
        let k = r.get("k").num().unwrap() as u32;
        if k != 5 && k != 11 {
            continue;
        }
        let d = r.get("D").num().unwrap();
        if d < 14.0 && d != 8.0 {
            continue;
        }
        writeln!(txt, "{:>2} {:<3} {:<22} {:>3} {:>6} {:>6} {:>6} {:>5} {:>7} {:>12}", k, r.get("curve").str(), r.get("arm").str(), d, r.get("rows").num().unwrap(), r.get("cols").num().unwrap(), r.get("rank").num().unwrap(), r.get("hf").num().unwrap(), r.get("std_max_deg").num().unwrap(), r.get("mults").num().unwrap()).unwrap();
    }
    // key by (k,curve,target,arm)
    let mut by: BTreeMap<(u32, String, u32), BTreeMap<String, &C>> = BTreeMap::new();
    for c in &cs {
        let arm = if c.arm.starts_with("tw_a") { "tw_a" } else if c.arm.starts_with("tw_b") { "tw_b" } else { "std" };
        by.entry((c.k, c.curve.clone(), c.target)).or_default().insert(arm.to_string(), c);
    }
    // arm statistics
    writeln!(txt, "\n-- per-arm statistics over all solves --").unwrap();
    for arm in ["std", "tw_a", "tw_b"] {
        let v: Vec<&C> = cs.iter().filter(|c| c.arm.starts_with(arm)).collect();
        let closed = v.iter().filter(|c| c.closed).count();
        let hf512 = v.iter().filter(|c| c.hf == 512.0).count();
        let m: Vec<f64> = v.iter().filter(|c| c.closed).map(|c| c.mults).collect();
        writeln!(txt, "{arm:<5}: solves {}, closed (std_max_deg<26) {} ({:.3}), HF=512 {} , mean mults (closed) {:.4e}, cv {:.3}", v.len(), closed, closed as f64 / v.len() as f64, hf512, mean(&m), sd(&m) / mean(&m)).unwrap();
    }
    let all_dims_same = cs.iter().all(|c| c.rows == cs[0].rows && c.cols == cs[0].cols);
    writeln!(txt, "matrix dimension at D=26: rows {} cols {} for every solve: {}; memory (dense u16) {:.1} MB", cs[0].rows, cs[0].cols, all_dims_same, cs[0].bytes / 1e6).unwrap();
    // H-C1
    let (mut pairs, mut same_rank, mut same_hf) = (0, 0, 0);
    for (_, m) in &by {
        if let Some(s) = m.get("std") {
            for a in ["tw_a", "tw_b"] {
                if let Some(t) = m.get(a) {
                    pairs += 1;
                    same_rank += (s.rank == t.rank) as usize;
                    same_hf += (s.hf == t.hf) as usize;
                }
            }
        }
    }
    let hc1 = pairs > 0 && same_rank as f64 >= 0.9 * pairs as f64 && same_hf as f64 >= 0.9 * pairs as f64;
    writeln!(txt, "\nH-C1: pairs (std,tw) {pairs}; rank equal {same_rank}; HF equal {same_hf} => {}", if hc1 { "PASS" } else { "FAIL" }).unwrap();
    // H-C3
    let frac = |arm: &str| {
        let v: Vec<&C> = cs.iter().filter(|c| c.arm.starts_with(arm)).collect();
        v.iter().filter(|c| c.closed).count() as f64 / v.len() as f64
    };
    let (fs, fa, fb) = (frac("std"), frac("tw_a"), frac("tw_b"));
    let ftw = (fa + fb) / 2.0;
    let hc3 = fs >= 0.9 && ftw >= 0.9 && (fs - ftw).abs() <= 0.1;
    writeln!(txt, "H-C3: closed fraction std {fs:.3}, tw_a {fa:.3}, tw_b {fb:.3} => {}", if hc3 { "PASS" } else { "FAIL" }).unwrap();
    let nonclosed: Vec<String> = cs.iter().filter(|c| !c.closed).map(|c| format!("k={} {} t{} {} (HF {}, stdmax {})", c.k, c.curve, c.target, c.arm, c.hf, c.std_max)).collect();
    writeln!(txt, "non-closed systems ({}):", nonclosed.len()).unwrap();
    for s in &nonclosed {
        writeln!(txt, "   {s}").unwrap();
    }

    // H-C2: ratios
    struct P {
        k: u32,
        curve: String,
        fam: String,
        rho: f64,
        both_closed: bool,
    }
    let mut ps: Vec<P> = vec![];
    let mut aa: Vec<P> = vec![];
    for ((k, curve, _t), m) in &by {
        let (Some(s), Some(a), Some(b)) = (m.get("std"), m.get("tw_a"), m.get("tw_b")) else { continue };
        let fam = if curve.starts_with('B') { "B" } else { "A" };
        for t in [a, b] {
            ps.push(P { k: *k, curve: curve.clone(), fam: fam.into(), rho: t.mults / s.mults, both_closed: t.closed && s.closed });
        }
        aa.push(P { k: *k, curve: curve.clone(), fam: fam.into(), rho: a.mults / b.mults, both_closed: a.closed && b.closed });
    }
    let gm = |v: &[&P]| -> f64 { (v.iter().map(|p| p.rho.ln()).sum::<f64>() / v.len() as f64).exp() };
    let mut rng2 = Rng::new("bootstrap-cost");
    let boot = |v: &[&P], rng: &mut Rng| -> (f64, f64) {
        let mut cl: BTreeMap<(u32, String), Vec<&P>> = BTreeMap::new();
        for p in v {
            cl.entry((p.k, p.curve.clone())).or_default().push(*p);
        }
        let cells: Vec<&Vec<&P>> = cl.values().collect();
        let mut gs = vec![];
        for _ in 0..2000 {
            let mut sum = 0.0;
            let mut n = 0.0;
            for _ in 0..cells.len() {
                let c = cells[rng.below(cells.len())];
                for _ in 0..c.len() {
                    let p = c[rng.below(c.len())];
                    sum += p.rho.ln();
                    n += 1.0;
                }
            }
            gs.push((sum / n).exp());
        }
        gs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        (pct(&gs, 0.025), pct(&gs, 0.975))
    };
    let verdict = |lo: f64, hi: f64| {
        if lo >= 0.8 && hi <= 1.25 {
            "UNCHANGED"
        } else if hi < 0.8 || lo > 1.25 {
            "CHANGED"
        } else {
            "INCONCLUSIVE"
        }
    };
    writeln!(txt, "\n-- H-C2: rho = mults(tw)/mults(std) at D=26, same target; geometric mean, 95 % two-level bootstrap; tolerance [0.8, 1.25] --").unwrap();
    let mut cost_js = String::new();
    let mut headline = String::new();
    let sets: Vec<(String, Vec<&P>)> = {
        let mut v: Vec<(String, Vec<&P>)> = vec![];
        v.push(("all pairs, both closed [PRIMARY]".into(), ps.iter().filter(|p| p.both_closed).collect()));
        v.push(("all pairs (closed or not)".into(), ps.iter().collect()));
        v.push(("family A, both closed".into(), ps.iter().filter(|p| p.both_closed && p.fam == "A").collect()));
        v.push(("family B, both closed".into(), ps.iter().filter(|p| p.both_closed && p.fam == "B").collect()));
        v.push(("A/A tw_a/tw_b, both closed [NOISE REFERENCE]".into(), aa.iter().filter(|p| p.both_closed).collect()));
        v
    };
    for (name, v) in &sets {
        if v.is_empty() {
            continue;
        }
        let g = gm(v);
        let (lo, hi) = boot(v, &mut rng2);
        let sdl = sd(&v.iter().map(|p| p.rho.ln()).collect::<Vec<_>>());
        writeln!(txt, "{name:<46} n={:<4} geo-mean {g:.4}  95% [{lo:.4}, {hi:.4}]  sd(log) {sdl:.3}  => {}", v.len(), verdict(lo, hi)).unwrap();
        if name.contains("PRIMARY") {
            write!(headline, "{{\"geo_mean\":{g:.5},\"lo\":{lo:.5},\"hi\":{hi:.5},\"n\":{},\"verdict\":\"{}\"}}", v.len(), verdict(lo, hi)).unwrap();
        }
    }
    writeln!(txt, "\nper k (both closed, std vs tw), and per k A/A:").unwrap();
    writeln!(txt, "{:>3} {:<7} {:>4} {:>8} {:>8} {:>8}   {:<7} {:>4} {:>8} {:>8} {:>8}", "k", "std/tw", "n", "geo", "lo", "hi", "A/A", "n", "geo", "lo", "hi").unwrap();
    for k in [5u32, 7, 9, 11, 13, 15] {
        for fam in ["A", "B", "all"] {
            let v: Vec<&P> = ps.iter().filter(|p| p.k == k && p.both_closed && (fam == "all" || p.fam == fam)).collect();
            let w: Vec<&P> = aa.iter().filter(|p| p.k == k && p.both_closed && (fam == "all" || p.fam == fam)).collect();
            if v.is_empty() {
                continue;
            }
            let (lo, hi) = boot(&v, &mut rng2);
            let (alo, ahi) = if w.is_empty() { (f64::NAN, f64::NAN) } else { boot(&w, &mut rng2) };
            let ag = if w.is_empty() { f64::NAN } else { gm(&w) };
            writeln!(txt, "{:>3} {:<7} {:>4} {:>8.4} {:>8.4} {:>8.4}   {:<7} {:>4} {:>8.4} {:>8.4} {:>8.4}", k, fam, v.len(), gm(&v), lo, hi, fam, w.len(), ag, alo, ahi).unwrap();
            write!(cost_js, "{{\"k\":{k},\"family\":\"{fam}\",\"n\":{},\"geo\":{:.5},\"lo\":{lo:.5},\"hi\":{hi:.5},\"aa_geo\":{},\"aa_lo\":{},\"aa_hi\":{}}},", v.len(), gm(&v), if ag.is_finite() { format!("{ag:.5}") } else { "null".into() }, if alo.is_finite() { format!("{alo:.5}") } else { "null".into() }, if ahi.is_finite() { format!("{ahi:.5}") } else { "null".into() }).unwrap();
        }
    }
    // distribution of rho
    let mut rs: Vec<f64> = ps.iter().filter(|p| p.both_closed).map(|p| p.rho).collect();
    rs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if !rs.is_empty() {
        writeln!(txt, "\nrho (std->tw, both closed) quantiles: min {:.3}  5% {:.3}  25% {:.3}  50% {:.3}  75% {:.3}  95% {:.3}  max {:.3}; fraction inside [0.8,1.25]: {:.3}", rs[0], pct(&rs, 0.05), pct(&rs, 0.25), pct(&rs, 0.5), pct(&rs, 0.75), pct(&rs, 0.95), rs[rs.len() - 1], rs.iter().filter(|&&x| (0.8..=1.25).contains(&x)).count() as f64 / rs.len() as f64).unwrap();
    }
    // H-C4: mean mults by k and arm
    writeln!(txt, "\n-- H-C4: mean counted mults (closed solves) by k and arm; setup mults in F_Q products --").unwrap();
    writeln!(txt, "{:>3} {:>14} {:>14} {:>14} {:>10}", "k", "std", "tw_a", "tw_b", "setup").unwrap();
    let mut kx = vec![];
    let mut ky = vec![];
    for k in [5u32, 7, 9, 11, 13, 15] {
        let m = |arm: &str| {
            let v: Vec<f64> = cs.iter().filter(|c| c.k == k && c.arm.starts_with(arm) && c.closed).map(|c| c.mults).collect();
            if v.is_empty() { f64::NAN } else { mean(&v) }
        };
        let su: Vec<f64> = cs.iter().filter(|c| c.k == k).map(|c| c.setup).collect();
        if su.is_empty() {
            continue;
        }
        writeln!(txt, "{:>3} {:>14.4e} {:>14.4e} {:>14.4e} {:>10.0}", k, m("std"), m("tw_a"), m("tw_b"), mean(&su)).unwrap();
        kx.push(k as f64);
        ky.push(((m("std") * m("tw_a") * m("tw_b")).cbrt()).ln());
    }
    if kx.len() >= 3 {
        let (mx, my) = (mean(&kx), mean(&ky));
        let slope = kx.iter().zip(&ky).map(|(x, y)| (x - mx) * (y - my)).sum::<f64>() / kx.iter().map(|x| (x - mx) * (x - mx)).sum::<f64>();
        writeln!(txt, "slope of ln(mean mults) per unit k (pooled arms): {slope:.4}").unwrap();
    }
    // invs
    let minv = mean(&cs.iter().map(|c| c.invs).collect::<Vec<_>>());
    writeln!(txt, "mean inversions per solve: {minv:.0} (negligible against mults)").unwrap();

    // summary.json
    write!(js, "\"hY1\":{{\"tested\":{z_tested},\"pass3\":{z_pass},\"gt5\":{z_gt5},\"const_fail\":{z_const},\"z_mean\":{zm:.4},\"z_sd\":{zsd:.4},\"verdict\":\"{}\"}},", if hy1 { "PASS" } else { "FAIL" }).unwrap();
    write!(js, "\"hY1_rows\":[{}],", y1_rows.trim_end_matches(',')).unwrap();
    let y2j: Vec<String> = y2.iter().map(|x| format!("{{\"k\":{},\"curve\":\"{}\",\"arm\":\"{}\",\"obs\":{},\"lo\":{},\"hi\":{},\"pred\":{:.5},\"covers\":{},\"exact_pred\":{}}}", x.0, x.1, x.2, fj(x.3), fj(x.4), fj(x.5), x.6, x.7, x.8)).collect();
    write!(js, "\"yield_ratio\":[{}],", y2j.join(",")).unwrap();
    write!(js, "\"aa_yield\":[{}],", aa_rows.trim_end_matches(',')).unwrap();
    write!(js, "\"cost_primary\":{},", if headline.is_empty() { "null".into() } else { headline }).unwrap();
    write!(js, "\"cost_by_k\":[{}],", cost_js.trim_end_matches(',')).unwrap();
    write!(js, "\"hC1\":{{\"pairs\":{pairs},\"rank_equal\":{same_rank},\"hf_equal\":{same_hf}}},").unwrap();
    write!(js, "\"hC3\":{{\"closed_std\":{fs:.4},\"closed_tw_a\":{fa:.4},\"closed_tw_b\":{fb:.4}}}").unwrap();
    js.push('}');
    std::fs::write(format!("{dir}/summary.txt"), &txt).unwrap();
    std::fs::write(format!("{dir}/summary.json"), &js).unwrap();
    println!("{txt}");
}
