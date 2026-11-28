type R = [u64; 3];
fn ra(a: R, b: R) -> R {
    std::array::from_fn(|i| add(a[i], b[i]))
}
fn rm(a: R, b: R, u: [u64; 4]) -> R {
    let mut t = [0; 5];
    for i in 0..3 {
        for j in 0..3 {
            t[i + j] = add(t[i + j], mul(a[i], b[j]));
        }
    }
    for k in (3..=4).rev() {
        let c = t[k];
        for j in 0..=3 {
            t[k - 3 + j] = sub(t[k - 3 + j], mul(c, u[j]));
        }
    }
    [t[0], t[1], t[2]]
}
fn mat(a: R, u: [u64; 4]) -> [[u64; 3]; 3] {
    let c: [R; 3] = std::array::from_fn(|j| {
        let mut e = [0; 3];
        e[j] = 1;
        rm(a, e, u)
    });
    std::array::from_fn(|i| std::array::from_fn(|j| c[j][i]))
}
fn tr(a: R, u: [u64; 4]) -> u64 {
    let m = mat(a, u);
    add(add(m[0][0], m[1][1]), m[2][2])
}
fn norm(a: R, u: [u64; 4]) -> u64 {
    let m = mat(a, u);
    (0..3).fold(0, |z, i| {
        add(
            z,
            mul(
                m[0][i],
                sub(
                    mul(m[1][(i + 1) % 3], m[2][(i + 2) % 3]),
                    mul(m[1][(i + 2) % 3], m[2][(i + 1) % 3]),
                ),
            ),
        )
    })
}
fn polar(a: R, b: R, c: R, u: [u64; 4]) -> u64 {
    let mut z = mul(mul(tr(a, u), tr(b, u)), tr(c, u));
    for (t, d) in [(rm(a, b, u), c), (rm(a, c, u), b), (rm(b, c, u), a)] {
        z = sub(z, mul(tr(t, u), tr(d, u)));
    }
    add(z, mul(2, tr(rm(rm(a, b, u), c, u), u)))
}
fn pd(a: R, b: R, c: R, u: [u64; 4]) -> u64 {
    let mut z = norm(ra(ra(a, b), c), u);
    for t in [ra(a, b), ra(a, c), ra(b, c)] {
        z = sub(z, norm(t, u));
    }
    for t in [a, b, c] {
        z = add(z, norm(t, u));
    }
    z
}
fn rv(n: &Poly, v: R, u: [u64; 4]) -> R {
    let mut xp = [[0; 3]; 4];
    let mut yp = xp;
    xp[0] = [1, 0, 0];
    yp[0] = xp[0];
    for i in 1..4 {
        xp[i] = rm(xp[i - 1], [0, 1, 0], u);
        yp[i] = rm(yp[i - 1], v, u);
    }
    let mut z = [0; 3];
    for (i, j) in monos(3) {
        let t = rm(xp[i], yp[j], u);
        for k in 0..3 {
            z[k] = add(z[k], mul(n[7 * i + j], t[k]));
        }
    }
    z
}
fn graph(ps: &[(u64, u64); 3]) -> ([u64; 4], R) {
    let mut u = vec![1];
    for (x, _) in ps {
        let mut t = vec![0; u.len() + 1];
        for (k, c) in u.iter().enumerate() {
            t[k] = sub(t[k], mul(*c, *x));
            t[k + 1] = add(t[k + 1], *c);
        }
        u = t;
    }
    let mut v = [0; 3];
    for i in 0..3 {
        let mut t = vec![1];
        let mut den = 1;
        for j in 0..3 {
            if i == j {
                continue;
            }
            den = mul(den, sub(ps[i].0, ps[j].0));
            let mut z = vec![0; t.len() + 1];
            for (k, c) in t.iter().enumerate() {
                z[k] = sub(z[k], mul(*c, ps[j].0));
                z[k + 1] = add(z[k + 1], *c);
            }
            t = z;
        }
        for k in 0..3 {
            v[k] = add(v[k], mul(mul(ps[i].1, inv(den)), t[k]));
        }
    }
    ([u[0], u[1], u[2], u[3]], v)
}
fn trace_gate(v: &[Poly], h: &[Vec<u64>], ts: &[[usize; 3]]) {
    let c = (1..P)
        .find(|c| (0..P).all(|x| add(add(mul(mul(x, x), x), x), *c) != 0))
        .unwrap();
    let mut n = 0;
    for (index, u) in [
        [121, 11, 121, 1],
        [126, 3, 124, 1],
        [125, 5, 123, 1],
        [c, 1, 0, 1],
    ]
    .into_iter()
    .enumerate()
    {
        let a: Vec<_> = v.iter().map(|t| rv(t, [3, 5, 7], u)).collect();
        for t in ts {
            assert_eq!(
                polar(a[t[0]], a[t[1]], a[t[2]], u),
                pd(a[t[0]], a[t[1]], a[t[2]], u)
            );
            n += 1;
        }
        println!("ALGEBRA {index} u={u:?} checks=120 curve_divisor_claim=false");
    }
    let points = crate::config::get().controls.map(|p| (p[0], p[1]));
    let mut split = 0;
    for skip in 0..4 {
        let ps: Vec<_> = points
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != skip)
            .map(|(_, p)| *p)
            .collect();
        let ps: [(u64, u64); 3] = ps.try_into().unwrap();
        let (u, y) = graph(&ps);
        let (a, g) = corrected(v, u, y);
        println!("CANCELLATION skip={skip} common_factor={g:?}");
        let mut ev = Vec::new();
        for t in ts {
            let mut direct = 0;
            for perm in [
                [0, 1, 2],
                [0, 2, 1],
                [1, 0, 2],
                [1, 2, 0],
                [2, 0, 1],
                [2, 1, 0],
            ] {
                let z = (0..3).fold(1, |z, k| {
                    mul(z, point_section(&v[t[perm[k]]], ps[k].0, ps[k].1, &g))
                });
                direct = add(direct, z);
            }
            let z = polar(a[t[0]], a[t[1]], a[t[2]], u);
            assert_eq!(z, direct);
            ev.push(z);
            split += 1;
        }
        let cs: Vec<_> = h
            .iter()
            .map(|r| r.iter().zip(&ev).fold(0, |z, (c, x)| add(z, mul(*c, *x))))
            .collect();
        assert!(cs.iter().any(|x| *x != 0));
        println!("SPLIT_TRIPLE skip={skip} corrected_projective={cs:?}");
    }
    println!("TRACE_PASS determinant_checks={n} split_tensor_checks={split} base_divisor_cancelled=true repeated_support_map=false general_lift=false");
}
