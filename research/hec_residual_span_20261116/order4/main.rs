mod cert;
mod field;
use hec_native_ic::{
    fp::Fp,
    instance::{factor_base, prime_factors},
    jac::{Curve, Div},
    poly::Poly,
    rng::Rng,
};
use std::{fs, io::Write};
fn action(c: &Curve, d: &Div, i: u64) -> Div {
    let mut u = d.u;
    let mut v = d.v;
    let degree = u.deg() as usize;
    for j in 0..u.n {
        if (degree + j) % 2 != 0 {
            u.c[j] = c.fp.neg(u.c[j])
        }
    }
    for j in 0..v.n {
        v.c[j] = c.fp.mul(v.c[j], if j % 2 == 0 { i } else { c.fp.neg(i) })
    }
    let out = Div { u, v };
    assert!(c.is_valid(&out));
    out
}
fn main() {
    cert::controls();
    let root = "research/hec_residual_span_20261116/order4";
    let mut points = fs::File::create_new(format!("{root}/POINTS.tsv")).unwrap();
    writeln!(points, "p\tmodel\td\tmodulus\tpoints").unwrap();
    let mut witness = fs::File::create_new(format!("{root}/WITNESSES.tsv")).unwrap();
    writeln!(witness, "p\tmodel\tm\tprime").unwrap();
    let mut polynomials = fs::File::create_new(format!("{root}/WEIL.tsv")).unwrap();
    writeln!(
        polynomials,
        "p\tmodel\tpolynomial_ascending\tfactor\tordinary"
    )
    .unwrap();
    println!("p\tmodel\ta7\ta5\ta3\ta1\tstatus\tN\tell\th\tordinary\tfq_simple_certified\tabs_simple_certified\tgroup_qualified\tm\torbits\tlambda\tunresolved_m");
    let (mut classified, mut smooth, mut simple, mut groups) = (0, 0, 0, 0);
    for p in [13u64, 17, 29] {
        assert!((2..).take_while(|d| d * d <= p).all(|d| p % d != 0));
        assert_eq!(p % 4, 1);
        let fp = Fp::new(p);
        let i = fp.sqrt(p - 1).unwrap();
        assert_eq!(fp.mul(i, i), p - 1);
        let fields: Vec<_> = (1..=4)
            .map(|d| {
                let f = field::Field::new(p, d);
                f.validate();
                f
            })
            .collect();
        for (id, a) in [[1u64, 2, 3, 1], [2, 1, 4, 1]].into_iter().enumerate() {
            classified += 1;
            let polynomial = Poly::from_coeffs(&[0, a[3], 0, a[2], 0, a[1], 0, a[0], 0, 1]);
            if Poly::gcd(&polynomial, &polynomial.deriv(&fp), &fp).deg() != 0 {
                println!("{p}\t{id}\t{}\t{}\t{}\t{}\tsingular\tnone\tnone\tnone\tfalse\tfalse\tfalse\tfalse\t0\t0\tnone\tnot_attempted",a[0],a[1],a[2],a[3]);
                continue;
            }
            smooth += 1;
            eprintln!("count_start p={p} model={id}");
            let c = Curve::new(fp, polynomial);
            let mut counts = [0; 4];
            for (d, f) in fields.iter().enumerate() {
                counts[d] = f.count(a);
                writeln!(
                    points,
                    "{p}\t{id}\t{}\t{:?}\t{}",
                    d + 1,
                    &f.modulus.c[..f.modulus.n],
                    counts[d]
                )
                .unwrap()
            }
            let reference = 1
                + (0..p)
                    .map(|x| 1 + fp.legendre(c.f.eval(x, &fp)) as i64)
                    .sum::<i64>();
            assert_eq!(counts[0], reference as u64);
            let weil = cert::weil(p, counts);
            let n: u64 = weil.iter().sum::<i128>().try_into().unwrap();
            assert!(n > 0);
            let ordinary = weil[4] % (p as i128) != 0;
            let factor = cert::proper_factor(p, &weil);
            let fq_simple = ordinary && factor.is_none();
            writeln!(polynomials, "{p}\t{id}\t{weil:?}\t{factor:?}\t{ordinary}").unwrap();
            let mut unresolved = vec![];
            if fq_simple {
                for m in 1..=8192 {
                    if cert::phi(m) > 64 {
                        continue;
                    }
                    let r = cert::primes()
                        .into_iter()
                        .find(|&r| cert::witness(&weil, m, r));
                    writeln!(
                        witness,
                        "{p}\t{id}\t{m}\t{}",
                        r.map(|r| r.to_string()).unwrap_or("none".into())
                    )
                    .unwrap();
                    if r.is_none() {
                        unresolved.push(m)
                    }
                }
            }
            let abs_simple = fq_simple && unresolved.is_empty();
            simple += usize::from(abs_simple);
            let fb = factor_base(&c);
            assert_eq!(
                counts[0],
                1 + fb
                    .iter()
                    .map(|&(_, y)| if y == 0 { 1 } else { 2 })
                    .sum::<u64>()
            );
            for &(x, y) in &fb {
                let d = c.place(x, y);
                assert_eq!(action(&c, &d, i), c.place(fp.neg(x), fp.mul(i, y)));
                assert_eq!(action(&c, &action(&c, &d, i), i), c.neg(&d));
                assert!(c.is_zero(&c.mul(&d, n)))
            }
            let mut rng = Rng::new(20261119 ^ p ^ id as u64);
            for _ in 0..64 {
                let j = rng.below(fb.len() as u64) as usize;
                let k = rng.below(fb.len() as u64) as usize;
                let a = c.mul(&c.place(fb[j].0, fb[j].1), rng.below(9) + 1);
                let b = c.mul(&c.place(fb[k].0, fb[k].1), rng.below(9) + 1);
                assert_eq!(
                    action(&c, &c.add(&a, &b), i),
                    c.add(&action(&c, &a, i), &action(&c, &b, i))
                );
                let twice = action(&c, &action(&c, &a, i), i);
                assert_eq!(twice, c.neg(&a));
                assert_eq!(action(&c, &action(&c, &twice, i), i), a)
            }
            if let Some(&(x, y)) = fb.iter().find(|v| v.1 != 0) {
                let wrong = Div {
                    u: Poly::x_minus(fp.neg(x), &fp),
                    v: Poly::constant(y),
                };
                assert!(!c.is_valid(&wrong))
            } else {
                eprintln!("no_nonbranch_negative_control p={p} model={id}")
            }
            let ell = *prime_factors(n).last().unwrap();
            let h = n / ell;
            let mut lambda = None;
            let mut orbits = 0;
            let mut group_ok = false;
            if ell > 2 && h % ell != 0 {
                if let Some(l) = Fp::new(ell).sqrt(ell - 1) {
                    let candidates = [l, ell - l];
                    let source = fb
                        .iter()
                        .map(|&(x, y)| c.mul(&c.place(x, y), h))
                        .find(|d| !c.is_zero(d));
                    if let Some(d) = source {
                        assert!(c.is_zero(&c.mul(&d, ell)));
                        let image = action(&c, &d, i);
                        let matches: Vec<_> = candidates
                            .into_iter()
                            .filter(|&v| c.mul(&d, v) == image)
                            .collect();
                        assert_eq!(matches.len(), 1);
                        lambda = Some(matches[0]);
                        for &(x, y) in &fb {
                            let projected = c.mul(&c.place(x, y), h);
                            assert!(c.is_zero(&c.mul(&projected, ell)));
                            assert_eq!(action(&c, &projected, i), c.mul(&projected, matches[0]));
                            if y == 0 {
                                assert!(c.is_zero(&projected))
                            }
                        }
                        let mut seen = vec![false; fb.len()];
                        let lf = Fp::new(ell);
                        for j in 0..fb.len() {
                            if seen[j] {
                                continue;
                            }
                            if fb[j].1 == 0 {
                                seen[j] = true;
                                continue;
                            }
                            orbits += 1;
                            let (mut xx, mut yy, mut w) = (fb[j].0, fb[j].1, 1);
                            for _ in 0..2 {
                                let k = fb.iter().position(|v| v.0 == xx).unwrap();
                                assert!(!seen[k]);
                                seen[k] = true;
                                let weight = if fb[k].1 == yy {
                                    w
                                } else {
                                    assert_eq!(yy, fp.neg(fb[k].1));
                                    lf.neg(w)
                                };
                                let pj = c.mul(&c.place(fb[j].0, fb[j].1), h);
                                assert_eq!(
                                    c.mul(&c.place(fb[k].0, fb[k].1), h),
                                    c.mul(&pj, weight)
                                );
                                xx = fp.neg(xx);
                                yy = fp.mul(i, yy);
                                w = lf.mul(w, matches[0]);
                            }
                            assert_eq!((xx, yy, w), (fb[j].0, fp.neg(fb[j].1), ell - 1));
                        }
                        assert!(seen.iter().all(|v| *v));
                        group_ok = true;
                        groups += 1
                    }
                }
            }
            println!("{p}\t{id}\t{}\t{}\t{}\t{}\tsmooth\t{n}\t{ell}\t{h}\t{ordinary}\t{fq_simple}\t{abs_simple}\t{group_ok}\t{}\t{orbits}\t{}\t{}",a[0],a[1],a[2],a[3],fb.len(),lambda.map(|v|v.to_string()).unwrap_or("none".into()),if fq_simple{format!("{unresolved:?}")}else{"not_attempted".into()});
        }
    }
    assert_eq!(classified, 6);
    eprintln!("classified={classified} smooth={smooth} absolute_certified={simple} group_qualified={groups} no_target_solves=true");
}
