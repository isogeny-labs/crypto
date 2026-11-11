#![allow(dead_code, unused_imports)]
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
fn constant<const P: i64, const D: i64>(gg: &[Pt<P, D>], qq: &[Pt<P, D>]) -> F<P, D> {
    gg.iter()
        .zip(qq)
        .skip(1)
        .fold(F::c(1), |v, (&g, &q)| v * (g.unwrap().0 - q.unwrap().0))
}
fn invmod(a: u64, r: u64) -> u64 {
    (1..r).find(|&b| a * b % r == 1).unwrap()
}
fn panel<const P: i64, const D: i64>(
    a: i64,
    b: i64,
    r: u64,
    g: Pt<P, D>,
    shift: Pt<P, D>,
    probe: Pt<P, D>,
    targets: &[u64],
    us: &[u64],
    reference: bool,
) -> Value {
    assert_ne!(mul(2, shift, a).unwrap().1 .1, 0);
    assert!((1..=10).all(|n| mul(n, probe, a).is_some()));
    let gg = graph(g, a, r);
    let count = 1
        + (0..P)
            .flat_map(|x| (0..P).map(move |y| (x, y)))
            .filter(|&(x, y)| (y * y - x * x * x - a * x - b).rem_euclid(P) == 0)
            .count();
    assert_eq!(count, r as usize);
    let mut inverse_actions = 0;
    let mut wrong_graph = 0;
    let mut basechecks = 0;
    let mut symmetries = 0;
    let mut bridges = 0;
    let mut ordinary_norm_calls = 0;
    let mut bridge_setup_calls = 0;
    let mut constants = vec![];
    let mut rows = vec![];
    for &k in targets {
        let q = mul(k, g, a);
        let qq = graph(q, a, r);
        let mut inverse_rows = vec![];
        let mut base_rows = vec![];
        if q != g && q != neg(g) {
            let c = constant(&gg, &qq);
            assert_ne!(c, F::c(0));
            assert_eq!(c.1, 0);
            assert_eq!(c, constant(&gg, &graph(neg(q), a, r)));
            assert_eq!(c, constant(&gg, &graph(mul(invmod(k, r), g, a), a, r)));
            symmetries += 2;
            constants.push(c);
            for (i, j) in [(1, 2), (1, 3), (2, 3)] {
                let pp = mul(i, probe, a);
                let rr = mul(j, probe, a);
                let mut prod = F::c(1);
                for (&gj, &qj) in gg.iter().zip(&qq) {
                    prod = prod * (add(pp, gj, a).unwrap().0 - add(rr, qj, a).unwrap().0)
                }
                let compact = phi(pp, a).unwrap().0 - phi(rr, a).unwrap().0;
                assert_ne!(compact, F::c(0));
                assert_eq!(prod, c * compact);
                basechecks += 1;
                base_rows.push(json!({"P_index":i,"R_index":j,"full_norm":prod.code(),"normalized_compact":compact.code()}));
            }
            let inverse_label = invmod((r + 1 - k) % r, r);
            for &uindex in us {
                let u = mul(uindex, g, a);
                let (out, co) = action(&gg, &qq, shift, u, a, b, r, reference);
                assert_eq!(out, mul(inverse_label, u, a));
                inverse_actions += 1;
                if u.is_some() {
                    let (cross, _) =
                        action(&gg, &graph(neg(q), a, r), shift, u, a, b, r, reference);
                    assert_ne!(out, cross);
                    wrong_graph += 1
                }
                inverse_rows.push(json!({"U_index":uindex,"U":pc(u),"coefficients":co.iter().map(|x|x.code()).collect::<Vec<_>>(),"output":pc(out)}));
            }
        }
        let special = if q == g {
            Some(1)
        } else if q == neg(g) {
            Some(r - 1)
        } else if add(q, q, a) == g {
            Some((r + 1) / 2)
        } else {
            None
        };
        let mut q2 = None;
        let mut qq2 = vec![];
        if special.is_none() {
            let (bg, _) = action(&gg, &qq, shift, g, a, b, r, reference);
            bridge_setup_calls += 1;
            q2 = add(g, neg(bg), a);
            assert!(q2.is_some() && q2 != g && q2 != neg(g));
            qq2 = graph(q2, a, r)
        }
        let mut bridge_rows = vec![];
        for &uindex in us {
            let u = mul(uindex, g, a);
            let out = if let Some(label) = special {
                mul(label, u, a)
            } else {
                let (v, _) = action(&gg, &qq2, shift, u, a, b, r, reference);
                ordinary_norm_calls += 1;
                add(u, neg(v), a)
            };
            assert_eq!(out, mul(k, u, a));
            bridges += 1;
            bridge_rows.push(json!({"U_index":uindex,"output":pc(out)}));
        }
        rows.push(json!({"Q_label":k,"Q":pc(q),"constant":if q!=g&&q!=neg(g){constant(&gg,&qq).code()}else{Value::Null},"base_probes":base_rows,"inverse_actions":inverse_rows,"ordinary_bridge":{"public_special_case":special,"derived_Q2":pc(q2),"rows":bridge_rows}}));
    }
    let mut unique = constants.clone();
    unique.sort_by_key(|x| (x.0, x.1));
    unique.dedup();
    json!({"p":P,"D":D,"r":r,"P_shift":pc(shift),"generic_probe":pc(probe),"inverse_affine_actions":inverse_actions,"wrong_second_graph_rejections":wrong_graph,"generic_base_norm_checks":basechecks,"constant_symmetry_checks":symmetries,"distinct_constants_in_declared_panel":unique.len(),"eligible_target_count":constants.len(),"ordinary_actions":bridges,"ordinary_norm_calls":ordinary_norm_calls,"bridge_setup_norm_calls":bridge_setup_calls,"rows":rows})
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let reference = args.iter().any(|x| x == "--cofactor-reference");
    let sa = Some((F::<11, 2>::c(0), F::new(0, 3)));
    let sb = Some((F::<17, 3>::c(1), F::new(0, 8)));
    let rt = (1..211).find(|x| x * x % 211 == 204).unwrap();
    let sc = Some((F::<211, 210>::c(0), F::new(0, rt)));
    let mut probe = None;
    for x in 0..211 {
        let negf = (-x * x * x - 7i64).rem_euclid(211);
        if let Some(y) = (1..211).find(|y| y * y % 211 == negf) {
            let s = Some((F::c(x), F::new(0, y)));
            if (1..=10).all(|n| mul(n, s, 0).is_some()) {
                probe = s;
                break;
            }
        }
    }
    assert!(probe.is_some());
    let a = panel::<11, 2>(
        2,
        7,
        7,
        Some((F::c(6), F::c(2))),
        mul(6, sa, 2),
        sa,
        &(1..7).collect::<Vec<_>>(),
        &(0..7).collect::<Vec<_>>(),
        reference,
    );
    let b = panel::<17, 3>(
        2,
        2,
        19,
        Some((F::c(5), F::c(1))),
        mul(6, sb, 2),
        sb,
        &(1..19).collect::<Vec<_>>(),
        &(0..19).collect::<Vec<_>>(),
        reference,
    );
    let c = panel::<211, 210>(
        0,
        7,
        199,
        Some((F::c(3), F::c(33))),
        sc,
        probe,
        &[1, 2, 3, 5, 7, 11, 17, 31, 63, 97, 128, 198],
        &[0, 1, 2, 3, 7, 31, 98, 101, 198],
        reference,
    );
    let out = json!({"all_checks_pass":true,"controls":[a,b,c],"scope":"full small-curve cross norms, inverse-affine actions, and ordinary-action bridge; unsigned"});
    if let Some(i) = args.iter().position(|x| x == "--check") {
        let saved: Value = serde_json::from_slice(&fs::read(&args[i + 1]).unwrap()).unwrap();
        assert!(out == saved, "certificate mismatch");
        eprintln!("cofactor-reference outputs match")
    }
    if args.iter().any(|x| x == "--transfer-check") {
        eprintln!(
            "{}",
            json!({"public_transfer_checks":TRANSFER_CALLS.load(Ordering::Relaxed),"giant_lookups":TRANSFER_LOOKUPS.load(Ordering::Relaxed),"uses_scalar_labels":false})
        );
    }
    println!("{}", serde_json::to_string(&out).unwrap());
}
