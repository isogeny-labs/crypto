use num_bigint::BigUint as Z;
use num_traits::{One, Zero};
#[derive(Clone)]
pub struct Fp {
    pub p: Z,
}
impl Fp {
    pub fn c(&self, n: i64) -> Z {
        if n >= 0 {
            Z::from(n as u64) % &self.p
        } else {
            (&self.p - Z::from(n.unsigned_abs()) % &self.p) % &self.p
        }
    }
    pub fn add(&self, a: &Z, b: &Z) -> Z {
        (a + b) % &self.p
    }
    pub fn sub(&self, a: &Z, b: &Z) -> Z {
        (a + &self.p - b) % &self.p
    }
    pub fn mul(&self, a: &Z, b: &Z) -> Z {
        a * b % &self.p
    }
    pub fn inv(&self, a: &Z) -> Z {
        assert!(!a.is_zero());
        a.modpow(&(&self.p - Z::from(2u32)), &self.p)
    }
    pub fn trim(&self, mut a: Vec<Z>) -> Vec<Z> {
        while a.len() > 1 && a.last().unwrap().is_zero() {
            a.pop();
        }
        a
    }
    pub fn scale(&self, a: &[Z], s: &Z) -> Vec<Z> {
        self.trim(a.iter().map(|x| self.mul(x, s)).collect())
    }
    pub fn sum(&self, a: &[Z], b: &[Z], scale: &Z) -> Vec<Z> {
        let mut z = vec![Z::zero(); a.len().max(b.len())];
        z[..a.len()].clone_from_slice(a);
        for (i, x) in b.iter().enumerate() {
            z[i] = self.add(&z[i], &self.mul(x, scale));
        }
        self.trim(z)
    }
    pub fn pm(&self, a: &[Z], b: &[Z]) -> Vec<Z> {
        let mut z = vec![Z::zero(); a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                z[i + j] = self.add(&z[i + j], &self.mul(x, y));
            }
        }
        self.trim(z)
    }
    pub fn der(&self, a: &[Z]) -> Vec<Z> {
        if a.len() == 1 {
            return vec![Z::zero()];
        }
        (1..a.len()).map(|i| self.mul(&Z::from(i), &a[i])).collect()
    }
    pub fn ev(&self, a: &[Z], x: &Z) -> Z {
        a.iter()
            .rev()
            .fold(Z::zero(), |z, c| self.add(&self.mul(&z, x), c))
    }
    pub fn matrix(&self, xs: &[Z], ys: &[Z], d: usize) -> Vec<Vec<Z>> {
        xs.iter()
            .zip(ys)
            .map(|(x, y)| {
                let mut powers = vec![Z::one()];
                for j in 1..=d {
                    powers.push(self.mul(&powers[j - 1], x));
                }
                let mut row = powers.clone();
                row.extend(powers.iter().map(|v| self.sub(&Z::zero(), &self.mul(y, v))));
                row
            })
            .collect()
    }
    pub fn rref(&self, mut m: Vec<Vec<Z>>) -> (Vec<Vec<Z>>, Vec<usize>, Vec<usize>, Z) {
        let n = m.len();
        let c = m[0].len();
        let mut rows: Vec<_> = (0..n).collect();
        let mut pivots = vec![];
        let mut product = Z::one();
        for j in 0..c {
            let k = pivots.len();
            let Some(i) = (k..n).find(|&i| !m[i][j].is_zero()) else {
                continue;
            };
            m.swap(k, i);
            rows.swap(k, i);
            let v = m[k][j].clone();
            product = self.mul(&product, &v);
            let inv = self.inv(&v);
            for col in j..c {
                m[k][col] = self.mul(&m[k][col], &inv)
            }
            let pivot = m[k].clone();
            for (row, line) in m.iter_mut().enumerate() {
                if row == k {
                    continue;
                }
                let v = line[j].clone();
                if v.is_zero() {
                    continue;
                }
                for col in j..c {
                    line[col] = self.sub(&line[col], &self.mul(&v, &pivot[col]));
                }
            }
            pivots.push(j);
            if pivots.len() == n {
                break;
            }
        }
        (m, pivots, rows, product)
    }
}
pub fn strings(a: &[Z]) -> Vec<String> {
    a.iter().map(|z| z.to_string()).collect()
}
pub fn parse(v: &serde_json::Value) -> Z {
    if let Some(s) = v.as_str() {
        s.parse().unwrap()
    } else {
        v.to_string().parse().unwrap()
    }
}
pub fn values(v: &serde_json::Value) -> Vec<Z> {
    v.as_array().unwrap().iter().map(parse).collect()
}
pub fn known(f: &Fp, a: &Z, b: &Z, n: i64, coord: usize) -> (Vec<Z>, Vec<Z>) {
    let curve = vec![b.clone(), a.clone(), Z::zero(), Z::one()];
    if n.abs() == 1 {
        return if coord == 0 {
            (vec![Z::zero(), Z::one()], vec![Z::one()])
        } else {
            (f.scale(&curve, &f.c(n)), vec![Z::one()])
        };
    }
    if n == 2 {
        let num = vec![
            f.mul(a, a),
            f.mul(&f.c(-8), b),
            f.mul(&f.c(-2), a),
            Z::zero(),
            Z::one(),
        ];
        return if coord == 0 {
            (num, f.scale(&curve, &f.c(4)))
        } else {
            (
                f.sum(
                    &f.pm(&f.der(&num), &curve),
                    &f.pm(&num, &f.der(&curve)),
                    &f.c(-1),
                ),
                f.scale(&curve, &f.c(8)),
            )
        };
    }
    assert_eq!(n, 3);
    let a2 = f.mul(a, a);
    let psi = vec![
        f.mul(&f.c(-1), &a2),
        f.mul(&f.c(12), b),
        f.mul(&f.c(6), a),
        Z::zero(),
        f.c(3),
    ];
    let q6 = vec![
        f.sub(&f.mul(&f.c(-8), &f.mul(b, b)), &f.mul(&a2, a)),
        f.mul(&f.c(-4), &f.mul(a, b)),
        f.mul(&f.c(-5), &a2),
        f.mul(&f.c(20), b),
        f.mul(&f.c(5), a),
        Z::zero(),
        Z::one(),
    ];
    let den = f.pm(&psi, &psi);
    let mut xp = vec![Z::zero()];
    xp.extend(den.clone());
    let num = f.sum(&xp, &f.pm(&curve, &q6), &f.c(-8));
    if coord == 0 {
        (num, den)
    } else {
        let u = f.sum(
            &f.pm(&f.der(&num), &psi),
            &f.pm(&num, &f.der(&psi)),
            &f.c(-2),
        );
        (f.pm(&curve, &u), f.scale(&f.pm(&den, &psi), &f.c(3)))
    }
}
