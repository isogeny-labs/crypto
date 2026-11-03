//! Actual determinant19 mixed translation atlases and invariant circuits.
use crate::{large_field::*, round02, round04};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use serde_json::json;
use std::{collections::BTreeSet, path::Path};
pub(crate) struct EC<'a> {
    pub(crate) f: &'a F,
    pub(crate) ss: bool,
    pub(crate) c: u64,
    pub(crate) trace: i64,
}
impl EC<'_> {
    fn neg(&self, p: &P) -> P {
        match p {
            P::Infinity => P::Infinity,
            P::Affine { x, y } => P::Affine {
                x: x.clone(),
                y: y.add(&if self.ss { self.f.o() } else { x.clone() }),
            },
        }
    }
    pub(crate) fn valid(&self, p: &P) -> bool {
        match p {
            P::Infinity => true,
            P::Affine { x, y } => {
                if self.ss {
                    self.f.sq(y).add(y)
                        == self.f.mul(&self.f.sq(x), x).add(x).add(&self.f.e(self.c))
                } else {
                    self.f.curve(self.f.e(self.c), self.f.o()).is_on_curve(p)
                }
            }
        }
    }
    fn add(&self, p: &P, q: &P) -> P {
        if !self.ss {
            return plus(&self.f.curve(self.f.e(self.c), self.f.o()), p, q);
        }
        let (P::Affine { x, y }, P::Affine { x: u, y: v }) = (p, q) else {
            return if *p == P::Infinity {
                q.clone()
            } else {
                p.clone()
            };
        };
        if x == u && y != v {
            return P::Infinity;
        }
        let l = if p == q {
            self.f.sq(x).add(&self.f.o())
        } else {
            self.f.mul(&y.add(v), &self.f.inv(&x.add(u)))
        };
        let nx = self.f.sq(&l).add(x).add(u);
        let ny = self.f.mul(&l, &x.add(&nx)).add(y).add(&self.f.o());
        let z = P::Affine { x: nx, y: ny };
        assert!(self.valid(&z));
        z
    }
    pub(crate) fn times(&self, p: &P, n: u64) -> P {
        let mut z = P::Infinity;
        for i in (0..64 - u64::leading_zeros(n)).rev() {
            z = self.add(&z, &z);
            if n >> i & 1 == 1 {
                z = self.add(&z, p)
            }
        }
        z
    }
    fn bigtimes(&self, p: &P, n: &BigUint) -> P {
        let mut z = P::Infinity;
        for i in (0..n.bits()).rev() {
            z = self.add(&z, &z);
            if n.bit(i) {
                z = self.add(&z, p)
            }
        }
        z
    }
    pub(crate) fn point(&self, x: &E) -> Option<P> {
        if !self.ss {
            return self.f.point(&self.f.curve(self.f.e(self.c), self.f.o()), x);
        }
        let y = self
            .f
            .quadratic(&self.f.mul(&self.f.sq(x), x).add(x).add(&self.f.e(self.c)))?;
        Some(P::Affine { x: x.clone(), y })
    }
    fn order(&self) -> BigUint {
        let (mut a, mut b) = (BigInt::from(2), BigInt::from(self.trace));
        for _ in 2..=self.f.m() {
            let c = &b * self.trace - &a * 2;
            (a, b) = (b, c);
        }
        ((BigInt::one() << self.f.m()) + BigInt::one() - b)
            .to_biguint()
            .unwrap()
    }
    fn torsion(&self) -> P {
        let mut h = self.order();
        while &h % 19u64 == BigUint::zero() {
            h /= 19u64;
        }
        for x in 1..4096 {
            if let Some(p) = self.point(&self.f.e(x)) {
                let mut p = self.bigtimes(&p, &h);
                if p == P::Infinity {
                    continue;
                }
                while self.times(&p, 19) != P::Infinity {
                    p = self.times(&p, 19);
                }
                return p;
            }
        }
        panic!("19-torsion point search bound")
    }
    fn line(&self, p: &P, q: &P, r: &P) -> Option<E> {
        let (P::Affine { x, y }, P::Affine { x: u, y: v }, P::Affine { x: rx, y: ry }) = (p, q, r)
        else {
            return None;
        };
        if x == u && p != q {
            return Some(rx.add(x)).filter(|a| !a.is_zero());
        }
        let l = if p == q {
            if self.ss {
                self.f.sq(x).add(&self.f.o())
            } else {
                x.add(&self.f.mul(y, &self.f.inv(x)))
            }
        } else {
            self.f.mul(&y.add(v), &self.f.inv(&x.add(u)))
        };
        let value = ry.add(y).add(&self.f.mul(&l, &rx.add(x)));
        if value.is_zero() {
            return None;
        }
        match self.add(p, q) {
            P::Infinity => Some(value),
            P::Affine { x, .. } => {
                let den = rx.add(&x);
                if den.is_zero() {
                    None
                } else {
                    Some(self.f.mul(&value, &self.f.inv(&den)))
                }
            }
        }
    }
    fn miller(&self, p: &P, r: &P) -> Option<E> {
        let mut v = p.clone();
        let mut z = self.f.o();
        for b in [0, 0, 1, 1] {
            z = self.f.mul(&self.f.sq(&z), &self.line(&v, &v, r)?);
            v = self.add(&v, &v);
            if b == 1 {
                z = self.f.mul(&z, &self.line(&v, p, r)?);
                v = self.add(&v, p);
            }
        }
        assert_eq!(v, P::Infinity);
        Some(z)
    }
    fn weil(&self, p: &P, q: &P) -> E {
        for x in 1..1024 {
            let Some(s) = self.point(&self.f.e(x)) else {
                continue;
            };
            let ns = self.neg(&s);
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                self.miller(p, &self.add(q, &s)),
                self.miller(p, &s),
                self.miller(q, &ns),
                self.miller(q, &self.add(p, &ns)),
            ) {
                let v = self
                    .f
                    .mul(&self.f.mul(&a, &c), &self.f.inv(&self.f.mul(&b, &d)));
                assert_eq!(self.f.powu(&v, 19), self.f.o());
                return v;
            }
        }
        panic!("19 pairing auxiliary budget")
    }
}
fn ord(t: i64) -> u64 {
    let f = [[0, 17], [1, t.rem_euclid(19) as u64]];
    let mut a = [[1, 0], [0, 1]];
    for n in 1..=6840 {
        let mut z = [[0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    z[i][j] = (z[i][j] + a[i][k] * f[k][j]) % 19;
                }
            }
        }
        a = z;
        if a == [[1, 0], [0, 1]] {
            return n;
        }
    }
    panic!("Frobenius order bound")
}
fn xyp(p: &P) -> [E; 2] {
    match p {
        P::Affine { x, y } => [x.clone(), y.clone()],
        _ => panic!("affine circuit sample"),
    }
}
fn invariant(f: &F, ord: &EC, ss: &EC, point: &[P], group: &[Vec<P>]) -> Vec<E> {
    let mut out = vec![f.o(); 8];
    for g in group {
        let z: Vec<_> = (0..5)
            .map(|i| {
                if i == 4 {
                    ss.add(&point[i], &g[i])
                } else {
                    ord.add(&point[i], &g[i])
                }
            })
            .collect();
        let coord: Vec<_> = z.iter().flat_map(xyp).collect();
        for k in 0..8 {
            let value = coord
                .iter()
                .enumerate()
                .fold(f.e((k + 1) as u64), |a, (j, x)| {
                    a.add(&f.mul(&f.e((1 + k * 13 + j * 7) as u64), x))
                });
            out[k] = f.mul(&out[k], &value);
        }
    }
    out
}
pub fn run(dir: &Path) {
    let input = round04::read(&dir.join("gl2b-integral-polarizations.json"));
    let orders: Vec<_> = [-2, -1, 1, 2].into_iter().map(|t| (t, ord(t))).collect();
    let m = orders.iter().fold(5, |m, (_, n)| num_integer::lcm(m, *n)) as u32;
    assert!(m <= 1440);
    println!("Q5 Frobenius orders{orders:?}; full common field degree{m}");
    let f = F::find(m);
    let mut records = vec![];
    for trace in [-1, 1] {
        let ec = EC {
            f: &f,
            ss: false,
            c: if trace == -1 { 0 } else { 1 },
            trace,
        };
        let sst = 2 * trace;
        let ss = EC {
            f: &f,
            ss: true,
            c: if sst == -2 { 0 } else { 1 },
            trace: sst,
        };
        let p = ec.torsion();
        let wp = if trace == -1 {
            f.frobp(&p, 1)
        } else {
            ec.neg(&f.frobp(&p, 1))
        };
        assert!(!(0..19).any(|i| ec.times(&p, i) == wp));
        let zeta = ec.weil(&p, &wp);
        assert_ne!(zeta, f.o());
        let s = ss.torsion();
        let fs = f.frobp(&s, 1);
        assert!(!(0..19).any(|i| ss.times(&s, i) == fs));
        let zss = ss.weil(&s, &fs);
        let exponent = (1..19).find(|i| f.powu(&zeta, *i) == zss).unwrap();
        let factor = (1..19).find(|i| i * exponent % 19 == 1).unwrap();
        let mut binding = None;
        for u in 0..19 {
            for v in 0..19 {
                let det = ((u * u) as i64 + sst * (u * v) as i64 + 2 * (v * v) as i64)
                    .rem_euclid(19) as u64;
                if det == factor && binding.is_none() {
                    binding = Some((u, v));
                }
            }
        }
        let (u, v) = binding.unwrap();
        let s = ss.add(&ss.times(&s, u), &ss.times(&fs, v));
        let fs = f.frobp(&s, 1);
        assert_eq!(ss.weil(&s, &fs), zeta);
        let ebase = [s, fs];
        for row in input["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["determinant"] == 19 && r["target_trace"] == trace)
        {
            let Some(glue) = row["elliptic_gluings"]
                .as_array()
                .unwrap()
                .iter()
                .find(|g| g["elliptic_trace"] == sst && g["anti_isometries"].as_u64().unwrap() > 0)
            else {
                continue;
            };
            let k: Vec<Vec<u64>> =
                serde_json::from_value(row["kernel"]["basis_columns"].clone()).unwrap();
            let anti: Vec<Vec<u64>> =
                serde_json::from_value(glue["first_anti_isometry"].clone()).unwrap();
            let mut group = vec![];
            let mut encoded = vec![];
            let mut unique = BTreeSet::new();
            for a in 0..19 {
                for b in 0..19 {
                    let v = [
                        (anti[0][0] * a + anti[0][1] * b) % 19,
                        (anti[1][0] * a + anti[1][1] * b) % 19,
                    ];
                    let coeff: Vec<_> =
                        k.iter().map(|r| (r[0] * v[0] + r[1] * v[1]) % 19).collect();
                    let mut ap: Vec<P> = (0..4)
                        .map(|i| {
                            ec.add(
                                &ec.times(&p, coeff[2 * i]),
                                &ec.times(&wp, coeff[2 * i + 1]),
                            )
                        })
                        .collect();
                    let mut hp = vec![];
                    for i in 0..4 {
                        let mut sum = P::Infinity;
                        for j in 0..4 {
                            let h = &row["hermitian_matrix"][i][j];
                            let ca = h["a"].as_i64().unwrap().rem_euclid(19) as u64;
                            let cb = h["b"].as_i64().unwrap().rem_euclid(19) as u64;
                            let wx = if trace == -1 {
                                f.frobp(&ap[j], 1)
                            } else {
                                ec.neg(&f.frobp(&ap[j], 1))
                            };
                            sum = ec.add(&sum, &ec.add(&ec.times(&ap[j], ca), &ec.times(&wx, cb)));
                        }
                        hp.push(sum);
                    }
                    assert!(hp.iter().all(|p| *p == P::Infinity));
                    let q = ss.add(&ss.times(&ebase[0], a), &ss.times(&ebase[1], b));
                    assert_eq!(ss.times(&q, 19), P::Infinity);
                    unique.insert(pj(&q).to_string());
                    ap.push(q);
                    encoded.push(
                        json!({"coefficients":[a,b],"point":ap.iter().map(pj).collect::<Vec<_>>()}),
                    );
                    group.push(ap);
                }
            }
            assert_eq!(unique.len(), 361);
            let mut frobenius = vec![];
            for (a, point) in group.iter().enumerate() {
                let u = a / 19;
                let v = a % 19;
                let nu = (17 * v) % 19;
                let nv = (u as i64 + sst * v as i64).rem_euclid(19) as usize;
                let mut dest = vec![];
                dest.push(
                    ec.neg(&(0..4).fold(P::Infinity, |z, i| ec.add(&z, &f.frobp(&point[i], 1)))),
                );
                for i in 0..3 {
                    dest.push(f.frobp(&point[i], 1));
                }
                dest.push(f.frobp(&point[4], 1));
                assert_eq!(dest, group[nu * 19 + nv]);
                frobenius.push(nu * 19 + nv);
            }
            let po = (1..4096)
                .filter_map(|i| ec.point(&f.e(i)))
                .find(|p| (1..=4).all(|i| ec.times(&ec.times(p, i), 19) != P::Infinity))
                .unwrap();
            let ps = (1..4096)
                .filter_map(|i| ss.point(&f.e(i)))
                .find(|p| ss.times(p, 19) != P::Infinity)
                .unwrap();
            let sample = vec![
                po.clone(),
                ec.times(&po, 2),
                ec.times(&po, 3),
                ec.times(&po, 4),
                ps,
            ];
            let values = invariant(&f, &ec, &ss, &sample, &group);
            let mut checks = vec![];
            for gi in [1, 19] {
                let shifted: Vec<_> = (0..5)
                    .map(|i| {
                        if i == 4 {
                            ss.add(&sample[i], &group[gi][i])
                        } else {
                            ec.add(&sample[i], &group[gi][i])
                        }
                    })
                    .collect();
                assert_eq!(invariant(&f, &ec, &ss, &shifted, &group), values);
                checks.push(gi);
            }
            records.push(json!({"coefficients":row["coefficients"],"coset":row["congruence_coset"],"target_trace":trace,"supersingular_trace":sst,"ordinary_basis":[pj(&p),pj(&wp)],"supersingular_companion_basis":ebase.iter().map(pj).collect::<Vec<_>>(),"supersingular_binding_centralizer":[u,v],"pairing":ej(&zeta),"kernel":encoded,"kernel_Frobenius_permutation":frobenius,"kernel_order":361,"kernel_H_checks":361,"atlas_equations":"four ordinary projective elliptic factors E_a plus Y^2 Z+Y Z^2=X^3+X Z^2+c Z^3; total dimension5","transition_maps":"translate ordinary factors and supersingular factor by each recorded graph point; all361 actions explicit; free action","F2_descent":"coefficient Frobenius followed by cyclic shift on augmentation; recorded permutation verifies graph descent","quotient_coordinates":"factored orbit polynomial product_g (U+sum_j Z_j coordinate_j(P+g)); coefficients are invariant functions on translation-stable affine chart","specialized_norm_invariants":values.iter().map(ej).collect::<Vec<_>>(),"invariance_generators_checked":checks,"expanded_quotient_ideal":false,"principal_polarization":"descends from H plus[19] by the retained isotropic graph; written geometric bridge and integral certificate","scope":"actual mixed ordinary/supersingular finite-etale quotient atlas and explicit invariant circuits; not an expanded image ideal or Jacobian realization"}));
            println!("Q5 trace{trace} coefficients{}:361 kernel points, Frobenius and invariant circuits checked",row["coefficients"]);
        }
    }
    round02::save(
        dir,
        "q5-mixed-translation-atlases.json",
        &json!({"schema":"koblitz.round07/q5-v1","field":{"degree":m,"low_terms":f.irr.low_terms},"frobenius_orders":orders,"records":records,"expanded_quotient_equations":"OPEN","Jacobian_realization":"OPEN","scope":"four frozen determinant19 graph realizations; explicit finite etale atlases and factored invariant coordinates"}),
    );
}
