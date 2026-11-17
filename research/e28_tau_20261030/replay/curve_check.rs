//! E28 validation (independent validator, 2026-10-31): the curve facts the
//! matched-rho choice rests on, checked with the validator's own `F_{p⁴}`
//! arithmetic (standard library only; shares no code with `crypto_lib`).
//!
//! Input (stdin): one line per curve, as printed by `dump_params.rs` (which
//! only calls the producer's instance generator to obtain the coefficients):
//! `p seed n h lambda a0..a3 b0..b3 gx0..gx3 gy0..gy3 cx0..cx3 cy0..cy3`.
//!
//! For each curve, in `F_p[t]/(t⁴ − γ)` with `γ` the least non-square mod `p`:
//! `G` on `y² = x³ + ax + b`; `[n]G = O` with `n` prime and `h·n` in the Hasse
//! interval (so `#E = h·n`, the only multiple of `n` there); `j = 1728·4a³/
//! (4a³ + 27b²)` satisfies `j^{p²} = j` and `j^p ≠ j` (`j ∈ F_{p²} ∖ F_p`, so
//! `j ≠ 0, 1728` and `Aut(E) = ±1`); `ψ(x, y) = (c_x x^{p²}, c_y y^{p²})` maps
//! `G` to `[λ]G`, `ψ²(G) = −G` and `λ² ≡ −1 (mod n)`.
//!
//! ```text
//! rustc -O --edition 2021 curve_check.rs -o curve_check && curve_check < params.txt
//! ```

use std::io::Read;

type E = [u64; 4];

struct F {
    p: u64,
    g: u64,
}

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

impl F {
    fn new(p: u64) -> F {
        let g = (2..p).find(|&c| pw(c, (p - 1) / 2, p) == p - 1).unwrap();
        F { p, g }
    }
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
        // t⁴ = γ
        let mut r = [c[0], c[1], c[2], c[3]];
        for k in 4..7 {
            r[k - 4] = (r[k - 4] + mm(c[k], self.g, p)) % p;
        }
        r
    }
    fn k(&self, v: u64) -> E {
        [v % self.p, 0, 0, 0]
    }
    fn pow(&self, a: &E, e: u128) -> E {
        let (mut r, mut b, mut e) = (self.k(1), *a, e);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.mul(&b, &b);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: &E) -> E {
        let q = (self.p as u128).pow(4);
        self.pow(a, q - 2)
    }
    fn zero(a: &E) -> bool {
        a.iter().all(|&x| x == 0)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum P {
    O,
    A(E, E),
}

fn padd(f: &F, a: &E, q: P, r: P) -> P {
    match (q, r) {
        (P::O, x) | (x, P::O) => x,
        (P::A(x1, y1), P::A(x2, y2)) => {
            let l = if x1 == x2 {
                if F::zero(&f.add(&y1, &y2)) {
                    return P::O;
                }
                let num = f.add(&f.mul(&f.k(3), &f.mul(&x1, &x1)), a);
                f.mul(&num, &f.inv(&f.add(&y1, &y1)))
            } else {
                f.mul(&f.sub(&y2, &y1), &f.inv(&f.sub(&x2, &x1)))
            };
            let x3 = f.sub(&f.sub(&f.mul(&l, &l), &x1), &x2);
            let y3 = f.sub(&f.mul(&l, &f.sub(&x1, &x3)), &y1);
            P::A(x3, y3)
        }
    }
}

fn pmul(f: &F, a: &E, q: P, mut k: u64) -> P {
    let (mut r, mut b) = (P::O, q);
    while k > 0 {
        if k & 1 == 1 {
            r = padd(f, a, r, b);
        }
        b = padd(f, a, b, b);
        k >>= 1;
    }
    r
}

fn is_prime(n: u64) -> bool {
    n >= 2 && (2..).take_while(|d: &u64| d * d <= n).all(|d| n % d != 0)
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut all = true;
    let mut count = 0;
    println!("[");
    let lines: Vec<&str> = s.lines().filter(|l| !l.trim().is_empty()).collect();
    for (li, line) in lines.iter().enumerate() {
        let v: Vec<u64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let (p, seed, n, h, lam) = (v[0], v[1], v[2], v[3], v[4]);
        let e = |k: usize| -> E { [v[k], v[k + 1], v[k + 2], v[k + 3]] };
        let (a, b, gx, gy, cx, cy) = (e(5), e(9), e(13), e(17), e(21), e(25));
        let f = F::new(p);
        let g = P::A(gx, gy);
        let on = f.mul(&gy, &gy) == f.add(&f.add(&f.mul(&f.mul(&gx, &gx), &gx), &f.mul(&a, &gx)), &b);
        let ng = pmul(&f, &a, g, n) == P::O;
        let p2 = (p as u128) * (p as u128);
        let p4 = p2 * p2;
        let order = (h as u128) * (n as u128);
        let hasse = (order as i128 - p4 as i128 - 1).abs() <= 2 * p2 as i128;
        let a3x4 = f.mul(&f.k(4), &f.mul(&f.mul(&a, &a), &a));
        let disc = f.add(&a3x4, &f.mul(&f.k(27), &f.mul(&b, &b)));
        let nonsing = !F::zero(&disc);
        let j = f.mul(&f.mul(&f.k(1728), &a3x4), &f.inv(&disc));
        let j_fp2 = f.pow(&j, p2) == j;
        let j_fp = f.pow(&j, p as u128) == j;
        let psi = |q: P| match q {
            P::O => P::O,
            P::A(x, y) => P::A(f.mul(&cx, &f.pow(&x, p2)), f.mul(&cy, &f.pow(&y, p2))),
        };
        let psig = psi(g);
        let psi_on = match psig {
            P::A(x, y) => f.mul(&y, &y) == f.add(&f.add(&f.mul(&f.mul(&x, &x), &x), &f.mul(&a, &x)), &b),
            P::O => false,
        };
        let psi_lambda = psig == pmul(&f, &a, g, lam);
        let psi2 = psi(psig) == match g {
            P::A(x, y) => P::A(x, f.neg(&y)),
            P::O => P::O,
        };
        let lam2 = ((lam as u128 * lam as u128) % n as u128) as u64 == n - 1;
        let ok = on && ng && is_prime(n) && hasse && nonsing && j_fp2 && !j_fp && psi_on && psi_lambda && psi2 && lam2;
        all &= ok;
        count += 1;
        println!(
            "  {{\"p\": {p}, \"seed\": {seed}, \"n\": {n}, \"h\": {h}, \"gamma\": {}, \"g_on_curve\": {on}, \"n_g_is_o\": {ng}, \"n_prime\": {}, \"hn_in_hasse\": {hasse}, \"j\": [{}, {}, {}, {}], \"j_in_fp2\": {j_fp2}, \"j_in_fp\": {j_fp}, \"psi_g_on_curve\": {psi_on}, \"psi_g_is_lambda_g\": {psi_lambda}, \"psi2_g_is_minus_g\": {psi2}, \"lambda2_is_minus1\": {lam2}, \"ok\": {ok}}}{}",
            f.g, is_prime(n), j[0], j[1], j[2], j[3],
            if li + 1 < lines.len() { "," } else { "" }
        );
    }
    println!("]");
    eprintln!("curves {count}, all ok {all}");
}
