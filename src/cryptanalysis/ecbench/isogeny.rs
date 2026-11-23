//! `transfer.isogeny`: move a prime-curve discrete logarithm across a
//! rational `ℓ`-isogeny, solve it on the isogenous curve, and check the
//! answer on the original.
//!
//! For `E : y² = x³ + ax + b` over `F_p` and a rational kernel
//! polynomial `h(x) = x − x₀` (`ℓ = 2`: `x₀` a root of `x³ + ax + b`;
//! `ℓ = 3`: a root of the 3-division polynomial `ψ₃ = 3x⁴ + 6ax² + 12bx −
//! a²`), Vélu's formulas (Washington, *Elliptic Curves*, Thm. 12.16) with
//! `g^x = 3x₀² + a`, `v = g^x` (`ℓ = 2`) or `2g^x` (`ℓ = 3`), `u = 4y₀² =
//! 4(x₀³ + ax₀ + b)` give
//!
//! ```text
//!   E′ : y² = x³ + (a − 5v)x + (b − 7(u + x₀v))
//!   φ(x, y) = ( x + v/(x − x₀) + u/(x − x₀)²,  y·(1 − 2u/(x − x₀)³ − v/(x − x₀)²) )
//! ```
//!
//! all over `F_p` even when the kernel points themselves are not (their
//! ordinates enter only through `y₀² ∈ F_p`).  `φ` is a homomorphism of
//! degree `ℓ`, so for `ℓ ≠ r` it maps `⟨G⟩` isomorphically onto the order-`r`
//! subgroup of `E′` and `Q = [k]G` becomes `φ(Q) = [k]φ(G)`.
//!
//! What is charged.  The isogenous curve is a different curve, but over
//! the same field and with the same group law, so its group operations
//! are GAE in the workload's own unit: the inner solve on `E′` is charged
//! as it would be on any curve, and the final check `[k]G = Q` on `E` is
//! one scalar multiplication.  Finding the kernel (`x^p mod ψ`, a gcd, a
//! root split) and evaluating `φ` twice are field operations, recorded as
//! native work in a `transfer` phase and left unpriced (AGENTS.md §5a
//! prices them across the band); they are `O(log p)` multiplications
//! against the solve's `O(√r)` group operations.

use crate::cryptanalysis::ic_boundary::{PrimeCurve, PrimePoint};

/// Field operations the transfer spends outside the group law.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FieldTally {
    pub muls: u64,
    pub invs: u64,
}

struct Fp<'t> {
    p: u64,
    tally: &'t mut FieldTally,
}

impl Fp<'_> {
    fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.tally.muls += 1;
        // Root-polynomial arithmetic uses canonical residues. Keep the
        // original wide remainder for external/noncanonical operands.
        if a < self.p && b < self.p {
            if self.p <= u16::MAX as u64 {
                return ((a as u32 * b as u32) % self.p as u32) as u64;
            }
            if self.p <= u32::MAX as u64 {
                return (a * b) % self.p;
            }
            // Fast reduction for p=2^61-1 (two end-around-carry limbs).
            if self.p == (1u64 << 61) - 1 {
                let product = a as u128 * b as u128;
                let folded = ((product as u64) & self.p) + (product >> 61) as u64;
                return if folded >= self.p {
                    folded - self.p
                } else {
                    folded
                };
            }
        }
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    fn add(&self, a: u64, b: u64) -> u64 {
        // p<=2^63-1 and canonical operands imply the sum fits in u64.
        if self.p <= u64::MAX / 2 && a < self.p && b < self.p {
            let sum = a + b;
            return if sum >= self.p { sum - self.p } else { sum };
        }
        ((a as u128 + b as u128) % self.p as u128) as u64
    }
    fn sub(&self, a: u64, b: u64) -> u64 {
        if self.p <= u64::MAX / 2 && a < self.p && b < self.p {
            return if a >= b { a - b } else { a + self.p - b };
        }
        ((a as u128 + self.p as u128 - b as u128) % self.p as u128) as u64
    }
    fn small(&self, c: u64) -> u64 {
        c % self.p
    }
    fn pow(&mut self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1 % self.p;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            e >>= 1;
            // Do not square the base after consuming the final exponent bit.
            if e != 0 {
                a = self.mul(a, a);
            }
        }
        r
    }
    fn inv(&mut self, a: u64) -> u64 {
        self.tally.invs += 1;
        // The 2- and 3-inverses occur frequently when normalizing
        // quadratic factors and the 3-division polynomial. These are
        // exact inverses in the odd prime fields used by this solver and
        // avoid a full Fermat exponentiation (O(log p) field products).
        // Compute via quotient/remainder to avoid 2*p+1 overflow.
        if a == 2 && self.p > 2 {
            return self.p / 2 + 1;
        }
        if a == 3 && self.p > 3 {
            return if self.p % 3 == 1 {
                2 * (self.p / 3) + 1
            } else {
                self.p / 3 + 1
            };
        }
        // Other inverses keep the original Fermat behavior; tally counts
        // an inversion, independently of how it is implemented.
        let mut scratch = FieldTally::default();
        let mut f = Fp {
            p: self.p,
            tally: &mut scratch,
        };
        f.pow(a, self.p - 2)
    }

    // For an odd prime p, Jacobi(a,p) is the Legendre symbol. Selecting
    // a Tonelli-Shanks nonresidue with binary reciprocity avoids running
    // an entire p-sized modular exponentiation for every trial candidate.
    fn jacobi_symbol(mut a: u64, mut n: u64) -> i8 {
        debug_assert!(n > 0 && n & 1 == 1);
        a %= n;
        let mut sign = 1i8;
        while a != 0 {
            let twos = a.trailing_zeros();
            if (twos & 1) != 0 && (n & 7 == 3 || n & 7 == 5) {
                sign = -sign;
            }
            a >>= twos;
            if a & 3 == 3 && n & 3 == 3 {
                sign = -sign;
            }
            let next = n % a;
            n = a;
            a = next;
        }
        if n == 1 { sign } else { 0 }
    }

    // Tonelli-Shanks for odd primes p = 1 (mod 4).
    //
    // Let p-1 = q*2^s, q odd. Compute h=a^((q-1)/2) only once, then
    // r=h*a=a^((q+1)/2), t=h^2*a=a^q. Use Jacobi(value,p) before the
    // nonresidue search: otherwise nonsquare inputs perform expensive
    // Tonelli setup and regress substantially relative to Euler rejection.
    fn sqrt_tonelli_shanks(&mut self, value: u64) -> Option<u64> {
        if value == 0 {
            return Some(0);
        }
        let p = self.p;
        // For an odd PRIME modulus, Jacobi(value,p) = Legendre(value,p).
        // Reject nonsquares before searching for an auxiliary nonresidue.
        if Self::jacobi_symbol(value, p) != 1 {
            return None;
        }
        // Atkin's single-exponent square-root formula for p ≡ 5 (mod 8).
        // For a quadratic residue a, with v=(2a)^((p-5)/8),
        // i=2a*v² is a square root of -1, and r=a*v*(i-1)
        // satisfies r²=a. Keep Jacobi rejection before this branch:
        // computing a candidate for nonsquares is a severe regression.
        if p & 7 == 5 {
            let twice = self.mul(2 % p, value);
            let v = self.pow(twice, (p - 5) / 8);
            let v_squared = self.mul(v, v);
            let i = self.mul(twice, v_squared);
            let av = self.mul(value, v);
            let factor = self.sub(i, 1);
            let candidate = self.mul(av, factor);
            return if self.mul(candidate, candidate) == value {
                Some(candidate)
            } else {
                None
            };
        }
        // High 2-adicity makes Tonelli-Shanks repeatedly scan and square
        // roots of unity. For s >= 20, Cipolla's Fp2 exponentiation is
        // faster on the tested NTT-friendly prime fields.
        if (p - 1).trailing_zeros() >= 20 {
            return self.sqrt_cipolla(value);
        }
        let s = (p - 1).trailing_zeros();
        let q = (p - 1) >> s;
        let mut z = 2u64;
        while Self::jacobi_symbol(z, p) != -1 {
            z += 1;
        }
        let mut c = self.pow(z, q);
        let h = self.pow(value, (q - 1) / 2);
        let h_squared = self.mul(h, h);
        let mut t = self.mul(h_squared, value);
        let mut r = self.mul(h, value);
        let mut m = s;
        while t != 1 {
            let mut i = 0u32;
            let mut tt = t;
            while tt != 1 && i < m {
                tt = self.mul(tt, tt);
                i += 1;
            }
            if i == m {
                return None;
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = self.mul(b, b);
            }
            r = self.mul(r, b);
            c = self.mul(b, b);
            t = self.mul(t, c);
            m = i;
        }
        Some(r)
    }

    // Multiplication in F_p[theta]/(theta^2-w), using three base-field
    // products plus w * (a1*b1). All inputs are canonical residues.
    #[inline]
    fn mul_quadratic_extension(
        &mut self,
        a: (u64, u64),
        b: (u64, u64),
        w: u64,
    ) -> (u64, u64) {
        let u = self.mul(a.0, b.0);
        let v = self.mul(a.1, b.1);
        let lhs = self.add(a.0, a.1);
        let rhs = self.add(b.0, b.1);
        let cross = self.mul(lhs, rhs);
        let wv = self.mul(w, v);
        let real = self.add(u, wv);
        let imag = self.sub(self.sub(cross, u), v);
        (real, imag)
    }

    #[inline]
    fn square_quadratic_extension(&mut self, a: (u64, u64), w: u64) -> (u64, u64) {
        let xx = self.mul(a.0, a.0);
        let yy = self.mul(a.1, a.1);
        let xy = self.mul(a.0, a.1);
        let wyy = self.mul(w, yy);
        (self.add(xx, wyy), self.add(xy, xy))
    }

    // Cipolla: for nonzero square a over an odd prime field, find t such
    // that w=t^2-a is a nonsquare; (t+theta)^((p+1)/2) lies in F_p and
    // squares to a, where theta^2=w. Verify before returning.
    //
    // The caller has already rejected nonresidues with the Jacobi symbol.
    // Only use this for fields with v2(p-1)>=20; ordinary Tonelli-Shanks
    // is faster at low 2-adicity.
    fn sqrt_cipolla(&mut self, a: u64) -> Option<u64> {
        let p = self.p;
        let mut t = 1u64;
        let w = loop {
            let tt = self.mul(t, t);
            let w = self.sub(tt, a);
            if Self::jacobi_symbol(w, p) == -1 {
                break w;
            }
            t += 1;
            if t >= p {
                // Defensive fallback for an invalid modulus.
                return None;
            }
        };
        let mut exponent = (p + 1) / 2;
        let mut base = (t, 1);
        let mut root = (1, 0);
        while exponent != 0 {
            if exponent & 1 != 0 {
                root = self.mul_quadratic_extension(root, base, w);
            }
            exponent >>= 1;
            if exponent != 0 {
                base = self.square_quadratic_extension(base, w);
            }
        }
        if root.1 != 0 {
            return None;
        }
        if self.mul(root.0, root.0) == a {
            Some(root.0)
        } else {
            None
        }
    }

    // Polynomials, lowest degree first.
    fn trim(v: &mut Vec<u64>) {
        while v.last() == Some(&0) {
            v.pop();
        }
    }

    // Root extraction repeatedly reduces modulo the same polynomial.
    // Convert it to monic once so each subsequent remainder avoids a
    // full Fermat inversion of the leading coefficient. The zero set is
    // unchanged because the scaling factor is nonzero in F_p.
    fn make_monic(&mut self, polynomial: &mut Vec<u64>) {
        if polynomial.len() <= 1 || polynomial.last() == Some(&1) {
            return;
        }
        let degree = polynomial.len() - 1;
        let inverse = self.inv(polynomial[degree]);
        for coefficient in &mut polynomial[..degree] {
            *coefficient = self.mul(*coefficient, inverse);
        }
        polynomial[degree] = 1;
    }

    fn rem(&mut self, a: &[u64], m: &[u64]) -> Vec<u64> {
        self.rem_owned(a.to_vec(), m)
    }

    // Polynomial multiplication already owns its coefficient buffer. Reduce
    // it in-place rather than cloning a second Vec for each powmod step.
    fn rem_owned(&mut self, mut r: Vec<u64>, m: &[u64]) -> Vec<u64> {
        Self::trim(&mut r);
        let dm = m.len() - 1;
        // Monic moduli dominate x^p mod f and Cantor-Zassenhaus splitting.
        // Their leading inverse is 1: avoid a Fermat inversion per remainder.
        let monic = m[dm] == 1;
        let inv = if monic { 1 } else { self.inv(m[dm]) };
        while r.len() > dm {
            let top = *r.last().unwrap();
            let c = if monic && top < self.p {
                top
            } else {
                self.mul(top, inv)
            };
            let s = r.len() - 1 - dm;
            // The leading coefficient cancels exactly, so do not multiply
            // and subtract it. Skip zero terms in sparse divisors.
            for (i, &mi) in m[..dm].iter().enumerate() {
                if mi != 0 {
                    let t = self.mul(c, mi);
                    r[s + i] = self.sub(r[s + i], t);
                }
            }
            r[s + dm] = 0;
            Self::trim(&mut r);
        }
        r
    }
    fn mulmod(&mut self, a: &[u64], b: &[u64], m: &[u64]) -> Vec<u64> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut t = vec![0u64; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                let v = self.mul(x, y);
                t[i + j] = self.add(t[i + j], v);
            }
        }
        self.rem_owned(t, m)
    }
    fn powmod(&mut self, base: &[u64], mut e: u64, m: &[u64]) -> Vec<u64> {
        let mut r = vec![1];
        let mut b = self.rem(base, m);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mulmod(&r, &b, m);
            }
            e >>= 1;
            // An unused final polynomial square is especially expensive.
            if e != 0 {
                b = self.mulmod(&b, &b, m);
            }
        }
        r
    }
    // Within Euclid's algorithm, remainders are needed only up to a
    // nonzero field scalar. Pseudo-division eliminates the leading term
    // without computing an inverse of every intermediate divisor:
    //   R <- lc(m) * R - lc(R) * x^shift * m.
    // This is NOT an exact modular remainder and must not be used by
    // powmod(), where the polynomial residue itself matters.
    fn pseudo_rem_owned(&mut self, mut r: Vec<u64>, m: &[u64]) -> Vec<u64> {
        Self::trim(&mut r);
        let degree = m.len() - 1;
        if degree == 0 {
            // A nonzero constant divides every polynomial.
            return Vec::new();
        }
        let leading = m[degree];
        while r.len() > degree {
            let len = r.len();
            let top = r[len - 1];
            let shift = len - degree - 1;
            if leading != 1 {
                for coefficient in &mut r[..len - 1] {
                    *coefficient = self.mul(*coefficient, leading);
                }
            }
            for (i, &mi) in m[..degree].iter().enumerate() {
                if mi != 0 {
                    let term = self.mul(top, mi);
                    r[shift + i] = self.sub(r[shift + i], term);
                }
            }
            r[len - 1] = 0;
            Self::trim(&mut r);
        }
        r
    }
    fn gcd(&mut self, a: &[u64], b: &[u64]) -> Vec<u64> {
        let mut a = a.to_vec();
        let mut b = b.to_vec();
        Self::trim(&mut a);
        Self::trim(&mut b);
        while !b.is_empty() {
            let r = self.pseudo_rem_owned(a, &b);
            a = b;
            b = r;
        }
        a
    }

    /// Every root in `F_p` of `f`, ascending: `g = gcd(f, x^p − x)`, then
    /// equal-degree splitting with `(x + δ)^((p−1)/2) − 1` for `δ = 0, 1, …`
    /// (deterministic).
    fn roots(&mut self, f: &[u64]) -> Vec<u64> {
        let p = self.p;
        let mut f = f.to_vec();
        Self::trim(&mut f);
        if f.len() <= 1 {
            return vec![];
        }
        self.make_monic(&mut f);
        // Degree one needs no Frobenius/GCD computation: a monic linear
        // polynomial x + c has exactly one rational root, -c.
        if f.len() == 2 {
            return vec![self.sub(0, f[0])];
        }
        // Every odd prime supports direct quadratic extraction. For
        // p = 1 (mod 4) use Tonelli-Shanks instead of a polynomial
        // Frobenius power, GCD, and splitting. Handle zero discriminants
        // before exponentiation (also faster for p = 3 mod 4).
        if f.len() == 3 && (p & 1) == 1 {
            let b_squared = self.mul(f[1], f[1]);
            let four_c = self.mul(4 % p, f[0]);
            let discriminant = self.sub(b_squared, four_c);
            let neg_b = self.sub(0, f[1]);
            // p <= 2^63-1, so odd v + p cannot overflow u64.
            let half = |v: u64| if v & 1 == 0 { v >> 1 } else { (v + p) >> 1 };
            if discriminant == 0 {
                return vec![half(neg_b)];
            }
            let square_root = if (p & 3) == 3 {
                // Negative discriminants occur in roughly half of random
                // quadratic queries. For <=32-bit primes a Jacobi test
                // is cheaper than the modular exponentiation it avoids.
                // This prefilter is intentionally absent from split():
                // there, a squarefree quadratic factor is known to split.
                if p <= u32::MAX as u64 && Self::jacobi_symbol(discriminant, p) != 1 {
                    return Vec::new();
                }
                Some(self.pow(discriminant, (p + 1) / 4))
            } else {
                self.sqrt_tonelli_shanks(discriminant)
            };
            let Some(s) = square_root else {
                return Vec::new();
            };
            if self.mul(s, s) != discriminant {
                return Vec::new();
            }
            let plus = self.add(neg_b, s);
            let minus = self.sub(neg_b, s);
            let mut roots = vec![half(plus), half(minus)];
            roots.sort_unstable();
            roots.dedup();
            return roots;
        }
        // A zero constant coefficient gives a known root x=0.
        // Strip its entire multiplicity before expensive Frobenius powers.
        // The public API reports distinct roots, not multiplicities.
        if f.len() >= 4 && f[0] == 0 {
            let mut valuation = 1usize;
            while valuation < f.len() - 1 && f[valuation] == 0 {
                valuation += 1;
            }
            let mut out = self.roots(&f[valuation..]);
            out.push(0);
            out.sort_unstable();
            out.dedup();
            return out;
        }
        // x^3 = c has exactly one solution over F_p when p = 2 mod 3,
        // since gcd(3,p-1)=1. The inverse of 3 modulo p-1 is (2p-1)/3.
        // The modulus is at most 2^63-1 in this implementation.
        if f.len() == 4 && f[1] == 0 && f[2] == 0 && p % 3 == 2 {
            let target = self.sub(0, f[0]);
            let cube_root = self.pow(target, (2 * p - 1) / 3);
            return vec![cube_root];
        }
        // An even quartic q(x) = x^4 + b*x^2 + c is quadratic in y=x^2.
        // Solve the two lower-degree problems instead of performing
        // degree-four Frobenius exponentiation, GCD and equal-degree splits.
        // The recursive quadratic solver returns every root in F_p, so this
        // also handles repeated roots and irreducible factors correctly.
        if f.len() == 5 && f[1] == 0 && f[3] == 0 && (p & 1) == 1 {
            let y_roots = self.roots(&[f[0], f[2], 1]);
            let mut out = Vec::new();
            for y in y_roots {
                let negative_y = self.sub(0, y);
                out.extend(self.roots(&[negative_y, 0, 1]));
            }
            out.sort_unstable();
            out.dedup();
            return out;
        }
        let xp = self.powmod(&[0, 1], p, &f);
        let mut h = xp;
        if h.len() < 2 {
            h.resize(2, 0);
        }
        h[1] = self.sub(h[1], 1);
        Self::trim(&mut h);
        let mut g = if h.is_empty() {
            f.clone()
        } else {
            self.gcd(&f, &h)
        };
        // Euclidean gcd does not necessarily return a monic polynomial.
        // Normalize before all subsequent equal-degree splitting.
        self.make_monic(&mut g);
        let mut out = Vec::new();
        self.split(g, &mut out, 0);
        out.sort_unstable();
        out.dedup();
        out
    }
    fn split(&mut self, g: Vec<u64>, out: &mut Vec<u64>, mut delta: u64) {
        let p = self.p;
        match g.len() {
            0 | 1 => {}
            2 => {
                // Every factor emitted by roots() is monic. Preserve a
                // fallback for nonmonic callers of this private helper.
                let neg = self.sub(0, g[0]);
                let root = if g[1] == 1 {
                    neg
                } else {
                    let inv = self.inv(g[1]);
                    self.mul(neg, inv)
                };
                out.push(root);
            }
            _ => {
                // A degree-two split factor can also be solved directly
                // for p = 1 (mod 4), avoiding another polynomial power/GCD.
                if g.len() == 3 && g[2] == 1 && (p & 1) == 1 {
                    let b_squared = self.mul(g[1], g[1]);
                    let four_c = self.mul(4 % p, g[0]);
                    let discriminant = self.sub(b_squared, four_c);
                    let neg_b = self.sub(0, g[1]);
                    let half = |v: u64| {
                        if v & 1 == 0 { v >> 1 } else { (v + p) >> 1 }
                    };
                    if discriminant == 0 {
                        out.push(half(neg_b));
                        return;
                    }
                    let square_root = if (p & 3) == 3 {
                        Some(self.pow(discriminant, (p + 1) / 4))
                    } else {
                        self.sqrt_tonelli_shanks(discriminant)
                    };
                    if let Some(s) = square_root {
                        if self.mul(s, s) == discriminant {
                            out.push(half(self.add(neg_b, s)));
                            out.push(half(self.sub(neg_b, s)));
                            return;
                        }
                    }
                }
                loop {
                let shift = self.small(delta);
                let e = self.powmod(&[shift, 1], (p - 1) / 2, &g);
                let mut e = e;
                if e.is_empty() {
                    e = vec![0];
                }
                e[0] = self.sub(e[0], 1);
                let mut d = self.gcd(&g, &e);
                delta += 1;
                if d.len() > 1 && d.len() < g.len() {
                    // GCD factors are only defined up to a field scalar.
                    // Make them monic before division and recursion: this
                    // eliminates repeated inversions within both branches.
                    self.make_monic(&mut d);
                    let other = {
                        // Exact division g / d, where d is now monic.
                        let mut q = vec![0u64; g.len() - d.len() + 1];
                        let mut r = g.clone();
                        while r.len() >= d.len() && !r.is_empty() {
                            let c = *r.last().unwrap();
                            let s = r.len() - d.len();
                            q[s] = c;
                            for (i, &di) in d[..d.len() - 1].iter().enumerate() {
                                if di != 0 {
                                    let t = self.mul(c, di);
                                    r[s + i] = self.sub(r[s + i], t);
                                }
                            }
                            r[s + d.len() - 1] = 0;
                            Self::trim(&mut r);
                        }
                        q
                    };
                    self.split(d, out, delta);
                    self.split(other, out, delta);
                    return;
                }
                if delta > 4 * 64 + p.min(1 << 20) {
                    return;
                }
                }
            },
        }
    }
}

/// A rational `ℓ`-isogeny `φ : E → E′` from a degree-1 kernel polynomial.
#[derive(Clone, Debug)]
pub struct Isogeny {
    pub ell: u64,
    pub x0: u64,
    pub u: u64,
    pub v: u64,
    pub source: PrimeCurve,
    pub target: PrimeCurve,
}

/// The rational `ℓ`-isogeny (`ℓ ∈ {2, 3}`) with the smallest kernel
/// abscissa, or `None` when `E` has none.
pub fn find_isogeny(e: &PrimeCurve, ell: u64, tally: &mut FieldTally) -> Option<Isogeny> {
    let p = e.p;
    let mut f = Fp { p, tally };
    let (a, b) = (e.a % p, e.b % p);
    // If b=0 and a!=0, psi3/3 = x^4 + 2a*x^2 - a^2/3.
    // The quadratic in y=x^2 has discriminant (16/3)*a^2.
    // For odd primes p>3, (3/p) = -1 exactly when p%12 is 5 or 7:
    // then even y does not exist and there is no rational 3-kernel.
    // This constant-time rejection avoids all polynomial factorization.
    if ell == 3 && p > 3 && b == 0 && a != 0 && matches!(p % 12, 5 | 7) {
        return None;
    }
    // The smallest rational kernel root is known without factoring:
    // ψ₂=x(x²+a) when b=0, and ψ₃=3x(x³+4b) when a=0.
    // Guard characteristic 2/3 and singular curves; preserve the generic
    // root solver for every other parameter set.
    let x0 = match ell {
        2 if p > 3 && b == 0 && a != 0 => 0,
        3 if p > 3 && a == 0 && b != 0 => 0,
        2 => *f.roots(&[b, a, 0, 1]).first()?,
        3 => {
            let a2 = f.mul(a, a);
            let c12 = f.small(12);
            let c6 = f.small(6);
            let t1 = f.mul(c12, b);
            let t2 = f.mul(c6, a);
            *f.roots(&[f.sub(0, a2), t1, t2, 0, f.small(3)]).first()?
        }
        _ => return None,
    };
    let x02 = f.mul(x0, x0);
    let c3 = f.small(3);
    let t3 = f.mul(c3, x02);
    let gx = f.add(t3, a);
    let v = if ell == 2 { gx } else { f.mul(2, gx) };
    let y0sq = {
        let t = f.mul(x02, x0);
        let ax = f.mul(a, x0);
        f.add(f.add(t, ax), b)
    };
    let u = if ell == 2 { 0 } else { f.mul(4, y0sq) };
    let v5 = f.mul(5 % p, v);
    let a1 = f.sub(a, v5);
    let x0v = f.mul(x0, v);
    let w = f.add(u, x0v);
    let w7 = f.mul(7 % p, w);
    let b1 = f.sub(b, w7);
    let target = PrimeCurve { p, a: a1, b: b1 };
    // Singular target: the polynomial was not a kernel (cannot happen for
    // a genuine root, but the transfer refuses rather than trusts).
    let disc = {
        let a1sq = f.mul(a1, a1);
        let a3 = f.mul(a1sq, a1);
        let b2 = f.mul(b1, b1);
        let l = f.mul(4 % p, a3);
        let r = f.mul(27 % p, b2);
        f.add(l, r)
    };
    if disc == 0 {
        return None;
    }
    Some(Isogeny {
        ell,
        x0,
        u,
        v,
        source: e.clone(),
        target,
    })
}

impl Isogeny {
    /// `φ(P)`; the identity for the identity and for a kernel point.
    pub fn map(&self, pt: PrimePoint, tally: &mut FieldTally) -> PrimePoint {
        if pt.infinity {
            return pt;
        }
        let p = self.source.p;
        let mut f = Fp { p, tally };
        let d = f.sub(pt.x, self.x0);
        if d == 0 {
            return PrimePoint::INFINITY;
        }
        let di = f.inv(d);
        let di2 = f.mul(di, di);
        let di3 = f.mul(di2, di);
        let vdi = f.mul(self.v, di);
        let udi2 = f.mul(self.u, di2);
        let x = f.add(f.add(pt.x, vdi), udi2);
        let u2 = f.mul(2 % p, self.u);
        let a = f.mul(u2, di3);
        let bb = f.mul(self.v, di2);
        let t = f.add(a, bb);
        let one_t = f.sub(1, t);
        let y = f.mul(pt.y, one_t);
        PrimePoint::affine(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::{find_prime_order_curve, CountedGroup, GroupOps};

    #[test]
    fn round139_even_quartics_match_exhaustive_roots() {
        for p in [3u64, 5, 7, 11, 13, 17, 19, 23, 31, 101] {
            let mut tally = FieldTally::default();
            let mut field = Fp { p, tally: &mut tally };
            for c0 in 0..p.min(23) {
                for c2 in 0..p.min(23) {
                    for scale in [1u64, 2, 3] {
                        let s = scale % p;
                        if s == 0 {
                            continue;
                        }
                        let poly = [
                            field.mul(s, c0), 0,
                            field.mul(s, c2), 0, s,
                        ];
                        let expected: Vec<u64> = (0..p).filter(|&x| {
                            let mut value = 0;
                            for &coefficient in poly.iter().rev() {
                                let prod = field.mul(value, x);
                                value = field.add(prod, coefficient);
                            }
                            value == 0
                        }).collect();
                        assert_eq!(field.roots(&poly), expected, "p={p}, poly={poly:?}");
                    }
                }
            }
        }
        for p in [65_521u64, 1_000_000_007, 1_000_000_009,
                  4_294_967_291, (1u64 << 61) - 1] {
            let mut tally = FieldTally::default();
            let mut field = Fp { p, tally: &mut tally };
            for i in 1..65u64 {
                let r = (i.wrapping_mul(7_919).wrapping_add(3)) % p;
                let s = (i.wrapping_mul(13_127).wrapping_add(11)) % p;
                let rsq = field.mul(r, r);
                let ssq = field.mul(s, s);
                let poly = [
                    field.mul(rsq, ssq), 0,
                    field.sub(0, field.add(rsq, ssq)), 0, 1,
                ];
                let mut expected = vec![r, field.sub(0, r), s, field.sub(0, s)];
                expected.sort_unstable();
                expected.dedup();
                assert_eq!(field.roots(&poly), expected, "p={p}, i={i}");
            }
        }
    }

    #[test]
    fn round139_no_rational_three_kernel_when_three_nonsquare() {
        for p in [5u64, 7, 17, 19, 29, 31, 1_000_000_007,
                  (1u64 << 61) - 1] {
            assert!(matches!(p % 12, 5 | 7));
            for a in 1..64u64 {
                let mut tally = FieldTally::default();
                let curve = PrimeCurve { p, a, b: 0 };
                assert!(find_isogeny(&curve, 3, &mut tally).is_none(),
                    "p={p}, a={a}");
            }
        }
        // Positive controls: the no-kernel optimization must not be
        // applied when 3 is a quadratic residue.
        for p in [11u64, 13, 23, 37, 65_521, 4_294_967_291] {
            assert!(!matches!(p % 12, 5 | 7));
            for a in 1..32u64 {
                let mut tally = FieldTally::default();
                let mut field = Fp { p, tally: &mut tally };
                let poly = [
                    { let aa = field.mul(a, a); field.sub(0, aa) }, 0,
                    field.mul(6 % p, a), 0, 3,
                ];
                let roots = field.roots(&poly);
                let iso = find_isogeny(&PrimeCurve { p, a, b: 0 }, 3, &mut tally);
                assert_eq!(iso.map(|x| x.x0), roots.first().copied(),
                           "p={p}, a={a}");
            }
        }
    }

    #[test]
    fn round138_sparse_polynomial_roots_match_bruteforce() {
        for p in [5u64, 7, 11, 13, 17, 19, 23, 31, 101] {
            let mut tally = FieldTally::default();
            let mut f = Fp { p, tally: &mut tally };
            for c in 0..p {
                let polynomials = [
                    vec![c, 0, 0, 1], // binomial cubic
                    vec![0, c, 0, 0, 1], // ψ3-like, known zero root
                    vec![0, 0, c, 0, 1], // repeated zero root
                    vec![0, 0, 0, c, 1],
                    vec![0, 0, 0, 0, 1],
                ];
                for poly in polynomials {
                    let found = f.roots(&poly);
                    let mut expected = Vec::new();
                    for x in 0..p {
                        let mut value = 0;
                        for &coefficient in poly.iter().rev() {
                            let product = f.mul(value, x);
                            value = f.add(product, coefficient);
                        }
                        if value == 0 {
                            expected.push(x);
                        }
                    }
                    assert_eq!(found, expected, "p={p}, poly={poly:?}");
                }
            }
        }
        for p in [1_000_000_007u64, 4_294_967_291] {
            let mut tally = FieldTally::default();
            let mut f = Fp { p, tally: &mut tally };
            assert_eq!(p % 3, 2);
            for c in 0..1000u64 {
                let roots = f.roots(&[c, 0, 0, 1]);
                assert_eq!(roots.len(), 1);
                let r = roots[0];
                let r2 = f.mul(r, r);
                let r3 = f.mul(r2, r);
                assert_eq!(f.add(r3, c), 0);
            }
        }
    }

    #[test]
    fn round138_known_isogeny_kernel_is_zero() {
        for p in [5u64, 7, 11, 13, 17, 101, 65_521] {
            for t in 1..p.min(20) {
                for (ell, a, b) in [(2u64, t, 0u64), (3, 0, t)] {
                    let mut tally = FieldTally::default();
                    let iso = find_isogeny(&PrimeCurve { p, a, b }, ell, &mut tally)
                        .expect("nonsingular curve has rational kernel");
                    assert_eq!(iso.x0, 0, "p={p}, ell={ell}");
                }
            }
        }
    }

    #[test]
    fn round128_arithmetic_matches_wide_modulo() {
        let primes = [101u64, 65_521, 1_000_000_007, 4_294_967_291, (1u64 << 61) - 1];
        for p in primes {
            let mut tally = FieldTally::default();
            let mut f = Fp { p, tally: &mut tally };
            let mut state = p ^ 0x9e37_79b9_7f4a_7c15;
            for _ in 0..20_000 {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let a = state % p;
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let b = state % p;
                assert_eq!(f.mul(a, b), (a as u128 * b as u128 % p as u128) as u64);
                assert_eq!(f.add(a, b), ((a as u128 + b as u128) % p as u128) as u64);
                assert_eq!(f.sub(a, b), ((a as u128 + p as u128 - b as u128) % p as u128) as u64);
            }
        }
    }

    #[test]
    fn round128_remainders_and_powers_match_naive_division() {
        fn old_rem(f: &mut Fp<'_>, a: &[u64], m: &[u64]) -> Vec<u64> {
            let mut r = a.to_vec();
            Fp::trim(&mut r);
            let dm = m.len() - 1;
            let inv = f.inv(m[dm]);
            while r.len() > dm {
                let c = f.mul(*r.last().unwrap(), inv);
                let shift = r.len() - 1 - dm;
                for (i, &mi) in m.iter().enumerate() {
                    let t = f.mul(c, mi);
                    r[shift + i] = f.sub(r[shift + i], t);
                }
                Fp::trim(&mut r);
            }
            r
        }
        for p in [101u64, 65_521, 1_000_000_007, (1u64 << 61) - 1] {
            let mut tally = FieldTally::default();
            let mut f = Fp { p, tally: &mut tally };
            let mut state = p ^ 0xabc1_2500_0000_0028;
            for i in 0..250 {
                let mut dividend = Vec::new();
                for _ in 0..8 {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    dividend.push(state % p);
                }
                let modulus = match i % 4 {
                    0 => vec![3, 0, 0, 1],
                    1 => vec![3, 2, 4, 1],
                    2 => vec![3, 0, 0, 2],
                    _ => vec![3, 2, 4, 7],
                };
                let got = f.rem(&dividend, &modulus);
                let expected = old_rem(&mut f, &dividend, &modulus);
                assert_eq!(got, expected);
                if i < 15 {
                    let optimized = f.powmod(&[1, 1], p, &modulus);
                    // Independent square-and-multiply with the original reducer.
                    let mut exponent = p;
                    let mut acc = vec![1];
                    let mut base = old_rem(&mut f, &[1, 1], &modulus);
                    while exponent > 0 {
                        if exponent & 1 == 1 {
                            let product = f.mulmod(&acc, &base, &modulus);
                            acc = old_rem(&mut f, &product, &modulus);
                        }
                        exponent >>= 1;
                        if exponent != 0 {
                            let square = f.mulmod(&base, &base, &modulus);
                            base = old_rem(&mut f, &square, &modulus);
                        }
                    }
                    assert_eq!(optimized, acc);
                }
            }
        }
    }

    /// On prime-order curves with a rational 3-isogeny, φ lands on E′, is a
    /// homomorphism on ⟨G⟩, and keeps the order.
    #[test]
    fn a_three_isogeny_is_a_homomorphism_onto_the_target() {
        let mut found = 0;
        for seed in 0..60u64 {
            let inst = find_prime_order_curve(16, seed);
            let mut t = FieldTally::default();
            let Some(phi) = find_isogeny(&inst.curve, 3, &mut t) else {
                continue;
            };
            found += 1;
            assert!(t.muls > 0);
            let g = inst.generator_point();
            let mut ops = GroupOps::default();
            let e = &inst.curve;
            let e1 = &phi.target;
            let pg = phi.map(g, &mut t);
            assert!(e1.is_on_curve(pg), "seed {seed}: φ(G) is on E′");
            assert!(!pg.infinity);
            for k in [2u64, 3, 17, inst.r - 1, inst.r / 2 + 5] {
                let kg = e.mul(&mut ops, g, k);
                assert_eq!(
                    phi.map(kg, &mut t),
                    e1.mul(&mut ops, pg, k),
                    "seed {seed}, k {k}"
                );
            }
            assert!(e1.mul(&mut ops, pg, inst.r).infinity);
        }
        assert!(
            found >= 5,
            "only {found} curves of 60 had a rational 3-isogeny"
        );
    }

    #[test]
    fn roots_are_every_rational_root() {
        let p = 101u64;
        let mut t = FieldTally::default();
        let mut f = Fp { p, tally: &mut t };
        // (x − 3)(x − 50)(x² + 2) (x² + 2 irreducible mod 101: −2 is a non-residue).
        let poly = {
            let a = [p - 3, 1];
            let b = [p - 50, 1];
            let c = [2, 0, 1];
            let ab = f.mulmod(&a, &b, &[0, 0, 0, 0, 0, 0, 1]);
            f.mulmod(&ab, &c, &[0, 0, 0, 0, 0, 0, 1])
        };
        assert_eq!(f.roots(&poly), vec![3, 50]);
    }

    #[test]
    fn round131_quadratic_fast_split_matches_exhaustive_roots() {
        // Cover p = 3 (mod 4) with two distinct roots, a repeated root,
        // nonresidue discriminants, and p = 1 (mod 4) fallback cases.
        for p in [3u64, 5, 7, 11, 13, 19, 23, 31, 101] {
            let mut tally = FieldTally::default();
            let mut field = Fp { p, tally: &mut tally };
            for b in 0..p {
                for c in 0..p {
                    let polynomial = [c, b, 1];
                    let expected: Vec<u64> = (0..p).filter(|&x| {
                        let x2 = field.mul(x, x);
                        let bx = field.mul(b, x);
                        let x2_bx = field.add(x2, bx);
                        field.add(x2_bx, c) == 0
                    }).collect();
                    assert_eq!(field.roots(&polynomial), expected,
                        "p={p}, b={b}, c={c}");
                }
            }
        }
    }


    #[test]
    fn round132_low_degree_paths_match_linear_quadratic_oracles() {
        // Small fields: all monic quadratics, and nonmonic scaled linear
        // polynomials with padded trailing zeros.
        for p in [3u64, 5, 7, 11, 13, 19, 23, 31, 101] {
            let mut tally = FieldTally::default();
            let mut field = Fp { p, tally: &mut tally };
            for a in 1..p {
                for b in 0..p {
                    let expected: Vec<u64> = (0..p).filter(|&x| {
                        let ax = field.mul(a, x);
                        field.add(ax, b) == 0
                    }).collect();
                    assert_eq!(field.roots(&[b, a, 0, 0]), expected,
                        "linear p={p} a={a} b={b}");
                }
            }
            for b in 0..p {
                for c in 0..p {
                    let expected: Vec<u64> = (0..p).filter(|&x| {
                        let x_squared = field.mul(x, x);
                        let bx = field.mul(b, x);
                        field.add(field.add(x_squared, bx), c) == 0
                    }).collect();
                    for scalar in [1u64, 2, 3] {
                        let poly = [field.mul(c, scalar), field.mul(b, scalar),
                                    scalar % p];
                        if scalar % p == 0 { continue; }
                        assert_eq!(field.roots(&poly), expected,
                            "quadratic p={p} b={b} c={c} scale={scalar}");
                    }
                }
            }
        }

        for p in [65_521u64, 1_000_000_007, 4_294_967_291, (1u64 << 61) - 1] {
            let mut tally = FieldTally::default();
            let mut field = Fp { p, tally: &mut tally };
            for i in 0..80u64 {
                let root = (31 + i * 7919) % p;
                let leading = [1u64, 3, 7, p - 1][(i as usize) % 4];
                let constant = field.sub(0, field.mul(leading, root));
                assert_eq!(field.roots(&[constant, leading, 0]), vec![root]);
                let r2 = (root + 1237) % p;
                let sum = field.add(root, r2);
                let product = field.mul(root, r2);
                let mut expected = vec![root, r2];
                expected.sort_unstable();
                expected.dedup();
                let quadratic = [product, field.sub(0, sum), 1];
                assert_eq!(field.roots(&quadratic), expected);
                // Repeated roots must not be returned twice.
                let repeated_product = field.mul(root, root);
                let twice_root = field.add(root, root);
                let repeated = [repeated_product, field.sub(0, twice_root), 1];
                assert_eq!(field.roots(&repeated), vec![root]);
            }
        }
    }

    #[test]
    fn round134_jacobi_and_shared_powers_match_legendre_oracle() {
        // The Jacobi identity relies on odd PRIME moduli. Test a range of
        // 2-adic valuations, including 2^23 and 2^27 dividing p-1.
        for p in [5u64, 17, 97, 257, 65_521, 998_244_353,
                  1_000_000_009, 2_013_265_921] {
            let mut tally = FieldTally::default();
            let mut field = Fp { p, tally: &mut tally };
            let max_value = if p < 300 { p } else { 160 };
            for i in 0..max_value {
                let a = if p < 300 { i } else { (i * 79_919 + 13) % p };
                let expected_residue = a == 0 || field.pow(a, (p - 1) / 2) == 1;
                let got = field.sqrt_tonelli_shanks(a);
                assert_eq!(got.is_some(), expected_residue, "p={p} a={a}");
                if let Some(root) = got {
                    assert_eq!(field.mul(root, root), a, "p={p} a={a}");
                }
                if a != 0 {
                    let legendre = field.pow(a, (p - 1) / 2);
                    let expected_sign = if legendre == 1 { 1 } else { -1 };
                    assert_eq!(Fp::jacobi_symbol(a, p), expected_sign,
                               "Jacobi p={p} a={a}");
                }
            }
        }
    }

    #[test]
    fn round133_tonelli_shanks_quadratic_roots_and_nonresidues() {
        // p=1 mod 4 exercises Tonelli-Shanks; p=3 mod 4 checks the
        // previously-fast path and the new zero-discriminant case.
        for p in [5u64, 13, 17, 29, 101, 65_521, 1_000_000_009,
                  1_000_000_007, 4_294_967_291, (1u64 << 61) - 1] {
            let mut tally = FieldTally::default();
            let mut f = Fp { p, tally: &mut tally };
            for i in 0..64u64 {
                let r1 = (3 + 139 * i) % p;
                let r2 = (11 + 127 * i) % p;
                let sum = f.add(r1, r2);
                let product = f.mul(r1, r2);
                let monic = [product, f.sub(0, sum), 1];
                let mut expected = vec![r1, r2];
                expected.sort_unstable();
                expected.dedup();
                assert_eq!(f.roots(&monic), expected, "p={p}, i={i}");
                for scalar in [2u64, 3, 7] {
                    let scaled = [
                        f.mul(monic[0], scalar % p),
                        f.mul(monic[1], scalar % p),
                        scalar % p,
                    ];
                    assert_eq!(f.roots(&scaled), expected, "scaled p={p}, i={i}");
                }
                let repeated_coeff = f.add(r1, r1);
                let repeated_constant = f.mul(r1, r1);
                let double = [repeated_constant, f.sub(0, repeated_coeff), 1];
                assert_eq!(f.roots(&double), vec![r1], "repeated p={p}, i={i}");
            }
            // x^2 - n must have no roots for independently selected
            // Euler-criterion nonresidue n.
            let mut n = 2u64;
            while f.pow(n, (p - 1) / 2) != p - 1 {
                n += 1;
            }
            let constant = f.sub(0, n);
            assert!(f.roots(&[constant, 0, 1]).is_empty(), "nonresidue p={p}");
        }
    }

    #[test]
    fn round129_monic_factor_roots_match_known_factors() {
        for p in [101u64, 65_521, 1_000_000_007, 4_294_967_291, (1u64 << 61) - 1] {
            let mut tally = FieldTally::default();
            let mut field = Fp { p, tally: &mut tally };
            for i in 0..24u64 {
                let factors = [(3 + 7 * i) % p, (13 + 7 * i) % p,
                    (29 + 7 * i) % p, (47 + 7 * i) % p];
                let mut polynomial = vec![1u64];
                for &root in &factors {
                    let mut next = vec![0u64; polynomial.len() + 1];
                    for (j, &coefficient) in polynomial.iter().enumerate() {
                        let term = field.mul(coefficient, root);
                        next[j] = field.sub(next[j], term);
                        next[j + 1] = field.add(next[j + 1], coefficient);
                    }
                    polynomial = next;
                }
                let mut expected = factors.to_vec();
                expected.sort_unstable();
                expected.dedup();
                assert_eq!(field.roots(&polynomial), expected, "monic p={p} i={i}");
                for scalar in [2u64, 3, 7] {
                    let scaled: Vec<u64> = polynomial.iter()
                        .map(|&coefficient| field.mul(coefficient, scalar))
                        .collect();
                    assert_eq!(field.roots(&scaled), expected, "scaled p={p} i={i}");
                }
            }
        }
    }

    #[test]
    fn round129_isogeny_psi3_roots_match_small_field_exhaustive_search() {
        let p = 101u64;
        let mut tally = FieldTally::default();
        let mut field = Fp { p, tally: &mut tally };
        for a in 0..12u64 {
            for b in 0..12u64 {
                // ψ3(x) = 3 x^4 + 6 a x^2 + 12 b x - a^2.
                let aa = field.mul(a, a);
                let poly = vec![field.sub(0, aa), field.mul(12, b),
                    field.mul(6, a), 0, 3];
                let roots = field.roots(&poly);
                let brute_force: Vec<u64> = (0..p).filter(|&x| {
                    poly.iter().rev().fold(0u64, |acc, &c| {
                        let prod = field.mul(acc, x);
                        field.add(prod, c)
                    }) == 0
                }).collect();
                assert_eq!(roots, brute_force, "ψ3 a={a} b={b}");
            }
        }
    }


    #[test]
    fn round137_pseudoremainder_gcd_matches_exact_field_euclid() {
        for p in [3u64, 5, 7, 11, 101, 65_521, 1_000_000_007,
                  4_294_967_291, (1u64 << 61) - 1] {
            let mut tally = FieldTally::default();
            let mut field = Fp { p, tally: &mut tally };
            let mut state = p ^ 0x137_2026_11_23u64;
            for _ in 0..128 {
                let mut next = || {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    state % p
                };
                let mut a = vec![next(), next(), next(), next(), next()];
                let mut b = vec![next(), next(), next(), next()];
                if a[4] == 0 { a[4] = 1; }
                if b[3] == 0 { b[3] = 1; }
                let mut x = a.clone();
                let mut y = b.clone();
                while !y.is_empty() {
                    let r = field.rem(&x, &y);
                    x = y;
                    y = r;
                }
                let mut got = field.gcd(&a, &b);
                if x.len() > 1 { field.make_monic(&mut x); }
                else if !x.is_empty() { x[0] = 1; }
                if got.len() > 1 { field.make_monic(&mut got); }
                else if !got.is_empty() { got[0] = 1; }
                assert_eq!(got, x, "p={p}, a={a:?}, b={b:?}");
            }
        }
    }

    #[test]
    fn round140_small_constant_inverses_match_fermat() {
        // Include both p=1 and p=2 (mod 3), 32-bit and 61-bit primes.
        for p in [3u64, 5, 7, 11, 13, 17, 19, 23, 31, 101,
                  65_521, 1_000_000_007, 1_000_000_009,
                  4_294_967_291, (1u64 << 61) - 1] {
            let mut tally = FieldTally::default();
            let mut field = Fp { p, tally: &mut tally };
            for a in [2u64, 3] {
                if a >= p { continue; }
                let got = field.inv(a);
                assert_eq!(got, field.pow(a, p - 2), "p={p}, a={a}");
                assert_eq!(field.mul(a, got), 1, "p={p}, a={a}");
            }
        }
    }


}
