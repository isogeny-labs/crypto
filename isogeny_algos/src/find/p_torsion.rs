//! Couveignes' p-torsion algorithm (ANTS-II 1996, "Computing l-isogenies using the p-torsion";
//! the algorithm "C2" in De Feo, J. Number Theory 131, 2011) in characteristic p = 2: the
//! l-isogeny phi between two given ordinary binary curves E, E~ maps the cyclic group E[2^k]
//! (étale part) bijectively onto E~[2^k], so with generators P, P~ there is an odd lambda with
//! phi(P) = lambda P~, and the x-map N/D of phi interpolates x(iP) -> x(i lambda P~). Try each
//! lambda (up to sign) until the interpolation gives a rational function of the right shape.
//!
//! Here:
//!  * The points of order 2^k live in an Artin–Schreier tower over F_q: halving
//!    x(2Q) = x^2 + beta/x^2 (beta = 1/j) gives x^2 + sqrt(x_t) x + sqrt(beta) = 0, i.e.
//!    x = a w with a = sqrt(x_t), w^2 + w = sqrt(beta)/x_t; when that has no root in the current
//!    field a level K[w]/(w^2 + w + c) is adjoined (both curves have the same 2-adic unit root,
//!    so E~'s equations are always solvable in E's tower). Elements are vectors of 2^L
//!    coordinates over F_q, multiplied recursively; w^2 + w = c is solved by GF(2)-linear algebra.
//!  * Multiples: x(P + Q) + x(P - Q) = x(P) x(Q) / (x(P) + x(Q))^2.
//!  * Interpolation as a linear system over F_q: the normalised x-map is N/D with N odd monic of
//!    degree l and D = h^2 even monic of degree l - 1 (x I'(x) = I(x) in characteristic 2), so
//!    l - 1 unknowns; each point x(iP) gives the K-equation N(X) = Y D(X), whose coordinates are
//!    F_q-linear equations. One point per Frobenius orbit suffices. A wrong lambda is usually
//!    detected as an inconsistent system; a solution is verified (D a square, h dividing the
//!    l-division polynomial, the doubling test, the codomain's j-invariant).
//!
//! The interpolation is done by linear algebra rather than by Couveignes' fast interpolation in
//! the tower, so a trial costs O(l^3) operations in F_q instead of quasi-linear (De Feo's
//! implementation reaches Õ(l^2) overall); there are 2^(k-2) = O(l) trials.
use crate::binary::{self, BinCurve};
use crate::field::{Field, Rng};
use crate::gf2n::GF2n;
use crate::poly::{self, Poly};

/// Recursive Artin–Schreier tower K_L = F_q[w_1]..[w_L], w_i^2 + w_i = c_i with c_i in K_{i-1}.
/// An element of K_i is a vector of 2^i coordinates: (low half) + (high half) w_i.
struct Tower<'a> {
    f: &'a GF2n,
    cs: Vec<Vec<u64>>,
}

impl<'a> Tower<'a> {
    fn level(&self) -> usize {
        self.cs.len()
    }
    fn dim(&self) -> usize {
        1 << self.level()
    }
    fn embed(&self, a: &[u64]) -> Vec<u64> {
        let mut v = a.to_vec();
        v.resize(self.dim(), 0);
        v
    }
    fn scalar(&self, a: u64) -> Vec<u64> {
        self.embed(&[a])
    }
    fn add(a: &[u64], b: &[u64]) -> Vec<u64> {
        a.iter().zip(b).map(|(x, y)| x ^ y).collect()
    }
    fn mul_at(&self, a: &[u64], b: &[u64], lvl: usize) -> Vec<u64> {
        if lvl == 0 {
            return vec![self.f.mul(a[0], b[0])];
        }
        let h = 1 << (lvl - 1);
        let (a0, a1) = a.split_at(h);
        let (b0, b1) = b.split_at(h);
        let p0 = self.mul_at(a0, b0, lvl - 1);
        let p1 = self.mul_at(a1, b1, lvl - 1);
        let s = self.mul_at(&Self::add(a0, a1), &Self::add(b0, b1), lvl - 1);
        let lo = Self::add(&p0, &self.mul_at(&p1, &self.cs[lvl - 1], lvl - 1));
        let hi = Self::add(&s, &p0);
        [lo, hi].concat()
    }
    fn mul(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        self.mul_at(a, b, self.level())
    }
    fn sq_at(&self, a: &[u64], lvl: usize) -> Vec<u64> {
        if lvl == 0 {
            return vec![self.f.sq(a[0])];
        }
        let h = 1 << (lvl - 1);
        let (a0, a1) = a.split_at(h);
        let s0 = self.sq_at(a0, lvl - 1);
        let s1 = self.sq_at(a1, lvl - 1);
        let lo = Self::add(&s0, &self.mul_at(&s1, &self.cs[lvl - 1], lvl - 1));
        [lo, s1].concat()
    }
    fn sq(&self, a: &[u64]) -> Vec<u64> {
        self.sq_at(a, self.level())
    }
    fn inv_at(&self, a: &[u64], lvl: usize) -> Vec<u64> {
        if lvl == 0 {
            return vec![self.f.inv(a[0])];
        }
        let h = 1 << (lvl - 1);
        let (a0, a1) = a.split_at(h);
        let s = Self::add(a0, a1);
        let nrm = Self::add(
            &self.mul_at(a0, &s, lvl - 1),
            &self.mul_at(&self.sq_at(a1, lvl - 1), &self.cs[lvl - 1], lvl - 1),
        );
        let ni = self.inv_at(&nrm, lvl - 1);
        [self.mul_at(&s, &ni, lvl - 1), self.mul_at(a1, &ni, lvl - 1)].concat()
    }
    fn inv(&self, a: &[u64]) -> Vec<u64> {
        self.inv_at(a, self.level())
    }
    fn sqrt_at(&self, a: &[u64], lvl: usize) -> Vec<u64> {
        if lvl == 0 {
            return vec![self.f.sqrt(a[0]).expect("square roots exist in GF(2^n)")];
        }
        let h = 1 << (lvl - 1);
        let (a0, a1) = a.split_at(h);
        let v = self.sqrt_at(a1, lvl - 1);
        let u = self.sqrt_at(
            &Self::add(a0, &self.mul_at(a1, &self.cs[lvl - 1], lvl - 1)),
            lvl - 1,
        );
        [u, v].concat()
    }
    fn sqrt(&self, a: &[u64]) -> Vec<u64> {
        self.sqrt_at(a, self.level())
    }
    fn is_zero(a: &[u64]) -> bool {
        a.iter().all(|&x| x == 0)
    }
    /// a^q (q = 2^n): n squarings.
    fn frob(&self, a: &[u64]) -> Vec<u64> {
        let mut r = a.to_vec();
        for _ in 0..self.f.n {
            r = self.sq(&r);
        }
        r
    }
    /// A root of w^2 + w = c in the current field (GF(2)-linear algebra), or None.
    fn solve_as(&self, c: &[u64]) -> Option<Vec<u64>> {
        let n = self.f.n as usize;
        let dbits = n * self.dim();
        let words = dbits.div_ceil(64);
        let to_bits = |a: &[u64]| -> Vec<u64> {
            let mut v = vec![0u64; words];
            for (j, &x) in a.iter().enumerate() {
                for b in 0..n {
                    if (x >> b) & 1 == 1 {
                        let k = j * n + b;
                        v[k / 64] |= 1 << (k % 64);
                    }
                }
            }
            v
        };
        // echelon basis of the images T(e_k) = e_k^2 + e_k, with preimages
        let mut basis: Vec<(usize, Vec<u64>, Vec<u64>)> = vec![];
        for k in 0..dbits {
            let mut e = vec![0u64; self.dim()];
            e[k / n] = 1 << (k % n);
            let img = Self::add(&self.sq(&e), &e);
            let mut v = to_bits(&img);
            let mut pre = vec![0u64; words];
            pre[k / 64] |= 1 << (k % 64);
            for (piv, bv, bp) in &basis {
                if (v[piv / 64] >> (piv % 64)) & 1 == 1 {
                    for w in 0..words {
                        v[w] ^= bv[w];
                        pre[w] ^= bp[w];
                    }
                }
            }
            if let Some(w) = v.iter().position(|&x| x != 0) {
                let piv = w * 64 + v[w].trailing_zeros() as usize;
                basis.push((piv, v, pre));
            }
        }
        let mut r = to_bits(c);
        let mut sol = vec![0u64; words];
        for (piv, bv, bp) in &basis {
            if (r[piv / 64] >> (piv % 64)) & 1 == 1 {
                for w in 0..words {
                    r[w] ^= bv[w];
                    sol[w] ^= bp[w];
                }
            }
        }
        if r.iter().any(|&x| x != 0) {
            return None;
        }
        let mut out = vec![0u64; self.dim()];
        for k in 0..dbits {
            if (sol[k / 64] >> (k % 64)) & 1 == 1 {
                out[k / n] |= 1 << (k % n);
            }
        }
        Some(out)
    }
    /// Adjoin a root w of w^2 + w = c (c in the current field); returns w.
    fn push(&mut self, c: Vec<u64>) -> Vec<u64> {
        self.cs.push(c);
        let mut w = vec![0u64; self.dim()];
        w[self.dim() / 2] = 1;
        w
    }
}

/// Statistics of a successful run.
#[derive(Clone, Debug)]
pub struct C2Outcome {
    /// 2^k-torsion used
    pub k: u32,
    /// degree of the tower over F_q
    pub tower_degree: usize,
    /// lambdas tried (including the successful one)
    pub trials: usize,
}

/// Kernel polynomial of the l-isogeny from the ordinary binary curve e to a curve of
/// j-invariant jt (l odd, l > 1), by Couveignes' 2-torsion algorithm; verified as in
/// `small_char::binary_kernel`. None if no l-isogeny is found.
pub fn couveignes_binary(
    f: &GF2n,
    e: &BinCurve,
    jt: u64,
    ell: usize,
    rng: &mut Rng,
) -> Option<(Poly<GF2n>, C2Outcome)> {
    if ell < 3 || ell.is_multiple_of(2) {
        return None;
    }
    let beta = e.disc(f); // 1/j
    let betat = f.inv(jt);
    // 2^(k-1) >= l + 4 distinct abscissas among the multiples
    let mut k = 3u32;
    while (1usize << (k - 1)) < ell + 4 {
        k += 1;
    }
    let mut t = Tower { f, cs: vec![] };
    // 4-torsion: x = beta^(1/4)
    let mut x = t.scalar(f.sqrt(f.sqrt(beta)?)?);
    let mut xt = t.scalar(f.sqrt(f.sqrt(betat)?)?);
    let sb = f.sqrt(beta)?;
    let sbt = f.sqrt(betat)?;
    for _ in 3..=k {
        // E: extend the tower when needed
        let a = t.sqrt(&x);
        let c = t.mul(&t.scalar(sb), &t.inv(&x));
        let w = match t.solve_as(&c) {
            Some(w) => w,
            None => {
                let w = t.push(c);
                xt = t.embed(&xt);
                w
            }
        };
        x = t.mul(&t.embed(&a), &w);
        // E~: solvable in the same field
        let at = t.sqrt(&xt);
        let ct = t.mul(&t.scalar(sbt), &t.inv(&xt));
        let wt = t.solve_as(&ct)?;
        xt = t.mul(&at, &wt);
    }
    let half = 1usize << (k - 1);
    let modk = 1usize << k;
    // multiples x(iP), i = 0..half (index 0 unused)
    let multiples = |x1: &Vec<u64>, b: u64| -> Option<Vec<Vec<u64>>> {
        let mut xs = vec![vec![]; half + 1];
        xs[1] = x1.clone();
        let x1sq = t.sq(x1);
        xs[2] = Tower::add(&x1sq, &t.mul(&t.scalar(b), &t.inv(&x1sq)));
        for i in 2..half {
            let s = Tower::add(&xs[i], x1);
            if Tower::is_zero(&s) {
                return None;
            }
            let q = t.mul(&t.mul(&xs[i], x1), &t.inv(&t.sq(&s)));
            xs[i + 1] = Tower::add(&xs[i - 1], &q);
        }
        if !Tower::is_zero(&xs[half]) {
            return None; // not of order 2^k
        }
        Some(xs)
    };
    let xs = multiples(&x, beta)?;
    let xts = multiples(&xt, betat)?;
    let fold = |i: usize| {
        let r = i % modk;
        r.min(modk - r)
    };
    // Frobenius on E[2^k]: x(P)^q = x(lambda_F P)
    let xq = t.frob(&xs[1]);
    let lf = (1..half).find(|&i| xs[i] == xq)?;
    // one representative per Frobenius orbit among 1..half-1
    let mut seen = vec![false; half];
    let mut reps = vec![];
    for i in 1..half {
        if seen[i] {
            continue;
        }
        reps.push(i);
        let mut j = i;
        loop {
            seen[j] = true;
            j = fold(j * lf);
            if j == i || seen[j] {
                break;
            }
        }
    }
    let d = (ell - 1) / 2;
    // powers X^0..X^l per representative
    let pows: Vec<Vec<Vec<u64>>> = reps
        .iter()
        .map(|&i| {
            let mut p = vec![t.scalar(1)];
            for _ in 0..ell {
                p.push(t.mul(p.last().unwrap(), &xs[i]));
            }
            p
        })
        .collect();
    let unknowns = ell - 1; // n_1, n_3, .., n_{l-2}; d_0, d_2, .., d_{l-3}
    let psi = binary::division_poly(f, &e.normalized(f), ell);
    for (t_idx, lambda) in (1..half).step_by(2).enumerate() {
        let trials = t_idx + 1;
        // incremental elimination over F_q: rows (coefficients, rhs) in echelon form
        let mut echelon: Vec<(usize, Vec<u64>, u64)> = vec![];
        let mut consistent = true;
        'reps: for (ri, &i) in reps.iter().enumerate() {
            let y = &xts[fold(i * lambda)];
            let px = &pows[ri];
            let yx: Vec<Vec<u64>> = (0..d + 1).map(|m| t.mul(y, &px[2 * m])).collect();
            let rhs = Tower::add(&px[ell], &yx[d]);
            for m in 0..t.dim() {
                let mut row = vec![0u64; unknowns];
                for jj in 0..d {
                    row[jj] = px[2 * jj + 1][m];
                    row[d + jj] = yx[jj][m];
                }
                let mut b = rhs[m];
                for (piv, prow, pb) in &echelon {
                    let c = row[*piv];
                    if c != 0 {
                        for (x, &pv) in row.iter_mut().zip(prow) {
                            *x ^= f.mul(c, pv);
                        }
                        b ^= f.mul(c, *pb);
                    }
                }
                match row.iter().position(|&x| x != 0) {
                    Some(piv) => {
                        let inv = f.inv(row[piv]);
                        let row: Vec<u64> = row.iter().map(|&x| f.mul(x, inv)).collect();
                        let b = f.mul(b, inv);
                        // keep the echelon reduced: clear this pivot from earlier rows
                        for (_, prow, pb) in echelon.iter_mut() {
                            let c = prow[piv];
                            if c != 0 {
                                for (x, &rv) in prow.iter_mut().zip(&row) {
                                    *x ^= f.mul(c, rv);
                                }
                                *pb ^= f.mul(c, b);
                            }
                        }
                        echelon.push((piv, row, b));
                    }
                    None => {
                        if b != 0 {
                            consistent = false;
                            break 'reps;
                        }
                    }
                }
            }
        }
        if !consistent || echelon.len() < unknowns {
            continue;
        }
        let mut sol = vec![0u64; unknowns];
        for (piv, _, b) in &echelon {
            sol[*piv] = *b;
        }
        // D = x^{l-1} + sum d_{2m} x^{2m}: h_m = sqrt(d_{2m})
        let mut h = vec![0u64; d + 1];
        h[d] = 1;
        for m in 0..d {
            h[m] = f.sqrt(sol[d + m])?;
        }
        let hp: Poly<GF2n> = h;
        if poly::rem(f, &psi, &hp).iter().all(|&c| c == 0)
            && binary::kohel_codomain(f, e, &hp).j(f) == jt
            && binary::is_kernel(f, e, &hp, rng)
        {
            return Some((
                hp,
                C2Outcome {
                    k,
                    tower_degree: t.dim(),
                    trials,
                },
            ));
        }
    }
    None
}
