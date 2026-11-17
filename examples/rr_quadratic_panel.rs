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
//!     rr_quadratic_panel --out DIR [--m 7,9,11,13] [--ell-offset 0] [--ell L1,L2] [--planted 4]
//!         [--unplanted 4] [--first-draw 0] [--budget-s 600] [--sat-conflicts N] [--arms incidence,norm]
//!         [--solvers f4,sat] [--xl-degree 4] [--plant-k 3] [--check] [--seed 20261115] [--jobs 1]
//!
//! `ℓ = (m + 2) / 3 + offset` (integer division; `--ell-offset` takes a comma list), unless
//! `--ell` gives an explicit comma list of `ℓ` values, which then replaces the offsets.
//! `--jobs` is accepted and ignored: runs are sequential (F4's internal rayon threads follow
//! `RAYON_NUM_THREADS`).  One JSON line per (draw, arm, solver) is appended to
//! `DIR/results.jsonl` and summarised on stdout.
//!
//! ## Follow-up arms (PREREG_FOLLOWUPS.md F1, F3, F5)
//!
//! * `rr4` (Boolean degree 4): the norm form with `α, β` eliminated, as
//!   `rr_degree_ladder::build_rr` (ported here with `a = curve.a = 0`): `3ℓ + 1` bits, the
//!   summand bits then one free bit `ε` (`β = H(e₁) + ε`); equations = the `X²` and `X¹`
//!   coefficient equations and `Tr(e₁)`.  Needs `m` odd and `x_R ≠ 0` (else `skipped`).
//!   One ordered point triple gives exactly one `(x-bits, ε)` (the pair fixes `α, β, γ`, so
//!   `f`, so its zeros), hence `n_solutions` must equal `truth_ordered_total_incl_2torsion`
//!   and `n_distinct_x_triples` (distinct ordered `(x₁, x₂, x₃)` among verified solutions)
//!   must equal `truth_ordered_xtriples`.
//! * `s4` (Boolean degree 6): `S₄(x₁, x₂, x₃, x_R)` Weil-descended with
//!   `koblitz_symmetrised::build_direct_x_system` (`3ℓ` bits), as `rr_degree_ladder`'s `x4`.
//!   `S₄(x₁,x₂,x₃,x_R) = 0` iff `±P₁ ± P₂ ± P₃ ± R = O` for points `P_i` over `x_i` **over the
//!   algebraic closure** (`rr_degree_ladder::x4_root_count` counts roots by evaluating `S₄`
//!   in the field for that reason); flipping every sign covers `−R`, so with rational `P_i`
//!   it is "some sign choice of the `P_i` sums to `R`".  An `x_i ∈ V` without a rational
//!   point has twist points (`φ(P) = −P`); two of them can contribute `P₁ + P₂ = T`, giving a
//!   field root with no rational decomposition.  So `truth_for_arm` for `s4` is
//!   `truth_s4_field_roots` (field evaluation over `V³`, terms of `plain_terms(3)`), and
//!   `truth_ordered_xtriples` (rational) is recorded beside it; they differ only by such
//!   twist roots, which fail the group-arithmetic verification by design.
//! * `norm4` (Boolean degree 4): four summands.  `f = xy + c₂x² + c₁y + c₀x + c₋ ∈ L(5O)`,
//!   `f = A + By`, `A = c₂x² + c₀x + c₋`, `B = x + c₁`.  `N(f) = (A + By)(A + B(y + x))
//!   = A² + xAB + B²(x³ + a x² + 1)`
//!   `= X⁵ + (c₂² + c₂ + a)X⁴ + (c₂c₁ + c₀ + c₁²)X³ + (c₀² + c₀c₁ + c₋ + 1 + a c₁²)X²
//!   + c₋c₁ X + (c₋² + c₁²)`.  Pinning: `f(−R) = 0` at `−R = (x_R, x_R + y_R)` gives
//!   `c₋ = x_R(x_R + y_R) + c₂x_R² + c₁(x_R + y_R) + c₀x_R` (linear in the `c`'s).  The five
//!   coefficients are matched with `E₁ … E₅`, the elementary symmetric functions of
//!   `{x_R, x₁, x₂, x₃, x₄}`: `5m` equations in `4ℓ + 3m` bits (summand bits, then `c₂, c₁,
//!   c₀`, `m` bits each).  A solution with every `B(x_i) ≠ 0` gives rational `y_i = A(x_i)/B(x_i)`
//!   and one ordered 4-tuple summing to `R`; conversely each 4-tuple fixes `f` (pole order
//!   exactly 5, monic in `xy`).  *Degenerate* solutions have `x_i = x_j = c₁` for two
//!   positions: `f = (x + c₁)(y + c x + d)`, both points over `c₁` are zeros (a pair `P, −P`,
//!   or `T` twice when `c₁ = 0`) and the other two positions carry `Q + Q' = R`.  A rational
//!   4-tuple with a pair `P_i = −P_j` maps 2-to-1 onto these (swapping the pair), and `c₁ ∈ V`
//!   with **no** rational point gives degenerate solutions with no rational 4-tuple at all.
//!   So `truth_for_arm` for `norm4` is `truth4_system_solutions`, the solution count of the
//!   coefficient equations computed in the field over `V⁴` (`c₂` from the `X⁴` Artin–Schreier
//!   equation, `c₁` from the quadratic that pinning, `X⁰` and `X³` give, then `X²` and `X¹`
//!   checked; needs `m` odd), split into `truth4_system_generic` (must equal
//!   `truth4_ordered_nopair`, the rational ordered 4-tuples with no pair `P_i = −P_j`) and
//!   `truth4_system_degenerate`; `truth4_ordered_total` (all rational ordered 4-tuples of
//!   base points, `T` included, summing to `R`) is recorded too.  A model verifies when the
//!   coefficient identity holds in the field and, generic, the recovered points are on the
//!   curve and sum to `R`, or, degenerate, a sign search over rational points over the
//!   `x_i` reaches `R` (a degenerate solution over a pointless `c₁` does not verify;
//!   `n_degenerate_nonrational` counts them).  `--plant-k 4` plants four summands; it is the
//!   default when every requested arm is `norm4` (3 otherwise; 3 keeps the RNG stream of the
//!   panel of record).
//!
//! Solver `xl` (`--xl-degree D`, default 4): one Macaulay matrix at degree `D` (rows `t·f`
//! for every input `f` and every monomial `t` with `deg t ≤ D − deg f`; columns every
//! monomial of degree `≤ D`, graded so the linear and constant columns come last), reduced
//! to echelon form with distinct leads, dense, in this file: `pq_xl::boolean_xl_counted`
//! asserts `n_vars ≤ 24` and enumerates all `2^n_vars` masks, which excludes most cells, so
//! it runs only as a cross-check when `n_vars ≤ 24` (`pq_xl_*` fields; `pq_xl_agrees` =
//! same total and linear rank).  The rows with a linear lead span the degree-≤1 part of the
//! row space; `xl_complete` = they are inconsistent or pin every bit.  Otherwise, with at
//! most 20 free bits left, the free bits are enumerated through the linear part and each
//! candidate is checked against the input system (`xl_enumerated`), so `n_solutions` is exact
//! whenever it is present.  Skipped when `C(n, ≤ D) > 3·10⁶` or rows·cols/8 > 2 GB.
//! `xl_ops` counts 64-bit word operations of this implementation (one per monomial, term,
//! table probe, scanned or XORed word and evaluated term), not `pq_xl`'s count.
//!
//! Every line carries `truth_for_arm` (the count to compare), `truth_for_arm_field` (its
//! head field) and `truth_for_arm_compares` (`n_solutions` or `n_distinct_x_triples`).
//! `--check` brute-forces every built system with at most 27 bits over its cube and records
//! `check_brute_solutions` (and `check_brute_distinct_x_triples` for `rr4`/`s4`).
use crypto_lib::binary_ecc::curve::{point_add, point_neg, scalar_mul};
use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_bench::random_subspace_basis;
use crypto_lib::cryptanalysis::koblitz_groebner::{FieldStructure, SymElement};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    find_irreducible_sparse, koblitz_point_count, points_with_x, KoblitzCurve,
};
use crypto_lib::cryptanalysis::koblitz_symmetrised::{build_direct_x_system, plain_terms};
use crypto_lib::cryptanalysis::pq_f4_f2::{groebner_basis_f4, solutions_from_reduced_basis};
use crypto_lib::cryptanalysis::pq_groebner_f2::{bit, F2BoolMono, F2BoolPoly, Mask, MASK_BITS};
use crypto_lib::cryptanalysis::pq_xl::boolean_xl_counted;
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

fn draw_instance(
    curve: &BinaryCurve,
    ell: usize,
    rng: &mut StdRng,
    planted: bool,
    plant_k: usize,
) -> Draw {
    let m = curve.m;
    for _ in 0..10000 {
        let basis = random_subspace_basis(m, ell, rng);
        let span = span_words(&basis);
        let base = factor_base(curve, &span);
        let target = if planted {
            let mut xs: Vec<u64> = base.iter().filter_map(|p| pt_words(p).map(|w| w.0)).collect();
            xs.sort_unstable();
            xs.dedup();
            if xs.len() < plant_k {
                continue;
            }
            let mut idx: Vec<usize> = Vec::new();
            while idx.len() < plant_k {
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

/// GF(2^m) on one word (`m ≤ 63`), `irr` the modulus including the `z^m` bit.  Independent
/// of `F2mElement` and of the symbolic code; checked against `F2mElement::mul` per cell.
#[derive(Clone, Copy)]
struct Gf {
    m: u32,
    irr: u64,
}

impl Gf {
    fn mul(&self, a: u64, b: u64) -> u64 {
        let (mut r, mut a, mut b) = (0u64, a, b);
        while b != 0 {
            if b & 1 != 0 {
                r ^= a;
            }
            b >>= 1;
            a <<= 1;
            if a >> self.m & 1 != 0 {
                a ^= self.irr;
            }
        }
        r
    }
    fn sq(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    fn sq_k(&self, a: u64, k: u32) -> u64 {
        (0..k).fold(a, |t, _| self.sq(t))
    }
    fn sqrt(&self, a: u64) -> u64 {
        self.sq_k(a, self.m - 1)
    }
    /// `a^(2^m − 2)`.
    fn inv(&self, a: u64) -> u64 {
        let (mut acc, mut t) = (1u64, self.sq(a));
        for _ in 1..self.m {
            acc = self.mul(acc, t);
            t = self.sq(t);
        }
        acc
    }
    fn trace(&self, a: u64) -> u64 {
        let (mut acc, mut t) = (a, a);
        for _ in 1..self.m {
            t = self.sq(t);
            acc ^= t;
        }
        acc
    }
    /// `m` odd: `H(c)² + H(c) = c` when `Tr(c) = 0`.
    fn half_trace(&self, a: u64) -> u64 {
        let (mut acc, mut t) = (a, a);
        for _ in 0..(self.m - 1) / 2 {
            t = self.sq(self.sq(t));
            acc ^= t;
        }
        acc
    }
    fn pow(&self, a: u64, e: u32) -> u64 {
        (0..e).fold(1u64, |acc, _| self.mul(acc, a))
    }
    /// Roots in `c` of `al c² + be c + ga = 0`, `al ≠ 0`, `m` odd.
    fn quad_roots(&self, al: u64, be: u64, ga: u64) -> Vec<u64> {
        if be == 0 {
            return vec![self.sqrt(self.mul(ga, self.inv(al)))];
        }
        // c = (be/al) z, z² + z = al·ga/be²
        let k = self.mul(self.mul(al, ga), self.inv(self.sq(be)));
        if self.trace(k) != 0 {
            return vec![];
        }
        let z = self.half_trace(k);
        let s = self.mul(be, self.inv(al));
        vec![self.mul(s, z), self.mul(s, z ^ 1)]
    }
}

/// Elementary symmetric functions `[1, E₁, …, E_k]` of `roots`.
fn elem_sym(gf: &Gf, roots: &[u64]) -> Vec<u64> {
    let mut e = vec![1u64];
    for &r in roots {
        let mut n = e.clone();
        n.push(0);
        for j in 1..n.len() {
            n[j] ^= gf.mul(r, e[j - 1]);
        }
        e = n;
    }
    e
}

/// Ordered triples of `x`-values in `V` (0 allowed) for which some sign choice of the
/// rational points over them sums to `target`, from the ordered point triples of `base0`
/// (the factor base plus `T`).
fn count_xtriples(curve: &BinaryCurve, base0: &[BinaryPoint], target: &BinaryPoint) -> u64 {
    let have: HashSet<(u64, u64)> = base0.iter().filter_map(pt_words).collect();
    let mut seen: HashSet<(u64, u64, u64)> = HashSet::new();
    for p in base0 {
        let rp = point_add(curve, target, &point_neg(p));
        for q in base0 {
            let r = point_add(curve, &rp, &point_neg(q));
            if let Some(rw) = pt_words(&r) {
                if have.contains(&rw) {
                    seen.insert((pt_words(p).unwrap().0, pt_words(q).unwrap().0, rw.0));
                }
            }
        }
    }
    seen.len() as u64
}

/// `x`-triples in `V³` at which `S₄(x₁, x₂, x₃, x_R)` vanishes in the field (the term list of
/// `plain_terms(3)`, as `rr_degree_ladder::x4_root_count`).
fn count_s4_field_roots(gf: &Gf, span: &[u64], xr: u64) -> u64 {
    let terms = plain_terms(3).expect("S4 terms");
    let mut count = 0u64;
    for &x1 in span {
        for &x2 in span {
            for &x3 in span {
                let xs = [x1, x2, x3, xr];
                let mut acc = 0u64;
                for t in &terms {
                    let mut prod = 1u64;
                    for (i, &e) in t.iter().enumerate() {
                        if e > 0 {
                            prod = gf.mul(prod, gf.pow(xs[i], e));
                        }
                    }
                    acc ^= prod;
                }
                if acc == 0 {
                    count += 1;
                }
            }
        }
    }
    count
}

/// Rational ordered 4-tuples of `base0` points summing to `target`: (all, with no pair
/// `P_i = −P_j`, `T` twice counting as such a pair).
fn count_truth4(curve: &BinaryCurve, base0: &[BinaryPoint], target: &BinaryPoint) -> (u64, u64) {
    let index: HashMap<(u64, u64), usize> = base0
        .iter()
        .enumerate()
        .filter_map(|(i, p)| pt_words(p).map(|w| (w, i)))
        .collect();
    let negs: Vec<usize> = base0.iter().map(|p| index[&pt_words(&point_neg(p)).unwrap()]).collect();
    let (mut total, mut nopair) = (0u64, 0u64);
    for (i1, p1) in base0.iter().enumerate() {
        let r1 = point_add(curve, target, &point_neg(p1));
        for (i2, p2) in base0.iter().enumerate() {
            let r2 = point_add(curve, &r1, &point_neg(p2));
            for (i3, p3) in base0.iter().enumerate() {
                let r3 = point_add(curve, &r2, &point_neg(p3));
                let Some(&i4) = pt_words(&r3).and_then(|w| index.get(&w)) else {
                    continue;
                };
                total += 1;
                let idx = [i1, i2, i3, i4];
                let pair = (0..4).any(|a| (a + 1..4).any(|b| idx[a] == negs[idx[b]]));
                if !pair {
                    nopair += 1;
                }
            }
        }
    }
    (total, nopair)
}

/// Solutions of the `norm4` coefficient equations in the field over `V⁴`:
/// (total, generic = every `x_i ≠ c₁`, degenerate).  `m` odd.
fn count_norm4_system(gf: &Gf, span: &[u64], xr: u64, yr: u64, a: u64) -> (u64, u64, u64) {
    let t = xr ^ yr;
    let k0 = gf.mul(xr, t);
    let r2 = gf.sq(xr);
    let (mut total, mut generic) = (0u64, 0u64);
    for &x1 in span {
        for &x2 in span {
            for &x3 in span {
                for &x4 in span {
                    let xs = [x1, x2, x3, x4];
                    let e = elem_sym(gf, &[xr, x1, x2, x3, x4]);
                    let e1a = e[1] ^ a;
                    if gf.trace(e1a) != 0 {
                        continue;
                    }
                    let h = gf.half_trace(e1a);
                    let s = gf.sqrt(e[5]);
                    for c2 in [h, h ^ 1] {
                        // c₋ = c₁ + s (X⁰), c₀ = E₂ + c₂c₁ + c₁² (X³), pinning
                        // c₋ = k0 + c₂x_R² + c₁t + c₀x_R  ⇒
                        // x_R c₁² + (t + x_R c₂ + 1) c₁ + (k0 + c₂x_R² + x_R E₂ + s) = 0
                        let be = t ^ gf.mul(xr, c2) ^ 1;
                        let ga = k0 ^ gf.mul(c2, r2) ^ gf.mul(xr, e[2]) ^ s;
                        for c1 in gf.quad_roots(xr, be, ga) {
                            let cm = c1 ^ s;
                            let c0 = e[2] ^ gf.mul(c2, c1) ^ gf.sq(c1);
                            let n2 = gf.sq(c0) ^ gf.mul(c0, c1) ^ cm ^ 1 ^ gf.mul(a, gf.sq(c1));
                            let n1 = gf.mul(cm, c1);
                            if n2 == e[3] && n1 == e[4] {
                                total += 1;
                                if xs.iter().all(|&x| x != c1) {
                                    generic += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    (total, generic, total - generic)
}

/// `Some(true)` if some sign choice of the rational points over `xs` sums to `target`,
/// `None` if some `x` has no rational point.
fn sign_search(curve: &BinaryCurve, xs: &[u64], target: &BinaryPoint) -> Option<bool> {
    let m = curve.m;
    let mut cands = Vec::new();
    for &x in xs {
        let pts = points_with_x(curve, &elem(x, m));
        if pts.is_empty() {
            return None;
        }
        cands.push(pts[0].clone());
    }
    Some((0..1u32 << xs.len()).any(|signs| {
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
    }))
}

fn sym_sqrt(e: &SymElement, st: &FieldStructure) -> SymElement {
    let mut t = e.clone();
    for _ in 1..st.n {
        t = t.square(st);
    }
    t
}

fn sym_half_trace(e: &SymElement, st: &FieldStructure) -> SymElement {
    let (mut acc, mut t) = (e.clone(), e.clone());
    for _ in 0..(st.n - 1) / 2 {
        t = t.square(st).square(st);
        acc = acc.add(&t);
    }
    acc
}

/// The trace as a Boolean polynomial; every other coordinate of `Σ e^{2^j}` must vanish
/// identically, which is checked.
fn sym_trace(e: &SymElement, st: &FieldStructure) -> F2BoolPoly {
    let (mut acc, mut t) = (e.clone(), e.clone());
    for _ in 1..st.n {
        t = t.square(st);
        acc = acc.add(&t);
    }
    assert!(
        acc.coords[1..].iter().all(|c| c.is_zero()),
        "trace lies in F_2"
    );
    acc.coords[0].clone()
}

/// `rr_degree_ladder::build_rr` with `a = curve.a`: `2m + 1` equations in `3ℓ + 1` bits.
fn build_rr4(
    curve: &BinaryCurve,
    basis: &[F2mElement],
    target: &BinaryPoint,
    st: &FieldStructure,
) -> Result<(Vec<F2BoolPoly>, usize), (usize, String)> {
    let (n, irr, ell) = (curve.m, &curve.irreducible, basis.len());
    let n_vars = 3 * ell + 1;
    if n_vars > MASK_BITS {
        return Err((n_vars, format!("n_vars {n_vars} > MASK_BITS {MASK_BITS}")));
    }
    if n % 2 == 0 {
        return Err((n_vars, format!("rr4 needs m odd (m = {n})")));
    }
    let BinaryPoint::Affine { x: r, y: s } = target else {
        panic!("target at infinity");
    };
    let Some(r_inv) = r.flt_inverse(irr) else {
        return Err((n_vars, "rr4 needs x_R != 0".to_string()));
    };
    let c = |v: &F2mElement| SymElement::constant(v, n, n_vars);
    let one = F2mElement::one(n);
    let x: Vec<SymElement> = (0..3)
        .map(|i| SymElement::from_subspace_vars(basis, i * ell, n, n_vars))
        .collect();
    let rc = c(r);
    let s1 = x[0].add(&x[1]).add(&x[2]);
    let x12 = x[0].mul(&x[1], st);
    let s2 = x12.add(&x[0].mul(&x[2], st)).add(&x[1].mul(&x[2], st));
    let s3 = x12.mul(&x[2], st);
    let e1 = rc.add(&s1);
    let e2 = rc.mul(&s1, st).add(&s2);
    let e3 = rc.mul(&s2, st).add(&s3);
    let e4 = rc.mul(&s3, st);
    let mut eps = SymElement::zero(n, n_vars);
    eps.coords[0] = F2BoolPoly::from_monos(vec![F2BoolMono::var((3 * ell) as u32)], n_vars);
    let beta = sym_half_trace(&e1, st).add(&eps);
    let k1 = r_inv.mul(&r.add(s).add(&one), irr);
    let alpha = c(&r_inv)
        .mul(&sym_sqrt(&e4, st), st)
        .add(&rc)
        .add(&c(&k1).mul(&beta, st));
    let gamma = c(&r.square(irr))
        .add(&rc.mul(&alpha, st))
        .add(&c(&r.add(s)).mul(&beta, st));
    let res4 = gamma.square(st).add(&beta.square(st)).add(&e4);
    assert!(
        res4.coords.iter().all(|p| p.is_zero()),
        "constant coefficient identity"
    );
    let res1 = beta.square(st).add(&beta).add(&e1);
    let tr = sym_trace(&e1, st);
    assert!(
        res1.coords[1..].iter().all(|p| p.is_zero()) && res1.coords[0] == tr,
        "X³ coefficient identity up to Tr(e₁)"
    );
    let ab = alpha.mul(&beta, st);
    let eq2 = alpha
        .square(st)
        .add(&ab)
        .add(&c(&curve.a).mul(&beta.square(st), st))
        .add(&e2);
    let eq3 = beta.mul(&gamma, st).add(&e3);
    let mut eqs: Vec<F2BoolPoly> = eq2.coords.into_iter().chain(eq3.coords).collect();
    eqs.push(tr);
    eqs.retain(|p| !p.is_zero());
    Ok((eqs, n_vars))
}

/// The four-summand norm form (see the module comment): `5m` equations in `4ℓ + 3m` bits.
fn build_norm4(
    curve: &BinaryCurve,
    basis: &[F2mElement],
    target: &BinaryPoint,
    st: &FieldStructure,
) -> Result<(Vec<F2BoolPoly>, usize), (usize, String)> {
    let (m, irr, ell) = (curve.m, &curve.irreducible, basis.len());
    let mu = m as usize;
    let n_vars = 4 * ell + 3 * mu;
    if n_vars > MASK_BITS {
        return Err((n_vars, format!("n_vars {n_vars} > MASK_BITS {MASK_BITS}")));
    }
    if m % 2 == 0 {
        // The field-level ground truth solves quadratics by the half trace.
        return Err((n_vars, format!("norm4 truth needs m odd (m = {m})")));
    }
    let BinaryPoint::Affine { x: xr, y: yr } = target else {
        panic!("target at infinity");
    };
    let k = |v: &F2mElement| SymElement::constant(v, m, n_vars);
    let xs: Vec<SymElement> = (0..4)
        .map(|i| SymElement::from_subspace_vars(basis, i * ell, m, n_vars))
        .collect();
    let c2 = SymElement::from_free_vars(4 * ell, m, n_vars);
    let c1 = SymElement::from_free_vars(4 * ell + mu, m, n_vars);
    let c0 = SymElement::from_free_vars(4 * ell + 2 * mu, m, n_vars);
    let t = xr.add(yr);
    // f(−R) = 0:  c₋ = x_R(x_R + y_R) + c₂x_R² + c₁(x_R + y_R) + c₀x_R
    let cm = k(&xr.mul(&t, irr))
        .add(&c2.mul_constant(&xr.square(irr), st))
        .add(&c1.mul_constant(&t, st))
        .add(&c0.mul_constant(xr, st));
    // E₁..E₅ of {x_R, x₁, x₂, x₃, x₄}
    let mut e: Vec<SymElement> = vec![k(&F2mElement::one(m)), k(xr)];
    for x in &xs {
        let mut ne = e.clone();
        ne.push(SymElement::zero(m, n_vars));
        for j in 1..ne.len() {
            ne[j] = ne[j].add(&x.mul(&e[j - 1], st));
        }
        e = ne;
    }
    let a = k(&curve.a);
    let one = k(&F2mElement::one(m));
    let c1sq = c1.square(st);
    let n4 = c2.square(st).add(&c2).add(&a);
    let n3 = c2.mul(&c1, st).add(&c0).add(&c1sq);
    let n2 = c0
        .square(st)
        .add(&c0.mul(&c1, st))
        .add(&cm)
        .add(&one)
        .add(&a.mul(&c1sq, st));
    let n1 = cm.mul(&c1, st);
    let n0 = cm.square(st).add(&c1sq);
    let mut polys: Vec<F2BoolPoly> = Vec::new();
    for (nn, ee) in [n4, n3, n2, n1, n0].iter().zip(&e[1..]) {
        polys.extend(nn.add(ee).coords);
    }
    polys.retain(|p| !p.is_zero());
    Ok((polys, n_vars))
}

/// Result of checking one assignment of an arm's bits.
struct Assess {
    ok: bool,
    /// The summand abscissas.
    xs: Vec<u64>,
    /// `norm4` only: generic, degenerate_rational, degenerate_nonrational,
    /// degenerate_fail, not_a_root (coefficient identity fails), generic_fail.
    class: &'static str,
}

/// The system for `arm`, or `Err((n_vars, reason))` when it is not built (wider than the
/// mask, or outside the arm's preconditions).
fn build_system(
    arm: &str,
    curve: &BinaryCurve,
    st: &FieldStructure,
    basis: &[F2mElement],
    target: &BinaryPoint,
) -> Result<(Vec<F2BoolPoly>, usize), (usize, String)> {
    let m = curve.m;
    let ell = basis.len();
    let mu = m as usize;
    match arm {
        "rr4" => return build_rr4(curve, basis, target, st),
        "norm4" => return build_norm4(curve, basis, target, st),
        "s4" => {
            let n_vars = 3 * ell;
            if n_vars > MASK_BITS {
                return Err((n_vars, format!("n_vars {n_vars} > MASK_BITS {MASK_BITS}")));
            }
            let BinaryPoint::Affine { x: xr, .. } = target else {
                panic!("target at infinity");
            };
            return match build_direct_x_system(basis, xr, 3, st) {
                Some((mut polys, nv)) => {
                    polys.retain(|p| !p.is_zero());
                    Ok((polys, nv))
                }
                None => Err((n_vars, "build_direct_x_system refused".to_string())),
            };
        }
        _ => {}
    }
    let n_vars = match arm {
        "incidence" => 3 * ell + 8 * mu,
        "norm" => 3 * ell + 2 * mu,
        _ => panic!("unknown arm {arm}"),
    };
    if n_vars > MASK_BITS {
        return Err((n_vars, format!("n_vars {n_vars} > MASK_BITS {MASK_BITS}")));
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
    assess(arm, curve, basis, target, bits).ok
}

fn assess(
    arm: &str,
    curve: &BinaryCurve,
    basis: &[F2mElement],
    target: &BinaryPoint,
    bits: &[bool],
) -> Assess {
    let m = curve.m;
    let ell = basis.len();
    let mu = m as usize;
    let n_summands = if arm == "norm4" { 4 } else { 3 };
    let xs: Vec<u64> = (0..n_summands)
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
                return Assess { ok: false, xs, class: "" };
            }
            acc = point_add(curve, &acc, &p);
        }
        return Assess { ok: acc == *target, xs, class: "" };
    }
    if arm == "norm4" {
        return assess_norm4(curve, target, bits, xs, ell);
    }
    // norm, rr4, s4: some sign choice of the points over x_0, x_1, x_2 sums to R
    let ok = sign_search(curve, &xs, target).unwrap_or(false);
    Assess { ok, xs, class: "" }
}

fn gf_of(curve: &BinaryCurve) -> Gf {
    let m = curve.m;
    let irr = curve
        .irreducible
        .low_terms
        .iter()
        .fold(1u64 << m, |a, &t| a | 1 << t);
    Gf { m, irr }
}

fn assess_norm4(
    curve: &BinaryCurve,
    target: &BinaryPoint,
    bits: &[bool],
    xs: Vec<u64>,
    ell: usize,
) -> Assess {
    let m = curve.m;
    let mu = m as usize;
    let gf = gf_of(curve);
    let field = |off: usize| (0..mu).filter(|&j| bits[off + j]).fold(0u64, |a, j| a | 1 << j);
    let (c2, c1, c0) = (field(4 * ell), field(4 * ell + mu), field(4 * ell + 2 * mu));
    let (xr, yr) = pt_words(target).expect("affine target");
    let a = word(&curve.a);
    let t = xr ^ yr;
    let cm = gf.mul(xr, t) ^ gf.mul(c2, gf.sq(xr)) ^ gf.mul(c1, t) ^ gf.mul(c0, xr);
    let e = elem_sym(&gf, &[xr, xs[0], xs[1], xs[2], xs[3]]);
    let c1sq = gf.sq(c1);
    let n = [
        gf.sq(c2) ^ c2 ^ a,
        gf.mul(c2, c1) ^ c0 ^ c1sq,
        gf.sq(c0) ^ gf.mul(c0, c1) ^ cm ^ 1 ^ gf.mul(a, c1sq),
        gf.mul(cm, c1),
        gf.sq(cm) ^ c1sq,
    ];
    if n[..] != e[1..] {
        return Assess { ok: false, xs, class: "not_a_root" };
    }
    if xs.iter().all(|&x| x != c1) {
        let mut acc = BinaryPoint::Infinity;
        for &x in &xs {
            let av = gf.mul(c2, gf.sq(x)) ^ gf.mul(c0, x) ^ cm;
            let y = gf.mul(av, gf.inv(x ^ c1));
            let p = BinaryPoint::Affine {
                x: elem(x, m),
                y: elem(y, m),
            };
            if !curve.is_on_curve(&p) {
                return Assess { ok: false, xs, class: "generic_fail" };
            }
            acc = point_add(curve, &acc, &p);
        }
        let ok = acc == *target;
        return Assess { ok, xs, class: if ok { "generic" } else { "generic_fail" } };
    }
    match sign_search(curve, &xs, target) {
        Some(true) => Assess { ok: true, xs, class: "degenerate_rational" },
        Some(false) => Assess { ok: false, xs, class: "degenerate_fail" },
        None => Assess { ok: false, xs, class: "degenerate_nonrational" },
    }
}

/// Arm-specific fields over a list of solutions (`rr4`/`s4`: distinct x-triples;
/// `norm4`: solution classes).  Empty for the panel-of-record arms.
fn arm_extra(
    arm: &str,
    curve: &BinaryCurve,
    basis: &[F2mElement],
    target: &BinaryPoint,
    sols: &[Vec<bool>],
) -> Map<String, Value> {
    let mut out = Map::new();
    match arm {
        "rr4" | "s4" => {
            let mut all: HashSet<Vec<u64>> = HashSet::new();
            let mut ver: HashSet<Vec<u64>> = HashSet::new();
            for b in sols {
                let a = assess(arm, curve, basis, target, b);
                if a.ok {
                    ver.insert(a.xs.clone());
                }
                all.insert(a.xs);
            }
            out.insert("n_distinct_x_triples".into(), json!(ver.len()));
            out.insert("n_distinct_x_triples_all".into(), json!(all.len()));
        }
        "norm4" => {
            let mut classes: std::collections::BTreeMap<&str, u64> = Default::default();
            for b in sols {
                *classes.entry(assess(arm, curve, basis, target, b).class).or_default() += 1;
            }
            out.insert("n_generic".into(), json!(classes.get("generic").copied().unwrap_or(0)));
            out.insert(
                "n_degenerate".into(),
                json!(classes
                    .iter()
                    .filter(|(k, _)| k.starts_with("degenerate"))
                    .map(|(_, v)| v)
                    .sum::<u64>()),
            );
            out.insert(
                "n_degenerate_nonrational".into(),
                json!(classes.get("degenerate_nonrational").copied().unwrap_or(0)),
            );
            out.insert("solution_classes".into(), json!(classes));
        }
        _ => {}
    }
    out
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
    extra: &dyn Fn(&[Vec<bool>]) -> Map<String, Value>,
) -> Value {
    let t0 = Instant::now();
    let (gb, stats) = groebner_basis_f4(polys.to_vec(), n_vars, Some(budget));
    let mut out = Map::new();
    if let Value::Object(mm) = serde_json::to_value(stats).expect("serialize F4Stats") {
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
                let all: Vec<Vec<bool>> = sols.iter().map(|&s| mask_bits(s, n_vars)).collect();
                out.extend(extra(&all));
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
    extra: &dyn Fn(&[Vec<bool>]) -> Map<String, Value>,
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
        for (k, v) in extra(&[model[..n_vars].to_vec()]) {
            out[format!("model_{k}")] = v;
        }
    }
    out["wall_outer_s"] = json!(t0.elapsed().as_secs_f64());
    out
}

const XL_MAX_COLS: f64 = 3e6;
const XL_MAX_BYTES: f64 = 2e9;

fn binom(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    (0..k).fold(1.0, |acc, i| acc * (n - i) as f64 / (i + 1) as f64)
}

fn binom_upto(n: usize, d: usize) -> f64 {
    (0..=d).map(|k| binom(n, k)).sum()
}

/// Every `k`-subset of `0..n` as a mask, appended to `out`.
fn combos(n: usize, k: usize, start: usize, cur: Mask, out: &mut Vec<Mask>) {
    if k == 0 {
        out.push(cur);
        return;
    }
    for v in start..=n - k {
        combos(n, k - 1, v + 1, cur | bit(v as u32), out);
    }
}

fn poly_degree(p: &F2BoolPoly) -> u32 {
    p.terms.iter().map(|t| t.degree()).max().unwrap_or(0)
}

/// Fixed-degree XL (see the module comment).
fn run_xl(
    polys: &[F2BoolPoly],
    n_vars: usize,
    d: u32,
    budget: Duration,
    verify: &dyn Fn(&[bool]) -> bool,
    extra: &dyn Fn(&[Vec<bool>]) -> Map<String, Value>,
) -> Value {
    let t0 = Instant::now();
    let deadline = t0 + budget;
    let du = d as usize;
    let cols_est = binom_upto(n_vars, du);
    let mut rows_est = 0f64;
    let mut unused = 0usize;
    for f in polys {
        let df = poly_degree(f) as usize;
        if df > du {
            unused += 1;
        } else {
            rows_est += binom_upto(n_vars, du - df);
        }
    }
    let bytes_est = rows_est * (cols_est / 64.0).ceil() * 8.0;
    let mut out = json!({
        "xl_degree": d, "xl_cols": cols_est as u64, "xl_rows_bound": rows_est as u64,
        "xl_bytes_bound": bytes_est as u64, "xl_polys_unused": unused,
        "time_budget_s": budget.as_secs_f64(),
    });
    if unused > 0 {
        // XL below the system degree drops those equations; whatever followed would be
        // enumeration over a weaker system, not XL.
        out["skipped"] = json!(format!(
            "XL degree {d} < degree of {unused} input polynomials (system degree {})",
            polys.iter().map(poly_degree).max().unwrap_or(0)
        ));
        out["decided"] = json!(false);
        out["wall_outer_s"] = json!(t0.elapsed().as_secs_f64());
        return out;
    }
    if cols_est > XL_MAX_COLS || bytes_est > XL_MAX_BYTES {
        out["skipped"] = json!(format!(
            "XL guard: cols {cols_est:.0} (max {XL_MAX_COLS:.0}), rows*cols/8 {bytes_est:.3e} B (max {XL_MAX_BYTES:.0e})"
        ));
        out["decided"] = json!(false);
        out["wall_outer_s"] = json!(t0.elapsed().as_secs_f64());
        return out;
    }
    let mut ops = 0u64;
    // Columns: degree D first, ..., degree 1, then the constant (last column).
    let mut monos: Vec<Mask> = Vec::new();
    let mut deg_start = vec![0usize; du + 2];
    for k in (0..=du).rev() {
        deg_start[k] = monos.len();
        combos(n_vars, k, 0, 0, &mut monos);
    }
    deg_start[du + 1] = 0;
    let n_cols = monos.len();
    let col_of: HashMap<Mask, usize> = monos.iter().enumerate().map(|(i, &m)| (m, i)).collect();
    ops += 3 * n_cols as u64;
    let words = n_cols.div_ceil(64);
    let lin_start = n_cols - (n_vars + 1);
    let mut pivot_of: Vec<u32> = vec![u32::MAX; n_cols];
    ops += n_cols as u64;
    let mut store: Vec<Vec<u64>> = Vec::new();
    let mut n_rows = 0u64;
    let mut timed_out = false;
    let lead = |row: &[u64], from: usize, ops: &mut u64| -> Option<usize> {
        for (w, &x) in row.iter().enumerate().skip(from) {
            *ops += 1;
            if x != 0 {
                return Some(w * 64 + x.trailing_zeros() as usize);
            }
        }
        None
    };
    'outer: for f in polys {
        let df = poly_degree(f) as usize;
        if df > du {
            continue;
        }
        let headroom = du - df;
        // monomials of degree ≤ headroom: indices deg_start[headroom] .. n_cols
        for &t in &monos[deg_start[headroom]..] {
            if Instant::now() > deadline {
                timed_out = true;
                break 'outer;
            }
            let prod = f.mul_mono(F2BoolMono::from_mask(t));
            ops += prod.terms.len() as u64 + f.terms.len() as u64;
            if prod.is_zero() {
                continue;
            }
            n_rows += 1;
            let mut row = vec![0u64; words];
            for term in &prod.terms {
                let c = col_of[&term.mask];
                row[c / 64] ^= 1 << (c % 64);
            }
            ops += 3 * prod.terms.len() as u64 + words as u64;
            let mut from = 0usize;
            while let Some(l) = lead(&row, from, &mut ops) {
                ops += 1;
                let p = pivot_of[l];
                if p == u32::MAX {
                    pivot_of[l] = store.len() as u32;
                    store.push(row);
                    break;
                }
                let w0 = l / 64;
                let prow = &store[p as usize];
                for w in w0..words {
                    row[w] ^= prow[w];
                }
                ops += (words - w0) as u64;
                from = w0;
            }
        }
    }
    out["xl_rows"] = json!(n_rows);
    out["xl_cols"] = json!(n_cols);
    out["xl_rank"] = json!(store.len());
    out["xl_timed_out"] = json!(timed_out);
    out["xl_matrix_wall_s"] = json!(t0.elapsed().as_secs_f64());
    if timed_out {
        out["xl_ops"] = json!(ops);
        out["decided"] = json!(false);
        out["wall_outer_s"] = json!(t0.elapsed().as_secs_f64());
        return out;
    }
    // The linear part: rows whose lead is a linear or constant column, as (vars, constant).
    let mut lin: Vec<(Mask, bool)> = Vec::new();
    for (c, &p) in pivot_of.iter().enumerate().skip(lin_start) {
        if p == u32::MAX {
            continue;
        }
        let row = &store[p as usize];
        let mut vars: Mask = 0;
        let mut cst = false;
        for cc in c..n_cols {
            if row[cc / 64] >> (cc % 64) & 1 == 1 {
                let mm = monos[cc];
                if mm == 0 {
                    cst = true;
                } else {
                    vars |= mm;
                }
            }
        }
        ops += (n_cols - c) as u64 / 64 + 1;
        lin.push((vars, cst));
    }
    let inconsistent = lin.iter().any(|&(v, c)| v == 0 && c);
    // Gauss–Jordan on the linear rows (distinct leads already: rank = lin.len()).
    let mut piv: Vec<(u32, Mask, bool)> = Vec::new();
    if !inconsistent {
        for &(v, c) in &lin {
            let (mut v, mut c) = (v, c);
            for &(pv, pm, pc) in &piv {
                if v & bit(pv) != 0 {
                    v ^= pm;
                    c ^= pc;
                }
            }
            ops += piv.len() as u64;
            if v == 0 {
                continue;
            }
            let pv = v.trailing_zeros();
            for e in piv.iter_mut() {
                if e.1 & bit(pv) != 0 {
                    e.1 ^= v;
                    e.2 ^= c;
                }
            }
            piv.push((pv, v, c));
        }
    }
    let lin_rank = if inconsistent { lin.len() } else { piv.len() };
    let free_vars: Vec<u32> = (0..n_vars as u32)
        .filter(|&k| !piv.iter().any(|e| e.0 == k))
        .collect();
    let n_free = if inconsistent { 0 } else { free_vars.len() };
    out["xl_linear_rank"] = json!(lin_rank);
    out["xl_inconsistent"] = json!(inconsistent);
    out["xl_free_after_linear"] = json!(n_free);
    out["xl_complete"] = json!(inconsistent || n_free == 0);
    let mut sols: Vec<Mask> = Vec::new();
    let mut enumerated = false;
    if inconsistent {
        enumerated = true;
    } else if n_free <= MAX_FREE {
        enumerated = true;
        for a in 0u64..1u64 << n_free {
            if a & 0xfff == 0 && Instant::now() > deadline {
                enumerated = false;
                timed_out = true;
                break;
            }
            let mut asg: Mask = 0;
            for (i, &fv) in free_vars.iter().enumerate() {
                if a >> i & 1 == 1 {
                    asg |= bit(fv);
                }
            }
            for &(pv, pm, pc) in &piv {
                let rest = pm & !bit(pv) & asg;
                if (rest.count_ones() % 2 == 1) ^ pc {
                    asg |= bit(pv);
                }
            }
            ops += piv.len() as u64;
            let mut okk = true;
            for p in polys {
                ops += p.terms.len() as u64;
                if p.eval(asg) != 0 {
                    okk = false;
                    break;
                }
            }
            if okk {
                sols.push(asg);
            }
        }
    }
    out["xl_enumerated"] = json!(enumerated);
    out["xl_timed_out"] = json!(timed_out);
    if enumerated {
        let all: Vec<Vec<bool>> = sols.iter().map(|&s| mask_bits(s, n_vars)).collect();
        let verified = all.iter().filter(|b| verify(b)).count();
        out["n_solutions"] = json!(sols.len());
        out["verified_solutions"] = json!(verified);
        if sols.len() <= 64 {
            let lists: Vec<Vec<usize>> = sols
                .iter()
                .map(|&s| (0..n_vars).filter(|&k| s & bit(k as u32) != 0).collect())
                .collect();
            out["solutions"] = json!(lists);
        }
        for (k, v) in extra(&all) {
            out[k] = v;
        }
    }
    out["decided"] = json!(enumerated);
    out["xl_ops"] = json!(ops);
    // Cross-check against the library's XL where it accepts the system.
    if n_vars <= 24 {
        let tx = Instant::now();
        let mut pops = 0u64;
        let red = boolean_xl_counted(polys.to_vec(), n_vars, d, &mut pops);
        let rank = red.iter().filter(|p| !p.is_zero()).count();
        let lrank = red
            .iter()
            .filter(|p| !p.is_zero() && poly_degree(p) <= 1)
            .count();
        out["pq_xl_ops"] = json!(pops);
        out["pq_xl_rank"] = json!(rank);
        out["pq_xl_linear_rank"] = json!(lrank);
        out["pq_xl_agrees"] = json!(rank == store.len() && lrank == lin.len());
        out["pq_xl_wall_s"] = json!(tx.elapsed().as_secs_f64());
    }
    out["wall_outer_s"] = json!(t0.elapsed().as_secs_f64());
    out
}

/// Brute force over `{0,1}^n_vars` (small systems only): every root.
fn brute_roots(polys: &[F2BoolPoly], n_vars: usize) -> Vec<Mask> {
    (0u64..1u64 << n_vars)
        .map(crypto_lib::cryptanalysis::pq_groebner_f2::mask_from64)
        .filter(|&a| polys.iter().all(|p| p.eval(a) == 0))
        .collect()
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
    // `--ell L1,L2,…`: explicit ℓ values, replacing `(m + 2) / 3 + offset`.
    let ells: Option<Vec<usize>> = get("--ell").map(|v| {
        v.split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().expect("--ell"))
            .collect()
    });
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
    let xl_degree: u32 = num("--xl-degree", "4").parse().expect("--xl-degree");
    let all_norm4 = arms.iter().all(|a| a == "norm4");
    let plant_k: usize = match get("--plant-k") {
        Some(v) => v.parse().expect("--plant-k"),
        None => {
            if arms.iter().any(|a| a == "norm4") && !all_norm4 {
                eprintln!("warning: norm4 with 3-summand arms: planting 3 summands (--plant-k)");
            }
            if all_norm4 {
                4
            } else {
                3
            }
        }
    };
    let check = args.iter().any(|a| a == "--check");
    for arm in &arms {
        assert!(
            ["incidence", "norm", "rr4", "s4", "norm4"].contains(&arm.as_str()),
            "unknown arm {arm}"
        );
    }
    for solver in &solvers {
        assert!(["f4", "sat", "xl"].contains(&solver.as_str()), "unknown solver {solver}");
    }
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
        let gf = gf_of(&curve);
        {
            // The one-word field arithmetic against F2mElement.
            let mut trng = StdRng::seed_from_u64(u64::from(m));
            for _ in 0..64 {
                let (a, b) = (trng.gen_range(0..1u64 << m), trng.gen_range(0..1u64 << m));
                let want = word(&elem(a, m).mul(&elem(b, m), &curve.irreducible));
                assert_eq!(gf.mul(a, b), want, "Gf::mul against F2mElement::mul");
                if a != 0 {
                    assert_eq!(gf.mul(a, gf.inv(a)), 1, "Gf::inv");
                }
                assert_eq!(gf.sq(gf.sqrt(a)), a, "Gf::sqrt");
            }
        }
        let cell_ells: Vec<usize> = match &ells {
            Some(v) => v.clone(),
            None => offs
                .iter()
                .map(|&off| ((m as i64 + 2) / 3 + off) as usize)
                .collect(),
        };
        for &ell in &cell_ells {
            let cell_seed = seed ^ (u64::from(m) << 40) ^ ((ell as u64) << 32);
            let mut rng = StdRng::seed_from_u64(cell_seed);
            for d in 0..planted + unplanted {
                let is_planted = d < planted;
                let dr = draw_instance(&curve, ell, &mut rng, is_planted, plant_k);
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
                let xtriples = count_xtriples(&curve, &base0, &dr.target);
                let (txr, tyr) = pt_words(&dr.target).unwrap();
                let mut head = json!({
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
                    "truth_ordered_xtriples": xtriples,
                    "plant_k": plant_k,
                });
                if arms.iter().any(|a| a == "s4") {
                    head["truth_s4_field_roots"] = json!(count_s4_field_roots(&gf, &dr.span, txr));
                }
                if arms.iter().any(|a| a == "norm4") {
                    let (t4, t4np) = count_truth4(&curve, &base0, &dr.target);
                    head["truth4_ordered_total"] = json!(t4);
                    head["truth4_ordered_nopair"] = json!(t4np);
                    if m % 2 == 1 {
                        let (st4, sg4, sd4) =
                            count_norm4_system(&gf, &dr.span, txr, tyr, word(&curve.a));
                        head["truth4_system_solutions"] = json!(st4);
                        head["truth4_system_generic"] = json!(sg4);
                        head["truth4_system_degenerate"] = json!(sd4);
                    }
                }
                for arm in &arms {
                    let name = format!("m{m}_l{ell}_d{d}_{arm}");
                    let mut info = head.clone();
                    info["arm"] = json!(arm);
                    info["system"] = json!(name);
                    let (tfield, tcompares) = match arm.as_str() {
                        "incidence" => ("truth_ordered_distinct_incl_2torsion", "n_solutions"),
                        "norm" => ("truth_ordered_total_incl_2torsion", "n_solutions"),
                        "rr4" => ("truth_ordered_xtriples", "n_distinct_x_triples"),
                        "s4" => ("truth_s4_field_roots", "n_distinct_x_triples_all"),
                        "norm4" => ("truth4_system_solutions", "n_solutions"),
                        _ => unreachable!(),
                    };
                    info["truth_for_arm"] = head[tfield].clone();
                    info["truth_for_arm_field"] = json!(tfield);
                    info["truth_for_arm_compares"] = json!(tcompares);
                    let tfa = head[tfield].clone();
                    let built = build_system(arm, &curve, &st, &dr.basis, &dr.target);
                    let (polys, n_vars) = match built {
                        Ok(v) => v,
                        Err((nv, reason)) => {
                            for solver in &solvers {
                                let mut line = info.clone();
                                line["n_vars"] = json!(nv);
                                line["solver"] = json!(solver);
                                line["result"] = json!({
                                    "skipped": reason,
                                    "decided": false,
                                });
                                writeln!(results, "{line}").expect("write");
                                println!("{name} {solver} SKIPPED n_vars={nv}: {reason}");
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
                    let extra = |sols: &[Vec<bool>]| arm_extra(arm, &curve, &dr.basis, &dr.target, sols);
                    if check && n_vars <= 27 {
                        let roots = brute_roots(&polys, n_vars);
                        info["check_brute_solutions"] = json!(roots.len());
                        if arm == "rr4" || arm == "s4" {
                            let all: Vec<Vec<bool>> =
                                roots.iter().map(|&s| mask_bits(s, n_vars)).collect();
                            let ex = extra(&all);
                            info["check_brute_distinct_x_triples"] = ex["n_distinct_x_triples_all"].clone();
                        }
                        eprintln!(
                            "{name} check: brute {} truth_for_arm {} ({tfield})",
                            roots.len(),
                            tfa
                        );
                    }
                    for solver in &solvers {
                        let res = match solver.as_str() {
                            "f4" => run_f4(&polys, n_vars, budget, &verify, &extra),
                            "sat" => run_sat(&enc, n_vars, budget, sat_conflicts, &verify, &extra),
                            "xl" => run_xl(&polys, n_vars, xl_degree, budget, &verify, &extra),
                            other => panic!("unknown solver {other}"),
                        };
                        let mut line = info.clone();
                        line["solver"] = json!(solver);
                        line["result"] = res;
                        writeln!(results, "{line}").expect("write");
                        results.flush().expect("flush");
                        let r = &line["result"];
                        println!(
                            "{name} {solver} nv={n_vars} np={} deg={degree} truth_tot={total} truth_dist={distinct} truth0_tot={total0} truth0_dist={distinct0} decided={} sd={} dr={} rows={} nsol={} ver={} status={} model_ok={} confl={} wall={:.2} tfa={tfa} ndx={} xlc={} xlcomplete={} skipped={}",
                            polys.len(),
                            r["decided"], r["solving_degree"], r["degree_reached"],
                            r["matrix_rows_max"], r["n_solutions"], r["verified_solutions"],
                            r["status"], r["model_verified"], r["conflicts"],
                            r["wall_outer_s"].as_f64().unwrap_or(0.0),
                            r["n_distinct_x_triples"], r["xl_cols"], r["xl_complete"],
                            r["skipped"]
                        );
                    }
                }
            }
        }
    }
}
