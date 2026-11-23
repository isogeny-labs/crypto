use hec_native_ic::{
    instance::{factor_base, thread_curve},
    jac::Div,
    oracle::{split, OracleStats},
    poly::Poly,
    rng::Rng,
};

fn main() {
    let (mut trials, mut splits, mut drops, mut repeated, mut ramified, mut low, mut skipped) =
        (0, 0, 0, 0, 0, 0, 0);
    for g in [2, 3, 4] {
        let (mut gt, mut gs, mut nonsplit_inputs) = (0, 0, 0);
        for p in [11, 13, 101] {
            for cc in [1, 2] {
                let Some(c) = thread_curve(p, cc, g) else {
                    continue;
                };
                let f = &c.fp;
                let fb = factor_base(&c);
                for cohort in [0, 1] {
                    let mut rng = Rng::new(2026111501);
                    for _ in 0..16 {
                        let steps = if cohort == 0 { g } else { 2 * g + 3 };
                        let d = (0..steps).fold(c.zero(), |d, _| {
                            let (x, y) = fb[rng.below(fb.len() as u64) as usize];
                            c.add_place(&d, x, y)
                        });
                        if d.u.deg() != g as isize {
                            low += 1;
                            continue;
                        }
                        nonsplit_inputs +=
                            u64::from(split(&d.u, f, &mut OracleStats::default()).is_none());
                        let mut points = Vec::new();
                        for &(x, y) in fb.iter().take(8) {
                            let ux = d.u.eval(x, f);
                            if ux == 0 {
                                skipped += 1;
                                continue;
                            }
                            for y in [Some(y), (y != 0).then_some(f.neg(y))]
                                .into_iter()
                                .flatten()
                            {
                                points.push((x, y, f.mul(f.sub(d.v.eval(x, f), y), f.inv(ux))));
                            }
                        }
                        for i in 0..points.len() {
                            for j in i + 1..points.len() {
                                let (xi, yi, zi) = points[i];
                                let (xj, yj, zj) = points[j];
                                if xi == xj {
                                    continue;
                                }
                                let lambda = f.mul(f.sub(zj, zi), f.inv(f.sub(xj, xi)));
                                let mu = f.sub(zi, f.mul(lambda, xi));
                                let w = Poly::from_coeffs(&[mu, lambda]);
                                let t = d.v.sub(&d.u.mul(&w, f), f);
                                assert_eq!(t.eval(xi, f), yi);
                                assert_eq!(t.eval(xj, f), yj);
                                let u =
                                    d.u.mul(&Poly::x_minus(xi, f), f)
                                        .mul(&Poly::x_minus(xj, f), f);
                                let r = t.mul(&t, f).sub(&c.f, f).div_exact(&u, f).monic(f);
                                assert_eq!(
                                    r.deg(),
                                    if lambda == 0 {
                                        g as isize - 1
                                    } else {
                                        g as isize
                                    }
                                );
                                drops += u64::from(lambda == 0);
                                let residual = Div {
                                    u: r,
                                    v: t.rem(&r, f),
                                };
                                assert!(c.is_valid(&residual));
                                let sum = c.add_place(&c.add_place(&d, xi, yi), xj, yj);
                                assert_eq!(
                                    residual,
                                    c.neg(&sum),
                                    "ordinary addition equivalence g={g} p={p}"
                                );
                                let got = split(&r, f, &mut OracleStats::default());
                                let mut rem = r;
                                let mut roots = Vec::new();
                                for x in 0..p {
                                    let mut m = 0;
                                    while rem.deg() > 0 && rem.eval(x, f) == 0 {
                                        rem = rem.div_exact(&Poly::x_minus(x, f), f);
                                        m += 1;
                                    }
                                    if m > 0 {
                                        roots.push((x, m));
                                    }
                                }
                                assert_eq!(got.is_some(), rem.deg() == 0);
                                if let Some(got) = got {
                                    assert_eq!(got.as_slice(), roots.as_slice());
                                    let mut total = sum;
                                    for &(x, m) in got.as_slice() {
                                        let y = t.eval(x, f);
                                        assert_eq!(f.mul(y, y), c.f.eval(x, f));
                                        total = c.add(&total, &c.mul(&c.place(x, y), m as u64));
                                    }
                                    assert_eq!(total, c.zero(), "principal identity");
                                    repeated +=
                                        u64::from(got.as_slice().iter().any(|&(_, m)| m > 1));
                                    ramified += u64::from(
                                        got.as_slice().iter().any(|&(x, _)| t.eval(x, f) == 0),
                                    );
                                    splits += 1;
                                    gs += 1;
                                }
                                trials += 1;
                                gt += 1;
                            }
                        }
                    }
                }
            }
        }
        assert!(
            nonsplit_inputs > 0,
            "missing non-split input coverage genus {g}"
        );
        println!(
            "GENUS g={g} trials={gt} split={gs} nonsplit_inputs={nonsplit_inputs} equivalence=true"
        );
    }
    assert!(trials > 0 && drops > 0 && repeated > 0 && ramified > 0);
    println!("TERMINAL trials={trials} split={splits} degree_drops={drops} repeated={repeated} ramified={ramified} low_degree={low} skipped_u={skipped} correct=true");
}
