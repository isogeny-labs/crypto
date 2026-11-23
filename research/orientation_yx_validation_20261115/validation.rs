//! Independent small-field controls for ordinary binary S3 orientation.
//! No library arithmetic or polynomial implementation is imported.
#![forbid(unsafe_code)]

#[cfg(test)]
mod tests {
    type Point = Option<(u32, u32)>;

    #[derive(Clone, Copy)]
    struct Field {
        n: u32,
        modulus: u32,
    }
    impl Field {
        fn q(self) -> u32 {
            1 << self.n
        }
        fn mul(self, mut a: u32, mut b: u32) -> u32 {
            let mut z = 0;
            while b != 0 {
                if b & 1 != 0 {
                    z ^= a;
                }
                b >>= 1;
                a <<= 1;
                if a & self.q() != 0 {
                    a ^= self.modulus;
                }
            }
            z
        }
        fn sq(self, a: u32) -> u32 {
            self.mul(a, a)
        }
        fn pow(self, mut a: u32, mut k: u32) -> u32 {
            let mut z = 1;
            while k != 0 {
                if k & 1 != 0 {
                    z = self.mul(z, a);
                }
                a = self.sq(a);
                k >>= 1;
            }
            z
        }
        fn div(self, a: u32, b: u32) -> u32 {
            assert_ne!(b, 0, "chart denominator must not vanish");
            self.mul(a, self.pow(b, self.q() - 2))
        }
        fn trace(self, mut x: u32) -> u32 {
            let mut z = 0;
            for _ in 0..self.n {
                z ^= x;
                x = self.sq(x);
            }
            assert!(z <= 1);
            z
        }
    }
    #[derive(Clone, Copy)]
    struct Curve {
        f: Field,
        a: u32,
        b: u32,
    }
    impl Curve {
        fn on(self, (x, y): (u32, u32)) -> bool {
            let f = self.f;
            f.sq(y) ^ f.mul(x, y) ^ f.mul(f.sq(x), x) ^ f.mul(self.a, f.sq(x)) ^ self.b == 0
        }
        fn points(self) -> Vec<(u32, u32)> {
            (0..self.f.q())
                .flat_map(|x| (0..self.f.q()).map(move |y| (x, y)))
                .filter(|&p| self.on(p))
                .collect()
        }
        fn neg(self, p: Point) -> Point {
            p.map(|(x, y)| (x, x ^ y))
        }
        // Direct chord/tangent law, with explicit identity and inverse branches.
        fn add(self, p: Point, q: Point) -> Point {
            let (Some((x, y)), Some((u, v))) = (p, q) else {
                return p.or(q);
            };
            let f = self.f;
            let slope = if x == u {
                if y != v || x == 0 {
                    return None;
                }
                x ^ f.div(y, x)
            } else {
                f.div(y ^ v, x ^ u)
            };
            let z = f.sq(slope) ^ slope ^ x ^ u ^ self.a;
            Some((z, f.mul(slope, x ^ z) ^ z ^ y))
        }
        fn s3(self, x: u32, u: u32, z: u32) -> u32 {
            let f = self.f;
            f.mul(f.sq(x ^ u), f.sq(z)) ^ f.mul(f.mul(x, u), z) ^ f.sq(f.mul(x, u)) ^ self.b
        }
        fn mu(self, x: u32, u: u32, z: u32) -> u32 {
            self.f.mul(x, u) ^ self.f.mul(x, z) ^ self.f.mul(u, z)
        }
        // All Vieta constraints, including the slope constraint. Repeated
        // abscissae carry intersection multiplicity through these coefficients.
        fn oriented(self, p: (u32, u32), q: (u32, u32), r: (u32, u32)) -> bool {
            let f = self.f;
            let mu = self.mu(p.0, q.0, r.0);
            self.s3(p.0, q.0, r.0) == 0
                && (0..f.q()).any(|l| {
                    f.sq(l) ^ l ^ self.a == p.0 ^ q.0 ^ r.0
                        && [p, q, r].iter().all(|&(x, y)| y == f.mul(l, x) ^ mu)
                })
        }
        // Eliminates the slope using the supplied nonzero-x target r.
        fn anchored(self, p: (u32, u32), q: (u32, u32), r: (u32, u32)) -> bool {
            assert_ne!(r.0, 0);
            let f = self.f;
            let mu = self.mu(p.0, q.0, r.0);
            self.s3(p.0, q.0, r.0) == 0
                && [p, q]
                    .iter()
                    .all(|&(x, y)| f.mul(r.0, y) ^ f.mul(r.1, x) ^ f.mul(mu, x ^ r.0) == 0)
        }
        fn chart(self, x: u32, t: u32) -> u32 {
            let f = self.f;
            f.mul(f.sq(x), f.sq(t) ^ t ^ x ^ self.a) ^ self.b
        }
        fn hybrid(self, p: (u32, u32), q: (u32, u32), r: (u32, u32)) -> bool {
            let f = self.f;
            assert!(p.0 != 0 && q.0 != 0 && r.0 != 0);
            let tp = f.div(p.1, p.0);
            let tq = f.div(q.1, q.0);
            let tr = f.div(r.1, r.0);
            let mu = self.mu(p.0, q.0, r.0);
            self.s3(p.0, q.0, r.0) == 0
                && [(p.0, tp), (q.0, tq), (r.0, tr)]
                    .iter()
                    .all(|&(x, t)| self.chart(x, t) == 0)
                && [(p.0, tp), (q.0, tq)]
                    .iter()
                    .all(|&(x, t)| f.mul(f.mul(r.0, x), t ^ tr) ^ f.mul(mu, x ^ r.0) == 0)
        }
    }
    fn fields() -> [Field; 3] {
        [
            Field {
                n: 2,
                modulus: 0b111,
            },
            Field {
                n: 3,
                modulus: 0b1011,
            },
            Field {
                n: 4,
                modulus: 0b10011,
            },
        ]
    }

    #[test]
    fn fields_and_chart_are_valid_in_odd_and_even_degrees() {
        for f in fields() {
            for x in 1..f.q() {
                assert_eq!(f.mul(x, f.div(1, x)), 1);
            }
            for a in [0, 1, 2] {
                for b in [1, 2] {
                    let c = Curve { f, a, b };
                    for x in 0..f.q() {
                        for t in 0..f.q() {
                            assert_eq!(c.chart(x, t) == 0, c.on((x, f.mul(x, t))));
                            if x == 0 {
                                assert_ne!(c.chart(x, t), 0);
                            }
                            if x != 0 {
                                assert_eq!(c.chart(x, t), c.chart(x, t ^ 1));
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn oriented_and_hybrid_systems_equal_the_full_group_relation() {
        let mut triples = 0u64;
        let mut hybrid = 0u64;
        let mut repeated = 0u64;
        let mut zero = 0u64;
        for f in fields() {
            for a in [0, 1, 2] {
                for b in [1, 2] {
                    let c = Curve { f, a, b };
                    let points = c.points();
                    for &p in &points {
                        for &q in &points {
                            for &r in &points {
                                let wanted = c.add(c.add(Some(p), Some(q)), Some(r)).is_none();
                                assert_eq!(
                                    c.oriented(p, q, r),
                                    wanted,
                                    "oriented n={} a={a} b={b} {p:?} {q:?} {r:?}",
                                    f.n
                                );
                                if r.0 != 0 {
                                    assert_eq!(c.anchored(p, q, r), wanted);
                                }
                                if p.0 != 0 && q.0 != 0 && r.0 != 0 {
                                    assert_eq!(c.hybrid(p, q, r), wanted);
                                    hybrid += 1;
                                }
                                if p.0 == q.0 || p.0 == r.0 || q.0 == r.0 {
                                    repeated += 1;
                                }
                                if p.0 == 0 || q.0 == 0 || r.0 == 0 {
                                    zero += 1;
                                }
                                triples += 1;
                            }
                        }
                    }
                }
            }
        }
        eprintln!("18 curves: triples={triples}, hybrid={hybrid}, repeated_x={repeated}, containing_x_zero={zero}");
    }

    #[test]
    fn rational_lifts_require_trace_and_s3_does_not_fix_signs() {
        let mut false_signed = 0u64;
        let mut nonrational = 0u64;
        for f in fields() {
            for a in [0, 1, 2] {
                for b in [1, 2] {
                    let c = Curve { f, a, b };
                    let pts = c.points();
                    for x in 0..f.q() {
                        for u in 0..f.q() {
                            for z in 0..f.q() {
                                if c.s3(x, u, z) != 0 {
                                    continue;
                                }
                                let exists = pts.iter().filter(|p| p.0 == x).any(|&p| {
                                    pts.iter().filter(|q| q.0 == u).any(|&q| {
                                        c.neg(c.add(Some(p), Some(q))).is_some_and(|r| r.0 == z)
                                    })
                                });
                                assert_eq!(exists, f.trace(x ^ u ^ z ^ a) == 0);
                                nonrational += u64::from(!exists);
                            }
                        }
                    }
                    for &p in &pts {
                        for &q in &pts {
                            if let Some(r) = c.neg(c.add(Some(p), Some(q))) {
                                let flipped = c.neg(Some(r)).unwrap();
                                if flipped != r {
                                    assert_eq!(c.s3(p.0, q.0, flipped.0), 0);
                                    assert!(!c.oriented(p, q, flipped));
                                    false_signed += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(false_signed > 0 && nonrational > 0);
        eprintln!("negative controls: wrong-sign S3 roots={false_signed}, S3 roots without rational lifts={nonrational}");
    }

    #[test]
    fn collinearity_alone_is_not_a_multiplicity_test() {
        let c = Curve {
            f: fields()[1],
            a: 0,
            b: 1,
        };
        let f = c.f;
        let witness = c
            .points()
            .into_iter()
            .find(|&p| p.0 != 0 && c.add(c.add(Some(p), Some(p)), Some(p)).is_some())
            .unwrap();
        // Three copies of any point make the ordinary determinant zero.
        let determinant = f.mul(witness.0, witness.1 ^ witness.1)
            ^ f.mul(witness.0, witness.1 ^ witness.1)
            ^ f.mul(witness.0, witness.1 ^ witness.1);
        assert_eq!(determinant, 0);
        assert!(!c.oriented(witness, witness, witness));
        eprintln!("determinant-only counterexample: GF(8), a=0, b=1, P=Q=R={witness:?}");
    }

    #[test]
    fn zero_target_denominator_needs_an_explicit_branch() {
        let c = Curve {
            f: fields()[1],
            a: 0,
            b: 1,
        };
        let f = c.f;
        let pts = c.points();
        let r = *pts.iter().find(|p| p.0 == 0).unwrap();
        let mut witness = None;
        for &p in &pts {
            for &q in &pts {
                let mu = c.mu(p.0, q.0, r.0);
                let cleared = c.s3(p.0, q.0, r.0) == 0
                    && [p, q]
                        .iter()
                        .all(|&(x, y)| f.mul(r.0, y) ^ f.mul(r.1, x) ^ f.mul(mu, x ^ r.0) == 0);
                if cleared && c.add(c.add(Some(p), Some(q)), Some(r)).is_some() {
                    witness = Some((p, q, r));
                }
            }
        }
        let (p, q, r) = witness.unwrap();
        assert!(!c.oriented(p, q, r));
        eprintln!("unguarded-anchor counterexample: GF(8), a=0, b=1, P={p:?}, Q={q:?}, R={r:?}");
        for p in pts {
            assert_eq!(c.add(Some(p), c.neg(Some(p))), None);
            assert_eq!(c.add(Some(p), None), Some(p));
        }
    }

    #[test]
    fn anchor_slope_redundancy_is_an_exact_polynomial_identity() {
        // Sparse polynomials over F2 in X,U,Z,Y,A,B. This check has no
        // finite-field reduction, so it checks a formal identity in all degrees.
        use std::collections::BTreeSet;
        #[derive(Clone)]
        struct Pol(BTreeSet<[u8; 6]>);
        impl Pol {
            fn var(i: usize) -> Self {
                let mut m = [0; 6];
                m[i] = 1;
                Self(BTreeSet::from([m]))
            }
            fn add(&self, q: &Self) -> Self {
                Self(self.0.symmetric_difference(&q.0).copied().collect())
            }
            fn mul(&self, q: &Self) -> Self {
                let mut out = BTreeSet::new();
                for a in &self.0 {
                    for b in &q.0 {
                        let m = std::array::from_fn(|i| a[i] + b[i]);
                        if !out.insert(m) {
                            out.remove(&m);
                        }
                    }
                }
                Self(out)
            }
            fn sq(&self) -> Self {
                self.mul(self)
            }
        }
        let [x, u, z, y, a, b] = std::array::from_fn(Pol::var);
        let mu = x.mul(&u).add(&x.mul(&z)).add(&u.mul(&z));
        let s3 = mu.sq().add(&x.mul(&u).mul(&z)).add(&b);
        let curve = y
            .sq()
            .add(&z.mul(&y))
            .add(&z.sq().mul(&z))
            .add(&a.mul(&z.sq()))
            .add(&b);
        let ym = y.add(&mu);
        let slope = ym
            .sq()
            .add(&z.mul(&ym))
            .add(&x.add(&u).add(&z).add(&a).mul(&z.sq()));
        assert!(slope.add(&curve).add(&s3).0.is_empty());
    }
}
