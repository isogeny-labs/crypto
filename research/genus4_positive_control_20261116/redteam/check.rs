//! Red-team recomputation for research/genus4_positive_control_20261116.
//!
//! Standalone (no crates): `rustc -O check.rs -o check && ./check ../run.jsonl`.
//! Reads only the committed run.jsonl (artifact sha256 e0f7050d...). Writes
//! nothing; prints the readings REDTEAM.md cites. Review checks, not results.

use std::collections::BTreeMap;
use std::env;
use std::fs;

#[derive(Clone, Debug, Default)]
struct Row {
    arm: String,
    p: u64,
    curve: u64,
    seed: u64,
    r: f64,
    group_ops: f64,
    rel_mul: f64,
    rel_add: f64,
    la_mul: f64,
    la_add: f64,
    native: f64,
    steps: f64,
    cantor_mul: f64,
    cantor_inv: f64,
    wall_ns: f64,
    walk_ops: f64,
    trials: f64,
    certified: bool,
    ok: bool,
    is_ic: bool,
}

fn field<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    let end = rest
        .find(|c: char| c == ',' || c == '}')
        .unwrap_or(rest.len());
    Some(rest[..end].trim().trim_matches('"'))
}
fn num(line: &str, key: &str) -> f64 {
    field(line, key).and_then(|s| s.parse().ok()).unwrap_or(0.0)
}

// splitmix64
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

/// Per-curve lists of a cost for one arm at one p.
fn by_curve(rows: &[Row], arm: &str, p: u64, f: &dyn Fn(&Row) -> f64) -> BTreeMap<u64, Vec<f64>> {
    let mut m: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    for r in rows.iter().filter(|r| r.arm == arm && r.p == p) {
        m.entry(r.curve).or_default().push(f(r));
    }
    m
}

fn pooled(m: &BTreeMap<u64, Vec<f64>>) -> f64 {
    let all: Vec<f64> = m.values().flatten().copied().collect();
    mean(&all)
}

#[derive(Clone, Copy)]
enum Boot {
    TwoStage,
    CurvesOnly,
    SeedsOnly,
}

/// Ratio of pooled means with a bootstrap; returns (est, quantiles at qs).
fn boot(
    num: &BTreeMap<u64, Vec<f64>>,
    den: &BTreeMap<u64, Vec<f64>>,
    mode: Boot,
    b: usize,
    seed: u64,
    qs: &[f64],
) -> (f64, Vec<f64>) {
    let est = pooled(num) / pooled(den);
    let curves: Vec<u64> = num.keys().copied().collect();
    let mut rng = Rng(seed);
    let mut bs = Vec::with_capacity(b);
    for _ in 0..b {
        let cs: Vec<u64> = match mode {
            Boot::SeedsOnly => curves.clone(),
            _ => (0..curves.len()).map(|_| curves[rng.below(curves.len())]).collect(),
        };
        let (mut n, mut d) = (Vec::new(), Vec::new());
        for c in cs {
            let (gn, gd) = (&num[&c], &den[&c]);
            match mode {
                Boot::CurvesOnly => {
                    n.extend_from_slice(gn);
                    d.extend_from_slice(gd);
                }
                _ => {
                    for _ in 0..gn.len() {
                        n.push(gn[rng.below(gn.len())]);
                    }
                    for _ in 0..gd.len() {
                        d.push(gd[rng.below(gd.len())]);
                    }
                }
            }
        }
        bs.push(mean(&n) / mean(&d));
    }
    bs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let qv = qs
        .iter()
        .map(|&q| bs[((q * b as f64) as usize).min(b - 1)])
        .collect();
    (est, qv)
}

const W: &str = "C_double_lp+pre+coord+unitT";
const V: &str = "C_double_lp+pre+coord";
const C: &str = "C_double_lp+pre";
const R: &str = "R_rho_neg";

fn main() {
    let path = env::args().nth(1).expect("path to run.jsonl");
    let text = fs::read_to_string(&path).expect("read");
    let mut rows = Vec::new();
    for line in text.lines() {
        if field(line, "kind") != Some("run") {
            continue;
        }
        let arm = field(line, "arm").unwrap_or("").to_string();
        rows.push(Row {
            is_ic: arm != R,
            arm,
            p: num(line, "p") as u64,
            curve: num(line, "curve_seed") as u64,
            seed: num(line, "seed") as u64,
            r: num(line, "r"),
            group_ops: num(line, "group_ops"),
            rel_mul: num(line, "rel_mul"),
            rel_add: num(line, "rel_add"),
            la_mul: num(line, "la_mul"),
            la_add: num(line, "la_add"),
            native: num(line, "native"),
            steps: num(line, "steps"),
            cantor_mul: num(line, "cantor_mul"),
            cantor_inv: num(line, "cantor_inv"),
            wall_ns: num(line, "wall_ns"),
            walk_ops: num(line, "walk_ops"),
            trials: num(line, "trials"),
            certified: field(line, "certified") == Some("true"),
            ok: field(line, "ok") == Some("true"),
        });
    }
    println!("runs parsed: {}", rows.len());
    println!(
        "certified&ok: {}/{}",
        rows.iter().filter(|r| r.certified && r.ok).count(),
        rows.len()
    );
    let qs = [0.025, 0.975, 0.9917, 0.995];
    let b = 20000;

    // Pricing readings.  `w` GAE per counted native operation.
    // P0  producer: IC gops + w(rel_mul+la_mul);      rho gops + w*native
    // P1  mul-only both arms: rho's canonicalisations hold no field
    //     multiplication (rho_canon: negation by subtraction, key compare),
    //     so rho native = 0 under a strict multiplication convention.
    // P2  IC mul+add (the genus-3 control's sensitivity, native_all);
    //     rho as emitted.
    // P3  P2 with rho's per-step walk additions a+aj, b+bj mod r
    //     (rho_canon/addmod, 2 per step; not emitted: estimate 2*steps).
    let ic = |r: &Row, w: f64, mode: u8| -> f64 {
        match mode {
            0 | 1 => r.group_ops + w * (r.rel_mul + r.la_mul),
            _ => r.group_ops + w * (r.rel_mul + r.la_mul + r.rel_add + r.la_add),
        }
    };
    let rho = |r: &Row, w: f64, mode: u8| -> f64 {
        match mode {
            0 | 2 => r.group_ops + w * r.native,
            1 => r.group_ops,
            _ => r.group_ops + w * (r.native + 2.0 * r.steps),
        }
    };
    let cost = |r: &Row, w: f64, mode: u8| if r.is_ic { ic(r, w, mode) } else { rho(r, w, mode) };

    for p in [2053u64, 4099, 8209] {
        println!("\n=== p = {p} ===");
        for (mode, name) in [
            (0u8, "P0 producer (IC mul; rho canon)"),
            (1, "P1 mul-only both arms (rho native = 0)"),
            (2, "P2 IC mul+add; rho as emitted"),
            (3, "P3 IC mul+add; rho canon + 2 addmod/step (estimate)"),
        ] {
            for arm in [C, V, W] {
                let mut line = format!("{name:52} {:6}", &arm[arm.len().min(14)..]);
                for w in [0.0, 0.1, 1.0] {
                    let n = by_curve(&rows, arm, p, &|r| cost(r, w, mode));
                    let d = by_curve(&rows, R, p, &|r| cost(r, w, mode));
                    let (e, q) = boot(&n, &d, Boot::TwoStage, b, 20261119, &qs);
                    line += &format!(" | w={w}: {e:.3} [{:.3}, {:.3}] q99.17={:.3}", q[0], q[1], q[2]);
                }
                println!("{line}");
            }
        }
    }

    // ── p = 8209 detail ──
    let p = 8209u64;
    println!("\n=== p = 8209 detail, arm w ===");
    let wr: Vec<&Row> = rows.iter().filter(|r| r.arm == W && r.p == p).collect();
    let rr: Vec<&Row> = rows.iter().filter(|r| r.arm == R && r.p == p).collect();
    let m = |v: &[&Row], f: &dyn Fn(&Row) -> f64| v.iter().map(|r| f(r)).sum::<f64>() / v.len() as f64;
    let (g, rm, rl, ra, la) = (
        m(&wr, &|r| r.group_ops),
        m(&wr, &|r| r.rel_mul),
        m(&wr, &|r| r.la_mul),
        m(&wr, &|r| r.rel_add),
        m(&wr, &|r| r.la_add),
    );
    println!("w means: group_ops {g:.4e} rel_mul {rm:.4e} la_mul {rl:.4e} rel_add {ra:.4e} la_add {la:.4e}");
    println!(
        "w trials {:.4e}  walk_ops {:.4e}  rel_mul/trial {:.1}",
        m(&wr, &|r| r.trials),
        m(&wr, &|r| r.walk_ops),
        rm / m(&wr, &|r| r.trials)
    );
    let (rg, rn, rs) = (m(&rr, &|r| r.group_ops), m(&rr, &|r| r.native), m(&rr, &|r| r.steps));
    println!("rho means: group_ops {rg:.4e} native {rn:.4e} steps {rs:.4e}");
    println!(
        "w=1 composition: IC = {:.4e} (gops {:.1}%, rel_mul {:.1}%, la_mul {:.1}%); rho = {:.4e} (gops {:.1}%, canon {:.1}%)",
        g + rm + rl,
        100.0 * g / (g + rm + rl),
        100.0 * rm / (g + rm + rl),
        100.0 * rl / (g + rm + rl),
        rg + rn,
        100.0 * rg / (rg + rn),
        100.0 * rn / (rg + rn)
    );
    println!(
        "break-even add weight x (adds priced x*w, rho as emitted, w=1): {:.3}",
        (rg + rn - g - rm - rl) / (ra + la)
    );
    println!(
        "IC / rho group ops only (P1 at w=1 point): {:.3}",
        (g + rm + rl) / rg
    );

    // rho efficiency against sqrt(pi r / 4)
    println!("\nrho efficiency (steps / sqrt(pi r/4)) and group_ops/steps:");
    let mut per_curve_exp: BTreeMap<u64, (f64, f64)> = BTreeMap::new();
    for r in &rr {
        let e = (std::f64::consts::PI * r.r / 4.0).sqrt();
        println!(
            "  curve {:3} seed {} r {:.4e} steps/E {:.3} gops/steps {:.4} native/gops {:.5}",
            r.curve,
            r.seed,
            r.r,
            r.steps / e,
            r.group_ops / r.steps,
            r.native / r.group_ops
        );
        let ent = per_curve_exp.entry(r.curve).or_insert((e, 0.0));
        ent.1 += r.group_ops / r.steps / 4.0;
    }
    let all_eff: Vec<f64> = rr
        .iter()
        .map(|r| r.steps / (std::f64::consts::PI * r.r / 4.0).sqrt())
        .collect();
    println!("  mean steps/E over 12 runs: {:.3}", mean(&all_eff));
    // Ratio against the expected rho (no rho sampling noise)
    let exp_gops: f64 =
        per_curve_exp.values().map(|(e, ov)| e * ov).sum::<f64>() / per_curve_exp.len() as f64;
    for w in [0.0, 0.1, 1.0] {
        let icm = m(&wr, &|r| r.group_ops + w * (r.rel_mul + r.la_mul));
        let icm_all = m(&wr, &|r| r.group_ops + w * (r.rel_mul + r.la_mul + r.rel_add + r.la_add));
        println!(
            "expected-rho reading w={w}: IC/E[rho] P0 {:.3}  P1 {:.3}  P2 {:.3}",
            icm / (exp_gops * (1.0 + w)),
            icm / exp_gops,
            icm_all / (exp_gops * (1.0 + w))
        );
    }

    // The cheapest registered rho at each weight (docs/ic/SUCCESS_CRITERIA.md
    // §1): plain rho (RhoWalk::plain, no canonicalisation, no look-ahead)
    // has E[steps] = sqrt(pi r / 2) and no priced native work.  Not
    // measured in this study; analytic expectation only (overhead taken 1.0).
    println!("\ncheapest-rho check (analytic plain rho, E = sqrt(pi r/2) group ops, native 0):");
    let mut sq = 0.0;
    let mut nc = 0.0;
    for (e, _) in per_curve_exp.values() {
        sq += e * std::f64::consts::SQRT_2; // sqrt(pi r/4)*sqrt(2) = sqrt(pi r/2)
        nc += 1.0;
    }
    let plain = sq / nc;
    for w in [0.0, 0.1, 1.0] {
        let neg_meas = m(&rr, &|r| r.group_ops + w * r.native);
        let icm = m(&wr, &|r| r.group_ops + w * (r.rel_mul + r.la_mul));
        let cheapest = neg_meas.min(plain);
        println!(
            "  w={w}: negation rho (measured) {neg_meas:.4e}  plain rho (expected) {plain:.4e}  cheapest {}  IC/cheapest {:.3}",
            if plain < neg_meas { "plain" } else { "negation" },
            icm / cheapest
        );
    }
    println!(
        "  w at which plain = negation (measured constants): {:.3}",
        plain / m(&rr, &|r| r.group_ops) - 1.0
    );
    // Pricing variants at w = 1 against both rho walks (estimates where the
    // rho counter is not emitted: partition hash = 1 per step, walk addmods =
    // 2 per step; plain rho has no canonicalisation).
    {
        let icm = m(&wr, &|r| r.group_ops + r.rel_mul + r.la_mul);
        let icm_all = m(&wr, &|r| r.group_ops + r.rel_mul + r.la_mul + r.rel_add + r.la_add);
        let (ng, nn, ns) = (rg, rn, rs);
        let ps = plain; // plain steps ~ group ops
        for (name, ic_v, neg_v, plain_v) in [
            ("producer (IC mul; rho canon)", icm, ng + nn, ps),
            ("+ rho partition hash/step", icm, ng + nn + ns, 2.0 * ps),
            ("IC mul+add; rho canon, no rho adds", icm_all, ng + nn, ps),
            ("IC mul+add; rho canon+hash+2 addmod/step", icm_all, ng + nn + 3.0 * ns, 4.0 * ps),
            ("mul only both (rho canon 0)", icm, ng, ps),
        ] {
            println!(
                "  w=1 {name:42} IC/neg {:.3}  IC/plain {:.3}  IC/cheapest {:.3}",
                ic_v / neg_v,
                ic_v / plain_v,
                ic_v / neg_v.min(plain_v)
            );
        }
    }
    // n plain-rho runs needed for a 97.5% upper bound below 1 if the mean
    // lands on expectation (Rayleigh CV 0.523, normal approx., curve term ignored)
    let pt = m(&wr, &|r| r.group_ops + (r.rel_mul + r.la_mul)) / plain;
    let need = (1.96 * 0.523 / (1.0 - pt)).powi(2);
    println!("  point {pt:.3}; runs needed for upper bound < 1 (approx.): {need:.1}");
    // LA, multiplications only against multiplications + additions
    let la = |arm: &str, f: &dyn Fn(&Row) -> f64| {
        let v: Vec<&Row> = rows.iter().filter(|r| r.arm == arm && r.p == p).collect();
        m(&v, f)
    };
    let (cm, vm, wm) = (la(C, &|r| r.la_mul), la(V, &|r| r.la_mul), la(W, &|r| r.la_mul));
    let (ca, va, wa) = (
        la(C, &|r| r.la_mul + r.la_add),
        la(V, &|r| r.la_mul + r.la_add),
        la(W, &|r| r.la_mul + r.la_add),
    );
    println!(
        "\nLA ratios p=8209: mul-only v/c {:.3} w/c {:.3} w/v {:.3}; mul+add v/c {:.3} w/c {:.3} w/v {:.3}",
        vm / cm,
        wm / cm,
        wm / vm,
        va / ca,
        wa / ca,
        wa / va
    );
    let r8209: Vec<f64> = per_curve_exp.values().map(|(e, _)| (e * e * 4.0 / std::f64::consts::PI)).collect();
    println!(
        "unit-T restart cap max(64, sqrt(r)/16) at p=8209: {:?} (binds only below 4096)",
        r8209.iter().map(|r| (r.sqrt() / 16.0).max(64.0) as u64).collect::<Vec<_>>()
    );

    // per-curve and leave-one-curve-out, P0 at w = 1
    println!("\nper-curve ratio (P0, w=1) and leave-one-curve-out:");
    let n = by_curve(&rows, W, p, &|r| cost(r, 1.0, 0));
    let d = by_curve(&rows, R, p, &|r| cost(r, 1.0, 0));
    for c in n.keys() {
        println!("  curve {c}: IC {:.4e} rho {:.4e} ratio {:.3}", mean(&n[c]), mean(&d[c]), mean(&n[c]) / mean(&d[c]));
    }
    for drop in n.keys() {
        let n2: BTreeMap<u64, Vec<f64>> = n.iter().filter(|(k, _)| *k != drop).map(|(k, v)| (*k, v.clone())).collect();
        let d2: BTreeMap<u64, Vec<f64>> = d.iter().filter(|(k, _)| *k != drop).map(|(k, v)| (*k, v.clone())).collect();
        let (e, q) = boot(&n2, &d2, Boot::TwoStage, b, 20261119, &qs);
        println!("  drop {drop}: {e:.3} [{:.3}, {:.3}] q99.5={:.3}", q[0], q[1], q[3]);
    }
    // resampling variants and bootstrap-seed spread
    for (mode, name) in [(Boot::TwoStage, "two-stage"), (Boot::CurvesOnly, "curves-only"), (Boot::SeedsOnly, "seeds-only")] {
        let mut ups = Vec::new();
        for s in 1..=10u64 {
            let (_, q) = boot(&n, &d, mode, 2000, s, &qs);
            ups.push(q[1]);
        }
        let (e, q) = boot(&n, &d, mode, b, 7, &qs);
        println!(
            "  {name:12} est {e:.3} [{:.3}, {:.3}] q99.17 {:.3} q99.5 {:.3}; upper97.5 over 10 seeds (B=2000): {:.3}..{:.3}",
            q[0],
            q[1],
            q[2],
            q[3],
            ups.iter().cloned().fold(f64::INFINITY, f64::min),
            ups.iter().cloned().fold(0.0, f64::max)
        );
    }
    // mean of per-run paired ratios (alternative estimator)
    let mut pr = Vec::new();
    for x in &wr {
        if let Some(y) = rr.iter().find(|y| y.curve == x.curve && y.seed == x.seed) {
            pr.push(cost(x, 1.0, 0) / cost(y, 1.0, 0));
        }
    }
    pr.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!(
        "  paired per-run ratios (w=1): mean {:.3} median {:.3} min {:.3} max {:.3}; runs above 1: {}/{}",
        mean(&pr),
        (pr[5] + pr[6]) / 2.0,
        pr[0],
        pr[pr.len() - 1],
        pr.iter().filter(|&&x| x > 1.0).count(),
        pr.len()
    );
    // pseudo A/A: rho seeds {1,2} against rho seeds {3,4}, same estimator
    for pp in [2053u64, 4099, 8209] {
        let a = by_curve(&rows.iter().filter(|r| r.seed <= 2).cloned().collect::<Vec<_>>(), R, pp, &|r| r.group_ops);
        let bb = by_curve(&rows.iter().filter(|r| r.seed > 2).cloned().collect::<Vec<_>>(), R, pp, &|r| r.group_ops);
        let (e, q) = boot(&a, &bb, Boot::TwoStage, b, 11, &qs);
        println!("  pseudo-A/A rho s1-2 / s3-4 at p={pp}: {e:.3} [{:.3}, {:.3}]", q[0], q[1]);
    }
    // all 10 curve multisets with replacement, seed means
    let keys: Vec<u64> = n.keys().copied().collect();
    let mut worst: f64 = 0.0;
    for i in 0..3 {
        for j in i..3 {
            for k in j..3 {
                let s = [keys[i], keys[j], keys[k]];
                let nn: f64 = s.iter().map(|c| mean(&n[c])).sum();
                let dd: f64 = s.iter().map(|c| mean(&d[c])).sum();
                worst = worst.max(nn / dd);
            }
        }
    }
    println!("  worst curve multiset (seed means), w=1: {worst:.3}");

    // FPM and wall time (practicality only)
    let fpm = |r: &Row| {
        if r.is_ic {
            r.cantor_mul + 10.0 * r.cantor_inv + r.rel_mul + 16.0 * r.la_mul
        } else {
            r.cantor_mul + 10.0 * r.cantor_inv
        }
    };
    let (e, q) = boot(&by_curve(&rows, W, p, &fpm), &by_curve(&rows, R, p, &fpm), Boot::TwoStage, b, 3, &qs);
    println!("\nFPM w/R: {e:.4} [{:.4}, {:.4}]", q[0], q[1]);
    let fpm_all = |r: &Row| fpm(r) + if r.is_ic { r.rel_add + r.la_add } else { 0.0 };
    let (e, q) = boot(&by_curve(&rows, W, p, &fpm_all), &by_curve(&rows, R, p, &fpm), Boot::TwoStage, b, 3, &qs);
    println!("FPM with IC additions at 1 field mult each: {e:.4} [{:.4}, {:.4}]", q[0], q[1]);
    println!(
        "wall time mean: w {:.3} s, rho {:.1} s, ratio {:.4} (shared host, practicality note only)",
        m(&wr, &|r| r.wall_ns) / 1e9,
        m(&rr, &|r| r.wall_ns) / 1e9,
        m(&wr, &|r| r.wall_ns) / m(&rr, &|r| r.wall_ns)
    );
    println!(
        "Cantor field mults per group op: IC {:.1}, rho {:.1}",
        m(&wr, &|r| r.cantor_mul) / g,
        m(&rr, &|r| r.cantor_mul) / rg
    );
    // calibrated readings using the genus-3 control's measured weights at
    // p = 8209 (w_rel 0.0047, w_la 0.0130 per genus-3 Cantor add); a genus-4
    // Cantor step is ~1.8x the genus-3 one in field mults, so these
    // over-price IC here.  Informational only (different curve, no pin).
    for k in [1.0, 10.0] {
        let icm = m(&wr, &|r| r.group_ops + k * (0.0047 * r.rel_mul + 0.0130 * r.la_mul));
        let rhm = m(&rr, &|r| r.group_ops);
        println!("borrowed-calibration x{k}: IC/rho {:.4} (rho canon priced 0)", icm / rhm);
    }
}
