//! Faugère–Svartz equivariant F4 as a library module: block-diagonalise a
//! Macaulay matrix under a cyclic symmetry, reduce each block alone, and
//! map the block results back to ordinary `F_2` polynomial rows.
//!
//! This is a **stage** tool.  It makes no end-to-end `S` claim and has no
//! `benchmarks/` entry or leaderboard row.
//!
//! # Provenance
//!
//! The prototype is the G2 experiment, `research/g2_equivariant_f4_20261008/`
//! (notes and timing tables: 31–128× on rank at `n = 11–23`, a projected
//! 300× dense at `n = 131`; it refuted a gauge-fixing argument that
//! predicted no gain).  Its code is spread over
//! `examples/g2_equivariant_orbit_probe.rs` (orbit counting and the first
//! block-rank identity), `examples/g2_wide_probe.rs` (the same with 192-bit
//! masks) and `examples/ic_improve_02_equivariant_oracle.rs`
//! (`equivariant_pass`, which also transforms the degree-fall rows back).
//! The examples are untouched; this module is the part that is a library.
//!
//! # The group action the prototype uses
//!
//! One Weil-descended Koblitz system over `F_{2^n}` (`n` an odd prime in the
//! prototype: 7, 11, 13, 17, 19, 23), written in a **normal basis** so that
//! the Frobenius `τ: x ↦ x²` permutes the coordinate bit-variables.  The
//! "barrel" decomposition system has five cyclic variable blocks of length
//! `n` — the one-hot selectors `s_0, s_1, s_2` (summand `i` lies in
//! `τ^{k_i}(W)`; `s_0` selects the target's shift) and the descended
//! coordinates `u_1, u_2` — plus `2l` window coefficients `c_1, c_2` that
//! are fixed.  The group is `G = ⟨σ⟩ ≅ Z/n` with
//! `σ: v_{b,k} ↦ v_{b,k+1 mod n}` on every cyclic block simultaneously, i.e.
//! a permutation of the `n_vars` Boolean variables of order `n`.  The
//! equation set is closed under `σ`: the `n` coordinates of `S₃` rotate, the
//! `u`-constraint coordinates rotate, the one-hot pair equations
//! `s[k] s[k']` go to `s[k+1] s[k'+1]`, and the one-hot sums are fixed.
//! [`CyclicAction`] is exactly this: a variable permutation of order
//! dividing `n`.  Only the cyclic Frobenius action is implemented; the
//! hyperoctahedral flips and summand permutations of the leads assessment
//! are not.
//!
//! # Orbits and blocks
//!
//! The Macaulay matrix at degree `D` has a row `m·f` for every input
//! polynomial `f` and every Boolean monomial `m` with
//! `deg m + deg f ≤ D` (the products that reduce to zero are dropped), and
//! a column for every monomial that occurs.  `σ` permutes rows, `(m, f) ↦
//! (σm, σf)`, and columns, so the row space `V ⊂ M = F_2[columns]` is a
//! `G`-submodule.  A row orbit has size `n` or is fixed (`n` prime); the
//! same for column orbits.  Because `n` is odd, `F_2[G]` is semisimple and
//! `V` splits into isotypic parts:
//!
//! * **trivial block** (over `F_2`): coordinates are the column orbit sums;
//!   the row of orbit representative `r̄` is `e_0·r̄ = Σ_t σ^t r̄`, whose
//!   entry at column orbit `c̄` is the parity of `r̄`'s entries over `c̄`
//!   (fixed columns included).  Reduced by the crate's own `F_2` elimination
//!   kernel (`rref_f2_counted`, the kernel `matrix_f4_f2_counted` calls).
//! * **character blocks**: for each Galois orbit `{j, 2j, …}` of nonzero
//!   characters of `Z/n` one block over `K = F_{2^d}`, `d = ord_n(2)`, with
//!   `ζ ∈ K` a primitive `n`-th root of unity.  Coordinates are the *free*
//!   column orbits (fixed columns lie in the trivial part only) and the
//!   entry of `r̄` at `c̄` is the discrete Fourier coefficient
//!   `Σ_t r̄[σ^t c̄] ζ^{-jt}`.  Rows fixed by `σ` contribute nothing.  For
//!   prime `n` every nonzero character orbit has size exactly `d`, so a
//!   block of `K`-rank `ρ_j` has `F_2`-dimension `d·ρ_j`.
//!
//! Hence `rank V = rank_triv + Σ_j d·rank_j` (the prototype's identity,
//! which held at `n = 7, 11, 13` and degrees 2–3), and each block is about
//! `1/n` of the rows and columns of `M`.
//!
//! # Why a true character DFT needs the splitting field
//!
//! Over `F_2` the group algebra `F_2[x]/(x^n − 1)` factors as
//! `F_2 × Π_j F_{2^{d}}`; the nontrivial characters take values in `K`,
//! not in `F_2` (for `n = 11, 13`, `x^n−1 = (x−1)Φ_n` with `Φ_n`
//! irreducible of degree `n−1`).  An `F_2`-only "orbit sum" transform sees
//! just the trivial character — the trivial block above — and cannot
//! separate the other `n−1` dimensions of each free orbit.  So the
//! character blocks are over `K` (prototype: table `GF(2^d)`, `d ≤ 16`,
//! bit-serial multiply up to 40) and the transform is the DFT with
//! `ζ ∈ K`.  The cost of the splitting field is what the prototype called
//! "extension-field cost"; `d = ord_n(2)` is 130 at `n = 131`, beyond this
//! module (`EquivariantError::FieldTooLarge`), where block elimination
//! would have to use polynomial arithmetic modulo `Φ_n`.
//!
//! # Inverse transform
//!
//! [`inverse_dft`] maps block coordinates back to `F_2` Boolean-ring rows.
//! A trivial-block row `u` becomes the orbit-sum polynomial
//! `Σ_{c̄∈u} Σ_{c∈c̄} c`.  A reduced character-block row `w ∈ K^{free}`
//! yields `d` rows, one per `F_2`-basis element `λ` of `K`:
//! `v_λ[σ^t c̄] = Tr_{K/F_2}(λ·w_{c̄}·ζ^{jt})`.  Then the forward transform
//! of `v_λ` is `λw` (the other Frobenius conjugates of `j` fall outside
//! the block), so `v_λ` is the preimage of `λw` in the `j`-isotypic part of
//! `V`.  The `rank_triv + Σ_j d·ρ_j` rows so produced are an `F_2`-basis of `V`
//! (not an echelon basis).
//!
//! # Row-space agreement
//!
//! [`row_space_agrees`] says: the `F_2` row space spanned by the inverse-
//! transformed equivariant rows equals the row space of the plain
//! `matrix_f4_f2` output at the same degree.  It is tested by computing the
//! reduced row echelon form of both row sets over one common monomial
//! order (descending mask) and comparing them row for row; RREF is unique,
//! so equality of RREFs is equality of spaces.  Both sides are spans of the
//! same Macaulay row set, so agreement is the test that block reduction
//! plus inverse transform lost and invented nothing.  Invariance of the
//! input set under `σ` is a precondition ([`equivariant_f4`] returns
//! [`EquivariantError::NotInvariant`] otherwise;
//! [`equivariant_f4_or_trivial`] falls back to the trivial action).
//!
//! # Measured (tests in this module, release build)
//!
//! "ops" are priced in the unit each kernel reports: 64-bit word XORs for
//! the `F_2` kernel (plain and trivial block), field multiply-adds for the
//! character blocks.  The last column prices one field multiply-add at one
//! word op (the prototype's table multiply) and, in brackets, at `d` word
//! ops (bit-serial multiply); the first is a lower bound on the
//! equivariant cost, the second is conservative.
//!
//! | system | n | degree | plain rows × cols | blocks | max block rows × cols | plain ops | equiv ops, 1× (d×) |
//! |:--|--:|--:|:--|:--|:--|--:|:--|
//! | Koblitz S3 barrel, `l = 3` | 11 | 3 | 17281 × 34606 | trivial + χ₁(d=10) | 1631 × 3166 | 164,522,573 | 1,246,949 + 475,613 = 1,722,562 (6,003,079) |
//! | random cyclic-invariant, 41 vars | 13 | 3 | 2542 × 9624 | trivial + χ₁(d=12) | 202 × 744 | 7,266,633 | 39,320 + 125,815 = 165,135 (1,549,100) |
//!
//! The Koblitz barrel needs `5n + 2l` variables, so with 64-bit masks it is
//! reachable only up to `n = 11`; `n = 13` there (71 variables) needs the
//! wide path of `examples/g2_wide_probe.rs`.  These are counted operations
//! on one instance each, not a wall-clock claim and not an `S`.

use std::collections::HashMap;

use crate::cryptanalysis::gf2_elim;
use crate::cryptanalysis::koblitz_groebner::{matrix_f4_f2_counted, rref_f2_counted};
use crate::cryptanalysis::pq_groebner_f2::{F2BoolMono, F2BoolPoly};

// ── Errors ────────────────────────────────────────────────────────────

/// Why an equivariant reduction was refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EquivariantError {
    /// The permutation is not a permutation of the variables, has too many
    /// variables, or its order is not 1 or the stated odd prime `n`.
    BadAction(String),
    /// The polynomial set is not closed under the action; `poly` indexes the
    /// first (deduplicated, nonzero) polynomial whose image is not in the set.
    NotInvariant { poly: usize },
    /// `ord_n(2)` exceeds the 40-bit field the module implements.
    FieldTooLarge { d: u32 },
}

// ── The action ────────────────────────────────────────────────────────

/// A cyclic group `⟨σ⟩ ≅ Z/n` acting on the Boolean variables by a
/// permutation `σ` of order dividing `n` (`n = 1`, or an odd prime so that
/// `F_2[G]` is semisimple and every orbit has size 1 or `n`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CyclicAction {
    n: usize,
    perm: Vec<usize>,
}

fn is_odd_prime(n: usize) -> bool {
    n >= 3
        && n % 2 == 1
        && (3..)
            .step_by(2)
            .take_while(|p| p * p <= n)
            .all(|p| !n.is_multiple_of(p))
}

impl CyclicAction {
    /// The trivial action (`n = 1`): one block, the plain matrix.
    pub fn trivial(n_vars: usize) -> Self {
        CyclicAction {
            n: 1,
            perm: (0..n_vars).collect(),
        }
    }

    /// `σ(v_i) = v_{perm[i]}`, a group of order `n`.
    pub fn new(n: usize, perm: Vec<usize>) -> Result<Self, EquivariantError> {
        let nv = perm.len();
        if nv > 64 {
            return Err(EquivariantError::BadAction("more than 64 variables".into()));
        }
        if n != 1 && !is_odd_prime(n) {
            return Err(EquivariantError::BadAction(format!(
                "order {n} is neither 1 nor an odd prime"
            )));
        }
        let mut seen = vec![false; nv];
        for &p in &perm {
            if p >= nv || std::mem::replace(&mut seen[p], true) {
                return Err(EquivariantError::BadAction("not a permutation".into()));
            }
        }
        // σ^n = id (and, n prime, the order is then 1 or n).
        for i in 0..nv {
            let mut x = i;
            for _ in 0..n {
                x = perm[x];
            }
            if x != i {
                return Err(EquivariantError::BadAction(format!(
                    "permutation does not have order dividing {n}"
                )));
            }
        }
        Ok(CyclicAction { n, perm })
    }

    /// The prototype's action: each block `[base, base + n)` of variables is
    /// rotated by one, `v_{base+k} ↦ v_{base+(k+1) mod n}`; all other
    /// variables are fixed.
    pub fn rotate_blocks(
        n_vars: usize,
        n: usize,
        bases: &[usize],
    ) -> Result<Self, EquivariantError> {
        let mut perm: Vec<usize> = (0..n_vars).collect();
        for &b in bases {
            if b + n > n_vars {
                return Err(EquivariantError::BadAction(
                    "block exceeds variables".into(),
                ));
            }
            for k in 0..n {
                perm[b + k] = b + (k + 1) % n;
            }
        }
        Self::new(n, perm)
    }

    /// The group order `n`.
    pub fn order(&self) -> usize {
        self.n
    }

    /// `σ` on a monomial mask.
    #[inline]
    pub fn apply_mask(&self, mut m: u64) -> u64 {
        let mut out = 0u64;
        while m != 0 {
            out |= 1u64 << self.perm[m.trailing_zeros() as usize];
            m &= m - 1;
        }
        out
    }

    /// `σ` on a polynomial.
    pub fn apply_poly(&self, p: &F2BoolPoly) -> F2BoolPoly {
        F2BoolPoly::from_monos(
            p.terms
                .iter()
                .map(|t| F2BoolMono::from_mask(self.apply_mask(t.mask)))
                .collect(),
            p.n_vars,
        )
    }
}

fn poly_key(p: &F2BoolPoly) -> Vec<u64> {
    let mut k: Vec<u64> = p.terms.iter().map(|t| t.mask).collect();
    k.sort_unstable();
    k
}

/// Drop zero polynomials and repeats (a repeat is a repeated Macaulay row
/// family; keeping one copy does not change the row space).
fn normalise(polys: &[F2BoolPoly]) -> Vec<F2BoolPoly> {
    let mut seen: HashMap<Vec<u64>, ()> = HashMap::new();
    let mut out = Vec::new();
    for p in polys {
        if p.terms.is_empty() {
            continue;
        }
        if seen.insert(poly_key(p), ()).is_none() {
            out.push(p.clone());
        }
    }
    out
}

/// Index of `σ(f_i)` for each `i`, or the first `i` whose image is absent.
fn poly_images(action: &CyclicAction, polys: &[F2BoolPoly]) -> Result<Vec<usize>, usize> {
    let index: HashMap<Vec<u64>, usize> = polys
        .iter()
        .enumerate()
        .map(|(i, p)| (poly_key(p), i))
        .collect();
    polys
        .iter()
        .enumerate()
        .map(|(i, p)| {
            index
                .get(&poly_key(&action.apply_poly(p)))
                .copied()
                .ok_or(i)
        })
        .collect()
}

/// Whether the (deduplicated, nonzero) polynomial set is closed under `σ`.
pub fn is_invariant(action: &CyclicAction, polys: &[F2BoolPoly]) -> bool {
    poly_images(action, &normalise(polys)).is_ok()
}

// ── GF(2^d) ───────────────────────────────────────────────────────────

/// `F_{2^d}`, `d ≤ 40`, elements as bit-polynomials in a `u64`; log tables
/// for `d ≤ 16`, bit-serial multiplication above.
#[derive(Clone, Debug)]
struct Gf {
    d: u32,
    poly: u64,
    exp: Vec<u32>,
    log: Vec<u32>,
}

impl Gf {
    fn bare(d: u32, poly: u64) -> Self {
        Gf {
            d,
            poly,
            exp: Vec::new(),
            log: Vec::new(),
        }
    }

    fn new(d: u32) -> Self {
        for low in (1u64..(1u64 << d)).step_by(2) {
            let f = (1u64 << d) | low;
            if Self::irreducible(f, d) {
                let mut g = Gf::bare(d, f);
                if d <= 16 {
                    g.build_tables();
                }
                return g;
            }
        }
        unreachable!("an irreducible polynomial of every degree exists");
    }

    fn irreducible(f: u64, d: u32) -> bool {
        let g = Gf::bare(d, f);
        let mut x = 2u64;
        for _ in 1..=(d / 2) {
            x = g.mul_slow(x, x);
            if Self::poly_gcd(x ^ 2, f) != 1 {
                return false;
            }
        }
        true
    }

    fn poly_gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            if a == 0 {
                return b;
            }
            let (da, db) = (63 - a.leading_zeros(), 63 - b.leading_zeros());
            if da < db {
                std::mem::swap(&mut a, &mut b);
                continue;
            }
            a ^= b << (da - db);
        }
        a
    }

    fn build_tables(&mut self) {
        let order = (1u64 << self.d) - 1;
        let mut fs = Vec::new();
        let (mut m, mut q) = (order, 2u64);
        while q * q <= m {
            if m % q == 0 {
                fs.push(q);
                while m % q == 0 {
                    m /= q;
                }
            }
            q += 1;
        }
        if m > 1 {
            fs.push(m);
        }
        let mut g = 2u64;
        while !fs.iter().all(|&f| self.pow_slow(g, order / f) != 1) {
            g += 1;
        }
        let size = 1usize << self.d;
        let mut exp = vec![0u32; 2 * size];
        let mut log = vec![0u32; size];
        let mut x = 1u64;
        for i in 0..order as usize {
            exp[i] = x as u32;
            log[x as usize] = i as u32;
            x = self.mul_slow(x, g);
        }
        for i in order as usize..2 * size {
            exp[i] = exp[i - order as usize];
        }
        self.exp = exp;
        self.log = log;
    }

    fn pow_slow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul_slow(r, a);
            }
            a = self.mul_slow(a, a);
            e >>= 1;
        }
        r
    }

    fn mul_slow(&self, mut a: u64, mut b: u64) -> u64 {
        let mut r = 0u64;
        while b != 0 {
            if b & 1 == 1 {
                r ^= a;
            }
            b >>= 1;
            a <<= 1;
            if (a >> self.d) & 1 == 1 {
                a ^= self.poly;
            }
        }
        r
    }

    #[inline]
    fn mul(&self, a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 {
            return 0;
        }
        if !self.exp.is_empty() {
            return self.exp[(self.log[a as usize] + self.log[b as usize]) as usize] as u64;
        }
        self.mul_slow(a, b)
    }

    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }

    fn inv(&self, a: u64) -> u64 {
        self.pow(a, (1u64 << self.d) - 2)
    }
}

/// The splitting field of `x^n − 1`: `K`, a primitive `n`-th root `ζ`, its
/// powers and the trace functional.
#[derive(Clone, Debug)]
struct CharField {
    gf: Gf,
    d: u32,
    zeta_pows: Vec<u64>,
    /// `Tr(x) = parity(x & tr_mask)`.
    tr_mask: u64,
    /// One representative `j` per Galois orbit of nonzero characters.
    js: Vec<usize>,
}

impl CharField {
    fn new(n: usize) -> Result<Self, EquivariantError> {
        let mut d = 1u32;
        let mut v = 2usize % n;
        while v != 1 {
            v = v * 2 % n;
            d += 1;
        }
        if d > 40 {
            return Err(EquivariantError::FieldTooLarge { d });
        }
        let gf = Gf::new(d);
        let order = (1u64 << d) - 1;
        let zeta = (2u64..)
            .map(|g| gf.pow(g, order / n as u64))
            .find(|&z| z != 1)
            .expect("n-th root of unity");
        let zeta_pows: Vec<u64> = (0..n).map(|t| gf.pow(zeta, t as u64)).collect();
        let mut tr_mask = 0u64;
        for b in 0..d {
            let (mut x, mut t) = (1u64 << b, 0u64);
            for _ in 0..d {
                t ^= x;
                x = gf.mul(x, x);
            }
            debug_assert!(t <= 1);
            tr_mask |= t << b;
        }
        let mut seen = vec![false; n];
        let mut js = Vec::new();
        for j in 1..n {
            if seen[j] {
                continue;
            }
            js.push(j);
            let mut k = j;
            while !seen[k] {
                seen[k] = true;
                k = 2 * k % n;
            }
        }
        Ok(CharField {
            gf,
            d,
            zeta_pows,
            tr_mask,
            js,
        })
    }

    #[inline]
    fn trace(&self, x: u64) -> bool {
        (x & self.tr_mask).count_ones() & 1 == 1
    }
}

// ── Orbit decomposition ───────────────────────────────────────────────

/// Which isotypic component a block is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockKind {
    /// Column orbit sums over `F_2`.
    Trivial,
    /// Character `χ_j` (and its Galois conjugates, `size = d` of them) over
    /// `F_{2^d}`.
    Character { j: usize, size: usize },
}

/// Shape of one block's input matrix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockShape {
    pub kind: BlockKind,
    pub rows: usize,
    pub cols: usize,
}

/// The orbit decomposition of the degree-`D` Macaulay matrix of an
/// invariant system: representatives, column orbits, and (on demand) the
/// block matrices.
#[derive(Clone, Debug)]
pub struct Decomposition {
    pub action: CyclicAction,
    pub degree: u32,
    pub n_vars: usize,
    /// Rows of the plain Macaulay matrix (nonzero products).
    pub plain_rows: usize,
    /// Columns (distinct monomials) of the plain Macaulay matrix.
    pub plain_cols: usize,
    /// Row-orbit representatives `(multiplier mask, polynomial index)`.
    pub row_reps: Vec<(u64, usize)>,
    /// The deduplicated input polynomials the indices refer to.
    pub polys: Vec<F2BoolPoly>,
    /// Canonical (minimal) mask of every column orbit and whether it is fixed.
    pub col_orbits: Vec<(u64, bool)>,
    rep_cols: Vec<Vec<u64>>,
    rep_fixed: Vec<bool>,
    /// column mask → (orbit index, `t` with `σ^t(canon) = column`).
    col_lookup: HashMap<u64, (u32, u32)>,
    /// orbit index → position among the free orbits.
    free_pos: Vec<Option<u32>>,
    free_orbits: Vec<u32>,
    field: Option<CharField>,
}

fn monomials_up_to(nv: usize, d: u32, out: &mut Vec<u64>) {
    fn rec(start: usize, nv: usize, left: u32, cur: u64, out: &mut Vec<u64>) {
        out.push(cur);
        if left == 0 {
            return;
        }
        for v in start..nv {
            rec(v + 1, nv, left - 1, cur | (1u64 << v), out);
        }
    }
    rec(0, nv, d, 0, out);
}

/// Columns of the row `m·p` with Boolean-ring parity cancellation.
fn row_columns(p: &F2BoolPoly, m: u64) -> Vec<u64> {
    let mut all: Vec<u64> = p.terms.iter().map(|t| t.mask | m).collect();
    all.sort_unstable();
    let mut out = Vec::new();
    let mut i = 0;
    while i < all.len() {
        let mut j = i;
        while j < all.len() && all[j] == all[i] {
            j += 1;
        }
        if (j - i) % 2 == 1 {
            out.push(all[i]);
        }
        i = j;
    }
    out
}

/// Map the input polynomials and the monomial set of the degree-`degree`
/// Macaulay matrix into the trivial block and the character blocks.
pub fn orbit_decompose(
    polys: &[F2BoolPoly],
    action: &CyclicAction,
    degree: u32,
) -> Result<Decomposition, EquivariantError> {
    let polys = normalise(polys);
    let n_vars = polys.first().map_or(action.perm.len(), |p| p.n_vars);
    let images =
        poly_images(action, &polys).map_err(|poly| EquivariantError::NotInvariant { poly })?;
    let n = action.n;
    let field = if n > 1 {
        Some(CharField::new(n)?)
    } else {
        None
    };

    let mut row_reps = Vec::new();
    let mut rep_cols = Vec::new();
    let mut rep_fixed = Vec::new();
    let mut plain_rows = 0usize;
    for (ei, p) in polys.iter().enumerate() {
        let pdeg = p
            .terms
            .iter()
            .map(|t| t.mask.count_ones())
            .max()
            .unwrap_or(0);
        if pdeg > degree {
            continue;
        }
        let mut mults = Vec::new();
        monomials_up_to(n_vars, degree - pdeg, &mut mults);
        for m in mults {
            let cols = row_columns(p, m);
            if cols.is_empty() {
                continue;
            }
            let (m1, e1) = (action.apply_mask(m), images[ei]);
            let fixed = (m1, e1) == (m, ei);
            if !fixed {
                // canonical iff (m, ei) is the minimum of its orbit
                let (mut mm, mut ee) = (m1, e1);
                let mut canonical = true;
                for _ in 1..n {
                    if (mm, ee) < (m, ei) {
                        canonical = false;
                        break;
                    }
                    mm = action.apply_mask(mm);
                    ee = images[ee];
                }
                if !canonical {
                    continue;
                }
            }
            plain_rows += if fixed { 1 } else { n };
            row_reps.push((m, ei));
            rep_cols.push(cols);
            rep_fixed.push(fixed);
        }
    }

    // Column orbits.
    let mut col_orbits: Vec<(u64, bool)> = Vec::new();
    let mut col_lookup: HashMap<u64, (u32, u32)> = HashMap::new();
    for cs in &rep_cols {
        for &c in cs {
            if col_lookup.contains_key(&c) {
                continue;
            }
            let mut members = vec![c];
            let mut x = action.apply_mask(c);
            while x != c {
                members.push(x);
                x = action.apply_mask(x);
            }
            let canon = *members.iter().min().expect("nonempty");
            let fixed = members.len() == 1;
            let idx = col_orbits.len() as u32;
            col_orbits.push((canon, fixed));
            let mut x = canon;
            for t in 0..members.len() {
                col_lookup.insert(x, (idx, t as u32));
                x = action.apply_mask(x);
            }
        }
    }
    let plain_cols = col_orbits
        .iter()
        .map(|&(_, fixed)| if fixed { 1 } else { n })
        .sum();
    let mut free_pos = vec![None; col_orbits.len()];
    let mut free_orbits = Vec::new();
    for (i, &(_, fixed)) in col_orbits.iter().enumerate() {
        if !fixed {
            free_pos[i] = Some(free_orbits.len() as u32);
            free_orbits.push(i as u32);
        }
    }
    Ok(Decomposition {
        action: action.clone(),
        degree,
        n_vars,
        plain_rows,
        plain_cols,
        row_reps,
        polys,
        col_orbits,
        rep_cols,
        rep_fixed,
        col_lookup,
        free_pos,
        free_orbits,
        field,
    })
}

impl Decomposition {
    /// Shapes of the blocks: the trivial block, then one per Galois orbit of
    /// nonzero characters (none for the trivial action).
    pub fn block_shapes(&self) -> Vec<BlockShape> {
        let mut v = vec![BlockShape {
            kind: BlockKind::Trivial,
            rows: self.row_reps.len(),
            cols: self.col_orbits.len(),
        }];
        if let Some(f) = &self.field {
            let rows = self.character_rows().len();
            for &j in &f.js {
                v.push(BlockShape {
                    kind: BlockKind::Character {
                        j,
                        size: f.d as usize,
                    },
                    rows,
                    cols: self.free_orbits.len(),
                });
            }
        }
        v
    }

    /// Representative rows with a nonzero nontrivial part.
    fn character_rows(&self) -> Vec<usize> {
        (0..self.row_reps.len())
            .filter(|&r| {
                !self.rep_fixed[r]
                    && self.rep_cols[r]
                        .iter()
                        .any(|c| !self.col_orbits[self.col_lookup[c].0 as usize].1)
            })
            .collect()
    }

    /// The trivial block: one packed `F_2` row per representative, entry at
    /// the column orbit = parity of the row over the orbit.
    pub fn trivial_matrix(&self) -> Vec<Vec<u64>> {
        let words = self.col_orbits.len().div_ceil(64).max(1);
        self.rep_cols
            .iter()
            .map(|cs| {
                let mut row = vec![0u64; words];
                for c in cs {
                    let k = self.col_lookup[c].0 as usize;
                    row[k / 64] ^= 1u64 << (k % 64);
                }
                row
            })
            .collect()
    }

    /// The character block `j`: `N_j[r̄][c̄] = Σ_t r̄[σ^t c̄] ζ^{-jt}` over the
    /// free column orbits, for the representatives with a nontrivial part.
    pub fn character_matrix(&self, j: usize) -> Vec<Vec<u64>> {
        let f = self.field.as_ref().expect("nontrivial action");
        let n = self.action.n;
        // ζ^{-jt} = ζ^{(n - j t mod n) mod n}
        let w: Vec<u64> = (0..n).map(|t| f.zeta_pows[(n - (j * t) % n) % n]).collect();
        self.character_rows()
            .into_iter()
            .map(|r| {
                let mut row = vec![0u64; self.free_orbits.len()];
                for c in &self.rep_cols[r] {
                    let (o, t) = self.col_lookup[c];
                    if let Some(k) = self.free_pos[o as usize] {
                        row[k as usize] ^= w[t as usize];
                    }
                }
                row
            })
            .collect()
    }
}

// ── Blockwise reduction ───────────────────────────────────────────────

/// The unit a block's operation count is in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpUnit {
    /// 64-bit word XORs (the crate's `F_2` kernel).
    WordXor,
    /// Multiply-adds in `F_{2^d}` (zero entries skipped).
    FieldMulAdd,
}

/// A block's reduced rows in block coordinates.
#[derive(Clone, Debug)]
pub enum BlockRows {
    /// Packed `F_2` rows over the column orbits (RREF).
    Bits(Vec<Vec<u64>>),
    /// `F_{2^d}` rows over the free column orbits (RREF) with their pivots.
    Field {
        rows: Vec<Vec<u64>>,
        pivots: Vec<usize>,
    },
}

/// One reduced block with its counts.
#[derive(Clone, Debug)]
pub struct BlockResult {
    pub kind: BlockKind,
    pub rows_in: usize,
    pub cols: usize,
    /// Rank over the block's field.
    pub rank: usize,
    /// Row-operation count in `unit`.
    pub ops: u64,
    pub unit: OpUnit,
    pub reduced: BlockRows,
}

/// RREF over `F_{2^d}`, counting multiply-adds (the prototype's count).
fn rref_gf_counted(
    gf: &Gf,
    mut m: Vec<Vec<u64>>,
    cols: usize,
    ops: &mut u64,
) -> (Vec<Vec<u64>>, Vec<usize>) {
    let mut rank = 0;
    let mut piv = Vec::new();
    for c in 0..cols {
        if rank == m.len() {
            break;
        }
        let Some(p) = (rank..m.len()).find(|&r| m[r][c] != 0) else {
            continue;
        };
        m.swap(rank, p);
        let inv = gf.inv(m[rank][c]);
        for v in m[rank].iter_mut() {
            if *v != 0 {
                *v = gf.mul(*v, inv);
                *ops += 1;
            }
        }
        let pivot = m[rank].clone();
        let nz: Vec<usize> = (c..cols).filter(|&k| pivot[k] != 0).collect();
        for r in 0..m.len() {
            if r != rank && m[r][c] != 0 {
                let f = m[r][c];
                for &k in &nz {
                    m[r][k] ^= gf.mul(f, pivot[k]);
                }
                *ops += nz.len() as u64;
            }
        }
        piv.push(c);
        rank += 1;
    }
    m.truncate(rank);
    (m, piv)
}

/// Output of [`equivariant_f4`].
#[derive(Clone, Debug)]
pub struct EquivariantF4 {
    pub decomposition: Decomposition,
    /// Trivial block first, then the character blocks.
    pub blocks: Vec<BlockResult>,
}

impl EquivariantF4 {
    /// `rank_triv + Σ_j d·rank_j`: the `F_2` dimension of the row space.
    pub fn total_rank(&self) -> usize {
        self.blocks
            .iter()
            .map(|b| match b.kind {
                BlockKind::Trivial => b.rank,
                BlockKind::Character { size, .. } => size * b.rank,
            })
            .sum()
    }

    /// Word XORs over the `F_2` blocks.
    pub fn word_ops(&self) -> u64 {
        self.sum_ops(OpUnit::WordXor)
    }

    /// Field multiply-adds over the character blocks.
    pub fn field_ops(&self) -> u64 {
        self.sum_ops(OpUnit::FieldMulAdd)
    }

    fn sum_ops(&self, unit: OpUnit) -> u64 {
        self.blocks
            .iter()
            .filter(|b| b.unit == unit)
            .map(|b| b.ops)
            .sum()
    }

    /// Total in word-op equivalents, one field multiply-add priced at
    /// `field_op_cost` word ops.
    pub fn priced_ops(&self, field_op_cost: u64) -> u64 {
        self.word_ops() + field_op_cost * self.field_ops()
    }

    /// The largest block input (rows, columns), by rows × columns.
    pub fn max_block_shape(&self) -> (usize, usize) {
        self.blocks
            .iter()
            .map(|b| (b.rows_in, b.cols))
            .max_by_key(|&(r, c)| r * c)
            .unwrap_or((0, 0))
    }

    /// Map the block results back to `F_2` Boolean-ring rows.
    pub fn inverse_dft(&self) -> Vec<F2BoolPoly> {
        inverse_dft(&self.decomposition, &self.blocks)
    }
}

/// Run the equivariant F4 step at `degree`: decompose, reduce every block
/// alone (the trivial block with the crate's `F_2` kernel, the character
/// blocks over `F_{2^d}`), and return the blockwise reduced rows with the
/// per-block operation counts.
pub fn equivariant_f4(
    polys: &[F2BoolPoly],
    action: &CyclicAction,
    degree: u32,
) -> Result<EquivariantF4, EquivariantError> {
    let dec = orbit_decompose(polys, action, degree)?;
    let mut blocks = Vec::new();

    let mut m = dec.trivial_matrix();
    let cols = dec.col_orbits.len();
    let rows_in = m.len();
    let mut ops = 0u64;
    let rank = rref_f2_counted(&mut m, cols, &mut ops);
    m.truncate(rank);
    blocks.push(BlockResult {
        kind: BlockKind::Trivial,
        rows_in,
        cols,
        rank,
        ops,
        unit: OpUnit::WordXor,
        reduced: BlockRows::Bits(m),
    });

    if let Some(f) = &dec.field {
        for &j in &f.js {
            let m = dec.character_matrix(j);
            let rows_in = m.len();
            let cols = dec.free_orbits.len();
            let mut ops = 0u64;
            let (rows, pivots) = rref_gf_counted(&f.gf, m, cols, &mut ops);
            blocks.push(BlockResult {
                kind: BlockKind::Character {
                    j,
                    size: f.d as usize,
                },
                rows_in,
                cols,
                rank: rows.len(),
                ops,
                unit: OpUnit::FieldMulAdd,
                reduced: BlockRows::Field { rows, pivots },
            });
        }
    }
    Ok(EquivariantF4 {
        decomposition: dec,
        blocks,
    })
}

/// As [`equivariant_f4`], but a system that is not invariant under `action`
/// is reduced with the trivial action (one block, the plain matrix) instead
/// of being refused.  The flag is `true` when the fallback was used.
pub fn equivariant_f4_or_trivial(
    polys: &[F2BoolPoly],
    action: &CyclicAction,
    degree: u32,
) -> Result<(EquivariantF4, bool), EquivariantError> {
    match equivariant_f4(polys, action, degree) {
        Err(EquivariantError::NotInvariant { .. }) => {
            let nv = polys.first().map_or(action.perm.len(), |p| p.n_vars);
            equivariant_f4(polys, &CyclicAction::trivial(nv), degree).map(|r| (r, true))
        }
        other => other.map(|r| (r, false)),
    }
}

// ── Inverse transform ─────────────────────────────────────────────────

fn orbit_members(dec: &Decomposition, canon: u64, fixed: bool) -> Vec<u64> {
    let n = if fixed { 1 } else { dec.action.n };
    let mut v = Vec::with_capacity(n);
    let mut x = canon;
    for _ in 0..n {
        v.push(x);
        x = dec.action.apply_mask(x);
    }
    v
}

/// The inverse transform from block coordinates to ordinary `F_2` rows.
///
/// Trivial rows become orbit-sum polynomials; every character-block row
/// becomes `d` rows `v_λ[σ^t c̄] = Tr(λ·w_{c̄}·ζ^{jt})`, `λ` over the
/// standard `F_2`-basis of `F_{2^d}`.  The result is an `F_2` basis of the
/// row space of the Macaulay matrix (trivial rows first).
pub fn inverse_dft(dec: &Decomposition, blocks: &[BlockResult]) -> Vec<F2BoolPoly> {
    let mut out = Vec::new();
    for b in blocks {
        match (&b.kind, &b.reduced) {
            (BlockKind::Trivial, BlockRows::Bits(rows)) => {
                for row in rows {
                    let mut monos = Vec::new();
                    for (wi, &word) in row.iter().enumerate() {
                        let mut x = word;
                        while x != 0 {
                            let k = wi * 64 + x.trailing_zeros() as usize;
                            x &= x - 1;
                            let (canon, fixed) = dec.col_orbits[k];
                            monos.extend(
                                orbit_members(dec, canon, fixed)
                                    .into_iter()
                                    .map(F2BoolMono::from_mask),
                            );
                        }
                    }
                    out.push(F2BoolPoly::from_monos(monos, dec.n_vars));
                }
            }
            (BlockKind::Character { j, .. }, BlockRows::Field { rows, .. }) => {
                let f = dec.field.as_ref().expect("character block needs a field");
                let n = dec.action.n;
                for row in rows {
                    for bit in 0..f.d {
                        let lambda = 1u64 << bit;
                        let mut monos = Vec::new();
                        for (k, &wk) in row.iter().enumerate() {
                            if wk == 0 {
                                continue;
                            }
                            let lw = f.gf.mul(lambda, wk);
                            let canon = dec.col_orbits[dec.free_orbits[k] as usize].0;
                            for (t, mm) in orbit_members(dec, canon, false).into_iter().enumerate()
                            {
                                if f.trace(f.gf.mul(lw, f.zeta_pows[(j * t) % n])) {
                                    monos.push(F2BoolMono::from_mask(mm));
                                }
                            }
                        }
                        out.push(F2BoolPoly::from_monos(monos, dec.n_vars));
                    }
                }
            }
            _ => unreachable!("block kind and rows always agree"),
        }
    }
    out
}

// ── Row-space agreement ───────────────────────────────────────────────

/// Canonical RREF of a row set over a fixed column order.
fn canonical_rref(
    rows: &[F2BoolPoly],
    col_index: &HashMap<u64, usize>,
    n_cols: usize,
) -> Vec<Vec<u64>> {
    let words = n_cols.div_ceil(64).max(1);
    let mut m: Vec<Vec<u64>> = rows
        .iter()
        .map(|p| {
            let mut r = vec![0u64; words];
            for t in &p.terms {
                let c = col_index[&t.mask];
                r[c / 64] ^= 1u64 << (c % 64);
            }
            r
        })
        .collect();
    let rank = gf2_elim::rref(&mut m, n_cols);
    m.truncate(rank);
    m
}

/// Whether two row sets span the same `F_2` space: their reduced row
/// echelon forms over one common column order (descending mask) coincide.
pub fn row_space_agrees(a: &[F2BoolPoly], b: &[F2BoolPoly]) -> bool {
    let mut masks: Vec<u64> = a
        .iter()
        .chain(b.iter())
        .flat_map(|p| p.terms.iter().map(|t| t.mask))
        .collect();
    masks.sort_unstable_by(|x, y| y.cmp(x));
    masks.dedup();
    let col_index: HashMap<u64, usize> = masks.iter().enumerate().map(|(i, &m)| (m, i)).collect();
    canonical_rref(a, &col_index, masks.len()) == canonical_rref(b, &col_index, masks.len())
}

/// The plain kernel at the same degree: reduced rows and word XORs.
pub fn plain_f4(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
) -> Option<(Vec<F2BoolPoly>, u64)> {
    matrix_f4_f2_counted(polys, n_vars, degree)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::{F2mElement, IrreduciblePoly};
    use crate::cryptanalysis::koblitz_groebner::{sym_semaev_s3, FieldStructure, SymElement};
    use crate::cryptanalysis::koblitz_index_calculus::find_irreducible;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    // ── random cyclic-invariant Boolean systems ──────────────────────

    /// `blocks` rotated blocks of `n` variables plus `fixed` fixed ones;
    /// `gens` random quadratics closed under `σ`, an invariant linear form
    /// and an invariant quadratic.
    fn random_invariant(
        n: usize,
        blocks: usize,
        fixed: usize,
        gens: usize,
        seed: u64,
    ) -> (Vec<F2BoolPoly>, CyclicAction, usize) {
        let nv = blocks * n + fixed;
        let bases: Vec<usize> = (0..blocks).map(|b| b * n).collect();
        let act = CyclicAction::rotate_blocks(nv, n, &bases).unwrap();
        let mut rng = StdRng::seed_from_u64(seed);
        let mut polys = Vec::new();
        let orbit = |p: &F2BoolPoly| {
            let mut v = vec![p.clone()];
            let mut q = act.apply_poly(p);
            while poly_key(&q) != poly_key(p) {
                v.push(q.clone());
                q = act.apply_poly(&q);
            }
            v
        };
        for _ in 0..gens {
            let mut monos = Vec::new();
            for _ in 0..rng.gen_range(5..9) {
                let mut mask = 0u64;
                for _ in 0..rng.gen_range(0..3) {
                    mask |= 1u64 << rng.gen_range(0..nv);
                }
                monos.push(F2BoolMono::from_mask(mask));
            }
            let p = F2BoolPoly::from_monos(monos, nv);
            if !p.terms.is_empty() {
                polys.extend(orbit(&p));
            }
        }
        // invariant linear form: orbit sum of x_0 plus a fixed variable
        let mut lin: Vec<F2BoolMono> = (0..n).map(|k| F2BoolMono::var(k as u32)).collect();
        if fixed > 0 {
            lin.push(F2BoolMono::var((blocks * n) as u32));
        }
        polys.push(F2BoolPoly::from_monos(lin, nv));
        // invariant quadratic: orbit sum of x_0 x_1 (needs two blocks' worth)
        let q = F2BoolPoly::from_monos(vec![F2BoolMono::from_mask(0b11), F2BoolMono::var(1)], nv);
        let mut qs = F2BoolPoly::zero(nv);
        for o in orbit(&q) {
            qs = qs.add(&o);
        }
        polys.push(qs);
        (polys, act, nv)
    }

    fn assert_agrees(
        polys: &[F2BoolPoly],
        act: &CyclicAction,
        nv: usize,
        degree: u32,
    ) -> EquivariantF4 {
        let (plain, _) = plain_f4(polys, nv, degree).expect("plain kernel within limits");
        let eq = equivariant_f4(polys, act, degree).expect("invariant system");
        let back = eq.inverse_dft();
        assert_eq!(back.len(), eq.total_rank(), "inverse rows are a basis");
        assert_eq!(eq.total_rank(), plain.len(), "ranks agree");
        assert!(row_space_agrees(&plain, &back), "row spaces differ");
        eq
    }

    #[test]
    fn random_invariant_row_space_agrees_n7_n11_n13() {
        for (n, blocks, fixed, gens) in [(7, 3, 2, 2), (11, 3, 2, 2), (13, 3, 2, 2)] {
            for seed in 0..2u64 {
                let (polys, act, nv) = random_invariant(n, blocks, fixed, gens, 0xe4f4 + seed);
                assert!(is_invariant(&act, &polys));
                for degree in [2u32, 3] {
                    let eq = assert_agrees(&polys, &act, nv, degree);
                    let shapes = eq.decomposition.block_shapes();
                    eprintln!(
                        "random n={n} seed={seed} D={degree}: plain {}x{} rank {} blocks {} (triv {}, chi {:?})",
                        eq.decomposition.plain_rows,
                        eq.decomposition.plain_cols,
                        eq.total_rank(),
                        shapes.len(),
                        eq.blocks[0].rank,
                        eq.blocks[1..].iter().map(|b| b.rank).collect::<Vec<_>>()
                    );
                }
            }
        }
    }

    #[test]
    fn non_invariant_system_is_rejected_or_trivial() {
        let (mut polys, act, nv) = random_invariant(7, 3, 2, 2, 7);
        // break invariance with a generator whose orbit is absent
        polys.push(F2BoolPoly::from_monos(
            vec![
                F2BoolMono::from_mask(0b101),
                F2BoolMono::var(3),
                F2BoolMono::one(),
            ],
            nv,
        ));
        assert!(!is_invariant(&act, &polys));
        assert!(matches!(
            equivariant_f4(&polys, &act, 3),
            Err(EquivariantError::NotInvariant { .. })
        ));
        let (eq, fell_back) = equivariant_f4_or_trivial(&polys, &act, 3).unwrap();
        assert!(fell_back);
        assert_eq!(eq.blocks.len(), 1, "trivial action is one block");
        let (plain, _) = plain_f4(&polys, nv, 3).unwrap();
        assert_eq!(eq.total_rank(), plain.len());
        assert!(row_space_agrees(&plain, &eq.inverse_dft()));
        // bad actions are refused
        assert!(CyclicAction::new(9, (0..9).collect()).is_err());
        assert!(CyclicAction::new(7, vec![1, 0]).is_err());
    }

    #[test]
    fn row_space_agrees_detects_a_missing_row() {
        let (polys, act, nv) = random_invariant(7, 3, 2, 2, 3);
        let eq = assert_agrees(&polys, &act, nv, 3);
        let mut back = eq.inverse_dft();
        back.pop();
        let (plain, _) = plain_f4(&polys, nv, 3).unwrap();
        assert!(!row_space_agrees(&plain, &back));
    }

    #[test]
    fn counted_row_ops_at_n13() {
        let (polys, act, nv) = random_invariant(13, 3, 2, 3, 0x13);
        let (plain, plain_ops) = plain_f4(&polys, nv, 3).unwrap();
        let eq = assert_agrees(&polys, &act, nv, 3);
        let d = eq.decomposition.field.as_ref().unwrap().d as u64;
        eprintln!(
            "n=13 D=3 vars={nv} plain {}x{} rank {} word_ops {plain_ops}",
            eq.decomposition.plain_rows,
            eq.decomposition.plain_cols,
            plain.len()
        );
        for b in &eq.blocks {
            eprintln!(
                "  block {:?}: {}x{} rank {} ops {} ({:?})",
                b.kind, b.rows_in, b.cols, b.rank, b.ops, b.unit
            );
        }
        eprintln!(
            "  equivariant: word_ops {} + field_ops {} = {} (x1)  /  {} (x{d}); plain {plain_ops}",
            eq.word_ops(),
            eq.field_ops(),
            eq.priced_ops(1),
            eq.priced_ops(d)
        );
        assert!(
            eq.priced_ops(1) < plain_ops,
            "per-block ops sum below the plain kernel"
        );
    }

    // ── a real Weil-descended Koblitz S3 barrel system ───────────────

    fn bits_of(e: &F2mElement) -> u64 {
        e.raw_bits().first().copied().unwrap_or(0)
    }

    fn elem_from_bits(bits: u64, n: u32) -> F2mElement {
        let positions: Vec<u32> = (0..n).filter(|k| (bits >> k) & 1 == 1).collect();
        F2mElement::from_bit_positions(&positions, n)
    }

    fn rank_f2(vectors: &[u64]) -> usize {
        let mut rows = vectors.to_vec();
        let mut rank = 0;
        for bit in (0..64).rev() {
            if let Some(p) = (rank..rows.len()).find(|&r| (rows[r] >> bit) & 1 == 1) {
                rows.swap(rank, p);
                let pv = rows[rank];
                for r in 0..rows.len() {
                    if r != rank && (rows[r] >> bit) & 1 == 1 {
                        rows[r] ^= pv;
                    }
                }
                rank += 1;
            }
        }
        rank
    }

    /// A normal basis `{α^{2^j}}` (poly coordinates) and the inverse change
    /// of basis (row `j`: normal coordinate `j` as a form on poly coords).
    fn normal_basis(
        n: u32,
        irr: &IrreduciblePoly,
        rng: &mut StdRng,
    ) -> (Vec<F2mElement>, Vec<u64>) {
        loop {
            let alpha = elem_from_bits(rng.gen_range(1..(1u64 << n)), n);
            let conj: Vec<F2mElement> = (0..n).map(|j| alpha.square_k_times(j, irr)).collect();
            let cols: Vec<u64> = conj.iter().map(bits_of).collect();
            if rank_f2(&cols) as u32 != n {
                continue;
            }
            let nn = n as usize;
            let mut rows: Vec<(u64, u64)> = (0..nn)
                .map(|i| {
                    let mut p_row = 0u64;
                    for (j, &c) in cols.iter().enumerate() {
                        if (c >> i) & 1 == 1 {
                            p_row |= 1 << j;
                        }
                    }
                    (p_row, 1u64 << i)
                })
                .collect();
            for col in 0..nn {
                let piv = (col..nn).find(|&r| (rows[r].0 >> col) & 1 == 1).unwrap();
                rows.swap(col, piv);
                let pv = rows[col];
                for r in 0..nn {
                    if r != col && (rows[r].0 >> col) & 1 == 1 {
                        rows[r].0 ^= pv.0;
                        rows[r].1 ^= pv.1;
                    }
                }
            }
            return (conj, rows.iter().map(|r| r.1).collect());
        }
    }

    fn to_normal_coords(coords: &[F2BoolPoly], pinv: &[u64], n_vars: usize) -> Vec<F2BoolPoly> {
        pinv.iter()
            .map(|row| {
                let mut acc = F2BoolPoly::zero(n_vars);
                for (i, c) in coords.iter().enumerate() {
                    if (row >> i) & 1 == 1 {
                        acc = acc.add(c);
                    }
                }
                acc
            })
            .collect()
    }

    /// The symmetric (target-selector) barrel system of
    /// `examples/g2_equivariant_orbit_probe.rs` for `K_0`, window
    /// `⟨1, z, …, z^{l-1}⟩`, two summands, in a normal basis; returns the
    /// equations, the variable count and the block bases `s0 s1 s2 u1 u2`.
    fn barrel_system(n: u32, l: u32) -> (Vec<F2BoolPoly>, usize, [usize; 5]) {
        let (nn, ll) = (n as usize, l as usize);
        let (s0, s1, s2) = (0, nn, 2 * nn);
        let (c1, c2) = (3 * nn, 3 * nn + ll);
        let (u1o, u2o) = (3 * nn + 2 * ll, 4 * nn + 2 * ll);
        let nv = 5 * nn + 2 * ll;
        assert!(nv <= 64, "64-bit masks");
        let irr = find_irreducible(n).expect("irreducible");
        let st = FieldStructure::new(n, &irr);
        let b = F2mElement::one(n);
        let mut rng = StdRng::seed_from_u64(0x62 + n as u64);
        let (normal, pinv) = normal_basis(n, &irr, &mut rng);
        let r = elem_from_bits(rng.gen_range(1..(1u64 << n)), n);
        let window: Vec<F2mElement> = (0..l)
            .map(|k| F2mElement::from_bit_positions(&[k], n))
            .collect();
        let s_elem = |var: usize| SymElement::from_subspace_vars(&[F2mElement::one(n)], var, n, nv);
        let u1 = SymElement::from_subspace_vars(&normal, u1o, n, nv);
        let u2 = SymElement::from_subspace_vars(&normal, u2o, n, nv);
        let barrel = |s_off: usize, c_off: usize| -> SymElement {
            let mut acc = SymElement::zero(n, nv);
            for k in 0..nn {
                let shifted: Vec<F2mElement> = window
                    .iter()
                    .map(|w| w.square_k_times(k as u32, &irr))
                    .collect();
                let y_k = SymElement::from_subspace_vars(&shifted, c_off, n, nv);
                acc = acc.add(&s_elem(s_off + k).mul(&y_k, &st));
            }
            acc
        };
        let mut x_r = SymElement::zero(n, nv);
        for k in 0..nn {
            let rk = SymElement::constant(&r.square_k_times(k as u32, &irr), n, nv);
            x_r = x_r.add(&s_elem(s0 + k).mul(&rk, &st));
        }
        let mut eqs = to_normal_coords(&sym_semaev_s3(&u1, &u2, &x_r, &b, &st), &pinv, nv);
        for (u, s_off, c_off) in [(&u1, s1, c1), (&u2, s2, c2)] {
            let diff = u.add(&barrel(s_off, c_off));
            eqs.extend(to_normal_coords(&diff.coords, &pinv, nv));
        }
        for off in [s1, s2, s0] {
            let mut monos: Vec<F2BoolMono> =
                (0..nn).map(|k| F2BoolMono::var((off + k) as u32)).collect();
            monos.push(F2BoolMono::one());
            eqs.push(F2BoolPoly::from_monos(monos, nv));
            for k in 0..nn {
                for k2 in (k + 1)..nn {
                    let m =
                        F2BoolMono::var((off + k) as u32).mul(F2BoolMono::var((off + k2) as u32));
                    eqs.push(F2BoolPoly::from_monos(vec![m], nv));
                }
            }
        }
        (eqs, nv, [s0, s1, s2, u1o, u2o])
    }

    fn koblitz_case(n: u32, degree: u32) -> (EquivariantF4, u64) {
        let (eqs, nv, bases) = barrel_system(n, 3);
        let act = CyclicAction::rotate_blocks(nv, n as usize, &bases).unwrap();
        assert!(is_invariant(&act, &eqs), "barrel system is sigma-invariant");
        let (plain, plain_ops) = plain_f4(&eqs, nv, degree).expect("within kernel limits");
        let eq = equivariant_f4(&eqs, &act, degree).unwrap();
        let back = eq.inverse_dft();
        assert_eq!(eq.total_rank(), plain.len());
        assert_eq!(back.len(), plain.len());
        assert!(row_space_agrees(&plain, &back));
        let dec = &eq.decomposition;
        let (mr, mc) = eq.max_block_shape();
        let d = dec.field.as_ref().unwrap().d as u64;
        eprintln!(
            "koblitz n={n} D={degree} vars={nv}: plain {}x{} rank {} | blocks {} (row reps {}, col orbits {}) max block {mr}x{mc} | plain word_ops {plain_ops} | equiv word_ops {} + field_ops {} => {} (x1) / {} (x{d})",
            dec.plain_rows,
            dec.plain_cols,
            plain.len(),
            eq.blocks.len(),
            dec.row_reps.len(),
            dec.col_orbits.len(),
            eq.word_ops(),
            eq.field_ops(),
            eq.priced_ops(1),
            eq.priced_ops(d)
        );
        (eq, plain_ops)
    }

    #[test]
    fn koblitz_s3_barrel_row_space_agrees_n7() {
        for degree in [2, 3] {
            koblitz_case(7, degree);
        }
    }

    #[test]
    fn koblitz_s3_barrel_row_space_agrees_n11() {
        let (eq, plain_ops) = koblitz_case(11, 3);
        assert!(eq.priced_ops(1) < plain_ops);
    }
}
