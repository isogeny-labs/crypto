//! CSIDH-512 hybrid: Montgomery kernel arithmetic, Edwards w codomain formula.
//! Run: cargo run --release --example model_step_w_csidh512
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
    let mut a = f.div(f.mul(two, f.add(c, d)), f.sub(c, d));
    if !state.plus {
        a = f.neg(a);
    }
    WState {
        a,
        d,
        c,
        plus: state.plus,
    }
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
    println!("field,task,degree,arm,median_ns,verified");
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
}
