//! Boolean-ring F4 against the native-XOR CDCL solver on two Riemann–Roch (Nagao)
//! point-decomposition systems over GF(2^m), on identical random draws.
//!
//! Curve: the Koblitz curve `E : y² + xy = x³ + 1` (`a = 0`, `b = 1`) over GF(2^m) in the
//! library's polynomial basis (`find_irreducible_sparse(m)`, the modulus `KoblitzCurve::new`
//! uses).  This is the Rust port of the equation-level reference
//! `research/rr_directions_20261115/harness/{rrsys,run_panel}.py` (`buildSystem`,
//! `decodeSolution`, `randomSubspace`, `drawInstance`, `countTruth`, `work`); the Python is
//! not called.
//!
//! Systems (`ℓ = dim V`; the first `3ℓ` bits are `u_{i,k}` with `x_i = Σ_k u_{i,k} v_k`):
//!
//! * `incidence` (Boolean degree 2): then `y_0, y_1, y_2, a, b, w_01, w_02, w_12`, `m` bits
//!   each (`3ℓ + 8m` bits).  Curve `y_i² + x_i y_i + x_i³ + 1 = 0`; incidence
//!   `x_i² + a x_i + c + b y_i = 0` with `c = x_R² + a x_R + b (x_R + y_R)`; witnesses
//!   `(x_i + x_j) w_ij + 1 = 0`.
//! * `norm` (Boolean degree 3): then `a, b` (`3ℓ + 2m` bits).  The elementary symmetric
//!   functions of `{x_0, x_1, x_2, x_R}` (`u = x_0 + x_1`, `v = x_0 x_1`, `w = x_2 + x_R`,
//!   `z = x_2 x_R`; `e1 = u + w`, `e2 = v + z + u w`, `e3 = u z + v w`, `e4 = v z`) equal
//!   `b + b²`, `a² + a b`, `b c`, `c² + b²`.
//!
//! Zero polynomials are dropped.  A system with more bits than `pq_groebner_f2::MASK_BITS`
//! (64, or 128 with `--features wide-vars`) is not built; its lines say `skipped`.
//!
//! Draws, per cell `(m, ℓ)`, from `StdRng::seed_from_u64(seed ^ (m << 40) ^ (ℓ << 32))`
//! (the derivation `rr_degree_ladder` uses, without its `a` term).  Draws `0 .. planted-1`
//! are planted, the rest unplanted.  Every attempt draws a fresh basis with
//! `koblitz_bench::random_subspace_basis`; the factor base is every affine point with
//! `x ∈ V \ {0}` (both signs).  Planted: three distinct base abscissas (uniform), a random
//! sign each, `R = P_1 + P_2 + P_3`.  Unplanted: `x` uniform in `[1, 2^m)` until it is on
//! the curve, `P` = the first point `points_with_x` returns, `R = [4]P`.  Either is redrawn
//! (new basis) while `R = O` or `x_R ∈ V`.  Ground truth by `point_add`:
//! `truth_ordered_total` = ordered triples of base points summing to `R`,
//! `truth_ordered_distinct` = those with pairwise distinct abscissas.  The systems also
//! admit `x_i = 0` (the 2-torsion point `(0, 1)`, since `0 ∈ V`), so the same two counts
//! over the factor base plus `(0, 1)` are recorded as `truth_ordered_*_incl_2torsion`.
//!
//! Solvers, on the identical `F2BoolPoly` list:
//!
//! * `f4`: `pq_f4_f2::groebner_basis_f4(polys, n_vars, Some(budget))`, then (finished, not
//!   oversize) `solutions_from_reduced_basis(.., 20 free, ..)`.  Every scalar `F4Stats`
//!   field, `n_solutions` (null above 2^20), `solutions` (at most 64, set-bit lists) and
//!   `verified_solutions` (decoded and checked by group arithmetic).
//! * `sat`: `sat::Solver` with one fresh Tseitin variable per distinct monomial of degree
//!   ≥ 2 (numbered after the system bits in order of first appearance, polynomial by
//!   polynomial, term by term; `t ↔ ∧ v_j` as `¬t ∨ v_j` for every `j` and
//!   `t ∨ ¬v_1 ∨ … ∨ ¬v_k`) and one native XOR per polynomial over its linear and Tseitin
//!   variables with right-hand side its constant term.  Nothing else.  `Solver::deadline`
//!   carries `--budget-s`, `Solver::conflict_budget` carries `--sat-conflicts` (default
//!   unlimited).  A model is decoded and checked the same way (`model_verified`).
//!
//! Decoding: `x_i` from the `u` bits; incidence takes `y_i` from its bits and needs the
//! three points on the curve with `P_0 + P_1 + P_2 = R`; norm needs some choice of signs
//! of the points over `x_0, x_1, x_2` summing to `R`.
//!
//! Files per system in `OUT/cells/`: `NAME.json` (`{"n_vars", "polys": [[[vars]...]...]}`,
//! the `bool_f4_file` format) and `NAME.cnf`: DIMACS with CryptoMiniSat XOR lines,
//! `p cnf VARS CONSTRAINTS` (CONSTRAINTS = clauses + XOR lines).  Variables `1..n_vars` are
//! the system bits `v_0..v_{n_vars-1}`, Tseitin variables follow.  An XOR line
//! `x l_1 … l_k 0` means `l_1 ⊕ … ⊕ l_k = 1`; a row with right-hand side 0 is written with
//! its first literal negated (the convention of `sat::parse_dimacs_xor`/`to_dimacs_xor`).
//! A polynomial equal to the constant 1 is written as the empty clause `0`.
//!
//!     rr_quadratic_panel --out DIR [--m 7,9,11,13] [--ell-offset 0] [--planted 4]
//!         [--unplanted 4] [--first-draw 0] [--budget-s 600] [--sat-conflicts N] [--arms incidence,norm]
//!         [--solvers f4,sat] [--seed 20261115] [--jobs 1]
//!
//! `ℓ = (m + 2) / 3 + offset` (integer division; `--ell-offset` takes a comma list).
//! `--jobs` is accepted and ignored: runs are sequential (F4's internal rayon threads follow
//! `RAYON_NUM_THREADS`).  One JSON line per (draw, arm, solver) is appended to
//! `DIR/results.jsonl` and summarised on stdout.
use crypto_lib::binary_ecc::curve::{point_add, point_neg, scalar_mul};
use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_bench::random_subspace_basis;
use crypto_lib::cryptanalysis::koblitz_groebner::{FieldStructure, SymElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    find_irreducible_sparse, koblitz_point_count, points_with_x, KoblitzCurve,
};
use crypto_lib::cryptanalysis::pq_f4_f2::{groebner_basis_f4, solutions_from_reduced_basis};
use crypto_lib::cryptanalysis::pq_groebner_f2::{bit, F2BoolPoly, Mask, MASK_BITS};
use crypto_lib::cryptanalysis::sat::{Lit, SolveResult, Solver};
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::time::{Duration, Instant};

const MAX_FREE: usize = 20;

fn word(e: &F2mElement) -> u64 {
    e.raw_bits().first().copied().unwrap_or(0)
}

fn elem(w: u64, m: u32) -> F2mElement {
    F2mElement::from_words(&[w], m)
}

fn pt_words(p: &BinaryPoint) -> Option<(u64, u64)> {
    match p {
        BinaryPoint::Infinity => None,
        BinaryPoint::Affine { x, y } => Some((word(x), word(y))),
    }
}

/// The curve `y² + xy = x³ + 1` over GF(2^m).  `KoblitzCurve::new(0, m)` when it admits the
/// cell (it refuses e.g. `m = 11`, `#E = 4·23²`); otherwise the same curve on the same
/// modulus as an arithmetic shell (generator unset, which nothing here uses).
fn make_curve(m: u32) -> (BinaryCurve, &'static str) {
    if let Some(kc) = KoblitzCurve::new(0, m) {
        return (kc.curve, "KoblitzCurve::new");
    }
    let irreducible = find_irreducible_sparse(m).expect("irreducible polynomial");
    (
        BinaryCurve {
            m,
            irreducible,
            a: F2mElement::zero(m),
            b: F2mElement::one(m),
            generator: BinaryPoint::Infinity,
            order: koblitz_point_count(0, m),
            cofactor: BigUint::from(1u32),
        },
        "shell (KoblitzCurve::new refused)",
    )
}

fn span_words(basis: &[F2mElement]) -> Vec<u64> {
    let mut out = vec![0u64];
    for b in basis {
        let w = word(b);
        let more: Vec<u64> = out.iter().map(|v| v ^ w).collect();
        out.extend(more);
    }
    out
}

/// Affine points with abscissa in `V \ {0}`, both signs, in span order.
fn factor_base(curve: &BinaryCurve, span: &[u64]) -> Vec<BinaryPoint> {
    let m = curve.m;
    span.iter()
        .filter(|&&x| x != 0)
        .flat_map(|&x| points_with_x(curve, &elem(x, m)))
        .collect()
}

struct Draw {
    basis: Vec<F2mElement>,
    span: Vec<u64>,
    target: BinaryPoint,
}

fn draw_instance(curve: &BinaryCurve, ell: usize, rng: &mut StdRng, planted: bool) -> Draw {
    let m = curve.m;
    for _ in 0..10000 {
        let basis = random_subspace_basis(m, ell, rng);
        let span = span_words(&basis);
        let base = factor_base(curve, &span);
        let target = if planted {
            let mut xs: Vec<u64> = base.iter().filter_map(|p| pt_words(p).map(|w| w.0)).collect();
            xs.sort_unstable();
            xs.dedup();
            if xs.len() < 3 {
                continue;
            }
            let mut idx: Vec<usize> = Vec::new();
            while idx.len() < 3 {
                let i = rng.gen_range(0..xs.len());
                if !idx.contains(&i) {
                    idx.push(i);
                }
            }
            let mut acc = BinaryPoint::Infinity;
            for &i in &idx {
                let pts = points_with_x(curve, &elem(xs[i], m));
                let p = if rng.gen::<bool>() {
                    point_neg(&pts[0])
                } else {
                    pts[0].clone()
                };
                acc = point_add(curve, &acc, &p);
            }
            acc
        } else {
            let p = loop {
                let x = rng.gen_range(1u64..(1u64 << m));
                let pts = points_with_x(curve, &elem(x, m));
                if let Some(p) = pts.into_iter().next() {
                    break p;
                }
            };
            scalar_mul(curve, &p, &BigUint::from(4u32))
        };
        match pt_words(&target) {
            None => continue,
            Some((xr, _)) if span.contains(&xr) => continue,
            Some(_) => {}
        }
        return Draw {
            basis,
            span,
            target,
        };
    }
    panic!("could not draw an instance");
}

/// (ordered total, ordered with pairwise distinct abscissas, factor-base size).
fn count_truth(curve: &BinaryCurve, base: &[BinaryPoint], target: &BinaryPoint) -> (u64, u64, usize) {
    let have: HashSet<(u64, u64)> = base.iter().filter_map(pt_words).collect();
    let (mut total, mut distinct) = (0u64, 0u64);
    for p in base {
        let rp = point_add(curve, target, &point_neg(p));
        for q in base {
            let r = point_add(curve, &rp, &point_neg(q));
            if let Some(rw) = pt_words(&r) {
                if have.contains(&rw) {
                    total += 1;
                    let (xp, xq) = (pt_words(p).unwrap().0, pt_words(q).unwrap().0);
                    if xp != xq && xp != rw.0 && xq != rw.0 {
                        distinct += 1;
                    }
                }
            }
        }
    }
    (total, distinct, base.len())
}

/// The system for `arm`, or `Err(n_vars)` when it exceeds the mask width.
fn build_system(
    arm: &str,
    curve: &BinaryCurve,
    st: &FieldStructure,
    basis: &[F2mElement],
    target: &BinaryPoint,
) -> Result<(Vec<F2BoolPoly>, usize), usize> {
    let m = curve.m;
    let ell = basis.len();
    let mu = m as usize;
    let n_vars = match arm {
        "incidence" => 3 * ell + 8 * mu,
        "norm" => 3 * ell + 2 * mu,
        _ => panic!("unknown arm {arm}"),
    };
    if n_vars > MASK_BITS {
        return Err(n_vars);
    }
    let irr = &curve.irreducible;
    let BinaryPoint::Affine { x: xr, y: yr } = target else {
        panic!("target at infinity");
    };
    let k = |v: &F2mElement| SymElement::constant(v, m, n_vars);
    let xs: Vec<SymElement> = (0..3)
        .map(|i| SymElement::from_subspace_vars(basis, i * ell, m, n_vars))
        .collect();
    let mut nxt = 3 * ell;
    let mut free = || {
        let e = SymElement::from_free_vars(nxt, m, n_vars);
        nxt += mu;
        e
    };
    let one = k(&F2mElement::one(m));
    let mut polys: Vec<F2BoolPoly> = Vec::new();
    if arm == "incidence" {
        let ys: Vec<SymElement> = (0..3).map(|_| free()).collect();
        let a = free();
        let b = free();
        let ws: Vec<SymElement> = (0..3).map(|_| free()).collect();
        let c = k(&xr.square(irr))
            .add(&a.mul_constant(xr, st))
            .add(&b.mul_constant(&xr.add(yr), st));
        for i in 0..3 {
            let (x, y) = (&xs[i], &ys[i]);
            let curve_eq = y
                .square(st)
                .add(&x.mul(y, st))
                .add(&x.square(st).mul(x, st))
                .add(&one);
            let inc = x.square(st).add(&a.mul(x, st)).add(&c).add(&b.mul(y, st));
            polys.extend(curve_eq.coords);
            polys.extend(inc.coords);
        }
        for (w, (i, j)) in ws.iter().zip([(0, 1), (0, 2), (1, 2)]) {
            polys.extend(xs[i].add(&xs[j]).mul(w, st).add(&one).coords);
        }
    } else {
        let a = free();
        let b = free();
        let c = k(&xr.square(irr))
            .add(&a.mul_constant(xr, st))
            .add(&b.mul_constant(&xr.add(yr), st));
        let u = xs[0].add(&xs[1]);
        let v = xs[0].mul(&xs[1], st);
        let w = xs[2].add(&k(xr));
        let z = xs[2].mul_constant(xr, st);
        let e1 = u.add(&w);
        let e2 = v.add(&z).add(&u.mul(&w, st));
        let e3 = u.mul(&z, st).add(&v.mul(&w, st));
        let e4 = v.mul(&z, st);
        let n1 = b.add(&b.square(st));
        let n2 = a.square(st).add(&a.mul(&b, st));
        let n3 = b.mul(&c, st);
        let n4 = c.square(st).add(&b.square(st));
        for (e, nn) in [(e1, n1), (e2, n2), (e3, n3), (e4, n4)] {
            polys.extend(e.add(&nn).coords);
        }
    }
    polys.retain(|p| !p.is_zero());
    Ok((polys, n_vars))
}

/// Independent check of an assignment (`bits[k]` = system bit `k`) by group arithmetic.
fn decode_ok(
    arm: &str,
    curve: &BinaryCurve,
    basis: &[F2mElement],
    target: &BinaryPoint,
    bits: &[bool],
) -> bool {
    let m = curve.m;
    let ell = basis.len();
    let mu = m as usize;
    let xs: Vec<u64> = (0..3)
        .map(|i| {
            (0..ell)
                .filter(|&k| bits[i * ell + k])
                .fold(0u64, |acc, k| acc ^ word(&basis[k]))
        })
        .collect();
    if arm == "incidence" {
        let mut acc = BinaryPoint::Infinity;
        for (i, &x) in xs.iter().enumerate() {
            let off = 3 * ell + i * mu;
            let y = (0..mu).filter(|&j| bits[off + j]).fold(0u64, |a, j| a | 1 << j);
            let p = BinaryPoint::Affine {
                x: elem(x, m),
                y: elem(y, m),
            };
            if !curve.is_on_curve(&p) {
                return false;
            }
            acc = point_add(curve, &acc, &p);
        }
        return acc == *target;
    }
    let mut cands = Vec::new();
    for &x in &xs {
        let pts = points_with_x(curve, &elem(x, m));
        if pts.is_empty() {
            return false;
        }
        cands.push(pts[0].clone());
    }
    (0..8u32).any(|signs| {
        let mut acc = BinaryPoint::Infinity;
        for (i, p) in cands.iter().enumerate() {
            let q = if signs >> i & 1 == 1 {
                point_neg(p)
            } else {
                p.clone()
            };
            acc = point_add(curve, &acc, &q);
        }
        acc == *target
    })
}

fn mask_bits(s: Mask, n_vars: usize) -> Vec<bool> {
    (0..n_vars).map(|k| s & bit(k as u32) != 0).collect()
}

/// Tseitin + native-XOR encoding: clauses, XOR rows `(vars, rhs)` (1-indexed), total vars.
struct Encoding {
    clauses: Vec<Vec<Lit>>,
    xors: Vec<(Vec<u32>, bool)>,
    n_total: u32,
    n_tseitin: u32,
}

fn encode(polys: &[F2BoolPoly], n_vars: usize) -> Encoding {
    let mut mono_var: HashMap<Mask, u32> = HashMap::new();
    let mut next = n_vars as u32 + 1;
    let mut clauses = Vec::new();
    let mut xors = Vec::new();
    for p in polys {
        let mut vars = Vec::new();
        let mut rhs = false;
        for t in &p.terms {
            let deg = t.degree();
            if deg == 0 {
                rhs ^= true;
                continue;
            }
            let lits: Vec<u32> = (0..n_vars as u32)
                .filter(|&k| t.mask & bit(k) != 0)
                .map(|k| k + 1)
                .collect();
            if deg == 1 {
                vars.push(lits[0]);
                continue;
            }
            let tv = *mono_var.entry(t.mask).or_insert_with(|| {
                let tv = next;
                next += 1;
                for &l in &lits {
                    clauses.push(vec![-(tv as Lit), l as Lit]);
                }
                let mut big: Vec<Lit> = lits.iter().map(|&l| -(l as Lit)).collect();
                big.insert(0, tv as Lit);
                clauses.push(big);
                tv
            });
            vars.push(tv);
        }
        xors.push((vars, rhs));
    }
    Encoding {
        clauses,
        xors,
        n_total: next - 1,
        n_tseitin: next - 1 - n_vars as u32,
    }
}

fn write_cnf(path: &std::path::Path, enc: &Encoding) {
    let mut lines = Vec::new();
    for c in &enc.clauses {
        let mut s: String = c.iter().map(|l| format!("{l} ")).collect();
        s.push('0');
        lines.push(s);
    }
    for (vars, rhs) in &enc.xors {
        if vars.is_empty() {
            if *rhs {
                lines.push("0".to_string());
            }
            continue;
        }
        let mut s = String::from("x");
        for (i, &v) in vars.iter().enumerate() {
            let l = if i == 0 && !rhs { -(v as i64) } else { v as i64 };
            s.push_str(&format!(" {l}"));
        }
        s.push_str(" 0");
        lines.push(s);
    }
    let mut text = format!("p cnf {} {}\n", enc.n_total, lines.len());
    for l in lines {
        text.push_str(&l);
        text.push('\n');
    }
    std::fs::write(path, text).expect("write cnf");
}

fn write_json(path: &std::path::Path, polys: &[F2BoolPoly], n_vars: usize) {
    let ps: Vec<Vec<Vec<usize>>> = polys
        .iter()
        .map(|p| {
            p.terms
                .iter()
                .map(|t| (0..n_vars).filter(|&k| t.mask & bit(k as u32) != 0).collect())
                .collect()
        })
        .collect();
    std::fs::write(path, json!({"n_vars": n_vars, "polys": ps}).to_string()).expect("write json");
}

fn run_f4(
    polys: &[F2BoolPoly],
    n_vars: usize,
    budget: Duration,
    verify: &dyn Fn(&[bool]) -> bool,
) -> Value {
    let t0 = Instant::now();
    let (gb, stats) = groebner_basis_f4(polys.to_vec(), n_vars, Some(budget));
    let mut out = Map::new();
    if let Value::Object(mm) = serde_json::to_value(&stats).expect("serialize F4Stats") {
        for (k, v) in mm {
            if !v.is_array() {
                out.insert(k, v);
            }
        }
    }
    let decided = !stats.timed_out && !stats.oversize;
    if decided {
        let mut tests = 0u64;
        match solutions_from_reduced_basis(&gb, n_vars, MAX_FREE, &mut tests) {
            Some(sols) => {
                let verified = sols
                    .iter()
                    .filter(|&&s| verify(&mask_bits(s, n_vars)))
                    .count();
                out.insert("n_solutions".into(), json!(sols.len()));
                out.insert("verified_solutions".into(), json!(verified));
                if sols.len() <= 64 {
                    let lists: Vec<Vec<usize>> = sols
                        .iter()
                        .map(|&s| (0..n_vars).filter(|&k| s & bit(k as u32) != 0).collect())
                        .collect();
                    out.insert("solutions".into(), json!(lists));
                }
            }
            None => {
                out.insert("n_solutions".into(), Value::Null);
                out.insert("verified_solutions".into(), Value::Null);
            }
        }
    }
    out.insert("basis_len".into(), json!(gb.len()));
    out.insert("decided".into(), json!(decided));
    out.insert("wall_outer_s".into(), json!(t0.elapsed().as_secs_f64()));
    Value::Object(out)
}

fn run_sat(
    enc: &Encoding,
    n_vars: usize,
    budget: Duration,
    conflicts: Option<u64>,
    verify: &dyn Fn(&[bool]) -> bool,
) -> Value {
    let t0 = Instant::now();
    let mut s = Solver::new(enc.n_total);
    let mut trivially_unsat = false;
    for c in &enc.clauses {
        trivially_unsat |= !s.add_clause(c.clone());
    }
    for (vars, rhs) in &enc.xors {
        trivially_unsat |= !s.add_xor(vars, *rhs);
    }
    s.conflict_budget = conflicts.unwrap_or(u64::MAX);
    s.deadline = Some(Instant::now() + budget);
    let res = if trivially_unsat {
        SolveResult::Unsat
    } else {
        s.solve()
    };
    let wall = t0.elapsed().as_secs_f64();
    let status = match res {
        SolveResult::Sat => "sat",
        SolveResult::Unsat => "unsat",
        SolveResult::Unknown => "undecided",
    };
    let mut out = json!({
        "status": status,
        "decided": res != SolveResult::Unknown,
        "conflicts": s.stats.conflicts,
        "decisions": s.stats.decisions,
        "restarts": s.stats.restarts,
        "propagations": s.stats.propagations,
        "xor_conflicts": s.stats.xor_conflicts,
        "learnt_clauses": s.stats.learnt_clauses,
        "n_tseitin": enc.n_tseitin,
        "n_solver_vars": enc.n_total,
        "n_clauses": enc.clauses.len(),
        "n_xors": enc.xors.len(),
        "conflict_budget": conflicts,
        "time_budget_s": budget.as_secs_f64(),
        "wall_s": wall,
    });
    if res == SolveResult::Sat {
        let model = s.model();
        out["model_verified"] = json!(verify(&model[..n_vars]));
        out["model_bits"] = json!((0..n_vars).filter(|&k| model[k]).collect::<Vec<_>>());
    }
    out["wall_outer_s"] = json!(t0.elapsed().as_secs_f64());
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let list = |name: &str, default: &str| -> Vec<String> {
        get(name)
            .unwrap_or_else(|| default.to_string())
            .split(',')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    };
    let out_dir = std::path::PathBuf::from(get("--out").expect("--out DIR is required"));
    let ms: Vec<u32> = list("--m", "7,9,11,13").iter().map(|s| s.parse().expect("--m")).collect();
    let offs: Vec<i64> = list("--ell-offset", "0")
        .iter()
        .map(|s| s.parse().expect("--ell-offset"))
        .collect();
    let num = |name: &str, d: &str| get(name).unwrap_or_else(|| d.to_string());
    let planted: u32 = num("--planted", "4").parse().expect("--planted");
    let unplanted: u32 = num("--unplanted", "4").parse().expect("--unplanted");
    // Draws below this index are drawn (the RNG stream is unchanged) but not solved,
    // so a cell can be resumed or split across processes with identical instances.
    let first_draw: u32 = num("--first-draw", "0").parse().expect("--first-draw");
    let budget_s: f64 = num("--budget-s", "600").parse().expect("--budget-s");
    let sat_conflicts: Option<u64> = get("--sat-conflicts").map(|s| s.parse().expect("--sat-conflicts"));
    let arms = list("--arms", "incidence,norm");
    let solvers = list("--solvers", "f4,sat");
    let seed: u64 = num("--seed", "20261115").parse().expect("--seed");
    let _jobs: usize = num("--jobs", "1").parse().expect("--jobs");
    let budget = Duration::from_secs_f64(budget_s);

    let cells = out_dir.join("cells");
    std::fs::create_dir_all(&cells).expect("create OUT/cells");
    let mut results = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(out_dir.join("results.jsonl"))
        .expect("open results.jsonl");
    eprintln!("MASK_BITS = {MASK_BITS}");

    for &m in &ms {
        let (curve, curve_source) = make_curve(m);
        let st = FieldStructure::new(m, &curve.irreducible);
        let order = koblitz_point_count(0, m);
        let irr_word: u64 = curve
            .irreducible
            .low_terms
            .iter()
            .fold(1u64 << m, |a, &t| a | 1 << t);
        for &off in &offs {
            let ell = ((m as i64 + 2) / 3 + off) as usize;
            let cell_seed = seed ^ (u64::from(m) << 40) ^ ((ell as u64) << 32);
            let mut rng = StdRng::seed_from_u64(cell_seed);
            for d in 0..planted + unplanted {
                let is_planted = d < planted;
                let dr = draw_instance(&curve, ell, &mut rng, is_planted);
                if d < first_draw {
                    continue;
                }
                let base = factor_base(&curve, &dr.span);
                let (total, distinct, nbase) = count_truth(&curve, &base, &dr.target);
                // Diagnostic only: the systems admit x_i = 0 (0 ∈ V), i.e. the 2-torsion
                // point (0, 1), which the factor base (x ∈ V \ {0}) excludes.
                let mut base0 = base.clone();
                base0.extend(points_with_x(&curve, &F2mElement::zero(m)));
                let (total0, distinct0, _) = count_truth(&curve, &base0, &dr.target);
                let (txr, tyr) = pt_words(&dr.target).unwrap();
                let head = json!({
                    "m": m, "ell": ell, "draw": d, "planted": is_planted,
                    "order": order.to_string(), "curve": "y^2+xy=x^3+1",
                    "curve_source": curve_source, "irreducible": irr_word,
                    "seed": seed, "cell_seed": cell_seed,
                    "basis": dr.basis.iter().map(word).collect::<Vec<_>>(),
                    "target": [txr, tyr],
                    "truth_ordered_total": total, "truth_ordered_distinct": distinct,
                    "factor_base_points": nbase,
                    "truth_ordered_total_incl_2torsion": total0,
                    "truth_ordered_distinct_incl_2torsion": distinct0,
                });
                for arm in &arms {
                    let name = format!("m{m}_l{ell}_d{d}_{arm}");
                    let mut info = head.clone();
                    info["arm"] = json!(arm);
                    info["system"] = json!(name);
                    let built = build_system(arm, &curve, &st, &dr.basis, &dr.target);
                    let (polys, n_vars) = match built {
                        Ok(v) => v,
                        Err(nv) => {
                            for solver in &solvers {
                                let mut line = info.clone();
                                line["n_vars"] = json!(nv);
                                line["solver"] = json!(solver);
                                line["result"] = json!({
                                    "skipped": format!("n_vars {nv} > MASK_BITS {MASK_BITS}"),
                                    "decided": false,
                                });
                                writeln!(results, "{line}").expect("write");
                                println!("{name} {solver} SKIPPED n_vars={nv} > MASK_BITS={MASK_BITS}");
                            }
                            continue;
                        }
                    };
                    let degree = polys
                        .iter()
                        .flat_map(|p| p.terms.iter().map(|t| t.degree()))
                        .max()
                        .unwrap_or(0);
                    let terms: usize = polys.iter().map(|p| p.terms.len()).sum();
                    info["n_vars"] = json!(n_vars);
                    info["n_polys"] = json!(polys.len());
                    info["degree"] = json!(degree);
                    info["terms"] = json!(terms);
                    write_json(&cells.join(format!("{name}.json")), &polys, n_vars);
                    let enc = encode(&polys, n_vars);
                    write_cnf(&cells.join(format!("{name}.cnf")), &enc);
                    let verify = |bits: &[bool]| decode_ok(arm, &curve, &dr.basis, &dr.target, bits);
                    for solver in &solvers {
                        let res = match solver.as_str() {
                            "f4" => run_f4(&polys, n_vars, budget, &verify),
                            "sat" => run_sat(&enc, n_vars, budget, sat_conflicts, &verify),
                            other => panic!("unknown solver {other}"),
                        };
                        let mut line = info.clone();
                        line["solver"] = json!(solver);
                        line["result"] = res;
                        writeln!(results, "{line}").expect("write");
                        results.flush().expect("flush");
                        let r = &line["result"];
                        println!(
                            "{name} {solver} nv={n_vars} np={} deg={degree} truth_tot={total} truth_dist={distinct} truth0_tot={total0} truth0_dist={distinct0} decided={} sd={} dr={} rows={} nsol={} ver={} status={} model_ok={} confl={} wall={:.2}",
                            polys.len(),
                            r["decided"], r["solving_degree"], r["degree_reached"],
                            r["matrix_rows_max"], r["n_solutions"], r["verified_solutions"],
                            r["status"], r["model_verified"], r["conflicts"],
                            r["wall_outer_s"].as_f64().unwrap_or(0.0)
                        );
                    }
                }
            }
        }
    }
}
