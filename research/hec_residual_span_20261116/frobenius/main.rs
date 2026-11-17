use hec_native_ic::{
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    poly::Poly,
};
fn rational_roots(mut p: Poly, f: &Fp) -> (Vec<(u64, usize)>, Poly) {
    let mut roots = vec![];
    for x in 0..f.p {
        let mut m = 0;
        while p.deg() > 0 && p.eval(x, f) == 0 {
            p = p.div_exact(&Poly::x_minus(x, f), f);
            m += 1;
        }
        if m > 0 {
            roots.push((x, m));
        }
    }
    (roots, p)
}
fn irreducible(p: &Poly, f: &Fp) -> bool {
    let n = p.deg() as usize;
    let mut xp = Poly::x();
    for i in 1..=n {
        xp = xp.powmod(f.p, p, f);
        if i <= n / 2 && Poly::gcd(p, &xp.sub(&Poly::x(), f), f).deg() > 0 {
            return false;
        }
    }
    xp == Poly::x()
}
fn group_sum(c: &Curve, fb: &[(u64, u64)], row: &[i64]) -> Div {
    let mut d = c.zero();
    for (p, &s) in fb.iter().zip(row) {
        if s != 0 {
            let z = c.mul(&c.place(p.0, p.1), s.unsigned_abs());
            d = c.add(&d, &if s < 0 { c.neg(&z) } else { z });
        }
    }
    d
}
fn norm_row(c: &Curve, fb: &[(u64, u64)], h: &Poly, roots: &[(u64, usize)]) -> Vec<i64> {
    let mut row = vec![0; fb.len()];
    for &(x, m) in roots {
        let j = fb.iter().position(|p| p.0 == x).unwrap();
        let y = h.eval(x, &c.fp);
        assert!(y == fb[j].1 || y == c.fp.neg(fb[j].1));
        row[j] += if y == fb[j].1 { m as i64 } else { -(m as i64) };
    }
    row
}
fn automorphism(c: &Curve, d: &Div, z: u64) -> Div {
    let n = d.u.deg() as u64;
    let inverse = c.fp.inv(z);
    let mut u = Poly::zero();
    let mut v = Poly::zero();
    for i in 0..d.u.n {
        u.c[i] = c.fp.mul(d.u.c[i], c.fp.pow(z, n - i as u64));
    }
    u.n = d.u.n;
    for i in 0..d.v.n {
        v.c[i] = c.fp.mul(d.v.c[i], c.fp.pow(inverse, i as u64));
    }
    v.n = d.v.n;
    let out = Div { u, v };
    assert!(c.is_valid(&out));
    out
}
fn character_data(c: &Curve) -> (u64, Vec<i64>) {
    let n = 2 * c.g + 1;
    assert_eq!(n, 9);
    assert_eq!(c.f.deg(), n as isize);
    assert_eq!(c.f.lc(), 1);
    assert!(c.f.c[1..n].iter().all(|&a| a == 0));
    assert_eq!((c.fp.p - 1) % n as u64, 0);
    let z = (2..c.fp.p)
        .find(|&z| c.fp.pow(z, 9) == 1 && c.fp.pow(z, 3) != 1)
        .expect("field primitive ninth root");
    let mut powers = vec![];
    let mut value = 1;
    for _ in 0..n {
        powers.push(value);
        value = c.fp.mul(value, z)
    }
    assert_eq!(value, 1);
    let mut coeff = vec![0i64; n];
    for t in 1..c.fp.p {
        let chi = c.fp.pow(t, (c.fp.p - 1) / n as u64);
        let j = powers.iter().position(|&v| v == chi).unwrap();
        coeff[j] -= c.fp.legendre(c.fp.add(t, c.f.c[0])) as i64;
    }
    (z, coeff)
}
fn eigenvalue(coeff: &[i64]) -> (u64, Poly, Poly, Poly) {
    let f = Fp::new(8623);
    let p = Poly::from_coeffs(&coeff.iter().map(|&a| f.from_i64(a)).collect::<Vec<_>>());
    let mut q = Poly::zero();
    q.n = 10;
    q.c[0] = f.neg(1);
    q.c[9] = 1;
    let b = p.sub(&Poly::one(), &f);
    let (g, s, t) = Poly::xgcd(&q, &b, &f);
    assert_eq!(s.mul(&q, &f).add(&t.mul(&b, &f), &f), g);
    assert_eq!(g.deg(), 1, "unique rational subgroup eigenvalue required");
    assert_eq!(g.lc(), 1);
    let lambda = f.neg(g.c[0]);
    assert_eq!(f.pow(lambda, 9), 1);
    assert_eq!(p.eval(lambda, &f), 1);
    (lambda, q, b, g)
}
fn character_logs(
    c: &Curve,
    fb: &[(u64, u64)],
    _source: &Div,
) -> (Vec<u64>, usize, usize, usize, u64) {
    let f = Fp::new(8623);
    let (z, coeff) = character_data(c);
    let (lambda, _, _, _) = eigenvalue(&coeff);
    let mut logs = vec![None; fb.len()];
    let mut x = fb[0].0;
    let y = fb[0].1;
    let mut value = 1;
    for _ in 0..9 {
        let j = fb
            .iter()
            .position(|p| p.0 == x)
            .expect("catalogue orbit coverage");
        assert!(logs[j].is_none());
        let signed = if fb[j].1 == y {
            value
        } else {
            assert_eq!(fb[j].1, c.fp.neg(y));
            f.neg(value)
        };
        logs[j] = Some(signed);
        x = c.fp.mul(x, z);
        value = f.mul(value, lambda);
    }
    assert_eq!(x, fb[0].0);
    assert_eq!(value, 1);
    let logs = logs
        .into_iter()
        .map(|v| v.expect("full signed orbit required"))
        .collect();
    (logs, 0, (c.fp.p - 1) as usize, 1, 0)
}
fn qualify(c: &Curve, fb: &[(u64, u64)], source: &Div, logs: &[u64]) {
    let (z, coeff) = character_data(c);
    let (lambda, _, _, g) = eigenvalue(&coeff);
    // Independent original-group checks of the full character-sum operator, not just its eigenvalue.
    for &(x, y) in fb {
        let point = c.place(x, y);
        let mut current = point;
        let mut sum = c.zero();
        for &a in &coeff {
            let term = c.mul(&current, a.unsigned_abs());
            sum = c.add(&sum, &if a < 0 { c.neg(&term) } else { term });
            current = automorphism(c, &current, z);
        }
        assert_eq!(sum, point);
    }
    let image = automorphism(c, source, z);
    assert_eq!(c.mul(source, lambda), image);
    for (j, &log) in logs.iter().enumerate() {
        assert_eq!(c.mul(source, log), c.mul(&c.place(fb[j].0, fb[j].1), 13));
    }
    let mut reversed = vec![0i64; 9];
    for j in 0..9 {
        reversed[(9 - j) % 9] = coeff[j];
    }
    let (inv, _, _, _) = eigenvalue(&reversed);
    assert_eq!(inv, Fp::new(8623).inv(lambda));
    assert_ne!(c.mul(source, inv), image);
    eprintln!("character_coefficients={coeff:?} zeta={z} lambda={lambda} gcd={:?} operator_checks={} reconstructions={} reversed_lambda={inv} reversed_group_rejected=true",&g.c[..g.n],fb.len(),logs.len());
}
fn recover(
    c: &Curve,
    fb: &[(u64, u64)],
    source: &Div,
    target: &Div,
    logs: &[u64],
) -> (Option<u64>, usize, usize) {
    let scalar = Fp::new(8623);
    let inv13 = scalar.inv(13);
    let mut r = *target;
    for offset in 0..2048usize {
        let (roots, rest) = rational_roots(r.u, &c.fp);
        if rest.deg() == 0 {
            let signed = norm_row(c, fb, &r.v, &roots);
            assert_eq!(group_sum(c, fb, &signed), r);
            let projected = signed.iter().zip(logs).fold(0, |s, (&a, &b)| {
                scalar.add(s, scalar.mul(scalar.from_i64(a), b))
            });
            // Projection gives log(13*R)=13*(k+offset); do not omit inverse13.
            let k = scalar.sub(scalar.mul(inv13, projected), offset as u64 % scalar.p);
            assert_eq!(c.mul(source, k), *target);
            return (Some(k), offset + 1, offset);
        }
        if offset + 1 < 2048 {
            r = c.add(&r, source)
        }
    }
    (None, 2048, 2047)
}
fn main() {
    use hec_native_ic::ic::{solve_dlp, IcParams};
    let c = Curve::new(
        Fp::new(19),
        Poly::from_coeffs(&[2, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
    );
    assert_eq!(c.g, 4);
    assert!(irreducible(&c.f, &c.fp));
    assert_eq!(hec_native_ic::instance::prime_factors(8623), vec![8623]);
    let fb = factor_base(&c);
    assert_eq!(fb.len(), 9);
    let source = c.mul(&c.place(fb[0].0, fb[0].1), 13);
    assert!(!c.is_zero(&source));
    assert!(c.is_zero(&c.mul(&source, 8623)));
    println!("case\tnorm_candidates\tcharacter_terms\tgcd_degree\tgauss_mulmods\trecovered\tdescent_trials\tdescent_adds\tnative_k\tnative_trials\tnative_precompute_ops\tnative_group_ops\tnative_solve_mulmods");
    let mut failures = 0;
    for (case, known) in [37u64, 2026, 7001].into_iter().enumerate() {
        let target = c.mul(&source, known);
        eprintln!("public_input case={case} source={source:?} target={target:?}");
        let (logs, candidates, labels, cache_rows, muls) = character_logs(&c, &fb, &source);
        qualify(&c, &fb, &source, &logs);
        let (k, trials, adds) = recover(&c, &fb, &source, &target, &logs);
        let params = IcParams {
            seed: 2026111600 + case as u64,
            max_trials: 200000,
            ..IcParams::default()
        };
        let native = solve_dlp(&c, &source, &target, 8623, &params);
        if k != Some(known) || native.k != Some(known) {
            failures += 1
        }
        println!("{case}\t{candidates}\t{labels}\t{cache_rows}\t{muls}\t{}\t{trials}\t{adds}\t{}\t{}\t{}\t{}\t{}",k.map_or_else(||"incomplete".into(),|x|x.to_string()),native.k.map_or_else(||"incomplete".into(),|x|x.to_string()),native.trials,native.precompute_ops,native.group_ops,native.solve.mulmods);
    }
    if failures != 0 {
        eprintln!("INCOMPLETE_SINGLE_TARGET_CONTROLS count={failures}");
        std::process::exit(2)
    }
}
