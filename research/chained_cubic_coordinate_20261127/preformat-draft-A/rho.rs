use crate::accounting;
use crate::divisor_a::{self, Div};
use crate::elliptic_a::EP;
use crate::field_d::{add, inv, mul, sub};
use std::cmp::Ordering;

pub fn ec_valid(p: EP) -> bool {
    p.is_none_or(|(x, y)| {
        x < 7 && y < 7 && mul(y, y) == add(add(mul(mul(x, x), x), mul(2, x)), 6)
    })
}
pub fn ec_neg(p: EP) -> EP {
    p.map(|(x, y)| (x, sub(0, y)))
}
pub fn ec_add(p: EP, q: EP) -> EP {
    assert!(ec_valid(p) && ec_valid(q));
    let (Some((x, y)), Some((u, v))) = (p, q) else {
        return p.or(q);
    };
    if x == u && add(y, v) == 0 {
        return None;
    }
    let slope = if x == u {
        assert_eq!(y, v);
        mul(add(mul(3, mul(x, x)), 2), inv(mul(2, y)))
    } else {
        mul(sub(v, y), inv(sub(u, x)))
    };
    let nx = sub(sub(mul(slope, slope), x), u);
    let ny = sub(mul(slope, sub(x, nx)), y);
    let out = Some((nx, ny));
    assert!(ec_valid(out));
    out
}
pub fn ec_mul(p: EP, n: usize) -> EP {
    generic_mul::<Elliptic>(&p, n)
}

pub trait Group {
    type Point: Clone;
    fn zero() -> Self::Point;
    fn add(a: &Self::Point, b: &Self::Point) -> Self::Point;
    fn neg(a: &Self::Point) -> Self::Point;
    fn compare(a: &Self::Point, b: &Self::Point) -> Ordering;
    fn hash(a: &Self::Point, seed: u64) -> u64;
}
pub struct Elliptic;
impl Group for Elliptic {
    type Point = EP;
    fn zero() -> EP {
        None
    }
    fn add(a: &EP, b: &EP) -> EP {
        ec_add(*a, *b)
    }
    fn neg(a: &EP) -> EP {
        ec_neg(*a)
    }
    fn compare(a: &EP, b: &EP) -> Ordering {
        accounting::compare(1);
        match (a, b) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some((x, y)), Some((u, v))) => {
                accounting::compare(1);
                match x.cmp(u) {
                    Ordering::Equal => {
                        accounting::compare(1);
                        y.cmp(v)
                    }
                    result => result,
                }
            }
        }
    }
    fn hash(a: &EP, seed: u64) -> u64 {
        match a {
            None => mix(seed),
            Some((x, y)) => mix(mix(mix(seed ^ 1) ^ u64::from(*x)) ^ u64::from(*y)),
        }
    }
}
pub struct Jacobian;
impl Group for Jacobian {
    type Point = Div;
    fn zero() -> Div {
        divisor_a::identity()
    }
    fn add(a: &Div, b: &Div) -> Div {
        divisor_a::cantor(a, b)
    }
    fn neg(a: &Div) -> Div {
        divisor_a::neg(a)
    }
    fn compare(a: &Div, b: &Div) -> Ordering {
        fn vector(a: &[u8], b: &[u8]) -> Ordering {
            for (x, y) in a.iter().zip(b) {
                accounting::compare(1);
                let cmp = x.cmp(y);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            accounting::compare(1);
            a.len().cmp(&b.len())
        }
        match vector(&a.u, &b.u) {
            Ordering::Equal => vector(&a.v, &b.v),
            result => result,
        }
    }
    fn hash(a: &Div, seed: u64) -> u64 {
        let mut h = mix(seed ^ u64::try_from(a.u.len()).unwrap());
        for &x in &a.u {
            h = mix(h ^ u64::from(x));
        }
        h = mix(h ^ u64::try_from(a.v.len()).unwrap());
        for &x in &a.v {
            h = mix(h ^ u64::from(x));
        }
        h
    }
}
pub fn generic_mul<A: Group>(p: &A::Point, mut n: usize) -> A::Point {
    let mut result = A::zero();
    let mut base = p.clone();
    while n > 0 {
        if n & 1 == 1 {
            result = A::add(&result, &base);
        }
        n >>= 1;
        if n > 0 {
            base = A::add(&base, &base);
        }
    }
    result
}
// Exactly the SplitMix64 finalizer with its Weyl increment. Each recorded
// mix charges this whole word-operation block; it is not a field operation.
fn mix(x: u64) -> u64 {
    accounting::hash_mix();
    let mut z = x.wrapping_add(0x9e3779b97f4a7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
pub fn target_seed(master: u64, q: EP) -> u64 {
    Elliptic::hash(&q, mix(master ^ 0x434d313943554231))
}
pub fn restart_stream(seed: u64, restart: usize) -> u64 {
    mix(mix(seed ^ 0x5245535441525431) ^ u64::try_from(restart).unwrap())
}
fn sa(a: usize, b: usize) -> usize {
    accounting::scalar_add();
    (a + b) % 11
}
fn ss(a: usize, b: usize) -> usize {
    accounting::scalar_add();
    (a + 11 - b) % 11
}
fn sm(a: usize, b: usize) -> usize {
    accounting::scalar_mul();
    (a * b) % 11
}
fn si(a: usize) -> usize {
    assert!(a > 0 && a < 11);
    (0..9).fold(1, |r, _| sm(r, a))
}

#[derive(Clone)]
struct State<P> {
    p: P,
    a: usize,
    b: usize,
}
fn normalize<A: Group>(s: State<A::Point>) -> State<A::Point> {
    let neg = A::neg(&s.p);
    if A::compare(&neg, &s.p) == Ordering::Less {
        State {
            p: neg,
            a: ss(0, s.a),
            b: ss(0, s.b),
        }
    } else {
        s
    }
}
fn advance<A: Group>(s: State<A::Point>, table: &[State<A::Point>], seed: u64) -> State<A::Point> {
    let index = usize::try_from(A::hash(&s.p, seed) & 7).unwrap();
    let jump = &table[index];
    normalize::<A>(State {
        p: A::add(&s.p, &jump.p),
        a: sa(s.a, jump.a),
        b: sa(s.b, jump.b),
    })
}

#[derive(Clone, Copy, Debug)]
pub struct Outcome {
    pub scalar: usize,
    pub restarts: usize,
    pub walk_steps: usize,
    pub degenerate_collisions: usize,
    pub used_streams: [u64; 32],
}
// Source and destination use this identical negation-folded, eight-adding
// Floyd implementation. No target scalar or product-group label enters it.
pub fn solve<A: Group>(g: &A::Point, q: &A::Point, order: usize, seed: u64) -> Option<Outcome> {
    assert_eq!(order, 11);
    assert!(A::compare(g, &A::zero()) != Ordering::Equal);
    let mut walk_steps = 0;
    let mut degenerate = 0;
    let mut used_streams = [0; 32];
    for restart in 0..32 {
        let stream = restart_stream(seed, restart);
        used_streams[restart] = stream;
        let mut coin = stream;
        let mut draw = || {
            coin = mix(coin);
            usize::try_from(coin % 11).unwrap()
        };
        let mut table = Vec::with_capacity(8);
        for _ in 0..8 {
            let (a, b) = (draw(), draw());
            let p = A::add(&generic_mul::<A>(g, a), &generic_mul::<A>(q, b));
            table.push(normalize::<A>(State { p, a, b }));
        }
        let (a, b) = (draw(), draw());
        let start = normalize::<A>(State {
            p: A::add(&generic_mul::<A>(g, a), &generic_mul::<A>(q, b)),
            a,
            b,
        });
        let mut tortoise = advance::<A>(start.clone(), &table, stream);
        let mut hare = advance::<A>(advance::<A>(start, &table, stream), &table, stream);
        walk_steps += 3;
        for _ in 0..64 {
            if A::compare(&tortoise.p, &hare.p) == Ordering::Equal {
                let numerator = ss(tortoise.a, hare.a);
                let denominator = ss(hare.b, tortoise.b);
                if denominator != 0 {
                    let scalar = sm(numerator, si(denominator));
                    assert_eq!(A::compare(&generic_mul::<A>(g, scalar), q), Ordering::Equal);
                    return Some(Outcome {
                        scalar,
                        restarts: restart,
                        walk_steps,
                        degenerate_collisions: degenerate,
                        used_streams,
                    });
                }
                degenerate += 1;
                break;
            }
            tortoise = advance::<A>(tortoise, &table, stream);
            hare = advance::<A>(advance::<A>(hare, &table, stream), &table, stream);
            walk_steps += 3;
        }
    }
    None
}
