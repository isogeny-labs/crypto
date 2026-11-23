//! A plain XL pass at one degree on dense bit rows, no closure.
//!
//! The question it answers for a committed draw: is the constant `1` in the
//! row span of {equation × monomial : degree of the product <= D}, with
//! x_i^2 = x_i?  That is the XL refutation test, nothing else: no MutantXL
//! closure, no fall products.  The rows are the same ones
//! `koblitz_groebner::visit_macaulay_rows` builds (an equation of degree `p`
//! times every monomial of degree <= D - p, equal terms cancelling in pairs).
//!
//! ```sh
//! # one JSON line per draw on stdout
//! RAYON_NUM_THREADS=2 cargo run --release --example xl_dense_degree -- run \
//!     --degree 5 --unsat --role registered research/dreg_closure_scale_20261016/draws/m3-19-7.jsonl
//! # sizes only, nothing is eliminated
//! ... xl_dense_degree -- run --degree 5 --count-only --draw 0 <file>
//! # score recorded run lines (see `score` below)
//! ... xl_dense_degree -- score research/dreg_xl_degree5_20261018/runs/*.jsonl
//! ```
//!
//! Engine.  Columns are the monomials that occur in some row, in descending
//! (degree, mask) order with the constant monomial last.  Rows are inserted
//! one at a time into an echelon basis whose pivot is the lowest set bit; a
//! pivot stores only the words from its leading word on.  Because the
//! constant column is last, `1` is in the span exactly when some row reduces
//! to the constant alone, and the run stops then (`refuted`).  Rows are
//! reduced in batches: the batch is first reduced against the frozen pivot
//! set in parallel (at most `--threads` threads), then its survivors are
//! reduced against each other and inserted in order.  Rank does not depend
//! on the order, and a refutation found is a refutation of the full matrix.
//!
//! Without early stop the engine runs the whole matrix and reports its rank.
//! A run that would need more than `--max-bytes` of dense rows is not started
//! and prints `"unmeasured"`.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::pq_groebner_f2::F2BoolPoly;
use num_bigint::BigUint;
use rayon::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Instant;

/// All monomials (variable-set masks) over `n` variables of degree <= `d`,
/// in no particular order.
fn monomials_up_to(n: usize, d: u32) -> Vec<u64> {
    fn rec(start: usize, n: usize, left: u32, cur: u64, out: &mut Vec<u64>) {
        out.push(cur);
        if left == 0 {
            return;
        }
        for v in start..n {
            rec(v + 1, n, left - 1, cur | (1u64 << v), out);
        }
    }
    let mut out = Vec::new();
    rec(0, n, d, 0, &mut out);
    out
}

/// `C(n, <= d)`.
fn binom_up_to(n: u64, d: u32) -> u64 {
    let (mut total, mut b) = (0u128, 1u128);
    for i in 0..=d as u64 {
        if i > n {
            break;
        }
        total += b;
        b = b * (n - i) as u128 / (i + 1) as u128;
    }
    total as u64
}

/// The sparse Macaulay rows of `polys` at `degree`: each row the sorted
/// monomials with odd multiplicity.  Empty products are dropped.
fn macaulay_sparse_rows(polys: &[F2BoolPoly], n_vars: usize, degree: u32) -> Vec<Vec<u64>> {
    let mut by_gap: Vec<Option<Vec<u64>>> = vec![None; degree as usize + 1];
    let mut rows = Vec::new();
    let mut all: Vec<u64> = Vec::new();
    for p in polys {
        let pdeg = p
            .terms
            .iter()
            .map(|t| t.mask.count_ones())
            .max()
            .unwrap_or(0);
        if pdeg > degree {
            continue;
        }
        let gap = (degree - pdeg) as usize;
        let mults = by_gap[gap].get_or_insert_with(|| monomials_up_to(n_vars, gap as u32));
        for &mult in mults.iter() {
            all.clear();
            all.extend(p.terms.iter().map(|t| t.mask | mult));
            all.sort_unstable();
            let mut kept = 0;
            let mut i = 0;
            while i < all.len() {
                let mut j = i;
                while j < all.len() && all[j] == all[i] {
                    j += 1;
                }
                if (j - i) % 2 == 1 {
                    all[kept] = all[i];
                    kept += 1;
                }
                i = j;
            }
            all.truncate(kept);
            if !all.is_empty() {
                rows.push(all.clone());
            }
        }
    }
    rows
}

#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
#[cfg(target_arch = "x86_64")]
unsafe fn xor_avx2(dst: &mut [u64], src: &[u64]) {
    for (d, s) in dst.iter_mut().zip(src) {
        *d ^= *s;
    }
}

/// `dst ^= src` over equal lengths.
#[inline]
fn xor_into(dst: &mut [u64], src: &[u64]) {
    debug_assert_eq!(dst.len(), src.len());
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            // SAFETY: the avx2 feature was just detected at run time.
            unsafe { xor_avx2(dst, src) };
            return;
        }
    }
    for (d, s) in dst.iter_mut().zip(src) {
        *d ^= *s;
    }
}

struct Pivot {
    /// Word index of the leading bit; `words` covers `start..W`.
    start: usize,
    words: Box<[u64]>,
}

struct Echelon {
    w: usize,
    /// Leading column -> index into `pivots`, or `u32::MAX`.
    by_lead: Vec<u32>,
    pivots: Vec<Pivot>,
    bytes: u64,
}

impl Echelon {
    fn new(cols: usize) -> Self {
        Echelon {
            w: cols.div_ceil(64),
            by_lead: vec![u32::MAX; cols],
            pivots: Vec::new(),
            bytes: 0,
        }
    }

    /// Reduce `row` (full width) against the pivots; return its leading
    /// column if it does not reduce to zero.
    fn reduce(&self, row: &mut [u64]) -> Option<usize> {
        let mut wi = 0;
        while wi < self.w {
            let word = row[wi];
            if word == 0 {
                wi += 1;
                continue;
            }
            let lead = wi * 64 + word.trailing_zeros() as usize;
            let pi = self.by_lead[lead];
            if pi == u32::MAX {
                return Some(lead);
            }
            let p = &self.pivots[pi as usize];
            xor_into(&mut row[p.start..], &p.words);
        }
        None
    }

    fn insert(&mut self, row: &[u64], lead: usize) {
        let start = lead / 64;
        let words: Box<[u64]> = row[start..].into();
        self.bytes += (words.len() * 8) as u64;
        self.by_lead[lead] = self.pivots.len() as u32;
        self.pivots.push(Pivot { start, words });
    }
}

#[derive(Debug, Clone, Default)]
pub struct XlOutcome {
    pub rows: usize,
    pub cols: usize,
    /// Rows removed by singleton-column peeling (each adds exactly 1 to the rank).
    pub peeled: usize,
    pub rows_dense: usize,
    pub cols_dense: usize,
    pub rank: usize,
    pub rows_done: usize,
    pub refuted: bool,
    pub pivot_bytes: u64,
    /// rows_dense x words x 8: the dense matrix before any trimming.
    pub dense_bytes: u64,
    /// Why the run stopped without a verdict, if it did.
    pub stopped: Option<&'static str>,
}

/// The distinct monomials of `rows`, in descending (degree, mask) order, so
/// the constant monomial (mask 0) is last.
fn column_order(rows: &[Vec<u64>]) -> Vec<u64> {
    let mut seen: HashMap<u64, ()> = HashMap::new();
    for r in rows {
        for &m in r {
            seen.insert(m, ());
        }
    }
    let mut cols: Vec<u64> = seen.into_keys().collect();
    cols.sort_unstable_by_key(|b| std::cmp::Reverse((b.count_ones(), *b)));
    cols
}

/// Sizes of the Macaulay matrix at `degree` (rows, columns) and of what is
/// left of it after peeling, without eliminating anything.
fn xl_sizes(polys: &[F2BoolPoly], n_vars: usize, degree: u32, peel: bool) -> XlOutcome {
    let srows = macaulay_sparse_rows(polys, n_vars, degree);
    let cols = column_order(&srows);
    let (irows, ncols) = to_index_rows(srows, &cols);
    let (live, peeled) = if peel {
        peel_singletons(&irows, ncols, cols.iter().position(|&m| m == 0))
    } else {
        ((0..irows.len()).collect(), 0)
    };
    let mut used = std::collections::BTreeSet::new();
    for &r in &live {
        used.extend(irows[r].iter().copied());
    }
    XlOutcome {
        rows: irows.len(),
        cols: ncols,
        peeled,
        rows_dense: live.len(),
        cols_dense: used.len(),
        dense_bytes: (live.len() as u64) * (used.len().div_ceil(64) as u64) * 8,
        ..Default::default()
    }
}

/// Sparse rows as ascending column indices.
fn to_index_rows(srows: Vec<Vec<u64>>, cols: &[u64]) -> (Vec<Vec<u32>>, usize) {
    let index: HashMap<u64, u32> = cols
        .iter()
        .enumerate()
        .map(|(i, &m)| (m, i as u32))
        .collect();
    let rows = srows
        .into_iter()
        .map(|r| {
            let mut v: Vec<u32> = r.iter().map(|m| index[m]).collect();
            v.sort_unstable();
            v
        })
        .collect();
    (rows, cols.len())
}

/// Repeatedly remove a row that holds a column no other live row holds.
/// Such a row cannot be part of any combination that equals `1` (its
/// private column would stay set), and it adds exactly 1 to the rank, so
/// dropping it leaves both the verdict and `rank - peeled` unchanged.  The
/// constant column is never used to peel.  Returns the live row indices
/// (ascending) and the number peeled.
fn peel_singletons(
    irows: &[Vec<u32>],
    ncols: usize,
    const_col: Option<usize>,
) -> (Vec<usize>, usize) {
    let mut count = vec![0u32; ncols];
    for r in irows {
        for &c in r {
            count[c as usize] += 1;
        }
    }
    // Column -> rows, as CSR.
    let mut start = vec![0usize; ncols + 1];
    for c in 0..ncols {
        start[c + 1] = start[c] + count[c] as usize;
    }
    let mut fill = start.clone();
    let mut adj = vec![0u32; start[ncols]];
    for (ri, r) in irows.iter().enumerate() {
        for &c in r {
            adj[fill[c as usize]] = ri as u32;
            fill[c as usize] += 1;
        }
    }
    let mut dead = vec![false; irows.len()];
    let mut stack: Vec<u32> = (0..ncols as u32)
        .filter(|&c| count[c as usize] == 1 && Some(c as usize) != const_col)
        .collect();
    let mut peeled = 0usize;
    while let Some(c) = stack.pop() {
        if count[c as usize] != 1 {
            continue;
        }
        let Some(&ri) = adj[start[c as usize]..start[c as usize + 1]]
            .iter()
            .find(|&&ri| !dead[ri as usize])
        else {
            continue;
        };
        dead[ri as usize] = true;
        peeled += 1;
        for &cc in &irows[ri as usize] {
            count[cc as usize] -= 1;
            if count[cc as usize] == 1 && Some(cc as usize) != const_col {
                stack.push(cc);
            }
        }
    }
    let live = (0..irows.len()).filter(|&r| !dead[r]).collect();
    (live, peeled)
}

/// The XL test at `degree`.  `peel` removes rows with a private column
/// first ([`peel_singletons`]).  The run stops without a verdict
/// (`stopped`) once the stored pivots pass `max_bytes`.
#[allow(clippy::too_many_arguments)]
fn xl_degree(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    early_stop: bool,
    peel: bool,
    threads: usize,
    max_bytes: u64,
    verbose: bool,
) -> XlOutcome {
    let srows = macaulay_sparse_rows(polys, n_vars, degree);
    let cols = column_order(&srows);
    let const_col = cols.iter().position(|&m| m == 0);
    let (irows_all, ncols) = to_index_rows(srows, &cols);
    drop(cols);
    let (live, peeled) = if peel {
        peel_singletons(&irows_all, ncols, const_col)
    } else {
        ((0..irows_all.len()).collect(), 0)
    };
    // Compact the columns the live rows still use, keeping their order.
    let mut used = vec![false; ncols];
    for &r in &live {
        for &c in &irows_all[r] {
            used[c as usize] = true;
        }
    }
    let mut remap = vec![u32::MAX; ncols];
    let mut nd = 0u32;
    for c in 0..ncols {
        if used[c] {
            remap[c] = nd;
            nd += 1;
        }
    }
    let cols_dense = nd as usize;
    let const_dense = const_col.and_then(|c| (remap[c] != u32::MAX).then(|| remap[c] as usize));
    let mut irows: Vec<Vec<u32>> = live
        .iter()
        .map(|&r| irows_all[r].iter().map(|&c| remap[c as usize]).collect())
        .collect();
    drop(irows_all);
    // Insert rows in ascending order of their leading column.
    irows.sort_by_key(|r| r[0]);
    let w = cols_dense.div_ceil(64);
    let mut out = XlOutcome {
        rows: irows.len() + peeled,
        cols: ncols,
        peeled,
        rows_dense: irows.len(),
        cols_dense,
        dense_bytes: (irows.len() as u64) * (w as u64) * 8,
        ..Default::default()
    };
    let mut ech = Echelon::new(cols_dense);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads.max(1))
        .build()
        .expect("thread pool");
    let started = Instant::now();
    let batch = 64usize;
    let mut next_log = 2000usize;
    let densify = |r: &Vec<u32>| -> Vec<u64> {
        let mut d = vec![0u64; w];
        for &c in r {
            d[c as usize / 64] ^= 1u64 << (c % 64);
        }
        d
    };
    let mut i = 0;
    'outer: while i < irows.len() {
        let end = (i + batch).min(irows.len());
        // Phase 1, parallel against the frozen pivot set.
        let reduced: Vec<(Vec<u64>, Option<usize>)> = pool.install(|| {
            irows[i..end]
                .par_iter()
                .map(|r| {
                    let mut d = densify(r);
                    let lead = ech.reduce(&mut d);
                    (d, lead)
                })
                .collect()
        });
        // Phase 2, in order: reduce against the pivots added since, insert.
        for (mut d, lead) in reduced {
            out.rows_done += 1;
            if lead.is_none() {
                continue;
            }
            let Some(lead) = ech.reduce(&mut d) else {
                continue;
            };
            if Some(lead) == const_dense {
                out.refuted = true;
                if early_stop {
                    out.rank = peeled + ech.pivots.len() + 1;
                    break 'outer;
                }
            }
            ech.insert(&d, lead);
        }
        i = end;
        if ech.bytes > max_bytes {
            out.stopped = Some("pivot memory cap reached");
            break;
        }
        if verbose && out.rows_done >= next_log {
            eprintln!(
                "  rows {}/{} rank {} pivot_bytes {} secs {:.0}",
                out.rows_done,
                irows.len(),
                ech.pivots.len(),
                ech.bytes,
                started.elapsed().as_secs_f64()
            );
            next_log += 2000;
        }
    }
    if out.stopped.is_none() && !(out.refuted && early_stop) {
        out.rank = peeled + ech.pivots.len();
    }
    out.pivot_bytes = ech.bytes;
    out
}

fn vm_hwm_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1)?.parse().ok())
        })
        .unwrap_or(0)
}

fn build_system(v: &Value) -> (Vec<F2BoolPoly>, usize) {
    let m = v.get("m").and_then(Value::as_u64).unwrap_or(3) as usize;
    let n = v["n"].as_u64().expect("n") as u32;
    let irr = find_irreducible_sparse(n).expect("field");
    let st = FieldStructure::new(n, &irr);
    let b = F2mElement::one(n);
    let basis: Vec<F2mElement> = v["v_basis"]
        .as_array()
        .expect("v_basis")
        .iter()
        .map(|w| F2mElement::from_biguint(&BigUint::from(w.as_u64().unwrap()), n))
        .collect();
    let x_r = F2mElement::from_biguint(&BigUint::from(v["x_r"].as_u64().unwrap()), n);
    let sys = build_decomposition_system(&basis, &x_r, &b, m, &st).expect("system");
    assert_eq!(
        sys.n_vars as u64,
        v["n_vars"].as_u64().unwrap(),
        "rebuilt shape"
    );
    (sys.equations, sys.n_vars)
}

fn run(args: &[String]) {
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let has = |name: &str| args.iter().any(|a| a == name);
    let degree: u32 = flag("--degree")
        .expect("--degree D")
        .parse()
        .expect("degree");
    let threads: usize = flag("--threads").map_or(2, |v| v.parse().expect("threads"));
    let max_bytes: u64 = flag("--max-bytes").map_or(6_500_000_000, |v| v.parse().expect("bytes"));
    let role = flag("--role").unwrap_or_else(|| "unlabelled".into());
    let draws: Option<Vec<u64>> =
        flag("--draw").map(|s| s.split(',').map(|x| x.parse().expect("draw")).collect());
    let want_sat = has("--sat");
    let want_unsat = has("--unsat");
    let count_only = has("--count-only");
    let no_early = has("--no-early-stop");
    let no_peel = has("--no-peel");
    let valued = [
        "--degree",
        "--threads",
        "--max-bytes",
        "--role",
        "--draw",
        "--expect",
    ];
    let files: Vec<&String> = args
        .iter()
        .enumerate()
        .filter(|(i, a)| {
            !a.starts_with("--") && !(*i > 0 && valued.contains(&args[i - 1].as_str()))
        })
        .map(|(_, a)| a)
        .collect();
    for file in files {
        let text = std::fs::read_to_string(file).unwrap_or_else(|e| panic!("{file}: {e}"));
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).expect("json");
            let sat = v["outcome"]["kind"] == "satisfiable";
            if let Some(ds) = &draws {
                if !ds.contains(&v["draw"].as_u64().unwrap()) {
                    continue;
                }
            } else if (sat && !want_sat) || (!sat && !want_unsat) {
                continue;
            }
            let (eqs, n_vars) = build_system(&v);
            let started = Instant::now();
            let o = if count_only {
                xl_sizes(&eqs, n_vars, degree, !no_peel)
            } else {
                xl_degree(
                    &eqs, n_vars, degree, !no_early, !no_peel, threads, max_bytes, true,
                )
            };
            let secs = started.elapsed().as_secs_f64();
            let status = if count_only {
                "count_only"
            } else if o.refuted || o.stopped.is_none() {
                "measured"
            } else {
                "unmeasured"
            };
            let refuted = if status == "measured" {
                o.refuted.to_string()
            } else {
                "null".into()
            };
            let expect = match flag("--expect").as_deref() {
                Some("refuted") => "true",
                Some("open") => "false",
                _ => "null",
            };
            println!(
                r#"{{"role":"{role}","status":"{status}","stopped":{},"cell":{},"n":{},"ell":{},"m":{},"draw":{},"solutions":{},"committed_outcome":{},"n_vars":{n_vars},"degree":{degree},"rows":{},"cols":{},"cols_full":{},"peeled":{},"rows_dense":{},"cols_dense":{},"rows_done":{},"rank":{},"rank_exact":{},"refuted":{refuted},"expect_refuted":{expect},"early_stop":{},"peel":{},"secs":{:.3},"pivot_bytes":{},"dense_bytes":{},"vmhwm_kb":{}}}"#,
                o.stopped.map_or("null".to_string(), |s| format!("\"{s}\"")),
                v["cell"],
                v["n"],
                v["ell"],
                v.get("m").and_then(Value::as_u64).unwrap_or(3),
                v["draw"],
                v["solutions"],
                v["outcome"],
                o.rows,
                o.cols,
                binom_up_to(n_vars as u64, degree),
                o.peeled,
                o.rows_dense,
                o.cols_dense,
                o.rows_done,
                o.rank,
                no_early && o.stopped.is_none() && !count_only,
                !no_early,
                !no_peel,
                secs,
                o.pivot_bytes,
                o.dense_bytes,
                vm_hwm_kb()
            );
        }
    }
}

/// The scorer: reads run lines, prints one verdict per draw and per cell.
///
/// Per draw (degree 5, role `registered`): `refuted at 5` when the line is
/// measured and refuted; `not refuted at 5 (XL degree >= 6, closure
/// undecided)` when measured and not; `unmeasured` otherwise.  Per cell: the
/// count of each over its registered draws.  A control line that disagrees
/// with its known outcome prints `CONTROL FAILURE`.
fn score(args: &[String]) {
    let mut cells: std::collections::BTreeMap<String, [u32; 3]> = Default::default();
    let mut control_failures = 0;
    for file in args {
        let text = std::fs::read_to_string(file).unwrap_or_else(|e| panic!("{file}: {e}"));
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).expect("json");
            let role = v["role"].as_str().unwrap_or("");
            let status = v["status"].as_str().unwrap_or("");
            let d = v["degree"].as_u64().unwrap_or(0);
            let draw = v["draw"].as_u64().unwrap_or(0);
            let cell = v["cell"].as_str().unwrap_or("?").to_string();
            let sat = v["solutions"].as_u64().unwrap_or(0) > 0;
            let verdict = match (status, v["refuted"].as_bool()) {
                ("measured", Some(true)) => "refuted",
                ("measured", Some(false)) => "not_refuted",
                _ => "unmeasured",
            };
            if role == "registered" {
                let e = cells.entry(format!("{cell} degree {d}")).or_default();
                match verdict {
                    "refuted" => e[0] += 1,
                    "not_refuted" => e[1] += 1,
                    _ => e[2] += 1,
                }
                let reading = match verdict {
                    "refuted" => {
                        format!("refuted at {d}: closure degree is exactly {d} (committed > 4)")
                    }
                    "not_refuted" => format!(
                        "not refuted at {d}: XL degree >= {}, closure undecided",
                        d + 1
                    ),
                    _ => "unmeasured".into(),
                };
                println!("{cell} draw {draw} degree {d}: {reading}");
            } else {
                // Controls: a satisfiable draw must never refute; the
                // positive-control line carries its expected verdict.
                let expect = v["expect_refuted"].as_bool();
                let bad = (sat && verdict == "refuted")
                    || matches!(
                        (expect, verdict),
                        (Some(true), "not_refuted") | (Some(false), "refuted")
                    );
                if bad {
                    control_failures += 1;
                }
                println!(
                    "control {role} {cell} draw {draw} degree {d}: {verdict}{}",
                    if bad { "  CONTROL FAILURE" } else { "" }
                );
            }
        }
    }
    for (k, e) in &cells {
        println!(
            "cell {k}: refuted {} not_refuted {} unmeasured {}",
            e[0], e[1], e[2]
        );
    }
    println!("control failures: {control_failures}");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("run") => run(&args[1..]),
        Some("score") => score(&args[1..]),
        _ => {
            eprintln!("usage: xl_dense_degree run --degree D [--unsat|--sat|--draw K,..] [--role R] [--count-only] [--no-early-stop] FILES | score LINES");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::koblitz_bench::closure_refutation_scan;
    use crypto_lib::cryptanalysis::pq_groebner_f2::F2BoolMono;

    fn xorshift(s: &mut u64) -> u64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }

    /// A random Boolean system of `k` polynomials of degree <= `deg` on `n`
    /// variables; with `plant`, every polynomial vanishes at a hidden point.
    fn random_system(n: usize, k: usize, deg: u32, plant: bool, seed: u64) -> Vec<F2BoolPoly> {
        let mut s = seed | 1;
        let point = xorshift(&mut s) & ((1u64 << n) - 1);
        (0..k)
            .map(|_| {
                let mut masks: Vec<u64> = Vec::new();
                for _ in 0..(3 + xorshift(&mut s) % 6) {
                    let mut m = 0u64;
                    for _ in 0..(1 + xorshift(&mut s) % deg as u64) {
                        m |= 1u64 << (xorshift(&mut s) % n as u64);
                    }
                    masks.push(m);
                }
                if plant {
                    let val = masks
                        .iter()
                        .fold(0u64, |a, m| a ^ ((m & !point == 0) as u64));
                    if val == 1 {
                        masks.push(0);
                    }
                } else if xorshift(&mut s) % 2 == 1 {
                    masks.push(0);
                }
                masks.sort_unstable();
                let mut kept: Vec<u64> = Vec::new();
                for m in masks {
                    if kept.last() == Some(&m) {
                        kept.pop();
                    } else {
                        kept.push(m);
                    }
                }
                F2BoolPoly {
                    terms: kept.into_iter().map(|mask| F2BoolMono { mask }).collect(),
                    n_vars: n,
                }
            })
            .filter(|p| !p.terms.is_empty())
            .collect()
    }

    fn agrees(eqs: &[F2BoolPoly], n_vars: usize, degrees: std::ops::RangeInclusive<u32>) -> usize {
        let mut refuted_seen = 0;
        for d in degrees {
            let scan = closure_refutation_scan(eqs, n_vars, d, d);
            let Some(reference) = scan.first() else {
                continue;
            };
            for peel in [false, true] {
                let mine = xl_degree(eqs, n_vars, d, false, peel, 2, u64::MAX, false);
                assert_eq!(mine.refuted, reference.xl_refuted, "refuted at degree {d}");
                // The reference reports rank 1 once its basis collapses to `1`.
                if !reference.xl_refuted {
                    assert_eq!(mine.rank, reference.xl_rank, "rank at {d}, peel {peel}");
                }
                let early = xl_degree(eqs, n_vars, d, true, peel, 1, u64::MAX, false);
                assert_eq!(early.refuted, reference.xl_refuted, "early stop at {d}");
            }
            refuted_seen += reference.xl_refuted as usize;
        }
        refuted_seen
    }

    #[test]
    fn engine_agrees_with_the_existing_xl_instrument_on_random_systems() {
        let mut refuted = 0;
        let mut open = 0;
        for seed in 1..=12u64 {
            for (n, k) in [(6usize, 8usize), (8, 14), (9, 12), (10, 16)] {
                let eqs = random_system(n, k, 2, seed % 2 == 0, seed * 7919 + n as u64);
                let r = agrees(&eqs, n, 2..=4);
                refuted += r;
                open += 3 - r;
            }
        }
        // The comparison saw both outcomes, not a constant.
        assert!(refuted > 0 && open > 0, "refuted {refuted} open {open}");
    }

    #[test]
    fn a_planted_solution_is_never_refuted() {
        for seed in 1..=10u64 {
            let eqs = random_system(9, 20, 2, true, seed * 104729);
            let o = xl_degree(&eqs, 9, 5, true, true, 2, u64::MAX, false);
            assert!(!o.refuted, "seed {seed}");
        }
    }

    #[test]
    fn engine_agrees_on_a_committed_chained_draw() {
        // research/dreg_ell_grid_20260925/runs/cell-10-2-7.jsonl, draw 0:
        // m = 3, n = 10, l = 2, N = 16, committed XL degree 5.
        let v: Value =
            serde_json::from_str(r#"{"n":10,"ell":2,"v_basis":[111,959],"x_r":61,"n_vars":16}"#)
                .unwrap();
        let (eqs, n_vars) = build_system(&v);
        assert_eq!(n_vars, 16);
        let d4 = xl_degree(&eqs, n_vars, 4, true, true, 2, u64::MAX, false);
        let d5 = xl_degree(&eqs, n_vars, 5, true, true, 2, u64::MAX, false);
        assert!(!d4.refuted, "committed XL degree is 5");
        assert!(d5.refuted, "committed XL degree is 5");
        let scan = closure_refutation_scan(&eqs, n_vars, 4, 5);
        assert_eq!(scan[0].xl_refuted, d4.refuted);
        assert_eq!(scan[1].xl_refuted, d5.refuted);
    }
}
