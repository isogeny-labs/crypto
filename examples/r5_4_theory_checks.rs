//! Lead R5.4 (theory): two computational sanity checks for the note in
//! `research/isogeny_fibers_20261010/round5/r5_4_theory/RESULTS.md`.
//!
//! * `conductor`: for primes p near 2^k, every ordinary trace t, the index
//!   f = [O_K : Z[pi]] and its largest prime factor. Records the fraction of
//!   isogeny classes (traces) whose levels are separated by a prime
//!   ell >= p^eps, the region where Conjecture 10 stays open (Theorem 4).
//! * `char2`: binary ordinary curves y^2 + xy = x^3 + a x^2 + b. Checks
//!   x(2P) = u(P)^2 with u = x + sqrt(b)/x, the halving criterion, the
//!   identity [2](F_u) = F_tz (Theorem 9), the equality of decomposable-target
//!   counts D_m^G, and the sweep accounting of Proposition 15.2' (a sweep
//!   over F finds every hit exactly |M| = 2 times; a transversal sweep finds
//!   the same decomposable targets with half the lookups).
//!
//! Standalone (no library code). Single-threaded. Output: JSON on stdout.
//!
//! ```sh
//! cargo run --release --example r5_4_theory_checks -- conductor > conductor.json
//! cargo run --release --example r5_4_theory_checks -- char2 > char2.json
//! ```

use std::collections::HashSet;

// ---------------------------------------------------------------- utilities

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // splitmix64
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for sp in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n.is_multiple_of(sp) {
            return n == sp;
        }
    }
    let mulmod = |a: u64, b: u64| ((a as u128 * b as u128) % n as u128) as u64;
    let powmod = |mut b: u64, mut e: u64| {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = mulmod(r, b);
            }
            b = mulmod(b, b);
            e >>= 1;
        }
        r
    };
    let mut d = n - 1;
    let mut s = 0;
    while d.is_multiple_of(2) {
        d /= 2;
        s += 1;
    }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

fn isqrt(n: u64) -> u64 {
    let mut r = (n as f64).sqrt() as u64;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

fn small_primes(limit: u64) -> Vec<u64> {
    let l = limit as usize;
    let mut sieve = vec![true; l + 1];
    let mut out = Vec::new();
    for i in 2..=l {
        if sieve[i] {
            out.push(i as u64);
            let mut j = i * i;
            while j <= l {
                sieve[j] = false;
                j += i;
            }
        }
    }
    out
}

/// Primes ell with ell^2 | n, with multiplicity floor(e/2): returns the
/// square part as a list of (ell, floor(e/2)). Uses trial division up to
/// n^(1/3); the cofactor then has at most two prime factors, and it
/// contributes a square only if it is a perfect square.
fn square_part(mut n: u64, primes: &[u64]) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let cube = (n as f64).cbrt() as u64 + 2;
    for &l in primes {
        if l > cube {
            break;
        }
        if n.is_multiple_of(l) {
            let mut e = 0;
            while n.is_multiple_of(l) {
                n /= l;
                e += 1;
            }
            if e >= 2 {
                out.push((l, e / 2));
            }
        }
    }
    if n > 1 {
        let s = isqrt(n);
        if s * s == n {
            out.push((s, 1));
        }
    }
    out
}

// ------------------------------------------------------------ conductor run

fn run_conductor() {
    let bits = [16u32, 20, 24, 28, 32, 36];
    let eps_grid: Vec<f64> = (0..=10).map(|i| i as f64 * 0.05).collect();
    let mut rows = Vec::new();
    for &b in &bits {
        // smallest prime >= 2^b + 1
        let mut p = (1u64 << b) + 1;
        while !is_prime_u64(p) {
            p += 1;
        }
        let tmax = isqrt(4 * p);
        let primes = small_primes(((4 * p) as f64).cbrt() as u64 + 3);
        let mut n_classes = 0u64;
        let mut f_is_1 = 0u64;
        let mut dk_3_4 = 0u64;
        let mut lmax_counts = vec![0u64; eps_grid.len()];
        let mut max_lmax = 0u64;
        let tm = tmax as i64;
        for t in -tm..=tm {
            let d = 4 * p as i64 - t * t; // |D|, D = t^2 - 4p < 0
            if d <= 0 || t == 0 {
                continue; // t = 0 is supersingular for p >= 5
            }
            n_classes += 1;
            let sq = square_part(d as u64, &primes);
            let mut v: u64 = 1;
            for &(l, e) in &sq {
                v *= l.pow(e);
            }
            let d0 = d as u64 / (v * v); // squarefree, D = -v^2 d0
                                         // -d0 = 1 mod 4  <=>  d0 = 3 mod 4
            let (f, dk) = if d0 % 4 == 3 {
                (v, d0)
            } else {
                (v / 2, 4 * d0)
            };
            if dk == 3 || dk == 4 {
                dk_3_4 += 1;
            }
            if f == 1 {
                f_is_1 += 1;
            }
            // largest prime of f
            let mut lmax = 1u64;
            let mut ff = f;
            for &(l, _) in &sq {
                if ff % l == 0 {
                    lmax = lmax.max(l);
                    while ff % l == 0 {
                        ff /= l;
                    }
                }
            }
            if ff > 1 {
                lmax = lmax.max(ff);
            }
            max_lmax = max_lmax.max(lmax);
            for (i, &e) in eps_grid.iter().enumerate() {
                if lmax > 1 && (lmax as f64) >= (p as f64).powf(e) {
                    lmax_counts[i] += 1;
                }
            }
        }
        // heuristic union bound: sum over odd primes ell >= p^eps,
        // ell <= 2 sqrt p, of (1 + (p|ell))/ell^2 (density of ell^2 | t^2-4p)
        let all_primes = small_primes(2 * isqrt(p) + 2);
        let mut heur = vec![0f64; eps_grid.len()];
        for &l in all_primes.iter().skip(1) {
            let leg = {
                // Euler criterion
                let mut r = 1u64;
                let mut bb = p % l;
                let mut e = (l - 1) / 2;
                while e > 0 {
                    if e & 1 == 1 {
                        r = r * bb % l;
                    }
                    bb = bb * bb % l;
                    e >>= 1;
                }
                if r == 1 {
                    2.0
                } else {
                    0.0
                }
            };
            let dens = leg / (l as f64 * l as f64);
            for (i, &e) in eps_grid.iter().enumerate() {
                if (l as f64) >= (p as f64).powf(e) {
                    heur[i] += dens;
                }
            }
        }
        let fr: Vec<String> = lmax_counts
            .iter()
            .map(|&c| format!("{:.6e}", c as f64 / n_classes as f64))
            .collect();
        let cn: Vec<String> = lmax_counts.iter().map(|c| c.to_string()).collect();
        let hs: Vec<String> = heur.iter().map(|h| format!("{:.6e}", h)).collect();
        rows.push(format!(
            "{{\"bits\":{b},\"p\":{p},\"ordinary_traces\":{n_classes},\"f_eq_1\":{f_is_1},\
\"dk_minus3_or_minus4\":{dk_3_4},\"max_lmax\":{max_lmax},\"eps\":[{}],\
\"count_lmax_ge_p_eps\":[{}],\"frac_lmax_ge_p_eps\":[{}],\"heuristic_union_bound\":[{}]}}",
            eps_grid
                .iter()
                .map(|e| format!("{e:.2}"))
                .collect::<Vec<_>>()
                .join(","),
            cn.join(","),
            fr.join(","),
            hs.join(",")
        ));
        eprintln!("conductor bits={b} p={p} classes={n_classes} done");
    }
    println!(
        "{{\"experiment\":\"conductor\",\"kind\":\"measured (exact count over all ordinary traces)\",\"rows\":[{}]}}",
        rows.join(",\n")
    );
}

// ---------------------------------------------------------------- GF(2^n)

#[derive(Clone, Copy)]
struct Gf {
    n: u32,
    poly: u64, // full modulus including x^n
}

impl Gf {
    fn mul(&self, a: u32, b: u32) -> u32 {
        let (mut a, mut b) = (a as u64, b as u64);
        let mut r = 0u64;
        while b != 0 {
            if b & 1 == 1 {
                r ^= a;
            }
            a <<= 1;
            b >>= 1;
        }
        for i in (self.n..(2 * self.n)).rev() {
            if (r >> i) & 1 == 1 {
                r ^= self.poly << (i - self.n);
            }
        }
        r as u32
    }
    fn sq(&self, a: u32) -> u32 {
        self.mul(a, a)
    }
    fn pow2k(&self, mut a: u32, k: u32) -> u32 {
        for _ in 0..k {
            a = self.sq(a);
        }
        a
    }
    fn inv(&self, a: u32) -> u32 {
        // a^(2^n - 2)
        let mut r = 1u32;
        let mut base = a;
        let mut e: u64 = (1u64 << self.n) - 2;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, base);
            }
            base = self.sq(base);
            e >>= 1;
        }
        r
    }
    fn sqrt(&self, a: u32) -> u32 {
        self.pow2k(a, self.n - 1)
    }
    fn tr(&self, a: u32) -> u32 {
        let mut t = a;
        let mut s = a;
        for _ in 1..self.n {
            s = self.sq(s);
            t ^= s;
        }
        t & 1
    }
    /// half-trace, n odd: solution z of z^2 + z = c when Tr(c) = 0
    fn htr(&self, c: u32) -> u32 {
        let mut z = 0u32;
        let mut s = c;
        for _ in 0..=((self.n - 1) / 2) {
            z ^= s;
            s = self.pow2k(s, 2);
        }
        z
    }
}

type Pt = Option<(u32, u32)>;

struct Curve {
    f: Gf,
    a: u32,
    b: u32,
}

impl Curve {
    fn neg(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (x, x ^ y))
    }
    fn add(&self, p: Pt, q: Pt) -> Pt {
        let f = &self.f;
        match (p, q) {
            (None, _) => q,
            (_, None) => p,
            (Some((x1, y1)), Some((x2, y2))) => {
                if x1 == x2 {
                    if y1 ^ y2 == x1 {
                        return None; // q = -p (includes the 2-torsion point)
                    }
                    // doubling
                    if x1 == 0 {
                        return None;
                    }
                    let l = x1 ^ f.mul(y1, f.inv(x1));
                    let x3 = f.sq(l) ^ l ^ self.a;
                    let y3 = f.sq(x1) ^ f.mul(l ^ 1, x3);
                    return Some((x3, y3));
                }
                let l = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
                let x3 = f.sq(l) ^ l ^ x1 ^ x2 ^ self.a;
                let y3 = f.mul(l, x1 ^ x3) ^ x3 ^ y1;
                Some((x3, y3))
            }
        }
    }
    fn mul(&self, mut k: u64, p: Pt) -> Pt {
        let mut r = None;
        let mut q = p;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(r, q);
            }
            q = self.add(q, q);
            k >>= 1;
        }
        r
    }
    fn on_curve(&self, p: Pt) -> bool {
        let f = &self.f;
        match p {
            None => true,
            Some((x, y)) => {
                f.sq(y) ^ f.mul(x, y) == f.mul(f.sq(x), x) ^ f.mul(self.a, f.sq(x)) ^ self.b
            }
        }
    }
    fn points(&self) -> Vec<(u32, u32)> {
        let f = &self.f;
        let mut v = Vec::new();
        v.push((0, f.sqrt(self.b)));
        for x in 1..(1u32 << f.n) {
            let rhs = f.mul(f.sq(x), x) ^ f.mul(self.a, f.sq(x)) ^ self.b;
            let c = f.mul(rhs, f.inv(f.sq(x)));
            if f.tr(c) == 0 {
                let z = f.htr(c);
                v.push((x, f.mul(x, z)));
                v.push((x, f.mul(x, z ^ 1)));
            }
        }
        v
    }
}

fn factor_small(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut d = 2;
    while d * d <= n {
        while n.is_multiple_of(d) {
            out.push(d);
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}

fn subspace(f: &Gf, dim: u32, rng: &mut Rng) -> HashSet<u32> {
    // span of `dim` independent random elements (Gaussian elimination by span growth)
    let mut span: HashSet<u32> = HashSet::new();
    span.insert(0);
    while (span.len() as u64) < (1u64 << dim) {
        let g = (rng.next() as u32) & ((1u32 << f.n) - 1);
        if span.contains(&g) {
            continue;
        }
        let cur: Vec<u32> = span.iter().copied().collect();
        for c in cur {
            span.insert(c ^ g);
        }
    }
    span
}

/// set of points of G that are m-sums (multisets) of base, m in {2, 3}
fn sums_in_g(c: &Curve, base: &[Pt], gset: &HashSet<Pt>, m: usize) -> (usize, usize) {
    let mut s2: HashSet<Pt> = HashSet::new();
    let k = base.len();
    for i in 0..k {
        for j in i..k {
            s2.insert(c.add(base[i], base[j]));
        }
    }
    if m == 2 {
        let d = s2.iter().filter(|p| gset.contains(p)).count();
        return (d, s2.len());
    }
    let mut s3: HashSet<Pt> = HashSet::new();
    for q in &s2 {
        for p in base {
            let s = c.add(*q, *p);
            if gset.contains(&s) {
                s3.insert(s);
            }
        }
    }
    (s3.len(), s2.len())
}

fn proj_columns(c: &Curve, base: &[Pt], h: u64) -> usize {
    let mut cols: HashSet<Pt> = HashSet::new();
    for p in base {
        let q = c.mul(h, *p);
        let nq = c.neg(q);
        let key = if q <= nq { q } else { nq };
        cols.insert(key);
    }
    cols.len()
}

fn run_char2() {
    let fields = [
        (11u32, (1u64 << 11) | (1 << 2) | 1),
        (13u32, (1u64 << 13) | (1 << 4) | (1 << 3) | (1 << 1) | 1),
        (17u32, (1u64 << 17) | (1 << 3) | 1),
    ];
    let mut rng = Rng(0x5254_2026_1012);
    let mut rows = Vec::new();
    for &(n, poly) in &fields {
        let f = Gf { n, poly };
        // curves: two Koblitz, two with random b
        let mut specs: Vec<(u32, u32, &str)> = vec![(0, 1, "koblitz"), (1, 1, "koblitz")];
        for a in [0u32, 1] {
            let b = loop {
                let b = (rng.next() as u32) & ((1u32 << n) - 1);
                if b > 1 {
                    break b;
                }
            };
            specs.push((a, b, "random_b"));
        }
        for (a, b, kind) in specs {
            let c = Curve { f, a, b };
            let pts = c.points();
            let nn = pts.len() as u64 + 1;
            let fac = factor_small(nn);
            let r = *fac.iter().max().unwrap();
            let h = nn / r;
            if h.is_multiple_of(r) || r < 50 {
                eprintln!("skip n={n} a={a} b={b}: N={nn}");
                continue;
            }
            let sb = f.sqrt(b);
            let t_pt: Pt = Some((0, sb));
            let u = |x: u32| x ^ f.mul(sb, f.inv(x));
            // (a) x(2P) = u(P)^2 and on-curve
            let mut bad_double = 0u64;
            let mut bad_curve = 0u64;
            let mut two_e: HashSet<Pt> = HashSet::new();
            for &(x, y) in &pts {
                let p = Some((x, y));
                if !c.on_curve(p) {
                    bad_curve += 1;
                }
                let d = c.add(p, p);
                two_e.insert(d);
                if x != 0 {
                    match d {
                        Some((xd, _)) => {
                            if xd != f.sq(u(x)) {
                                bad_double += 1;
                            }
                        }
                        None => bad_double += 1,
                    }
                }
            }
            two_e.insert(None);
            // (b) halving criterion: P in 2E  <=>  Tr(x) = Tr(a)
            let tra = f.tr(a);
            let mut bad_halving = 0u64;
            for &(x, y) in &pts {
                let in2 = two_e.contains(&Some((x, y)));
                if in2 != (f.tr(x) == tra) {
                    bad_halving += 1;
                }
            }
            // G
            let mut gen = None;
            for &p in &pts {
                let g = c.mul(h, Some(p));
                if g.is_some() {
                    gen = g;
                    break;
                }
            }
            let mut gset: HashSet<Pt> = HashSet::new();
            let mut acc = None;
            for _ in 0..r {
                gset.insert(acc);
                acc = c.add(acc, gen);
            }
            for seed in 0..3u32 {
                let dim = match n {
                    11 => 5,
                    13 => 6,
                    _ => 7,
                };
                let v = subspace(&f, dim, &mut rng);
                let v2: HashSet<u32> = v.iter().map(|&e| f.sq(e)).collect();
                // F_u: points other than O, T with u(P) in V
                let fu: Vec<Pt> = pts
                    .iter()
                    .filter(|&&(x, _)| x != 0 && v.contains(&u(x)))
                    .map(|&p| Some(p))
                    .collect();
                // T-saturation check
                let fu_set: HashSet<Pt> = fu.iter().copied().collect();
                let sat_fail = fu
                    .iter()
                    .filter(|&&p| !fu_set.contains(&c.add(p, t_pt)))
                    .count();
                // F_tz: points Q != O with x(Q) in V^2 and Tr(x) = Tr(a)
                let ftz: Vec<Pt> = pts
                    .iter()
                    .filter(|&&(x, _)| v2.contains(&x) && f.tr(x) == tra)
                    .map(|&p| Some(p))
                    .collect();
                let ftz_set: HashSet<Pt> = ftz.iter().copied().collect();
                let dbl_fu: HashSet<Pt> = fu.iter().map(|&p| c.add(p, p)).collect();
                let identity_holds = dbl_fu == ftz_set;
                let sym_diff = dbl_fu.symmetric_difference(&ftz_set).count();
                // plain x-base on V for reference
                let fx: Vec<Pt> = pts
                    .iter()
                    .filter(|&&(x, _)| x != 0 && v.contains(&x))
                    .map(|&p| Some(p))
                    .collect();
                // transversal of F_u / <T>: keep P with x(P) <= x(P+T) (x(P+T) = sqrt(b)/x)
                let omega: Vec<Pt> = fu
                    .iter()
                    .filter(|p| {
                        let (x, _) = p.unwrap();
                        let xt = f.mul(sb, f.inv(x));
                        x < xt || (x == xt && p.unwrap() <= c.add(**p, t_pt).unwrap())
                    })
                    .copied()
                    .collect();
                let mut res = Vec::new();
                for m in [2usize, 3] {
                    let (d_fu, _) = sums_in_g(&c, &fu, &gset, m);
                    let (d_tz, _) = sums_in_g(&c, &ftz, &gset, m);
                    let (d_x, _) = sums_in_g(&c, &fx, &gset, m);
                    res.push((m, d_fu, d_tz, d_x));
                }
                // sweep accounting at m = 3: pair table of F_u (T-stable)
                let mut pt2: HashSet<Pt> = HashSet::new();
                for i in 0..fu.len() {
                    for j in i..fu.len() {
                        pt2.insert(c.add(fu[i], fu[j]));
                    }
                }
                let mut hits_f = 0u64;
                let mut hits_o = 0u64;
                let mut dec_f = 0u64;
                let mut dec_o = 0u64;
                let mut mismatch = 0u64;
                let mut hits_ratio_exact = true;
                for rpt in &gset {
                    let hf = fu
                        .iter()
                        .filter(|&&p| pt2.contains(&c.add(*rpt, c.neg(p))))
                        .count() as u64;
                    let ho = omega
                        .iter()
                        .filter(|&&p| pt2.contains(&c.add(*rpt, c.neg(p))))
                        .count() as u64;
                    hits_f += hf;
                    hits_o += ho;
                    if hf > 0 {
                        dec_f += 1;
                    }
                    if ho > 0 {
                        dec_o += 1;
                    }
                    if (hf > 0) != (ho > 0) {
                        mismatch += 1;
                    }
                    if hf != 2 * ho {
                        hits_ratio_exact = false;
                    }
                }
                let cols_fu = proj_columns(&c, &fu, h);
                let cols_tz = proj_columns(&c, &ftz, h);
                let cols_x = proj_columns(&c, &fx, h);
                let rs: Vec<String> = res
                    .iter()
                    .map(|(m, a1, a2, a3)| {
                        format!(
                            "{{\"m\":{m},\"D_m_G_Fu\":{a1},\"D_m_G_Ftz\":{a2},\"D_m_G_Fx\":{a3}}}"
                        )
                    })
                    .collect();
                rows.push(format!(
                    "{{\"n\":{n},\"a\":{a},\"b\":{b},\"kind\":\"{kind}\",\"N\":{nn},\"r\":{r},\"h\":{h},\
\"seed\":{seed},\"dimV\":{dim},\"bad_on_curve\":{bad_curve},\"bad_x2P_eq_u2\":{bad_double},\
\"bad_halving_trace\":{bad_halving},\"Fu\":{},\"Fu_saturation_failures\":{sat_fail},\
\"Omega\":{},\"Ftz\":{},\"Fx\":{},\"double_Fu_eq_Ftz\":{identity_holds},\"symdiff\":{sym_diff},\
\"cols_Fu\":{cols_fu},\"cols_Ftz\":{cols_tz},\"cols_Fx\":{cols_x},\"census\":[{}],\
\"m3_sweep\":{{\"targets\":{},\"lookups_F\":{},\"lookups_Omega\":{},\"hits_F\":{hits_f},\
\"hits_Omega\":{hits_o},\"decomposable_F\":{dec_f},\"decomposable_Omega\":{dec_o},\
\"decomposable_mismatch\":{mismatch},\"hits_F_eq_2_hits_Omega_every_target\":{hits_ratio_exact}}}}}",
                    fu.len(),
                    omega.len(),
                    ftz.len(),
                    fx.len(),
                    rs.join(","),
                    gset.len(),
                    gset.len() * fu.len(),
                    gset.len() * omega.len(),
                ));
                eprintln!("char2 n={n} a={a} kind={kind} seed={seed} done");
            }
        }
    }
    println!(
        "{{\"experiment\":\"char2\",\"kind\":\"measured (exhaustive)\",\"rows\":[\n{}\n]}}",
        rows.join(",\n")
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("conductor") => run_conductor(),
        Some("char2") => run_char2(),
        _ => {
            eprintln!("usage: r5_4_theory_checks conductor|char2");
            std::process::exit(2);
        }
    }
}
