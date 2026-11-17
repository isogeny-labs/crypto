//! All-lift F_Q-lines u*F_Q for the composite-degree c2pnb curves (native
//! replacement of the historical `c2pnb/c2pnb_lines.py`, Amendment 3).
//!
//! E: y^2 + xy = x^3 + a x^2 + b over F_{2^N}, N = 16k, a, b in F_{2^16},
//! Q = 2^{4k}. For x = u y, y in F_Q, u in F_{2^16}:
//!   Tr_N(b/x^2) = Tr_N(c/x) = Tr_Q(T(c/u)/y)   (c = sqrt b),
//!   Tr_N(x)     = Tr_Q(T(u) y),                T = Tr_{F_{2^N}/F_Q},
//! so T(u) = T(c/u) = 0 and Tr_N(a) = 0 make every nonzero x in u F_Q lift,
//! and u in F_{2^16} keeps the line pi_16-stable. This program checks the
//! hypotheses on the real parameters, counts such u exactly, and verifies
//! lifting by direct computation on sampled line elements.
//!
//! Usage: c2pnb <binary-curve-params.txt> <out.json> [samples]

use std::fmt::Write as _;

const CURVES: [&str; 5] = [
    "c2pnb176v1",
    "c2pnb208w1",
    "c2pnb272w1",
    "c2pnb304w1",
    "c2pnb368w1",
];

type El = Vec<u64>;

struct Gf {
    n: usize,
    limbs: usize,
    poly: El, // includes the x^N term
}

fn bit(a: &[u64], i: usize) -> bool {
    (a[i / 64] >> (i % 64)) & 1 == 1
}

fn degree(a: &[u64]) -> Option<usize> {
    for i in (0..a.len()).rev() {
        if a[i] != 0 {
            return Some(i * 64 + 63 - a[i].leading_zeros() as usize);
        }
    }
    None
}

impl Gf {
    fn new(poly: El) -> Gf {
        let n = degree(&poly).expect("zero polynomial");
        let limbs = n / 64 + 1;
        let mut p = poly;
        p.resize(limbs.max(p.len()), 0);
        Gf { n, limbs, poly: p }
    }
    fn zero(&self) -> El {
        vec![0; self.limbs]
    }
    fn one(&self) -> El {
        let mut z = self.zero();
        z[0] = 1;
        z
    }
    fn is_zero(&self, a: &[u64]) -> bool {
        a.iter().all(|&w| w == 0)
    }
    fn add(&self, a: &[u64], b: &[u64]) -> El {
        a.iter().zip(b).map(|(x, y)| x ^ y).collect()
    }
    fn mul(&self, a: &[u64], b: &[u64]) -> El {
        // carryless product into 2 * limbs words, then reduce
        let mut r = vec![0u64; 2 * self.limbs + 1];
        for (i, &aw) in a.iter().enumerate() {
            for j in 0..64 {
                if (aw >> j) & 1 == 1 {
                    // r ^= b << (64 i + j)
                    let sh = j;
                    for (k, &bw) in b.iter().enumerate() {
                        r[i + k] ^= bw << sh;
                        if sh > 0 {
                            r[i + k + 1] ^= bw >> (64 - sh);
                        }
                    }
                }
            }
        }
        self.reduce(r)
    }
    fn reduce(&self, mut r: El) -> El {
        let top = r.len() * 64;
        for i in (self.n..top).rev() {
            if bit(&r, i) {
                // r ^= poly << (i - n)
                let s = i - self.n;
                let (w, b) = (s / 64, s % 64);
                for (k, &pw) in self.poly.iter().enumerate() {
                    if k + w < r.len() {
                        r[k + w] ^= pw << b;
                    }
                    if b > 0 && k + w + 1 < r.len() {
                        r[k + w + 1] ^= pw >> (64 - b);
                    }
                }
            }
        }
        r.truncate(self.limbs);
        r
    }
    fn sq(&self, a: &[u64]) -> El {
        self.mul(a, a)
    }
    fn frob(&self, a: &[u64], k: usize) -> El {
        let mut z = a.to_vec();
        for _ in 0..k {
            z = self.sq(&z);
        }
        z
    }
    fn pow_2k_minus_2(&self, a: &[u64], k: usize) -> El {
        // a^(2^k - 2) = prod_{i=1}^{k-1} a^(2^i)
        let mut r = self.one();
        let mut z = self.sq(a);
        for _ in 1..k {
            r = self.mul(&r, &z);
            z = self.sq(&z);
        }
        r
    }
    fn inv(&self, a: &[u64]) -> El {
        assert!(!self.is_zero(a));
        self.pow_2k_minus_2(a, self.n)
    }
    fn tr_rel(&self, a: &[u64], d: usize) -> El {
        let mut t = self.zero();
        let mut z = a.to_vec();
        for _ in 0..self.n / d {
            t = self.add(&t, &z);
            z = self.frob(&z, d);
        }
        t
    }
    fn tr(&self, a: &[u64]) -> u64 {
        let t = self.tr_rel(a, 1);
        assert!(t[1..].iter().all(|&w| w == 0) && t[0] <= 1);
        t[0]
    }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn element(&mut self, g: &Gf) -> El {
        let mut e: El = (0..g.limbs).map(|_| self.next()).collect();
        let extra = g.limbs * 64 - g.n;
        let last = g.limbs - 1;
        e[last] &= u64::MAX >> extra;
        e
    }
}

fn hex_to_el(hex: &str) -> El {
    let digits: Vec<u8> = hex
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect();
    let mut out = vec![0u64; digits.len() / 16 + 1];
    for (i, &dg) in digits.iter().rev().enumerate() {
        out[i / 16] |= (dg as u64) << (4 * (i % 16));
    }
    out
}

/// The hex value after `key:` in an OpenSSL parameter dump block, up to the
/// next `Key:` line.
fn field(block: &str, key: &str) -> El {
    let mut lines = block.lines();
    let mut acc = String::new();
    let mut on = false;
    for l in &mut lines {
        let t = l.trim_start();
        if on {
            if t.contains(':') && t.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
                break;
            }
            acc.push_str(t);
        } else if let Some(rest) = t.strip_prefix(&format!("{key}:")) {
            on = true;
            acc.push_str(rest);
        }
    }
    let acc: String = acc
        .chars()
        .filter(|c| !c.is_whitespace() && *c != ':')
        .collect();
    hex_to_el(&acc)
}

fn rank_insert(basis: &mut Vec<El>, piv: &mut Vec<(usize, El)>, v: El) -> bool {
    let mut x = v.clone();
    loop {
        let Some(h) = degree(&x) else {
            return false;
        };
        if let Some((_, p)) = piv.iter().find(|(ph, _)| *ph == h) {
            x = x.iter().zip(p).map(|(a, b)| a ^ b).collect();
        } else {
            piv.push((h, x));
            basis.push(v);
            return true;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let text = std::fs::read_to_string(&args[1]).expect("params file");
    let out = &args[2];
    let samples: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(200);
    let mut rows = vec![];
    for name in CURVES {
        let block = text
            .split(&format!("===== {name} ====="))
            .nth(1)
            .expect("curve block")
            .split("=====")
            .next()
            .unwrap();
        let g = Gf::new(field(block, "Polynomial"));
        let pad = |mut e: El| {
            e.resize(g.limbs, 0);
            e
        };
        let a = pad(field(block, "A"));
        let b = pad(field(block, "B"));
        let n = g.n;
        let k = n / 16;
        let in16 = |z: &El| g.frob(z, 16) == *z;
        let mut rng = Rng(0x11F7_2026_1027 ^ n as u64);
        let tra = g.tr(&a);
        // F_{2^16}: span of 16 independent Tr_{N/16} images
        let (mut b16, mut piv) = (vec![], vec![]);
        while b16.len() < 16 {
            let z = rng.element(&g);
            let t = g.tr_rel(&z, 16);
            rank_insert(&mut b16, &mut piv, t);
        }
        assert!(b16.iter().all(in16));
        // T restricted to F_{2^16}: u^(2^{4kj}) = u^(2^{(4kj) mod 16})
        let t16 = |u: &El| {
            let mut t = g.zero();
            for j in 0..4 {
                t = g.add(&t, &g.frob(u, (4 * k * j) % 16));
            }
            t
        };
        let imgs: Vec<El> = b16.iter().map(&t16).collect();
        let mut ker: Vec<El> = vec![];
        let (mut u, mut tu) = (g.zero(), g.zero());
        for gc in 1u64..(1u64 << 16) {
            let i = gc.trailing_zeros() as usize;
            u = g.add(&u, &b16[i]);
            tu = g.add(&tu, &imgs[i]);
            if g.is_zero(&tu) {
                ker.push(u.clone());
            }
        }
        let c = g.frob(&b, n - 1); // sqrt(b)
        assert_eq!(g.sq(&c), b);
        assert!(in16(&c));
        let mut good: Vec<El> = vec![];
        for u in &ker {
            let uinv = g.pow_2k_minus_2(u, 16); // inverse inside F_{2^16}
            assert_eq!(g.mul(u, &uinv), g.one());
            if g.is_zero(&t16(&g.mul(&c, &uinv))) {
                good.push(u.clone());
            }
        }
        let lifts = |x: &El| {
            let w = g.add(&g.add(x, &a), &g.mul(&b, &g.inv(&g.sq(x))));
            g.tr(&w) == 0
        };
        let sample_fq = |rng: &mut Rng| loop {
            let y = g.tr_rel(&rng.element(&g), 4 * k);
            if !g.is_zero(&y) {
                break y;
            }
        };
        let (mut tw_hits, mut std_hits) = (0usize, 0usize);
        for _ in 0..samples {
            let y = sample_fq(&mut rng);
            if let Some(u0) = good.first() {
                tw_hits += lifts(&g.mul(u0, &y)) as usize;
            }
            std_hits += lifts(&y) as usize;
        }
        let mut row = String::new();
        let _ = write!(
            row,
            "{{\"curve\":\"{name}\",\"N\":{n},\"k\":{k},\"a_in_F16\":{},\"b_in_F16\":{},\"Tr_a\":{tra},\"u_kernel_T\":{},\"u_all_lift\":{},\"samples\":{samples},\"sampled_lift_twisted\":{},\"sampled_lift_standard\":{std_hits}}}",
            in16(&a),
            in16(&b),
            ker.len(),
            good.len(),
            if good.is_empty() { "null".to_string() } else { tw_hits.to_string() }
        );
        eprintln!("{row}");
        rows.push(row);
    }
    std::fs::write(out, format!("[{}]\n", rows.join(",\n"))).unwrap();
}
