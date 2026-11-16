type E2 = [u64; 2];
fn ea(a: E2, b: E2) -> E2 {
    [add(a[0], b[0]), add(a[1], b[1])]
}
fn en(a: E2) -> E2 {
    [sub(0, a[0]), sub(0, a[1])]
}
fn em(a: E2, b: E2, r: [u64; 3]) -> E2 {
    let t = mul(a[1], b[1]);
    [
        sub(mul(a[0], b[0]), mul(t, r[0])),
        sub(add(mul(a[0], b[1]), mul(a[1], b[0])), mul(t, r[1])),
    ]
}
fn ep(mut a: E2, mut n: u64, r: [u64; 3]) -> E2 {
    let mut z = [1, 0];
    while n > 0 {
        if n & 1 != 0 {
            z = em(z, a, r)
        }
        a = em(a, a, r);
        n >>= 1;
    }
    z
}
fn ei(a: E2, r: [u64; 3]) -> E2 {
    let norm = add(
        sub(mul(a[0], a[0]), mul(mul(a[0], a[1]), r[1])),
        mul(mul(a[1], a[1]), r[0]),
    );
    let z = inv(norm);
    [mul(sub(a[0], mul(a[1], r[1])), z), mul(sub(0, a[1]), z)]
}
fn ev2(f: &Poly, x: E2, y: E2, r: [u64; 3], axis: Option<usize>) -> E2 {
    let mut z = [0, 0];
    for (i, j) in monos(6) {
        let mut e = [i, j];
        let mut c = f[7 * i + j];
        if let Some(k) = axis {
            if e[k] == 0 {
                continue;
            }
            c = mul(c, e[k] as u64);
            e[k] -= 1;
        }
        z = ea(
            z,
            em(
                [c, 0],
                em(ep(x, e[0] as u64, r), ep(y, e[1] as u64, r), r),
                r,
            ),
        );
    }
    z
}
fn base_divisor_control(v: &[Poly], h: &[Vec<u64>], ts: &[[usize; 3]]) {
    let cfg = crate::config::get();
    let r = cfg.residual;
    let [a, b] = cfg.tangent;
    let f = curve();
    let s = cfg
        .points
        .iter()
        .copied()
        .find(|p| {
            p[2] == 1
                && peval(&r, p[0]) != 0
                && partial(&f, p[0], p[1], 1) != 0
                && p[1] != add(mul(a, p[0]), b)
        })
        .expect("off-tangent base-divisor control point");
    let u0 = um(&r, &[sub(0, s[0]), 1]);
    let u: [u64; 4] = u0.try_into().unwrap();
    let c = mul(sub(s[1], add(mul(a, s[0]), b)), inv(peval(&r, s[0])));
    let vv = ua(&[b, a], &us(&r, c), false);
    let y: R = std::array::from_fn(|i| vv.get(i).copied().unwrap_or(0));
    assert!(eval_u(&f, &y, &u).is_empty());
    let (corrected_values, g) = corrected(v, u, y);
    assert_eq!(g, r, "full_degree2_base_cancelled");
    let disc = sub(mul(r[1], r[1]), mul(4, r[0]));
    let fp = hec_native_ic::fp::Fp::new(P);
    let (roots, kind) = if let Some(d) = fp.sqrt(disc) {
        (
            [
                [mul(add(sub(0, r[1]), d), inv(2)), 0],
                [mul(sub(sub(0, r[1]), d), inv(2)), 0],
            ],
            "split",
        )
    } else {
        let x = [0, 1];
        ([x, ep(x, P, r)], "quadratic")
    };
    assert_ne!(roots[0], roots[1]);
    let ps = [
        (roots[0], ea(em([a, 0], roots[0], r), [b, 0])),
        (roots[1], ea(em([a, 0], roots[1], r), [b, 0])),
        ([s[0], 0], [s[1], 0]),
    ];
    for (x, y) in ps {
        assert_eq!(ev2(&f, x, y, r, None), [0, 0]);
    }
    let vals: Vec<[E2; 3]> = v
        .iter()
        .map(|n| {
            std::array::from_fn(|k| {
                let (x, y) = ps[k];
                let raw = ev2(n, x, y, r, None);
                if k == 2 {
                    return em(raw, ei([peval(&g, s[0]), 0], r), r);
                }
                assert_eq!(raw, [0, 0]);
                let dy = en(em(
                    ev2(&f, x, y, r, Some(0)),
                    ei(ev2(&f, x, y, r, Some(1)), r),
                    r,
                ));
                let dn = ea(
                    ev2(n, x, y, r, Some(0)),
                    em(ev2(n, x, y, r, Some(1)), dy, r),
                );
                let dg = ea([r[1], 0], em([2, 0], x, r));
                em(dn, ei(dg, r), r)
            })
        })
        .collect();
    let mut tensors = vec![];
    for t in ts {
        let mut direct = [0, 0];
        for q in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            direct = ea(
                direct,
                em(
                    em(vals[t[q[0]]][0], vals[t[q[1]]][1], r),
                    vals[t[q[2]]][2],
                    r,
                ),
            );
        }
        assert_eq!(direct[1], 0, "symmetric_tensor_descends");
        let got = polar(
            corrected_values[t[0]],
            corrected_values[t[1]],
            corrected_values[t[2]],
            u,
        );
        assert_eq!(got, direct[0], "independent_extension_derivative_limit");
        tensors.push(got);
    }
    let coords: Vec<_> = h
        .iter()
        .map(|a| {
            a.iter()
                .zip(&tensors)
                .fold(0, |z, (a, b)| add(z, mul(*a, *b)))
        })
        .collect();
    assert!(coords.iter().any(|a| *a != 0));
    println!("BASE_DIVISOR_CONTROL kind={kind} degree=2 tensors=120 independent_extension_limits=true S={s:?}");
}
