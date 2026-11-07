//! Reduction of the ECDLP to the matrix C (papers 1-4):
//! l = 3n' ; l random multiples n_j*G (rows P_j) and l random multiples -n'_i*Q
//! (rows Q_i), evaluated on the Riemann-Roch basis of L(l*O) (1, x, y, x^2, xy, ...;
//! pole orders 0,2,3,...,l). The left kernel of the 2l x l evaluation matrix M is the
//! same space as for the papers' complete degree-n' monomial matrix (a degree-n' form
//! restricted to E is exactly an element of L(3n' O)), only the columns are fewer.
//! With M = [M1; M2] (P rows over Q rows), K = [-C | I] where M2 = C*M1.
//! A zero s x s minor C[alpha|beta] gives l points that sum to O:
//!   P_j (j not in beta) and Q_i (i in alpha), hence m = sum n_j / sum n'_i.
use crate::curve::*;
use crate::rng::*;
use crate::field::*;

pub struct Kernel {
    pub l: usize,
    pub c: Vec<u64>,      // l x l row-major, row = Q index, col = P index
    pub np: Vec<u64>,     // scalars of the P rows
    pub nq: Vec<u64>,     // scalars of the Q rows (point is -nq*Q)
    pub n: u64,
}

impl Kernel {
    #[inline] pub fn at(&self, i: usize, j: usize) -> u64 { self.c[i * self.l + j] }
    /// Discrete-log candidate from a zero minor (rows alpha, cols beta). Integer bookkeeping mod n, not charged.
    pub fn recover(&self, alpha: &[usize], beta: &[usize]) -> Option<u64> {
        let mut inb = vec![false; self.l];
        for &j in beta { inb[j] = true; }
        let mut sp = 0u128; for j in 0..self.l { if !inb[j] { sp += self.np[j] as u128 } }
        let mut sq = 0u128; for &i in alpha { sq += self.nq[i] as u128 }
        let (sp, sq) = ((sp % self.n as u128) as u64, (sq % self.n as u128) as u64);
        if sq == 0 || alpha.is_empty() || beta.len() >= self.l { return None; }
        Some(((sp as u128 * crate::rho::modinv(sq, self.n) as u128) % self.n as u128) as u64)
    }
}

pub fn rr_row(f: &Fp, x: u64, y: u64, l: usize, out: &mut [u64]) {
    // x^0..x^{l/2} then y*x^0..y*x^{l/2-2}   (l even)  ; l odd handled by floor
    let nx = l / 2 + 1;
    let ny = l - nx;
    let mut xp = 1u64;
    for i in 0..nx { out[i] = xp; if i + 1 < nx { xp = f.mul(xp, x); } }
    let mut yp = y;
    for i in 0..ny { out[nx + i] = yp; if i + 1 < ny { yp = f.mul(yp, x); } }
}

/// Build C. None if M1 is singular (probability ~1/n; the attempt is lost).
pub fn build(c: &Curve, g: (u64, u64), q: (u64, u64), n: u64, nprime: usize, rng: &mut Rng) -> Option<Kernel> {
    let f = &c.f;
    let l = 3 * nprime;
    assert!(n as usize > 2 * l + 2 || n > 1_000_000);
    let mut used = std::collections::HashSet::new();
    let mut np = vec![0u64; l]; let mut nq = vec![0u64; l];
    let mut m1 = vec![0u64; l * l]; let mut m2 = vec![0u64; l * l];
    for j in 0..l {
        loop { let s = 1 + rng.below(n - 1); if used.insert(s) { np[j] = s; break; } }
        let pt = c.mul_scalar(np[j], Some(g))?;
        rr_row(f, pt.0, pt.1, l, &mut m1[j * l..(j + 1) * l]);
    }
    let mut usedq = std::collections::HashSet::new();
    for i in 0..l {
        loop { let s = 1 + rng.below(n - 1); if usedq.insert(s) { nq[i] = s; break; } }
        let pt = c.neg(c.mul_scalar(nq[i], Some(q)))?;
        rr_row(f, pt.0, pt.1, l, &mut m2[i * l..(i + 1) * l]);
    }
    // Solve M1^T Z = M2^T  (Z = C^T) by Gauss-Jordan on [M1^T | M2^T], l x 2l.
    let w = 2 * l;
    let mut a = vec![0u64; l * w];
    for r in 0..l { for cc in 0..l { a[r * w + cc] = m1[cc * l + r]; a[r * w + l + cc] = m2[cc * l + r]; } }
    for k in 0..l {
        let mut pr = k; while pr < l && a[pr * w + k] == 0 { pr += 1; }
        if pr == l { return None; }
        if pr != k { for cc in 0..w { a.swap(pr * w + cc, k * w + cc); } }
        let iv = f.inv(a[k * w + k]);
        for cc in k..w { a[k * w + cc] = f.mul(a[k * w + cc], iv); }
        for r in 0..l {
            if r == k { continue; }
            let fac = a[r * w + k];
            if fac == 0 { continue; }
            for cc in k + 1..w { let t = f.mul(fac, a[k * w + cc]); a[r * w + cc] = f.sub(a[r * w + cc], t); }
            a[r * w + k] = 0;
        }
    }
    // Z = right half: Z[r][s] = C^T[r][s] = C[s][r]
    let mut cm = vec![0u64; l * l];
    for r in 0..l { for s in 0..l { cm[s * l + r] = a[r * w + l + s]; } }
    Some(Kernel { l, c: cm, np, nq, n })
}

/// Development-only: check that l points of a claimed minor sum to O (uncounted curve ops on a fresh field).
pub fn support_sums_to_o(c: &Curve, g: (u64, u64), q: (u64, u64), k: &Kernel, alpha: &[usize], beta: &[usize]) -> bool {
    let mut inb = vec![false; k.l];
    for &j in beta { inb[j] = true; }
    let mut s: Pt = None;
    for j in 0..k.l { if !inb[j] { s = c.add(s, c.mul_scalar(k.np[j], Some(g))); } }
    for &i in alpha { s = c.add(s, c.neg(c.mul_scalar(k.nq[i], Some(q)))); }
    s.is_none()
}
