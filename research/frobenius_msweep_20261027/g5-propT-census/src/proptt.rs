//! Proposition T, instance by instance, for the chained S3 system at any m.
//!
//!   system(sigma^j R, V)(c, sigma^j w) = sigma^j system(R, sigma^{-j} V)(c, w)      (same Boolean coordinates c)
//!
//! where system(R, B)(c, w) is the residual vector of the chained system with x_i = sum_k c_ik b_k (b_k the
//! basis B) and free auxiliary coordinates w. It holds iff the S3 coefficients are in F_2, i.e. b in F_2.
//! The same code path runs the Koblitz arm (b = 1), the ordinary-curve null (b random outside F_2, must fail)
//! and the wrong-direction control (basis sigma^{+j}V instead of sigma^{-j}V, must fail).

use crate::bases::*;
use crate::chain::{member, residuals};
use crate::field::Fq;
use crate::rng::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Arm {
    Koblitz,
    WrongDir,
    Ordinary,
    Stable,
}

impl Arm {
    pub fn name(&self) -> &'static str {
        match self {
            Arm::Koblitz => "koblitz",
            Arm::WrongDir => "wrongdir",
            Arm::Ordinary => "ordinary",
            Arm::Stable => "stable",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TrialOut {
    /// number of random assignments (of K) where the field-level identity failed
    pub eq_fail: u32,
    pub assigns: u32,
    /// number of assignments where the Boolean recombination (matrix of sigma^j on residual bits) failed
    pub bool_fail: u32,
    /// the n x n F_2 matrix of sigma^j is singular
    pub rec_singular: bool,
    /// planted chain solution was accepted by `member` on the source side (instrument check)
    pub plant_ok: Option<bool>,
    /// the conjugated planted solution is a chain solution for (sigma^j R) with the same b
    pub conj_member: Option<bool>,
    /// the conjugated legs lie in V (domain correspondence)
    pub dom_ok: Option<bool>,
    /// random (target, tuple) pair: member(source) == member(conjugate)
    pub neg_agree: Option<bool>,
    /// stable arm only: sigma^{-j} B lies in span(B) (change of coordinates exists)
    pub stable_cob: Option<bool>,
}

fn comb(f: &Fq, basis: &[u32], bits: u32) -> u32 {
    let _ = f;
    let mut x = 0u32;
    for (k, &b) in basis.iter().enumerate() {
        if (bits >> k) & 1 == 1 {
            x ^= b;
        }
    }
    x
}

/// Matrix of sigma^j as columns: image of each standard basis vector.
pub fn sigma_matrix(f: &Fq, j: u32) -> Vec<u32> {
    (0..f.n).map(|i| f.frob(1u32 << i, j)).collect()
}

pub fn apply_matrix(cols: &[u32], v: u32) -> u32 {
    let mut out = 0u32;
    for (i, &c) in cols.iter().enumerate() {
        if (v >> i) & 1 == 1 {
            out ^= c;
        }
    }
    out
}

/// Draw distinct nonzero legs from `elems` one at a time, each accepted only if S3(prev, x, .) has a root;
/// follow a random root. Returns (legs, x_R) with the chain satisfied, or None after 64 failed draws.
pub fn plant_adaptive(
    f: &Fq,
    b: u32,
    elems: &[u32],
    m: usize,
    rng: &mut Rng,
) -> Option<(Vec<u32>, u32)> {
    use crate::chain::{roots, Roots};
    let pick = |rng: &mut Rng, t: &Vec<u32>| -> u32 {
        loop {
            let x = elems[rng.below(elems.len() as u64) as usize];
            if x != 0 && !t.contains(&x) {
                return x;
            }
        }
    };
    if elems.len() < m + 2 {
        return None;
    }
    let mut t: Vec<u32> = vec![pick(rng, &vec![])];
    let mut prev = t[0];
    for i in 1..m {
        let mut found = None;
        for _ in 0..64 {
            let x = pick(rng, &t);
            let r = match roots(f, b, prev, x) {
                Roots::None | Roots::All => continue,
                Roots::One(v) => v,
                Roots::Two(v, w) => {
                    if rng.below(2) == 0 {
                        v
                    } else {
                        w
                    }
                }
            };
            found = Some((x, r));
            break;
        }
        let (x, r) = found?;
        t.push(x);
        prev = r;
        if i + 1 == m {
            return Some((t, r));
        }
    }
    None
}

/// One instance. `stable` supplies sigma-stable bases for the Stable arm (n = 17 only), else empty.
pub fn run_trial(
    f: &Fq,
    arm: Arm,
    m: usize,
    k_assign: usize,
    stable: &[Vec<u32>],
    rng: &mut Rng,
) -> Option<TrialOut> {
    let n = f.n;
    let b: u32 = match arm {
        Arm::Ordinary => loop {
            let c = f.rand(rng);
            if c > 1 {
                break c;
            }
        },
        _ => 1,
    };
    let basis: Vec<u32> = if arm == Arm::Stable {
        if stable.is_empty() {
            return None;
        }
        stable[rng.below(stable.len() as u64) as usize].clone()
    } else {
        // dimension l in lmin..=lmax where 2^lmin - 1 >= 2m, so that m distinct nonzero legs can be drawn
        let lmax = (n - 1).min(8);
        let mut lmin = 1u32;
        while (1u32 << lmin) - 1 < 2 * m as u32 {
            lmin += 1;
        }
        let lmin = lmin.min(lmax);
        let l = lmin + rng.below((lmax - lmin + 1) as u64) as u32;
        random_subspace(f, l, rng)
    };
    let l = basis.len();
    let xr = loop {
        let c = f.rand(rng);
        if c != 0 {
            break c;
        }
    };
    let j = 1 + rng.below((n - 1) as u64) as u32;
    let basis_src = match arm {
        Arm::WrongDir => frob_basis(f, &basis, j),
        _ => frob_basis(f, &basis, n - j),
    };
    let mut out = TrialOut::default();
    let mat = sigma_matrix(f, j);
    out.rec_singular = rank(&mat) < n as usize;
    let xr_j = f.frob(xr, j);
    for _ in 0..k_assign {
        let bits: Vec<u32> = (0..m).map(|_| rng.below(1u64 << l) as u32).collect();
        let w: Vec<u32> = (0..m.saturating_sub(2)).map(|_| f.rand(rng)).collect();
        let xs_dst: Vec<u32> = bits.iter().map(|&c| comb(f, &basis, c)).collect(); // system(sigma^j R, V)
        let xs_src: Vec<u32> = bits.iter().map(|&c| comb(f, &basis_src, c)).collect(); // system(R, sigma^-j V)
        let w_j: Vec<u32> = w.iter().map(|&u| f.frob(u, j)).collect();
        let left = residuals(f, b, xr_j, &xs_dst, &w_j);
        let right = residuals(f, b, xr, &xs_src, &w);
        out.assigns += 1;
        let mapped: Vec<u32> = right.iter().map(|&e| f.frob(e, j)).collect();
        if left != mapped {
            out.eq_fail += 1;
        }
        let bm: Vec<u32> = right.iter().map(|&e| apply_matrix(&mat, e)).collect();
        if left != bm {
            out.bool_fail += 1;
        }
    }
    if arm == Arm::Stable {
        out.stable_cob = Some(basis_src.iter().all(|&v| coords(&basis, v).is_some()));
    }
    // Solution level. Source side: tuples in W = span(basis_src); destination: sigma^j of them must be in V.
    let src_elems = span(&independent(&basis_src));
    let draw_tuple = |rng: &mut Rng| -> Vec<u32> {
        let mut t: Vec<u32> = vec![];
        let mut guard = 0;
        while t.len() < m && guard < 10_000 {
            guard += 1;
            let x = src_elems[rng.below(src_elems.len() as u64) as usize];
            if x != 0 && !t.contains(&x) {
                t.push(x);
            }
        }
        t
    };
    // positive: an adaptively planted chain solution with all legs in W (random legs, each accepted only if the
    // next S3 has a root, so that planting succeeds at every m); then its conjugate must be a solution too.
    let planted = plant_adaptive(f, b, &src_elems, m, rng);
    if let Some((t, xr_src)) = planted {
        out.plant_ok = Some(member(f, b, xr_src, &t));
        let t_dst: Vec<u32> = t.iter().map(|&x| f.frob(x, j)).collect();
        out.conj_member = Some(member(f, b, f.frob(xr_src, j), &t_dst));
        out.dom_ok = Some(t_dst.iter().all(|&x| coords(&basis, x).is_some()));
    }
    // random pair: membership of (target, tuple) equals membership of the conjugates
    let t = draw_tuple(rng);
    if t.len() == m {
        let xr_rand = f.rand(rng);
        let t_dst: Vec<u32> = t.iter().map(|&x| f.frob(x, j)).collect();
        if xr_rand != 0 {
            out.neg_agree =
                Some(member(f, b, xr_rand, &t) == member(f, b, f.frob(xr_rand, j), &t_dst));
        }
    }
    Some(out)
}

#[derive(Clone, Debug, Default)]
pub struct CellTally {
    pub trials: u64,
    pub assigns: u64,
    pub trials_eq_fail: u64,
    pub assigns_eq_fail: u64,
    pub trials_bool_fail: u64,
    pub rec_singular: u64,
    pub planted: u64,
    pub plant_bad: u64,
    pub conj_member_n: u64,
    pub conj_member_fail: u64,
    pub dom_n: u64,
    pub dom_fail: u64,
    pub neg_n: u64,
    pub neg_disagree: u64,
    pub stable_n: u64,
    pub stable_cob_fail: u64,
}

impl CellTally {
    pub fn add(&mut self, t: &TrialOut) {
        self.trials += 1;
        self.assigns += t.assigns as u64;
        self.assigns_eq_fail += t.eq_fail as u64;
        if t.eq_fail > 0 {
            self.trials_eq_fail += 1;
        }
        if t.bool_fail > 0 {
            self.trials_bool_fail += 1;
        }
        if t.rec_singular {
            self.rec_singular += 1;
        }
        if let Some(ok) = t.plant_ok {
            self.planted += 1;
            if !ok {
                self.plant_bad += 1;
            }
        }
        if let Some(ok) = t.conj_member {
            self.conj_member_n += 1;
            if !ok {
                self.conj_member_fail += 1;
            }
        }
        if let Some(ok) = t.dom_ok {
            self.dom_n += 1;
            if !ok {
                self.dom_fail += 1;
            }
        }
        if let Some(ok) = t.neg_agree {
            self.neg_n += 1;
            if !ok {
                self.neg_disagree += 1;
            }
        }
        if let Some(ok) = t.stable_cob {
            self.stable_n += 1;
            if !ok {
                self.stable_cob_fail += 1;
            }
        }
    }
}
