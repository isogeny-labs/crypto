include!("helpers.rs");
use std::sync::atomic::{AtomicU64, Ordering};
static TRANSFER_CALLS: AtomicU64 = AtomicU64::new(0);
static TRANSFER_LOOKUPS: AtomicU64 = AtomicU64::new(0);
fn key<const P: i64, const D: i64>(p: Pt<P, D>) -> Option<(i64, i64, i64, i64)> {
    p.map(|(x, y)| (x.0, x.1, y.0, y.1))
}
fn transfer<const P: i64, const D: i64>(
    g: Pt<P, D>,
    q: Pt<P, D>,
    u: Pt<P, D>,
    a: i64,
    r: u64,
) -> Pt<P, D> {
    let delta = add(g, neg(q), a);
    assert!(delta.is_some());
    let mut m = 1;
    while m * m < r {
        m += 1
    }
    let mut table = std::collections::HashMap::new();
    let mut dcur = None;
    let mut gcur = None;
    for _ in 0..m {
        assert!(table.insert(key(dcur), gcur).is_none());
        dcur = add(dcur, delta, a);
        gcur = add(gcur, g, a)
    }
    let mut probe = u;
    let mut label = None;
    TRANSFER_CALLS.fetch_add(1, Ordering::Relaxed);
    for _ in 0..=m {
        TRANSFER_LOOKUPS.fetch_add(1, Ordering::Relaxed);
        if let Some(&baby) = table.get(&key(probe)) {
            return add(label, baby, a);
        }
        probe = add(probe, neg(dcur), a);
        label = add(label, gcur, a);
    }
    panic!("public transfer search failed")
}
fn graph<const P: i64, const D: i64>(q: Pt<P, D>, a: i64, r: u64) -> Vec<Pt<P, D>> {
    let mut v = vec![];
    let mut cur = None;
    for _ in 0..r {
        v.push(cur);
        cur = add(cur, q, a)
    }
    assert!(cur.is_none());
    v
}
fn frob<const P: i64, const D: i64>(p: Pt<P, D>) -> Pt<P, D> {
    p.map(|(x, y)| (F::new(x.0, -x.1), F::new(y.0, -y.1)))
}
fn phi<const P: i64, const D: i64>(p: Pt<P, D>, a: i64) -> Pt<P, D> {
    add(p, neg(frob(p)), a)
}
fn coeff<const P: i64, const D: i64>(
    ds: &[F<P, D>],
    xs: &[F<P, D>],
    reference: bool,
) -> [F<P, D>; 3] {
    if !reference {
        let mut v = [F::c(1), F::c(0), F::c(0)];
        for (&d, &x) in ds.iter().zip(xs) {
            v = [v[0] * d, v[1] * d + v[0], v[2] * d + v[0] * x]
        }
        return v;
    }
    let n = ds.len();
    let mut pre = vec![F::c(1); n + 1];
    let mut post = pre.clone();
    for i in 0..n {
        pre[i + 1] = pre[i] * ds[i]
    }
    for i in (0..n).rev() {
        post[i] = post[i + 1] * ds[i]
    }
    let mut v = [pre[n], F::c(0), F::c(0)];
    for i in 0..n {
        let c = pre[i] * post[i + 1];
        v[1] = v[1] + c;
        v[2] = v[2] + c * xs[i]
    }
    v
}
fn decode<const P: i64, const D: i64>(
    x: F<P, D>,
    shift: Pt<P, D>,
    a: i64,
    b: i64,
    r: u64,
) -> Pt<P, D> {
    let rhs = x * x * x + F::c(a) * x + F::c(b);
    let mut candidates = vec![];
    for i in 0..P {
        for j in 0..P {
            let y = F::new(i, j);
            if y * y != rhs {
                continue;
            }
            let q = add(Some((x, y)), neg(shift), a);
            let rational = q.map(|(x, y)| x.1 == 0 && y.1 == 0).unwrap_or(true);
            if rational && mul(r, q, a).is_none() {
                candidates.push(q)
            }
        }
    }
    assert_eq!(candidates.len(), 1);
    candidates[0]
}
fn action<const P: i64, const D: i64>(
    gg: &[Pt<P, D>],
    qq: &[Pt<P, D>],
    shift: Pt<P, D>,
    u: Pt<P, D>,
    a: i64,
    b: i64,
    r: u64,
    reference: bool,
) -> (Pt<P, D>, [F<P, D>; 3]) {
    let rr = add(shift, u, a);
    let mut ds = vec![];
    let mut xs = vec![];
    for (&g, &q) in gg.iter().zip(qq) {
        let x = add(shift, g, a).unwrap().0;
        let y = add(rr, q, a).unwrap().0;
        ds.push(x - y);
        xs.push(x)
    }
    let v = coeff(&ds, &xs, reference);
    assert_eq!(v[0], F::c(0));
    assert_ne!(v[1], F::c(0));
    let out = decode(v[2] / v[1], shift, a, b, r);
    if std::env::args().any(|x| x == "--transfer-check") {
        assert_eq!(out, transfer(gg[1], qq[1], u, a, r));
    }
    (out, v)
}
