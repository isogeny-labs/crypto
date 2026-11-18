use std::io::{self, BufWriter, Write};

// Independent implementation: no production Rust or earlier C++ arithmetic.
// Quadratic basis u+v*z, z^2=d, with d selected by an exhaustive nonsquare test.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Element {
    u: u32,
    v: u32,
}

#[derive(Clone, Copy, Debug)]
struct QuadraticField {
    p: u32,
    d: u32,
}

impl QuadraticField {
    fn new(p: u32) -> Self {
        let d = (2..p)
            .find(|&candidate| !(0..p).any(|x| x * x % p == candidate))
            .expect("no nonsquare in an odd prime field");
        Self { p, d }
    }

    fn base(self, x: u32) -> Element {
        Element { u: x % self.p, v: 0 }
    }

    fn add_base(self, x: u32, y: u32) -> u32 {
        (x + y) % self.p
    }

    fn sub_base(self, x: u32, y: u32) -> u32 {
        (x + self.p - y % self.p) % self.p
    }

    fn mul_base(self, x: u32, y: u32) -> u32 {
        (u64::from(x) * u64::from(y) % u64::from(self.p)) as u32
    }

    fn power_base(self, mut x: u32, mut exponent: u32) -> u32 {
        let mut result = 1;
        while exponent != 0 {
            if exponent & 1 != 0 {
                result = self.mul_base(result, x);
            }
            x = self.mul_base(x, x);
            exponent >>= 1;
        }
        result
    }

    fn add(self, a: Element, b: Element) -> Element {
        Element { u: self.add_base(a.u, b.u), v: self.add_base(a.v, b.v) }
    }

    fn neg(self, a: Element) -> Element {
        Element { u: self.sub_base(0, a.u), v: self.sub_base(0, a.v) }
    }

    fn sub(self, a: Element, b: Element) -> Element {
        self.add(a, self.neg(b))
    }

    fn mul(self, a: Element, b: Element) -> Element {
        Element {
            u: self.add_base(self.mul_base(a.u, b.u), self.mul_base(self.d, self.mul_base(a.v, b.v))),
            v: self.add_base(self.mul_base(a.u, b.v), self.mul_base(a.v, b.u)),
        }
    }

    fn inverse(self, a: Element) -> Element {
        assert_ne!(a, self.base(0), "zero inverse");
        let norm = self.sub_base(self.mul_base(a.u, a.u), self.mul_base(self.d, self.mul_base(a.v, a.v)));
        assert_ne!(norm, 0, "nonzero element with zero norm");
        let inverse_norm = self.power_base(norm, self.p - 2);
        Element {
            u: self.mul_base(a.u, inverse_norm),
            v: self.mul_base(self.sub_base(0, a.v), inverse_norm),
        }
    }

    fn index(self, a: Element) -> usize {
        (a.u + self.p * a.v) as usize
    }

    fn square_roots_and_check(self) -> Vec<Option<Element>> {
        let mut roots = vec![None; (self.p * self.p) as usize];
        for u in 0..self.p {
            for v in 0..self.p {
                let a = Element { u, v };
                if a != self.base(0) {
                    assert_eq!(self.mul(a, self.inverse(a)), self.base(1), "inverse check");
                }
                let index = self.index(self.mul(a, a));
                if roots[index].is_none() {
                    roots[index] = Some(a);
                }
            }
        }
        for a in 0..self.p {
            let root = roots[self.index(self.base(a))].expect("base element missing a quadratic lift");
            assert_eq!(self.mul(root, root), self.base(a));
        }
        roots
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AffinePoint {
    x: Element,
    y: Element,
}

#[derive(Clone, Copy, Debug)]
struct Curve {
    field: QuadraticField,
    a: u32,
    b: u32,
}

impl Curve {
    fn rhs(self, x: Element) -> Element {
        let f = self.field;
        f.add(f.add(f.mul(f.mul(x, x), x), f.mul(f.base(self.a), x)), f.base(self.b))
    }

    fn contains(self, point: AffinePoint) -> bool {
        self.field.mul(point.y, point.y) == self.rhs(point.x)
    }

    fn sum(self, a: AffinePoint, b: AffinePoint) -> Option<AffinePoint> {
        let f = self.field;
        assert!(self.contains(a) && self.contains(b));
        if a.x == b.x && f.add(a.y, b.y) == f.base(0) {
            return None;
        }
        let slope = if a.x == b.x {
            assert_eq!(a.y, b.y, "same abscissa without opposite or equal ordinates");
            f.mul(
                f.add(f.mul(f.base(3), f.mul(a.x, a.x)), f.base(self.a)),
                f.inverse(f.mul(f.base(2), a.y)),
            )
        } else {
            f.mul(f.sub(b.y, a.y), f.inverse(f.sub(b.x, a.x)))
        };
        let x = f.sub(f.sub(f.mul(slope, slope), a.x), b.x);
        let result = AffinePoint { x, y: f.sub(f.mul(slope, f.sub(a.x, x)), a.y) };
        assert!(self.contains(result), "sum not on curve");
        Some(result)
    }

    fn polynomial(self, x: u32, y: u32, z: u32) -> u32 {
        let f = self.field;
        let xy = f.mul_base(x, y);
        let x_plus_y = f.add_base(x, y);
        let difference = f.sub_base(x, y);
        let first = f.mul_base(f.mul_base(difference, difference), f.mul_base(z, z));
        let middle = f.add_base(f.mul_base(x_plus_y, f.add_base(xy, self.a)), f.mul_base(2, self.b));
        let second = f.mul_base(2, f.mul_base(middle, z));
        let third = f.mul_base(f.sub_base(xy, self.a), f.sub_base(xy, self.a));
        let fourth = f.mul_base(4, f.mul_base(self.b, x_plus_y));
        f.sub_base(f.add_base(first, third), f.add_base(second, fourth))
    }
}

fn census(p: u32, output: &mut impl Write) {
    let f = QuadraticField::new(p);
    let square_roots = f.square_roots_and_check();
    let mut curves = 0_u64;
    let mut tuples = 0_u64;
    let mut zeros = 0_u64;
    let mut false_zeros = 0_u64;
    let mut missing_zeros = 0_u64;
    for a in 0..p {
        for b in 0..p {
            let discriminant_core = f.add_base(f.mul_base(4, f.mul_base(f.mul_base(a, a), a)),
                                                f.mul_base(27, f.mul_base(b, b)));
            if discriminant_core == 0 {
                continue;
            }
            curves += 1;
            let curve = Curve { field: f, a, b };
            let points: Vec<AffinePoint> = (0..p).map(|x| {
                let x = f.base(x);
                let y = square_roots[f.index(curve.rhs(x))].expect("no base-coordinate lift");
                let point = AffinePoint { x, y };
                assert!(curve.contains(point));
                point
            }).collect();
            let mut relation = vec![false; (p * p * p) as usize];
            for x in 0..p {
                for y in 0..p {
                    for sign_x in [false, true] {
                        for sign_y in [false, true] {
                            let mut left = points[x as usize];
                            let mut right = points[y as usize];
                            if sign_x { left.y = f.neg(left.y); }
                            if sign_y { right.y = f.neg(right.y); }
                            if let Some(sum) = curve.sum(left, right) {
                                if sum.x.v == 0 {
                                    relation[((x * p + y) * p + sum.x.u) as usize] = true;
                                }
                            }
                        }
                    }
                }
            }
            for x in 0..p {
                for y in 0..p {
                    for z in 0..p {
                        tuples += 1;
                        let vanishes = curve.polynomial(x, y, z) == 0;
                        let sum_exists = relation[((x * p + y) * p + z) as usize];
                        zeros += u64::from(vanishes);
                        false_zeros += u64::from(vanishes && !sum_exists);
                        missing_zeros += u64::from(!vanishes && sum_exists);
                    }
                }
            }
        }
    }
    writeln!(output,
        "{{\"q\":{p},\"m\":3,\"curves\":{curves},\"tuples\":{tuples},\"zeros\":{zeros},\"false_zeros\":{false_zeros},\"missing_zeros\":{missing_zeros}}}"
    ).expect("write output");
    eprintln!("{{\"q\":{p},\"nonsquare\":{},\"inverse_checks\":{},\"base_square_lifts\":{p},\"sum_membership_checks\":true}}", f.d, p*p-1);
    assert_eq!(curves, u64::from(p * (p - 1)), "nonsingular-model count");
    assert_eq!(false_zeros, 0, "S3 false zero over p={p}");
    assert_eq!(missing_zeros, 0, "S3 missed sum over p={p}");
}

fn main() {
    let mode = std::env::args().nth(1).expect("usage: validator --preflight|--preflight-large|--prime-s3");
    let primes: &[u32] = match mode.as_str() {
        "--preflight" => &[3],
        "--preflight-large" => &[23],
        "--prime-s3" => &[5, 7, 11, 13, 17, 19],
        _ => panic!("unknown mode"),
    };
    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());
    for &p in primes {
        census(p, &mut output);
    }
}
