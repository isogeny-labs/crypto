use hec_native_ic::{fp::Fp, poly::Poly};
use std::sync::OnceLock;

#[derive(Clone)]
pub struct Config {
    pub qf: [u64; 15],
    pub points: Vec<[u64; 3]>,
    pub o: [u64; 3],
    pub tangent: [u64; 2],
    pub residual: [u64; 3],
    pub canonical: [[u64; 3]; 4],
    pub controls: [[u64; 3]; 4],
}
static CONFIG: OnceLock<Config> = OnceLock::new();
pub fn get() -> &'static Config {
    CONFIG
        .get()
        .expect("chart must be computed before section construction")
}
pub fn install(c: Config) {
    assert!(CONFIG.set(c).is_ok());
}
fn power(x: &Poly, n: u8, fp: &Fp) -> Poly {
    (0..n).fold(Poly::one(), |a, _| a.mul(x, fp))
}
fn restrict(f: &[u64; 15], x: &Poly, y: &Poly, axis: Option<usize>) -> Poly {
    let fp = Fp::new(crate::field::p());
    super::target::monomials(4)
        .iter()
        .zip(f)
        .fold(Poly::zero(), |z, (&(a, b, _), c)| {
            let mut ex = [a, b];
            let mut c = *c;
            if let Some(k) = axis {
                if ex[k] == 0 {
                    return z;
                }
                c = c * ex[k] as u64 % crate::field::p();
                ex[k] -= 1;
            }
            z.add(
                &power(x, ex[0], &fp)
                    .mul(&power(y, ex[1], &fp), &fp)
                    .scale(c, &fp),
                &fp,
            )
        })
}
fn roots(f: &Poly) -> Vec<u64> {
    assert!(!f.is_zero());
    if f.deg() <= 0 {
        return vec![];
    }
    let fp = Fp::new(crate::field::p());
    let h = Poly::gcd(f, &Poly::x().powmod(crate::field::p(), f, &fp).sub(&Poly::x(), &fp), &fp);
    let r = hec_native_ic::oracle::split(&h, &fp, &mut Default::default()).unwrap();
    let mut r: Vec<_> = r.as_slice().iter().map(|r| r.0).collect();
    r.sort_unstable();
    r
}
pub fn evaluate(f: &[u64; 15], p: [u64; 3]) -> u64 {
    let fp = Fp::new(crate::field::p());
    super::target::monomials(4)
        .iter()
        .zip(f)
        .fold(0, |s, (&(a, b, c), v)| {
            (s + v * fp.pow(p[0], a as u64) % crate::field::p() * fp.pow(p[1], b as u64) % crate::field::p()
                * fp.pow(p[2], c as u64))
                % crate::field::p()
        })
}
pub fn derivative(f: &[u64; 15], p: [u64; 3], axis: usize) -> u64 {
    let fp = Fp::new(crate::field::p());
    super::target::monomials(4)
        .iter()
        .zip(f)
        .fold(0, |s, (&(a, b, c), v)| {
            let mut e = [a, b, c];
            if e[axis] == 0 {
                return s;
            }
            let n = e[axis] as u64;
            e[axis] -= 1;
            (s + v * n % crate::field::p() * fp.pow(p[0], e[0] as u64) % crate::field::p() * fp.pow(p[1], e[1] as u64) % crate::field::p()
                * fp.pow(p[2], e[2] as u64))
                % crate::field::p()
        })
}
pub fn points(f: &[u64; 15]) -> Vec<[u64; 3]> {
    let fp = Fp::new(crate::field::p());
    let mut out = vec![];
    for x in 0..crate::field::p() {
        let fy = restrict(f, &Poly::constant(x), &Poly::x(), None);
        for y in roots(&fy) {
            out.push([x, y, 1]);
        }
    }
    let mut inf = Poly::zero();
    for (&(a, _, c), v) in super::target::monomials(4).iter().zip(f) {
        if c == 0 {
            inf.c[a as usize] = *v;
            inf.n = inf.n.max(a as usize + 1);
        }
    }
    for x in roots(&inf) {
        out.push([x, 1, 0]);
    }
    assert!(out.iter().all(|p| evaluate(f, *p) == 0));
    let _ = fp;
    out
}
fn collinear(a: [u64; 3], b: [u64; 3], c: [u64; 3]) -> bool {
    let z = (a[0] * ((b[1] * c[2] + crate::field::p() - b[2] * c[1] % crate::field::p()) % crate::field::p())
        + a[1] * ((b[2] * c[0] + crate::field::p() - b[0] * c[2] % crate::field::p()) % crate::field::p())
        + a[2] * ((b[0] * c[1] + crate::field::p() - b[1] * c[0] % crate::field::p()) % crate::field::p()))
        % crate::field::p();
    z == 0
}
fn intersections(f: &[u64; 15], p: [u64; 3], q: [u64; 3]) -> Option<[[u64; 3]; 4]> {
    let fp = Fp::new(crate::field::p());
    let (x, y, t0, t1, inf) = if p[0] != q[0] {
        let a = fp.sub(q[1], p[1]) * fp.inv(fp.sub(q[0], p[0])) % crate::field::p();
        let b = fp.sub(p[1], a * p[0] % crate::field::p());
        (
            Poly::x(),
            Poly::from_coeffs(&[b, a]),
            p[0],
            q[0],
            if a == 0 { [1, 0, 0] } else { [fp.inv(a), 1, 0] },
        )
    } else {
        (Poly::constant(p[0]), Poly::x(), p[1], q[1], [0, 1, 0])
    };
    let poly = restrict(f, &x, &y, None);
    let den = Poly::from_coeffs(&[fp.neg(t0), 1]).mul(&Poly::from_coeffs(&[fp.neg(t1), 1]), &fp);
    let (res, r) = poly.divrem(&den, &fp);
    assert!(r.is_zero());
    if res.is_zero() {
        return None;
    }
    let mut hit = vec![p, q];
    for t in roots(&res) {
        hit.push([x.eval(t, &fp), y.eval(t, &fp), 1]);
    }
    if poly.deg() < 4 {
        hit.push(inf);
    }
    hit.sort_unstable();
    hit.dedup();
    if hit.len() != 4 {
        return None;
    }
    let points: [[u64; 3]; 4] = hit.try_into().unwrap();
    assert!(points.iter().all(|p| evaluate(f, *p) == 0));
    Some(points)
}
pub fn discover(f: [u64; 15]) -> Config {
    assert_eq!(f[0], 1, "monic X4 model required");
    assert!(f.iter().all(|c| *c < crate::field::p()));
    let fp = Fp::new(crate::field::p());
    let points = points(&f);
    assert_eq!(
        points,
        super::target::rational_points(&f, crate::field::p()),
        "independent_point_list"
    );
    let aff: Vec<_> = points.iter().copied().filter(|p| p[2] == 1).collect();
    let mut anchor = None;
    for o in aff.iter().take(32) {
        let fy = derivative(&f, *o, 1);
        if fy == 0 {
            println!("ANCHOR_EXCLUDE point={o:?} reason=vertical");
            continue;
        }
        let a = fp.neg(derivative(&f, *o, 0)) * fp.inv(fy) % crate::field::p();
        let b = fp.sub(o[1], a * o[0] % crate::field::p());
        let x = Poly::x();
        let y = Poly::from_coeffs(&[b, a]);
        let line = restrict(&f, &x, &y, None);
        let factor = Poly::from_coeffs(&[fp.neg(o[0]), 1]);
        let den = factor.mul(&factor, &fp);
        let (r, rem) = line.divrem(&den, &fp);
        assert!(rem.is_zero());
        if r.deg() != 2 || r.eval(o[0], &fp) == 0 {
            println!("ANCHOR_EXCLUDE point={o:?} reason=flex_or_infinity");
            continue;
        }
        let r = r.monic(&fp);
        if Poly::gcd(&r, &r.deriv(&fp), &fp).deg() != 0
            || Poly::gcd(&r, &restrict(&f, &x, &y, Some(1)), &fp).deg() != 0
        {
            println!("ANCHOR_EXCLUDE point={o:?} reason=repeated_or_vertical_base");
            continue;
        }
        anchor = Some((*o, [a, b], [r.c[0], r.c[1], 1]));
        break;
    }
    let (o, tangent, residual) = anchor.expect("no_supported_anchor_first32");
    let mut k = None;
    let mut pairs = 0;
    let small: Vec<_> = aff.iter().take(32).copied().collect();
    'outer: for a in 0..small.len() {
        for b in a + 1..small.len() {
            pairs += 1;
            if let Some(v) = intersections(&f, small[a], small[b]) {
                k = Some(v);
                break 'outer;
            }
        }
    }
    let canonical = k.expect("no_split_canonical_line_first32");
    let mut controls = vec![];
    for p in &aff {
        if derivative(&f, *p, 1) == 0 || controls.iter().any(|q: &[u64; 3]| q[0] == p[0]) {
            continue;
        }
        if (0..controls.len())
            .any(|i| (i + 1..controls.len()).any(|j| collinear(controls[i], controls[j], *p)))
        {
            continue;
        }
        controls.push(*p);
        if controls.len() == 4 {
            break;
        }
    }
    let controls = controls.try_into().expect("four_regular_control_points");
    println!("CHART point_count={} O={o:?} tangent={tangent:?} residual={residual:?} canonical={canonical:?} canonical_pairs_tried={pairs} controls={controls:?} archived_anchor_used=false",points.len());
    Config {
        qf: f,
        points,
        o,
        tangent,
        residual,
        canonical,
        controls,
    }
}
