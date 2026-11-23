use std::cell::Cell;
mod meter;
mod tracked_set;
use tracked_set::TrackedSet;
#[derive(Clone,Copy,Debug)]
struct Divisor { degree: usize, u: [u64;3], v: [u64;2] }
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ArithmeticCounts {
    base_adds: u64,
    base_muls: u64,
    base_inversions: u64,
    scalar_adds: u64,
    scalar_muls: u64,
    scalar_inversions: u64,
}
impl ArithmeticCounts {
    fn difference(self, before: Self) -> Self {
        Self {
            base_adds: self.base_adds - before.base_adds,
            base_muls: self.base_muls - before.base_muls,
            base_inversions: self.base_inversions - before.base_inversions,
            scalar_adds: self.scalar_adds - before.scalar_adds,
            scalar_muls: self.scalar_muls - before.scalar_muls,
            scalar_inversions: self.scalar_inversions - before.scalar_inversions,
        }
    }
    fn add_assign(&mut self, other: Self) {
        self.base_adds += other.base_adds;
        self.base_muls += other.base_muls;
        self.base_inversions += other.base_inversions;
        self.scalar_adds += other.scalar_adds;
        self.scalar_muls += other.scalar_muls;
        self.scalar_inversions += other.scalar_inversions;
    }
}
thread_local! {
    static PRICE_ACTIVE: Cell<bool> = const { Cell::new(false) };
    static PRICE_COUNTS: Cell<ArithmeticCounts> = Cell::new(ArithmeticCounts::default());
    static BASE_MODULUS: Cell<u64> = const { Cell::new(739) };
    static SCALAR_MODULUS: Cell<u64> = const { Cell::new(34147) };
}
fn charge(modulus: u64, action: impl FnOnce(&mut ArithmeticCounts)) {
    PRICE_ACTIVE.with(|active| {
        if active.get() {
            PRICE_COUNTS.with(|counts| {
                let mut c = counts.get();
                assert!(
                    BASE_MODULUS.with(Cell::get) == modulus
                        || SCALAR_MODULUS.with(Cell::get) == modulus
                );
                action(&mut c);
                counts.set(c);
            });
        }
    });
}
fn arithmetic_counts() -> ArithmeticCounts {
    PRICE_COUNTS.with(Cell::get)
}
fn reset_arithmetic_counts() {
    PRICE_COUNTS.with(|c| c.set(ArithmeticCounts::default()));
}

fn trim(mut a: Vec<u64>) -> Vec<u64> {
    while {
        meter::collection(usize::from(!a.is_empty()), 0, 0, 1);
        a.last() == Some(&0)
    } {
        a.pop();
    }
    a
}
fn add(a: u64, b: u64, p: u64) -> u64 {
    charge(p, |c| {
        if p == BASE_MODULUS.with(Cell::get) {
            c.base_adds += 1
        } else {
            c.scalar_adds += 1
        }
    });
    (a + b) % p
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    charge(p, |c| {
        if p == BASE_MODULUS.with(Cell::get) {
            c.base_adds += 1
        } else {
            c.scalar_adds += 1
        }
    });
    (a + p - b) % p
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    charge(p, |c| {
        if p == BASE_MODULUS.with(Cell::get) {
            c.base_muls += 1
        } else {
            c.scalar_muls += 1
        }
    });
    ((a as u128 * b as u128) % p as u128) as u64
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    assert_eq!(n, p - 2);
    charge(p, |c| {
        if p == BASE_MODULUS.with(Cell::get) {
            c.base_inversions += 1
        } else {
            c.scalar_inversions += 1
        }
    });
    let mut out = 1;
    while n > 0 {
        if n & 1 != 0 {
            out = mul(out, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    out
}
fn pmul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let mut out = vec![0; a.len() + b.len() - 1];
    meter::collection(0, 0, out.len(), 0);
    for (i, &x) in a.iter().enumerate() {
        meter::collection(1, 0, 0, 0);
        for (j, &y) in b.iter().enumerate() {
            meter::collection(2, 0, 0, 0);
            out[i + j] = add(out[i + j], mul(x, y, p), p);
        }
    }
    trim(out)
}
fn peval(a: &[u64], x: u64, p: u64) -> u64 {
    a.iter().rev().fold(0, |v, &c| {
        meter::collection(1, 0, 0, 0);
        add(mul(v, x, p), c, p)
    })
}
fn divide_monic(mut a: Vec<u64>, b: &[u64], p: u64) -> (Vec<u64>, Vec<u64>) {
    meter::collection(usize::from(!b.is_empty()), 0, 0, 1);
    assert_eq!(b.last(), Some(&1));
    let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
    meter::collection(0, 0, q.len(), 0);
    while {
        meter::collection(0, 0, 0, 1);
        a.len() >= b.len()
    } {
        let shift = a.len() - b.len();
        meter::collection(1, 1, 0, 0); // last coefficient read and copied to q
        let c = *a.last().unwrap();
        q[shift] = c;
        for (i, &bi) in b.iter().enumerate() {
            meter::collection(2, 0, 0, 0);
            a[i + shift] = sub(a[i + shift], mul(c, bi, p), p);
        }
        a = trim(a);
    }
    (trim(q), a)
}
fn complement(d: Divisor, lambda: u64, h: &[u64], p: u64) -> (Vec<u64>, Vec<u64>) {
    let u = &d.u[..=d.degree];
    let mut w = vec![0; u.len()];
    meter::collection(0, 0, w.len(), 0);
    for i in 0..u.len() {
        meter::collection(1 + usize::from(i < d.degree), 0, 0, 1);
        let vi = if i < d.degree { d.v[i] } else { 0 };
        w[i] = add(vi, mul(lambda, u[i], p), p);
    }
    let w2 = pmul(&w, &w, p);
    let numerator = trim(
        (0..h.len())
            .map(|i| {
                meter::collection(1 + usize::from(i < w2.len()), 0, 1, 0);
                sub(*w2.get(i).unwrap_or(&0), h[i], p)
            })
            .collect(),
    );
    let (quotient, remainder) = divide_monic(numerator, u, p);
    meter::collection(0, 0, 0, 1);
    assert!(remainder.is_empty());
    (quotient, w)
}
fn rational_y_by_x(h: &[u64], p: u64) -> Vec<Vec<u64>> {
    let mut square_roots = vec![Vec::new(); p as usize];
    meter::collection(0, 0, p as usize, 0);
    for y in 0..p {
        square_roots[mul(y, y, p) as usize].push(y);
        meter::collection(0, 0, 1, 0);
    }
    (0..p)
        .map(|x| {
            let roots = &square_roots[peval(h, x, p) as usize];
            meter::collection(1, roots.len(), 1, 0);
            roots.clone()
        })
        .collect()
}
type SieveResult = (
    TrackedSet,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    Vec<Vec<u64>>,
);
fn point_sieve_candidates(d: Divisor, h: &[u64], p: u64, rational_y: &[Vec<u64>]) -> SieveResult {
    let mut buckets = vec![Vec::<u64>::new(); p as usize];
    meter::collection(0, 0, p as usize, 0);
    let mut point_probes = 0;
    let mut denominators = Vec::new();
    let mut exceptional_denominators = Vec::new();
    let mut exceptional_points = Vec::new();
    let mut all_lambda_points = Vec::new();
    let u = &d.u[..=d.degree];
    let v = &d.v[..d.degree];
    let (q0, _) = complement(d, 0, h, p);
    let mut exceptional = TrackedSet::new();
    for x in 0..p {
        let ux = peval(u, x, p);
        let vx = peval(v, x, p);
        meter::collection(0, 0, 0, 1);
        if ux == 0 {
            // q_lambda = q_0 + 2 lambda v + lambda^2 u.  At a root
            // of u, this is linear in lambda even for a double root.
            let qx = peval(&q0, x, p);
            let slope = mul(2, vx, p);
            meter::collection(0, 0, 0, 1 + usize::from(slope == 0));
            if slope != 0 {
                exceptional_denominators.push((x, qx, slope));
                meter::collection(0, 0, 1, 0);
            } else if qx == 0 {
                exceptional.extend(0..p);
                all_lambda_points.push(x);
                meter::collection(0, 0, 1, 0);
            }
            continue;
        }
        meter::collection(1, 0, 0, 1);
        if rational_y[x as usize].is_empty() {
            continue;
        }
        denominators.push((x, ux, vx));
        meter::collection(0, 0, 1, 0);
    }
    let ordinary_count = denominators.len();
    let all_denominators: Vec<u64> = denominators
        .iter()
        .map(|&(_, ux, _)| {
            meter::collection(1, 1, 0, 0);
            ux
        })
        .chain(exceptional_denominators.iter().map(|&(_, _, slope)| {
            meter::collection(1, 1, 0, 0);
            slope
        }))
        .collect();
    let mut prefix = vec![1; all_denominators.len() + 1];
    meter::collection(0, 0, prefix.len(), 0);
    let mut batch_multiplies = 0;
    for (i, &d) in all_denominators.iter().enumerate() {
        meter::collection(2, 0, 0, 0);
        prefix[i + 1] = mul(prefix[i], d, p);
        batch_multiplies += 1;
    }
    meter::collection(0, 0, 0, 2);
    let batch_inversions = usize::from(!all_denominators.is_empty());
    let mut suffix = if all_denominators.is_empty() {
        1
    } else {
        meter::collection(1, 0, 0, 0);
        pow(*prefix.last().unwrap(), p - 2, p)
    };
    for i in (0..all_denominators.len()).rev() {
        meter::collection(3, 0, 0, 2); // denominator, prefix, selected tuple; inverse assertion and branch
        let d_i = all_denominators[i];
        let inv = mul(prefix[i], suffix, p);
        suffix = mul(suffix, d_i, p);
        batch_multiplies += 2;
        assert_eq!(mul(inv, d_i, p), 1);
        if i >= ordinary_count {
            let (x, qx, _) = exceptional_denominators[i - ordinary_count];
            let lambda = mul(sub(0, qx, p), inv, p);
            exceptional.insert(lambda);
            exceptional_points.push((lambda, x));
            meter::collection(0, 0, 1, 0);
            continue;
        }
        let (x, ux, vx) = denominators[i];
        meter::collection(1, 0, 0, 1); // selected rational-y vector and inverse assertion
        assert_eq!(mul(inv, ux, p), 1);
        for &y in &rational_y[x as usize] {
            meter::collection(1, 0, 0, 0);
            point_probes += 1;
            let lambda = mul(sub(y, vx, p), inv, p);
            buckets[lambda as usize].push(x);
            meter::collection(0, 0, 1, 0);
        }
    }
    let collision_candidates = buckets
        .iter()
        .filter(|b| {
            meter::collection(1, 0, 0, 1);
            b.len() >= 3
        })
        .count();
    let exceptional_count = exceptional.len();
    let mut candidates = exceptional;
    for (lambda, bucket) in buckets.iter().enumerate() {
        meter::collection(1, 0, 0, 1);
        if bucket.len() >= 3 {
            meter::collection(0, 0, 0, 1); // duplicate cardinality assertion
            assert_eq!(
                bucket
                    .iter()
                    .map(|&x| {
                        meter::collection(1, 1, 0, 0);
                        x
                    })
                    .collect::<TrackedSet>()
                    .len(),
                bucket.len()
            );
            candidates.insert(lambda as u64);
        }
    }
    for (lambda, x) in exceptional_points {
        meter::collection(1, 0, 0, 0);
        buckets[lambda as usize].push(x);
        meter::collection(0, 0, 1, 0);
    }
    for x in all_lambda_points {
        meter::collection(1, 0, 0, 0);
        for bucket in &mut buckets {
            meter::collection(1, 0, 0, 0);
            bucket.push(x);
            meter::collection(0, 0, 1, 0);
        }
    }
    (
        candidates,
        ordinary_count,
        exceptional_denominators.len(),
        batch_inversions,
        batch_multiplies,
        point_probes,
        collision_candidates,
        exceptional_count,
        buckets,
    )
}
fn point_sieve_candidates_finite_difference(d: Divisor, h: &[u64], p: u64, rational_y: &[Vec<u64>]) -> SieveResult {
    let mut buckets = vec![Vec::<u64>::new(); p as usize];
    meter::collection(0, 0, p as usize, 0);
    let mut point_probes = 0;
    let mut denominators = Vec::new();
    let mut exceptional_denominators = Vec::new();
    let mut exceptional_points = Vec::new();
    let mut all_lambda_points = Vec::new();
    let u = &d.u[..=d.degree];
    let v = &d.v[..d.degree];
    let (q0, _) = complement(d, 0, h, p);
    let mut exceptional = TrackedSet::new();
    // Candidate preconditions: degree <= 2, canonical active coefficients,
    // 0 < p <= 2^63. Inactive array tails are deliberately ignored.
    let u0 = u[0];
    let u1 = u.get(1).copied().unwrap_or(0);
    let u2 = u.get(2).copied().unwrap_or(0);
    let v0 = v.first().copied().unwrap_or(0);
    let v1 = v.get(1).copied().unwrap_or(0);
    meter::collection(u.len() + v.len(), 0, 0, 0);
    let mut u_value = u0;
    let mut u_delta = add(u1, u2, p);
    let u_delta2 = add(u2, u2, p);
    let mut v_value = v0;
    for x in 0..p {
        let ux = u_value;
        let vx = v_value;
        // Advance before either continue; branch bodies retain old ux/vx.
        u_value = add(u_value, u_delta, p);
        u_delta = add(u_delta, u_delta2, p);
        v_value = add(v_value, v1, p);
        meter::collection(0, 0, 0, 1);
        if ux == 0 {
            // q_lambda = q_0 + 2 lambda v + lambda^2 u.  At a root
            // of u, this is linear in lambda even for a double root.
            let qx = peval(&q0, x, p);
            let slope = mul(2, vx, p);
            meter::collection(0, 0, 0, 1 + usize::from(slope == 0));
            if slope != 0 {
                exceptional_denominators.push((x, qx, slope));
                meter::collection(0, 0, 1, 0);
            } else if qx == 0 {
                exceptional.extend(0..p);
                all_lambda_points.push(x);
                meter::collection(0, 0, 1, 0);
            }
            continue;
        }
        meter::collection(1, 0, 0, 1);
        if rational_y[x as usize].is_empty() {
            continue;
        }
        denominators.push((x, ux, vx));
        meter::collection(0, 0, 1, 0);
    }
    let ordinary_count = denominators.len();
    let all_denominators: Vec<u64> = denominators
        .iter()
        .map(|&(_, ux, _)| {
            meter::collection(1, 1, 0, 0);
            ux
        })
        .chain(exceptional_denominators.iter().map(|&(_, _, slope)| {
            meter::collection(1, 1, 0, 0);
            slope
        }))
        .collect();
    let mut prefix = vec![1; all_denominators.len() + 1];
    meter::collection(0, 0, prefix.len(), 0);
    let mut batch_multiplies = 0;
    for (i, &d) in all_denominators.iter().enumerate() {
        meter::collection(2, 0, 0, 0);
        prefix[i + 1] = mul(prefix[i], d, p);
        batch_multiplies += 1;
    }
    meter::collection(0, 0, 0, 2);
    let batch_inversions = usize::from(!all_denominators.is_empty());
    let mut suffix = if all_denominators.is_empty() {
        1
    } else {
        meter::collection(1, 0, 0, 0);
        pow(*prefix.last().unwrap(), p - 2, p)
    };
    for i in (0..all_denominators.len()).rev() {
        meter::collection(3, 0, 0, 2); // denominator, prefix, selected tuple; inverse assertion and branch
        let d_i = all_denominators[i];
        let inv = mul(prefix[i], suffix, p);
        suffix = mul(suffix, d_i, p);
        batch_multiplies += 2;
        assert_eq!(mul(inv, d_i, p), 1);
        if i >= ordinary_count {
            let (x, qx, _) = exceptional_denominators[i - ordinary_count];
            let lambda = mul(sub(0, qx, p), inv, p);
            exceptional.insert(lambda);
            exceptional_points.push((lambda, x));
            meter::collection(0, 0, 1, 0);
            continue;
        }
        let (x, ux, vx) = denominators[i];
        meter::collection(1, 0, 0, 1); // selected rational-y vector and inverse assertion
        assert_eq!(mul(inv, ux, p), 1);
        for &y in &rational_y[x as usize] {
            meter::collection(1, 0, 0, 0);
            point_probes += 1;
            let lambda = mul(sub(y, vx, p), inv, p);
            buckets[lambda as usize].push(x);
            meter::collection(0, 0, 1, 0);
        }
    }
    let collision_candidates = buckets
        .iter()
        .filter(|b| {
            meter::collection(1, 0, 0, 1);
            b.len() >= 3
        })
        .count();
    let exceptional_count = exceptional.len();
    let mut candidates = exceptional;
    for (lambda, bucket) in buckets.iter().enumerate() {
        meter::collection(1, 0, 0, 1);
        if bucket.len() >= 3 {
            meter::collection(0, 0, 0, 1); // duplicate cardinality assertion
            assert_eq!(
                bucket
                    .iter()
                    .map(|&x| {
                        meter::collection(1, 1, 0, 0);
                        x
                    })
                    .collect::<TrackedSet>()
                    .len(),
                bucket.len()
            );
            candidates.insert(lambda as u64);
        }
    }
    for (lambda, x) in exceptional_points {
        meter::collection(1, 0, 0, 0);
        buckets[lambda as usize].push(x);
        meter::collection(0, 0, 1, 0);
    }
    for x in all_lambda_points {
        meter::collection(1, 0, 0, 0);
        for bucket in &mut buckets {
            meter::collection(1, 0, 0, 0);
            bucket.push(x);
            meter::collection(0, 0, 1, 0);
        }
    }
    (
        candidates,
        ordinary_count,
        exceptional_denominators.len(),
        batch_inversions,
        batch_multiplies,
        point_probes,
        collision_candidates,
        exceptional_count,
        buckets,
    )
}
