//! Bounded affine reference arithmetic (SEC 1 v2, section 2.2.1).
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counts {
    pub group_add_calls: u64,
    pub field_inversions: u64,
    pub character_tests: u64,
    pub square_roots: u64,
    pub hash_lookups: u64,
    pub linear_cells: u64,
    pub linear_inversions: u64,
}

impl Counts {
    pub fn difference(self, old: Self) -> Self {
        Self {
            group_add_calls: self.group_add_calls - old.group_add_calls,
            field_inversions: self.field_inversions - old.field_inversions,
            character_tests: self.character_tests - old.character_tests,
            square_roots: self.square_roots - old.square_roots,
            hash_lookups: self.hash_lookups - old.hash_lookups,
            linear_cells: self.linear_cells - old.linear_cells,
            linear_inversions: self.linear_inversions - old.linear_inversions,
        }
    }
    pub fn accumulate(&mut self, other: Self) {
        self.group_add_calls += other.group_add_calls;
        self.field_inversions += other.field_inversions;
        self.character_tests += other.character_tests;
        self.square_roots += other.square_roots;
        self.hash_lookups += other.hash_lookups;
        self.linear_cells += other.linear_cells;
        self.linear_inversions += other.linear_inversions;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}
pub type Element = Option<Point>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameters {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}

pub struct Curve {
    pub params: Parameters,
    pub counts: Counts,
}

pub fn norm(x: i128, p: u64) -> u64 {
    x.rem_euclid(p as i128) as u64
}

pub fn power(mut x: u64, mut k: u64, p: u64) -> u64 {
    let mut result = 1;
    while k != 0 {
        if k & 1 != 0 {
            result = ((result as u128 * x as u128) % p as u128) as u64;
        }
        x = ((x as u128 * x as u128) % p as u128) as u64;
        k >>= 1;
    }
    result
}

pub fn prime(n: u64) -> bool {
    n >= 2
        && (2..)
            .take_while(|d| d * d <= n)
            .all(|d| !n.is_multiple_of(d))
}

impl Curve {
    pub fn new(p: u64, a: u64, b: u64) -> Self {
        Self {
            params: Parameters { p, a, b },
            counts: Counts::default(),
        }
    }
    pub fn nonsingular(&self) -> bool {
        let Parameters { p, a, b } = self.params;
        norm(4 * (a as i128).pow(3) + 27 * (b as i128).pow(2), p) != 0
    }
    pub fn f(&self, x: u64) -> u64 {
        let Parameters { p, a, b } = self.params;
        norm((x as i128).pow(3) + a as i128 * x as i128 + b as i128, p)
    }
    pub fn on_curve(&self, point: Element) -> bool {
        point.is_none_or(|q| {
            q.x < self.params.p && q.y < self.params.p && q.y * q.y % self.params.p == self.f(q.x)
        })
    }
    pub fn character(&mut self, x: u64) -> i64 {
        self.counts.character_tests += 1;
        let p = self.params.p;
        let value = power(self.f(x), (p - 1) / 2, p);
        if value == p - 1 {
            -1
        } else {
            value as i64
        }
    }
    pub fn point(&mut self, x: u64, known_square: bool) -> Element {
        if !known_square && self.character(x) != 1 {
            return None;
        }
        self.counts.square_roots += 1;
        let p = self.params.p;
        let y = power(self.f(x), (p + 1) / 4, p);
        assert_eq!(y * y % p, self.f(x));
        Some(Point { x, y: y.min(p - y) })
    }
    pub fn neg(&self, point: Element) -> Element {
        point.map(|q| Point {
            x: q.x,
            y: (self.params.p - q.y) % self.params.p,
        })
    }
    pub fn add(&mut self, left: Element, right: Element) -> Element {
        self.counts.group_add_calls += 1;
        let (Some(l), Some(r)) = (left, right) else {
            return left.or(right);
        };
        let p = self.params.p;
        if l.x == r.x && (l.y + r.y) % p == 0 {
            return None;
        }
        let (num, den) = if l == r {
            (
                3 * (l.x as i128).pow(2) + self.params.a as i128,
                2 * l.y as i128,
            )
        } else {
            (r.y as i128 - l.y as i128, r.x as i128 - l.x as i128)
        };
        self.counts.field_inversions += 1;
        let den = norm(den, p);
        assert_ne!(den, 0);
        let slope = norm(num * power(den, p - 2, p) as i128, p);
        let x = norm((slope as i128).pow(2) - l.x as i128 - r.x as i128, p);
        let y = norm(slope as i128 * (l.x as i128 - x as i128) - l.y as i128, p);
        Some(Point { x, y })
    }
    pub fn mul(&mut self, mut k: u64, mut point: Element) -> Element {
        let mut result = None;
        while k != 0 {
            if k & 1 != 0 {
                result = self.add(result, point);
            }
            k >>= 1;
            if k != 0 {
                point = self.add(point, point);
            }
        }
        result
    }
}

/// SplitMix64, for reproducible public fixtures, never cryptographic secrets.
pub struct Rng(pub u64);
impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: u64) -> u64 {
        assert_ne!(n, 0);
        let limit = u64::MAX - u64::MAX % n;
        loop {
            let x = self.next_u64();
            if x < limit {
                return x % n;
            }
        }
    }
    pub fn shuffle<T>(&mut self, values: &mut [T]) {
        for i in (1..values.len()).rev() {
            values.swap(i, self.below(i as u64 + 1) as usize);
        }
    }
}

pub fn discover(p: u64) -> (Parameters, u64, Point) {
    for b in 1..p {
        let mut e = Curve::new(p, p - 3, b);
        if !e.nonsingular() {
            continue;
        }
        let n = (p as i64 + 1 + (0..p).map(|x| e.character(x)).sum::<i64>()) as u64;
        if prime(n) {
            let g = (0..p).find_map(|x| e.point(x, false)).unwrap();
            assert!(e.mul(n, Some(g)).is_none());
            return (e.params, n, g);
        }
    }
    panic!("No prime-order model found in the specified a=-3 family");
}
