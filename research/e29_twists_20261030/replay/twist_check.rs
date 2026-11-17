//! E29 validation (independent validator, 2026-10-31): the kill test's group,
//! checked on the canonical models `y² = x³ + b₀t³` over `F_p[t]/(t⁴ − γ)`
//! (`γ` the least non-square mod `p`) with the validator's own arithmetic
//! (standard library only; shares no code with `crypto_lib`).
//!
//! Input (stdin): one line per curve, `p n b0` (from the committed kill.json),
//! optionally followed by `gx0..gx3 gy0..gy3` (Stage A's generator).
//!
//! Per curve: a point `P` (Stage A's generator when given, else the first
//! `x ∈ F_p` with `x³ + b₀t³` a square, root by Tonelli–Shanks); `[n]P = O`,
//! `n` prime and in the Hasse interval of `F_{p⁴}` (so `#E = n`); `ψ(x, y) =
//! (t^{1−p} x^p, t^{3(1−p)/2} y^p)` (the `p`-power Frobenius composed with the
//! isomorphism `E^{(p)} → E`, derived by the validator: `u⁶ = t^{3(1−p)}`) maps
//! `P` to a point of `E` and to `[λ]P` for a 24th root of unity `λ` mod `n`;
//! the order of `λ`, of `λ_ω` (the cubic automorphism `(x, y) ↦ (ωx, y)`), and
//! of the group `⟨λ, λ_ω, −1⟩` they generate in `(Z/n)^*`.  With this `u`,
//! `N(u) = t^{(1−q)/2} = −1`, so `ψ⁴ = −1` and `λ` has order `8` (derived by
//! the validator after a first run of this check printed order 8 and was
//! wrongly gated on 24); E18's `ψ_p` (`u = b^{−(p−1)/6}`) differs from it by an
//! automorphism, so the invariant compared with the kill test is the group
//! order `|⟨λ_p, λ_ω, −1⟩| = 24` and the order `24` of `λ·λ_ω`.
//!
//! ```text
//! rustc -O --edition 2021 twist_check.rs -o twist_check && twist_check < twist_input.txt
//! ```

use std::io::Read;

type E = [u64; 4];

fn mm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn pw(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mm(r, b, p);
        }
        b = mm(b, b, p);
        e >>= 1;
    }
    r
}

struct F {
    p: u64,
    g: u64,
}

impl F {
    fn add(&self, a: &E, b: &E) -> E {
        let mut r = [0; 4];
        for i in 0..4 {
            r[i] = (a[i] + b[i]) % self.p;
        }
        r
    }
    fn neg(&self, a: &E) -> E {
        let mut r = [0; 4];
        for i in 0..4 {
            r[i] = (self.p - a[i]) % self.p;
        }
        r
    }
    fn sub(&self, a: &E, b: &E) -> E {
        self.add(a, &self.neg(b))
    }
    fn mul(&self, a: &E, b: &E) -> E {
        let p = self.p;
        let mut c = [0u64; 7];
        for i in 0..4 {
            for j in 0..4 {
                c[i + j] = (c[i + j] + mm(a[i], b[j], p)) % p;
            }
        }
        let mut r = [c[0], c[1], c[2], c[3]];
        for k in 4..7 {
            r[k - 4] = (r[k - 4] + mm(c[k], self.g, p)) % p;
        }
        r
    }
    fn k(&self, v: u64) -> E {
        [v % self.p, 0, 0, 0]
    }
    fn pow(&self, a: &E, mut e: u128) -> E {
        let (mut r, mut b) = (self.k(1), *a);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.mul(&b, &b);
            e >>= 1;
        }
        r
    }
    fn q(&self) -> u128 {
        (self.p as u128).pow(4)
    }
    fn inv(&self, a: &E) -> E {
        self.pow(a, self.q() - 2)
    }
    fn is_sq(&self, a: &E) -> bool {
        self.pow(a, (self.q() - 1) / 2) == self.k(1)
    }
    /// Tonelli–Shanks; `t` is a non-square (its norm is `−γ`, `p ≡ 1 mod 4`).
    fn sqrt(&self, a: &E) -> E {
        let q = self.q();
        let (mut s, mut m) = (0u32, q - 1);
        while m % 2 == 0 {
            m /= 2;
            s += 1;
        }
        let z = [0, 1, 0, 0];
        assert!(!self.is_sq(&z));
        let mut c = self.pow(&z, m);
        let mut x = self.pow(a, (m + 1) / 2);
        let mut t = self.pow(a, m);
        let mut ms = s;
        let one = self.k(1);
        while t != one {
            let mut i = 0;
            let mut tt = t;
            while tt != one {
                tt = self.mul(&tt, &tt);
                i += 1;
            }
            let mut b = c;
            for _ in 0..(ms - i - 1) {
                b = self.mul(&b, &b);
            }
            x = self.mul(&x, &b);
            c = self.mul(&b, &b);
            t = self.mul(&t, &c);
            ms = i;
        }
        x
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum P {
    O,
    A(E, E),
}

fn padd(f: &F, q: P, r: P) -> P {
    match (q, r) {
        (P::O, x) | (x, P::O) => x,
        (P::A(x1, y1), P::A(x2, y2)) => {
            let l = if x1 == x2 {
                if f.add(&y1, &y2) == [0; 4] {
                    return P::O;
                }
                f.mul(&f.mul(&f.k(3), &f.mul(&x1, &x1)), &f.inv(&f.add(&y1, &y1)))
            } else {
                f.mul(&f.sub(&y2, &y1), &f.inv(&f.sub(&x2, &x1)))
            };
            let x3 = f.sub(&f.sub(&f.mul(&l, &l), &x1), &x2);
            let y3 = f.sub(&f.mul(&l, &f.sub(&x1, &x3)), &y1);
            P::A(x3, y3)
        }
    }
}

fn pmul(f: &F, q: P, mut k: u64) -> P {
    let (mut r, mut b) = (P::O, q);
    while k > 0 {
        if k & 1 == 1 {
            r = padd(f, r, b);
        }
        b = padd(f, b, b);
        k >>= 1;
    }
    r
}

fn mm128(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn pw128(mut b: u64, mut e: u64, n: u64) -> u64 {
    let mut r = 1u64;
    b %= n;
    while e > 0 {
        if e & 1 == 1 {
            r = mm128(r, b, n);
        }
        b = mm128(b, b, n);
        e >>= 1;
    }
    r
}
fn is_prime(n: u64) -> bool {
    n >= 2 && (2..).take_while(|d: &u64| d * d <= n).all(|d| n % d != 0)
}
fn ord_mod(x: u64, n: u64) -> u64 {
    // order of x in (Z/n)^*, x a 24th root of unity
    (1..=24u64).find(|&k| 24 % k == 0 && pw128(x, k, n) == 1).unwrap_or(0)
}
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let lines: Vec<&str> = s.lines().filter(|l| !l.trim().is_empty()).collect();
    let mut all = true;
    println!("[");
    for (li, line) in lines.iter().enumerate() {
        let v: Vec<u64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let (p, n, b0) = (v[0], v[1], v[2]);
        let g = (2..p).find(|&c| pw(c, (p - 1) / 2, p) == p - 1).unwrap();
        let f = F { p, g };
        let bt3: E = [0, 0, 0, b0 % p]; // b0 t^3
        let rhs = |x: &E| f.add(&f.mul(&f.mul(x, x), x), &bt3);
        let (pt, from_stage_a) = if v.len() >= 11 {
            (P::A([v[3], v[4], v[5], v[6]], [v[7], v[8], v[9], v[10]]), true)
        } else {
            let mut found = P::O;
            for x in 0..p {
                let xe = f.k(x);
                let r = rhs(&xe);
                if r != [0; 4] && f.is_sq(&r) {
                    found = P::A(xe, f.sqrt(&r));
                    break;
                }
            }
            (found, false)
        };
        let on = match pt {
            P::A(x, y) => f.mul(&y, &y) == rhs(&x),
            P::O => false,
        };
        let n_kills = pmul(&f, pt, n) == P::O && pt != P::O;
        let p2 = (p as u128) * (p as u128);
        let p4 = p2 * p2;
        let hasse = (n as i128 - p4 as i128 - 1).abs() <= 2 * p2 as i128;
        // psi
        let t: E = [0, 1, 0, 0];
        let cx = f.inv(&f.pow(&t, (p - 1) as u128));
        let cy = f.inv(&f.pow(&t, (3 * (p - 1) / 2) as u128));
        let psi = |q: P| match q {
            P::O => P::O,
            P::A(x, y) => P::A(f.mul(&cx, &f.pow(&x, p as u128)), f.mul(&cy, &f.pow(&y, p as u128))),
        };
        let pp = psi(pt);
        let psi_on = match pp {
            P::A(x, y) => f.mul(&y, &y) == rhs(&x),
            P::O => false,
        };
        let cx_in_fp = cx[1] == 0 && cx[2] == 0 && cx[3] == 0;
        // a primitive 24th root of unity mod n
        let n24 = n % 24 == 1;
        let mut z = 0u64;
        if n24 {
            for a in 2..1000u64 {
                let c = pw128(a, (n - 1) / 24, n);
                if pw128(c, 12, n) != 1 && pw128(c, 8, n) != 1 {
                    z = c;
                    break;
                }
            }
        }
        let mut lambda_p = 0u64;
        let mut kfound = None;
        if z != 0 {
            let mut zk = 1u64;
            for k in 0..24u64 {
                if pmul(&f, pt, zk) == pp {
                    kfound = Some(k);
                    lambda_p = zk;
                    break;
                }
                zk = mm128(zk, z, n);
            }
        }
        let ord_p = kfound.map(|k| 24 / gcd(k, 24)).unwrap_or(0);
        // cubic automorphism (x, y) -> (omega x, y), omega in F_p of order 3
        let omega = (2..p).map(|a| pw(a, (p - 1) / 3, p)).find(|&w| w != 1).unwrap();
        let aut = match pt {
            P::A(x, y) => P::A(f.mul(&f.k(omega), &x), y),
            P::O => P::O,
        };
        let mut lam_w = 0u64;
        if z != 0 {
            let mut zk = 1u64;
            for _ in 0..24u64 {
                if pmul(&f, pt, zk) == aut {
                    lam_w = zk;
                    break;
                }
                zk = mm128(zk, z, n);
            }
        }
        let ord_w = if lam_w != 0 { ord_mod(lam_w, n) } else { 0 };
        // the group <lambda_p, lambda_w, -1> inside mu_24
        let mut grp = std::collections::BTreeSet::new();
        if lambda_p != 0 && lam_w != 0 {
            grp.insert(1u64);
            loop {
                let before = grp.len();
                let cur: Vec<u64> = grp.iter().cloned().collect();
                for &x in &cur {
                    for gen in [lambda_p, lam_w, n - 1] {
                        grp.insert(mm128(x, gen, n));
                    }
                }
                if grp.len() == before {
                    break;
                }
            }
        }
        // This psi uses u = t^{(1-p)/2}, whose norm is t^{(1-q)/2} = -1, so psi^4 = -1 and
        // lambda_p has order 8; E18's psi_p (u = b^{-(p-1)/6}) differs from it by an
        // automorphism.  The invariant is the group <lambda_p, lambda_omega, -1>, and
        // psi composed with the cubic automorphism has order lcm(8, 3) = 24.
        let ord_pw = if lambda_p != 0 && lam_w != 0 { ord_mod(mm128(lambda_p, lam_w, n), n) } else { 0 };
        let ok = on && n_kills && is_prime(n) && hasse && psi_on && n24 && ord_p == 8 && ord_w == 3 && ord_pw == 24 && grp.len() == 24;
        all &= ok;
        println!(
            "  {{\"p\": {p}, \"n\": {n}, \"b0\": {b0}, \"gamma\": {g}, \"point_from_stage_a\": {from_stage_a}, \"point_on_curve\": {on}, \"n_p_is_o\": {n_kills}, \"n_prime\": {}, \"n_in_hasse\": {hasse}, \"n_mod_24_is_1\": {n24}, \"psi_x_multiplier_in_fp\": {cx_in_fp}, \"psi_p_on_curve\": {psi_on}, \"lambda_p\": {lambda_p}, \"order_lambda_p\": {ord_p}, \"lambda_omega\": {lam_w}, \"order_lambda_omega\": {ord_w}, \"order_lambda_p_times_lambda_omega\": {ord_pw}, \"group_order\": {}, \"ok\": {ok}}}{}",
            is_prime(n), grp.len(), if li + 1 < lines.len() { "," } else { "" }
        );
    }
    println!("]");
    eprintln!("curves {}, all ok {all}", lines.len());
}
