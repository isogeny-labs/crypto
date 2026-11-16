// CT22: an abstract HLP (2,2)-gluing four-torsion module.
// No Jacobian divisor correspondence or particular Kohel degree-7 map is constructed.
// Usage: rustc --edition=2021 --test ct22_mod4_gluing.rs -o /tmp/ct22 && /tmp/ct22
// [pcc:unverified]
use std::collections::BTreeSet;

type V = [u8; 4];
const ZERO: V = [0, 0, 0, 0];

fn add(a: V, b: V) -> V {
    std::array::from_fn(|i| (a[i] + b[i]) % 4)
}
fn twice(a: V) -> V { add(a, a) }
fn all() -> Vec<V> {
    (0..256_usize).map(|mut n| {
        let mut v = ZERO;
        for x in &mut v { *x = (n % 4) as u8; n /= 4; }
        v
    }).collect()
}
fn graph() -> BTreeSet<V> {
    [ZERO, [2, 0, 2, 0], [0, 2, 0, 2], [2, 2, 2, 2]]
        .iter().copied().collect()
}
fn quotient_class(v: V, graph: &BTreeSet<V>) -> V {
    graph.iter().map(|&g| add(v, g)).min().expect("nonempty graph")
}
// The canonical lift of F_101 Frobenius on each E[4] has
// characteristic polynomial X^2-X+1 modulo 4:
// #E1(F101)=109 -> trace -7 == 1 (mod 4);
// #E2(F101)=113 -> trace -11 == 1 (mod 4).
fn frob(v: V) -> V {
    [3 * v[1] % 4, (v[0] + v[1]) % 4,
     3 * v[3] % 4, (v[2] + v[3]) % 4]
}
fn pairing(v: V, w: V) -> u8 {
    // On [2]^{-1}G the product e4 pairing is even; divide by 2
    // to obtain the induced alternating F2 pairing on [2]^{-1}G/G.
    let z = (v[0] as i32 * w[1] as i32 - v[1] as i32 * w[0] as i32
           + v[2] as i32 * w[3] as i32 - v[3] as i32 * w[2] as i32)
           .rem_euclid(4);
    assert_eq!(z % 2, 0, "pairing not even on quotient preimage");
    (z / 2) as u8
}
fn subgroup(x: V, y: V, g: &BTreeSet<V>) -> BTreeSet<V> {
    [quotient_class(ZERO, g), x, y, quotient_class(add(x, y), g)]
        .iter().copied().collect()
}
fn frob_stable(k: &[V], g: &BTreeSet<V>) -> bool {
    let image: BTreeSet<V> = k.iter().map(|&v| quotient_class(frob(v), g)).collect();
    let source: BTreeSet<V> = k.iter().copied().collect();
    image == source
}
// Illustrative unit B=I+F in Z/4[F] of determinant 3 == 7 mod 4.
// It is NOT the actual E1/E2 degree-7 Kohel isogeny.
fn example_alpha(v: V) -> V { add(v, frob(v)) }

#[test]
fn full_four_torsion_census_and_kernel_invariance() {
    let v = all();
    // On the actual prime-field E1/E2 four-torsion, t=-7/-11 and p=101
    // imply F^2-F+I=0 mod 4, hence F^3=-I. Primitive 4-torsion
    // points have full Frobenius orbit 6; their x-coordinate classes
    // modulo +/- have orbit 3. Thus x lives in F_(101^3), while the
    // full point (including y sign) first lives in F_(101^6).
    for &x in &v {
        let cube = frob(frob(frob(x)));
        let minus: V = std::array::from_fn(|i| (4 - x[i]) % 4);
        assert_eq!(cube, minus);
        assert_eq!(frob(frob(frob(cube))), x);
    }
    for u in 0..4_u8 { for w in 0..4_u8 {
        if u % 2 == 0 && w % 2 == 0 { continue; }
        let first: V = [u, w, 0, 0];
        let minus: V = std::array::from_fn(|i| (4 - first[i]) % 4);
        let mut current = first;
        let mut point_period = 0;
        loop {
            current = frob(current);
            point_period += 1;
            assert!(point_period <= 6);
            if current == first { break; }
        }
        assert_eq!(point_period, 6);
        current = first;
        let mut x_period = 0;
        loop {
            current = frob(current);
            x_period += 1;
            assert!(x_period <= 3);
            if current == first || current == minus { break; }
        }
        assert_eq!(x_period, 3);
    }}
    let g = graph();
    let s: Vec<V> = v.iter().copied().filter(|&x| g.contains(&twice(x))).collect();
    let q: BTreeSet<V> = s.iter().copied().map(|x| quotient_class(x, &g)).collect();
    assert_eq!((v.len(), g.len(), s.len(), q.len()), (256, 4, 64, 16));

    // The quotient symplectic form is well-defined, nondegenerate, and Frobenius
    // acts as an isometry. In particular it is independent of every representative.
    for &x in &q {
        assert_eq!(pairing(x, x), 0);
        for &y in &q {
            let b = pairing(x, y);
            for &a in &g { for &c in &g {
                assert_eq!(pairing(add(x, a), add(y, c)), b);
            }}
            assert_eq!(pairing(frob(x), frob(y)), b);
            assert_eq!(pairing(x, y), pairing(y, x));
        }
        assert!(q.contains(&quotient_class(frob(x), &g)));
    }
    for &x in &q {
        let orthogonal = q.iter().filter(|&&y| pairing(x, y) == 0).count();
        assert_eq!(orthogonal, if x == ZERO { 16 } else { 8 });
    }

    let members: Vec<V> = q.iter().copied().filter(|&x| x != ZERO).collect();
    let mut planes: BTreeSet<Vec<V>> = BTreeSet::new();
    for (i, &x) in members.iter().enumerate() {
        for &y in members.iter().skip(i + 1) {
            if pairing(x, y) != 0 { continue; }
            let k = subgroup(x, y, &g);
            if k.len() == 4 { planes.insert(k.iter().copied().collect()); }
        }
    }
    assert_eq!(planes.len(), 15);
    let stable: Vec<Vec<V>> = planes.into_iter()
        .filter(|k| frob_stable(k, &g)).collect();
    assert_eq!(stable.len(), 3);

    // N P=[2], so the opposite branch-point pairing is exactly
    // ker(N|J[2]) = A[2]/G, not an arbitrary chosen Lagrangian.
    let opposite: BTreeSet<V> = q.iter().copied().filter(|&x| twice(x) == ZERO).collect();
    assert_eq!(opposite.len(), 4);
    let mut intersections: Vec<usize> = stable.iter().map(|k| {
        k.iter().filter(|&&x| opposite.contains(&x)).count()
    }).collect();
    intersections.sort();
    assert_eq!(intersections, [1, 1, 4]);
    assert_eq!(q.iter().filter(|&&x| quotient_class(frob(x), &g) == x).count(), 1);

    // The explicit *illustrative* mod-4 unit commutes with Frobenius,
    // has determinant 3 on each elliptic factor and preserves the
    // quotient's F2 Weil form. It maps all 64 lifts bijectively.
    assert_eq!(v.iter().map(|&x| example_alpha(x)).collect::<BTreeSet<V>>().len(), 256);
    assert!(v.iter().all(|&x| example_alpha(frob(x)) == frob(example_alpha(x))));
    let g1: BTreeSet<V> = g.iter().map(|&x| example_alpha(x)).collect();
    assert_eq!(g1.len(), 4);
    let s1: BTreeSet<V> = s.iter().map(|&x| example_alpha(x)).collect();
    assert_eq!(s1.len(), 64);
    let q1: BTreeSet<V> = s1.iter().copied().map(|x| quotient_class(x, &g1)).collect();
    assert_eq!(q1.len(), 16);
    let map = |x: V| quotient_class(example_alpha(x), &g1);
    assert_eq!(q.iter().copied().map(map).collect::<BTreeSet<V>>().len(), 16);
    let k1: BTreeSet<V> = opposite.iter().copied().map(map).collect();
    let expected: BTreeSet<V> = q1.iter().copied().filter(|&x| twice(x) == ZERO).collect();
    assert_eq!(k1, expected);
    for &x in &q { for &y in &q {
        assert_eq!(pairing(map(x), map(y)), pairing(x, y));
        assert_eq!(map(quotient_class(add(x, y), &g)),
                   quotient_class(add(map(x), map(y)), &g1));
    }}
    for k in &stable {
        let image: BTreeSet<V> = k.iter().copied().map(map).collect();
        assert_eq!(image.len(), 4);
        // The two other stable Lagrangians cannot map to the split norm kernel.
        assert_eq!(image == k1, k.iter().copied().collect::<BTreeSet<V>>() == opposite);
    }

    // Deliberately rotate the target 2-torsion gluing by a Frobenius cycle:
    // the graph no longer agrees with alpha(G), despite both being stable.
    let wrong: BTreeSet<V> = [ZERO, [2, 0, 0, 2], [0, 2, 2, 2], [2, 2, 2, 0]]
        .iter().copied().collect();
    assert_eq!(wrong.len(), 4);
    assert!(wrong.iter().all(|&x| wrong.contains(&frob(x))));
    assert_eq!(wrong.intersection(&g1).count(), 1);
    assert_ne!(wrong, g1);
    println!("CT22 [pcc:unverified]: 256 E[4] points, 64 lifts, 16 J[2] classes");
    println!("15 Lagrangians, 3 Frobenius-stable, intersections with norm kernel [1,1,4]");
    println!("illustrative B=I+F (det 7 mod 4) passes; rotated target graph fails");
}
