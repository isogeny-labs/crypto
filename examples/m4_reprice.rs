//! Re-pricing of the ECC2K-130 Weil-descent table at a measured oracle
//! exponent `w` and a measured word-op to rho-step conversion
//! (`research/m4_reprice_20261024/README.md`).
//!
//! ```sh
//! cargo run --release --example m4_reprice > research/m4_reprice_20261024/table.md
//! ```
//!
//! Derived, not measured.  The model is section 2 of
//! `research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md` (the same
//! as `examples/descent_degree_cost.rs`), with only the inputs changed:
//!
//! ```text
//!     targets        l + max(log2(m!) + n - m*l, 0)  = log2 max(2^l, m! 2^(n-(m-1)l))
//!     oracle         w * log2 C(N, <=D)  per target, N = (m-2) n + m l   [column-touch units]
//!     linear algebra log2(m) + 2 l                   [never converted]
//!     total          log2( 2^(targets + oracle) + 2^algebra )
//! ```
//!
//! Unit B ("rho steps") adds `log2(Ir per word-op / Ir per rho step)` to the
//! oracle only, from `research/k_conversion_20261018/raw.jsonl` (`ir`,
//! `word_ops`, rho `ir / walk_steps` at n = 83).  `K` as "Ir per column" is
//! NOT used: it already contains the super-linear elimination cost, so
//! stacking it on `cols^w` would count that cost twice.
//!
//! `w` is fitted here as the OLS slope of `log2 word_ops` on `log2 columns`
//! over the closure rows of `raw.jsonl`.  The target-count floor pins `l`
//! (34 at m = 4, D = 5).  The positive controls assert the existing rows
//! before anything else prints.

use std::fs;

const N_FIELD: usize = 131;
const RHO: f64 = 60.81;

fn log2_add(a: f64, b: f64) -> f64 {
    a.max(b) + (1.0 + (-(a - b).abs()).exp2()).log2()
}

/// log2 C(N, <= d), same recurrence as `descent_degree_cost`.
fn log2_monomials(n_vars: usize, d: u32) -> f64 {
    let (mut total, mut binom) = (0f64, 1f64);
    for i in 0..=d as usize {
        if i > n_vars {
            break;
        }
        total += binom;
        binom = binom * (n_vars - i) as f64 / (i + 1) as f64;
    }
    total.log2()
}

fn log2_factorial(m: usize) -> f64 {
    (1..=m).map(|k| (k as f64).log2()).sum()
}

struct Cost {
    total: f64,
    oracle: f64,
    unknowns: usize,
}

/// `conv` is added (log2) to the oracle only.
fn cost(m: usize, l: usize, degree: u32, w: f64, conv: f64) -> Cost {
    let unknowns = (m - 2) * N_FIELD + m * l;
    let per_relation = (log2_factorial(m) + N_FIELD as f64 - (m * l) as f64).max(0.0);
    let targets = l as f64 + per_relation;
    let oracle = if degree == 0 { 0.0 } else { w * log2_monomials(unknowns, degree) + conv };
    let algebra = (m as f64).log2() + (2 * l) as f64;
    Cost { total: log2_add(targets + oracle, algebra), oracle, unknowns }
}

fn best(m: usize, deg: &dyn Fn(usize) -> u32, w: f64, conv: f64) -> (usize, Cost) {
    let mut b: Option<(f64, usize)> = None;
    for l in 4..70 {
        let t = cost(m, l, deg(l), w, conv).total;
        if b.is_none_or(|(x, _)| t < x) {
            b = Some((t, l));
        }
    }
    let l = b.expect("a row").1;
    (l, cost(m, l, deg(l), w, conv))
}

fn control(name: &str, m: usize, deg: &dyn Fn(usize) -> u32, w: f64, want_l: usize, want: f64) {
    let (l, c) = best(m, deg, w, 0.0);
    let ok = l == want_l && (c.total - want).abs() < 0.005;
    println!("control {name}: l={l} total=2^{:.2} expected l={want_l} 2^{want:.2} {}", c.total, if ok { "OK" } else { "FAIL" });
    assert!(ok, "positive control failed: {name}");
}

/// Extract a numeric field `"key": value` from one JSON line.
fn num(line: &str, key: &str) -> Option<f64> {
    let pat = format!("\"{key}\": ");
    let rest = &line[line.find(&pat)? + pat.len()..];
    let end = rest.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-')).unwrap_or(rest.len());
    rest[..end].parse().ok()
}

struct Fit {
    n: usize,
    slope: f64,
    se: f64,
    intercept: f64,
    t95: f64,
}

/// OLS of y on x, with the two-sided 95% t value for n - 2 degrees of freedom.
fn ols(xs: &[f64], ys: &[f64]) -> Fit {
    let n = xs.len() as f64;
    let (mx, my) = (xs.iter().sum::<f64>() / n, ys.iter().sum::<f64>() / n);
    let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let slope = sxy / sxx;
    let intercept = my - slope * mx;
    let rss: f64 = xs.iter().zip(ys).map(|(x, y)| (y - intercept - slope * x).powi(2)).sum();
    let t95 = match xs.len() - 2 {
        19 => 2.093,
        21 => 2.080,
        d => panic!("no t value for {d} degrees of freedom"),
    };
    Fit { n: xs.len(), slope, se: (rss / (n - 2.0) / sxx).sqrt(), intercept, t95 }
}

struct Measured {
    pooled: Fit,
    deg3: Fit,
    ratio_deg3: (f64, f64),
    ratio_deg4: (f64, f64),
    ir_per_op: (f64, f64, f64), // min, median, max
    ir_per_rho_step: f64,
    cols_range: (f64, f64),
}

fn measure(path: &str) -> Measured {
    let text = fs::read_to_string(path).expect("raw.jsonl");
    let (mut xs, mut ys, mut ratios) = (vec![], vec![], vec![]);
    let (mut x3, mut y3) = (vec![], vec![]);
    let (mut r3, mut r4) = (vec![], vec![]);
    let (mut rho_ir, mut rho_steps) = (0f64, 0f64);
    for line in text.lines() {
        if line.contains("\"kind\": \"closure\"") {
            let (c, w, ir, d) = (num(line, "columns").unwrap(), num(line, "word_ops").unwrap(), num(line, "ir").unwrap(), num(line, "degree").unwrap());
            xs.push(c.log2());
            ys.push(w.log2());
            ratios.push(ir / w);
            if d == 3.0 {
                x3.push(c.log2());
                y3.push(w.log2());
                r3.push(w.log2() / c.log2());
            } else {
                r4.push(w.log2() / c.log2());
            }
        } else if line.contains("\"kind\": \"rho\"") && line.contains("\"cell\": \"n83\"") {
            rho_ir += num(line, "ir").unwrap();
            rho_steps += num(line, "walk_steps").unwrap();
        }
    }
    let span = |v: &[f64]| (v.iter().cloned().fold(f64::MAX, f64::min), v.iter().cloned().fold(f64::MIN, f64::max));
    ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Measured {
        pooled: ols(&xs, &ys),
        deg3: ols(&x3, &y3),
        ratio_deg3: span(&r3),
        ratio_deg4: span(&r4),
        ir_per_op: (ratios[0], ratios[ratios.len() / 2], *ratios.last().unwrap()),
        ir_per_rho_step: rho_ir / rho_steps,
        cols_range: span(&xs),
    }
}

fn main() {
    println!("Positive controls (original model, no conversion):\n");
    println!("```");
    control("m=3 free", 3, &|_| 0, 1.0, 33, 68.58);
    control("m=4 free", 4, &|_| 0, 1.0, 27, 56.46);
    control("m=3 D=2 w=1", 3, &|_| 2, 1.0, 37, 76.12);
    control("m=3 D=6 w=1", 3, &|_| 6, 1.0, 43, 88.05);
    control("m=3 D=6 w=2", 3, &|_| 6, 2.0, 45, 122.58);
    control("m=3 D=7 w=1", 3, &|_| 7, 1.0, 44, 90.53);
    control("m=3 D=7 w=2", 3, &|_| 7, 2.0, 45, 133.02);
    control("m=4 D=2 w=1", 4, &|_| 2, 1.0, 30, 62.88);
    control("m=4 D=2 w=2", 4, &|_| 2, 2.0, 33, 69.64);
    control("m=4 D=6 w=1", 4, &|_| 6, 1.0, 34, 76.31);
    control("m=4 D=6 w=2", 4, &|_| 6, 2.0, 34, 118.59);
    control("m=4 D=7 w=1", 4, &|_| 7, 1.0, 34, 82.11);
    control("m=4 D=7 w=2", 4, &|_| 7, 2.0, 34, 130.21);
    control("m=4 D=5 w=1 (sec 7)", 4, &|_| 5, 1.0, 34, 71.13);
    control("m=4 D=5 w=2 (sec 7/8)", 4, &|_| 5, 2.0, 34, 106.52);
    control("m=3 fit w=1", 3, &|l| (l as u32).div_ceil(2) + 4, 1.0, 4, 158.96);
    control("m=3 fit w=2", 3, &|l| (l as u32).div_ceil(2) + 4, 2.0, 4, 192.34);
    control("m=4 fit w=1", 4, &|l| l as u32 + 4, 1.0, 4, 173.13);
    control("m=4 fit w=2", 4, &|l| l as u32 + 4, 2.0, 4, 222.68);
    println!("```\n");

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/research/k_conversion_20261018/raw.jsonl");
    let me = measure(path);
    let r4 = me.ratio_deg4;
    let conv = |ir_op: f64| (ir_op / me.ir_per_rho_step).log2();
    let conv_mid = conv(me.ir_per_op.1);
    println!("Measured inputs (`research/k_conversion_20261018/raw.jsonl`, {} closure rows, columns 2^{:.1} to 2^{:.1}):\n", me.pooled.n, me.cols_range.0, me.cols_range.1);
    println!("- Degree-3 rows only ({} rows): slope w = {:.2}, 95% interval [{:.2}, {:.2}], prefactor 2^{:.1}; columns span under 0.5 bits of log2, so the interval says nothing about w at 2^36 columns.", me.deg3.n, me.deg3.slope, me.deg3.slope - me.deg3.t95 * me.deg3.se, me.deg3.slope + me.deg3.t95 * me.deg3.se, me.deg3.intercept);
    println!("- All rows pooled ({} rows): slope w = {:.2}, 95% interval [{:.2}, {:.2}]. CONFOUNDED: the 17902-column cell is degree 4 (denser), the others degree 3, so this slope mixes size with degree. Rows within a cell are replicates, so every SE is understated.", me.pooled.n, me.pooled.slope, me.pooled.slope - me.pooled.t95 * me.pooled.se, me.pooled.slope + me.pooled.t95 * me.pooled.se);
    println!("- log2 word_ops / log2 columns: degree 3 {:.2} to {:.2}; degree 4 {:.2} to {:.2} (the driver reports 2.18 to 2.33 at degree 4). The ratio rises with size and degree, so w is not a constant.", me.ratio_deg3.0, me.ratio_deg3.1, r4.0, r4.1);
    println!("- Ir per word-op: min {:.2}, median {:.2}, max {:.2}; Ir per n=83 rho step {:.1}.", me.ir_per_op.0, me.ir_per_op.1, me.ir_per_op.2, me.ir_per_rho_step);
    println!("- Conversion word-ops to rho steps (added to the oracle only): median {:+.2} bits, range [{:+.2}, {:+.2}].", conv_mid, conv(me.ir_per_op.0), conv(me.ir_per_op.2));
    println!("- The tables use word_ops = cols^w (the note's convention, unit prefactor); the unit prefactor is exact at the degree-4 cell when w is its ratio (2.19), and overstates the measured count at degree 3 (ratio 1.83 to 1.95).\n");

    println!("Rho reference 2^{RHO}. A = column-touch units (the note's convention). B = rho-step units (A's oracle + conversion median). `l` minimised over 4..69; the LA term is never converted.\n");
    println!("| m | degree D | w | l | N | A: oracle | A: total | A: vs rho | B: total | B: vs rho |");
    println!("|--:|---|--:|--:|--:|--:|--:|--:|--:|--:|");
    let ws = [1.0, 2.0, r4.0, 2.25, r4.1.max(2.33), me.pooled.slope];
    for m in [3usize, 4] {
        let fit_name = if m == 3 { "ceil(l/2)+4 (m=3 fit)" } else { "l+4 (m=4 fit)" };
        let fit: Box<dyn Fn(usize) -> u32> =
            if m == 3 { Box::new(|l| (l as u32).div_ceil(2) + 4) } else { Box::new(|l| l as u32 + 4) };
        let mut rows: Vec<(String, Box<dyn Fn(usize) -> u32>)> = Vec::new();
        for d in [2u32, 4, 5, 6, 7] {
            rows.push((format!("{d}, constant"), Box::new(move |_| d)));
        }
        rows.push((fit_name.to_string(), fit));
        let (l, c) = best(m, &|_| 0, 1.0, 0.0);
        println!("| {m} | free oracle (floor) | - | {l} | {} | - | 2^{:.2} | {:+.2} | 2^{:.2} | {:+.2} |", c.unknowns, c.total, c.total - RHO, c.total, c.total - RHO);
        for (name, deg) in &rows {
            for &w in &ws {
                let (l, a) = best(m, deg.as_ref(), w, 0.0);
                let (lb, b) = best(m, deg.as_ref(), w, conv_mid);
                let lcell = if l == lb { format!("{l}") } else { format!("{l} / {lb}") };
                println!("| {m} | {name} | {w:.2} | {lcell} | {} | 2^{:.2} | 2^{:.2} | {:+.2} | 2^{:.2} | {:+.2} |", a.unknowns, a.oracle, a.total, a.total - RHO, b.total, b.total - RHO);
            }
        }
    }
}
