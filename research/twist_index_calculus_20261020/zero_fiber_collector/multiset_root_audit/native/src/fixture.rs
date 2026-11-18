// Support from the frozen finite-residual native auditor, lines 1..488.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const WORKLOAD_SHA: &str = "17ecb4c302526bc58e592466071e1c2704eaaf43a0a0ca1efe563e1bd69b3c10";
const PRIMES: [u64; 7] = [37, 73, 97, 157, 229, 241, 337];
type E = [u64; 4];
type Terms = Vec<(usize, i8)>;
fn mm(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn neg(a: u64, n: u64) -> u64 {
    (n - a) % n
}
fn pow(mut a: u64, mut k: u64, n: u64) -> u64 {
    let mut v = 1;
    while k != 0 {
        if k & 1 != 0 {
            v = mm(v, a, n);
        }
        a = mm(a, a, n);
        k >>= 1;
    }
    v
}
fn digest(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
fn load(path: &Path, sha: &str) -> Value {
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(digest(&bytes), sha, "input hash: {}", path.display());
    serde_json::from_slice(&bytes).unwrap()
}
fn u(v: &Value) -> u64 {
    v.as_u64().unwrap()
}
fn s(v: &Value) -> &str {
    v.as_str().unwrap()
}
fn arr(v: &Value) -> E {
    serde_json::from_value(v.clone()).unwrap()
}
fn write(path: &Path, v: &Value) {
    std::fs::write(path, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}

#[derive(Clone, Copy)]
struct Field {
    p: u64,
    d: u64,
}
impl Field {
    fn new(p: u64) -> Self {
        assert!(PRIMES.contains(&p));
        Self {
            p,
            d: (2..p).find(|&d| pow(d, (p - 1) / 2, p) == p - 1).unwrap(),
        }
    }
    fn scalar(self, x: u64) -> E {
        [x % self.p, 0, 0, 0]
    }
    fn add(self, a: E, b: E) -> E {
        std::array::from_fn(|i| (a[i] + b[i]) % self.p)
    }
    fn neg(self, a: E) -> E {
        std::array::from_fn(|i| neg(a[i], self.p))
    }
    fn sub(self, a: E, b: E) -> E {
        self.add(a, self.neg(b))
    }
    fn mul(self, a: E, b: E) -> E {
        let mut c = [0u64; 7];
        for i in 0..4 {
            for j in 0..4 {
                c[i + j] = (c[i + j] + a[i] * b[j]) % self.p;
            }
        }
        for k in (4..7).rev() {
            c[k - 4] = (c[k - 4] + self.d * c[k]) % self.p;
        }
        [c[0], c[1], c[2], c[3]]
    }
    fn pow(self, mut a: E, mut k: u64) -> E {
        let mut v = self.scalar(1);
        while k != 0 {
            if k & 1 != 0 {
                v = self.mul(v, a);
            }
            a = self.mul(a, a);
            k >>= 1;
        }
        v
    }
    fn inv(self, a: E) -> E {
        assert_ne!(a, [0; 4]);
        self.pow(a, self.p.pow(4) - 2)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Point {
    x: E,
    y: E,
    inf: bool,
}
const O: Point = Point {
    x: [0; 4],
    y: [0; 4],
    inf: true,
};
fn reject(p: Point) -> bool {
    !p.inf && p.x[1..] == [0, 0, 0]
}
struct Curve {
    f: Field,
    b: E,
}
impl Curve {
    fn neg(&self, p: Point) -> Point {
        if p.inf {
            O
        } else {
            Point {
                y: self.f.neg(p.y),
                ..p
            }
        }
    }
    fn signed(&self, p: Point, sign: i8) -> Point {
        assert!([-1, 1].contains(&sign));
        if sign == 1 {
            p
        } else {
            self.neg(p)
        }
    }
    fn on(&self, p: Point) -> bool {
        p.inf || self.f.mul(p.y, p.y) == self.f.add(self.f.mul(self.f.mul(p.x, p.x), p.x), self.b)
    }
    fn add(&self, a: Point, b: Point) -> Point {
        if a.inf {
            return b;
        }
        if b.inf {
            return a;
        }
        let f = self.f;
        let m = if a.x == b.x {
            if a.y != b.y || a.y == [0; 4] {
                return O;
            }
            f.mul(
                f.mul(f.scalar(3), f.mul(a.x, a.x)),
                f.inv(f.mul(f.scalar(2), a.y)),
            )
        } else {
            f.mul(f.sub(b.y, a.y), f.inv(f.sub(b.x, a.x)))
        };
        let x = f.sub(f.sub(f.mul(m, m), a.x), b.x);
        Point {
            x,
            y: f.sub(f.mul(m, f.sub(a.x, x)), a.y),
            inf: false,
        }
    }
    fn mul(&self, mut a: Point, mut k: u64) -> Point {
        let mut v = O;
        while k != 0 {
            if k & 1 != 0 {
                v = self.add(v, a);
            }
            a = self.add(a, a);
            k >>= 1;
        }
        v
    }
}
#[derive(Clone)]
struct Factor {
    point: Point,
    col: usize,
    coefficient: u64,
}
struct Case {
    c: Curve,
    pool: Vec<Factor>,
    n: u64,
    original_len: usize,
    l0: usize,
    active: usize,
    anchor: u64,
    g: Point,
    actions: Vec<Vec<(usize, i8)>>,
}
impl Case {
    fn new(raw: &Value) -> Self {
        let p = u(&raw["p"]);
        let n = u(&raw["n"]);
        assert!(n > 3 && n % 2 == 1 && (3..=n.isqrt()).step_by(2).all(|d| n % d != 0));
        let c = Curve {
            f: Field::new(p),
            b: [0, 0, 0, u(&raw["b0"])],
        };
        let g = Point {
            x: arr(&raw["generator_x"]),
            y: arr(&raw["generator_y"]),
            inf: false,
        };
        assert!(c.on(g));
        assert_eq!(c.mul(g, n), O);
        let base: Vec<Point> = raw["base_coordinates"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| Point {
                x: arr(&v[0]),
                y: arr(&v[1]),
                inf: false,
            })
            .collect();
        assert!(base.iter().all(|&p| c.on(p) && p.x != [0; 4]));
        let l0 = base.iter().take_while(|p| p.x[1..] == [0, 0, 0]).count();
        assert!(l0 > 0 && l0 < base.len() && l0 % 12 == 0);
        let anchor = if base[0] == g {
            1
        } else {
            assert_eq!(base[0], c.neg(g));
            n - 1
        };
        let mut scalars = BTreeSet::new();
        for j in 0..4 {
            for k in 0..3 {
                let t = mm(
                    pow(u(&raw["lambda_frob"]), j, n),
                    pow(u(&raw["lambda_cubic"]), k, n),
                    n,
                );
                scalars.extend([t, neg(t, n)]);
            }
        }
        assert_eq!(scalars.len(), 24);
        assert!(!scalars.contains(&0));
        for &a in &scalars {
            for &b in &scalars {
                assert!(scalars.contains(&mm(a, b, n)));
            }
        }
        let byx: BTreeMap<E, usize> = base.iter().enumerate().map(|(i, p)| (p.x, i)).collect();
        assert_eq!(byx.len(), base.len());
        let mut cols = vec![usize::MAX; base.len()];
        let mut coefs = vec![0; base.len()];
        let mut next = 0;
        for i in 0..base.len() {
            if cols[i] != usize::MAX {
                continue;
            }
            let col = next;
            next += 1;
            for &scalar in &scalars {
                let im = c.mul(base[i], scalar);
                let id = byx[&im.x];
                let coef = if im == base[id] {
                    scalar
                } else {
                    assert_eq!(im, c.neg(base[id]));
                    neg(scalar, n)
                };
                if cols[id] == usize::MAX {
                    cols[id] = col;
                    coefs[id] = coef;
                } else {
                    assert_eq!((cols[id], coefs[id]), (col, coef));
                }
            }
        }
        assert_eq!(next * 12, base.len());
        let mut pool: Vec<Factor> = base
            .iter()
            .enumerate()
            .map(|(i, &point)| Factor {
                point,
                col: cols[i],
                coefficient: coefs[i],
            })
            .collect();
        let omega = u(&raw["omega"]);
        let delta = (n + 1 - u(&raw["lambda_cubic"])) % n;
        assert!(omega != 1 && pow(omega, 3, p) == 1 && delta != 0);
        for (i, &pt) in base.iter().enumerate() {
            let psi = Point {
                x: c.f.mul(c.f.scalar(omega), pt.x),
                ..pt
            };
            assert_eq!(psi, c.mul(pt, u(&raw["lambda_cubic"])));
            let point = c.add(pt, c.neg(psi));
            assert_eq!(point, c.mul(pt, delta));
            assert!(!point.inf && c.on(point));
            pool.push(Factor {
                point,
                col: cols[i],
                coefficient: mm(delta, coefs[i], n),
            });
        }
        for (i, &pt) in base[..l0].iter().enumerate() {
            let point = c.add(pt, pt);
            assert!(!point.inf && c.on(point));
            pool.push(Factor {
                point,
                col: cols[i],
                coefficient: mm(2, coefs[i], n),
            });
        }
        let all: BTreeMap<(E, E), (usize, i8)> = pool
            .iter()
            .enumerate()
            .flat_map(|(i, f)| {
                [
                    ((f.point.x, f.point.y), (i, 1)),
                    ((f.point.x, c.neg(f.point).y), (i, -1)),
                ]
            })
            .collect();
        assert_eq!(all.len(), 2 * pool.len());
        let actions: Vec<Vec<(usize, i8)>> = scalars
            .iter()
            .map(|&scalar| {
                pool.iter()
                    .map(|f| {
                        let im = c.mul(f.point, scalar);
                        let to = all[&(im.x, im.y)];
                        assert_eq!(pool[to.0].col, f.col);
                        let coeff = mm(f.coefficient, scalar, n);
                        assert_eq!(
                            pool[to.0].coefficient,
                            if to.1 == 1 { coeff } else { neg(coeff, n) }
                        );
                        to
                    })
                    .collect()
            })
            .collect();
        // Degree-four power basis for B, not an assumption imported from a name.
        let mut axes = BTreeSet::new();
        for k in 0..4 {
            let b = c.f.pow(c.b, k);
            let a: Vec<_> = b
                .iter()
                .enumerate()
                .filter(|(_, v)| **v != 0)
                .map(|(i, _)| i)
                .collect();
            assert_eq!(a.len(), 1);
            axes.insert(a[0]);
        }
        assert_eq!(axes.len(), 4);
        Self {
            c,
            pool,
            n,
            original_len: base.len(),
            l0,
            active: l0 / 12,
            anchor,
            g,
            actions,
        }
    }
    fn physical(&self, terms: &Terms, double: bool) -> bool {
        let first = self.original_len * if double { 2 } else { 1 };
        terms.len() == 5
            && terms.windows(2).all(|w| w[0] < w[1])
            && terms
                .iter()
                .all(|&(i, t)| i < self.pool.len() && [-1, 1].contains(&t))
            && terms.iter().map(|t| t.0).collect::<BTreeSet<_>>().len() == 5
            && terms.iter().filter(|t| t.0 < self.l0).count() == 4
            && terms
                .iter()
                .filter(|t| t.0 >= first && t.0 < first + self.l0)
                .count()
                == 1
            && terms
                .iter()
                .map(|t| self.pool[t.0].point.x)
                .collect::<BTreeSet<_>>()
                .len()
                == 5
            && terms.iter().fold(O, |sum, &(id, t)| {
                self.c.add(sum, self.c.signed(self.pool[id].point, t))
            }) == O
    }
    fn row(&self, terms: &Terms) -> Vec<u64> {
        let mut v = vec![0; self.active];
        for &(id, t) in terms {
            let f = &self.pool[id];
            assert!(f.col < self.active);
            v[f.col] = (v[f.col]
                + if t == 1 {
                    f.coefficient
                } else {
                    neg(f.coefficient, self.n)
                })
                % self.n;
        }
        let mut row = v[1..].to_vec();
        row.push(neg(mm(self.anchor, v[0], self.n), self.n));
        row
    }
    fn rows(&self, physical: &BTreeSet<Terms>) -> Value {
        let rows: Vec<_> = physical.iter().map(|t| self.row(t)).collect();
        let zeros = rows.iter().filter(|r| r.iter().all(|&a| a == 0)).count();
        let projective: BTreeSet<_> = rows.iter().filter_map(|r| canonical(r, self.n)).collect();
        let (rank, solution) = rref(rows, self.active - 1, self.n);
        let checked = solution
            .as_ref()
            .map(|x| {
                let mut logs = vec![self.anchor];
                logs.extend(x);
                self.pool
                    .iter()
                    .filter(|f| f.col < self.active)
                    .all(|f| self.c.mul(self.g, mm(logs[f.col], f.coefficient, self.n)) == f.point)
            })
            .unwrap_or(false);
        assert_eq!(checked, rank == self.active - 1);
        json!({"rank":rank,"projective_rows":projective.len(),"projective_augmented_rows":projective,"zero_rows":zeros,"factor_logs":solution,"full_rank_point_verified":checked})
    }
}
fn canonical(row: &[u64], n: u64) -> Option<Vec<u64>> {
    row.iter().find(|&&x| x != 0).map(|&x| {
        let inv = pow(x, n - 2, n);
        row.iter().map(|&a| mm(a, inv, n)).collect()
    })
}
fn rref(mut rows: Vec<Vec<u64>>, cols: usize, n: u64) -> (usize, Option<Vec<u64>>) {
    assert!(rows
        .iter()
        .all(|r| r.len() == cols + 1 && r.iter().all(|&x| x < n)));
    let mut rank = 0;
    for col in 0..cols {
        let Some(pivot) = (rank..rows.len()).find(|&i| rows[i][col] != 0) else {
            continue;
        };
        rows.swap(rank, pivot);
        let inv = pow(rows[rank][col], n - 2, n);
        for x in &mut rows[rank] {
            *x = mm(*x, inv, n);
        }
        let pivotrow = rows[rank].clone();
        for (i, row) in rows.iter_mut().enumerate() {
            if i == rank {
                continue;
            }
            let m = row[col];
            for j in col..=cols {
                row[j] = (row[j] + n - mm(m, pivotrow[j], n)) % n;
            }
        }
        rank += 1;
    }
    assert!(
        rows.iter()
            .all(|r| r[..cols].iter().any(|&x| x != 0) || r[cols] == 0),
        "inconsistent factor row"
    );
    let solution = if rank == cols {
        Some(rows[..cols].iter().map(|r| r[cols]).collect())
    } else {
        None
    };
    (rank, solution)
}
fn eval(equation: &Value, root: [u64; 3], p: u64) -> u64 {
    let mut exps = BTreeSet::new();
    equation.as_array().unwrap().iter().fold(0, |sum, t| {
        let e: [u32; 3] = serde_json::from_value(t[0].clone()).unwrap();
        let c = u(&t[1]);
        assert!(c > 0 && c < p && exps.insert(e));
        let mut v = c;
        for i in 0..3 {
            v = mm(v, pow(root[i], e[i] as u64, p), p);
        }
        (sum + v) % p
    })
}
