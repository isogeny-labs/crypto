use hec_native_ic::{
    counted,
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    oracle::{split, OracleStats},
    poly::Poly,
};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone)]
struct Case {
    a: Div,
    t: u64,
    known: Vec<u64>,
}
fn reference(c: &Curve, z: &Case) -> Poly {
    let f = &c.fp;
    let w = z.a.v.add(&z.a.u.scale(z.t, f), f);
    let mut q = c.f.sub(&w.mul(&w, f), f).div_exact(&z.a.u, f);
    for &x in &z.known {
        q = q.div_exact(&Poly::x_minus(x, f), f);
    }
    q
}
fn last(c: &Curve, z: &Case) -> u64 {
    assert_eq!(z.known.len(), c.g);
    let f = &c.fp;
    let mut x = f.sub(f.add(f.mul(z.t, z.t), z.a.u.c[c.g - 1]), c.f.c[2 * c.g]);
    for &v in &z.known {
        x = f.sub(x, v);
    }
    x
}
fn main() {
    counted::disable();
    let (mut total, mut pair_total, mut deficient_total) = (0, 0, 0);
    for (g, p, cc) in [(2, 101, 2), (3, 113, 3), (4, 109, 1)] {
        let mut coeff = vec![0; 2 * g + 2];
        coeff[0] = cc;
        coeff[2 * g + 1] = 1;
        let c = Curve::new(Fp::new(p), Poly::from_coeffs(&coeff));
        let f = &c.fp;
        assert_eq!(Poly::gcd(&c.f, &c.f.deriv(f), f), Poly::one());
        let fb = factor_base(&c);
        let &(x, y) = fb.iter().find(|p| p.0 != 0).unwrap();
        let base = c.place(x, y);
        let (
            mut anchor,
            mut eligible,
            mut pairbuckets,
            mut deficient,
            mut skipped,
            mut rawpairs,
            mut overlaps,
        ) = (c.zero(), vec![], 0, 0, 0, 0, 0);
        for _ in 0..32 {
            anchor = c.add(&anchor, &base);
            if anchor.u.deg() != g as isize {
                skipped += 1;
                continue;
            }
            let mut buckets = BTreeMap::<u64, BTreeSet<(u64, u64)>>::new();
            for &(x, y) in &fb {
                let u = anchor.u.eval(x, f);
                if u == 0 {
                    continue;
                }
                let iv = f.inv(u);
                let v = anchor.v.eval(x, f);
                for yy in if y == 0 { vec![y] } else { vec![y, f.neg(y)] } {
                    let t = f.mul(f.sub(yy, v), iv);
                    buckets.entry(t).or_default().insert((x, yy));
                }
            }
            for (t, pts) in buckets {
                let xs: Vec<_> = pts.iter().map(|p| p.0).collect();
                if xs.len() < 2 {
                    continue;
                }
                assert_eq!(xs.iter().copied().collect::<BTreeSet<_>>().len(), xs.len());
                assert!(xs.len() <= g + 1);
                pairbuckets += 1;
                rawpairs += xs.len() * (xs.len() - 1) / 2;
                let w = anchor.v.add(&anchor.u.scale(t, f), f);
                let h = c.f.sub(&w.mul(&w, f), f).div_exact(&anchor.u, f);
                // All pairs reconstruct the same full h; no multiplicity or rank from pair count.
                for i in 0..xs.len() {
                    for j in i + 1..xs.len() {
                        let pair = Poly::x_minus(xs[i], f).mul(&Poly::x_minus(xs[j], f), f);
                        let residual = h.div_exact(&pair, f);
                        assert_eq!(pair.mul(&residual, f), h);
                    }
                }
                if xs.len() < g {
                    deficient += 1;
                    continue;
                }
                let z = Case {
                    a: anchor,
                    t,
                    known: xs[..g].to_vec(),
                };
                let q = reference(&c, &z);
                assert_eq!(q.deg(), 1);
                let x = last(&c, &z);
                assert_eq!(q, Poly::x_minus(x, f));
                let roots = split(&q, f, &mut OracleStats::default()).unwrap();
                assert_eq!(roots.as_slice(), &[(x, 1)]);
                let mut group = anchor;
                for &v in z.known.iter().chain(std::iter::once(&x)) {
                    let yy = w.eval(v, f);
                    assert_eq!(f.mul(yy, yy), c.f.eval(v, f));
                    group = c.add_place(&group, v, yy);
                }
                assert!(c.is_zero(&group));
                if z.known.contains(&x) {
                    overlaps += 1
                }
                eligible.push(z);
            }
        }
        assert!(!eligible.is_empty());
        if g >= 3 {
            assert!(deficient > 0)
        }
        let mut first = None;
        for rep in 0..2 {
            let mut sig = vec![];
            let mut old = None;
            for arm in 0..2 {
                counted::reset();
                let outputs: Vec<_> = eligible
                    .iter()
                    .map(|z| {
                        if arm == 0 {
                            let q = reference(&c, z);
                            let r = split(&q, f, &mut OracleStats::default()).unwrap();
                            r.as_slice()[0].0
                        } else {
                            last(&c, z)
                        }
                    })
                    .collect();
                let counts = counted::snapshot();
                counted::disable();
                if let Some(ref v) = old {
                    assert_eq!(v, &outputs)
                } else {
                    old = Some(outputs)
                }
                for (m, v) in &counts {
                    assert_eq!(*m, p);
                    assert_eq!(v[8], 2 * v[7] + v[5] + v[6]);
                    println!(
                        "COST\tg={g}\tp={p}\trep={rep}\tarm={arm}\tcases={}\tcounts={v:?}",
                        eligible.len()
                    );
                }
                sig.push(counts);
            }
            if let Some(ref v) = first {
                assert_eq!(v, &sig)
            } else {
                first = Some(sig)
            }
        }
        println!("COVERAGE\tg={g}\tp={p}\tpair_buckets={pairbuckets}\tpair_choices={rawpairs}\teligible={}\tinsufficient_known_roots={deficient}\tskipped_anchors={skipped}\tlast_root_repeated_known={overlaps}",eligible.len());
        total += eligible.len();
        pair_total += pairbuckets;
        deficient_total += deficient;
    }
    eprintln!("PASS genera=2,3,4 eligible={total} pair_buckets={pair_total} insufficient_known_roots={deficient_total} full_identity_and_native_checks=true identical_repeat_vectors=true complete_solver=false independent_verification=false");
}
