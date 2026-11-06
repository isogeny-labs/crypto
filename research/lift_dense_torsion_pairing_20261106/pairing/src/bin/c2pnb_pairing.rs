//! P5: T-stable lines among the pi_16-stable all-lift lines of the c2pnb
//! curves (N = 16k, Q = 2^(4k), u in F_{2^16}). Own multi-limb GF(2^N)
//! arithmetic; reads the OpenSSL parameter dump used by the Oct 27 study.
//!
//!   c2pnb_pairing <binary-curve-params.txt> <out.jsonl>

use std::io::Write as _;

const L: usize = 6; // 384 bits >= 368
type El = [u64; L];
const ZERO: El = [0; L];

const CURVES: [&str; 5] = ["c2pnb176v1", "c2pnb208w1", "c2pnb272w1", "c2pnb304w1", "c2pnb368w1"];

fn deg(a: &[u64]) -> Option<usize> {
    (0..a.len()).rev().find(|&i| a[i] != 0).map(|i| i * 64 + 63 - a[i].leading_zeros() as usize)
}

struct G {
    n: usize,
    poly: [u64; 2 * L],
}
impl G {
    fn reduce(&self, mut r: [u64; 2 * L]) -> El {
        let n = self.n;
        while let Some(dg) = deg(&r) {
            if dg < n {
                break;
            }
            let sh = dg - n;
            let (ws, bs) = (sh / 64, sh % 64);
            for i in (0..2 * L).rev() {
                let w = self.poly[i];
                if w == 0 {
                    continue;
                }
                if i + ws < 2 * L {
                    r[i + ws] ^= w << bs;
                }
                if bs > 0 && i + ws + 1 < 2 * L {
                    r[i + ws + 1] ^= w >> (64 - bs);
                }
            }
        }
        let mut o = ZERO;
        o.copy_from_slice(&r[..L]);
        o
    }
    fn mul(&self, a: &El, b: &El) -> El {
        let mut r = [0u64; 2 * L];
        for i in 0..L {
            let mut w = a[i];
            while w != 0 {
                let j = w.trailing_zeros() as usize;
                w &= w - 1;
                for k in 0..L {
                    r[i + k] ^= b[k] << j;
                    if j > 0 {
                        r[i + k + 1] ^= b[k] >> (64 - j);
                    }
                }
            }
        }
        self.reduce(r)
    }
    fn sq(&self, a: &El) -> El {
        let mut r = [0u64; 2 * L];
        for i in 0..L {
            let (mut lo, mut hi) = (0u64, 0u64);
            for j in 0..32 {
                lo |= ((a[i] >> j) & 1) << (2 * j);
                hi |= ((a[i] >> (j + 32)) & 1) << (2 * j);
            }
            r[2 * i] = lo;
            r[2 * i + 1] = hi;
        }
        self.reduce(r)
    }
    fn frob(&self, a: &El, k: usize) -> El {
        let mut z = *a;
        for _ in 0..k {
            z = self.sq(&z);
        }
        z
    }
    fn sqrt(&self, a: &El) -> El {
        self.frob(a, self.n - 1)
    }
    /// sum_{j < N/d} z^(2^(d j))
    fn tr_rel(&self, a: &El, d: usize) -> El {
        let (mut t, mut z) = (ZERO, *a);
        for _ in 0..self.n / d {
            for i in 0..L {
                t[i] ^= z[i];
            }
            z = self.frob(&z, d);
        }
        t
    }
    /// Inverse of u in F_{2^16} (checked): u^(2^16 - 2).
    fn inv16(&self, u: &El) -> El {
        let mut r = [0u64; L];
        r[0] = 1;
        let mut z = *u;
        for _ in 1..16 {
            z = self.sq(&z);
            r = self.mul(&r, &z);
        }
        r
    }
}

fn hex(s: &str) -> Vec<u64> {
    let d: Vec<u64> = s.chars().filter(|c| c.is_ascii_hexdigit()).map(|c| c.to_digit(16).unwrap() as u64).collect();
    let mut o = vec![0u64; d.len() / 16 + 1];
    for (i, &v) in d.iter().rev().enumerate() {
        o[i / 16] |= v << (4 * (i % 16));
    }
    o
}

fn field_of(block: &str, key: &str) -> Vec<u64> {
    let mut acc = String::new();
    let mut on = false;
    for l in block.lines() {
        let t = l.trim();
        if on {
            if t.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
                break;
            }
            acc.push_str(t);
        } else if let Some(rest) = t.strip_prefix(&format!("{key}:")) {
            on = true;
            acc.push_str(rest);
        }
    }
    hex(&acc.replace(':', ""))
}

fn el(v: &[u64]) -> El {
    let mut o = ZERO;
    for (i, &w) in v.iter().enumerate() {
        if w != 0 {
            o[i] = w;
        }
    }
    o
}

fn is_zero(a: &El) -> bool {
    a.iter().all(|&w| w == 0)
}
fn xor(a: &El, b: &El) -> El {
    let mut o = *a;
    for i in 0..L {
        o[i] ^= b[i];
    }
    o
}
fn key(a: &El) -> [u64; L] {
    let mut k = *a;
    k.reverse();
    k
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let text = std::fs::read_to_string(&args[1]).expect("params");
    let mut fh = std::fs::File::create(&args[2]).unwrap();
    for name in CURVES {
        let block = text.split(&format!("===== {name} =====")).nth(1).unwrap().split("=====").next().unwrap();
        let pv = field_of(block, "Polynomial");
        let mut poly = [0u64; 2 * L];
        poly[..pv.len()].copy_from_slice(&pv);
        let n = deg(&poly).unwrap();
        let g = G { n, poly };
        let a = el(&field_of(block, "A"));
        let b = el(&field_of(block, "B"));
        assert!(n % 16 == 0);
        let k = n / 16;
        let qd = 4 * k; // Q = 2^(4k), [F_N : F_Q] = 4
        // x^(2^N) = x sanity (irreducibility is the standard's claim)
        let mut x = ZERO;
        x[0] = 2;
        assert_eq!(g.frob(&x, n), x);
        let tr_a = g.tr_rel(&a, 1)[0] & 1;
        let in16 = |z: &El| g.frob(z, 16) == *z;
        assert!(in16(&a) && in16(&b), "{name}: a, b not in F_2^16");
        let rb = g.sqrt(&b);
        let b4 = g.sqrt(&rb);
        // basis of F_{2^16}: traces Tr_{N/16}(w) of x^i
        let mut basis: Vec<El> = vec![];
        let mut piv: Vec<(usize, El)> = vec![];
        let mut w = ZERO;
        w[0] = 1;
        let mut tries = 0;
        while basis.len() < 16 {
            tries += 1;
            assert!(tries < 2000);
            w = g.mul(&w, &x);
            let t = g.tr_rel(&w, 16);
            assert!(in16(&t));
            let mut r = t;
            loop {
                match deg(&r) {
                    None => break,
                    Some(h) => {
                        if let Some((_, p)) = piv.iter().find(|(ph, _)| *ph == h) {
                            r = xor(&r, p);
                        } else {
                            piv.push((h, r));
                            basis.push(t);
                            break;
                        }
                    }
                }
            }
        }
        // T = Tr_{N/Q}; on F_{2^16}, z^Q = z^(2^(4k mod 16))
        let s = qd % 16;
        let tq = |z: &El| -> El {
            let (mut t, mut y) = (ZERO, *z);
            for _ in 0..4 {
                t = xor(&t, &y);
                y = g.frob(&y, s);
            }
            t
        };
        let mut f16: Vec<El> = vec![];
        let mut u = ZERO;
        let (mut ker, mut valid, mut tst) = (0u64, 0u64, 0u64);
        let mut full_checked = 0;
        let mut valid_us: Vec<El> = vec![];
        for gc in 1u32..(1 << 16) {
            u = xor(&u, &basis[gc.trailing_zeros() as usize]);
            let ui = g.inv16(&u);
            let mut one = ZERO;
            one[0] = 1;
            assert_eq!(g.mul(&u, &ui), one);
            let t1 = tq(&u);
            let c = g.mul(&rb, &ui);
            let t2 = tq(&c);
            if full_checked < 64 {
                // cross-check the shortcut against the full relative trace
                assert_eq!(t1, g.tr_rel(&u, qd));
                assert_eq!(t2, g.tr_rel(&c, qd));
                full_checked += 1;
            }
            if g.frob(&u, 4) == u {
                f16.push(u);
            }
            if is_zero(&t1) {
                ker += 1;
                if is_zero(&t2) {
                    valid += 1;
                    let sv = g.mul(&rb, &g.mul(&ui, &ui));
                    if g.frob(&sv, qd) == sv {
                        tst += 1;
                    }
                    valid_us.push(u);
                }
            }
        }
        assert_eq!(f16.len(), 15);
        // lines: classes of u under F_{2^4}^* = F_Q^* ∩ F_{2^16}
        let mut canon: Vec<[u64; L]> = valid_us
            .iter()
            .map(|u| f16.iter().map(|c| key(&g.mul(u, c))).min().unwrap())
            .collect();
        canon.sort_unstable();
        let mut sizes: Vec<usize> = vec![];
        let mut i = 0;
        while i < canon.len() {
            let j = (i..canon.len()).find(|&j| canon[j] != canon[i]).unwrap_or(canon.len());
            sizes.push(j - i);
            i = j;
        }
        let lines = sizes.len();
        let all15 = sizes.iter().all(|&s| s == 15);
        let tr_b4_zero = is_zero(&g.tr_rel(&b4, qd));
        let rb_in_q = g.frob(&rb, qd) == rb;
        let b4_valid_line = {
            let ok = is_zero(&tq(&b4)) && is_zero(&tq(&g.mul(&rb, &g.inv16(&b4))));
            ok
        };
        let row = format!(
            "{{\"kind\":\"c2pnb\",\"curve\":\"{name}\",\"N\":{n},\"k\":{k},\"Q_bits\":{qd},\"Tr_a\":{tr_a},\"u_kernel_T\":{ker},\"u_valid\":{valid},\"lines\":{lines},\"class_size_15\":{all15},\"u_tstable\":{tst},\"tr_b4_zero\":{tr_b4_zero},\"b4_line_valid\":{b4_valid_line},\"sqrt_b_in_FQ\":{rb_in_q},\"u_valid_excl_tstable\":{}}}",
            valid - tst
        );
        eprintln!("{row}");
        writeln!(fh, "{row}").unwrap();
    }
}
