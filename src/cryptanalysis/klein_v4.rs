//! **E22 Stage B of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (§9.7, §9.10):
//! `S₅` in the invariants of the full 2-torsion group, solved by a weighted
//! Macaulay solve in four unknowns of weights `(2, 2, 1, 1)`.**
//!
//! The curve and the coordinate are [`super::quartic_folds::KleinInstance`]'s:
//! full rational 2-torsion and `z = g⁻¹(x)`, in which the three translations
//! are `z ↦ −z, 1/z, −1/z`.  With `u = z² + z⁻²`, `P₁ = Π(z_i − 1/z_i)`,
//! `P₂ = Π(z_i + 1/z_i)` and `e_k = e_k(u)`, `H = S₅ · Π(cz_i + 1)⁸(cz_R + 1)⁸ /
//! Πz_i⁴` lies in the span of `{e_ie_j} ∪ {P₁e_k} ∪ {P₂e_k} ∪ {P₁P₂e_k}` (30
//! functions; [`SymmetrisedV4::precompute`] checks it on fresh points).
//! `e₁ = (P₂² − P₁² − 4e₃)/16` and `e₄ = (P₁² + P₂²)/2 − 4e₂ − 16` leave the
//! unknowns `(e₂, e₃, P₁, P₂)`: weighted Bézout `64`, weighted Macaulay degree
//! `11`.
//!
//! The solver core (weighted grevlex, triangularisation, Macaulay rows,
//! echelon, normal forms, eigenvectors, extraction, reruns and the shape
//! trace) is [`super::d4_weighted_s5`]'s, copied with the weights, the equation
//! degree and the bound changed, so that lane's solver is untouched.

use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::time::Instant;

use rand::rngs::StdRng;
use serde::Serialize;

use super::gaudry_cubic::{am, charpoly_mod_p, eigenvectors_mod_p, mm, rref_mod_p, sm, PolyRing};
use super::gaudry_quartic::{
    charpoly_u16, echelon_by_lead, eigenvectors_u16, split_quartic, Curve4, Fp4, NormalForms4, Pt4,
    QuarticSolve, ReplayOutcome, E4,
};
use super::quartic_folds::KleinInstance;
use super::residual_walk::{inv_mod, pow_mod};

/// Weights of `(e₂, e₃, P₁, P₂)`.
pub const WEIGHTS: [u8; 4] = [2, 2, 1, 1];
/// Weighted degree of every Weil component of `H`.
pub const EQUATION_WDEG: u16 = 4;
/// `Σ dᵢ − Σ wⱼ + 1` for four equations of weighted degree 8.
pub const MACAULAY_BOUND: u16 = 4 * 4 - (2 + 2 + 1 + 1) + 1;
/// The unknown whose multiplication matrix is diagonalised (`P₂`).
const EIG_VAR: usize = 3;

pub fn wdeg(m: &[u8; 4]) -> u16 {
    m.iter()
        .zip(WEIGHTS)
        .map(|(&e, w)| e as u16 * w as u16)
        .sum()
}

/// Weighted grevlex: weighted degree first, then the reverse-lexicographic
/// tie-break of `gaudry_quartic`'s grevlex.
pub fn wcmp(a: &[u8; 4], b: &[u8; 4]) -> Ordering {
    let (da, db) = (wdeg(a), wdeg(b));
    if da != db {
        return da.cmp(&db);
    }
    for i in (0..4).rev() {
        if a[i] != b[i] {
            return b[i].cmp(&a[i]);
        }
    }
    Ordering::Equal
}

/// Monomials of weighted degree `≤ d`, descending in weighted grevlex.
pub fn weighted_monomials_desc(d: u16) -> Vec<[u8; 4]> {
    let mut v = Vec::new();
    for a in 0..=d / 2 {
        for b in 0..=(d - 2 * a) / 2 {
            for c in 0..=(d - 2 * a - 2 * b) {
                for e in 0..=(d - 2 * a - 2 * b - c) {
                    v.push([a as u8, b as u8, c as u8, e as u8]);
                }
            }
        }
    }
    v.sort_by(|x, y| wcmp(y, x));
    v
}

fn leading(f: &HashMap<[u8; 4], u64>) -> Option<[u8; 4]> {
    f.iter()
        .filter(|(_, &c)| c != 0)
        .map(|(m, _)| *m)
        .max_by(wcmp)
}

/// Make the leading monomials pairwise distinct (as `gaudry_quartic`'s
/// `triangularise_leads4`, in the weighted order).
fn triangularise(
    comps: &[HashMap<[u8; 4], u64>],
    p: u64,
    muls: &mut u64,
) -> (Vec<HashMap<[u8; 4], u64>>, Vec<Option<[u8; 4]>>) {
    let mut fs: Vec<HashMap<[u8; 4], u64>> = comps.to_vec();
    for _round in 0..16 {
        fs.sort_by(|a, b| match (leading(a), leading(b)) {
            (Some(x), Some(y)) => wcmp(&y, &x),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        });
        let mut changed = false;
        for i in 1..fs.len() {
            let (Some(li), Some(lj)) = (leading(&fs[i]), leading(&fs[i - 1])) else {
                continue;
            };
            if li == lj {
                let k = mm(fs[i][&li], inv_mod(fs[i - 1][&lj], p), p);
                let prev = fs[i - 1].clone();
                for (&m, &c) in &prev {
                    let e = fs[i].entry(m).or_insert(0);
                    *e = sm(*e, mm(k, c, p), p);
                    *muls += 1;
                }
                fs[i].retain(|_, c| *c != 0);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let leads = fs.iter().map(leading).collect();
    (fs, leads)
}

fn eval(comp: &HashMap<[u8; 4], u64>, e: &[u64; 4], p: u64, muls: &mut u64) -> u64 {
    let mut acc = 0u64;
    for (m, &c) in comp {
        let mut t = c;
        for k in 0..4 {
            for _ in 0..m[k] {
                t = mm(t, e[k], p);
                *muls += 1;
            }
        }
        acc = am(acc, t, p);
    }
    acc
}

/// The shape of one weighted solve at one degree, recorded on a first
/// system and replayed on later systems with the same leading monomials
/// (F4Remake's idea, as [`super::gaudry_quartic::ShapeTrace`] for the
/// unweighted solve): the triangularised leads, the built rows that reduced
/// to zero, the pivot columns and the quotient dimension.  A replay does
/// not build the zero rows and must reach the same pivots and dimension,
/// or the full solve runs (charged as well).
#[derive(Clone, Debug, Default, Serialize)]
pub struct WeightedTrace {
    pub degree: u16,
    pub leads: Vec<Option<[u8; 4]>>,
    pub zero_ids: Vec<u32>,
    pub pivot_cols: Vec<u32>,
    pub dim: usize,
}

enum Inner {
    Solved(Vec<[u64; 4]>),
    /// A replay did not reproduce the trace's shape.
    Mismatch,
    Unsolved(String),
    /// A degenerate eigenspace needs a weight-2 unknown's multiplication
    /// matrix, which this degree cannot build.
    NeedHigher,
    /// An eigenspace did not resolve to rational points (a group stayed
    /// above dimension 1, or a vector failed extraction or verification).
    Unresolved(String),
}

/// [`super::gaudry_cubic::split_eigenspace`], keeping the vectors of each
/// eigenvalue `μ` of the restricted map together: one group per rational
/// `μ`.  The same operations and the same count.
fn split_groups(
    w: &[Vec<u64>],
    mv: &[Vec<u64>],
    p: u64,
    rng: &mut StdRng,
    muls: &mut u64,
) -> Vec<Vec<Vec<u64>>> {
    let k = w.len();
    let dim = mv.len();
    let mut y = vec![vec![0u64; dim]; k];
    for i in 0..k {
        for l in 0..dim {
            if w[i][l] == 0 {
                continue;
            }
            for j in 0..dim {
                if mv[l][j] != 0 {
                    y[i][j] = am(y[i][j], mm(w[i][l], mv[l][j], p), p);
                    *muls += 1;
                }
            }
        }
    }
    let mut a = vec![vec![0u64; k]; k];
    for i in 0..k {
        let mut sys: Vec<Vec<u64>> = (0..dim)
            .map(|r| {
                let mut row: Vec<u64> = (0..k).map(|j| w[j][r]).collect();
                row.push(y[i][r]);
                row
            })
            .collect();
        let pivots = rref_mod_p(&mut sys, p, muls);
        for (r, &pc) in pivots.iter().enumerate() {
            if pc < k {
                a[i][pc] = sys[r][k];
            }
        }
    }
    let mut at = vec![vec![0u64; k]; k];
    for i in 0..k {
        for j in 0..k {
            at[j][i] = a[i][j];
        }
    }
    let cp = charpoly_mod_p(&a, p, muls);
    let ring = PolyRing::new(p);
    let mus = ring.roots(&cp, rng);
    *muls += ring.muls.get();
    let mut out = Vec::new();
    for mu in mus {
        let mut group = Vec::new();
        for v in eigenvectors_mod_p(&at, mu, p, muls) {
            let mut u = vec![0u64; dim];
            for j in 0..k {
                if v[j] == 0 {
                    continue;
                }
                for r in 0..dim {
                    if w[j][r] != 0 {
                        u[r] = am(u[r], mm(v[j], w[j][r], p), p);
                        *muls += 1;
                    }
                }
            }
            group.push(u);
        }
        out.push(group);
    }
    out
}

/// Read the rational points off resolved eigenspace groups of `M_π` at
/// eigenvalue `lam`.  `Err` when any group has more than one vector, or a
/// vector has no `1`-coordinate, a `π`-coordinate other than `lam`, or
/// fails one of the equations: the quotient's multiplication matrices are
/// then not those of a radical zero-dimensional system here, and a root
/// could be lost, so the solve must not report "solved".
fn extract_points(
    groups: &[Vec<Vec<u64>>],
    lam: u64,
    idx: &[usize],
    comps: &[HashMap<[u8; 4], u64>],
    p: u64,
    verify_muls: &mut u64,
) -> Result<Vec<[u64; 4]>, String> {
    let mut out = Vec::new();
    for g in groups {
        if g.len() > 1 {
            return Err(format!(
                "an eigenspace of π = {lam} stays of dimension {} after σ₁, σ₂, σ₃",
                g.len()
            ));
        }
        for w in g {
            if w[idx[0]] == 0 {
                return Err(format!("an eigenvector of π = {lam} has no 1-coordinate"));
            }
            let inv = inv_mod(w[idx[0]], p);
            let e: [u64; 4] = core::array::from_fn(|k| mm(w[idx[k + 1]], inv, p));
            if e[EIG_VAR] != lam {
                return Err(format!(
                    "an eigenvector of π = {lam} reads π = {}",
                    e[EIG_VAR]
                ));
            }
            if !comps.iter().all(|c| eval(c, &e, p, verify_muls) == 0) {
                return Err(format!("the point read at π = {lam} fails the system"));
            }
            out.push(e);
        }
    }
    Ok(out)
}

#[allow(clippy::too_many_arguments)]
fn solve_at(
    comps: &[HashMap<[u8; 4], u64>],
    d: u16,
    p: u64,
    rng: &mut StdRng,
    st: &mut QuarticSolve,
    row_selection: bool,
    debug: bool,
    replay: Option<&WeightedTrace>,
    record: Option<&mut WeightedTrace>,
) -> Inner {
    assert!(p < 1 << 15, "rows are u16 and products must fit u32");
    let t0 = Instant::now();
    st.equation_degree = EQUATION_WDEG as u8;
    st.macaulay_degree = d as u8;
    let cols = weighted_monomials_desc(d);
    let col_index: HashMap<[u8; 4], usize> =
        cols.iter().enumerate().map(|(i, m)| (*m, i)).collect();
    let mut tri = 0u64;
    let (comps, leads) = triangularise(comps, p, &mut tri);
    st.triangularise_muls += tri;
    if replay.is_some_and(|t| t.degree != d || t.leads != leads) {
        return Inner::Mismatch;
    }
    let skip: std::collections::HashSet<u32> = replay
        .map(|t| t.zero_ids.iter().copied().collect())
        .unwrap_or_default();
    let mut built = 0u32;
    let divides = |lm: &[u8; 4], m: &[u8; 4]| (0..4).all(|k| lm[k] <= m[k]);
    let mut rows: Vec<Vec<u16>> = Vec::new();
    for (i, comp) in comps.iter().enumerate() {
        let Some(dg) = comp
            .iter()
            .filter(|(_, &v)| v != 0)
            .map(|(m, _)| wdeg(m))
            .max()
        else {
            continue;
        };
        if dg > d {
            return Inner::Unsolved("an equation above the Macaulay degree".into());
        }
        for sh in weighted_monomials_desc(d - dg) {
            if row_selection
                && leads
                    .iter()
                    .take(i)
                    .any(|lm| lm.is_some_and(|lm| divides(&lm, &sh)))
            {
                continue;
            }
            built += 1;
            if skip.contains(&(built - 1)) {
                continue;
            }
            let mut row = vec![0u16; cols.len()];
            for (&e, &c) in comp {
                let m: [u8; 4] = core::array::from_fn(|k| e[k] + sh[k]);
                let ci = col_index[&m];
                row[ci] = am(row[ci] as u64, c, p) as u16;
            }
            rows.push(row);
        }
    }
    st.rows = rows.len();
    st.cols = cols.len();
    st.build_ms += t0.elapsed().as_secs_f64() * 1e3;
    if debug {
        eprintln!(
            "  weighted degree {d}: {} rows × {} columns ({:.3} GB)",
            st.rows,
            st.cols,
            (st.rows * st.cols * 2) as f64 / 1e9
        );
    }
    let t1 = Instant::now();
    let mut ech_muls = 0u64;
    let ech = echelon_by_lead(rows, p as u32, &mut ech_muls, false);
    st.echelon_muls += ech_muls;
    st.echelon_ms += t1.elapsed().as_secs_f64() * 1e3;
    st.zero_rows = ech.zero_rows;
    st.pivots = ech.pivot_row.iter().filter(|&&r| r != u32::MAX).count();
    let pivot_cols: Vec<u32> = (0..cols.len() as u32)
        .filter(|&c| ech.pivot_row[c as usize] != u32::MAX)
        .collect();
    if replay.is_some_and(|t| t.pivot_cols != pivot_cols) {
        return Inner::Mismatch;
    }
    let is_std: Vec<bool> = (0..cols.len())
        .map(|c| ech.pivot_row[c] == u32::MAX && wdeg(&cols[c]) < d)
        .collect();
    let standard: Vec<usize> = (0..cols.len()).filter(|&c| is_std[c]).collect();
    let dim = standard.len();
    st.dim = dim;
    if replay.is_some_and(|t| t.dim != dim) {
        return Inner::Mismatch;
    }
    if let Some(out) = record {
        *out = WeightedTrace {
            degree: d,
            leads: leads.clone(),
            zero_ids: ech.zero_ids.clone(),
            pivot_cols,
            dim,
        };
    }
    let mut std_index = vec![u32::MAX; cols.len()];
    for (i, &c) in standard.iter().enumerate() {
        std_index[c] = i as u32;
    }
    let mut low: Vec<usize> = Vec::with_capacity(5);
    for v in 0..=4usize {
        let m: [u8; 4] = core::array::from_fn(|k| u8::from(v > 0 && k == v - 1));
        low.push(col_index[&m]);
    }
    if low.iter().any(|&c| !is_std[c]) {
        return Inner::Unsolved("1 or an unknown is not standard".into());
    }
    let idx: Vec<usize> = low.iter().map(|&c| std_index[c] as usize).collect();
    let t2 = Instant::now();
    let mut nfs = NormalForms4 {
        ech: &ech,
        is_std,
        std_index,
        dim,
        p: p as u32,
        memo: vec![None; cols.len()],
        reached: 0,
    };
    let mult_matrix =
        |nfs: &mut NormalForms4, var: usize, muls: &mut u64| -> Option<Vec<Vec<u64>>> {
            let mut targets = Vec::with_capacity(dim);
            for &bc in &standard {
                let mut m = cols[bc];
                m[var] += 1;
                targets.push(*col_index.get(&m)?);
            }
            nfs.ensure(&targets, muls).ok()?;
            let mut out = vec![vec![0u64; dim]; dim];
            for (bi, &tc) in targets.iter().enumerate() {
                for (ri, v) in nfs.vector(tc).into_iter().enumerate() {
                    out[ri][bi] = v;
                }
            }
            Some(out)
        };
    let mut nf_muls = 0u64;
    let m_pi = mult_matrix(&mut nfs, EIG_VAR, &mut nf_muls);
    st.nf_muls += nf_muls;
    st.nf_ms += t2.elapsed().as_secs_f64() * 1e3;
    st.reached_pivots = nfs.reached;
    let Some(m_pi) = m_pi else {
        return Inner::Unsolved("border unreachable at this degree".into());
    };
    let t3 = Instant::now();
    let mut cp_muls = 0u64;
    let cp = charpoly_u16(&m_pi, p as u32, &mut cp_muls);
    st.charpoly_muls += cp_muls;
    st.charpoly_ms += t3.elapsed().as_secs_f64() * 1e3;
    let t4 = Instant::now();
    let ring = PolyRing::new(p);
    let lambdas = ring.roots(&cp, rng);
    st.roots_muls += ring.muls.get();
    st.rational_eigenvalues = lambdas.len();
    let mut mt = vec![vec![0u16; dim]; dim];
    for i in 0..dim {
        for j in 0..dim {
            mt[j][i] = m_pi[i][j] as u16;
        }
    }
    drop(m_pi);
    let mut others: Vec<Option<Vec<Vec<u64>>>> = vec![None; 3];
    let mut out = Vec::new();
    for lam in lambdas {
        let mut ev = 0u64;
        let ker = eigenvectors_u16(&mt, lam, p as u32, &mut ev);
        if ker.len() > 1 {
            st.degenerate_eigenspaces += 1;
        }
        // Split each group of more than one vector by σ₁, then σ₂, then σ₃,
        // keeping the vectors of each eigenvalue together.
        let mut groups = vec![ker];
        for (k, slot) in others.iter_mut().enumerate() {
            if groups.iter().all(|g| g.len() <= 1) {
                break;
            }
            if slot.is_none() {
                let mut nm = 0u64;
                *slot = mult_matrix(&mut nfs, k, &mut nm);
                st.nf_muls += nm;
            }
            let Some(mv) = slot.as_ref() else {
                st.eigenvector_muls += ev;
                return Inner::NeedHigher;
            };
            let mut next = Vec::new();
            for g in groups {
                if g.len() <= 1 {
                    next.push(g);
                } else {
                    next.extend(split_groups(&g, mv, p, rng, &mut ev));
                }
            }
            groups = next;
        }
        st.eigenvector_muls += ev;
        let mut vm = 0u64;
        let got = extract_points(&groups, lam, &idx, &comps, p, &mut vm);
        st.verify_muls += vm;
        match got {
            Ok(pts) => out.extend(pts),
            Err(why) => return Inner::Unresolved(why),
        }
    }
    st.eigen_ms += t4.elapsed().as_secs_f64() * 1e3;
    out.sort_unstable();
    out.dedup();
    st.e_solutions = out.len();
    st.outcome = "solved".into();
    Inner::Solved(out)
}

/// The weighted solve: at `d` (normally [`MACAULAY_BOUND`]), and again one
/// degree higher (up to `d + 2`) if a degenerate eigenspace of `M_π` needs a
/// weight-2 unknown to split or an eigenspace does not resolve to verified
/// rational points; past `d + 2` it reports unsolved, never a partial set.
/// Both runs are charged in `st`.  Returns the `F_p`-rational solutions
/// `(σ₁, σ₂, σ₃, π)` and the degree that produced them.
pub fn solve_weighted(
    comps: &[HashMap<[u8; 4], u64>],
    d: u16,
    p: u64,
    rng: &mut StdRng,
    st: &mut QuarticSolve,
    row_selection: bool,
    debug: bool,
) -> (Option<Vec<[u64; 4]>>, u16) {
    solve_weighted_traced(comps, d, p, rng, st, row_selection, debug, None, None)
}

/// [`solve_weighted`] with a shape trace at the base degree `d`: with
/// `replay = Some(t)` the rows `t` records as reducing to zero are not
/// built, and a replay that does not reach `t`'s leads, pivots and
/// dimension runs the full solve instead (both charged in `st`; a fallback
/// is never free).  With `record = Some(out)` the full solve's shape at `d`
/// is written to `out`.
#[allow(clippy::too_many_arguments)]
pub fn solve_weighted_traced(
    comps: &[HashMap<[u8; 4], u64>],
    d: u16,
    p: u64,
    rng: &mut StdRng,
    st: &mut QuarticSolve,
    row_selection: bool,
    debug: bool,
    replay: Option<&WeightedTrace>,
    mut record: Option<&mut WeightedTrace>,
) -> (Option<Vec<[u64; 4]>>, u16) {
    let t0 = Instant::now();
    let mut deg = d;
    let res = loop {
        // Count the degenerate eigenspaces of the last attempt only.
        st.degenerate_eigenspaces = 0;
        // The trace is for the base degree; a rerun above it is a full solve.
        let attempt = if deg == d && replay.is_some() {
            match solve_at(comps, deg, p, rng, st, row_selection, debug, replay, None) {
                Inner::Mismatch => {
                    st.replay = ReplayOutcome::FellBack;
                    None
                }
                other => {
                    st.replay = ReplayOutcome::Replayed;
                    Some(other)
                }
            }
        } else {
            None
        };
        let attempt = match attempt {
            Some(a) => a,
            None => solve_at(
                comps,
                deg,
                p,
                rng,
                st,
                row_selection,
                debug,
                None,
                if deg == d { record.take() } else { None },
            ),
        };
        match attempt {
            Inner::Solved(s) => break Some(s),
            Inner::Mismatch => unreachable!("a full solve has no trace to mismatch"),
            Inner::Unsolved(why) => {
                st.outcome = why;
                break None;
            }
            Inner::NeedHigher if deg < d + 2 => {
                if debug {
                    eprintln!(
                        "  degenerate eigenspace: rerun at weighted degree {}",
                        deg + 1
                    );
                }
                deg += 1;
            }
            Inner::NeedHigher => {
                st.outcome = "degenerate eigenspace unresolved".into();
                break None;
            }
            Inner::Unresolved(why) if deg < d + 2 => {
                if debug {
                    eprintln!("  {why}: rerun at weighted degree {}", deg + 1);
                }
                deg += 1;
            }
            Inner::Unresolved(why) => {
                st.outcome = format!("unresolved: {why}");
                break None;
            }
        }
    };
    st.total_ms = t0.elapsed().as_secs_f64() * 1e3;
    st.close_public();
    (res, deg)
}

/// The interpolation nodes for `z_R`.
const NODES: [u64; 9] = [2, 3, 4, 5, 6, 7, 8, 9, 10];

/// The 30 basis functions: `(kind, i, j)` with kind `0` = `e_ie_j`
/// (`i ≤ j`), `1` = `P₁e_i`, `2` = `P₂e_i`, `3` = `P₁P₂e_i`.
pub fn v4_basis() -> Vec<(u8, usize, usize)> {
    let mut v = Vec::new();
    for i in 0..5 {
        for j in i..5 {
            v.push((0, i, j));
        }
    }
    for kind in 1..4 {
        for i in 0..5 {
            v.push((kind, i, 0));
        }
    }
    v
}

/// `(e₀, …, e₄)` of `u`, `P₁`, `P₂` at a point `z ∈ (F_{p⁴}^*)⁴`.
fn invariants(f: &Fp4, z: &[E4; 4]) -> ([E4; 5], E4, E4) {
    let mut e = [E4::ZERO; 5];
    e[0] = E4::ONE;
    let (mut p1, mut p2) = (E4::ONE, E4::ONE);
    for zi in z {
        let inv = f.inv(zi);
        let u = f.add(&f.sq(zi), &f.sq(&inv));
        for k in (1..5).rev() {
            e[k] = f.add(&e[k], &f.mul(&e[k - 1], &u));
        }
        p1 = f.mul(&p1, &f.sub(zi, &inv));
        p2 = f.mul(&p2, &f.add(zi, &inv));
    }
    (e, p1, p2)
}

fn basis_values(f: &Fp4, z: &[E4; 4], basis: &[(u8, usize, usize)]) -> Vec<E4> {
    let (e, p1, p2) = invariants(f, z);
    let p12 = f.mul(&p1, &p2);
    basis
        .iter()
        .map(|&(kind, i, j)| match kind {
            0 => f.mul(&e[i], &e[j]),
            1 => f.mul(&p1, &e[i]),
            2 => f.mul(&p2, &e[i]),
            _ => f.mul(&p12, &e[i]),
        })
        .collect()
}

/// `H(z₁, …, z₄; z_R)` with the denominators cleared and `Πz_i⁴` divided out.
fn h_value(ki: &KleinInstance, curve: &Curve4, z: &[E4; 4], zr: &E4) -> Option<E4> {
    let f = &curve.f;
    let mut xs = [E4::ZERO; 5];
    let mut scale = E4::ONE;
    for (k, zi) in z.iter().chain(std::iter::once(zr)).enumerate() {
        xs[k] = ki.x_of_z(f, zi)?;
        let den = f.add(&f.mul(&ki.c, zi), &E4::ONE);
        let d2 = f.sq(&den);
        let d4 = f.sq(&d2);
        scale = f.mul(&scale, &f.sq(&d4));
        if k < 4 {
            let z2 = f.sq(zi);
            scale = f.mul(&scale, &f.inv(&f.sq(&z2)));
        }
    }
    Some(f.mul(&curve.s5(&xs), &scale))
}

/// `H = Σ_m coef[m](z_R) · m`, nine `F_{p⁴}` coefficients in `z_R` per basis
/// function.
pub struct SymmetrisedV4 {
    pub basis: Vec<(u8, usize, usize)>,
    pub coef: Vec<[E4; 9]>,
    pub checked_points: usize,
    pub precompute_muls: u64,
    /// Each basis function in `(e₂, e₃, P₁, P₂)` over `F_p`.
    pub expansions: Vec<HashMap<[u8; 4], u64>>,
}

type Poly4 = HashMap<[u8; 4], u64>;

fn pmul(a: &Poly4, b: &Poly4, p: u64) -> Poly4 {
    let mut out: Poly4 = HashMap::new();
    for (ma, &ca) in a {
        for (mb, &cb) in b {
            let m: [u8; 4] = core::array::from_fn(|k| ma[k] + mb[k]);
            let e = out.entry(m).or_insert(0);
            *e = am(*e, mm(ca, cb, p), p);
        }
    }
    out.retain(|_, v| *v != 0);
    out
}

fn mono(m: [u8; 4], c: u64) -> Poly4 {
    HashMap::from([(m, c)])
}

fn padd(a: &Poly4, b: &Poly4, p: u64) -> Poly4 {
    let mut out = a.clone();
    for (m, &c) in b {
        let e = out.entry(*m).or_insert(0);
        *e = am(*e, c, p);
    }
    out.retain(|_, v| *v != 0);
    out
}

/// `e₀, …, e₄` in `(e₂, e₃, P₁, P₂)`.
fn e_in_unknowns(p: u64) -> [Poly4; 5] {
    let i16 = inv_mod(16, p);
    let i2 = inv_mod(2, p);
    let e1 = padd(
        &padd(
            &mono([0, 0, 0, 2], i16),
            &mono([0, 0, 2, 0], (p - i16) % p),
            p,
        ),
        &mono([0, 1, 0, 0], (p - mm(4, i16, p)) % p),
        p,
    );
    let e4 = padd(
        &padd(&mono([0, 0, 2, 0], i2), &mono([0, 0, 0, 2], i2), p),
        &padd(&mono([1, 0, 0, 0], p - 4), &mono([0, 0, 0, 0], p - 16), p),
        p,
    );
    [
        mono([0, 0, 0, 0], 1),
        e1,
        mono([1, 0, 0, 0], 1),
        mono([0, 1, 0, 0], 1),
        e4,
    ]
}

impl SymmetrisedV4 {
    /// Interpolate on 30 random points of `(F_{p⁴}^*)⁴` and check on `checks`
    /// fresh ones.  `Err` if `H` is not in the span.
    pub fn precompute(
        ki: &KleinInstance,
        curve: &Curve4,
        rng: &mut StdRng,
        checks: usize,
    ) -> Result<SymmetrisedV4, String> {
        let f = &curve.f;
        let p = f.p;
        let before = f.muls();
        let basis = v4_basis();
        let nb = basis.len();
        let vinv: [[u64; 9]; 9] = {
            let mut m: Vec<Vec<u64>> = NODES
                .iter()
                .enumerate()
                .map(|(i, &k)| {
                    let mut row: Vec<u64> = (0..9).map(|j| pow_mod(k, j, p)).collect();
                    row.extend((0..9).map(|c| u64::from(c == i)));
                    row
                })
                .collect();
            let mut scratch = 0u64;
            rref_mod_p(&mut m, p, &mut scratch);
            core::array::from_fn(|j| core::array::from_fn(|k| m[j][9 + k]))
        };
        let random_z = |rng: &mut StdRng| -> [E4; 4] {
            core::array::from_fn(|_| loop {
                let v = f.random(rng);
                if !v.is_zero() && !f.add(&f.mul(&ki.c, &v), &E4::ONE).is_zero() {
                    break v;
                }
            })
        };
        let coefficients_in_last = |z: &[E4; 4]| -> Option<[E4; 9]> {
            let vals: Option<Vec<E4>> = NODES
                .iter()
                .map(|&k| h_value(ki, curve, z, &f.from_fp(k)))
                .collect();
            let vals = vals?;
            Some(core::array::from_fn(|j| {
                vals.iter()
                    .enumerate()
                    .fold(E4::ZERO, |acc, (k, v)| f.add(&acc, &f.scale(v, vinv[j][k])))
            }))
        };
        loop {
            let mut sys: Vec<Vec<E4>> = Vec::with_capacity(nb);
            while sys.len() < nb {
                let z = random_z(rng);
                let Some(rhs) = coefficients_in_last(&z) else {
                    continue;
                };
                let mut row = basis_values(f, &z, &basis);
                row.extend(rhs);
                sys.push(row);
            }
            let mut ok = true;
            for c in 0..nb {
                let Some(r) = (c..nb).find(|&r| !sys[r][c].is_zero()) else {
                    ok = false;
                    break;
                };
                sys.swap(r, c);
                let inv = f.inv(&sys[c][c]);
                let pivot: Vec<E4> = sys[c].iter().map(|v| f.mul(v, &inv)).collect();
                sys[c] = pivot.clone();
                for r in 0..nb {
                    if r == c || sys[r][c].is_zero() {
                        continue;
                    }
                    let k = sys[r][c];
                    for j in c..pivot.len() {
                        if !pivot[j].is_zero() {
                            let v = f.mul(&k, &pivot[j]);
                            sys[r][j] = f.sub(&sys[r][j], &v);
                        }
                    }
                }
            }
            if !ok {
                continue;
            }
            let coef: Vec<[E4; 9]> = (0..nb)
                .map(|r| core::array::from_fn(|j| sys[r][nb + j]))
                .collect();
            let mut t = 0;
            while t < checks {
                let z = random_z(rng);
                let Some(want) = coefficients_in_last(&z) else {
                    continue;
                };
                let bv = basis_values(f, &z, &basis);
                for j in 0..9 {
                    let got = bv
                        .iter()
                        .zip(&coef)
                        .fold(E4::ZERO, |acc, (m, c)| f.add(&acc, &f.mul(m, &c[j])));
                    if got != want[j] {
                        return Err(format!(
                            "H is not in the span of the V₄ invariants (check point {t}, coefficient z_R^{j})"
                        ));
                    }
                }
                t += 1;
            }
            let es = e_in_unknowns(p);
            let p1 = mono([0, 0, 1, 0], 1);
            let p2 = mono([0, 0, 0, 1], 1);
            let p12 = mono([0, 0, 1, 1], 1);
            let expansions = basis
                .iter()
                .map(|&(kind, i, j)| match kind {
                    0 => pmul(&es[i], &es[j], p),
                    1 => pmul(&p1, &es[i], p),
                    2 => pmul(&p2, &es[i], p),
                    _ => pmul(&p12, &es[i], p),
                })
                .collect();
            return Ok(SymmetrisedV4 {
                basis,
                coef,
                checked_points: checks,
                precompute_muls: f.muls() - before,
                expansions,
            });
        }
    }
}

/// The four Weil components of `H(·; z_R)` over `F_p` in `(e₂, e₃, P₁, P₂)`
/// (`F_{p⁴}` multiplications charged to `f`, the `F_p` combination to
/// `muls`).
pub fn v4_system(pre: &SymmetrisedV4, f: &Fp4, z_r: &E4, muls: &mut u64) -> Vec<Poly4> {
    let p = f.p;
    let mut pows = [E4::ONE; 9];
    for k in 1..9 {
        pows[k] = f.mul(&pows[k - 1], z_r);
    }
    let mut out: Vec<Poly4> = vec![HashMap::new(); 4];
    for (c, exp) in pre.coef.iter().zip(&pre.expansions) {
        let v = (0..9).fold(E4::ZERO, |acc, j| f.add(&acc, &f.mul(&c[j], &pows[j])));
        for (i, comp) in out.iter_mut().enumerate() {
            if v.0[i] == 0 {
                continue;
            }
            for (m, &e) in exp {
                let t = comp.entry(*m).or_insert(0);
                *t = am(*t, mm(v.0[i], e, p), p);
                *muls += 1;
            }
        }
    }
    for comp in out.iter_mut() {
        comp.retain(|_, v| *v != 0);
    }
    out
}

/// What recovering the decompositions from the invariant solutions cost.
#[derive(Clone, Debug, Default, Serialize)]
pub struct LiftBackV4 {
    pub muls: u64,
    pub split: usize,
    pub rooted: usize,
    pub group_additions: u64,
    /// Every decomposition found, as sorted `z`-quadruples (the `64`
    /// translates of each solution included).
    pub decompositions: Vec<[u64; 4]>,
}

/// Tonelli–Shanks in `F_p`, every multiplication counted (as
/// [`super::d4_weighted_s5`]'s).
fn sqrt_counted(n: u64, p: u64, muls: &mut u64) -> Option<u64> {
    let n = n % p;
    if n == 0 {
        return Some(0);
    }
    *muls += 2 * 64;
    if pow_mod(n, (p - 1) / 2, p) != 1 {
        return None;
    }
    let mut q = p - 1;
    let mut s = 0u32;
    while q.is_multiple_of(2) {
        q /= 2;
        s += 1;
    }
    let mut z = 2u64;
    while pow_mod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
        *muls += 2 * 64;
    }
    let (mut m, mut c, mut t, mut r) = (
        s,
        pow_mod(z, q, p),
        pow_mod(n, q, p),
        pow_mod(n, q.div_ceil(2), p),
    );
    *muls += 6 * 64;
    while t != 1 {
        let mut i = 0u32;
        let mut tt = t;
        while tt != 1 {
            tt = mm(tt, tt, p);
            *muls += 1;
            i += 1;
            if i == m {
                return None;
            }
        }
        let mut b = c;
        for _ in 0..(m - i - 1) {
            b = mm(b, b, p);
            *muls += 1;
        }
        m = i;
        c = mm(b, b, p);
        t = mm(t, c, p);
        r = mm(r, b, p);
        *muls += 3;
    }
    Some(r)
}

/// From `(e₂, e₃, P₁, P₂)`: `e₁, e₄`, the roots `u_i`, a `z_i` per root, the
/// coset of `V₄⁴` fixed by `P₁, P₂`, one group check (`±r` on `x`), and the
/// coset's translates.  Every multiplication charged.
pub fn lift_back_v4(
    ki: &KleinInstance,
    curve: &Curve4,
    sols: &[[u64; 4]],
    r: &Pt4,
    rng: &mut StdRng,
) -> LiftBackV4 {
    let f = &curve.f;
    let p = f.p;
    let mut lb = LiftBackV4::default();
    let before = f.muls();
    let mut fp = 0u64;
    let mut found: BTreeSet<[u64; 4]> = BTreeSet::new();
    let (i16, i2) = (inv_mod(16, p), inv_mod(2, p));
    for s in sols {
        let (e2, e3, q1, q2) = (s[0], s[1], s[2], s[3]);
        let (q1s, q2s) = (mm(q1, q1, p), mm(q2, q2, p));
        let e1 = mm(sm(sm(q2s, q1s, p), mm(4, e3, p), p), i16, p);
        let e4 = sm(sm(mm(am(q1s, q2s, p), i2, p), mm(4, e2, p), p), 16 % p, p);
        fp += 6;
        let Some(us) = split_quartic(&[e1, e2, e3, e4], p, rng, &mut fp) else {
            continue;
        };
        lb.split += 1;
        // One z per u: z² = (u ± √(u² − 4))/2.
        let zs: Option<Vec<u64>> = us
            .iter()
            .map(|&u| {
                let disc = sm(mm(u, u, p), 4 % p, p);
                fp += 1;
                let sd = sqrt_counted(disc, p, &mut fp)?;
                let z2 = mm(am(u, sd, p), i2, p);
                fp += 1;
                let z = sqrt_counted(z2, p, &mut fp)?;
                (z != 0).then_some(z)
            })
            .collect();
        let Some(zs) = zs else {
            continue;
        };
        lb.rooted += 1;
        // The coset: flip z₁ (−z, 1/z, −1/z) until P₁, P₂ match.
        let pv = |zs: &[u64], fp: &mut u64| -> (u64, u64) {
            let (mut a, mut b) = (1u64, 1u64);
            for &z in zs {
                let iz = inv_mod(z, p);
                a = mm(a, sm(z, iz, p), p);
                b = mm(b, am(z, iz, p), p);
                *fp += 2 + 2 * 64;
            }
            (a, b)
        };
        let alt = |z: u64, t: usize| -> u64 {
            match t {
                0 => z,
                1 => (p - z) % p,
                2 => inv_mod(z, p),
                _ => (p - inv_mod(z, p)) % p,
            }
        };
        let mut rep: Option<Vec<u64>> = None;
        for t in 0..4 {
            let mut c = zs.clone();
            c[0] = alt(c[0], t);
            if pv(&c, &mut fp) == (q1, q2) {
                rep = Some(c);
                break;
            }
        }
        let Some(rep) = rep else {
            continue;
        };
        let pts: Option<Vec<Pt4>> = rep
            .iter()
            .map(|&z| {
                ki.x_of_z(f, &f.from_fp(z))
                    .and_then(|x| curve.lift_x(&x, rng))
            })
            .collect();
        let Some(pts) = pts else {
            continue;
        };
        let mut hit = false;
        for sgn in 0..8u32 {
            let mut acc = pts[0];
            for (i, q) in pts.iter().enumerate().skip(1) {
                acc = if sgn >> (i - 1) & 1 == 1 {
                    curve.sub(&acc, q)
                } else {
                    curve.add(&acc, q)
                };
                lb.group_additions += 1;
            }
            if !acc.inf && acc.x == r.x {
                hit = true;
                break;
            }
        }
        if !hit {
            continue;
        }
        // The coset's translates: (t₁, …, t₄) ∈ V₄⁴ with t₁ ⊕ … ⊕ t₄ = 0
        // (V₄ as F₂²: 1 = (1,0) neg, 2 = (0,1) inv, 3 = both).
        for t1 in 0..4usize {
            for t2 in 0..4usize {
                for t3 in 0..4usize {
                    let t4 = t1 ^ t2 ^ t3;
                    let mut q = [
                        alt(rep[0], t1),
                        alt(rep[1], t2),
                        alt(rep[2], t3),
                        alt(rep[3], t4),
                    ];
                    q.sort_unstable();
                    let fixed = q
                        .iter()
                        .any(|&z| z == 1 || z == p - 1 || mm(z, z, p) == p - 1);
                    if !fixed && q.windows(2).all(|w| w[0] != w[1]) {
                        found.insert(q);
                    }
                }
            }
        }
    }
    lb.muls = fp + (f.muls() - before);
    lb.decompositions = found.into_iter().collect();
    lb
}

// ── E31: the chain solve (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md, E31) ───
//
// Additive.  Nothing above is changed; `solve_weighted_traced` stays the
// E24/E25 solver.

/// The Macaulay matrix at weighted degree `d`, triangularised and in
/// forward echelon, built by exactly the operations (and counts) of
/// `solve_at`'s first half.
#[allow(dead_code)] // `comps` and `std_index` serve the E31 solver
struct Built {
    cols: Vec<[u8; 4]>,
    col_index: HashMap<[u8; 4], usize>,
    comps: Vec<HashMap<[u8; 4], u64>>,
    ech: super::gaudry_quartic::Echelon,
    is_std: Vec<bool>,
    standard: Vec<usize>,
    std_index: Vec<u32>,
}

fn build_echelon(
    comps: &[HashMap<[u8; 4], u64>],
    d: u16,
    p: u64,
    st: &mut QuarticSolve,
    row_selection: bool,
) -> Result<Built, String> {
    assert!(p < 1 << 15, "rows are u16 and products must fit u32");
    let t0 = Instant::now();
    st.equation_degree = EQUATION_WDEG as u8;
    st.macaulay_degree = d as u8;
    let cols = weighted_monomials_desc(d);
    let col_index: HashMap<[u8; 4], usize> =
        cols.iter().enumerate().map(|(i, m)| (*m, i)).collect();
    let mut tri = 0u64;
    let (comps, leads) = triangularise(comps, p, &mut tri);
    st.triangularise_muls += tri;
    let divides = |lm: &[u8; 4], m: &[u8; 4]| (0..4).all(|k| lm[k] <= m[k]);
    let mut rows: Vec<Vec<u16>> = Vec::new();
    for (i, comp) in comps.iter().enumerate() {
        let Some(dg) = comp
            .iter()
            .filter(|(_, &v)| v != 0)
            .map(|(m, _)| wdeg(m))
            .max()
        else {
            continue;
        };
        if dg > d {
            return Err("an equation above the Macaulay degree".into());
        }
        for sh in weighted_monomials_desc(d - dg) {
            if row_selection
                && leads
                    .iter()
                    .take(i)
                    .any(|lm| lm.is_some_and(|lm| divides(&lm, &sh)))
            {
                continue;
            }
            let mut row = vec![0u16; cols.len()];
            for (&e, &c) in comp {
                let m: [u8; 4] = core::array::from_fn(|k| e[k] + sh[k]);
                let ci = col_index[&m];
                row[ci] = am(row[ci] as u64, c, p) as u16;
            }
            rows.push(row);
        }
    }
    st.rows = rows.len();
    st.cols = cols.len();
    st.build_ms += t0.elapsed().as_secs_f64() * 1e3;
    let t1 = Instant::now();
    let mut ech_muls = 0u64;
    let ech = echelon_by_lead(rows, p as u32, &mut ech_muls, false);
    st.echelon_muls += ech_muls;
    st.echelon_ms += t1.elapsed().as_secs_f64() * 1e3;
    st.zero_rows = ech.zero_rows;
    st.pivots = ech.pivot_row.iter().filter(|&&r| r != u32::MAX).count();
    let is_std: Vec<bool> = (0..cols.len())
        .map(|c| ech.pivot_row[c] == u32::MAX && wdeg(&cols[c]) < d)
        .collect();
    let standard: Vec<usize> = (0..cols.len()).filter(|&c| is_std[c]).collect();
    st.dim = standard.len();
    let mut std_index = vec![u32::MAX; cols.len()];
    for (i, &c) in standard.iter().enumerate() {
        std_index[c] = i as u32;
    }
    Ok(Built {
        cols,
        col_index,
        comps,
        ech,
        is_std,
        standard,
        std_index,
    })
}

/// The `P₂`-chains of the standard monomials: `heads[i]` and its length
/// `len[i]` (`h, P₂h, …, P₂^{len−1}h` standard, `P₂^{len}h` not), and the
/// column of each chain's border target `P₂^{len}h` (`None` if outside the
/// matrix).  Every standard monomial lies in exactly one chain.
struct Chains {
    heads: Vec<usize>,
    len: Vec<usize>,
    target: Vec<Option<usize>>,
}

fn p2_chains(b: &Built) -> Chains {
    let up = |c: usize| -> Option<usize> {
        let mut m = b.cols[c];
        m[EIG_VAR] += 1;
        b.col_index.get(&m).copied()
    };
    let down_is_std = |c: usize| -> bool {
        let mut m = b.cols[c];
        if m[EIG_VAR] == 0 {
            return false;
        }
        m[EIG_VAR] -= 1;
        b.col_index.get(&m).is_some_and(|&j| b.is_std[j])
    };
    let mut ch = Chains {
        heads: Vec::new(),
        len: Vec::new(),
        target: Vec::new(),
    };
    for &h in &b.standard {
        if down_is_std(h) {
            continue;
        }
        let mut k = 1usize;
        let mut c = h;
        let target = loop {
            match up(c) {
                Some(n) if b.is_std[n] => {
                    c = n;
                    k += 1;
                }
                other => break other,
            }
        };
        ch.heads.push(h);
        ch.len.push(k);
        ch.target.push(target);
    }
    ch
}

/// E31 step 1: the structure that prices the chain solve, on one system.
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChainStructure {
    pub dim: usize,
    pub pivots: usize,
    /// Number of `P₂`-chains (border targets needing a normal form).
    pub chains: usize,
    pub chain_lengths: Vec<usize>,
    /// Pivots reached from all targets together (what the existing normal
    /// forms resolve).
    pub reached_union: usize,
    /// `Σ_t |reached from t|`.
    pub reached_sum: usize,
    /// Nonzero entries right of the pivot on the union's rows, on
    /// non-standard and on standard columns.
    pub union_nnz_nonstd: u64,
    pub union_nnz_std: u64,
    /// `Σ_t Σ_{c reached from t}` (nonzeros right of the pivot of row `c`):
    /// an upper bound on a left-looking normal form's multiplications.
    pub left_looking_bound: u64,
    pub unreachable: bool,
}

/// [`ChainStructure`] of one system at weighted degree `d` (a diagnostic;
/// it charges nothing anywhere).
pub fn chain_structure(comps: &[HashMap<[u8; 4], u64>], d: u16, p: u64) -> Option<ChainStructure> {
    let mut st = QuarticSolve::default();
    let b = build_echelon(comps, d, p, &mut st, true).ok()?;
    let ch = p2_chains(&b);
    let ncols = b.cols.len();
    let mut out = ChainStructure {
        dim: b.standard.len(),
        pivots: st.pivots,
        chains: ch.heads.len(),
        chain_lengths: ch.len.clone(),
        ..Default::default()
    };
    let right = |c: usize| -> (u64, u64) {
        let row = &b.ech.rows[b.ech.pivot_row[c] as usize];
        let (mut ns, mut s) = (0u64, 0u64);
        for j in (c + 1)..ncols {
            if row[j] != 0 {
                if b.is_std[j] {
                    s += 1;
                } else {
                    ns += 1;
                }
            }
        }
        (ns, s)
    };
    let mut union = vec![false; ncols];
    for t in &ch.target {
        let Some(t) = *t else {
            out.unreachable = true;
            return Some(out);
        };
        let mut seen = vec![false; ncols];
        let mut stack = vec![t];
        seen[t] = true;
        while let Some(c) = stack.pop() {
            let pr = b.ech.pivot_row[c];
            if pr == u32::MAX {
                out.unreachable = true;
                return Some(out);
            }
            out.reached_sum += 1;
            union[c] = true;
            let (ns, s) = right(c);
            out.left_looking_bound += ns + s;
            let row = &b.ech.rows[pr as usize];
            for j in (c + 1)..ncols {
                if row[j] != 0 && !b.is_std[j] && !seen[j] {
                    seen[j] = true;
                    stack.push(j);
                }
            }
        }
    }
    for c in 0..ncols {
        if union[c] {
            out.reached_union += 1;
            let (ns, s) = right(c);
            out.union_nnz_nonstd += ns;
            out.union_nnz_std += s;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::quartic_folds::{generate_klein_instance, klein_base};
    use rand::SeedableRng;

    /// A planted sum of four base points from four distinct translation
    /// orbits comes back with its 64 translates, the quotient is 64, and
    /// every returned quadruple sums to `±R` in the group (§9.10, E22).
    #[test]
    fn v4_solve_returns_the_planted_orbit_of_decompositions() {
        let ki = generate_klein_instance(37, 1, 64).unwrap();
        let curve = ki.inst.curve();
        let f = &curve.f;
        let mut rng = StdRng::seed_from_u64(7);
        let fb = klein_base(&ki, &mut rng).unwrap();
        let pre = SymmetrisedV4::precompute(&ki, &curve, &mut rng, 4).unwrap();
        let mut pick: Vec<usize> = Vec::new();
        for i in 0..fb.points.len() {
            if pick.iter().all(|&j| fb.col[j] != fb.col[i]) {
                pick.push(i);
            }
            if pick.len() == 4 {
                break;
            }
        }
        assert_eq!(pick.len(), 4, "four distinct columns");
        let b = &fb.points;
        let r = curve.add(
            &curve.sub(&curve.add(&b[pick[0]], &b[pick[1]]), &b[pick[2]]),
            &b[pick[3]],
        );
        let z_r = ki.z_of_x(f, &r.x).unwrap();
        let mut fp = 0u64;
        let comps = v4_system(&pre, f, &z_r, &mut fp);
        let mut st = QuarticSolve::default();
        let (sols, _) = solve_weighted_traced(
            &comps,
            MACAULAY_BOUND,
            37,
            &mut rng,
            &mut st,
            true,
            false,
            None,
            None,
        );
        assert!(sols.is_some(), "{}", st.outcome);
        assert_eq!(st.dim, 64);
        let lb = lift_back_v4(&ki, &curve, &sols.unwrap(), &r, &mut rng);
        let z_of = |i: usize| ki.z_of_x(f, &b[i].x).unwrap().0[0];
        let mut planted: [u64; 4] = core::array::from_fn(|k| z_of(pick[k]));
        planted.sort_unstable();
        assert!(
            lb.decompositions.contains(&planted),
            "the planted quadruple is missing"
        );
        assert_eq!(
            lb.decompositions.len() % 64,
            0,
            "translates come in orbits of 64"
        );
    }
}
