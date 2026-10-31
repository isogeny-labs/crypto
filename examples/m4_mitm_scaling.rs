//! # m = 4 two-plus-two meet-in-the-middle: scaling and validation
//!
//! A self-contained scaling study of the m = 4 decomposition given by a
//! 2 + 2 meet-in-the-middle over a random factor base:
//!
//! ```text
//! R = F_a + F_b + F_c + F_d
//! table  T = { F_i + F_j : i <= j }              (|F|(|F|+1)/2 additions)
//! query    for each pair (i, j): R - (F_i + F_j) in T ?
//! ```
//!
//! With factor-base size `B = p^{1/5}` this is the m = 4 analogue of
//! `rootfind-mitm`, which stops at m = 3, and of `pkm-semaev`, which
//! refuses m > 3 because its grid recovery is written for m <= 3.  The
//! search here is purely in the prime group and charges one addition per
//! group operation.
//!
//! Two exponents are reported, because they are different quantities:
//!
//! - **planted solve** (a target known to decompose): table build plus the
//!   pairs scanned until the planted decomposition is reached.  This is the
//!   per-target solve cost and is expected to scale like `B^2 = p^{2/5}`;
//! - **uniform relation** (a random on-curve target): a full pair scan
//!   refutes a target, and a target decomposes with probability about
//!   `B^4 / p`, so the expected cost of one relation is `p / B^2 = p^{3/5}`.
//!   The difference between the two is the solve constant
//!   (`B^2` table plus the `B^4 / p` rarity) that can eat an exponent gain.
//!
//! Every recovered decomposition is re-added in the group and compared
//! against the target; the run fails if any witness does not verify.
//!
//! **Not a security claim.** Synthetic known-answer instances only; public
//! parameters, no deployed curve, no key recovery, no sub-rho claim.
//!
//! ```bash
//! cargo run --release --example m4_mitm_scaling
//! cargo run --release --example m4_mitm_scaling -- --bits 24,28,32,36,40,44,48,52
//! M4_OUT=research/ic_m4_mitm_scaling_20261031/run.txt \
//!   cargo run --release --example m4_mitm_scaling
//! ```

use std::collections::HashMap;
use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crypto_lib::cryptanalysis::prime_fast::{FastCurve, Pt};
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;

fn largest_prime_below_pow2(bits: u32, res_mod4: u64) -> u64 {
    let mut p = (1u64 << bits) - 1;
    loop {
        if p % 4 == res_mod4 && is_prime_u64(p) {
            return p;
        }
        p -= 2;
    }
}

fn random_factor_base(curve: &FastCurve, size: usize, rng: &mut StdRng) -> Vec<Pt> {
    let p = curve.f.p;
    let mut pts: Vec<Pt> = Vec::with_capacity(size);
    while pts.len() < size {
        let x = rng.gen_range(1..p);
        if let Some(pt) = curve.lift_x(x) {
            pts.push(pt);
        }
    }
    pts
}

fn add(curve: &FastCurve, a: Pt, b: Pt) -> Option<Pt> {
    curve.add(Some(a), Some(b))
}

fn three_sum(curve: &FastCurve, a: Pt, b: Pt, c: Pt) -> Option<Pt> {
    add(curve, add(curve, a, b)?, c)
}

struct MitmTable {
    map: HashMap<(u64, u64), (u16, u16)>,
    sums: Vec<Option<Pt>>,
    adds: u64,
}

impl MitmTable {
    fn build(curve: &FastCurve, base: &[Pt]) -> Self {
        let b = base.len();
        let mut map = HashMap::new();
        let mut sums = Vec::with_capacity(b * (b + 1) / 2);
        let mut adds = 0u64;
        for i in 0..b {
            for j in i..b {
                let s = add(curve, base[i], base[j]);
                if let Some(sp) = s {
                    map.entry(curve.canonical(sp))
                        .or_insert((i as u16, j as u16));
                }
                sums.push(s);
                adds += 1;
            }
        }
        MitmTable { map, sums, adds }
    }

    fn index(&self, b: usize, i: usize, j: usize) -> usize {
        i * b - i * (i - 1) / 2 + (j - i)
    }

    /// Pairs scanned until `r` is written as a sum of four base points.
    fn query(
        &self,
        curve: &FastCurve,
        base: &[Pt],
        r: Pt,
    ) -> (u64, Option<(usize, usize, usize, usize)>) {
        let b = base.len();
        let mut q = 0u64;
        for i in 0..b {
            for j in i..b {
                let Some(s) = self.sums[self.index(b, i, j)] else {
                    continue;
                };
                q += 1;
                let Some(d) = add(curve, r, curve.neg(s)) else {
                    continue;
                };
                if let Some(&(k, l)) = self.map.get(&curve.canonical(d)) {
                    return (q, Some((i, j, k as usize, l as usize)));
                }
            }
        }
        (q, None)
    }
}

fn verify_four(curve: &FastCurve, base: &[Pt], r: Pt, idx: (usize, usize, usize, usize)) -> bool {
    let (i, j, k, l) = idx;
    three_sum(curve, base[i], base[j], base[k])
        .and_then(|s| add(curve, s, base[l]))
        .map(|s| curve.canonical(s) == curve.canonical(r))
        .unwrap_or(false)
}

/// One planted m = 4 instance against a shared table.  Returns (additions
/// charged to this target after amortising the table, verified).
fn planted_query(curve: &FastCurve, base: &[Pt], tab: &MitmTable, rng: &mut StdRng) -> (f64, bool) {
    let b = base.len();
    let mut idx = [0usize; 4];
    for k in 0..4 {
        loop {
            let c = rng.gen_range(0..b);
            if !idx[..k].contains(&c) {
                idx[k] = c;
                break;
            }
        }
    }
    let mut r = base[idx[0]];
    for &k in &idx[1..] {
        r = add(curve, r, base[k]).expect("planted sum is finite");
    }
    let (q, hit) = tab.query(curve, base, r);
    let ok = hit.map(|h| verify_four(curve, base, r, h)).unwrap_or(false);
    (q as f64, ok)
}

/// Uniform target: full pair scans, each refuting one target, until a
/// decomposition is found.  Returns (additions, verified, targets_tried).
fn uniform_relations(
    curve: &FastCurve,
    base: &[Pt],
    rng: &mut StdRng,
    relations: usize,
) -> (u64, bool, u64) {
    let b = base.len();
    let mut table: HashMap<(u64, u64), (u16, u16)> = HashMap::new();
    let mut sums: Vec<Option<Pt>> = Vec::with_capacity(b * (b + 1) / 2);
    let mut adds = 0u64;
    for i in 0..b {
        for j in i..b {
            let s = add(curve, base[i], base[j]);
            if let Some(sp) = s {
                table
                    .entry(curve.canonical(sp))
                    .or_insert((i as u16, j as u16));
            }
            sums.push(s);
            adds += 1;
        }
    }

    let p = curve.f.p;
    let mut tried = 0u64;
    let mut found = 0usize;
    let mut all_ok = true;
    while found < relations {
        // Draw a random on-curve target; lift_x returns None off-curve.
        let r = loop {
            let x = rng.gen_range(1..p);
            if let Some(pt) = curve.lift_x(x) {
                break pt;
            }
        };
        tried += 1;
        let mut hit: Option<(usize, usize, usize, usize)> = None;
        'outer: for i in 0..b {
            for j in i..b {
                let Some(s) = sums[i * b - i * (i - 1) / 2 + (j - i)] else {
                    continue;
                };
                adds += 1;
                let Some(d) = add(curve, r, curve.neg(s)) else {
                    continue;
                };
                if let Some(&(k, l)) = table.get(&curve.canonical(d)) {
                    hit = Some((i, j, k as usize, l as usize));
                    break 'outer;
                }
            }
        }
        if let Some((i, j, k, l)) = hit {
            let ok = three_sum(curve, base[i], base[j], base[k])
                .and_then(|s| add(curve, s, base[l]))
                .map(|s| curve.canonical(s) == curve.canonical(r))
                .unwrap_or(false);
            all_ok &= ok;
            found += 1;
        }
    }
    (adds, all_ok, tried)
}

struct Fit {
    slope: f64,
    r2: f64,
    points: usize,
}

fn fit(xs: &[f64], ys: &[f64]) -> Fit {
    let n = xs.len();
    let mx = xs.iter().sum::<f64>() / n as f64;
    let my = ys.iter().sum::<f64>() / n as f64;
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    for i in 0..n {
        sxx += (xs[i] - mx) * (xs[i] - mx);
        sxy += (xs[i] - mx) * (ys[i] - my);
    }
    let slope = sxy / sxx;
    let intercept = my - slope * mx;
    let mut ss_res = 0.0;
    let mut ss_tot = 0.0;
    for i in 0..n {
        let pred = intercept + slope * xs[i];
        ss_res += (ys[i] - pred) * (ys[i] - pred);
        ss_tot += (ys[i] - my) * (ys[i] - my);
    }
    Fit {
        slope,
        r2: 1.0 - ss_res / ss_tot,
        points: n,
    }
}

fn main() {
    let mut bits: Vec<u32> = vec![24, 28, 32, 36, 40, 44, 48, 52];
    let mut uniform_bits: Vec<u32> = vec![24, 28, 32, 36, 40];
    let mut planted_targets = 40usize;
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--bits" => {
                bits = args[i + 1].split(',').map(|s| s.parse().unwrap()).collect();
                i += 2;
            }
            "--uniform-bits" => {
                uniform_bits = args[i + 1].split(',').map(|s| s.parse().unwrap()).collect();
                i += 2;
            }
            "--targets" => {
                planted_targets = args[i + 1].parse().unwrap();
                i += 2;
            }
            other => panic!("unknown flag {other}"),
        }
    }

    let mut out = String::new();
    out.push_str("# m = 4 two-plus-two MITM scaling\n\n");
    println!("{out}");

    // ── planted solve ─────────────────────────────────────────────
    println!("=== planted m=4 decomposition (table + query until hit) ===");
    println!(
        "{:>4}  {:>5}  {:>7}  {:>9}  {:>12}  {:>7}",
        "bits", "|F|", "pairs", "adds/tgt", "verified", "ms"
    );
    let mut px = Vec::new();
    let mut py = Vec::new();
    for &b in &bits {
        let p = largest_prime_below_pow2(b, 3);
        let curve = FastCurve::new(&format!("m4-{b}bit"), p, p - 3, 1, 2);
        let bsz = (p as f64).powf(0.2).round().max(4.0) as usize;
        let mut rng = StdRng::seed_from_u64(0x4D34_0000 ^ b as u64);
        let base = random_factor_base(&curve, bsz, &mut rng);
        let t0 = Instant::now();
        let tab = MitmTable::build(&curve, &base);
        let mut adds = 0f64;
        let mut ok = 0usize;
        for _ in 0..planted_targets {
            let (a, good) = planted_query(&curve, &base, &tab, &mut rng);
            adds += a;
            ok += good as usize;
        }
        let ms = t0.elapsed().as_secs_f64() * 1e3;
        let per = tab.adds as f64 / planted_targets as f64 + adds / planted_targets as f64;
        println!(
            "{b:>4}  {bsz:>5}  {:>7}  {per:>12.1}  {:>4}/{:<2}  {ms:>7.1}",
            bsz * (bsz + 1) / 2,
            ok,
            planted_targets
        );
        assert_eq!(ok, planted_targets, "planted witnesses must all verify");
        px.push((p as f64).log2());
        py.push(per.log2());
    }
    let f = fit(&px, &py);
    println!(
        "\nplanted solve exponent (log2 adds per target vs log2 p) = {:.3} (R^2 {:.4}, {} sizes)",
        f.slope, f.r2, f.points
    );
    assert!(
        (0.30..0.50).contains(&f.slope),
        "planted solve exponent {:.3} outside the expected B^2 = p^0.4 band",
        f.slope
    );

    // ── uniform relation ──────────────────────────────────────────
    println!("\n=== uniform m=4 relation (full scans until a decomposition) ===");
    println!(
        "{:>4}  {:>5}  {:>12}  {:>9}  {:>7}",
        "bits", "|F|", "adds/rel", "verified", "ms"
    );
    let mut ux = Vec::new();
    let mut uy = Vec::new();
    for &b in &uniform_bits {
        let p = largest_prime_below_pow2(b, 3);
        let curve = FastCurve::new(&format!("m4u-{b}bit"), p, p - 3, 1, 2);
        let bsz = (p as f64).powf(0.2).round().max(4.0) as usize;
        let mut rng = StdRng::seed_from_u64(0x4D34_1000 ^ b as u64);
        let base = random_factor_base(&curve, bsz, &mut rng);
        let t0 = Instant::now();
        let (adds, ok, _tried) = uniform_relations(&curve, &base, &mut rng, 2);
        let ms = t0.elapsed().as_secs_f64() * 1e3;
        let per = adds as f64 / 2.0;
        println!("{b:>4}  {bsz:>5}  {per:>12.1}  {:>9}  {ms:>7.1}", ok);
        assert!(ok, "uniform witnesses must verify");
        ux.push((p as f64).log2());
        uy.push(per.log2());
    }
    let fu = fit(&ux, &uy);
    println!(
        "\nuniform relation exponent (log2 adds per relation vs log2 p) = {:.3} (R^2 {:.4}, {} sizes)",
        fu.slope, fu.r2, fu.points
    );
    println!(
        "\nexpected: planted ~ 2/5 = 0.400, uniform ~ 3/5 = 0.600; \
         the gap is the table B^2 and the B^4/p rarity"
    );

    if let Ok(path) = std::env::var("M4_OUT") {
        out.push_str(&format!(
            "planted_solve_exponent {:.4}\nplanted_r2 {:.4}\nuniform_relation_exponent {:.4}\nuniform_r2 {:.4}\n",
            f.slope, f.r2, fu.slope, fu.r2
        ));
        if let Some(dir) = std::path::Path::new(&path).parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = std::fs::write(path, out);
    }
}
