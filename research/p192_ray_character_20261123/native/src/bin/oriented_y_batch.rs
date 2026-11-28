//! Native finite replay of the oriented y-product pair identity.
//! Run: cargo run --release --manifest-path research/p192_ray_character_20261123/native/Cargo.toml --bin oriented_y_batch
//! The polynomial arithmetic is simple quadratic multiplication for a finite
//! control; P-192 counts below are derived, never executed.

use num_bigint::BigUint;
use num_traits::{One, Zero};

#[derive(Clone, Copy)]
struct Field(u64);

impl Field {
    fn add(self, x: u64, y: u64) -> u64 {
        (x + y) % self.0
    }
    fn neg(self, x: u64) -> u64 {
        if x == 0 {
            0
        } else {
            self.0 - x
        }
    }
    fn sub(self, x: u64, y: u64) -> u64 {
        self.add(x, self.neg(y))
    }
    fn mul(self, x: u64, y: u64) -> u64 {
        ((x as u128 * y as u128) % self.0 as u128) as u64
    }
    fn pow(self, mut x: u64, mut k: u64) -> u64 {
        let mut out = 1;
        while k > 0 {
            if k & 1 == 1 {
                out = self.mul(out, x);
            }
            x = self.mul(x, x);
            k >>= 1;
        }
        out
    }
    fn inv(self, x: u64) -> u64 {
        assert_ne!(x, 0);
        self.pow(x, self.0 - 2)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
    infinity: bool,
}

impl Point {
    fn infinity() -> Self {
        Self {
            x: 0,
            y: 0,
            infinity: true,
        }
    }
}

#[derive(Clone, Copy)]
struct Curve {
    f: Field,
    a: u64,
    b: u64,
}

impl Curve {
    fn add(self, p: Point, q: Point) -> Point {
        if p.infinity {
            return q;
        }
        if q.infinity {
            return p;
        }
        if p.x == q.x && self.f.add(p.y, q.y) == 0 {
            return Point::infinity();
        }
        let slope = if p == q {
            let top = self.f.add(self.f.mul(3, self.f.mul(p.x, p.x)), self.a);
            self.f.mul(top, self.f.inv(self.f.mul(2, p.y)))
        } else {
            self.f
                .mul(self.f.sub(q.y, p.y), self.f.inv(self.f.sub(q.x, p.x)))
        };
        let x = self.f.sub(self.f.sub(self.f.mul(slope, slope), p.x), q.x);
        let y = self.f.sub(self.f.mul(slope, self.f.sub(p.x, x)), p.y);
        Point {
            x,
            y,
            infinity: false,
        }
    }
    fn scale(self, mut p: Point, mut k: u64) -> Point {
        let mut out = Point::infinity();
        while k > 0 {
            if k & 1 == 1 {
                out = self.add(out, p);
            }
            p = self.add(p, p);
            k >>= 1;
        }
        out
    }
    fn consecutive(self, p: Point, start: u64, step: u64, count: usize) -> Vec<Point> {
        let mut out = Vec::with_capacity(count);
        if count == 0 {
            return out;
        }
        let mut current = self.scale(p, start);
        let increment = self.scale(p, step);
        for _ in 0..count {
            out.push(current);
            current = self.add(current, increment);
        }
        out
    }
}

type Poly = Vec<u64>;
fn poly_mul(f: Field, a: &[u64], b: &[u64]) -> Poly {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            out[i + j] = f.add(out[i + j], f.mul(*x, *y));
        }
    }
    out
}
fn poly_product(f: Field, mut values: Vec<Poly>) -> Poly {
    assert!(!values.is_empty());
    while values.len() > 1 {
        values = values
            .chunks(2)
            .map(|pair| {
                if pair.len() == 2 {
                    poly_mul(f, &pair[0], &pair[1])
                } else {
                    pair[0].clone()
                }
            })
            .collect();
    }
    values.pop().unwrap()
}
fn poly_eval(f: Field, coefficients: &[u64], x: u64) -> u64 {
    coefficients
        .iter()
        .rev()
        .fold(0, |acc, c| f.add(f.mul(acc, x), *c))
}
fn pair_numerator(curve: Curve, v: u64) -> Poly {
    let f = curve.f;
    let a = curve.a;
    let b = curve.b;
    let a2 = f.mul(a, a);
    let a3 = f.mul(a2, a);
    let b2 = f.mul(b, b);
    let v2 = f.mul(v, v);
    let v3 = f.mul(v2, v);
    let c0 = f.sub(
        f.add(
            f.add(a3, f.mul(a2, v2)),
            f.add(f.mul(2, f.mul(a, f.mul(b, v))), f.mul(8, b2)),
        ),
        f.mul(b, v3),
    );
    let c1 = f.sub(
        f.sub(
            f.add(f.mul(3, f.mul(a2, v)), f.mul(2, f.mul(a, b))),
            f.mul(a, v3),
        ),
        f.mul(9, f.mul(b, v2)),
    );
    let c2 = f.sub(f.sub(a2, f.mul(3, f.mul(a, v2))), f.mul(9, f.mul(b, v)));
    let c3 = f.neg(f.add(f.add(f.mul(a, v), b), v3));
    vec![c0, c1, c2, c3]
}
fn blocks(n: u64) -> (Vec<(u64, u64, u64)>, u64) {
    let width = ((n / 2) as f64).sqrt() as u64;
    let width = width.max(1);
    let count = n / (4 * width);
    let remainder = n - 4 * width * count;
    let mut out = vec![(0, width, count)];
    if remainder >= 4 {
        out.push((4 * width * count, remainder / 4, 1));
    }
    let tail_start = 4 * width * count + 4 * (remainder / 4);
    (out, tail_start)
}
fn batched(curve: Curve, point: Point, order: u64) -> u64 {
    assert_eq!(order % 4, 1);
    assert!(curve.scale(point, order).infinity);
    let n = (order - 1) / 2;
    let (blocks, tail_start) = blocks(n);
    let f = curve.f;
    let mut out = 1;
    for (offset, width, count) in blocks {
        let offsets = curve.consecutive(point, 1, 2, width as usize);
        let numerator = poly_product(
            f,
            offsets.iter().map(|p| pair_numerator(curve, p.x)).collect(),
        );
        let denominator = poly_product(f, offsets.iter().map(|p| vec![f.neg(p.x), 1]).collect());
        let centers = curve
            .consecutive(point, offset + 2 * width, 4 * width, count as usize)
            .into_iter()
            .chain(curve.consecutive(point, offset + 2 * width + 1, 4 * width, count as usize));
        for center in centers {
            let top = poly_eval(f, &numerator, center.x);
            let bottom = poly_eval(f, &denominator, center.x);
            out = f.mul(out, f.mul(top, f.inv(f.pow(bottom, 3))));
        }
    }
    for j in tail_start + 1..=n {
        out = f.mul(out, curve.scale(point, j).y);
    }
    out
}
fn direct(curve: Curve, point: Point, order: u64) -> u64 {
    let mut out = 1;
    let mut current = point;
    for _ in 0..(order - 1) / 2 {
        out = curve.f.mul(out, current.y);
        current = curve.add(current, point);
    }
    out
}
fn control(p: u64, a: u64, b: u64, gx: u64, gy: u64, order: u64, marks: &[u64]) {
    let curve = Curve { f: Field(p), a, b };
    let g = Point {
        x: gx,
        y: gy,
        infinity: false,
    };
    assert!(curve.scale(g, order).infinity);
    let base = batched(curve, g, order);
    assert_eq!(base, direct(curve, g, order));
    for &d in marks {
        let q = curve.scale(g, d);
        let value = batched(curve, q, order);
        assert_eq!(value, direct(curve, q, order), "direct mismatch at {d}");
        let symbol = Field(order).pow(d, (order - 1) / 2);
        let expected = if symbol == 1 { 1 } else { p - 1 };
        assert_eq!(
            curve.f.mul(value, curve.f.inv(base)),
            expected,
            "sign mismatch at {d}"
        );
    }
    println!(
        "finite_control p={p} a={a} b={b} order={order} marks={} pass",
        marks.len()
    );
}
fn main() {
    let all: Vec<u64> = (1..97).collect();
    control(79, 0, 3, 1, 2, 97, &all);
    control(79, 1, 6, 1, 18, 97, &all);
    control(6666679, 5, 5, 9970, 1003793, 10009, &[1, 2, 3, 5]);
    let r: BigUint = "6277101735386680763835789423176059013767194773182842284081"
        .parse()
        .unwrap();
    let n = (&r - BigUint::one()) >> 1usize;
    let width = (&n >> 1usize).sqrt();
    let four = BigUint::from(4u8);
    let count = &n / (&four * &width);
    let rem = &n - &four * &width * &count;
    let tail_width = &rem / &four;
    let centers = &count * BigUint::from(2u8)
        + if tail_width.is_zero() {
            BigUint::zero()
        } else {
            BigUint::from(2u8)
        };
    let offsets = &width + &tail_width;
    let tail = &rem - &four * &tail_width;
    let samples = &centers + &offsets + &tail;
    let tree_bytes = BigUint::from(32u8)
        * std::cmp::max(
            BigUint::from(3u8) * &width + BigUint::one(),
            centers.clone(),
        );
    let cap_elements = BigUint::one() << 45usize;
    let cap_centers = (&n + BigUint::from(2u8) * &cap_elements - BigUint::one())
        / (BigUint::from(2u8) * cap_elements);
    println!("p192_size n={n} centers={centers} offsets={offsets} direct_tail={tail} point_samples={samples}");
    println!("p192_gate minimum_tree_bytes={tree_bytes} one_pib_centers_lower={cap_centers}");
}
