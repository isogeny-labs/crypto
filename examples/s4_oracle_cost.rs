//! Decomposition-oracle cost at `m = 3`: the **chained** `S₃` system
//! ([`build_decomposition_system`], `3ℓ + n` unknowns, `2n` equations)
//! against the **eliminated** `S₄` system ([`build_eliminated_s4_system`]
//! with a target, `3ℓ` unknowns, `n` equations), on the same factor-base
//! subspace and the same target `x_R`, every answer verified.
//! Study: `research/s4_oracle_cost_20261018/` (PREREGISTRATION.md).
//!
//! ```sh
//! cargo run --release --example s4_oracle_cost -- --cell 16:5 --draw 1 \
//!   --arm chained|eliminated --seed 20261018 --budget-secs 600 [--rep 1]
//! ```
//!
//! One JSON line per invocation (one cell, draw, arm, repetition). The
//! instance (basis, `x_R`, planted summands) is a pure function of
//! `(seed, n, ℓ, draw)`, so both arms and every repetition see the same one.
//! Even draws take a uniformly random `x_R`; odd draws plant a satisfiable
//! target: random `x₁, x₂, x₃ ∈ V`, `u ∈ S3LinkRoots(x₁, x₂)`,
//! `x_R ∈ S3LinkRoots(u, x₃)`, retried until both links have a root.
//!
//! Independent truth, computed before F4 runs and never from F4's output:
//! `E` = the exhaustive set of `(x₁, x₂, x₃) ∈ V³` with `S₄(x₁,x₂,x₃,x_R) = 0`
//! (the resultant, by `eval_s4_general`'s quartic), and `E_rat ⊆ E` = those
//! with a common root `u ∈ F_{2ⁿ}` of the two `S₃` links (by `S3LinkRoots`).
//! The chained system is exactly `E_rat`; the eliminated one is exactly `E`.
//!
//! The solutions an arm reports are read off the Gröbner basis F4 returned
//! (depth-first search with constant propagation, projected to the `3ℓ`
//! summand coordinates), each is checked against `binary_semaev_s4` over
//! `F2mElement`, and the set is compared with `E` / `E_rat`. The search has a
//! node cap; an incomplete extraction is reported as unverified, never as a
//! match. A timed-out or oversize F4 run is unmeasured.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::binary_semaev::binary_semaev_s4;
use crypto_lib::cryptanalysis::koblitz_bench::{random_subspace_basis, S3LinkRoots};
use crypto_lib::cryptanalysis::koblitz_groebner::{
    build_decomposition_system, build_eliminated_s4_system, FieldStructure,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::pq_f4_f2::groebner_basis_f4;
use crypto_lib::cryptanalysis::pq_groebner_f2::F2BoolPoly;
use crypto_lib::cryptanalysis::semaev_decomp::{quartic_in_x3_general, Gf2};
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::{BTreeSet, HashMap};
use std::time::{Duration, Instant};

const NODE_CAP: u64 = 20_000_000;

/// Reads the polynomials as constraints and searches for assignments of the
/// first `k` variables that extend to a full zero of every polynomial.
struct Extractor {
    n_vars: usize,
    k: usize,
    nodes: u64,
    complete: bool,
    found: BTreeSet<u64>,
}

impl Extractor {
    /// Fix every variable a unit polynomial (`v` or `v + 1`) forces, repeating
    /// until nothing changes. `None` on a contradiction (a polynomial `1`).
    fn propagate(polys: Vec<F2BoolPoly>, assign: &mut [Option<bool>]) -> Option<Vec<F2BoolPoly>> {
        let mut polys = polys;
        loop {
            let mut forced: Option<(u32, bool)> = None;
            let mut next: Vec<F2BoolPoly> = Vec::with_capacity(polys.len());
            for p in polys {
                if p.is_zero() {
                    continue;
                }
                if p.terms.len() == 1 && p.terms[0].mask == 0 {
                    return None;
                }
                if forced.is_none() {
                    if p.terms.len() == 1 && p.terms[0].degree() == 1 {
                        forced = Some((p.terms[0].mask.trailing_zeros(), false));
                    } else if p.terms.len() == 2 && p.terms[1].mask == 0 && p.terms[0].degree() == 1
                    {
                        forced = Some((p.terms[0].mask.trailing_zeros(), true));
                    }
                }
                next.push(p);
            }
            polys = next;
            match forced {
                None => return Some(polys),
                Some((v, val)) => {
                    assign[v as usize] = Some(val);
                    polys = polys.iter().map(|p| p.substitute(v, val)).collect();
                }
            }
        }
    }

    fn pick(polys: &[F2BoolPoly], assign: &[Option<bool>], lo: usize, hi: usize) -> Option<u32> {
        let mut count: HashMap<u32, u64> = HashMap::new();
        for p in polys {
            let d = p.terms.iter().map(|m| m.degree()).max().unwrap_or(0) as u64;
            for t in &p.terms {
                for v in 0..64u32 {
                    if (t.mask >> v) & 1 == 1 && (v as usize) >= lo && (v as usize) < hi {
                        *count.entry(v).or_default() += 1000 / (d + 1);
                    }
                }
            }
        }
        count
            .into_iter()
            .filter(|(v, _)| assign[*v as usize].is_none())
            .max_by_key(|&(v, c)| (c, std::cmp::Reverse(v)))
            .map(|(v, _)| v)
    }

    /// Does some completion of the remaining variables zero every polynomial?
    fn exists(&mut self, polys: Vec<F2BoolPoly>, assign: &mut Vec<Option<bool>>) -> bool {
        self.nodes += 1;
        if self.nodes > NODE_CAP {
            self.complete = false;
            return false;
        }
        let saved = assign.clone();
        let Some(polys) = Self::propagate(polys, assign) else {
            *assign = saved;
            return false;
        };
        if polys.is_empty() {
            return true;
        }
        let Some(v) = Self::pick(&polys, assign, 0, self.n_vars) else {
            *assign = saved;
            return false;
        };
        for val in [false, true] {
            let mut a = assign.clone();
            a[v as usize] = Some(val);
            let sub: Vec<F2BoolPoly> = polys.iter().map(|p| p.substitute(v, val)).collect();
            if self.exists(sub, &mut a) {
                return true;
            }
            if !self.complete {
                break;
            }
        }
        *assign = saved;
        false
    }

    fn search(&mut self, polys: Vec<F2BoolPoly>, mut assign: Vec<Option<bool>>) {
        self.nodes += 1;
        if self.nodes > NODE_CAP {
            self.complete = false;
            return;
        }
        let Some(polys) = Self::propagate(polys, &mut assign) else {
            return;
        };
        match Self::pick(&polys, &assign, 0, self.k) {
            Some(v) => {
                for val in [false, true] {
                    let mut a = assign.clone();
                    a[v as usize] = Some(val);
                    let sub: Vec<F2BoolPoly> = polys.iter().map(|p| p.substitute(v, val)).collect();
                    self.search(sub, a);
                    if !self.complete {
                        return;
                    }
                }
            }
            None => {
                // Every projected variable that still occurs is fixed. The
                // unfixed ones left are free in the projection (no polynomial
                // mentions them) or constrained only through other variables.
                let free: Vec<usize> = (0..self.k).filter(|&i| assign[i].is_none()).collect();
                if free.len() > 22 {
                    self.complete = false;
                    return;
                }
                let base: u64 = (0..self.k)
                    .filter(|&i| assign[i] == Some(true))
                    .fold(0, |a, i| a | (1 << i));
                for bits in 0..(1u64 << free.len()) {
                    let mut pt = base;
                    let mut a = assign.clone();
                    for (j, &i) in free.iter().enumerate() {
                        let b = (bits >> j) & 1 == 1;
                        a[i] = Some(b);
                        if b {
                            pt |= 1 << i;
                        }
                    }
                    if self.found.contains(&pt) {
                        continue;
                    }
                    let sub = polys
                        .iter()
                        .map(|p| {
                            let mut q = p.clone();
                            for &i in &free {
                                q = q.substitute(i as u32, a[i] == Some(true));
                            }
                            q
                        })
                        .collect();
                    if self.exists(sub, &mut a) {
                        self.found.insert(pt);
                    }
                    if !self.complete {
                        return;
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flag = |name: &str| -> Option<String> {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let (n, ell): (u32, usize) = {
        let c = flag("--cell").expect("--cell n:ell");
        let (a, b) = c.split_once(':').expect("n:ell");
        (a.parse().expect("n"), b.parse().expect("ell"))
    };
    let arm = flag("--arm").expect("--arm chained|eliminated");
    assert!(arm == "chained" || arm == "eliminated", "--arm");
    let draw: u64 = flag("--draw").map_or(0, |v| v.parse().expect("--draw"));
    let rep: u64 = flag("--rep").map_or(1, |v| v.parse().expect("--rep"));
    let seed: u64 = flag("--seed").map_or(1, |v| v.parse().expect("--seed"));
    let budget: u64 = flag("--budget-secs").map_or(600, |v| v.parse().expect("--budget-secs"));

    let irr = find_irreducible_sparse(n).expect("field");
    let st = FieldStructure::new(n, &irr);
    let gf = Gf2::new(&irr);
    let b = F2mElement::one(n);
    let word = |e: &F2mElement| e.raw_bits().first().copied().unwrap_or(0);
    let elem = |w: u64| F2mElement::from_biguint(&BigUint::from(w), n);
    let mask = (1u64 << n) - 1;

    // ── The instance: a pure function of (seed, n, ell, draw). ─────────
    let mut rng = StdRng::seed_from_u64(
        seed ^ ((n as u64) << 40) ^ ((ell as u64) << 32) ^ draw.wrapping_mul(0x9E37_79B9),
    );
    let basis = random_subspace_basis(n, ell, &mut rng);
    let bw: Vec<u64> = basis.iter().map(word).collect();
    let span: Vec<u64> = (0..1u64 << ell)
        .map(|c| {
            (0..ell)
                .filter(|t| (c >> t) & 1 == 1)
                .fold(0, |a, t| a ^ bw[t])
        })
        .collect();
    let planted = draw % 2 == 1;
    let solver = S3LinkRoots::new(&irr, 1);
    let mut plant: Option<[u64; 3]> = None;
    let x_r = if planted {
        let mut found = None;
        for _ in 0..100_000 {
            let pick = |rng: &mut StdRng| span[rng.gen_range(0..span.len())];
            let (x1, x2, x3) = (pick(&mut rng), pick(&mut rng), pick(&mut rng));
            let us = solver.roots(x1, x2);
            if us.is_empty() {
                continue;
            }
            let u = us[rng.gen_range(0..us.len())];
            let rs = solver.roots(u, x3);
            if rs.is_empty() {
                continue;
            }
            found = Some((x1, x2, x3, rs[rng.gen_range(0..rs.len())]));
            break;
        }
        let (x1, x2, x3, xr) = found.expect("planted target");
        plant = Some([x1, x2, x3]);
        xr
    } else {
        rng.gen::<u64>() & mask
    };
    let xr_el = elem(x_r);

    // ── Independent truth: E (resultant zeros) and E_rat (rational u). ─
    let t = Instant::now();
    let mut e_set: BTreeSet<[u64; 3]> = BTreeSet::new();
    let mut e_rat: BTreeSet<[u64; 3]> = BTreeSet::new();
    let mut closing: HashMap<u64, Vec<u64>> = HashMap::new();
    for &x3 in &span {
        for u in solver.roots(x3, x_r) {
            closing.entry(u).or_default().push(x3);
        }
    }
    for &x1 in &span {
        for &x2 in &span {
            let q = quartic_in_x3_general(x1, x2, x_r, 1, &gf);
            for &x3 in &span {
                let mut v = 0u64;
                for i in (0..q.c.len()).rev() {
                    v = gf.mul(v, x3) ^ q.c[i];
                }
                if v == 0 {
                    e_set.insert([x1, x2, x3]);
                }
            }
            for u in solver.roots(x1, x2) {
                if let Some(x3s) = closing.get(&u) {
                    for &x3 in x3s {
                        e_rat.insert([x1, x2, x3]);
                    }
                }
            }
        }
    }
    let truth_secs = t.elapsed().as_secs_f64();
    let planted_in_e_rat = plant.is_none_or(|p| e_rat.contains(&p));
    let e_rat_subset_e = e_rat.is_subset(&e_set);

    // ── Build the arm's system. ────────────────────────────────────────
    let t = Instant::now();
    let sys = if arm == "chained" {
        build_decomposition_system(&basis, &xr_el, &b, 3, &st)
    } else {
        build_eliminated_s4_system(&basis, &b, Some(&xr_el), [0; 4], &st)
    };
    let sys = sys.expect("unknowns within MAX_VARS");
    let build_secs = t.elapsed().as_secs_f64();
    let n_eqs = sys.equations.len();
    let n_vars = sys.n_vars;
    let input_terms: usize = sys.equations.iter().map(|p| p.terms.len()).sum();
    let deg = |p: &F2BoolPoly| p.terms.iter().map(|m| m.degree()).max().unwrap_or(0);
    let top_degree = sys.equations.iter().map(deg).max().unwrap_or(0);
    let loadavg = std::fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|s| s.split_whitespace().next().map(str::to_owned))
        .unwrap_or_else(|| "null".into());

    let (gb, s) = groebner_basis_f4(
        sys.equations.clone(),
        n_vars,
        Some(Duration::from_secs(budget)),
    );
    let load_after = std::fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|s| s.split_whitespace().next().map(str::to_owned))
        .unwrap_or_else(|| "null".into());
    let finished = !s.timed_out && !s.oversize;

    // ── Read the solutions off the basis and verify them. ──────────────
    let (mut verdict, mut sols, mut nodes, mut ext_complete, mut bad) = (
        "unmeasured",
        BTreeSet::<[u64; 3]>::new(),
        0u64,
        false,
        0usize,
    );
    let mut ext_secs = 0.0;
    if finished {
        let t = Instant::now();
        let k = 3 * ell;
        let mut ex = Extractor {
            n_vars,
            k,
            nodes: 0,
            complete: true,
            found: BTreeSet::new(),
        };
        ex.search(gb.clone(), vec![None; n_vars]);
        ext_secs = t.elapsed().as_secs_f64();
        nodes = ex.nodes;
        ext_complete = ex.complete;
        let coord = |pt: u64, i: usize| -> u64 {
            (0..ell)
                .filter(|t| (pt >> (i * ell + t)) & 1 == 1)
                .fold(0, |a, t| a ^ bw[t])
        };
        for &pt in &ex.found {
            sols.insert([coord(pt, 0), coord(pt, 1), coord(pt, 2)]);
        }
        // Each reported solution must satisfy binary_semaev_s4 over F2mElement.
        for s3 in &sols {
            let v = binary_semaev_s4(&elem(s3[0]), &elem(s3[1]), &elem(s3[2]), &xr_el, &b, &irr);
            if !v.is_zero() {
                bad += 1;
            }
        }
        let want = if arm == "chained" { &e_rat } else { &e_set };
        verdict = if !ext_complete {
            "extraction_incomplete"
        } else if bad > 0 {
            "wrong_solution"
        } else if &sols == want {
            "verified"
        } else {
            "set_mismatch"
        };
    }

    let (rows, cols) = (s.matrix_rows_max, s.matrix_cols_max);
    // Counters outside `word_xors`: work the arm does that the unit omits.
    let unpriced = s.divisor_tests + s.pair_candidate_visits + s.pair_cover_lookups;
    println!(
        r#"{{"arm":"{arm}","n":{n},"ell":{ell},"draw":{draw},"rep":{rep},"seed":{seed},"planted":{planted},"x_r":{x_r},"v_basis":{bw:?},"n_vars":{n_vars},"n_eqs":{n_eqs},"input_terms":{input_terms},"top_degree":{top_degree},"timed_out":{},"oversize":{},"finished":{finished},"solving_degree":{},"degree_reached":{},"basis_len":{},"steps":{},"word_xors":{},"word_xors_performed":{},"divisor_tests":{},"pair_candidate_visits":{},"pair_cover_lookups":{},"unpriced_native":{unpriced},"matrix_rows_max":{rows},"matrix_cols_max":{cols},"matrix_rows_sum":{},"peak_matrix_bytes":{},"reducer_rows":{},"f4_secs":{:.3},"build_ns":{},"eliminate_ns":{},"system_build_secs":{build_secs:.3},"truth_secs":{truth_secs:.3},"extract_secs":{ext_secs:.3},"extract_nodes":{nodes},"extract_complete":{ext_complete},"n_solutions":{},"e_size":{},"e_rat_size":{},"e_rat_subset_e":{e_rat_subset_e},"planted_in_e_rat":{planted_in_e_rat},"semaev_s4_failures":{bad},"verdict":"{verdict}","loadavg_before":{loadavg},"loadavg_after":{load_after}}}"#,
        s.timed_out,
        s.oversize,
        s.solving_degree,
        s.degree_reached,
        s.basis_len,
        s.steps,
        s.word_xors,
        s.word_xors_performed,
        s.divisor_tests,
        s.pair_candidate_visits,
        s.pair_cover_lookups,
        s.matrix_rows_sum,
        s.peak_matrix_bytes,
        s.reducer_rows,
        s.wall_ns as f64 / 1e9,
        s.build_ns,
        s.eliminate_ns,
        sols.len(),
        e_set.len(),
        e_rat.len(),
    );
}
