//! Evaluation-space Jacobian arithmetic for Tian's published F_523 quartic.
//! This starts with destination classes S,T; a source-to-Jacobian map and a
//! complete relation attack are separate gates and are not claimed here.
use five_leads_study::three_leads::{Elliptic, Field, Point};
use serde_json::json;
use std::cell::Cell;
use std::collections::BTreeMap;

const P: u64 = 523;
const N: usize = 85; // evaluation injective for L(7 D0), degree 84
const QUARTIC: [(usize, usize, usize, u64); 15] = [
    (4, 0, 0, 1),
    (3, 1, 0, 144),
    (2, 2, 0, 112),
    (1, 3, 0, 393),
    (0, 4, 0, 488),
    (3, 0, 1, 397),
    (2, 1, 1, 49),
    (1, 2, 1, 299),
    (0, 3, 1, 500),
    (2, 0, 2, 311),
    (1, 1, 2, 185),
    (0, 2, 2, 288),
    (1, 0, 3, 462),
    (0, 1, 3, 468),
    (0, 0, 4, 284),
];
#[derive(Default)]
struct F {
    m: Cell<u64>,
    s: Cell<u64>,
    i: Cell<u64>,
    add: Cell<u64>,
}
impl F {
    fn a(&self, x: u64, y: u64) -> u64 {
        self.add.set(self.add.get() + 1);
        (x + y) % P
    }
    fn n(&self, x: u64) -> u64 {
        if x == 0 {
            0
        } else {
            P - x
        }
    }
    fn sub(&self, x: u64, y: u64) -> u64 {
        self.a(x, self.n(y))
    }
    fn m(&self, x: u64, y: u64) -> u64 {
        self.m.set(self.m.get() + 1);
        x * y % P
    }
    fn sq(&self, x: u64) -> u64 {
        self.s.set(self.s.get() + 1);
        x * x % P
    }
    fn pow(&self, mut x: u64, mut n: u64) -> u64 {
        let mut y = 1;
        while n > 0 {
            if n & 1 != 0 {
                y = self.m(y, x)
            };
            n >>= 1;
            if n > 0 {
                x = self.sq(x)
            }
        }
        y
    }
    fn inv(&self, x: u64) -> u64 {
        assert_ne!(x, 0);
        self.i.set(self.i.get() + 1);
        let mut b = x;
        let mut n = P - 2;
        let mut y = 1;
        while n > 0 {
            if n & 1 != 0 {
                y = y * b % P
            };
            b = b * b % P;
            n >>= 1
        }
        y
    }
    fn dot(&self, a: &[u64], b: &[u64]) -> u64 {
        a.iter()
            .zip(b)
            .fold(0, |v, (&x, &y)| self.a(v, self.m(x, y)))
    }
    fn counts(&self) -> serde_json::Value {
        json!({"m":self.m.get(),"s":self.s.get(),"i":self.i.get(),"field_adds":self.add.get()})
    }
}
fn basis_monomials(degree: usize) -> Vec<(usize, usize)> {
    (0..=3)
        .flat_map(|i| {
            (0..=degree.saturating_sub(i))
                .filter(move |_| i <= degree)
                .map(move |j| (i, j))
        })
        .collect()
}
fn powers(f: &F, x: u64, n: usize) -> Vec<u64> {
    let mut out = vec![1];
    for _ in 0..n {
        out.push(f.m(*out.last().unwrap(), x))
    }
    out
}
fn quartic(f: &F, x: u64, y: u64, z: u64) -> u64 {
    let xp = powers(f, x, 4);
    let yp = powers(f, y, 4);
    let zp = powers(f, z, 4);
    QUARTIC.iter().fold(0, |acc, &(i, j, k, c)| {
        f.a(acc, f.m(c, f.m(xp[i], f.m(yp[j], zp[k]))))
    })
}
fn row_basis(f: &F, rows: Vec<Vec<u64>>, cols: usize) -> Vec<Vec<u64>> {
    let mut pivots: BTreeMap<usize, Vec<u64>> = BTreeMap::new();
    for mut row in rows {
        assert_eq!(row.len(), cols);
        for (&j, pivot) in &pivots {
            if row[j] != 0 {
                let scale = row[j];
                for k in j..cols {
                    row[k] = f.sub(row[k], f.m(scale, pivot[k]))
                }
            }
        }
        if let Some(j) = row.iter().position(|&v| v != 0) {
            let inv = f.inv(row[j]);
            for v in &mut row[j..] {
                *v = f.m(*v, inv)
            }
            pivots.insert(j, row);
        }
    }
    let mut rows: Vec<(usize, Vec<u64>)> = pivots.into_iter().collect();
    for b in (0..rows.len()).rev() {
        let j = rows[b].0;
        for a in 0..b {
            let scale = rows[a].1[j];
            if scale != 0 {
                for k in j..cols {
                    rows[a].1[k] = f.sub(rows[a].1[k], f.m(scale, rows[b].1[k]))
                }
            }
        }
    }
    rows.into_iter().map(|(_, r)| r).collect()
}
fn nullspace(f: &F, mut rows: Vec<Vec<u64>>, cols: usize) -> Vec<Vec<u64>> {
    for row in &rows {
        assert_eq!(row.len(), cols)
    }
    let mut pivots = Vec::new();
    let mut rank = 0;
    for j in 0..cols {
        let Some(k) = (rank..rows.len()).find(|&k| rows[k][j] != 0) else {
            continue;
        };
        rows.swap(rank, k);
        let inv = f.inv(rows[rank][j]);
        for v in &mut rows[rank][j..] {
            *v = f.m(*v, inv)
        }
        for k in 0..rows.len() {
            if k == rank {
                continue;
            }
            let scale = rows[k][j];
            if scale != 0 {
                for l in j..cols {
                    rows[k][l] = f.sub(rows[k][l], f.m(scale, rows[rank][l]))
                }
            }
        }
        pivots.push(j);
        rank += 1;
        if rank == rows.len() {
            break;
        }
    }
    let mut out = Vec::new();
    for free in 0..cols {
        if pivots.binary_search(&free).is_ok() {
            continue;
        }
        let mut row = vec![0; cols];
        row[free] = 1;
        for (k, &j) in pivots.iter().enumerate() {
            row[j] = f.n(rows[k][free])
        }
        out.push(row);
    }
    out
}
fn lincomb(f: &F, coeff: &[u64], rows: &[Vec<u64>], n: usize) -> Vec<u64> {
    let mut out = vec![0; n];
    for (&c, row) in coeff.iter().zip(rows) {
        if c != 0 {
            for j in 0..n {
                out[j] = f.a(out[j], f.m(c, row[j]))
            }
        }
    }
    out
}
fn mul_vectors(f: &F, a: &[u64], b: &[u64]) -> Vec<u64> {
    a.iter().zip(b).map(|(&x, &y)| f.m(x, y)).collect()
}
fn intersection(f: &F, a: &[Vec<u64>], b: &[Vec<u64>]) -> Vec<Vec<u64>> {
    let cols = a.len() + b.len();
    let matrix: Vec<Vec<u64>> = (0..N)
        .map(|i| {
            a.iter()
                .map(|r| r[i])
                .chain(b.iter().map(|r| f.n(r[i])))
                .collect()
        })
        .collect();
    let ker = nullspace(f, matrix.clone(), cols);
    for vector in &ker {
        for row in &matrix {
            assert_eq!(f.dot(row, vector), 0, "false intersection nullvector")
        }
    }
    let out = ker
        .iter()
        .map(|v| lincomb(f, &v[..a.len()], a, N))
        .collect();
    row_basis(f, out, N)
}
fn residual(f: &F, mut v: Vec<u64>, basis: &[Vec<u64>]) -> Vec<u64> {
    for row in basis {
        let j = row.iter().position(|&v| v != 0).unwrap();
        let c = v[j];
        if c != 0 {
            for k in j..N {
                v[k] = f.sub(v[k], f.m(c, row[k]))
            }
        }
    }
    v
}
fn preimage(
    f: &F,
    candidate: &[Vec<u64>],
    multiplier: &[Vec<u64>],
    target: &[Vec<u64>],
) -> Vec<Vec<u64>> {
    let target = row_basis(f, target.to_vec(), N);
    let mut constraints = Vec::new();
    for e in multiplier {
        let remainders: Vec<Vec<u64>> = candidate
            .iter()
            .map(|v| residual(f, mul_vectors(f, v, e), &target))
            .collect();
        for j in 0..N {
            constraints.push(remainders.iter().map(|r| r[j]).collect())
        }
    }
    let coeffs = nullspace(
        f,
        row_basis(f, constraints, candidate.len()),
        candidate.len(),
    );
    row_basis(
        f,
        coeffs.iter().map(|c| lincomb(f, c, candidate, N)).collect(),
        N,
    )
}
fn series_mul(f: &F, a: &[u64], b: &[u64]) -> Vec<u64> {
    let n = a.len();
    assert_eq!(b.len(), n);
    let mut out = vec![0; n];
    for i in 0..n {
        for j in 0..n - i {
            out[i + j] = f.a(out[i + j], f.m(a[i], b[j]))
        }
    }
    out
}
fn series_pow(f: &F, a: &[u64], n: usize) -> Vec<u64> {
    let mut out = vec![0; a.len()];
    out[0] = 1;
    for _ in 0..n {
        out = series_mul(f, &out, a)
    }
    out
}
fn series_eval(
    f: &F,
    terms: &[(usize, usize, usize, u64)],
    x: &[u64],
    y: &[u64],
    z: &[u64],
) -> Vec<u64> {
    let mut out = vec![0; x.len()];
    for &(i, j, k, c) in terms {
        let a = series_mul(
            f,
            &series_mul(f, &series_pow(f, x, i), &series_pow(f, y, j)),
            &series_pow(f, z, k),
        );
        for h in 0..out.len() {
            out[h] = f.a(out[h], f.m(c, a[h]))
        }
    }
    out
}
fn local_series(
    f: &F,
    point: (u64, u64, u64),
    order: usize,
    parameter: usize,
    dependent: usize,
) -> [Vec<u64>; 3] {
    let mut vars = [vec![0; order], vec![0; order], vec![0; order]];
    for j in 0..3 {
        vars[j][0] = [point.0, point.1, point.2][j]
    }
    vars[parameter][1] = 1;
    assert_ne!(parameter, dependent);
    for k in 1..order {
        let constant = series_eval(f, &QUARTIC, &vars[0], &vars[1], &vars[2])[k];
        vars[dependent][k] = 1;
        let derivative = f.sub(
            series_eval(f, &QUARTIC, &vars[0], &vars[1], &vars[2])[k],
            constant,
        );
        assert_ne!(derivative, 0, "bad local parameter at {point:?}");
        vars[dependent][k] = f.n(f.m(constant, f.inv(derivative)));
    }
    assert!(series_eval(f, &QUARTIC, &vars[0], &vars[1], &vars[2])
        .iter()
        .all(|&v| v == 0));
    vars
}
fn monomial_jets(
    f: &F,
    monomials: &[(usize, usize)],
    degree: usize,
    vars: &[Vec<u64>; 3],
) -> Vec<Vec<u64>> {
    let len = vars[0].len();
    let mut rows = vec![vec![0; monomials.len()]; len];
    for (j, &(i, k)) in monomials.iter().enumerate() {
        let values = series_mul(
            f,
            &series_mul(f, &series_pow(f, &vars[0], i), &series_pow(f, &vars[1], k)),
            &series_pow(f, &vars[2], degree - i - k),
        );
        for n in 0..len {
            rows[n][j] = values[n]
        }
    }
    rows
}
fn reduce_xy(f: &F, ix: usize, iy: usize, a: &[u64; 4], b: &[u64; 4]) -> Vec<u64> {
    let mut out = vec![0; 9];
    fn recurse(
        f: &F,
        ix: usize,
        iy: usize,
        coeff: u64,
        a: &[u64; 4],
        b: &[u64; 4],
        out: &mut [u64],
    ) {
        if coeff == 0 {
            return;
        }
        if ix >= 3 {
            for j in 0..3 {
                recurse(f, ix - 3 + j, iy, f.m(coeff, f.n(a[j])), a, b, out)
            }
        } else if iy >= 3 {
            for j in 0..3 {
                recurse(f, ix, iy - 3 + j, f.m(coeff, f.n(b[j])), a, b, out)
            }
        } else {
            out[3 * ix + iy] = f.a(out[3 * ix + iy], coeff)
        }
    }
    recurse(f, ix, iy, 1, a, b, &mut out);
    out
}
fn cubic_constraints(
    f: &F,
    monomials: &[(usize, usize)],
    a: &[u64; 4],
    b: &[u64; 4],
) -> Vec<Vec<u64>> {
    let mut multiplication = Vec::new();
    for ix in 0..3 {
        for iy in 0..3 {
            let mut column = vec![0; 9];
            for &(x, y, _, c) in &QUARTIC {
                let red = reduce_xy(f, ix + x, iy + y, a, b);
                for j in 0..9 {
                    column[j] = f.a(column[j], f.m(c, red[j]))
                }
            }
            multiplication.push(column)
        }
    }
    let left_kernel = nullspace(f, multiplication, 9);
    assert_eq!(
        left_kernel.len(),
        3,
        "quartic/cubic intersection has degree 3"
    );
    left_kernel
        .iter()
        .map(|functional| {
            monomials
                .iter()
                .map(|&(ix, iy)| f.dot(functional, &reduce_xy(f, ix, iy, a, b)))
                .collect()
        })
        .collect()
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Class(Vec<Vec<u64>>);
struct QuarticKm {
    f: F,
    h: u64,
    roots: [(u64, u64); 3],
    points: Vec<(u64, u64)>,
    mons: Vec<(usize, usize)>,
    v2: Vec<Vec<u64>>,
    v: Vec<Vec<u64>>,
    v4: Vec<Vec<u64>>,
    l_d0: Vec<Vec<u64>>,
    identity: Class,
}
impl QuarticKm {
    fn new() -> Self {
        let f = F::default();
        assert_eq!(quartic(&f, 359, 1, 0), 0);
        let mut line = None;
        for c in 0..P {
            let mut roots = Vec::new();
            for y in 0..P {
                let x = (359 * y + P - c) % P;
                if quartic(&f, x, y, 1) == 0 {
                    roots.push((x, y))
                }
            }
            if roots.len() == 3 {
                line = Some((c, [roots[0], roots[1], roots[2]]));
                break;
            }
        }
        let (h, roots) = line.expect("split hyperplane through P0");
        let mut points = Vec::new();
        'scan: for x in 0..P {
            for y in 0..P {
                if (x + P - 359 * y % P + h) % P != 0 && quartic(&f, x, y, 1) == 0 {
                    points.push((x, y));
                    if points.len() == N {
                        break 'scan;
                    }
                }
            }
        }
        assert_eq!(points.len(), N);
        let mons = basis_monomials(9);
        assert_eq!(mons.len(), 34);
        let h_powers: Vec<(u64, u64, u64, u64)> = points
            .iter()
            .map(|&(x, y)| {
                let value = f.a(f.sub(x, f.m(359, y)), h);
                assert_ne!(value, 0);
                (
                    f.inv(f.pow(value, 9)),
                    f.inv(f.pow(value, 6)),
                    f.inv(f.pow(value, 3)),
                    f.inv(f.pow(value, 12)),
                )
            })
            .collect();
        let v: Vec<Vec<u64>> = mons
            .iter()
            .map(|&(i, j)| {
                points
                    .iter()
                    .enumerate()
                    .map(|(k, &(x, y))| {
                        f.m(f.m(f.pow(x, i as u64), f.pow(y, j as u64)), h_powers[k].0)
                    })
                    .collect()
            })
            .collect();
        assert_eq!(row_basis(&f, v.clone(), N).len(), 34);
        let l_d0 = row_basis(
            &f,
            mons.iter()
                .filter(|&&(i, j)| i + j <= 3)
                .map(|&(i, j)| {
                    points
                        .iter()
                        .enumerate()
                        .map(|(k, &(x, y))| {
                            f.m(f.m(f.pow(x, i as u64), f.pow(y, j as u64)), h_powers[k].2)
                        })
                        .collect()
                })
                .collect(),
            N,
        );
        assert_eq!(l_d0.len(), 10);
        let v2 = row_basis(
            &f,
            mons.iter()
                .enumerate()
                .filter(|(_, &(i, j))| i + j <= 6)
                .map(|(_, &(i, j))| {
                    points
                        .iter()
                        .enumerate()
                        .map(|(k, &(x, y))| {
                            f.m(f.m(f.pow(x, i as u64), f.pow(y, j as u64)), h_powers[k].1)
                        })
                        .collect()
                })
                .collect(),
            N,
        );
        assert_eq!(v2.len(), 22);
        let v4 = row_basis(
            &f,
            basis_monomials(12)
                .iter()
                .map(|&(i, j)| {
                    points
                        .iter()
                        .enumerate()
                        .map(|(k, &(x, y))| {
                            f.m(f.m(f.pow(x, i as u64), f.pow(y, j as u64)), h_powers[k].3)
                        })
                        .collect()
                })
                .collect(),
            N,
        );
        assert_eq!(v4.len(), 46);
        let identity_on_h = Class(v2.clone());
        let mut out = Self {
            f,
            h,
            roots,
            points,
            mons,
            v2,
            v,
            v4,
            l_d0,
            identity: identity_on_h,
        };
        let cubic = basis_monomials(3);
        let generic: Vec<u64> = cubic
            .iter()
            .map(|&(i, j)| u64::from(i == 3 && j == 0))
            .collect();
        let moved = out.translate(&out.identity, &generic);
        assert_eq!(moved.0.len(), 22);
        out.identity = moved;
        out
    }
    fn line(&self, x: u64, y: u64, z: u64) -> u64 {
        self.f
            .a(self.f.sub(x, self.f.m(359, y)), self.f.m(self.h, z))
    }
    fn cubic_value(&self, coeff: &[u64], x: u64, y: u64, z: u64) -> u64 {
        basis_monomials(3)
            .iter()
            .enumerate()
            .fold(0, |sum, (j, &(i, k))| {
                self.f.a(
                    sum,
                    self.f.m(
                        coeff[j],
                        self.f.m(
                            self.f.pow(x, i as u64),
                            self.f
                                .m(self.f.pow(y, k as u64), self.f.pow(z, (3 - i - k) as u64)),
                        ),
                    ),
                )
            })
    }
    fn translate(&self, class: &Class, cubic: &[u64]) -> Class {
        let f = &self.f;
        let factors: Vec<u64> = self
            .points
            .iter()
            .map(|&(x, y)| {
                let h = self.line(x, y, 1);
                assert_ne!(h, 0);
                f.m(self.cubic_value(cubic, x, y, 1), f.inv(f.pow(h, 3)))
            })
            .collect();
        let moved = Class(row_basis(
            f,
            class
                .0
                .iter()
                .map(|row| mul_vectors(f, row, &factors))
                .collect(),
            N,
        ));
        assert_eq!(moved.0.len(), 22);
        let v_basis = row_basis(f, self.v.clone(), N);
        for row in &moved.0 {
            assert!(
                residual(f, row.clone(), &v_basis).iter().all(|&x| x == 0),
                "translated class outside V"
            )
        }
        moved
    }
    fn cubic_triple_at_p0(&self) -> Vec<u64> {
        let f = &self.f;
        let cubic = basis_monomials(3);
        let vars = local_series(f, (359, 1, 0), 4, 0, 2);
        let jets = monomial_jets(f, &cubic, 3, &vars);
        let ker = nullspace(f, jets[..3].to_vec(), cubic.len());
        assert_eq!(ker.len(), 7);
        let mut candidates = ker.clone();
        for i in 0..ker.len() {
            for j in i + 1..ker.len() {
                candidates.push(
                    ker[i]
                        .iter()
                        .zip(&ker[j])
                        .map(|(&a, &b)| f.a(a, b))
                        .collect(),
                )
            }
        }
        for candidate in candidates {
            let order3 = f.dot(&jets[3], &candidate);
            if order3 == 0 {
                continue;
            }
            if self
                .roots
                .iter()
                .all(|&(x, y)| self.cubic_value(&candidate, x, y, 1) != 0)
            {
                return candidate;
            }
        }
        panic!("no movable cubic with exact triple P0 zero")
    }
    fn class_from_cubics(&self, a: [u64; 4], b: [u64; 4], cubic: &[u64]) -> Class {
        let f = &self.f;
        let mut constraints = cubic_constraints(f, &self.mons, &a, &b);
        for &(x, y) in &self.roots {
            let parameter = if self.partial(x, y, 1, 0) != 0 { 1 } else { 0 };
            let vars = local_series(
                f,
                (x, y, 1),
                3,
                parameter,
                if parameter == 1 { 0 } else { 1 },
            );
            constraints.extend(monomial_jets(f, &self.mons, 9, &vars));
            let av = f.a(
                f.a(f.a(a[0], f.m(a[1], x)), f.m(a[2], f.sq(x))),
                f.pow(x, 3),
            );
            let bv = f.a(
                f.a(f.a(b[0], f.m(b[1], y)), f.m(b[2], f.sq(y))),
                f.pow(y, 3),
            );
            assert!(av != 0 || bv != 0, "published divisor hits H")
        }
        assert_eq!(constraints.len(), 12);
        let coeffs = nullspace(f, constraints, self.mons.len());
        assert_eq!(coeffs.len(), 22, "input divisor degree");
        let on_h = Class(row_basis(
            f,
            coeffs
                .iter()
                .map(|row| lincomb(f, row, &self.v, N))
                .collect(),
            N,
        ));
        assert_eq!(on_h.0.len(), 22);
        self.translate(&on_h, cubic)
    }
    fn partial(&self, x: u64, y: u64, z: u64, variable: usize) -> u64 {
        let f = &self.f;
        QUARTIC.iter().fold(0, |sum, &(i, j, k, c)| {
            let d = [i, j, k][variable];
            if d == 0 {
                return sum;
            }
            let exp = [i, j, k];
            let term = f.m(
                f.m(c, d as u64),
                f.m(
                    f.pow(x, (exp[0] - usize::from(variable == 0)) as u64),
                    f.m(
                        f.pow(y, (exp[1] - usize::from(variable == 1)) as u64),
                        f.pow(z, (exp[2] - usize::from(variable == 2)) as u64),
                    ),
                ),
            );
            f.a(sum, term)
        })
    }
    // Khuri-Makdisi small-model addflip, following the complete sequence of
    // image and preimage spaces in Sage's implementation of Theorem 5.3.
    fn flip(&self, a: &Class, b: &Class) -> Class {
        let f = &self.f;
        let products =
            a.0.iter()
                .flat_map(|u| b.0.iter().map(move |v| mul_vectors(f, u, v)))
                .collect();
        let w1 = row_basis(f, products, N);
        assert_eq!(w1.len(), 46, "product dimension");
        let w2 = preimage(f, &self.v, &self.v, &w1);
        assert!(w2.len() >= 10, "small-model w2");
        let w3 = preimage(f, &self.v4, &self.v2, &w1);
        assert_eq!(w3.len(), 22, "small-model w3");
        let w4 = row_basis(
            f,
            self.v4.iter().map(|v| mul_vectors(f, &w2[0], v)).collect(),
            N,
        );
        assert_eq!(w4.len(), 46, "small-model w4");
        let w5 = preimage(f, &self.v, &w3, &w4);
        assert_eq!(w5.len(), 22, "small-model w5");
        Class(w5)
    }
    fn neg(&self, a: &Class) -> Class {
        if a == &self.identity {
            return a.clone();
        }
        self.flip(a, &self.identity)
    }
    fn is_zero(&self, a: &Class) -> bool {
        !intersection(&self.f, &a.0, &self.l_d0).is_empty()
    }
    fn equivalent(&self, a: &Class, b: &Class) -> bool {
        self.is_zero(&self.add(a, &self.neg(b)))
    }
    fn add(&self, a: &Class, b: &Class) -> Class {
        if a == &self.identity {
            return b.clone();
        }
        if b == &self.identity {
            return a.clone();
        }
        self.neg(&self.flip(a, b))
    }
    fn mul(&self, a: &Class, mut n: u64) -> Class {
        let mut acc = self.identity.clone();
        let mut base = a.clone();
        while n > 0 {
            if n & 1 != 0 {
                acc = self.add(&acc, &base)
            };
            n >>= 1;
            if n > 0 {
                base = self.add(&base, &base)
            }
        }
        acc
    }
}
fn linear_dlp_fixture() {
    // The published rational S,T classes are supplied as the transfer
    // fixture. This search receives T, never the published scalar.
    let km = QuarticKm::new();
    let moved_cubic = km.cubic_triple_at_p0();
    let s = km.class_from_cubics([380, 3, 364, 1], [190, 302, 340, 1], &moved_cubic);
    let t = km.class_from_cubics([25, 158, 205, 1], [491, 424, 108, 1], &moved_cubic);
    let after_setup = km.f.counts();
    let negative_target = km.neg(&t);
    let mut accumulator = km.identity.clone();
    let mut recovered = None;
    let cap = 10_000u64;
    for scalar in 1..=cap {
        accumulator = km.add(&accumulator, &s);
        if km.is_zero(&km.add(&accumulator, &negative_target)) {
            recovered = Some(scalar);
            break;
        }
        if scalar % 256 == 0 {
            eprintln!("linear destination search checked {scalar} scalars");
        }
    }
    let after_search = km.f.counts();
    let scalar = recovered.expect("destination search exhausted");
    fn pack(a: u64, b: u64, c: u64) -> u64 {
        a + 523 * b + 523 * 523 * c
    }
    let field = Field::new(523, 3, 2);
    let source = Elliptic {
        f: field,
        c: [pack(243, 339, 359), pack(140, 92, 82), 0, 1],
        binary: false,
    };
    let p = Point::affine(7, pack(310, 306, 155));
    let q = Point::affine(pack(38, 393, 311), pack(33, 167, 418));
    assert!(source.valid(p) && source.valid(q));
    assert_eq!(
        source.mul(p, scalar),
        q,
        "recovered scalar fails source check"
    );
    println!(
        "{}",
        json!({"schema":"tian-km-linear-dlp/v1","p":P,"r":143069629,"cap":cap,
        "recovered":scalar,"source_verified":true,"method":"bounded_linear_search_on_published_destination_classes",
        "source_map_executed":false,"index_calculus_executed":false,
        "after_setup_counts":after_setup,"after_search_counts":after_search,
        "source_verification_counts":source.f.counts()})
    );
}
fn main() {
    if std::env::args().any(|arg| arg == "--dlp-linear") {
        linear_dlp_fixture();
        return;
    }
    let km = QuarticKm::new();
    let after_geometry = km.f.counts();
    let moved_cubic = km.cubic_triple_at_p0();
    let s = km.class_from_cubics([380, 3, 364, 1], [190, 302, 340, 1], &moved_cubic);
    let t = km.class_from_cubics([25, 158, 205, 1], [491, 424, 108, 1], &moved_cubic);
    let after_classes = km.f.counts();
    eprintln!("quartic gate: identity");
    assert!(km.is_zero(&km.identity), "identity is nonzero");
    eprintln!("quartic gate: source S");
    assert!(!km.is_zero(&s), "published S is zero");
    eprintln!("quartic gate: S plus zero");
    let s_plus_zero = km.add(&s, &km.identity);
    eprintln!("quartic gate: S plus zero equivalence");
    assert!(km.equivalent(&s_plus_zero, &s), "S+0 != S");
    eprintln!("quartic gate: doubling");
    let before_double = km.f.counts();
    let double = km.add(&s, &s);
    let after_double = km.f.counts();
    assert!(
        km.equivalent(&double, &km.mul(&s, 2)),
        "doubling inconsistent"
    );
    let before_scalar = km.f.counts();
    let scalar = km.mul(&s, 1987);
    let after_scalar = km.f.counts();
    assert!(km.equivalent(&scalar, &t), "published T != 1987 S");
    let order = km.mul(&s, 143069629);
    assert!(km.is_zero(&order), "source prime order not preserved");
    assert!(km.is_zero(&km.add(&s, &km.neg(&s))), "inverse law");
    assert!(
        km.equivalent(&km.add(&s, &t), &km.add(&t, &s)),
        "commutative law"
    );
    assert!(
        km.equivalent(&km.add(&km.add(&s, &s), &t), &km.add(&s, &km.add(&s, &t))),
        "associative law"
    );
    println!(
        "{}",
        json!({"schema":"tian-km-gate/v2","p":P,"line_c":km.h,"line_roots":km.roots,"evaluation_points":km.points.len(),"v_dimension":km.v.len(),"identity_dimension":km.identity.0.len(),"movable_cubic":moved_cubic,"s_t_checked":true,"t_equals_1987_s":true,"r_s_is_zero":true,"group_law_controls":true,"counts":km.f.counts(),"snapshots":{"after_geometry":after_geometry,"after_classes":after_classes,"before_double":before_double,"after_double":after_double,"before_scalar":before_scalar,"after_scalar":after_scalar}})
    );
}
