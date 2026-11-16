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
    fn class_from_rational_triple(&self, triple: &[(u64, u64); 3]) -> Class {
        let f = &self.f;
        let mut constraints = Vec::new();
        for &(x, y) in triple {
            assert_eq!(quartic(f, x, y, 1), 0);
            assert_ne!(self.line(x, y, 1), 0);
            constraints.push(
                self.mons
                    .iter()
                    .map(|&(i, j)| f.m(f.pow(x, i as u64), f.pow(y, j as u64)))
                    .collect(),
            );
        }
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
        }
        assert_eq!(constraints.len(), 12);
        let coefficients = nullspace(f, constraints, self.mons.len());
        assert_eq!(coefficients.len(), 22);
        Class(row_basis(
            f,
            coefficients
                .iter()
                .map(|c| lincomb(f, c, &self.v, N))
                .collect(),
            N,
        ))
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
const R: u64 = 143_069_629;

// A line section and the fixed hyperplane H have linearly equivalent
// degree-four divisors.  With P0 on H, every fully split affine line gives
// four-term relations among the rational-point classes [P-P0].
fn line_sieve_rank(solve: bool, mapped: bool) {
    let f = F::default();
    let mut points = Vec::new();
    for x in 0..P {
        for y in 0..P {
            if quartic(&f, x, y, 1) == 0 {
                points.push((x, y));
            }
        }
    }
    let mut infinity = Vec::new();
    for x in 0..P {
        if quartic(&f, x, 1, 0) == 0 {
            infinity.push((x, 1));
        }
    }
    if quartic(&f, 1, 0, 0) == 0 {
        infinity.push((1, 0));
    }
    assert!(infinity.contains(&(359, 1)));
    let mut h = None;
    for c in 0..P {
        let roots: Vec<_> = points
            .iter()
            .enumerate()
            .filter(|(_, &(x, y))| f.a(f.sub(x, f.m(359, y)), c) == 0)
            .map(|(idx, _)| idx)
            .collect();
        if roots.len() == 3 {
            h = Some((c, roots));
            break;
        }
    }
    let (h_constant, h_roots) = h.expect("split hyperplane");
    let width = points.len();
    let mut pivots: BTreeMap<usize, Vec<u64>> = BTreeMap::new();
    let mut full_lines = 0usize;
    let mut count_scalar_mul = 0u64;
    let mut count_scalar_add = 0u64;
    let mut count_scalar_inv = 0u64;
    let mut collect = |line: &[usize]| {
        if line.len() != 4 {
            return;
        }
        full_lines += 1;
        let mut row = vec![0u64; width];
        for &idx in line {
            row[idx] += 1;
        }
        for &idx in &h_roots {
            row[idx] = (row[idx] + R - 1) % R;
        }
        for (&pivot, prior) in &pivots {
            if row[pivot] == 0 {
                continue;
            }
            let c = row[pivot];
            for j in pivot..width {
                row[j] = (row[j] + R - c * prior[j] % R) % R;
                count_scalar_mul += 1;
                count_scalar_add += 1;
            }
        }
        if let Some(pivot) = row.iter().position(|&x| x != 0) {
            let inverse = scalar_pow(row[pivot], R - 2);
            count_scalar_inv += 1;
            for x in &mut row[pivot..] {
                *x = *x * inverse % R;
                count_scalar_mul += 1;
            }
            pivots.insert(pivot, row);
        }
    };
    for slope in 0..P {
        let mut lines = vec![Vec::new(); P as usize];
        for (idx, &(x, y)) in points.iter().enumerate() {
            let intercept = f.sub(y, f.m(slope, x));
            lines[intercept as usize].push(idx);
        }
        // A line with a rational point at infinity can have at most three
        // distinct affine intersections, hence it never reaches collect().
        for line in &lines {
            if line.len() == 4 {
                assert_ne!(
                    quartic(&f, 1, slope, 0),
                    0,
                    "split line has a fifth point at infinity"
                );
            }
            collect(line);
        }
    }
    let mut vertical = vec![Vec::new(); P as usize];
    for (idx, &(x, _)) in points.iter().enumerate() {
        vertical[x as usize].push(idx);
    }
    for line in &vertical {
        if line.len() == 4 {
            assert_ne!(quartic(&f, 0, 1, 0), 0);
        }
        collect(line);
    }
    let rank = pivots.len();
    let descent = if solve {
        Some(line_sieve_descent(&points, &h_roots, &pivots, mapped))
    } else {
        None
    };
    println!(
        "{}",
        json!({
            "schema":if mapped {"tian-quartic-line-sieve-mapped-dlp/v1"} else if solve {"tian-quartic-line-sieve-dlp/v1"} else {"tian-quartic-line-sieve-rank/v1"}, "p":P,"r":R,
            "affine_rational_points":width,"infinity_rational_points":infinity,
            "h_constant":h_constant,"h_roots":h_roots.iter().map(|&i| points[i]).collect::<Vec<_>>(),
            "fully_split_lines":full_lines,"relation_rank":rank,"nullity":width-rank,
            "scalar_field_counts":{"multiplications":count_scalar_mul,"additions":count_scalar_add,"inversions":count_scalar_inv},
            "curve_field_counts":f.counts(),"descent":descent,
        })
    );
}
fn invert_matrix(f: &F, matrix: &[Vec<u64>]) -> Vec<Vec<u64>> {
    let n = matrix.len();
    assert!(matrix.iter().all(|row| row.len() == n));
    let mut a: Vec<Vec<u64>> = matrix
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let mut v = row.clone();
            v.extend((0..n).map(|j| u64::from(i == j)));
            v
        })
        .collect();
    for i in 0..n {
        let pivot = (i..n)
            .find(|&j| a[j][i] != 0)
            .expect("singular evaluation matrix");
        a.swap(i, pivot);
        let inv = f.inv(a[i][i]);
        for j in i..2 * n {
            a[i][j] = f.m(a[i][j], inv);
        }
        for k in 0..n {
            if k == i {
                continue;
            }
            let c = a[k][i];
            if c != 0 {
                for j in i..2 * n {
                    a[k][j] = f.sub(a[k][j], f.m(c, a[i][j]));
                }
            }
        }
    }
    a.into_iter().map(|row| row[n..].to_vec()).collect()
}
struct RationalEvaluator {
    columns: Vec<usize>,
    weights: Vec<Vec<u64>>,
    points: Vec<(u64, u64)>,
    original_indices: Vec<usize>,
}
impl RationalEvaluator {
    fn new(km: &QuarticKm, all: &[(u64, u64)]) -> Self {
        let f = &km.f;
        let mut cols = Vec::new();
        let mut independent = Vec::new();
        for col in 0..N {
            let v: Vec<u64> = km.v.iter().map(|row| row[col]).collect();
            let mut candidate = independent.clone();
            candidate.push(v.clone());
            if row_basis(f, candidate, km.v.len()).len() > independent.len() {
                independent.push(v);
                cols.push(col);
                if cols.len() == km.v.len() {
                    break;
                }
            }
        }
        assert_eq!(cols.len(), 34);
        let a: Vec<Vec<u64>> =
            km.v.iter()
                .map(|row| cols.iter().map(|&j| row[j]).collect())
                .collect();
        let a_inv = invert_matrix(f, &a);
        let mut points = Vec::new();
        let mut original_indices = Vec::new();
        let mut weights = Vec::new();
        for (idx, &(x, y)) in all.iter().enumerate() {
            let h = km.line(x, y, 1);
            if h == 0 {
                continue;
            }
            let h_inv9 = f.inv(f.pow(h, 9));
            let monomial_values: Vec<u64> = km
                .mons
                .iter()
                .map(|&(i, j)| f.m(f.m(f.pow(x, i as u64), f.pow(y, j as u64)), h_inv9))
                .collect();
            let lambda: Vec<u64> = a_inv
                .iter()
                .map(|row| f.dot(row, &monomial_values))
                .collect();
            for (j, basis) in km.v.iter().enumerate() {
                let selected: Vec<u64> = cols.iter().map(|&c| basis[c]).collect();
                assert_eq!(f.dot(&lambda, &selected), monomial_values[j]);
            }
            points.push((x, y));
            original_indices.push(idx);
            weights.push(lambda);
        }
        Self {
            columns: cols,
            weights,
            points,
            original_indices,
        }
    }
    fn eval(&self, f: &F, row: &[u64], idx: usize) -> u64 {
        let selected: Vec<u64> = self.columns.iter().map(|&j| row[j]).collect();
        f.dot(&self.weights[idx], &selected)
    }
}
fn fixed_descent_space(km: &QuarticKm) -> Vec<Vec<u64>> {
    let f = &km.f;
    let p0 = local_series(f, (359, 1, 0), 3, 0, 2);
    let mut constraints = monomial_jets(f, &km.mons, 9, &p0);
    for &(x, y) in &km.roots {
        let parameter = if km.partial(x, y, 1, 0) != 0 { 1 } else { 0 };
        let vars = local_series(
            f,
            (x, y, 1),
            6,
            parameter,
            if parameter == 1 { 0 } else { 1 },
        );
        constraints.extend(monomial_jets(f, &km.mons, 9, &vars));
    }
    assert_eq!(constraints.len(), 21);
    let coefficients = nullspace(f, constraints, km.mons.len());
    assert_eq!(coefficients.len(), 13);
    row_basis(
        f,
        coefficients
            .iter()
            .map(|c| lincomb(f, c, &km.v, N))
            .collect(),
        N,
    )
}
fn residual_triple(
    km: &QuarticKm,
    fixed: &[Vec<u64>],
    evaluator: &RationalEvaluator,
    class: &Class,
) -> Option<Vec<usize>> {
    let intersection = intersection(&km.f, &class.0, fixed);
    assert_eq!(intersection.len(), 1, "unexpected descent intersection");
    let h = &intersection[0];
    let mut roots = Vec::new();
    for i in 0..evaluator.points.len() {
        if evaluator.eval(&km.f, h, i) != 0 {
            continue;
        }
        let basepoint = class.0.iter().all(|row| evaluator.eval(&km.f, row, i) == 0);
        if !basepoint {
            roots.push(evaluator.original_indices[i]);
        }
    }
    if roots.len() == 3 {
        let triple = [
            evaluator.points[evaluator
                .original_indices
                .iter()
                .position(|&j| j == roots[0])
                .unwrap()],
            evaluator.points[evaluator
                .original_indices
                .iter()
                .position(|&j| j == roots[1])
                .unwrap()],
            evaluator.points[evaluator
                .original_indices
                .iter()
                .position(|&j| j == roots[2])
                .unwrap()],
        ];
        let rational_class = km.class_from_rational_triple(&triple);
        assert!(
            km.equivalent(&rational_class, &km.neg(class)),
            "descent divisor relation failed"
        );
        Some(roots)
    } else {
        None
    }
}
fn point_log_kernel(pivots: &BTreeMap<usize, Vec<u64>>, width: usize) -> (Vec<u64>, u64, u64) {
    assert_eq!(pivots.len() + 1, width, "line rank must have nullity one");
    let free = (0..width).find(|i| !pivots.contains_key(i)).unwrap();
    let mut values = vec![0; width];
    let mut multiplications = 0;
    let mut additions = 0;
    values[free] = 1;
    for (&j, row) in pivots.iter().rev() {
        let mut sum = 0;
        for k in j + 1..width {
            sum = (sum + row[k] * values[k] % R) % R;
            multiplications += 1;
            additions += 1;
        }
        values[j] = (R - sum) % R;
    }
    for row in pivots.values() {
        assert_eq!(
            row.iter()
                .zip(&values)
                .fold(0, |a, (&x, &y)| (a + x * y % R) % R),
            0
        );
    }
    (values, multiplications, additions)
}
fn line_sieve_descent(
    points: &[(u64, u64)],
    h_roots: &[usize],
    pivots: &BTreeMap<usize, Vec<u64>>,
    mapped: bool,
) -> serde_json::Value {
    let km = QuarticKm::new();
    assert_eq!(km.roots.iter().all(|p| points.contains(p)), true);
    assert_eq!(h_roots.len(), 3);
    let moved_cubic = km.cubic_triple_at_p0();
    let published_s = km.class_from_cubics([380, 3, 364, 1], [190, 302, 340, 1], &moved_cubic);
    let published_t = km.class_from_cubics([25, 158, 205, 1], [491, 424, 108, 1], &moved_cubic);
    let (s, t) = if mapped {
        let receipt: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(
                "research/five_leads_e2e_20261019/correctness/tian-cubic-trace-map-pq-v1.json",
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(receipt["schema"], "tian-cubic-trace-map-pq/v1");
        assert!(receipt["checked"]
            .as_array()
            .unwrap()
            .iter()
            .all(|v| v["matched_published_destination"] == true));
        let parse = |key: &str| -> Class {
            Class(
                receipt[key]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|row| {
                        row.as_array()
                            .unwrap()
                            .iter()
                            .map(|v| v.as_u64().unwrap())
                            .collect()
                    })
                    .collect(),
            )
        };
        let s = parse("mapped_p_class");
        let t = parse("mapped_q_class");
        assert!(km.equivalent(&s, &published_s) && km.equivalent(&t, &published_t));
        (s, t)
    } else {
        (published_s, published_t)
    };
    let evaluator = RationalEvaluator::new(&km, points);
    let fixed = fixed_descent_space(&km);
    let (logs, kernel_multiplications, kernel_additions) = point_log_kernel(pivots, points.len());
    let mut attempts = Vec::new();
    let mut s_triple = None;
    let mut t_triple = None;
    // Public S,T are the only inputs; scalar 1987 is never used to form
    // candidates. Small sequential translates are a deterministic descent.
    let mut s_multiple = km.identity.clone();
    let mut t_multiple = km.identity.clone();
    for a in 0..=12u64 {
        if a > 0 {
            s_multiple = km.add(&s_multiple, &s);
        }
        if let Some(roots) = residual_triple(&km, &fixed, &evaluator, &s_multiple) {
            let value = roots.iter().fold(0, |acc, &i| (acc + logs[i]) % R);
            attempts.push(json!({"a":a,"b":0,"roots":roots,"kernel_sum":value}));
            if a > 0 && value != 0 && s_triple.is_none() {
                s_triple = Some((a, value));
            }
        } else {
            attempts.push(json!({"a":a,"b":0,"roots":null}));
        }
        if s_triple.is_some() {
            break;
        }
    }
    for b in 1..=12u64 {
        t_multiple = km.add(&t_multiple, &t);
        if let Some(roots) = residual_triple(&km, &fixed, &evaluator, &t_multiple) {
            let value = roots.iter().fold(0, |acc, &i| (acc + logs[i]) % R);
            attempts.push(json!({"a":0,"b":b,"roots":roots,"kernel_sum":value}));
            if t_triple.is_none() {
                t_triple = Some((b, value));
            }
        } else {
            attempts.push(json!({"a":0,"b":b,"roots":null}));
        }
        if s_triple.is_some() && t_triple.is_some() {
            break;
        }
    }
    let recovered = if let (Some((a, sv)), Some((b, tv))) = (s_triple, t_triple) {
        assert_eq!(sv * scalar_pow(sv, R - 2) % R, 1);
        assert_eq!(b * scalar_pow(b, R - 2) % R, 1);
        let scale = (R - a) * scalar_pow(sv, R - 2) % R;
        let k = (R - scale * tv % R) * scalar_pow(b, R - 2) % R;
        Some(k)
    } else {
        None
    };
    let after_descent = km.f.counts();
    let mut destination_verified = false;
    let mut source_verified = false;
    let mut source_counts = None;
    if let Some(k) = recovered {
        destination_verified = km.equivalent(&km.mul(&s, k), &t);
        fn pack(a: u64, b: u64, c: u64) -> u64 {
            a + 523 * b + 523 * 523 * c
        }
        let source = Elliptic {
            f: Field::new(523, 3, 2),
            c: [pack(243, 339, 359), pack(140, 92, 82), 0, 1],
            binary: false,
        };
        let generator = Point::affine(7, pack(310, 306, 155));
        let target = Point::affine(pack(38, 393, 311), pack(33, 167, 418));
        source_verified =
            source.valid(generator) && source.valid(target) && source.mul(generator, k) == target;
        source_counts = Some(source.f.counts());
        assert!(
            destination_verified && source_verified,
            "recovered log did not verify"
        );
    }
    json!({"attempts":attempts,"recovered":recovered,"destination_verified":destination_verified,
        "source_verified":source_verified,"source_verification_counts":source_counts,
        "after_descent_curve_field_counts":after_descent,"after_verification_curve_field_counts":km.f.counts(),
        "scalar_recovery_inversions":if recovered.is_some(){2}else{0},
        "scalar_kernel_multiplications":kernel_multiplications,"scalar_kernel_additions":kernel_additions,
        "factor_base_size":logs.len(),"fixed_dimension":fixed.len(),"off_h_evaluation_points":evaluator.points.len(),
        "source_map_executed":mapped,"index_calculus_executed":true})
}
fn scalar_pow(mut x: u64, mut n: u64) -> u64 {
    let mut y = 1u64;
    while n > 0 {
        if n & 1 != 0 {
            y = y * x % R;
        }
        n >>= 1;
        if n > 0 {
            x = x * x % R;
        }
    }
    y
}
fn main() {
    if std::env::args().any(|arg| arg == "--line-sieve-mapped-dlp") {
        line_sieve_rank(true, true);
        return;
    }
    let trace_pq_only = std::env::args().any(|arg| arg == "--trace-map-pq");
    if trace_pq_only || std::env::args().any(|arg| arg == "--trace-map-gate") {
        let km = QuarticKm::new();
        let f = &km.f;
        let e = [60u64, 392, 337, 1];
        let y_poly = [188u64, 405, 180];
        assert!((0..P).all(|x| (e[0] + e[1] * x + e[2] * x * x + x * x * x) % P != 0));
        let multiply = |a: [u64; 3], b: [u64; 3]| -> [u64; 3] {
            let mut t = [0u64; 5];
            for i in 0..3 {
                for j in 0..3 {
                    t[i + j] = f.a(t[i + j], f.m(a[i], b[j]));
                }
            }
            for i in (3..5).rev() {
                let high = t[i];
                for j in 0..3 {
                    t[i - 3 + j] = f.sub(t[i - 3 + j], f.m(high, e[j]));
                }
            }
            [t[0], t[1], t[2]]
        };
        let mut xp = vec![[1, 0, 0]];
        let mut yp = vec![[1, 0, 0]];
        for _ in 0..9 {
            xp.push(multiply(*xp.last().unwrap(), [0, 1, 0]));
            yp.push(multiply(*yp.last().unwrap(), y_poly));
        }
        let mut quartic_at_e = [0u64; 3];
        for &(i, j, _, c) in &QUARTIC {
            let term = multiply(xp[i], yp[j]);
            for k in 0..3 {
                quartic_at_e[k] = f.a(quartic_at_e[k], f.m(c, term[k]));
            }
        }
        assert_eq!(quartic_at_e, [0; 3]);
        let osculating = [448u64, 375, 99, 290, 462, 9, 150, 439, 205, 1];
        let mut cubic_at_e = [0u64; 3];
        for (idx, &(i, j)) in basis_monomials(3).iter().enumerate() {
            let term = multiply(xp[i], yp[j]);
            for k in 0..3 {
                cubic_at_e[k] = f.a(cubic_at_e[k], f.m(osculating[idx], term[k]));
            }
        }
        assert_eq!(cubic_at_e, [0; 3]);
        let mut e_constraints = vec![vec![0u64; km.mons.len()]; 3];
        for (col, &(i, j)) in km.mons.iter().enumerate() {
            let term = multiply(xp[i], yp[j]);
            for k in 0..3 {
                e_constraints[k][col] = term[k];
            }
        }
        let input = std::fs::read_to_string(
            "research/five_leads_e2e_20261019/correctness/tian-cubic-map-gate-v2.jsonl",
        )
        .unwrap();
        let rows: Vec<serde_json::Value> = input
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(rows.len(), 101);
        let movable = km.cubic_triple_at_p0();
        let s = km.class_from_cubics([380, 3, 364, 1], [190, 302, 340, 1], &movable);
        let t = km.class_from_cubics([25, 158, 205, 1], [491, 424, 108, 1], &movable);
        let mut results = Vec::new();
        let mut expected = km.identity.clone();
        let selected: Vec<_> = if trace_pq_only {
            vec![&rows[0], rows.last().unwrap()]
        } else {
            rows.iter().collect()
        };
        let mut mapped_p = None;
        let mut mapped_q = None;
        for row in selected {
            let mut constraints: Vec<Vec<u64>> = row["trace_divisor_constraints"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| {
                    r.as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_u64().unwrap())
                        .collect()
                })
                .collect();
            assert_eq!(constraints.len(), 9);
            constraints.extend(e_constraints.clone());
            let coefficients = nullspace(f, constraints, km.mons.len());
            assert_eq!(coefficients.len(), 22, "trace class constraint rank");
            let class = Class(row_basis(
                f,
                coefficients
                    .iter()
                    .map(|c| lincomb(f, c, &km.v, N))
                    .collect(),
                N,
            ));
            assert_eq!(class.0.len(), 22);
            let scalar = row["scalar_check_only"].as_u64().unwrap();
            if scalar <= 100 {
                expected = km.add(&expected, &s);
            }
            let matched = if scalar <= 100 {
                km.equivalent(&class, &expected)
            } else {
                km.equivalent(&class, &t)
            };
            if scalar == 1 {
                mapped_p = Some(class.0.clone());
            }
            if scalar == 1987 {
                mapped_q = Some(class.0.clone());
            }
            results.push(json!({"scalar":scalar,"class_dimension":class.0.len(),"matched_published_destination":matched}));
            assert!(
                matched,
                "source trace disagrees with published class for {scalar}"
            );
        }
        println!(
            "{}",
            json!({"schema":if trace_pq_only {"tian-cubic-trace-map-pq/v1"} else {"tian-cubic-trace-map-gate/v1"},
            "residual_cubic":e,"residual_y_poly":y_poly,"osculating_cubic":osculating,
            "checked":results,"mapped_p_class":mapped_p,"mapped_q_class":mapped_q,
            "curve_field_counts":km.f.counts()})
        );
        return;
    }
    if std::env::args().any(|arg| arg == "--osculating9-gate") {
        let f = F::default();
        let mons = basis_monomials(3);
        let vars = local_series(&f, (359, 1, 0), 10, 0, 2);
        let jets = monomial_jets(&f, &mons, 3, &vars);
        let kernel = nullspace(&f, jets[..9].to_vec(), mons.len());
        let mut data = Vec::new();
        for cubic in &kernel {
            let mut support = Vec::new();
            for x in 0..P {
                for y in 0..P {
                    if quartic(&f, x, y, 1) == 0 {
                        let value = mons.iter().enumerate().fold(0, |acc, (j, &(i, k))| {
                            f.a(
                                acc,
                                f.m(cubic[j], f.m(f.pow(x, i as u64), f.pow(y, k as u64))),
                            )
                        });
                        if value == 0 {
                            support.push((x, y));
                        }
                    }
                }
            }
            data.push(
                json!({"cubic":cubic,"jet9":f.dot(&jets[9],cubic),"rational_support":support}),
            );
        }
        println!(
            "{}",
            json!({"schema":"tian-osculating9-gate/v1","kernel_dimension":kernel.len(),"data":data})
        );
        return;
    }
    if std::env::args().any(|arg| arg == "--line-sieve-dlp") {
        line_sieve_rank(true, false);
        return;
    }
    if std::env::args().any(|arg| arg == "--line-sieve-rank") {
        line_sieve_rank(false, false);
        return;
    }
    if std::env::args().any(|arg| arg == "--published-support") {
        let f = F::default();
        let support = |a: [u64; 4], b: [u64; 4]| -> Vec<(u64, u64)> {
            let mut result = Vec::new();
            for x in 0..P {
                for y in 0..P {
                    if quartic(&f, x, y, 1) != 0 {
                        continue;
                    }
                    let cx = (a[0] + a[1] * x + a[2] * x * x + x * x * x) % P;
                    let cy = (b[0] + b[1] * y + b[2] * y * y + y * y * y) % P;
                    if cx == 0 && cy == 0 {
                        result.push((x, y));
                    }
                }
            }
            result
        };
        println!(
            "{}",
            json!({"schema":"tian-published-divisor-support/v1",
            "s_rational_support":support([380,3,364,1],[190,302,340,1]),
            "t_rational_support":support([25,158,205,1],[491,424,108,1])})
        );
        return;
    }
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
