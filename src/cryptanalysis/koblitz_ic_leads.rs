//! Native builds of leads L1, L3 and L4 of
//! `research/koblitz_ic_new_leads_20261018/` (lead L2, partial enlargement,
//! is [`crate::cryptanalysis::partial_enlargement`]).  The measurement is
//! `research/koblitz_ic_leads_build_20261024/`.
//!
//! - [`lambda`]: the λ-coordinate factor-base closure.  A base
//!   `{P : λ(P) ∈ W}` with `W ∌ 1` is one sign of each `±` pair of
//!   `[2]⁻¹{P : x(P) ∈ L(W) + a}`, `L(w) = w² + w`, checked exhaustively.
//! - [`agreeing_gluing`]: the Raddum–Semaev Agreeing–Gluing algorithm on
//!   the chained `S₃` decomposition system, at block level (variables are
//!   the summands `xᵢ ∈ V` and the intermediate points `eⱼ ∈ F_{2^n}`),
//!   with every root solve and table probe counted, against a counted
//!   pairs-and-solve on the same chain.
//! - [`degree_bounds`]: the Caminata–Gorla Macaulay bound
//!   (Steiner, ToSC 2024(1), Corollary 2.9 and Theorem 3.2) for the
//!   Weil-descended chained systems, with the generic-coordinates
//!   hypothesis checked, beside the measured closure degrees.

/// λ-coordinate factor bases (lead L1).
pub mod lambda {
    use crate::binary_ecc::IrreduciblePoly;
    use crate::cryptanalysis::ic_boundary::ArtinSchreier;
    use crate::cryptanalysis::semaev_decomp::Gf2;
    use std::collections::{HashMap, HashSet};

    /// A finite affine point `(x, y)`; `None` is the point at infinity.
    pub type Pt = Option<(u64, u64)>;

    /// Affine group law on `y² + xy = x³ + a·x² + b` over `gf` (the
    /// formulas do not involve `b`).
    pub fn add(gf: &Gf2, a: u64, p: Pt, q: Pt) -> Pt {
        let (x1, y1) = match p {
            None => return q,
            Some(v) => v,
        };
        let (x2, y2) = match q {
            None => return p,
            Some(v) => v,
        };
        if x1 == x2 {
            if y1 ^ y2 == x1 || x1 == 0 {
                return None;
            }
            let s = x1 ^ gf.mul(y1, gf.inv(x1));
            let x3 = gf.sqr(s) ^ s ^ a;
            let y3 = gf.sqr(x1) ^ gf.mul(s ^ 1, x3);
            return Some((x3, y3));
        }
        let s = gf.mul(y1 ^ y2, gf.inv(x1 ^ x2));
        let x3 = gf.sqr(s) ^ s ^ x1 ^ x2 ^ a;
        let y3 = gf.mul(s, x1 ^ x3) ^ x3 ^ y1;
        Some((x3, y3))
    }

    /// `−P = (x, x + y)`.
    pub fn neg(p: Pt) -> Pt {
        p.map(|(x, y)| (x, x ^ y))
    }

    /// `λ(P) = x + y/x` (for `x ≠ 0`).
    pub fn lambda(gf: &Gf2, x: u64, y: u64) -> u64 {
        x ^ gf.mul(y, gf.inv(x))
    }

    /// Every affine point with `x ≠ 0` of `y² + xy = x³ + a·x² + b`, via
    /// `y = x·z`, `z² + z = x + a + b/x²`.
    pub fn points(gf: &Gf2, a: u64, b: u64) -> Vec<(u64, u64)> {
        let ash = ArtinSchreier::new(gf);
        let mut out = Vec::new();
        for x in 1..(1u64 << gf.n) {
            let c = x ^ a ^ gf.mul(b, gf.sqr(gf.inv(x)));
            if let Some(z) = ash.solve(c) {
                let y = gf.mul(x, z);
                out.push((x, y));
                out.push((x, y ^ x));
            }
        }
        out
    }

    /// The outcome of [`check`] on one curve.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct LambdaCheck {
        pub n: u32,
        pub a: u64,
        /// Affine points with `x ≠ 0`.
        pub points: usize,
        /// `x(2P) = λ² + λ + a` for every such point (against doubling as
        /// `(P + Q) + P − Q`, so not against its own formula).
        pub doubling_identity: bool,
        /// `λ(−P) = λ(P) + 1` for every such point.
        pub negation_identity: bool,
        /// Subspaces tried, and for every one: `B = A ⊔ (−A)` and `|W′| = |W|`.
        pub subspaces: usize,
        pub closure_identity: bool,
        pub base_sizes: Vec<usize>,
        /// The curve constant `b` (Koblitz curves: `1`).
        pub b: u64,
        /// Subspaces `W ∋ 1` tried, and for every one: `A = −A`,
        /// `|L(W)| = |W|/2` and `A = [2]⁻¹{P : x(P) ∈ L(W) + a}` exactly.
        pub subspaces_with_one: usize,
        pub closure_identity_with_one: bool,
    }

    fn span(basis: &[u64]) -> Vec<u64> {
        let mut v = vec![0u64];
        for &b in basis {
            let w: Vec<u64> = v.iter().map(|&x| x ^ b).collect();
            v.extend(w);
        }
        v
    }

    /// Check the closure on `y² + xy = x³ + a·x² + 1` over `F_2[t]/irr`
    /// for the subspaces spanned by each of `bases` (dependent ones are
    /// skipped and not counted).  [`check_curve`] with `b = 1`.
    pub fn check(irr: &IrreduciblePoly, a: u64, bases: &[Vec<u64>]) -> LambdaCheck {
        check_curve(irr, a, 1, bases)
    }

    /// [`check`] on `y² + xy = x³ + a·x² + b` for any `b ≠ 0`.  Subspaces
    /// avoiding `1` are checked against `B = A ⊔ (−A)`; subspaces holding
    /// `1` against `A = B`.
    pub fn check_curve(irr: &IrreduciblePoly, a: u64, b: u64, bases: &[Vec<u64>]) -> LambdaCheck {
        assert!(b != 0, "b = 0 is singular");
        let gf = Gf2::new(irr);
        let pts = points(&gf, a, b);
        let aux = Some(pts[pts.len() / 3]);
        let mut doubled: HashMap<(u64, u64), Option<u64>> = HashMap::with_capacity(pts.len());
        let (mut dbl, mut negok) = (true, true);
        for &(x, y) in &pts {
            let l = lambda(&gf, x, y);
            let p = Some((x, y));
            let d = add(&gf, a, add(&gf, a, add(&gf, a, p, aux), p), neg(aux));
            let dx = d.map(|v| v.0);
            dbl &= dx == Some(gf.sqr(l) ^ l ^ a);
            negok &= lambda(&gf, x, y ^ x) == l ^ 1;
            doubled.insert((x, y), dx);
        }
        let mut out = LambdaCheck {
            n: gf.n,
            a,
            points: pts.len(),
            doubling_identity: dbl,
            negation_identity: negok,
            subspaces: 0,
            closure_identity: true,
            base_sizes: Vec::new(),
            b,
            subspaces_with_one: 0,
            closure_identity_with_one: true,
        };
        for basis in bases {
            let mut w = span(basis);
            w.sort_unstable();
            w.dedup();
            if w.len() != 1 << basis.len() {
                continue;
            }
            let has_one = w.contains(&1);
            let wset: HashSet<u64> = w.iter().copied().collect();
            let wp: HashSet<u64> = w.iter().map(|&v| gf.sqr(v) ^ v ^ a).collect();
            let set_a: HashSet<(u64, u64)> = pts
                .iter()
                .copied()
                .filter(|&(x, y)| wset.contains(&lambda(&gf, x, y)))
                .collect();
            let set_b: HashSet<(u64, u64)> = pts
                .iter()
                .copied()
                .filter(|p| matches!(doubled[p], Some(v) if wp.contains(&v)))
                .collect();
            let neg_a: HashSet<(u64, u64)> = set_a.iter().map(|&(x, y)| (x, y ^ x)).collect();
            if has_one {
                let ok = wp.len() * 2 == w.len() && neg_a == set_a && set_a == set_b;
                out.closure_identity_with_one &= ok;
                out.subspaces_with_one += 1;
                out.base_sizes.push(set_a.len());
                continue;
            }
            let ok = wp.len() == w.len()
                && set_a.is_disjoint(&neg_a)
                && set_a.union(&neg_a).copied().collect::<HashSet<_>>() == set_b;
            out.closure_identity &= ok;
            out.subspaces += 1;
            out.base_sizes.push(set_a.len());
        }
        out
    }
}

/// Agreeing–Gluing on the chained `S₃` system (lead L3).
pub mod agreeing_gluing {
    use crate::binary_ecc::IrreduciblePoly;
    use crate::cryptanalysis::ic_boundary::ArtinSchreier;
    use crate::cryptanalysis::semaev_decomp::Gf2;
    use std::collections::{HashMap, HashSet};

    /// Counted outcome of one solve.
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct ChainCount {
        /// Ordered solutions `(x₁, …, x_m, e₁, …, e_{m−2})`, the count
        /// [`crate::cryptanalysis::koblitz_bench::chained_solution_count`]
        /// returns.
        pub solutions: u64,
        /// Quadratic solves `S₃(·, ·, d) = 0` for `d`.
        pub root_solves: u64,
        /// Membership tests against `V` or table lookups.
        pub probes: u64,
        /// Entries stored in the glued table (memory).
        pub table_entries: u64,
        /// Partial solutions the agreeing step discarded (their shared
        /// variable has no partner on the other side).
        pub discarded: u64,
    }

    struct Field<'a> {
        gf: Gf2,
        ash: ArtinSchreier,
        b: u64,
        _irr: &'a IrreduciblePoly,
    }

    impl<'a> Field<'a> {
        fn new(irr: &'a IrreduciblePoly, b: u64) -> Self {
            let gf = Gf2::new(irr);
            let ash = ArtinSchreier::new(&gf);
            Self {
                gf,
                ash,
                b,
                _irr: irr,
            }
        }

        /// Every `d` with `S₃(a, c, d) = (a+c)²d² + acd + (ac)² + b = 0`
        /// (distinct values, at most two).
        fn roots(&self, a: u64, c: u64, out: &mut Vec<u64>) {
            out.clear();
            let gf = &self.gf;
            let (lead, mid) = (gf.sqr(a ^ c), gf.mul(a, c));
            let tail = gf.sqr(mid) ^ self.b;
            if lead == 0 {
                if mid != 0 {
                    out.push(gf.mul(tail, gf.inv(mid)));
                }
                return;
            }
            if mid == 0 {
                out.push(gf.sqr_k(gf.mul(tail, gf.inv(lead)), gf.n - 1));
                return;
            }
            let scale = gf.mul(mid, gf.inv(lead));
            let rhs = gf.mul(gf.mul(tail, lead), gf.inv(gf.sqr(mid)));
            if let Some(z) = self.ash.solve(rhs) {
                out.push(gf.mul(scale, z));
                out.push(gf.mul(scale, z ^ 1));
            }
        }
    }

    fn span(basis: &[u64]) -> Vec<u64> {
        let mut v = vec![0u64];
        for &b in basis {
            let w: Vec<u64> = v.iter().map(|&x| x ^ b).collect();
            v.extend(w);
        }
        v
    }

    /// Pairs-and-solve on the chain: enumerate `x₁, …, x_{m−1}`, solve each
    /// link for its intermediate point, and close by solving the last link
    /// for `x_m` and testing membership of `V`.  `m ∈ {3, 4}`.
    pub fn pairs_and_solve(
        m: usize,
        basis: &[u64],
        x_r: u64,
        b: u64,
        irr: &IrreduciblePoly,
    ) -> ChainCount {
        assert!(m == 3 || m == 4, "m = 3 or 4");
        let f = Field::new(irr, b);
        let v = span(basis);
        let vset: HashSet<u64> = v.iter().copied().collect();
        let mut c = ChainCount::default();
        let (mut r1, mut r2, mut r3) = (Vec::new(), Vec::new(), Vec::new());
        for &x1 in &v {
            for &x2 in &v {
                f.roots(x1, x2, &mut r1);
                c.root_solves += 1;
                for &e1 in &r1 {
                    if m == 3 {
                        f.roots(e1, x_r, &mut r2);
                        c.root_solves += 1;
                        for &x3 in &r2 {
                            c.probes += 1;
                            c.solutions += vset.contains(&x3) as u64;
                        }
                    } else {
                        for &x3 in &v {
                            f.roots(e1, x3, &mut r2);
                            c.root_solves += 1;
                            for &e2 in &r2 {
                                f.roots(e2, x_r, &mut r3);
                                c.root_solves += 1;
                                for &x4 in &r3 {
                                    c.probes += 1;
                                    c.solutions += vset.contains(&x4) as u64;
                                }
                            }
                        }
                    }
                }
            }
        }
        c
    }

    /// Agreeing–Gluing at block level.  Each link is a list of local
    /// solutions on its block variables.  **Agreeing** discards a local
    /// solution of one link whose shared intermediate point no local
    /// solution of the neighbouring link takes; **gluing** joins two links
    /// on that point.  The order is the one Agreeing–Gluing's cost model
    /// prefers on a chain: glue from both ends onto the middle point.
    ///
    /// - `m = 3`: the right link `S₃(e₁, x₃, x_R)` lists `e₁` (one solve
    ///   per `x₃`); the left link `S₃(x₁, x₂, e₁)` is agreed onto that set
    ///   by solving it for `x₂` per `(e₁, x₁)` and testing `x₂ ∈ V`.
    /// - `m = 4`: the left link lists `e₁` per `(x₁, x₂)` and is stored as
    ///   a table `e₁ → multiplicity`; the right two links list `(e₂, x₄)`,
    ///   then `e₁` per `(e₂, x₃)`, each agreed against the table.
    pub fn agreeing_gluing(
        m: usize,
        basis: &[u64],
        x_r: u64,
        b: u64,
        irr: &IrreduciblePoly,
    ) -> ChainCount {
        assert!(m == 3 || m == 4, "m = 3 or 4");
        let f = Field::new(irr, b);
        let v = span(basis);
        let vset: HashSet<u64> = v.iter().copied().collect();
        let mut c = ChainCount::default();
        let (mut r1, mut r2) = (Vec::new(), Vec::new());
        if m == 3 {
            // Right link: e₁ with its multiplicity over x₃.
            let mut right: HashMap<u64, u64> = HashMap::new();
            for &x3 in &v {
                f.roots(x3, x_r, &mut r1);
                c.root_solves += 1;
                for &e1 in &r1 {
                    *right.entry(e1).or_insert(0) += 1;
                }
            }
            c.table_entries = right.len() as u64;
            for (&e1, &mult) in &right {
                let mut hits = 0u64;
                for &x1 in &v {
                    f.roots(x1, e1, &mut r2);
                    c.root_solves += 1;
                    for &x2 in &r2 {
                        c.probes += 1;
                        hits += vset.contains(&x2) as u64;
                    }
                }
                if hits == 0 {
                    c.discarded += mult;
                }
                c.solutions += hits * mult;
            }
            return c;
        }
        // m = 4.  Left link table e₁ → #(x₁, x₂).
        let mut left: HashMap<u64, u64> = HashMap::new();
        for &x1 in &v {
            for &x2 in &v {
                f.roots(x1, x2, &mut r1);
                c.root_solves += 1;
                for &e1 in &r1 {
                    *left.entry(e1).or_insert(0) += 1;
                }
            }
        }
        c.table_entries = left.len() as u64;
        // Right links: e₂ per x₄, then e₁ per (e₂, x₃), agreed on e₁.
        let mut e2s: Vec<u64> = Vec::new();
        for &x4 in &v {
            f.roots(x4, x_r, &mut r1);
            c.root_solves += 1;
            e2s.extend_from_slice(&r1);
        }
        for &e2 in &e2s {
            for &x3 in &v {
                f.roots(e2, x3, &mut r2);
                c.root_solves += 1;
                for &e1 in &r2 {
                    c.probes += 1;
                    match left.get(&e1) {
                        Some(&k) => c.solutions += k,
                        None => c.discarded += 1,
                    }
                }
            }
        }
        c
    }

    /// The other gluing order at `m = 4`, `(L₁ ⋈ L₂) ⋈ L₃`: list `e₁` per
    /// `(x₁, x₂)`, extend each by `e₂` per `x₃`, and agree on `e₂` against
    /// the right link's table `e₂ → #x₄`.  `L₁ ⋈ (L₂ ⋈ L₃)` is
    /// [`agreeing_gluing`]'s order, so with it every order of the
    /// three-link chain is measured.
    pub fn agreeing_gluing_left(
        basis: &[u64],
        x_r: u64,
        b: u64,
        irr: &IrreduciblePoly,
    ) -> ChainCount {
        let f = Field::new(irr, b);
        let v = span(basis);
        let mut c = ChainCount::default();
        let (mut r1, mut r2) = (Vec::new(), Vec::new());
        let mut right: HashMap<u64, u64> = HashMap::new();
        for &x4 in &v {
            f.roots(x4, x_r, &mut r1);
            c.root_solves += 1;
            for &e2 in &r1 {
                *right.entry(e2).or_insert(0) += 1;
            }
        }
        c.table_entries = right.len() as u64;
        for &x1 in &v {
            for &x2 in &v {
                f.roots(x1, x2, &mut r1);
                c.root_solves += 1;
                for &e1 in &r1 {
                    for &x3 in &v {
                        f.roots(e1, x3, &mut r2);
                        c.root_solves += 1;
                        for &e2 in &r2 {
                            c.probes += 1;
                            match right.get(&e2) {
                                Some(&k) => c.solutions += k,
                                None => c.discarded += 1,
                            }
                        }
                    }
                }
            }
        }
        c
    }
}

/// Solving-degree bounds for the descended chained systems (lead L4).
pub mod degree_bounds {
    use crate::cryptanalysis::pq_groebner_f2::F2BoolPoly;

    /// Caminata–Gorla bound for one system, as Steiner states it
    /// (ToSC 2024(1), arXiv 2310.03637v2, Corollary 2.9): if `(F^hom)` is
    /// in generic coordinates, `sd_DRL(F) ≤ reg(F^hom) ≤ d₁ + … + d_l − l + 1`
    /// with `d₁ ≥ d₂ ≥ …` and `l = min(N + 1, #F)`.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MacaulayBound {
        /// Boolean unknowns.
        pub n_vars: usize,
        /// Equations of the descended system (field equations excluded).
        pub equations: usize,
        /// Largest total degree among them, measured on the system.
        pub max_degree: u32,
        /// Generic coordinates, by Theorem 3.2 (4): a pure power of every
        /// variable lies in the initial ideal.  With the field equations
        /// `x² + x` adjoined, `x²` is the top component of a generator for
        /// every variable, so this holds whenever they are adjoined.
        pub generic_coordinates: bool,
        /// The Macaulay bound with the field equations adjoined.
        pub bound: u64,
    }

    /// The bound for `system` in `n_vars` unknowns, field equations adjoined.
    pub fn macaulay_bound(system: &[F2BoolPoly], n_vars: usize) -> MacaulayBound {
        let occurring = system
            .iter()
            .flat_map(|p| p.terms.iter())
            .fold(0u64, |acc, t| acc | t.mask);
        let mut degrees: Vec<u32> = system
            .iter()
            .map(|p| {
                p.terms
                    .iter()
                    .map(|t| t.mask.count_ones())
                    .max()
                    .unwrap_or(0)
            })
            .filter(|&d| d > 0)
            .collect();
        let max_degree = degrees.iter().copied().max().unwrap_or(0);
        // One field equation per variable, degree 2.
        degrees.extend(std::iter::repeat_n(2, n_vars));
        degrees.sort_unstable_by(|a, b| b.cmp(a));
        let l = (n_vars + 1).min(degrees.len());
        let bound = degrees[..l].iter().map(|&d| d as u64).sum::<u64>() - l as u64 + 1;
        // Every variable gets x² from its field equation, so Theorem 3.2 (4)
        // holds once the field equations are adjoined; a variable outside
        // `n_vars` would have none.
        let generic_coordinates = n_vars >= 64 || occurring >> n_vars == 0;
        MacaulayBound {
            n_vars,
            equations: system.len(),
            max_degree,
            generic_coordinates,
            bound,
        }
    }

    /// The field-level Macaulay bound of the chain over `F_{2^n}` before
    /// descent, `log₂` of it: unknowns `x₁ … x_m` and `e₁ … e_{m−2}`,
    /// `m − 1` links of total degree 4, `m` subspace equations
    /// `L_V(xᵢ) = 0` of degree `2^ℓ`, and `m − 2` field equations
    /// `e^{2^n} = e`.  Generic coordinates hold by the same pure-power
    /// test (`L_V` and the field equations give one per unknown).
    pub fn field_level_bound_log2(m: usize, n: u32, ell: u32) -> f64 {
        let vars = 2 * m - 2;
        let mut degrees: Vec<f64> = Vec::new();
        degrees.extend(std::iter::repeat_n(4.0, m - 1));
        degrees.extend(std::iter::repeat_n(2f64.powi(ell as i32), m));
        degrees.extend(std::iter::repeat_n(2f64.powi(n as i32), m - 2));
        degrees.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let l = (vars + 1).min(degrees.len());
        (degrees[..l].iter().sum::<f64>() - l as f64 + 1.0).log2()
    }

    /// Caminata–Ceria–Gorla (arXiv 2112.10506, Corollary 4.12), `log₂`:
    /// the descended system is `Weil(F′)` for the field-level chain `F′`
    /// (links, `L_V(xᵢ)`, field equations of `F_{2^n}`), so
    /// `sd ≤ n · reg(F′ʰ) − n + 1`, with `reg(F′ʰ)` taken at its Macaulay
    /// bound ([`field_level_bound_log2`]).  The hypothesis `t ∤ 0` modulo
    /// `F′ʰ` is not checked; the value is reported only to show the order.
    pub fn weil_restriction_bound_log2(m: usize, n: u32, ell: u32) -> f64 {
        let reg = 2f64.powf(field_level_bound_log2(m, n, ell));
        (n as f64 * reg - n as f64 + 1.0).log2()
    }

    /// `τ(r, c, t)` of Huang–Kosters–Yang–Yeo (arXiv 1505.02532,
    /// Theorem 1.1): `max(⌊2t(c − 1)(log_c(r/2t) + 1)⌋, 0)`.
    fn hkyy_tau(r: f64, c: f64, t: f64) -> f64 {
        (2.0 * t * (c - 1.0) * ((r / (2.0 * t)).ln() / c.ln() + 1.0))
            .floor()
            .max(0.0)
    }

    /// The HKYY last-fall-degree bound of the descended chain over `F₂`
    /// (arXiv 1505.02532, Theorem 1.1, `q = 2`), independent of `n`.  The
    /// field-level system has `2m − 2` unknowns and at most
    /// `s = 2^{mℓ + m − 2}` solutions (`2^ℓ` roots of each `L_V(xᵢ)`, at
    /// most two of each quadratic link in `eⱼ`).  Its last fall degree
    /// `d_F` is unmeasured; the bound is evaluated with `d_F ≤ (2m − 1)s`,
    /// where it no longer depends on `d_F`.  Radicality and the injective
    /// projection are not checked.
    pub fn last_fall_bound(m: usize, ell: u32) -> f64 {
        let t = (2 * m - 2) as f64;
        let s = 2f64.powi((m as u32 * ell + m as u32 - 2) as i32);
        let first = hkyy_tau(((t + 1.0) * s).max(2f64.powi(ell as i32)), 2.0, t);
        let second = t * hkyy_tau(2.0 * s, 2.0, 1.0);
        first.max(second).max(2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::F2mElement;
    use crate::cryptanalysis::koblitz_bench::{chained_solution_count, ladder_sample};
    use crate::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
    use crate::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    /// L1: the closure holds on both Koblitz curves over F_{2^11} and
    /// F_{2^13}, for subspaces of three dimensions each.
    #[test]
    fn lambda_bases_are_halved_affine_x_bases() {
        let mut rng = StdRng::seed_from_u64(20261024);
        for n in [11u32, 13] {
            let irr = find_irreducible_sparse(n).unwrap();
            let mask = (1u64 << n) - 1;
            for a in 0..2u64 {
                let bases: Vec<Vec<u64>> = [3usize, n as usize / 2, n as usize - 3]
                    .iter()
                    .cycle()
                    .take(9)
                    .map(|&d| (0..d).map(|_| rng.gen::<u64>() & mask).collect())
                    .collect();
                let r = lambda::check(&irr, a, &bases);
                assert!(r.doubling_identity, "n={n} a={a}");
                assert!(r.negation_identity, "n={n} a={a}");
                assert!(r.subspaces >= 6, "enough valid subspaces at n={n} a={a}");
                assert!(r.closure_identity, "n={n} a={a}");
            }
        }
    }

    /// L1, Amendment 3: subspaces holding `1` give `A = B` exactly, and the
    /// closure holds for `b ≠ 1` too.
    #[test]
    fn lambda_closure_with_one_and_any_b() {
        let mut rng = StdRng::seed_from_u64(20261025);
        let n = 11u32;
        let irr = find_irreducible_sparse(n).unwrap();
        let mask = (1u64 << n) - 1;
        for b in [1u64, 0x2a5, 0x713] {
            for a in 0..2u64 {
                let bases: Vec<Vec<u64>> = (0..8)
                    .map(|i| {
                        let mut v: Vec<u64> = (0..4).map(|_| rng.gen::<u64>() & mask).collect();
                        if i % 2 == 0 {
                            v[0] = 1;
                        }
                        v
                    })
                    .collect();
                let r = lambda::check_curve(&irr, a, b, &bases);
                assert!(r.doubling_identity && r.negation_identity, "b={b} a={a}");
                assert!(r.subspaces >= 2 && r.subspaces_with_one >= 3, "b={b} a={a}");
                assert!(
                    r.closure_identity && r.closure_identity_with_one,
                    "b={b} a={a}"
                );
            }
        }
    }

    /// L3: both counted solvers return the exact solution count.
    #[test]
    fn agreeing_gluing_counts_exactly() {
        let mut rng = StdRng::seed_from_u64(7);
        let mut nonzero = 0;
        for (m, n, ell) in [(3usize, 9u32, 3usize), (3, 10, 4), (4, 9, 3), (4, 8, 2)] {
            let irr = find_irreducible_sparse(n).unwrap();
            for _ in 0..6 {
                let (basis, x_r) = ladder_sample(n, ell, &mut rng);
                let words: Vec<u64> = basis
                    .iter()
                    .map(|e| e.raw_bits().first().copied().unwrap_or(0))
                    .collect();
                let xr = x_r.raw_bits().first().copied().unwrap_or(0);
                let exact = chained_solution_count(m, &basis, &x_r, &F2mElement::one(n), &irr);
                let ps = agreeing_gluing::pairs_and_solve(m, &words, xr, 1, &irr);
                let ag = agreeing_gluing::agreeing_gluing(m, &words, xr, 1, &irr);
                assert_eq!(ps.solutions, exact, "pairs m={m} n={n} ell={ell}");
                assert_eq!(ag.solutions, exact, "A-G m={m} n={n} ell={ell}");
                if m == 4 {
                    let left = agreeing_gluing::agreeing_gluing_left(&words, xr, 1, &irr);
                    assert_eq!(left.solutions, exact, "left order n={n} ell={ell}");
                }
                nonzero += (exact > 0) as u32;
            }
        }
        assert!(nonzero > 0, "the panel has satisfiable draws");
    }

    /// L4: the descended links have degree 3, generic coordinates hold,
    /// and the bound is `2N + 3` once the links outnumber `N + 1`.
    #[test]
    fn macaulay_bound_on_a_descended_chain() {
        let n = 12u32;
        let irr = find_irreducible_sparse(n).unwrap();
        let st = FieldStructure::new(n, &irr);
        let mut rng = StdRng::seed_from_u64(1);
        let (basis, x_r) = ladder_sample(n, 3, &mut rng);
        let sys = build_decomposition_system(&basis, &x_r, &F2mElement::one(n), 3, &st).unwrap();
        let b = degree_bounds::macaulay_bound(&sys.equations, sys.n_vars);
        assert_eq!(b.max_degree, 3);
        assert!(b.generic_coordinates);
        // N = n + 3ℓ = 21 unknowns.  The top N + 1 degrees are the cubic
        // links first, then degree 2 (quadratic links and field equations).
        assert_eq!(b.n_vars, 21);
        let cubic = sys
            .equations
            .iter()
            .filter(|p| p.terms.iter().map(|t| t.mask.count_ones()).max() == Some(3))
            .count() as u64;
        let l = 22u64;
        let top3 = cubic.min(l);
        assert_eq!(b.bound, 3 * top3 + 2 * (l - top3) - l + 1);
        assert!(b.bound >= 21 + 2 && b.bound <= 2 * 21 + 3);
        assert!(degree_bounds::field_level_bound_log2(4, 131, 33) > 131.0);
        assert!(degree_bounds::weil_restriction_bound_log2(4, 131, 33) > 138.0);
        // Independent of n, but far above the measured degree 4 at ℓ = 6.
        assert!(degree_bounds::last_fall_bound(3, 6) > 6.0 + 4.0);
    }
}
