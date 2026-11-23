//! P4 audit: [K:F_2] for K = F_2(u F_q, a, b) on the real curves, GF(2^N) in
//! a polynomial basis (u64 limbs), and the Amendment 1 trap condition.
use crate::json;

struct Gf {
    n: usize,
    limbs: usize,
    poly: Vec<u64>,
}

fn hex_to_limbs(hex: &str) -> Vec<u64> {
    let d: Vec<u64> = hex.trim_start_matches("0x").chars().filter(|c| c.is_ascii_hexdigit()).map(|c| c.to_digit(16).unwrap() as u64).collect();
    let mut out = vec![0u64; d.len() / 16 + 1];
    for (i, &v) in d.iter().rev().enumerate() {
        out[i / 16] |= v << (4 * (i % 16));
    }
    out
}

fn bit(a: &[u64], i: usize) -> bool {
    i / 64 < a.len() && (a[i / 64] >> (i % 64)) & 1 == 1
}

impl Gf {
    fn new(poly_hex: &str) -> Gf {
        let p = hex_to_limbs(poly_hex);
        let n = (0..p.len() * 64).rev().find(|&i| bit(&p, i)).unwrap();
        let limbs = n / 64 + 1;
        let mut poly = p;
        poly.resize(limbs + 1, 0);
        Gf { n, limbs, poly }
    }
    fn el(&self, hex: &str) -> Vec<u64> {
        let mut e = hex_to_limbs(hex);
        e.resize(self.limbs, 0);
        assert!(e.len() == self.limbs, "element wider than the field");
        e
    }
    fn mul(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        let mut r = vec![0u64; 2 * self.limbs + 1];
        for (i, &aw) in a.iter().enumerate() {
            for j in 0..64 {
                if (aw >> j) & 1 == 1 {
                    for (k, &bw) in b.iter().enumerate() {
                        r[i + k] ^= bw << j;
                        if j > 0 {
                            r[i + k + 1] ^= bw >> (64 - j);
                        }
                    }
                }
            }
        }
        for i in (self.n..r.len() * 64).rev() {
            if bit(&r, i) {
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
    /// least j >= 1 with z^(2^j) = z (z = 0 or 1 gives 1)
    fn deg(&self, z: &[u64]) -> usize {
        let mut w = self.mul(z, z);
        let mut j = 1;
        while w != z {
            w = self.mul(&w, &w);
            j += 1;
            assert!(j <= self.n, "degree above N: field polynomial not irreducible?");
        }
        j
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

pub fn audit(_c2pnb_params: &str, oakley_rows: &str) {
    let rows = json::parse(&std::fs::read_to_string(oakley_rows).unwrap());
    for r in rows.arr() {
        let curve = r.get("curve").str();
        let n = r.get("N").num().unwrap() as usize;
        let d = r.get("d").num().unwrap() as usize;
        let g = Gf::new(r.get("field_poly_hex").str());
        assert_eq!(g.n, n);
        let a = g.el(r.get("a_hex").str());
        let b = g.el(r.get("b_hex").str());
        let (da, db) = (g.deg(&a), g.deg(&b));
        // the c2pnb.rs pi_16-stable lines: u in F_{2^16}, F_q = F_{2^{N/4}};
        // trapped only if [K:F_2] = N/4, i.e. u, a, b all in F_{2^gcd(16, N/4)}
        let pi16 = if n % 16 == 0 {
            let small = gcd(16, n / 4);
            format!(
                "{{\"field_of_trap\":{small},\"a_in_it\":{},\"b_in_it\":{},\"any_u_can_trap\":{}}}",
                small % da == 0,
                small % db == 0,
                small % da == 0 && small % db == 0
            )
        } else {
            "null".into()
        };
        let mut lines = vec![];
        for t in r.get("twisted").arr() {
            let u = g.el(t.get("u_hex").str());
            let du = g.deg(&u);
            let dk = lcm(lcm(lcm(d, du), da), db);
            let trapped = dk < n && 2 * dk != n;
            lines.push(format!(
                "{{\"u\":\"{}\",\"deg_u\":{du},\"deg_K\":{dk},\"trapped\":{trapped}}}",
                t.get("u_hex").str()
            ));
        }
        println!(
            "{{\"curve\":\"{curve}\",\"N\":{n},\"d\":{d},\"Tr_a\":{},\"deg_a\":{da},\"deg_b\":{db},\"lines\":[{}],\"c2pnb_rs_pi16_lines\":{pi16}}}",
            r.get("Tr_N_a").num().map(|x| x.to_string()).unwrap_or("null".into()),
            lines.join(",")
        );
    }
}
