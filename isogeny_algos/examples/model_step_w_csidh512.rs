//! CSIDH-512 hybrid: Montgomery kernel arithmetic, Edwards w codomain formula.
//! Run: cargo run --release --example model_step_w_csidh512
use isogeny_algos::bigint::Big;
use isogeny_algos::field::{Field, Rng};
use isogeny_algos::fpm::FpM;
use isogeny_algos::kernel::montgomery::{self, XZ};
use isogeny_algos::path::csidh::Csidh;
use std::hint::black_box;
use std::time::Instant;

fn rhs<F: Field>(f: &F, a: F::E, x: F::E) -> F::E {
    f.mul(x, f.add(f.add(f.sq(x), f.mul(a, x)), f.one()))
}

fn sample_kernel<F: Field>(cs: &Csidh<F>, a: F::E, ell: u64, rng: &mut Rng) -> XZ<F::E> {
    let f = &cs.fp;
    let cof = cs.p1.divrem_small(ell).0;
    loop {
        let x = f.random(rng);
        if f.is_zero(rhs(f, a, x)) || f.sqrt(rhs(f, a, x)).is_none() {
            continue;
        }
        let k = montgomery::ladder_xz(f, montgomery::a24(f, a), (x, f.one()), &cof);
        if !f.is_zero(k.1) {
            return k;
        }
    }
}

/// w=d*x_E^2*y_E^2 as homogeneous W:Z, for the Edwards model over Fp.
/// The branch uses the rational 4-torsion normalization from Moriya et al., Cor. 2.
fn w_from_mont<F: Field>(f: &F, a: F::E, p: XZ<F::E>, plus: bool) -> XZ<F::E> {
    let (x, z) = p;
    let two = f.from_u64(2);
    let xpz = f.add(x, z);
    let xmz = f.sub(x, z);
    let (d, top, bottom) = if plus {
        (f.sub(a, two), xmz, xpz)
    } else {
        (f.add(a, two), xpz, xmz)
    };
    let w = f.mul(d, f.mul(f.mul(x, z), f.sq(top)));
    let z_out = f.mul(
        f.add(f.add(f.sq(x), f.mul(a, f.mul(x, z))), f.sq(z)),
        f.sq(bottom),
    );
    (w, z_out)
}

fn w_double<F: Field>(f: &F, d: F::E, p: XZ<F::E>) -> XZ<F::E> {
    let (w, z) = p;
    let four = f.from_u64(4);
    let sum2 = f.sq(f.add(w, z));
    let diff2 = f.sq(f.sub(w, z));
    let wz = f.mul(w, z);
    (
        f.mul(f.mul(four, wz), f.sub(f.mul(d, sum2), f.mul(four, wz))),
        f.mul(f.mul(d, sum2), diff2),
    )
}

/// A CSIDH curve represented by its canonical Montgomery coefficient and a
/// projective ordinary Edwards coefficient D:C.  Building this state costs a
/// square-character check; subsequent odd-degree steps retain the branch.
#[derive(Clone, Copy)]
struct WState<E> {
    a: E,
    d: E,
    c: E,
    plus: bool,
}

fn prepare_w<F: Field>(f: &F, a: F::E) -> WState<F::E> {
    let two = f.from_u64(2);
    let ap = f.add(a, two);
    let am = f.sub(a, two);
    let plus = f.sqrt(am).is_none();
    let (d, c) = if plus { (am, ap) } else { (ap, am) };
    WState { a, d, c, plus }
}

fn base_w<F: Field>(f: &F) -> WState<F::E> {
    let two = f.from_u64(2);
    WState {
        a: f.zero(),
        d: two,
        c: f.neg(two),
        plus: false,
    }
}

fn w_double_projective<F: Field>(f: &F, d: F::E, c: F::E, p: XZ<F::E>) -> XZ<F::E> {
    let (w, z) = p;
    let four = f.from_u64(4);
    let sum2 = f.sq(f.add(w, z));
    let diff2 = f.sq(f.sub(w, z));
    let wz4 = f.mul(four, f.mul(w, z));
    let ds = f.mul(d, sum2);
    (f.mul(wz4, f.sub(ds, f.mul(c, wz4))), f.mul(ds, diff2))
}

fn w_multiples_projective<F: Field>(
    f: &F,
    d: F::E,
    c: F::E,
    k: XZ<F::E>,
    count: usize,
) -> Vec<XZ<F::E>> {
    if count == 0 {
        return vec![];
    }
    let mut out = vec![k];
    if count == 1 {
        return out;
    }
    out.push(w_double_projective(f, d, c, k));
    while out.len() < count {
        let n = out.len();
        out.push(w_add(f, out[n - 1], k, out[n - 2]));
    }
    out
}

fn w_projective_successor<F: Field>(
    cs: &Csidh<F>,
    state: WState<F::E>,
    ell: u64,
    k: XZ<F::E>,
) -> WState<F::E> {
    let f = &cs.fp;
    let wk = w_from_mont(f, state.a, k, state.plus);
    w_successor_from_w(cs, state, ell, wk)
}

fn w_successor_from_w<F: Field>(
    cs: &Csidh<F>,
    state: WState<F::E>,
    ell: u64,
    wk: XZ<F::E>,
) -> WState<F::E> {
    let mut next = w_successor_projective_only(cs, state, ell, wk);
    next.a = recover_montgomery(&cs.fp, next);
    next
}

fn recover_montgomery<F: Field>(f: &F, state: WState<F::E>) -> F::E {
    let mut a = f.div(
        f.mul(f.from_u64(2), f.add(state.c, state.d)),
        f.sub(state.c, state.d),
    );
    if !state.plus {
        a = f.neg(a);
    }
    a
}

fn w_successor_projective_only<F: Field>(
    cs: &Csidh<F>,
    state: WState<F::E>,
    ell: u64,
    wk: XZ<F::E>,
) -> WState<F::E> {
    let f = &cs.fp;
    let ws = w_multiples_projective(f, state.d, state.c, wk, ((ell - 1) / 2) as usize);
    let two = f.from_u64(2);
    let mut num = f.one();
    let mut den = f.one();
    for &(w, z) in &ws {
        num = f.mul(num, f.add(w, z));
        den = f.mul(den, f.mul(two, z));
    }
    let d = f.mul(f.pow(state.d, ell as u128), f.sq(f.sq(f.sq(num))));
    let c = f.mul(f.pow(state.c, ell as u128), f.sq(f.sq(f.sq(den))));
    WState {
        a: state.a,
        d,
        c,
        plus: state.plus,
    }
}

fn w_ladder<F: Field>(f: &F, state: WState<F::E>, p: XZ<F::E>, k: &Big) -> XZ<F::E> {
    if k.is_zero() {
        return (f.zero(), f.one());
    }
    let mut r0 = p;
    let mut r1 = w_double_projective(f, state.d, state.c, p);
    for i in (0..k.bits() - 1).rev() {
        if k.bit(i) {
            r0 = w_add(f, r0, r1, p);
            r1 = w_double_projective(f, state.d, state.c, r1);
        } else {
            r1 = w_add(f, r0, r1, p);
            r0 = w_double_projective(f, state.d, state.c, r0);
        }
    }
    r0
}

/// Moriya–Onuki–Takagi Algorithm 3, restricted to the positive eigenspace.
fn sample_w_kernel<F: Field>(
    cs: &Csidh<F>,
    state: WState<F::E>,
    ell: u64,
    positive: bool,
    rng: &mut Rng,
) -> XZ<F::E> {
    let f = &cs.fp;
    let half = f.q().sub_small(1).shr(1);
    let cof = cs.p1.divrem_small(4 * ell).0;
    loop {
        let r = f.random(rng);
        let w = f.sq(r);
        if w == f.zero() || w == f.one() || w == f.neg(f.one()) {
            continue;
        }
        let doubled = w_double_projective(f, state.d, state.c, (w, f.one()));
        let character_arg = f.mul(doubled.0, doubled.1);
        if f.is_zero(character_arg) || (f.pow_big(character_arg, &half) == f.one()) == positive {
            continue;
        }
        let point = if positive {
            (doubled.1, doubled.0)
        } else {
            doubled
        };
        let k = w_ladder(f, state, point, &cof);
        if !f.is_zero(k.0) && !f.is_zero(k.1) {
            return k;
        }
    }
}

fn native_w_step<F: Field>(
    cs: &Csidh<F>,
    state: WState<F::E>,
    ell: u64,
    positive: bool,
    rng: &mut Rng,
) -> WState<F::E> {
    let k = sample_w_kernel(cs, state, ell, positive, rng);
    w_successor_from_w(cs, state, ell, k)
}

fn native_w_action<F: Field>(cs: &Csidh<F>, exps: &[i32], seed: u64) -> F::E {
    let mut state = base_w(&cs.fp);
    let mut rng = Rng::new(seed);
    for (i, &e) in exps.iter().enumerate() {
        for _ in 0..e.unsigned_abs() {
            let k = sample_w_kernel(cs, state, cs.primes[i], e > 0, &mut rng);
            state = w_successor_projective_only(cs, state, cs.primes[i], k);
        }
    }
    recover_montgomery(&cs.fp, state)
}

/// Equation (14) of Moriya–Onuki–Takagi, sharing the kernel multiples with a
/// codomain update in a future fused implementation.
fn w_isog_eval<F: Field>(
    f: &F,
    d: F::E,
    c: F::E,
    kernel: XZ<F::E>,
    ell: u64,
    p: XZ<F::E>,
) -> XZ<F::E> {
    let ws = w_multiples_projective(f, d, c, kernel, ((ell - 1) / 2) as usize);
    let mut num = f.one();
    let mut den = f.one();
    for &(wi, zi) in &ws {
        num = f.mul(num, f.sub(f.mul(p.1, wi), f.mul(zi, p.0)));
        den = f.mul(den, f.sub(f.mul(p.0, wi), f.mul(p.1, zi)));
    }
    (f.mul(p.0, f.sq(num)), f.mul(p.1, f.sq(den)))
}

fn sample_w_batch<F: Field>(
    cs: &Csidh<F>,
    state: WState<F::E>,
    product: &Big,
    positive: bool,
    half: &Big,
    rng: &mut Rng,
) -> XZ<F::E> {
    let f = &cs.fp;
    let divisor = product.mul_small(4);
    let (cof, rem) = cs.p1.divrem(&divisor);
    assert!(rem.is_zero());
    loop {
        let r = f.random(rng);
        let w = f.sq(r);
        if w == f.zero() || w == f.one() || w == f.neg(f.one()) {
            continue;
        }
        let doubled = w_double_projective(f, state.d, state.c, (w, f.one()));
        let character_arg = f.mul(doubled.0, doubled.1);
        if f.is_zero(character_arg) || (f.pow_big(character_arg, half) == f.one()) == positive {
            continue;
        }
        let point = if positive {
            (doubled.1, doubled.0)
        } else {
            doubled
        };
        let point = w_ladder(f, state, point, &cof);
        if !f.is_zero(point.0) && !f.is_zero(point.1) {
            return point;
        }
    }
}

/// Native batch action: a sampled w point serves every prime component it has.
/// Missing components stay pending for the next sample. Canonicalize only once.
fn native_w_action_batched<F: Field>(cs: &Csidh<F>, exps: &[i32], seed: u64) -> F::E {
    let f = &cs.fp;
    let half = f.q().sub_small(1).shr(1);
    let mut state = base_w(f);
    let mut rng = Rng::new(seed);
    let mut pending = exps.to_vec();
    for positive in [true, false] {
        let mut rounds = 0;
        while pending.iter().any(|&e| e != 0 && (e > 0) == positive) {
            rounds += 1;
            assert!(rounds < 256, "batch sampling stalled");
            let indices: Vec<_> = pending
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| (e != 0 && (e > 0) == positive).then_some(i))
                .collect();
            let mut product = Big::from_u64(1);
            for &i in &indices {
                product = product.mul_small(cs.primes[i]);
            }
            let mut point = sample_w_batch(cs, state, &product, positive, &half, &mut rng);
            for &i in &indices {
                let ell = cs.primes[i];
                product = product.divrem_small(ell).0;
                let kernel = w_ladder(f, state, point, &product);
                if f.is_zero(kernel.0) || f.is_zero(kernel.1) {
                    continue;
                }
                let image = w_isog_eval(f, state.d, state.c, kernel, ell, point);
                state = w_successor_projective_only(cs, state, ell, kernel);
                point = image;
                pending[i] -= if positive { 1 } else { -1 };
            }
        }
    }
    recover_montgomery(f, state)
}

fn w_tree<F: Field>(
    cs: &Csidh<F>,
    state: &mut WState<F::E>,
    q: XZ<F::E>,
    indices: &[usize],
    pending: &mut [i32],
    sign: i32,
    stack: &mut Vec<XZ<F::E>>,
    split: &[Vec<usize>],
    offset: usize,
) {
    let f = &cs.fp;
    if f.is_zero(q.0) || f.is_zero(q.1) {
        return;
    }
    if indices.len() == 1 {
        let i = indices[0];
        let ell = cs.primes[i];
        let ws = w_multiples_projective(f, state.d, state.c, q, ((ell - 1) / 2) as usize);
        let mut num = f.one();
        let mut den = f.one();
        let two = f.from_u64(2);
        for &(w, z) in &ws {
            num = f.mul(num, f.add(w, z));
            den = f.mul(den, f.mul(two, z));
        }
        for p in stack.iter_mut() {
            let (w, z) = *p;
            let mut pn = f.one();
            let mut pd = f.one();
            for &(wi, zi) in &ws {
                pn = f.mul(pn, f.sub(f.mul(z, wi), f.mul(zi, w)));
                pd = f.mul(pd, f.sub(f.mul(w, wi), f.mul(z, zi)));
            }
            *p = (f.mul(w, f.sq(pn)), f.mul(z, f.sq(pd)));
        }
        state.d = f.mul(f.pow(state.d, ell as u128), f.sq(f.sq(f.sq(num))));
        state.c = f.mul(f.pow(state.c, ell as u128), f.sq(f.sq(f.sq(den))));
        pending[i] -= sign;
        return;
    }
    let mid = split[offset][offset + indices.len() - 1] - offset + 1;
    let mut right_product = Big::from_u64(1);
    for &i in &indices[mid..] {
        right_product = right_product.mul_small(cs.primes[i]);
    }
    let left = w_ladder(f, *state, q, &right_product);
    stack.push(q);
    w_tree(
        cs,
        state,
        left,
        &indices[..mid],
        pending,
        sign,
        stack,
        split,
        offset,
    );
    let right = stack.pop().expect("stacked w point");
    w_tree(
        cs,
        state,
        right,
        &indices[mid..],
        pending,
        sign,
        stack,
        split,
        offset + mid,
    );
}

fn w_plan<F: Field>(cs: &Csidh<F>, indices: &[usize]) -> Vec<Vec<usize>> {
    let n = indices.len();
    let mut split = vec![vec![0usize; n]; n];
    let mut cost = vec![vec![0.0f64; n]; n];
    let mut log_prefix = vec![0.0f64; n + 1];
    let mut push_prefix = vec![0.0f64; n + 1];
    for (j, &i) in indices.iter().enumerate() {
        let ell = cs.primes[i] as f64;
        let half = ((cs.primes[i] - 1) / 2) as f64;
        log_prefix[j + 1] = log_prefix[j] + ell.log2();
        push_prefix[j + 1] = push_prefix[j] + 4.0 * half + 4.0;
        cost[j][j] = 8.0 * half + 3.0 * ell.log2() + 10.0;
    }
    for len in 2..=n {
        for lo in 0..=n - len {
            let hi = lo + len - 1;
            let mut best = f64::INFINITY;
            for mid in lo..hi {
                let candidate = 12.0 * (log_prefix[hi + 1] - log_prefix[mid + 1])
                    + cost[lo][mid]
                    + push_prefix[mid + 1]
                    - push_prefix[lo]
                    + cost[mid + 1][hi];
                if candidate < best {
                    best = candidate;
                    split[lo][hi] = mid;
                }
            }
            cost[lo][hi] = best;
        }
    }
    split
}

fn native_w_action_tree<F: Field>(cs: &Csidh<F>, exps: &[i32], seed: u64) -> F::E {
    let f = &cs.fp;
    let half = f.q().sub_small(1).shr(1);
    let mut state = base_w(f);
    let mut rng = Rng::new(seed);
    let mut pending = exps.to_vec();
    for positive in [true, false] {
        let mut rounds = 0;
        while pending.iter().any(|&e| e != 0 && (e > 0) == positive) {
            rounds += 1;
            assert!(rounds < 256, "tree sampling stalled");
            let indices: Vec<_> = pending
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| (e != 0 && (e > 0) == positive).then_some(i))
                .collect();
            let mut product = Big::from_u64(1);
            for &i in &indices {
                product = product.mul_small(cs.primes[i]);
            }
            let point = sample_w_batch(cs, state, &product, positive, &half, &mut rng);
            let mut stack = Vec::new();
            let split = w_plan(cs, &indices);
            w_tree(
                cs,
                &mut state,
                point,
                &indices,
                &mut pending,
                if positive { 1 } else { -1 },
                &mut stack,
                &split,
                0,
            );
        }
    }
    recover_montgomery(f, state)
}

fn w_add<F: Field>(f: &F, p: XZ<F::E>, q: XZ<F::E>, diff: XZ<F::E>) -> XZ<F::E> {
    (
        f.mul(diff.1, f.sq(f.sub(f.mul(p.0, q.1), f.mul(q.0, p.1)))),
        f.mul(diff.0, f.sq(f.sub(f.mul(p.0, q.0), f.mul(p.1, q.1)))),
    )
}

fn w_multiples<F: Field>(f: &F, d: F::E, k: XZ<F::E>, count: usize) -> Vec<XZ<F::E>> {
    if count == 0 {
        return vec![];
    }
    let mut out = vec![k];
    if count == 1 {
        return out;
    }
    out.push(w_double(f, d, k));
    while out.len() < count {
        let n = out.len();
        out.push(w_add(f, out[n - 1], k, out[n - 2]));
    }
    out
}

fn w_codomain<F: Field>(cs: &Csidh<F>, a: F::E, ell: u64, ws: &[XZ<F::E>], plus: bool) -> F::E {
    let f = &cs.fp;
    let two = f.from_u64(2);
    let ap = f.add(a, two);
    let am = f.sub(a, two);
    let d = if plus { f.div(am, ap) } else { f.div(ap, am) };
    let mut num = f.one();
    let mut den = f.one();
    for &(w, z) in ws {
        num = f.mul(num, f.add(w, z));
        den = f.mul(den, f.mul(two, z));
    }
    let t = f.div(num, den);
    let t8 = f.sq(f.sq(f.sq(t)));
    let d_new = f.mul(f.pow(d, ell as u128), t8);
    let mut a_new = f.div(f.mul(two, f.add(f.one(), d_new)), f.sub(f.one(), d_new));
    if f.sqrt(f.sub(f.one(), d_new)).is_none() {
        a_new = f.neg(a_new);
    }
    a_new
}

fn w_successor<F: Field>(cs: &Csidh<F>, a: F::E, ell: u64, ms: &[XZ<F::E>]) -> F::E {
    let f = &cs.fp;
    let plus = f.sqrt(f.sub(a, f.from_u64(2))).is_none();
    let ws: Vec<_> = ms.iter().map(|&p| w_from_mont(f, a, p, plus)).collect();
    w_codomain(cs, a, ell, &ws, plus)
}

fn w_native_successor<F: Field>(cs: &Csidh<F>, a: F::E, ell: u64, k: XZ<F::E>) -> F::E {
    let f = &cs.fp;
    let two = f.from_u64(2);
    let ap = f.add(a, two);
    let am = f.sub(a, two);
    let plus = f.sqrt(am).is_none();
    let d = if plus { f.div(am, ap) } else { f.div(ap, am) };
    let wk = w_from_mont(f, a, k, plus);
    let ws = w_multiples(f, d, wk, ((ell - 1) / 2) as usize);
    w_codomain(cs, a, ell, &ws, plus)
}

fn compare<F: Field>(cs: &Csidh<F>, a: F::E, ell: u64, k: XZ<F::E>) {
    let f = &cs.fp;
    let ms = montgomery::multiples(f, montgomery::a24(f, a), k, ((ell - 1) / 2) as usize);
    let expected = montgomery::velu_codomain(f, a, &ms, ell);
    let actual = w_successor(cs, a, ell, &ms);
    assert_eq!(actual, expected, "different successor for degree {ell}");
    let native = w_native_successor(cs, a, ell, k);
    assert_eq!(native, expected, "w arithmetic differs for degree {ell}");
    let state = prepare_w(f, a);
    let projective = w_projective_successor(cs, state, ell, k);
    assert_eq!(
        projective.a, expected,
        "projective w differs for degree {ell}"
    );
    assert_eq!(
        prepare_w(f, expected).plus,
        projective.plus,
        "Edwards branch changed"
    );
}

fn sparse_action<F: Field>(cs: &Csidh<F>, seed: u64, edwards: bool) -> F::E {
    let f = &cs.fp;
    let mut a = f.zero();
    let mut state = edwards.then(|| base_w(f));
    let mut rng = Rng::new(seed);
    for &ell in &[3u64, 5, 7, 31] {
        let k = sample_kernel(cs, a, ell, &mut rng);
        if edwards {
            let next = w_projective_successor(cs, state.expect("Edwards state"), ell, k);
            a = next.a;
            state = Some(next);
        } else {
            let ms = montgomery::multiples(f, montgomery::a24(f, a), k, ((ell - 1) / 2) as usize);
            a = montgomery::velu_codomain(f, a, &ms, ell);
        }
    }
    a
}

fn median_ns(mut op: impl FnMut(), n: usize) -> u128 {
    let mut times = Vec::with_capacity(n);
    for _ in 0..n {
        let t = Instant::now();
        op();
        times.push(t.elapsed().as_nanos());
    }
    times.sort_unstable();
    times[n / 2]
}

fn main() {
    let cs = Csidh::<FpM<8>>::csidh512();
    let f = &cs.fp;
    println!("field,task,degree,arm,median_ns,exact_output_checked");
    for &ell in &[3u64, 5, 7, 31, 587] {
        let a = f.zero();
        let k = sample_kernel(&cs, a, ell, &mut Rng::new(9000 + ell));
        compare(&cs, a, ell, k);
        let seed_kernel = sample_kernel(&cs, a, 3, &mut Rng::new(8181));
        let seed_ms = montgomery::multiples(f, montgomery::a24(f, a), seed_kernel, 1);
        let a_next = montgomery::velu_codomain(f, a, &seed_ms, 3);
        let k_next = sample_kernel(&cs, a_next, ell, &mut Rng::new(19_000 + ell));
        compare(&cs, a_next, ell, k_next);
        let ms = montgomery::multiples(f, montgomery::a24(f, a), k, ((ell - 1) / 2) as usize);
        let state = prepare_w(f, a);
        for (name, arm) in [
            ("montgomery", 0),
            ("edwards_w_hybrid", 1),
            ("edwards_w_native", 2),
            ("montgomery", 3),
            ("edwards_w_projective", 4),
            ("edwards_w_projective_cold", 5),
        ] {
            let ns = median_ns(
                || {
                    let value = match arm {
                        0 => montgomery::velu_codomain(f, a, black_box(&ms), ell),
                        1 => w_successor(&cs, a, ell, black_box(&ms)),
                        2 => w_native_successor(&cs, a, ell, black_box(k)),
                        3 => {
                            let points = montgomery::multiples(
                                f,
                                montgomery::a24(f, a),
                                black_box(k),
                                ((ell - 1) / 2) as usize,
                            );
                            montgomery::velu_codomain(f, a, &points, ell)
                        }
                        4 => w_projective_successor(&cs, black_box(state), ell, black_box(k)).a,
                        _ => {
                            w_projective_successor(
                                &cs,
                                prepare_w(f, black_box(a)),
                                ell,
                                black_box(k),
                            )
                            .a
                        }
                    };
                    black_box(value);
                },
                21,
            );
            let task = if arm >= 2 {
                "known_kernel_point_to_codomain"
            } else {
                "codomain_given_kernel_multiples"
            };
            println!("CSIDH-512,{task},{ell},{name},{ns},true");
        }
    }
    for seed in 44_221..44_253 {
        let mont = sparse_action(&cs, seed, false);
        let edwards = sparse_action(&cs, seed, true);
        assert_eq!(mont, edwards, "sparse action differs for seed {seed}");
    }
    for (name, edwards) in [("montgomery", false), ("edwards_w_projective", true)] {
        let ns = median_ns(
            || {
                black_box(sparse_action(&cs, 44_221, edwards));
            },
            21,
        );
        println!("CSIDH-512,sparse_positive_action,3_5_7_31,{name},{ns},true");
    }
    for &ell in &[3u64, 5, 7, 31, 587] {
        let index = cs.primes.iter().position(|&v| v == ell).unwrap();
        for positive in [true, false] {
            let reference = cs.step(f.zero(), index, positive, &mut Rng::new(81_000 + ell));
            let native = native_w_step(&cs, base_w(f), ell, positive, &mut Rng::new(82_000 + ell));
            assert_eq!(
                reference, native.a,
                "native step mismatch for {ell} {positive}"
            );
            let label = if positive { "positive" } else { "negative" };
            for (arm, w) in [("montgomery_step", false), ("native_edwards_w", true)] {
                let ns = median_ns(
                    || {
                        let result = if w {
                            native_w_step(
                                &cs,
                                base_w(f),
                                ell,
                                positive,
                                &mut Rng::new(83_000 + ell),
                            )
                            .a
                        } else {
                            cs.step(f.zero(), index, positive, &mut Rng::new(83_000 + ell))
                        };
                        black_box(result);
                    },
                    7,
                );
                println!("CSIDH-512,full_step_{label},{ell},{arm},{ns},true");
            }
            let mut times = [Vec::new(), Vec::new()];
            for trial in 0..33u64 {
                let seed = 200_000 + 1_000 * ell + trial;
                let mut outputs = [None, None];
                for pass in 0..2 {
                    let arm = (pass + trial as usize) % 2;
                    let start = Instant::now();
                    let result = if arm == 0 {
                        cs.step(f.zero(), index, positive, &mut Rng::new(seed))
                    } else {
                        native_w_step(&cs, base_w(f), ell, positive, &mut Rng::new(seed)).a
                    };
                    black_box(result);
                    outputs[arm] = Some(result);
                    times[arm].push(start.elapsed().as_nanos());
                }
                assert_eq!(
                    outputs[0], outputs[1],
                    "seed sweep step mismatch for {ell} {positive} {seed}"
                );
            }
            for (arm, name) in [(0, "montgomery_step"), (1, "native_edwards_w")] {
                times[arm].sort_unstable();
                println!(
                    "CSIDH-512,seed_sweep_step_{label},{ell},{name},{},true",
                    times[arm][16]
                );
            }
        }
    }
    let mut cases = vec![
        ("small_mixed", vec![(0, 1), (1, -1), (2, 1), (9, -1)]),
        ("large_mixed", vec![(0, 1), (cs.primes.len() - 1, -1)]),
    ];
    cases.push((
        "all_74_mixed",
        (0..cs.primes.len())
            .map(|i| (i, if i % 2 == 0 { 1 } else { -1 }))
            .collect(),
    ));
    for (label, entries) in cases {
        let mut exps = vec![0i32; cs.primes.len()];
        for (i, e) in entries {
            exps[i] = e;
        }
        let reference = cs.action_fast(f.zero(), &exps, &mut Rng::new(123_001));
        for seed in [123_001, 123_002, 123_003] {
            let actual = native_w_action(&cs, &exps, seed);
            assert_eq!(
                actual, reference,
                "native w action mismatch: {label}, seed {seed}"
            );
            let batched = native_w_action_batched(&cs, &exps, seed);
            assert_eq!(
                batched, reference,
                "batched w action mismatch: {label}, seed {seed}"
            );
            let tree = native_w_action_tree(&cs, &exps, seed);
            assert_eq!(
                tree, reference,
                "tree w action mismatch: {label}, seed {seed}"
            );
        }
        for (arm, mode) in [
            ("montgomery_action_fast", 0),
            ("native_edwards_w", 1),
            ("native_edwards_w_batched", 2),
            ("native_edwards_w_tree", 3),
        ] {
            let ns = median_ns(
                || {
                    let result = match mode {
                        0 => cs.action_fast(f.zero(), &exps, &mut Rng::new(123_010)),
                        1 => native_w_action(&cs, &exps, 123_010),
                        2 => native_w_action_batched(&cs, &exps, 123_010),
                        _ => native_w_action_tree(&cs, &exps, 123_010),
                    };
                    black_box(result);
                },
                5,
            );
            println!("CSIDH-512,complete_action,{label},{arm},{ns},true");
        }
    }
}
