//! **Gaudry's `n = 4` decomposition on an abscissa line `x ∈ u·F_q` of a
//! curve `y² + xy = x³ + b` over `F_{q⁴}`, `q = 2^m`** (the C1 end-to-end
//! study, `research/c1_end_to_end_solve_20261107/`).
//!
//! Two lines share every line of code here and differ only in `u`:
//!
//! * the **standard base** `u = 1`, `{P : x(P) ∈ F_q^*}`, about half of
//!   whose abscissae lift;
//! * the **twisted all-lift line** `u ∈ ker Tr_{q⁴/q}` with
//!   `Tr_{q⁴/q}(√b / u) = 0` (`research/lift_dense_factor_base_20261027`,
//!   `research/c1_twisted_line_ladder_20261027`), on which every
//!   `x ∈ u·F_q^*` lifts when `Tr(a) = 0`.
//!
//! Construction rules of `research/subgroup_trap_20261107` (binding for
//! the study and enforced here): `a = 0` (so `Tr(a) = 0`); `b ∉ F_{q²}`,
//! so `K = F_2(line, a, b) = F_{q⁴}` and `[K : F_2] = N = 4m` for every
//! line; `u ∉ b^{1/4}·F_q` (the 2-torsion-stable lines of
//! `lift_dense_torsion_pairing_20261106`) and `u ∉ F_q`.
//!
//! **The system.**  `S₅(x₁, …, x₄, x_R)` is symmetric in `x₁..x₄` and, in
//! characteristic 2, a polynomial in `e₁², e₂², e₃², e₄` of them (the
//! ladder's measured fact, checked again by this module's tests).  With
//! `x_i = u·y_i`, `y_i ∈ F_q`, and `E_j = e_j(y)²` (`j ≤ 3`), `E₄ = e₄(y)`,
//! a term `e₁^{i₁}e₂^{i₂}e₃^{i₃}e₄^{i₄} x_R^j b^l` becomes
//! `u^{i₁+2i₂+3i₃+4i₄} x_R^j b^l · E₁^{i₁/2}E₂^{i₂/2}E₃^{i₃/2}E₄^{i₄}`, and
//! the four `F_q`-coordinates of its coefficient give four equations over
//! `F_q` in `(E₁, E₂, E₃, E₄)`, weights `(2, 2, 2, 1)`: the shape of the
//! `gaudry-d4-bin` weighted system.  They are solved by the same counted
//! weighted Macaulay solve with the eigen step ([`solve_bin`]); each
//! rational solution is split through `X⁴ + e₁X³ + e₂X² + e₃X + e₄`, its
//! roots lifted and the signs checked against the target.
//!
//! **The charge** is `gaudry-d4-bin`'s: every `F_q` multiplication of the
//! line search, the `u`- and `b`-power precomputation, the Weil
//! restriction, the solve, the split and the lifts of a split is solver
//! work in `F_q multiplications`, priced at the curve's own
//! `fq_muls_per_add`; sign checks are group additions; the base's lifts
//! are `sqrt_solves` at the measured lift price.  The symbolic `S₅` over
//! `F_2[x₁..x₅, b]` is a universal formula (no curve, no target) and is
//! not charged; its term count is recorded.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;
use std::sync::OnceLock;

use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::cryptanalysis::gaudry_quartic_bin::{
    fq_muls_per_add, group_order_bin, roots_with, solve_bin, BinCurve, BinD4Solve, BinOptions,
    BinSolve, BinTrace, CurveB, Eb, Fe, Fq4, PtB, WEIGHTS,
};
use crate::cryptanalysis::ic_boundary::{CountedGroup, FactorBase, GroupOps, OracleCounters};
use crate::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params, SolverTotals, SystemShape,
};

/// The factor-base plug-in's name.
pub const BASE: &str = "bin-x-line";
/// The oracle plug-in's name.
pub const ORACLE: &str = "gaudry-x-bin";

// ── The curves: a = 0, b ∉ F_{q²} ────────────────────────────────────────

/// `y² + xy = x³ + b` over `F_{q⁴}`, `b` drawn from `seed` outside
/// `F_{q²}` (`a = 0`: `Tr(a) = 0`).
pub fn generate_instance_a0(m: u32, seed: u64) -> CurveB {
    let mut rng = StdRng::seed_from_u64(seed ^ 0xa0b1_d4c1);
    let f = Fq4::new(m);
    loop {
        let b = f.random(&mut rng);
        if b.is_zero() || f.in_fq2(&b) {
            continue;
        }
        return CurveB::new(f, Eb::ZERO, b);
    }
}

/// The first instance seed from `seed` whose `#E = h·r` has `r` prime,
/// `h ≤ max_cofactor`; deterministic in `(m, seed, max_cofactor)`.
/// `m ≤ 15` (one-word orders).
pub fn find_bin_curve_a0(m: u32, seed: u64, max_cofactor: u64) -> Result<BinCurve, String> {
    if !(4..=15).contains(&m) {
        return Err(format!("f2m4_a0 takes m from 4 to 15, not {m}"));
    }
    for s in seed..seed + 2_000 {
        let curve = generate_instance_a0(m, s);
        let mut rng = StdRng::seed_from_u64(s ^ 0x0c0f);
        let Some(order) = group_order_bin(&curve, &mut rng) else {
            continue;
        };
        let Ok(order64) = u64::try_from(order) else {
            continue;
        };
        let Some(h) = (2..=max_cofactor).find(|&h| {
            order64.is_multiple_of(h)
                && crate::cryptanalysis::residual_walk::is_prime_u64(order64 / h)
        }) else {
            continue;
        };
        let r = u128::from(order64 / h);
        let h = u128::from(h);
        let generator = loop {
            let x = curve.f.random(&mut rng);
            let Some(p) = curve.lift_x(&x, &mut rng) else {
                continue;
            };
            let g = curve.mul(&p, h);
            if !g.inf {
                break g;
            }
        };
        if !curve.mul(&generator, r).inf {
            continue;
        }
        return Ok(BinCurve {
            curve,
            instance_seed: s,
            group_order: order,
            r,
            cofactor: h,
            generator,
        });
    }
    Err(format!(
        "no a = 0 curve with cofactor ≤ {max_cofactor} over F_(2^{m})^4 from seed {seed}"
    ))
}

// ── The line ─────────────────────────────────────────────────────────────

/// Which line: `u = 1`, or a twisted all-lift line drawn from a seed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line {
    Standard,
    Twisted { seed: u64 },
}

impl Line {
    pub fn from_params(p: &Params) -> Result<Line, String> {
        match p.get("line") {
            Some("standard") => {
                if p.get("line_seed").is_some() {
                    return Err("bin-x-line: line_seed is for line=twisted only".into());
                }
                Ok(Line::Standard)
            }
            Some("twisted") => Ok(Line::Twisted {
                seed: p.u64("line_seed")?,
            }),
            Some(other) => Err(format!(
                "bin-x-line: line is standard or twisted, not `{other}`"
            )),
            None => Err("bin-x-line needs line=standard or line=twisted,line_seed=N".into()),
        }
    }
}

/// The chosen line and what choosing it cost.
#[derive(Clone, Copy, Debug)]
pub struct LineState {
    pub u: Eb,
    /// `F_q` multiplications of the search for `u` and of the scalings
    /// `y ↦ u·y` of the base build.
    pub fq_muls: u64,
    /// Candidates `z` drawn before `u` was accepted (0 for `u = 1`).
    pub draws: u64,
}

/// `Tr_{q⁴/q}(v)` (counted `F_q` multiplications through `frob`).
pub fn trace_q(f: &Fq4, v: &Eb) -> Eb {
    let v1 = f.frob(v);
    let v2 = f.frob(&v1);
    let v3 = f.frob(&v2);
    v.add(&v1).add(&v2).add(&v3)
}

/// Whether `v ∈ c·F_q` for a unit `c` (counted).
fn in_line(f: &Fq4, v: &Eb, c: &Eb) -> bool {
    f.mul(v, &f.inv(c)).in_fq()
}

/// The twisted line's `u`: `u = z^q + z` (so `Tr_{q⁴/q}(u) = 0`) with
/// `Tr_{q⁴/q}(√b/u) = 0`, `u ∉ F_q`, `u ∉ b^{1/4}·F_q`.  Deterministic
/// in `(curve, seed)`.  Returns `(u, draws)`.
pub fn twisted_u(c: &CurveB, seed: u64) -> (Eb, u64) {
    let f = &c.f;
    let mut rng = StdRng::seed_from_u64(seed ^ 0x7769_7374_6564);
    let sqrt_b = f.sq(&c.beta);
    let mut draws = 0u64;
    loop {
        draws += 1;
        let z = f.random(&mut rng);
        let u = f.frob(&z).add(&z);
        if u.is_zero() || u.in_fq() {
            continue;
        }
        if !trace_q(f, &f.mul(&sqrt_b, &f.inv(&u))).is_zero() {
            continue;
        }
        if in_line(f, &u, &c.beta) {
            continue;
        }
        return (u, draws);
    }
}

/// `x = u·y` for `y ∈ F_q` (no multiplication for `u = 1`).
fn x_of(f: &Fq4, u: &Eb, y: Fe) -> Eb {
    if *u == Eb::ONE {
        f.from_fq(y)
    } else {
        f.scale(u, y)
    }
}

/// Shared between the base (which chooses the line) and the oracle (which
/// charges its cost and needs `u`).
pub type SharedLine = Rc<RefCell<Option<LineState>>>;

// ── The base ─────────────────────────────────────────────────────────────

/// **`bin-x-line`**: `{±P : x(P) ∈ u·F_q^*}`, one column per abscissa
/// `{P, −P}`.  Parameters `line=standard` or `line=twisted,line_seed=N`.
/// The twisted line must lift on every abscissa (checked: an error
/// otherwise).
pub struct XLineBase<'a> {
    pub curve: &'a BinCurve,
    pub shared: SharedLine,
}

impl FactorBaseBuilder<CurveB> for XLineBase<'_> {
    fn name(&self) -> &str {
        BASE
    }
    fn describe(&self, params: &Params) -> String {
        match Line::from_params(params) {
            Ok(Line::Standard) => "x-line x(P) ∈ F_q^* (standard base), columns {±P}".into(),
            Ok(Line::Twisted { seed }) => format!(
                "twisted all-lift line x(P) ∈ u·F_q^*, u = z^q + z with Tr(√b/u) = 0 (line_seed {seed}), columns {{±P}}"
            ),
            Err(e) => e,
        }
    }
    fn parameters(&self) -> &[(&str, &str)] {
        &[
            ("line", "standard (u = 1) or twisted (all-lift u)"),
            ("line_seed", "twisted only: the seed of the draw of u"),
        ]
    }
    fn build(
        &self,
        _ctx: &InstanceCtx<CurveB>,
        params: &Params,
        _ops: &mut GroupOps,
    ) -> Result<FactorBase<PtB>, String> {
        let d = self.curve;
        let c = &d.curve;
        let f = &c.f;
        if !c.a.is_zero() {
            return Err("bin-x-line runs on a = 0 curves (f2m4_a0) only".into());
        }
        if f.in_fq2(&c.b) {
            return Err("bin-x-line needs b ∉ F_(q^2): [K:F_2] < N".into());
        }
        let line = Line::from_params(params)?;
        let m0 = f.f.muls();
        let (u, draws) = match line {
            Line::Standard => (Eb::ONE, 0),
            Line::Twisted { seed } => twisted_u(c, seed),
        };
        let mut line_muls = f.f.muls() - m0;
        let mut rng = StdRng::seed_from_u64(0x9b);
        let minus = u64::try_from(d.r - 1).map_err(|_| "bin-x-line: r needs one word")?;
        let (mut points, mut col_of, mut coef_of) = (Vec::new(), Vec::new(), Vec::new());
        let (mut columns, mut lifts) = (0usize, 0u64);
        for y in 1..f.f.q as Fe {
            let before = f.f.muls();
            let x = x_of(f, &u, y);
            line_muls += f.f.muls() - before;
            lifts += 1;
            let Some(pt) = c.lift_x(&x, &mut rng) else {
                continue;
            };
            let pt = if CountedGroup::key(c, &pt) & 1 == 1 {
                pt
            } else {
                CurveB::neg(c, &pt)
            };
            for (q, coef) in [(pt, 1), (CurveB::neg(c, &pt), minus)] {
                points.push(q);
                col_of.push(columns);
                coef_of.push(coef);
            }
            columns += 1;
        }
        if columns == 0 {
            return Err("no abscissa of the line lifts".into());
        }
        if matches!(line, Line::Twisted { .. }) && columns as u64 != u64::from(f.f.q) - 1 {
            return Err(format!(
                "the twisted line is not all-lift: {columns} of {} abscissae lift",
                f.f.q - 1
            ));
        }
        *self.shared.borrow_mut() = Some(LineState {
            u,
            fq_muls: line_muls,
            draws,
        });
        let m = f.f.m;
        let label = match line {
            Line::Standard => "standard x-line".to_string(),
            Line::Twisted { seed } => format!("twisted line (line_seed {seed})"),
        };
        let mut fb = FactorBase::from_column_map(
            format!(
                "{label} over F_(2^{m})^4, u = {:#x}, {columns} columns {{±P}}",
                f.pack(&u)
            ),
            points,
            col_of,
            coef_of,
            columns,
            |q| CountedGroup::key(c, q),
            |q| CountedGroup::key(c, &CurveB::neg(c, q)),
            |q| f.pack(&q.x) as u64,
        )?;
        fb.cost.count("sqrt_solves", lifts);
        Ok(fb)
    }
}

// ── S₅ over F₂[x₁..x₅, b], symmetrised (ported from the C1 ladder) ───────

mod semaev {
    //! `S₃(x₁,x₂,x₃) = (x₁+x₂)²x₃² + x₁x₂x₃ + x₁²x₂² + b`, `S₄ = Res_X(S₃(x₁,x₂,X),
    //! S₃(x₃,x₄,X))`, `S₅ = Res_X(S₃(x₁,x₂,X), S₄(X,x₃,x₄,x₅))` over `F₂`,
    //! then written in `e₁..e₄` of `x₁..x₄` (`research/c1_twisted_line_ladder_20261027/src/semaev.rs`).
    use std::collections::BTreeMap;

    const SLOTS: usize = 7; // x1..x5 = 0..4, b = 5, X = 6
    const BITS: u32 = 6;
    type Mono = u64;
    pub type Poly = Vec<Mono>;

    fn mono(exps: &[(usize, u32)]) -> Mono {
        exps.iter()
            .map(|&(s, e)| u64::from(e) << (BITS * s as u32))
            .sum()
    }
    pub fn exp(m: Mono, s: usize) -> u32 {
        ((m >> (BITS * s as u32)) & ((1 << BITS) - 1)) as u32
    }
    fn norm(mut v: Vec<Mono>) -> Poly {
        v.sort_unstable();
        let mut out: Vec<Mono> = Vec::with_capacity(v.len());
        for m in v {
            if out.last() == Some(&m) {
                out.pop();
            } else {
                out.push(m);
            }
        }
        out
    }
    fn add(a: &Poly, b: &Poly) -> Poly {
        let mut v = a.clone();
        v.extend_from_slice(b);
        norm(v)
    }
    fn mul(a: &Poly, b: &Poly) -> Poly {
        let mut v = Vec::with_capacity(a.len() * b.len());
        for &x in a {
            for &y in b {
                v.push(x + y);
            }
        }
        norm(v)
    }
    fn var(s: usize) -> Poly {
        vec![mono(&[(s, 1)])]
    }
    fn one() -> Poly {
        vec![0]
    }
    fn relabel(p: &Poly, map: &[usize; SLOTS]) -> Poly {
        norm(
            p.iter()
                .map(|&m| {
                    (0..SLOTS)
                        .map(|s| u64::from(exp(m, s)) << (BITS * map[s] as u32))
                        .sum()
                })
                .collect(),
        )
    }
    fn coeffs_in(p: &Poly, s: usize) -> Vec<Poly> {
        let dmax = p.iter().map(|&m| exp(m, s)).max().unwrap_or(0) as usize;
        let mut out = vec![vec![]; dmax + 1];
        for &m in p {
            let d = exp(m, s);
            out[d as usize].push(m - (u64::from(d) << (BITS * s as u32)));
        }
        out.into_iter().map(norm).collect()
    }
    /// The Sylvester resultant (a determinant over F₂, by a subset DP).
    fn resultant(f: &[Poly], g: &[Poly]) -> Poly {
        let (df, dg) = (f.len() - 1, g.len() - 1);
        let n = df + dg;
        let zero: Poly = vec![];
        let mut rows: Vec<Vec<&Poly>> = vec![];
        for i in 0..dg {
            let mut r = vec![&zero; n];
            for d in 0..=df {
                r[i + (df - d)] = &f[d];
            }
            rows.push(r);
        }
        for i in 0..df {
            let mut r = vec![&zero; n];
            for d in 0..=dg {
                r[i + (dg - d)] = &g[d];
            }
            rows.push(r);
        }
        let mut cur: BTreeMap<u32, Poly> = BTreeMap::new();
        cur.insert(0, one());
        for row in rows.iter().take(n) {
            let mut next: BTreeMap<u32, Poly> = BTreeMap::new();
            for (&s, minor) in &cur {
                for (c, entry) in row.iter().enumerate() {
                    if s >> c & 1 == 1 || entry.is_empty() {
                        continue;
                    }
                    let t = mul(entry, minor);
                    let e = next.entry(s | 1 << c).or_default();
                    *e = add(e, &t);
                }
            }
            cur = next;
        }
        cur.remove(&((1u32 << n) - 1)).unwrap_or_default()
    }
    fn s3(a: usize, b: usize, c: usize) -> Poly {
        let ab = add(&var(a), &var(b));
        let t1 = mul(&mul(&ab, &ab), &mul(&var(c), &var(c)));
        let t2 = mul(&mul(&var(a), &var(b)), &var(c));
        let t3 = mul(&mul(&var(a), &var(a)), &mul(&var(b), &var(b)));
        add(&add(&t1, &t2), &add(&t3, &var(5)))
    }
    pub fn s5_poly() -> Poly {
        let s4 = resultant(&coeffs_in(&s3(0, 1, 6), 6), &coeffs_in(&s3(2, 3, 6), 6));
        let map = [6, 2, 3, 4, 4, 5, 6];
        let s4r = relabel(&s4, &map);
        resultant(&coeffs_in(&s3(0, 1, 6), 6), &coeffs_in(&s4r, 6))
    }
    /// `e₁^{i₀} e₂^{i₁} e₃^{i₂} e₄^{i₃} x₅^j b^l`.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ETerm {
        pub i: [u8; 4],
        pub j: u8,
        pub l: u8,
    }
    fn elem() -> [Poly; 4] {
        let mut e: [Poly; 4] = [vec![], vec![], vec![], vec![]];
        for mask in 1u32..16 {
            let k = mask.count_ones() as usize;
            let mut p = one();
            for s in 0..4 {
                if mask >> s & 1 == 1 {
                    p = mul(&p, &var(s));
                }
            }
            e[k - 1] = add(&e[k - 1], &p);
        }
        e
    }
    /// `p` (symmetric in slots 0..3, degree ≤ `dmax` in each) in `e₁..e₄`,
    /// or `None` if it is not symmetric.
    pub fn symmetrise(p: &Poly, dmax: u32) -> Option<Vec<ETerm>> {
        let e = elem();
        let mut parts: Vec<[u32; 4]> = vec![];
        for a1 in 0..=dmax {
            for a2 in 0..=a1 {
                for a3 in 0..=a2 {
                    for a4 in 0..=a3 {
                        parts.push([a1, a2, a3, a4]);
                    }
                }
            }
        }
        let np = parts.len();
        let words = np.div_ceil(64);
        let pidx: BTreeMap<[u32; 4], usize> = parts
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, p)| (p, i))
            .collect();
        let mut epow: Vec<Vec<Poly>> = vec![];
        for ek in &e {
            let mut v = vec![one()];
            for d in 1..=dmax as usize {
                let nxt = mul(&v[d - 1], ek);
                v.push(nxt);
            }
            epow.push(v);
        }
        let mut m: Vec<Vec<u64>> = vec![vec![0u64; words]; np];
        for (mi, lam) in parts.iter().enumerate() {
            let i = [lam[0] - lam[1], lam[1] - lam[2], lam[2] - lam[3], lam[3]];
            let mut q = epow[0][i[0] as usize].clone();
            for k in 1..4 {
                if i[k] > 0 {
                    q = mul(&q, &epow[k][i[k] as usize]);
                }
            }
            for &mm in &q {
                let ex = [exp(mm, 0), exp(mm, 1), exp(mm, 2), exp(mm, 3)];
                if ex[0] >= ex[1] && ex[1] >= ex[2] && ex[2] >= ex[3] {
                    if let Some(&li) = pidx.get(&ex) {
                        m[mi][li / 64] ^= 1 << (li % 64);
                    }
                }
            }
        }
        let mut by: BTreeMap<(u32, u32), Vec<u64>> = BTreeMap::new();
        for &mm in p {
            let ex = [exp(mm, 0), exp(mm, 1), exp(mm, 2), exp(mm, 3)];
            let (j, l) = (exp(mm, 4), exp(mm, 5));
            if ex[0] > dmax {
                return None;
            }
            if ex[0] >= ex[1] && ex[1] >= ex[2] && ex[2] >= ex[3] {
                let v = by.entry((j, l)).or_insert_with(|| vec![0u64; words]);
                let li = pidx[&ex];
                v[li / 64] ^= 1 << (li % 64);
            }
        }
        let mut order: Vec<usize> = (0..np).collect();
        order.sort_by(|&a, &b| parts[b].cmp(&parts[a]));
        let mut out = vec![];
        for ((j, l), mut v) in by {
            for &mi in &order {
                if v[mi / 64] >> (mi % 64) & 1 == 1 {
                    let lam = parts[mi];
                    out.push(ETerm {
                        i: [
                            (lam[0] - lam[1]) as u8,
                            (lam[1] - lam[2]) as u8,
                            (lam[2] - lam[3]) as u8,
                            lam[3] as u8,
                        ],
                        j: j as u8,
                        l: l as u8,
                    });
                    for w in 0..words {
                        v[w] ^= m[mi][w];
                    }
                }
            }
            if v.iter().any(|&w| w != 0) {
                return None;
            }
        }
        out.sort();
        Some(out)
    }
}

pub use semaev::ETerm;

/// `S₅` in `e₁..e₄` of `x₁..x₄`, computed once per process.
pub fn s5_eterms() -> &'static [ETerm] {
    static TERMS: OnceLock<Vec<ETerm>> = OnceLock::new();
    TERMS.get_or_init(|| {
        let p = semaev::s5_poly();
        let t = semaev::symmetrise(&p, 8).expect("S₅ is symmetric in x₁..x₄");
        assert!(
            t.iter()
                .all(|t| t.i[0] % 2 == 0 && t.i[1] % 2 == 0 && t.i[2] % 2 == 0),
            "S₅ is a polynomial in e₁², e₂², e₃², e₄ in characteristic 2"
        );
        t
    })
}

/// `S₅(x₁..x₄, x_R)` through its `e`-form (counted; tests and checks).
pub fn eval_s5(f: &Fq4, b: &Eb, x: &[Eb; 4], xr: &Eb) -> Eb {
    let mut e = [Eb::ZERO; 5];
    e[0] = Eb::ONE;
    for xi in x {
        for j in (1..5).rev() {
            let t = f.mul(&e[j - 1], xi);
            e[j] = e[j].add(&t);
        }
    }
    let pow = |a: &Eb, k: u8| (0..k).fold(Eb::ONE, |acc, _| f.mul(&acc, a));
    let mut s = Eb::ZERO;
    for t in s5_eterms() {
        let mut v = f.mul(&pow(xr, t.j), &pow(b, t.l));
        for i in 0..4 {
            v = f.mul(&v, &pow(&e[i + 1], t.i[i]));
        }
        s = s.add(&v);
    }
    s
}

// ── One decomposition ────────────────────────────────────────────────────

/// The target-independent part of the system: for each term of `S₅`, its
/// monomial in `E`, its power of `x_R` and `u^w b^l`.
pub struct XPre {
    pub u: Eb,
    pub terms: Vec<([u8; 4], u8, Eb)>,
    pub max_j: u8,
    pub precompute_muls: u64,
}

impl XPre {
    pub fn new(c: &CurveB, u: Eb) -> XPre {
        let f = &c.f;
        let before = f.f.muls();
        let eterms = s5_eterms();
        let wmax = eterms
            .iter()
            .map(|t| {
                t.i[0] as usize + 2 * t.i[1] as usize + 3 * t.i[2] as usize + 4 * t.i[3] as usize
            })
            .max()
            .unwrap_or(0);
        let lmax = eterms.iter().map(|t| t.l as usize).max().unwrap_or(0);
        let mut upow = vec![Eb::ONE];
        for w in 1..=wmax {
            upow.push(f.mul(&upow[w - 1], &u));
        }
        let mut bpow = vec![Eb::ONE];
        for l in 1..=lmax {
            bpow.push(f.mul(&bpow[l - 1], &c.b));
        }
        let mut terms = Vec::with_capacity(eterms.len());
        for t in eterms {
            let w =
                t.i[0] as usize + 2 * t.i[1] as usize + 3 * t.i[2] as usize + 4 * t.i[3] as usize;
            let coef = f.mul(&upow[w], &bpow[t.l as usize]);
            terms.push(([t.i[0] / 2, t.i[1] / 2, t.i[2] / 2, t.i[3]], t.j, coef));
        }
        let max_j = eterms.iter().map(|t| t.j).max().unwrap_or(0);
        XPre {
            u,
            terms,
            max_j,
            precompute_muls: f.f.muls() - before,
        }
    }

    /// The Weil restriction for the target abscissa `xr`: four `F_q`
    /// polynomials in `(E₁, E₂, E₃, E₄)`.
    pub fn system(&self, f: &Fq4, xr: &Eb) -> Vec<HashMap<[u8; 4], Fe>> {
        let mut xp = vec![Eb::ONE];
        for j in 1..=self.max_j as usize {
            xp.push(f.mul(&xp[j - 1], xr));
        }
        let mut coef: BTreeMap<[u8; 4], Eb> = BTreeMap::new();
        for (key, j, c0) in &self.terms {
            let c = if *j == 0 {
                *c0
            } else {
                f.mul(c0, &xp[*j as usize])
            };
            let e = coef.entry(*key).or_insert(Eb::ZERO);
            *e = e.add(&c);
        }
        let mut comps: Vec<HashMap<[u8; 4], Fe>> = vec![HashMap::new(); 4];
        for (key, c) in coef {
            for (r, comp) in comps.iter_mut().enumerate() {
                if c.0[r] != 0 {
                    comp.insert(key, c.0[r]);
                }
            }
        }
        comps
    }
}

fn close(s: &mut BinSolve) {
    s.total_muls = s.weil_muls
        + s.triangularise_muls
        + s.echelon_muls
        + s.nf_muls
        + s.charpoly_muls
        + s.roots_muls
        + s.eigenvector_muls
        + s.verify_muls
        + s.split_muls;
}

/// The signs `s_i` with `Σ s_i P_i = r`, if any (each candidate's four
/// additions run on the curve's operation counter).
fn signed_sum(curve: &CurveB, pts: &[PtB; 4], r: &PtB) -> Option<[PtB; 4]> {
    for s in 0..16u32 {
        let signed: [PtB; 4] = core::array::from_fn(|i| {
            if s >> i & 1 == 1 {
                curve.neg(&pts[i])
            } else {
                pts[i]
            }
        });
        let mut acc = PtB::INF;
        for q in &signed {
            acc = curve.add(&acc, q);
        }
        if !acc.inf && acc.x == r.x && acc.y == r.y {
            return Some(signed);
        }
    }
    None
}

/// Decompose `r = ±P₁ ± P₂ ± P₃ ± P₄`, `x(P_i) ∈ u·F_q^*`, distinct
/// abscissae.  `Err` carries what an unsolved attempt cost.
#[allow(clippy::result_large_err)]
pub fn solve_x_bin(
    curve: &CurveB,
    pre: &XPre,
    r: &PtB,
    opts: BinOptions,
    macaulay: u8,
    rng: &mut StdRng,
    trace: Option<&mut BinTrace>,
) -> Result<BinD4Solve, BinSolve> {
    let f = &curve.f;
    let fq = &f.f;
    let g = &fq.tab;
    let mut out = BinD4Solve::default();
    out.solve.frob_roots = opts.frob_roots;
    let before = fq.muls();
    let comps = pre.system(f, &r.x);
    out.solve.weil_muls = fq.muls() - before;
    let sols = solve_bin::<4>(
        fq,
        &comps,
        &WEIGHTS,
        3,
        macaulay,
        opts.sparsest,
        &mut out.solve,
        trace,
    );
    let Some(sols) = sols else {
        close(&mut out.solve);
        return Err(out.solve);
    };
    out.rational_solutions = sols.len();
    let mut found = BTreeSet::new();
    for s in sols {
        // s = (E₁, E₂, E₃, E₄); e_j = √E_j (j ≤ 3), e₄ = E₄.
        let b0 = fq.muls();
        let e = [fq.sqrt(s[0]), fq.sqrt(s[1]), fq.sqrt(s[2]), s[3]];
        out.solve.split_muls += fq.muls() - b0;
        let mut sm = 0;
        let quartic = [e[3], e[2], e[1], e[0], 1];
        let rts = roots_with(fq, &quartic, &mut sm, opts.frob_roots);
        out.solve.split_muls += sm;
        let mut multisets = Vec::new();
        for (i1, &a) in rts.iter().enumerate() {
            for (i2, &b) in rts.iter().enumerate().skip(i1) {
                for (i3, &c) in rts.iter().enumerate().skip(i2) {
                    for &d in rts.iter().skip(i3) {
                        let y = [a, b, c, d];
                        let mut el = [1u32, 0, 0, 0, 0];
                        for &v in &y {
                            for k in (1..5).rev() {
                                el[k] ^= g.mul(el[k - 1], v);
                            }
                        }
                        out.solve.split_muls += 16;
                        if el[1] == e[0] && el[2] == e[1] && el[3] == e[2] && el[4] == e[3] {
                            multisets.push(y);
                        }
                    }
                }
            }
        }
        for ys in multisets {
            if ys.contains(&0) {
                continue;
            }
            let mut sorted = ys;
            sorted.sort_unstable();
            if sorted.windows(2).any(|w| w[0] == w[1]) {
                continue;
            }
            let lift_before = fq.muls();
            let pts: Option<Vec<PtB>> = ys
                .iter()
                .map(|&y| curve.lift_x(&x_of(f, &pre.u, y), rng))
                .collect();
            out.solve.split_muls += fq.muls() - lift_before;
            let Some(pts) = pts else {
                continue;
            };
            let pts = [pts[0], pts[1], pts[2], pts[3]];
            if let Some(signed) = signed_sum(curve, &pts, r) {
                if found.insert(sorted) {
                    out.witnesses.push(signed);
                }
                if out.witness.is_none() {
                    out.witness = Some(signed);
                }
            }
        }
    }
    out.decompositions = found.into_iter().collect();
    close(&mut out.solve);
    Ok(out)
}

// ── The oracle ───────────────────────────────────────────────────────────

/// **`gaudry-x-bin`**: [`solve_x_bin`] on the line the `bin-x-line` base
/// chose.  Charged as `gaudry-d4-bin` is (module docs); unsolved systems
/// are charged when `charge_unsolved`.
pub struct XLineOracle<'a> {
    pub curve: &'a BinCurve,
    pub macaulay: u8,
    pub seed: u64,
    pub opts: BinOptions,
    pub charge_unsolved: bool,
    pub shared: SharedLine,
    trace: BinTrace,
    pre: Option<XPre>,
    totals: SolverTotals,
    per_add: u64,
}

impl<'a> XLineOracle<'a> {
    pub fn new(
        curve: &'a BinCurve,
        macaulay: u8,
        seed: u64,
        opts: BinOptions,
        shared: SharedLine,
    ) -> Self {
        XLineOracle {
            curve,
            macaulay,
            seed,
            opts,
            charge_unsolved: false,
            shared,
            trace: BinTrace::default(),
            pre: None,
            totals: SolverTotals {
                solver: "x-line-macaulay".into(),
                op_unit: "F_q multiplications".into(),
                ..Default::default()
            },
            per_add: fq_muls_per_add(&curve.curve),
        }
    }
    fn shape(&self) -> SystemShape {
        SystemShape {
            n_vars: 4,
            n_equations: 4,
            degrees: vec![8, 8, 8, 8],
            semi_regular_degree: Some(26),
        }
    }
}

impl DecompositionOracle<CurveB> for XLineOracle<'_> {
    fn name(&self) -> &str {
        ORACLE
    }
    fn summands(&self) -> u32 {
        4
    }
    fn describe(&self, _params: &Params) -> String {
        format!(
            "S₅ on the x-line u·F_q in (e₁², e₂², e₃², e₄) of x/u, weights (2, 2, 2, 1), Macaulay degree {}, eigen step along the chains{}{}",
            self.macaulay,
            if self.opts.sparsest { ", sparsest pivots" } else { "" },
            if self.opts.replay { ", shape replay" } else { "" }
        )
    }
    fn prepare(
        &mut self,
        _ctx: &InstanceCtx<CurveB>,
        _fb: &FactorBase<PtB>,
        _params: &Params,
        _ops: &mut GroupOps,
    ) -> Result<(), String> {
        let line =
            (*self.shared.borrow()).ok_or("gaudry-x-bin runs on the bin-x-line base only")?;
        let pre = XPre::new(&self.curve.curve, line.u);
        self.totals.ops += line.fq_muls + pre.precompute_muls;
        for (k, v) in [
            ("line_fq_muls", line.fq_muls),
            ("line_draws", line.draws),
            ("precompute_fq_muls", pre.precompute_muls),
            ("fp_muls_per_add", self.per_add),
            ("s5_terms", s5_eterms().len() as u64),
        ] {
            self.totals.extra.insert(k.into(), v);
        }
        self.pre = Some(pre);
        Ok(())
    }
    fn decompose(
        &mut self,
        _ctx: &InstanceCtx<CurveB>,
        fb: &FactorBase<PtB>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: PtB,
    ) -> Option<Vec<usize>> {
        let pre = self.pre.as_ref()?;
        let c = &self.curve.curve;
        let mut rng =
            StdRng::seed_from_u64(self.seed ^ self.totals.calls.wrapping_mul(0x9e37_79b9));
        let adds_before = c.ops();
        let t = std::time::Instant::now();
        let sol = solve_x_bin(
            c,
            pre,
            &point,
            self.opts,
            self.macaulay,
            &mut rng,
            self.opts.replay.then_some(&mut self.trace),
        );
        self.totals.wall_ns += t.elapsed().as_nanos() as u64;
        ops.adds += c.ops() - adds_before;
        self.totals.calls += 1;
        self.totals.shape = Some(self.shape());
        let degree = u32::from(self.macaulay);
        let sol = match sol {
            Ok(sol) => sol,
            Err(cost) => {
                *self
                    .totals
                    .extra
                    .entry("degenerate_systems".into())
                    .or_insert(0) += 1;
                if self.charge_unsolved {
                    self.totals.ops += cost.total_muls;
                    *self
                        .totals
                        .extra
                        .entry("degenerate_fq_muls".into())
                        .or_insert(0) += cost.total_muls;
                }
                self.totals
                    .observe_degree(degree, degree, cost.total_muls, true, false);
                return None;
            }
        };
        self.totals.ops += sol.solve.total_muls;
        for (k, v) in [
            ("weil_fq_muls", sol.solve.weil_muls),
            ("echelon_fq_muls", sol.solve.echelon_muls),
            ("normal_form_fq_muls", sol.solve.nf_muls),
            ("charpoly_fq_muls", sol.solve.charpoly_muls),
            ("split_fq_muls", sol.solve.split_muls),
            ("rational_solutions", sol.rational_solutions as u64),
            ("decompositions", sol.decompositions.len() as u64),
            (
                "systems_with_a_decomposition",
                u64::from(!sol.decompositions.is_empty()),
            ),
        ] {
            *self.totals.extra.entry(k.into()).or_insert(0) += v;
        }
        self.totals.solving_degree_sum += u64::from(self.macaulay);
        self.totals.solving_degree_max = self.totals.solving_degree_max.max(degree);
        self.totals.calls_with_a_degree += 1;
        let idx: Option<Vec<usize>> = sol.witness.as_ref().and_then(|w| {
            w.iter()
                .map(|q| fb.index_of_key(CountedGroup::key(c, q)))
                .collect()
        });
        if sol.witness.is_some() && idx.is_none() {
            counters.lift_failures += 1;
        }
        self.totals
            .observe_degree(degree, degree, sol.solve.total_muls, false, idx.is_some());
        idx
    }
    fn last_system(&self) -> Option<SystemShape> {
        Some(self.shape())
    }
    fn solver_totals(&self) -> Option<SolverTotals> {
        Some(self.totals.clone())
    }
}

/// Every unordered decomposition `r = ±P₁ ± P₂ ± P₃ ± P₄` over a base of
/// distinct columns, by a pair table (an independent check of the
/// oracle; uncounted, tests only).  Returns sorted packed abscissae.
pub fn enumerate_decompositions(c: &CurveB, base: &[PtB], r: &PtB) -> BTreeSet<[u64; 4]> {
    let f = &c.f;
    let n = base.len();
    let signed: Vec<(usize, PtB)> = (0..n)
        .flat_map(|i| [(i, base[i]), (i, c.neg(&base[i]))])
        .collect();
    let key = |p: &PtB| (p.inf, f.pack(&p.x), f.pack(&p.y));
    let mut pairs: HashMap<(bool, u128, u128), Vec<(usize, usize)>> = HashMap::new();
    for a in 0..signed.len() {
        for b in a + 1..signed.len() {
            if signed[a].0 == signed[b].0 {
                continue;
            }
            let s = c.add(&signed[a].1, &signed[b].1);
            pairs
                .entry(key(&s))
                .or_default()
                .push((signed[a].0, signed[b].0));
        }
    }
    let mut out = BTreeSet::new();
    for a in 0..signed.len() {
        for b in a + 1..signed.len() {
            if signed[a].0 == signed[b].0 {
                continue;
            }
            let s = c.add(&signed[a].1, &signed[b].1);
            let rest = c.add(r, &c.neg(&s));
            if let Some(v) = pairs.get(&key(&rest)) {
                for &(i, j) in v {
                    let cols = [signed[a].0, signed[b].0, i, j];
                    let mut set: Vec<usize> = cols.to_vec();
                    set.sort_unstable();
                    set.dedup();
                    if set.len() == 4 {
                        let mut xs = [0u64; 4];
                        for (k, &col) in set.iter().enumerate() {
                            xs[k] = f.pack(&base[col].x) as u64;
                        }
                        xs.sort_unstable();
                        out.insert(xs);
                    }
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_points(c: &BinCurve, line: Line) -> (Vec<PtB>, Eb) {
        let shared: SharedLine = Rc::new(RefCell::new(None));
        let b = XLineBase {
            curve: c,
            shared: shared.clone(),
        };
        let mut p = Params::default();
        match line {
            Line::Standard => {
                p.set("line", "standard");
            }
            Line::Twisted { seed } => {
                p.set("line", "twisted");
                p.set("line_seed", seed.to_string());
            }
        }
        let ctx = InstanceCtx {
            group: &c.curve,
            generator: c.generator,
            target: c.generator,
            r: c.r as u64,
            cofactor: c.cofactor as u64,
            group_order: c.group_order as u64,
            name: "test".into(),
            field_degree: None,
        };
        let fb = b.build(&ctx, &p, &mut GroupOps::default()).unwrap();
        let pts: Vec<PtB> = fb.points.iter().step_by(2).copied().collect();
        let u = shared.borrow().unwrap().u;
        (pts, u)
    }

    #[test]
    fn s5_e_form_vanishes_on_sums_and_matches_the_curve() {
        let c = find_bin_curve_a0(5, 1, 8).unwrap();
        let cv = &c.curve;
        let f = &cv.f;
        let mut rng = StdRng::seed_from_u64(5);
        let rand_pt = |rng: &mut StdRng| loop {
            let x = f.random(rng);
            if let Some(p) = cv.lift_x(&x, rng) {
                return p;
            }
        };
        for _ in 0..6 {
            let p: [PtB; 4] = core::array::from_fn(|_| rand_pt(&mut rng));
            let r = p.iter().fold(PtB::INF, |a, q| cv.add(&a, q));
            if r.inf {
                continue;
            }
            let xs = [p[0].x, p[1].x, p[2].x, p[3].x];
            assert!(eval_s5(f, &cv.b, &xs, &r.x).is_zero());
            let y = f.random(&mut rng);
            assert_eq!(
                eval_s5(f, &cv.b, &xs, &y),
                cv.s5(&[xs[0], xs[1], xs[2], xs[3], y])
            );
        }
    }

    #[test]
    fn curve_and_lines_follow_the_construction_rules() {
        for m in [5u32, 7] {
            let c = find_bin_curve_a0(m, 1, 8).unwrap();
            let cv = &c.curve;
            assert!(cv.a.is_zero() && !cv.f.in_fq2(&cv.b));
            assert!(cv.mul(&c.generator, c.r).inf);
            assert_eq!(c.cofactor * c.r, c.group_order);
            let (std, _) = base_points(&c, Line::Standard);
            let (tw, u) = base_points(&c, Line::Twisted { seed: 1 });
            let (_, u2) = base_points(&c, Line::Twisted { seed: 2 });
            assert_eq!(tw.len() as u32, cv.f.f.q - 1, "all-lift");
            assert!(std.len() < tw.len());
            assert!(trace_q(&cv.f, &u).is_zero() && !u.in_fq());
            assert!(!in_line(&cv.f, &u, &cv.beta));
            assert!(!in_line(&cv.f, &u2, &u), "A/A lines differ");
        }
    }

    #[test]
    fn oracle_finds_every_decomposition_the_pair_table_finds() {
        let c = find_bin_curve_a0(5, 1, 8).unwrap();
        let cv = &c.curve;
        let mut rng = StdRng::seed_from_u64(11);
        for line in [Line::Standard, Line::Twisted { seed: 1 }] {
            let (base, u) = base_points(&c, line);
            let pre = XPre::new(cv, u);
            let (mut agree, mut nonempty) = (0, 0);
            for t in 0..12u128 {
                let r = cv.mul(&c.generator, 1 + 7919 * t);
                let want = enumerate_decompositions(cv, &base, &r);
                let opts = BinOptions {
                    weighted: true,
                    sparsest: true,
                    ..Default::default()
                };
                let Ok(sol) = solve_x_bin(cv, &pre, &r, opts, 26, &mut rng, None) else {
                    continue;
                };
                let got: BTreeSet<[u64; 4]> = sol
                    .witnesses
                    .iter()
                    .map(|w| {
                        let mut xs = [0u64; 4];
                        for (k, p) in w.iter().enumerate() {
                            xs[k] = cv.f.pack(&p.x) as u64;
                        }
                        xs.sort_unstable();
                        xs
                    })
                    .collect();
                for w in &sol.witnesses {
                    let s = w.iter().fold(PtB::INF, |a, q| cv.add(&a, q));
                    assert_eq!(s, r);
                }
                assert_eq!(got, want, "{line:?} target {t}");
                agree += 1;
                nonempty += usize::from(!want.is_empty());
            }
            assert!(agree >= 10, "{line:?}: {agree} closed systems");
            assert!(nonempty > 0, "{line:?}: no decomposition at all");
        }
    }
}
