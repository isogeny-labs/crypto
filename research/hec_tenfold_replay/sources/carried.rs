use hec_native_ic::{
    jac::{Curve, Div},
    poly::Poly,
};
pub fn cofactor(c: &Curve, d: &Div) -> Poly {
    c.f.sub(&d.v.mul(&d.v, &c.fp), &c.fp).div_exact(&d.u, &c.fp)
}
pub fn step(
    c: &Curve,
    d: &Div,
    h: &mut Poly,
    e: u64,
    y: u64,
    inv: &[u64],
    cached: Option<u64>,
) -> Div {
    let fp = &c.fp;
    let k = cached.or_else(|| {
        let u = d.u.eval(e, fp);
        (u != 0).then(|| fp.mul(fp.sub(y, d.v.eval(e, fp)), inv[u as usize]))
    });
    let Some(k) = k.filter(|_| d.u.n == c.g + 1) else {
        let next = c.add_place(d, e, y);
        *h = cofactor(c, &next);
        return next;
    };
    let times_e = |a| match e {
        0 => 0,
        1 => a,
        _ => fp.mul(e, a),
    };
    let mut u = Poly::zero();
    u.n = c.g + 1;
    u.c[c.g] = 1;
    let mut w = d.v.add(&d.u.scale(k, fp), fp);
    for i in (1..=c.g).rev() {
        let vplus = fp.add(d.v.c[i], w.c[i]);
        u.c[i - 1] = fp.add(fp.sub(h.c[i], fp.mul(k, vplus)), times_e(u.c[i]));
    }
    let v = u.scale(k, fp).sub(&w, fp);
    for i in 0..=c.g {
        let previous = if i == 0 { 0 } else { d.u.c[i - 1] };
        w.c[i] = fp.add(
            fp.sub(previous, times_e(d.u.c[i])),
            fp.mul(k, fp.sub(w.c[i], v.c[i])),
        );
    }
    w.c[c.g + 1] = 1;
    w.n = c.g + 2;
    *h = w;
    Div { u, v }
}
