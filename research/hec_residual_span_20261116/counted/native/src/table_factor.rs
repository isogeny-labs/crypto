//! Exact degree<=4 root tabulation; experimental example only.
//! Proof and cold-cost boundary: research/hec_table_oracle_20261114/PROTOCOL.md.
use hec_native_ic::{fp::Fp, oracle::Roots, poly::Poly};
const MISS: u64 = u64::MAX;

pub struct Tables {
    degree_limit: usize,
    pub fp: Fp,
    square: Vec<u64>,
    cube: Vec<u64>,
    inverse: Vec<u64>,
    projective: Vec<u64>,
    scale: Vec<u64>,
    n: u64,
    zeta: u64,
    kappa: u64,
}
fn pack(mut rs: Vec<(u64, u32)>) -> Roots {
    rs.sort_unstable_by_key(|e| e.0);
    let mut out = Roots::default();
    for (x, m) in rs {
        if out.len > 0 && out.r[out.len - 1].0 == x {
            out.r[out.len - 1].1 += m;
        } else {
            assert!(out.len < 4);
            out.r[out.len] = (x, m);
            out.len += 1;
        }
    }
    out
}
impl Tables {
    pub fn new(p: u64) -> Self {
        Self::for_degree(p, 4)
    }
    pub fn for_degree(p: u64, degree_limit: usize) -> Self {
        assert!(p > 3 && p <= 1_000_000, "bounded odd prime field required");
        assert!((2..=4).contains(&degree_limit));
        let f = Fp::new(p);
        let len = p as usize;
        let mut square = vec![MISS; len];
        let (mut value, mut delta) = (0, 1);
        for y in 0..=p / 2 {
            square[value as usize] = y;
            value = f.add(value, delta);
            delta = f.add(delta, 2);
        }
        if degree_limit == 2 {
            return Self {
                degree_limit,
                fp: f,
                square,
                cube: Vec::new(),
                inverse: Vec::new(),
                projective: Vec::new(),
                scale: Vec::new(),
                n: 0,
                zeta: 0,
                kappa: 0,
            };
        }
        let mut cube = vec![MISS; len];
        for x in 0..p {
            cube[f.mul(f.mul(x, x), x) as usize] = x;
        }
        let mut inverse = vec![0; len];
        inverse[1] = 1;
        for x in 2..p {
            inverse[x as usize] = f.mul(p - p / x, inverse[(p % x) as usize]);
        }
        let n = (2..p).find(|&v| square[v as usize] == MISS).unwrap();
        let (mut projective, mut scale, mut zeta, mut kappa) = (Vec::new(), Vec::new(), 0, 0);
        if p % 3 == 2 {
            projective = vec![MISS; len + 1];
            scale = vec![0; len + 1];
            for t in 0..p {
                let nt2 = f.mul(n, f.mul(t, t));
                let den = f.add(1, f.mul(3 % p, nt2));
                if den == 0 {
                    continue;
                }
                let num = f.mul(t, f.add(3 % p, nt2));
                let inv = inverse[den as usize];
                let image = f.mul(num, inv) as usize;
                projective[image] = t;
                scale[image] = inv;
            }
            projective[len] = p; // infinity is a cube of infinity
            let t = square[f.mul(f.neg(3 % p), inverse[n as usize]) as usize];
            assert_ne!(t, MISS);
            kappa = f.mul(n, t);
            assert_eq!(
                projective.iter().filter(|&&v| v != MISS).count(),
                (len + 1) / 3
            );
        } else {
            let a = (2..p).find(|&v| cube[v as usize] == MISS).unwrap();
            zeta = f.pow(a, (p - 1) / 3);
            assert_ne!(zeta, 1);
        }
        Self {
            degree_limit,
            fp: f,
            square,
            cube,
            inverse,
            projective,
            scale,
            n,
            zeta,
            kappa,
        }
    }
    pub fn bytes(&self) -> usize {
        8 * (self.square.len()
            + self.cube.len()
            + self.inverse.len()
            + self.projective.len()
            + self.scale.len())
    }
    fn inv(&self, a: u64) -> u64 {
        assert_ne!(a, 0);
        if self.inverse.is_empty() {
            assert_eq!(a, 2);
            return (self.fp.p + 1) / 2;
        }
        self.inverse[a as usize]
    }
    pub fn factor_base(&self, curve: &hec_native_ic::jac::Curve) -> Vec<(u64, u64)> {
        assert_eq!(self.fp.p, curve.fp.p);
        (0..self.fp.p)
            .filter_map(|x| self.sq(curve.f.eval(x, &self.fp)).map(|y| (x, y)))
            .collect()
    }
    fn sq(&self, a: u64) -> Option<u64> {
        let v = self.square[a as usize];
        (v != MISS).then_some(v)
    }
    fn cr(&self, a: u64) -> Option<u64> {
        let v = self.cube[a as usize];
        (v != MISS).then_some(v)
    }
    fn quadratic(&self, b: u64, c: u64) -> Option<Roots> {
        let f = &self.fp;
        let d = f.sub(f.mul(b, b), f.mul(4 % f.p, c));
        let s = self.sq(d)?;
        let half = self.inv(2);
        Some(if s == 0 {
            pack(vec![(f.mul(f.neg(b), half), 2)])
        } else {
            pack(vec![
                (f.mul(f.sub(s, b), half), 1),
                (f.mul(f.sub(f.neg(s), b), half), 1),
            ])
        })
    }
    fn cubic(&self, b: u64, c: u64, d: u64) -> Option<Roots> {
        let f = &self.fp;
        let p = f.p;
        let i3 = self.inv(3);
        let shift = f.mul(b, i3);
        let bb = f.mul(b, b);
        let a = f.sub(c, f.mul(bb, i3));
        let q = f.add(
            f.sub(
                f.mul(2 % p, f.mul(f.mul(bb, b), self.inv(27 % p))),
                f.mul(f.mul(b, c), i3),
            ),
            d,
        );
        let delta = f.neg(f.add(
            f.mul(4 % p, f.mul(f.mul(a, a), a)),
            f.mul(27 % p, f.mul(q, q)),
        ));
        if delta == 0 {
            if a == 0 {
                assert_eq!(q, 0);
                return Some(pack(vec![(f.neg(shift), 3)]));
            }
            let single = f.mul(f.mul(3, q), self.inv(a));
            let double = f.neg(f.mul(single, self.inv(2)));
            return Some(pack(vec![
                (f.sub(single, shift), 1),
                (f.sub(double, shift), 2),
            ]));
        }
        self.sq(delta)?;
        let cardano = f.mul(f.neg(delta), self.inv(108 % p));
        let ar = f.neg(f.mul(q, self.inv(2)));
        if p % 3 == 1 {
            let s = self.sq(cardano).expect("square Cardano radicand");
            let mut aa = f.add(ar, s);
            if aa == 0 {
                aa = f.sub(ar, s);
            }
            let u = self.cr(aa)?;
            if u == 0 {
                return None;
            }
            let v = f.mul(f.neg(f.mul(a, i3)), self.inv(u));
            let z = self.zeta;
            let zz = f.mul(z, z);
            Some(pack(vec![
                (f.sub(f.add(u, v), shift), 1),
                (f.sub(f.add(f.mul(z, u), f.mul(zz, v)), shift), 1),
                (f.sub(f.add(f.mul(zz, u), f.mul(z, v)), shift), 1),
            ]))
        } else {
            let ai = self
                .sq(f.mul(cardano, self.inv(self.n)))
                .expect("imaginary Cardano coordinate");
            let key = if ar == 0 {
                p as usize
            } else {
                f.mul(ai, self.inv(ar)) as usize
            };
            let t = self.projective[key];
            if t == MISS {
                return None;
            }
            let (u, v) = if t == p {
                (
                    0,
                    self.cr(f.mul(ai, self.inv(self.n)))
                        .expect("base cube bijection"),
                )
            } else {
                let u = self
                    .cr(f.mul(ar, self.scale[key]))
                    .expect("base cube bijection");
                (u, f.mul(u, t))
            };
            let cross = f.mul(self.kappa, v);
            Some(pack(vec![
                (f.sub(f.add(u, u), shift), 1),
                (f.sub(f.sub(f.neg(u), cross), shift), 1),
                (f.sub(f.add(f.neg(u), cross), shift), 1),
            ]))
        }
    }
    fn repeated(&self, u: &Poly) -> Option<Roots> {
        let f = &self.fp;
        let g = Poly::gcd(u, &u.deriv(f), f);
        assert!(g.deg() > 0);
        let reduced = u.div_exact(&g, f);
        let distinct = self.split(&reduced)?;
        let mut rest = *u;
        let mut rs = Vec::new();
        for &(x, _) in distinct.as_slice() {
            let lin = Poly::x_minus(x, f);
            let mut m = 0;
            loop {
                let (q, r) = rest.divrem(&lin, f);
                if !r.is_zero() {
                    break;
                }
                rest = q;
                m += 1;
            }
            rs.push((x, m));
        }
        (rest.deg() == 0).then(|| pack(rs))
    }
    fn quartic(&self, u: &Poly) -> Option<Roots> {
        let f = &self.fp;
        let p = f.p;
        let (b, c, d, e) = (u.c[3], u.c[2], u.c[1], u.c[0]);
        let bb = f.mul(b, b);
        let shift = f.mul(b, self.inv(4));
        let a = f.sub(c, f.mul(3 % p, f.mul(bb, self.inv(8 % p))));
        let q = f.add(
            f.sub(d, f.mul(f.mul(b, c), self.inv(2))),
            f.mul(f.mul(bb, b), self.inv(8 % p)),
        );
        let r = f.sub(
            f.add(
                f.sub(e, f.mul(f.mul(b, d), self.inv(4))),
                f.mul(f.mul(bb, c), self.inv(16 % p)),
            ),
            f.mul(3 % p, f.mul(f.mul(bb, bb), self.inv(256 % p))),
        );
        if q == 0 {
            let zs = self.quadratic(a, r)?;
            let mut rs = Vec::new();
            for &(z, m) in zs.as_slice() {
                let s = self.sq(z)?;
                if s == 0 {
                    rs.push((f.neg(shift), 2 * m));
                } else {
                    rs.push((f.sub(s, shift), m));
                    rs.push((f.sub(f.neg(s), shift), m));
                }
            }
            return Some(pack(rs));
        }
        let resolvent = self.cubic(
            f.add(a, a),
            f.sub(f.mul(a, a), f.mul(4 % p, r)),
            f.neg(f.mul(q, q)),
        )?;
        if resolvent.len < 3 {
            return self.repeated(u);
        }
        let z = resolvent.r[0].0;
        assert_ne!(z, 0);
        let w = self.sq(z)?;
        let qw = f.mul(q, self.inv(w));
        let half = self.inv(2);
        let aa = f.mul(f.sub(f.add(a, z), qw), half);
        let bb = f.mul(f.add(f.add(a, z), qw), half);
        let left = self.quadratic(w, aa)?;
        let right = self.quadratic(f.neg(w), bb)?;
        Some(pack(
            left.as_slice()
                .iter()
                .chain(right.as_slice())
                .map(|&(x, m)| (f.sub(x, shift), m))
                .collect(),
        ))
    }
    pub fn split(&self, u: &Poly) -> Option<Roots> {
        assert_eq!(u.lc(), 1);
        assert!(u.deg() <= self.degree_limit as isize);
        match u.deg() {
            0 => Some(Roots::default()),
            1 => Some(pack(vec![(self.fp.neg(u.c[0]), 1)])),
            2 => self.quadratic(u.c[1], u.c[0]),
            3 => self.cubic(u.c[2], u.c[1], u.c[0]),
            4 => self.quartic(u),
            _ => panic!("degree<=4 required"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hec_native_ic::{
        oracle::{split_reference, OracleStats},
        rng::Rng,
    };
    fn check(t: &Tables, u: &Poly) {
        let got = t.split(u);
        if t.fp.p <= 13 {
            let mut rest = *u;
            let mut brute = Vec::new();
            for x in 0..t.fp.p {
                let lin = Poly::x_minus(x, &t.fp);
                let mut multiplicity = 0;
                loop {
                    let (q, r) = rest.divrem(&lin, &t.fp);
                    if !r.is_zero() {
                        break;
                    }
                    rest = q;
                    multiplicity += 1;
                }
                if multiplicity != 0 {
                    brute.push((x, multiplicity));
                }
            }
            let expected = (rest.deg() == 0).then(|| pack(brute));
            assert_eq!(
                got, expected,
                "independent root division p={} u={u:?}",
                t.fp.p
            );
        }
        let reference = split_reference(u, &t.fp, &mut OracleStats::default());
        assert_eq!(got, reference, "p={} u={u:?}", t.fp.p);
        if let Some(roots) = got {
            let mut q = Poly::one();
            for &(x, m) in roots.as_slice() {
                for _ in 0..m {
                    q = q.mul(&Poly::x_minus(x, &t.fp), &t.fp);
                }
            }
            assert_eq!(q, *u);
        }
    }
    #[test]
    fn exhaustive_all_small_polynomials() {
        for p in [5u64, 7, 11, 13] {
            let t = Tables::new(p);
            for d in 1..=4 {
                for mut n in 0..p.pow(d as u32) {
                    let mut cs = vec![0; d + 1];
                    for c in &mut cs[..d] {
                        *c = n % p;
                        n /= p;
                    }
                    cs[d] = 1;
                    check(&t, &Poly::from_coeffs(&cs));
                }
            }
        }
    }
    #[test]
    fn randomized_and_constructed() {
        let mut rng = Rng::new(20261114);
        for p in [17, 19, 31, 101, 251, 503, 1009, 4001] {
            let t = Tables::new(p);
            for _ in 0..2000 {
                for d in [3, 4] {
                    let mut cs: Vec<_> = (0..d).map(|_| rng.below(p)).collect();
                    cs.push(1);
                    check(&t, &Poly::from_coeffs(&cs));
                }
                let mut u = Poly::one();
                let repeated = rng.below(p);
                for _ in 0..4 {
                    let x = if rng.below(2) == 0 {
                        repeated
                    } else {
                        rng.below(p)
                    };
                    u = u.mul(&Poly::x_minus(x, &t.fp), &t.fp);
                }
                check(&t, &u);
            }
        }
    }
}
