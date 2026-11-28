fn chars3(f: &Field) -> Vec<Vec<E>> {
    (0..3)
        .map(|a| {
            (0..27)
                .map(|p| {
                    let a = digits(a);
                    let p = digits(p);
                    f.pow(
                        base(cold_zeta()),
                        ((0..3).map(|i| a[i] * p[i]).sum::<usize>() % 3) as u128,
                    )
                })
                .collect()
        })
        .collect()
}
fn section_scalar(
    c: &Curve,
    ks: &[Div],
    hs: &[H],
    neg: &[usize],
    alpha: &[E],
    chars: &[Vec<E>],
    x: &Div,
) -> Option<(Vec<E>, [E; 8])> {
    let vals: Option<Vec<E>> = (0..27)
        .map(|i| evaluate(c, &ks[neg[i]], &hs[neg[i]], x))
        .collect();
    let vals = vals?;
    let ph = projections(&vals, alpha, chars, &c.field);
    let f = &c.field;
    let mut z = Z;
    z[1] = 1;
    let prod = f.mul(ph[1], ph[2]);
    let s = std::array::from_fn(|i| {
        if i == 0 {
            f.mul(base(13), f.mul(ph[0], ph[0]))
        } else {
            field_trace(f.mul(f.pow(z, (i - 1) as u128), prod), f)
        }
    });
    Some((vals, s))
}
