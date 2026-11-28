use hec_native_ic::{
    fp::Fp,
    jac::{Curve, Div},
    poly::Poly,
};
mod data {
    include!("terms.rs");
}
pub type K = [u64; 4];
const PAIRS: [(usize, usize); 10] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (1, 1),
    (1, 2),
    (1, 3),
    (2, 2),
    (2, 3),
    (3, 3),
];
fn index(e: [u8; 4]) -> usize {
    PAIRS
        .iter()
        .position(|&(i, j)| {
            let mut v = [0; 4];
            v[i] += 1;
            v[j] += 1;
            v == e
        })
        .unwrap()
}
fn mons(k: &K, f: &Fp) -> [u64; 10] {
    let mut v = [0; 10];
    for (t, &(i, j)) in PAIRS.iter().enumerate() {
        v[t] = if k[i] == 0 || k[j] == 0 {
            0
        } else if k[i] == 1 {
            k[j]
        } else if k[j] == 1 {
            k[i]
        } else {
            f.mul(k[i], k[j])
        }
    }
    v
}
fn constant(f: &Fp, a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else if b == 1 {
        a
    } else if b == f.p - 1 {
        f.neg(a)
    } else {
        f.mul(a, b)
    }
}
pub fn project(c: &Curve, d: &Div) -> K {
    match d.u.deg() {
        0 => [0, 0, 0, 1],
        1 => {
            let r = c.fp.neg(d.u.c[0]);
            [0, 1, r, c.fp.mul(r, r)]
        }
        2 => {
            let a = d.u.c[1];
            [
                1,
                c.fp.neg(a),
                d.u.c[0],
                c.fp.add(
                    c.fp.mul(d.v.c[1], d.v.c[1]),
                    c.fp.mul(a, c.fp.sub(c.fp.mul(a, a), d.u.c[0])),
                ),
            ]
        }
        _ => panic!("genus2 projection only"),
    }
}
pub fn same(f: &Fp, a: &K, b: &K) -> bool {
    let Some(j) = a.iter().position(|&v| v != 0) else {
        return false;
    };
    b[j] != 0 && (0..4).all(|i| f.mul(a[i], b[j]) == f.mul(b[i], a[j]))
}
pub struct Compiled {
    pub q: K,
    coefs: [[u64; 10]; 16],
}
impl Compiled {
    pub fn new(c: &Curve, d: &Div) -> Self {
        assert_eq!(c.g, 2);
        assert_eq!(c.f.c[5], 1);
        assert!(c.f.c[1..5].iter().all(|&v| v == 0));
        let f = &c.fp;
        let q = project(c, d);
        let qm = mons(&q, f);
        let max = data::FORMS
            .iter()
            .flat_map(|v| v.iter())
            .map(|t| t.1 as usize)
            .max()
            .unwrap();
        let mut cp = vec![1; max + 1];
        for j in 1..cp.len() {
            cp[j] = f.mul(cp[j - 1], c.f.c[0])
        }
        let mut coefs = [[0u64; 10]; 16];
        for (j, terms) in data::FORMS.iter().enumerate() {
            for &(v, p, k, l) in *terms {
                let m = qm[index(l)];
                if m == 0 {
                    continue;
                }
                let weight = constant(f, m, cp[p as usize]);
                let weight = constant(f, weight, f.from_i64(v));
                let i = index(k);
                coefs[j][i] = f.add(coefs[j][i], weight)
            }
        }
        Self { q, coefs }
    }
    fn form(&self, f: &Fp, m: &[u64; 10], i: usize, j: usize) -> u64 {
        let mut out = 0;
        for t in 0..10 {
            if m[t] != 0 && self.coefs[4 * i + j][t] != 0 {
                out = f.add(out, constant(f, m[t], self.coefs[4 * i + j][t]))
            }
        }
        out
    }
    pub fn diagonal(&self, f: &Fp, p: &K) -> u64 {
        let a = f.sub(constant(f, p[0], self.q[3]), constant(f, p[3], self.q[0]));
        let b = f.sub(constant(f, p[2], self.q[1]), constant(f, p[1], self.q[2]));
        let s = f.add(a, b);
        f.mul(s, s)
    }
    pub fn check_diagonal(&self, f: &Fp, p: &K) {
        assert_eq!(self.diagonal(f, p), self.form(f, &mons(p, f), 0, 0))
    }
    pub fn step(&self, f: &Fp, p: &K, difference: &K) -> (K, usize) {
        let j = difference.iter().position(|&v| v != 0).unwrap();
        let mon = mons(p, f);
        let mut b = [0; 4];
        for i in 0..4 {
            b[i] = if i == 0 && j == 0 {
                self.diagonal(f, p)
            } else {
                self.form(f, &mon, i, j)
            }
        }
        let lead = b[j];
        let z = difference[j];
        let mut w = [0; 4];
        for i in 0..4 {
            w[i] = if i == j {
                f.mul(lead, z)
            } else {
                let v = f.mul(b[i], z);
                f.sub(f.add(v, v), f.mul(lead, difference[i]))
            }
        }
        assert!(w.iter().any(|&v| v != 0));
        (w, j)
    }
}
pub struct Decoded {
    pub u: Poly,
    pub k4: u64,
    pub roots: Vec<(u64, u32)>,
}
pub fn decode(c: &Curve, k: &K) -> Option<Decoded> {
    let f = &c.fp;
    if k[0] == 0 {
        if k[1] == 0 {
            assert_eq!(k[2], 0);
            return Some(Decoded {
                u: Poly::one(),
                k4: 0,
                roots: vec![],
            });
        }
        let r = f.mul(k[2], f.inv(k[1]));
        return Some(Decoded {
            u: Poly::x_minus(r, f),
            k4: 0,
            roots: vec![(r, 1)],
        });
    }
    let v = f.mul(k[0], k[2]);
    let four = f.add(f.add(v, v), f.add(v, v));
    let delta = f.sub(f.mul(k[1], k[1]), four);
    let s = f.sqrt(delta)?;
    let inverse = f.inv(f.add(k[0], k[0]));
    let mut roots = vec![
        (f.mul(f.sub(k[1], s), inverse), 1),
        (f.mul(f.add(k[1], s), inverse), 1),
    ];
    roots.sort();
    if roots[0].0 == roots[1].0 {
        roots = vec![(roots[0].0, 2)]
    }
    let inv = f.add(inverse, inverse);
    let a1 = f.neg(f.mul(k[1], inv));
    let a0 = f.mul(k[2], inv);
    let k4 = f.mul(k[3], inv);
    Some(Decoded {
        u: Poly::from_coeffs(&[a0, a1, 1]),
        k4,
        roots,
    })
}
pub fn recover(c: &Curve, d: &Div, next: &K, a: &Decoded) -> Div {
    let f = &c.fp;
    let guess = match a.u.deg() {
        0 => return c.zero(),
        1 => {
            let x = a.roots[0].0;
            let y = f.sqrt(c.f.eval(x, f)).unwrap();
            c.place(x, y)
        }
        2 => {
            let a0 = a.u.c[0];
            let a1 = a.u.c[1];
            let aa = f.mul(a1, a1);
            let b1sq = f.sub(a.k4, f.mul(a1, f.sub(aa, a0)));
            let b1 = f.sqrt(b1sq).unwrap();
            let b0 = if b1 != 0 {
                let fourth = f.mul(aa, aa);
                let t = f.mul(aa, a0);
                let three = f.add(f.add(t, t), t);
                let rx = f.add(f.sub(fourth, three), f.mul(a0, a0));
                let numerator = f.add(rx, f.mul(a1, b1sq));
                f.mul(numerator, f.inv(f.add(b1, b1)))
            } else {
                let t = f.mul(a0, f.sub(aa, f.add(a0, a0)));
                let sq = f.add(c.f.c[0], f.mul(a1, t));
                f.sqrt(sq).unwrap()
            };
            Div {
                u: a.u,
                v: Poly::from_coeffs(&[b0, b1]),
            }
        }
        _ => unreachable!(),
    };
    let probe = best_add(c, &guess, d);
    if same(f, &project(c, &probe), next) {
        guess
    } else {
        c.neg(&guess)
    }
}
pub fn points(c: &Curve, d: &Div, roots: &[(u64, u32)]) -> Vec<(u64, u64, u32)> {
    roots
        .iter()
        .map(|&(x, n)| (x, d.v.eval(x, &c.fp), n))
        .collect()
}

pub fn best_add(c: &Curve, p: &Div, d: &Div) -> Div {
    if d.u.deg() == 1 {
        c.add_place(p, c.fp.neg(d.u.c[0]), d.v.c[0])
    } else {
        c.add(p, d)
    }
}
