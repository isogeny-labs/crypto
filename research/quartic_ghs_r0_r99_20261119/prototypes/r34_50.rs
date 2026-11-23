//! Deterministic producer-side screen for R34-R50.
//!
//! This is deliberately a correctness verifier, not a benchmark.  It checks
//! a width-two simultaneous scalar multiplication against two independent
//! scalar multiplications, then checks that a forced infinity retry consumes
//! the same coefficient stream and charges every attempted walker.

const P: i64 = 223;
const CURVE_A: i64 = 0;
const CURVE_B: i64 = 7;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Point {
    Infinity,
    Affine { x: i64, y: i64 },
}

fn modulo(x: i64) -> i64 {
    x.rem_euclid(P)
}

fn inverse(x: i64) -> i64 {
    let (mut old_r, mut r) = (modulo(x), P);
    let (mut old_s, mut s) = (1_i64, 0_i64);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    assert_eq!(old_r, 1, "attempted to invert a non-unit");
    modulo(old_s)
}

fn on_curve(point: Point) -> bool {
    match point {
        Point::Infinity => true,
        Point::Affine { x, y } => modulo(y * y) == modulo(x * x * x + CURVE_A * x + CURVE_B),
    }
}

fn negate(point: Point) -> Point {
    match point {
        Point::Infinity => Point::Infinity,
        Point::Affine { x, y } => Point::Affine { x, y: modulo(-y) },
    }
}

fn add(left: Point, right: Point) -> Point {
    match (left, right) {
        (Point::Infinity, point) | (point, Point::Infinity) => point,
        (Point::Affine { x: x1, y: y1 }, Point::Affine { x: x2, y: y2 }) => {
            if x1 == x2 && modulo(y1 + y2) == 0 {
                return Point::Infinity;
            }
            let slope = if x1 == x2 {
                modulo((3 * x1 * x1 + CURVE_A) * inverse(2 * y1))
            } else {
                modulo((y2 - y1) * inverse(x2 - x1))
            };
            let x3 = modulo(slope * slope - x1 - x2);
            let y3 = modulo(slope * (x1 - x3) - y1);
            let out = Point::Affine { x: x3, y: y3 };
            assert!(on_curve(out));
            out
        }
    }
}

fn scalar_mul(mut point: Point, mut scalar: u64) -> Point {
    let mut out = Point::Infinity;
    while scalar != 0 {
        if scalar & 1 == 1 {
            out = add(out, point);
        }
        scalar >>= 1;
        if scalar != 0 {
            point = add(point, point);
        }
    }
    out
}

fn point_order(point: Point) -> u64 {
    let mut acc = Point::Infinity;
    for order in 1..=4 * P as u64 {
        acc = add(acc, point);
        if acc == Point::Infinity {
            return order;
        }
    }
    panic!("point order exceeded the Hasse-safe search cap")
}

fn choose_base() -> (Point, u64) {
    let mut best = (Point::Infinity, 0);
    for x in 0..P {
        for y in 0..P {
            let point = Point::Affine { x, y };
            if on_curve(point) {
                let order = point_order(point);
                if order > best.1 {
                    best = (point, order);
                }
            }
        }
    }
    assert!(best.1 > 32, "toy curve did not expose a useful subgroup");
    best
}

fn window2_table(g: Point, q: Point) -> [Point; 16] {
    let mut table = [Point::Infinity; 16];
    for a_digit in 0..4_u64 {
        for b_digit in 0..4_u64 {
            table[(4 * a_digit + b_digit) as usize] =
                add(scalar_mul(g, a_digit), scalar_mul(q, b_digit));
        }
    }
    table
}

fn joint_window2(table: &[Point; 16], a: u64, b: u64) -> Point {
    let bits = 64 - (a | b).leading_zeros() as usize;
    if bits == 0 {
        return Point::Infinity;
    }
    let top_window = (bits - 1) / 2;
    let mut out = Point::Infinity;
    for window in (0..=top_window).rev() {
        if window != top_window {
            out = add(out, out);
            out = add(out, out);
        }
        let shift = 2 * window;
        let digit = ((((a >> shift) & 3) << 2) | ((b >> shift) & 3)) as usize;
        out = add(out, table[digit]);
    }
    out
}

fn canonicalize(point: Point, a: u64, b: u64, order: u64) -> (Point, u64, u64) {
    match point {
        Point::Infinity => (point, a, b),
        Point::Affine { y, .. } if y > P / 2 => (
            negate(point),
            if a == 0 { 0 } else { order - a },
            if b == 0 { 0 } else { order - b },
        ),
        _ => (point, a, b),
    }
}

#[derive(Clone)]
struct PairStream {
    pairs: Vec<(u64, u64)>,
    cursor: usize,
    draws: u64,
}

impl PairStream {
    fn next(&mut self) -> (u64, u64) {
        let pair = self.pairs[self.cursor];
        self.cursor += 1;
        self.draws += 2;
        pair
    }
}

#[derive(Debug, Eq, PartialEq)]
struct WalkerReceipt {
    point: Point,
    a: u64,
    b: u64,
    attempts: u64,
    discarded: u64,
    draws: u64,
    cursor: usize,
}

fn construct_walker<F>(
    stream: &mut PairStream,
    g: Point,
    q: Point,
    order: u64,
    mut multiply: F,
) -> WalkerReceipt
where
    F: FnMut(u64, u64) -> Point,
{
    let mut attempts = 0;
    loop {
        let (a, b) = stream.next();
        attempts += 1;
        let point = multiply(a, b);
        assert_eq!(point, add(scalar_mul(g, a), scalar_mul(q, b)));
        if point == Point::Infinity {
            continue;
        }
        let (point, a, b) = canonicalize(point, a, b, order);
        return WalkerReceipt {
            point,
            a,
            b,
            attempts,
            discarded: attempts - 1,
            draws: stream.draws,
            cursor: stream.cursor,
        };
    }
}

fn main() {
    let (g, order) = choose_base();
    assert_eq!(scalar_mul(g, order), Point::Infinity);
    let target_scalar = 17 % order;
    assert_ne!(target_scalar, 0);
    let q = scalar_mul(g, target_scalar);
    assert!(on_curve(g) && on_curve(q));
    assert_eq!(scalar_mul(q, order), Point::Infinity);
    let table = window2_table(g, q);

    let bound = order.min(96);
    let mut checked = 0_u64;
    for a in 0..bound {
        for b in 0..bound {
            let reference = add(scalar_mul(g, a), scalar_mul(q, b));
            let candidate = joint_window2(&table, a, b);
            assert_eq!(
                candidate, reference,
                "joint multiplication mismatch at ({a},{b})"
            );
            checked += 1;
        }
    }

    // The first pair is constructed to give (order-target_scalar)G + Q = O.
    // The next pair is selected deterministically to be non-infinite.
    let forced_retry = ((order - target_scalar) % order, 1);
    assert_eq!(add(scalar_mul(g, forced_retry.0), q), Point::Infinity);
    let success = (1..order)
        .map(|a| (a, 1))
        .find(|&(a, b)| add(scalar_mul(g, a), scalar_mul(q, b)) != Point::Infinity)
        .expect("no successful retry coefficient pair");
    let pairs = vec![forced_retry, success, (2 % order, 1)];
    let mut reference_stream = PairStream {
        pairs: pairs.clone(),
        cursor: 0,
        draws: 0,
    };
    let mut candidate_stream = PairStream {
        pairs,
        cursor: 0,
        draws: 0,
    };
    let reference = construct_walker(&mut reference_stream, g, q, order, |a, b| {
        add(scalar_mul(g, a), scalar_mul(q, b))
    });
    let candidate = construct_walker(&mut candidate_stream, g, q, order, |a, b| {
        joint_window2(&table, a, b)
    });
    assert_eq!(candidate, reference);
    assert_eq!(candidate.attempts, 2);
    assert_eq!(candidate.discarded, 1);
    assert_eq!(candidate.draws, 4);
    assert_eq!(reference_stream.cursor, candidate_stream.cursor);
    assert_eq!(
        candidate.point,
        canonicalize(
            add(scalar_mul(g, candidate.a), scalar_mul(q, candidate.b)),
            candidate.a,
            candidate.b,
            order,
        )
        .0
    );

    println!(
        "R34-R50 PASS p={P} order={order} coefficient_pairs={checked} retry_attempts={} discarded={} draws={} cursor={}",
        candidate.attempts, candidate.discarded, candidate.draws, candidate.cursor
    );
}
