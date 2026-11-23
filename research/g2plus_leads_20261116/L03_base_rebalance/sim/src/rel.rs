use crate::jac::{pow_mod, MAX_GENUS};
use crate::la::Nat;
use crate::rng::Rng;
/// Residues modulo a monic `u` of degree `d ≤ 5`.
pub struct ModU {
    p: u64,
    d: usize,
    u: [u64; MAX_GENUS],
}

pub type Res = [u64; MAX_GENUS];

impl ModU {
    fn mul(&self, a: &Res, b: &Res, n: &mut Nat) -> Res {
        let (p, d) = (self.p, self.d);
        let mut t = [0u64; 2 * MAX_GENUS];
        for i in 0..d {
            for j in 0..d {
                t[i + j] = (t[i + j] + a[i] * b[j]) % p;
            }
        }
        n.mul += (d * d) as u64;
        n.add += (d * d) as u64;
        for k in (d..(2 * d).saturating_sub(1)).rev() {
            let c = t[k];
            for j in 0..d {
                t[k - d + j] = (t[k - d + j] + (p - self.u[j]) * c) % p;
            }
            n.mul += d as u64;
            n.add += d as u64;
        }
        let mut out = [0u64; MAX_GENUS];
        out[..d].copy_from_slice(&t[..d]);
        out
    }
    /// `a · (x + aa)`.
    fn mul_xpa(&self, a: &Res, aa: u64, n: &mut Nat) -> Res {
        let (p, d) = (self.p, self.d);
        let top = a[d - 1];
        let mut out = [0u64; MAX_GENUS];
        out[1..d].copy_from_slice(&a[..d - 1]);
        for j in 0..d {
            out[j] = (out[j] + (p - self.u[j]) * top + aa * a[j]) % p;
        }
        let work = if aa == 0 { d } else { 2 * d } as u64;
        n.mul += work;
        n.add += work;
        out
    }
    fn base(&self, aa: u64) -> Res {
        let mut b = [0u64; MAX_GENUS];
        if self.d == 1 {
            b[0] = (aa + self.p - self.u[0]) % self.p;
        } else {
            b[0] = aa;
            b[1] = 1;
        }
        b
    }
    /// `(x + aa)^e mod u`, `e ≥ 1`.
    fn pow_xpa(&self, aa: u64, e: u64, n: &mut Nat) -> Res {
        let mut res = self.base(aa);
        let bits = 64 - e.leading_zeros();
        for i in (0..bits - 1).rev() {
            res = self.mul(&res, &res, n);
            if (e >> i) & 1 == 1 {
                res = self.mul_xpa(&res, aa, n);
            }
        }
        res
    }
}

pub fn mod_u(p: u64, u: &[u64]) -> ModU {
    let d = u.len() - 1;
    let mut uu = [0u64; MAX_GENUS];
    uu[..d].copy_from_slice(&u[..d]);
    ModU { p, d, u: uu }
}

/// `u | x^p − x` for monic `u` of degree `1..=5`.
pub fn splits_completely(p: u64, u: &[u64], n: &mut Nat) -> bool {
    let ctx = mod_u(p, u);
    let xp = ctx.pow_xpa(0, p, n);
    xp[..ctx.d] == ctx.base(0)[..ctx.d]
}

pub fn vtrim(a: &mut Vec<u64>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

pub fn cinv(p: u64, a: u64, n: &mut Nat) -> u64 {
    // Fermat inverse, counted as its multiplications.
    let e = p - 2;
    n.mul += u64::from(64 - e.leading_zeros()) + u64::from(e.count_ones()) - 2;
    pow_mod(a, e, p)
}

pub fn vrem(p: u64, a: &[u64], m: &[u64], n: &mut Nat) -> Vec<u64> {
    let mut r = a.to_vec();
    vtrim(&mut r);
    let dm = m.len() - 1;
    let lead = m[dm];
    let li = if lead == 1 { 1 } else { cinv(p, lead, n) };
    while r.len() > dm {
        let top = r.len() - 1;
        let mut c = r[top];
        if li != 1 {
            c = c * li % p;
            n.mul += 1;
        }
        for j in 0..dm {
            let t = c * m[j] % p;
            let x = r[top - dm + j];
            r[top - dm + j] = if x >= t { x - t } else { x + p - t };
        }
        n.mul += dm as u64;
        n.add += dm as u64;
        r.pop();
        vtrim(&mut r);
    }
    r
}

pub fn vgcd_monic(p: u64, a: &[u64], b: &[u64], n: &mut Nat) -> Vec<u64> {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    vtrim(&mut a);
    vtrim(&mut b);
    while !b.is_empty() {
        let r = vrem(p, &a, &b, n);
        a = b;
        b = r;
    }
    if let Some(&lead) = a.last() {
        if lead != 1 {
            let il = cinv(p, lead, n);
            for c in a.iter_mut() {
                *c = *c * il % p;
            }
            n.mul += a.len() as u64;
        }
    }
    a
}

/// `a / m` for monic `m` dividing `a`.
pub fn vdiv_monic(p: u64, a: &[u64], m: &[u64], n: &mut Nat) -> Vec<u64> {
    let dm = m.len() - 1;
    let mut r = a.to_vec();
    let mut q = vec![0u64; a.len() - dm];
    for top in (dm..a.len()).rev() {
        let c = r[top];
        q[top - dm] = c;
        for j in 0..dm {
            let t = c * m[j] % p;
            let x = r[top - dm + j];
            r[top - dm + j] = if x >= t { x - t } else { x + p - t };
        }
        n.mul += dm as u64;
        n.add += dm as u64;
    }
    q
}

/// The roots of a monic `u` known to split completely (Cantor–Zassenhaus).
pub fn roots_of(p: u64, u: &[u64], rng: &mut Rng, n: &mut Nat) -> Option<Vec<u64>> {
    let d = u.len() - 1;
    if d == 0 {
        return Some(Vec::new());
    }
    if d == 1 {
        return Some(vec![(p - u[0]) % p]);
    }
    for _ in 0..64 {
        let aa = rng.range(0, p);
        let ctx = mod_u(p, u);
        let h = ctx.pow_xpa(aa, (p - 1) / 2, n);
        let mut hm1: Vec<u64> = h[..d].to_vec();
        hm1[0] = (hm1[0] + p - 1) % p;
        vtrim(&mut hm1);
        if hm1.is_empty() {
            continue;
        }
        let g = vgcd_monic(p, u, &hm1, n);
        let dg = g.len() - 1;
        if dg == 0 || dg == d {
            continue;
        }
        let q = vdiv_monic(p, u, &g, n);
        let mut out = roots_of(p, &g, rng, n)?;
        out.extend(roots_of(p, &q, rng, n)?);
        return Some(out);
    }
    None
}

/// `χ(disc u)` for monic `u` of degree `d ≥ 2`: `0` if `u` has a repeated
/// root, else `(−1)^{d−k}` with `k` the number of irreducible factors.
/// `chi[a]` is the quadratic character of `a`.  A pseudo-remainder chain on
/// `(u, u′)`: no inversion, the scalar factors enter through their characters.
pub fn disc_character(p: u64, u: &[u64], chi: &[i8], n: &mut Nat) -> i8 {
    let d = u.len() - 1;
    debug_assert!(d >= 2 && u[d] == 1);
    let mut a: Vec<u64> = u.to_vec();
    let mut b: Vec<u64> = (1..=d).map(|i| u[i] * (i as u64 % p) % p).collect();
    n.mul += d as u64;
    vtrim(&mut b);
    // disc(u) = (−1)^{d(d−1)/2} Res(u, u′)
    let mut sign: i8 = if (d * (d - 1) / 2) % 2 == 1 {
        chi[(p - 1) as usize]
    } else {
        1
    };
    loop {
        if b.is_empty() {
            return 0;
        }
        let m = a.len() - 1;
        let nn = b.len() - 1;
        if nn == 0 {
            return if m % 2 == 1 {
                sign * chi[b[0] as usize]
            } else {
                sign
            };
        }
        // pseudo-remainder: lc(b)^δ a = q b + r, δ = m − n + 1 steps.
        let lb = b[nn];
        let delta = m - nn + 1;
        let mut r = a.clone();
        for step in 0..delta {
            let top = m - step;
            let c = r[top];
            for x in r.iter_mut().take(top) {
                *x = *x * lb % p;
            }
            for j in 0..nn {
                let t = c * b[j] % p;
                let x = r[top - nn + j];
                r[top - nn + j] = if x >= t { x - t } else { x + p - t };
            }
            n.mul += (top + nn) as u64;
            n.add += nn as u64;
            r.truncate(top);
        }
        vtrim(&mut r);
        if r.is_empty() {
            return 0;
        }
        let dr = r.len() - 1;
        // Res(a, b) = (−1)^{mn} lc(b)^{m − dr − δn} Res(b, r)
        if (m + dr + delta * nn) % 2 == 1 {
            sign *= chi[lb as usize];
        }
        if (m * nn) % 2 == 1 {
            sign *= chi[(p - 1) as usize];
        }
        a = b;
        b = r;
    }
}
