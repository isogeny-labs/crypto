use crate::curve::*;
use crate::gf::*;
use crate::semaev::ETerm;

#[derive(Clone)]
pub struct Rng(pub u64);
impl Rng {
    pub fn new(seed: u64, tag: &str) -> Rng {
        let mut h = seed ^ 0xCBF2_9CE4_8422_2325;
        for b in tag.bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01B3);
        }
        let mut r = Rng(h);
        r.next();
        r
    }
    pub fn next(&mut self) -> u64 {
        // splitmix64
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, m: u64) -> u64 {
        // rejection sampling
        let lim = u64::MAX - (u64::MAX % m);
        loop {
            let v = self.next();
            if v < lim {
                return v % m;
            }
        }
    }
    pub fn e4(&mut self, f: &Fq4) -> E4 {
        let q = f.f.q as u64;
        [self.below(q) as u16, self.below(q) as u16, self.below(q) as u16, self.below(q) as u16]
    }
}

/// Evaluate S_5 (e-basis form) at (x1..x4, xr) in F_{Q^4}.
pub fn eval_s5(f: &Fq4, terms: &[ETerm], b: &E4, x: &[E4; 4], xr: &E4) -> E4 {
    let mut e = [ZERO; 5];
    e[0] = ONE;
    for xi in x {
        for j in (1..5).rev() {
            let t = f.mul(&e[j - 1], xi);
            e[j] = add(&e[j], &t);
        }
    }
    let mut s = ZERO;
    for t in terms {
        let mut v = f.pow(xr, t.j as u64);
        v = f.mul(&v, &f.pow(b, t.l as u64));
        for i in 0..4 {
            if t.i[i] > 0 {
                v = f.mul(&v, &f.pow(&e[i + 1], t.i[i] as u64));
            }
        }
        s = add(&s, &v);
    }
    s
}

/// Order of E(F_{Q^4}) = 2 + 2 #{x != 0 lifting}, by direct count (small k only).
pub fn order_direct(cv: &Curve) -> u64 {
    let f = cv.f;
    let k = f.f.k;
    let mut n = 0u64;
    for v in 1u64..(1u64 << (4 * k)) {
        let x = f.unpack(v);
        if cv.lifts(&x) {
            n += 1;
        }
    }
    2 + 2 * n
}

/// Elements of F_Q embedded: [y,0,0,0]
pub fn fq_el(y: u16) -> E4 {
    [y, 0, 0, 0]
}

/// Factor base of a line {u y : y in F_Q^*} (u = ONE: standard base): both
/// signs of every lifting abscissa.
pub fn build_fb(cv: &Curve, u: &E4) -> (Vec<Pt>, u64) {
    let f = cv.f;
    let mut fb = vec![];
    let mut lifting = 0u64;
    for y in 1..f.f.q {
        let x = f.mul(u, &fq_el(y as u16));
        if let Some(p) = cv.lift(&x) {
            lifting += 1;
            fb.push(p);
            fb.push(cv.neg(&p));
        }
    }
    (fb, lifting)
}

pub fn sample_targets(cv: &Curve, rng: &mut Rng, n: usize) -> Vec<Pt> {
    let f = cv.f;
    (0..n)
        .map(|_| loop {
            let x = rng.e4(f);
            if is_zero(&x) {
                continue;
            }
            if let Some(p) = cv.lift(&x) {
                break if rng.next() & 1 == 1 { cv.neg(&p) } else { p };
            }
        })
        .collect()
}

/// Is z in F_Q (z^Q = z)?
pub fn in_fq(f: &Fq4, z: &E4) -> bool {
    f.frob(z) == *z
}

pub fn fmt_e4(f: &Fq4, z: &E4) -> String {
    format!("{:#x}", f.pack(z))
}

/// Random u in ker T (u = z^Q + z, Hilbert 90 for the cyclic Gal(F_{Q^4}/F_Q))
/// with T(sqrt(b)/u) = 0 (probability 1/Q). Rejects u whose line u F_Q meets
/// F_16 (those are the "u in F_16" lines of family A) and the exceptional u = 0.
pub fn random_u_outside_f16(f: &Fq4, b: &E4, rng: &mut Rng, avoid: &[E4]) -> E4 {
    let c = f.sqrt(b);
    loop {
        let z = rng.e4(f);
        let u = add(&f.frob(&z), &z);
        if is_zero(&u) {
            continue;
        }
        debug_assert!(f.tr_q(&u) == 0);
        if f.tr_q(&f.mul(&c, &f.inv(&u))) != 0 {
            continue;
        }
        let iu = f.inv(&u);
        let mut meets = false;
        for v in 1..16u8 {
            if in_fq(f, &f.mul(&f.f16(v), &iu)) {
                meets = true;
            }
        }
        // different line from every u in `avoid`
        for w in avoid {
            if in_fq(f, &f.mul(w, &iu)) {
                meets = true;
            }
        }
        if !meets {
            return u;
        }
    }
}

/// All valid u in F_16^* for (b): T(u) = T(sqrt(b)/u) = 0.
pub fn valid_u_f16(f: &Fq4, b: &E4) -> Vec<u8> {
    let c = f.sqrt(b);
    (1..16u8)
        .filter(|&v| {
            let u = f.f16(v);
            f.tr_q(&u) == 0 && f.tr_q(&f.mul(&c, &f.inv(&u))) == 0
        })
        .collect()
}
