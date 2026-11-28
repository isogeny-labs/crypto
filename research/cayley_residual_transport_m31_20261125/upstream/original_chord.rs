//! Signed chord extraction on public ordinary M31 curves.
use crate::upstream::{Arithmetic, Curve, Point};
pub const P: u64 = 2_147_483_647;
pub type Affine = (u64, u64);
pub type Pair = (Affine, Affine);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Selector {
    Identity,
    Lambda(u64),
}

pub fn add(x: u64, y: u64) -> u64 {
    (x + y) % P
}
pub fn sub(x: u64, y: u64) -> u64 {
    (x + P - y) % P
}
pub fn mul(x: u64, y: u64) -> u64 {
    ((x as u128 * y as u128) % P as u128) as u64
}
pub fn pow(mut x: u64, mut n: u64) -> u64 {
    let mut r = 1;
    while n != 0 {
        if n & 1 != 0 {
            r = mul(r, x);
        }
        x = mul(x, x);
        n >>= 1;
    }
    r
}
pub fn inv(x: u64) -> u64 {
    assert!(x > 0 && x < P);
    pow(x, P - 2)
}

#[derive(Default)]
pub struct Diagnostics {
    pub first_quadratic: usize,
    pub first_linear: usize,
    pub identity_selector: usize,
    pub zero_slope_discriminant: usize,
    pub zero_coordinate_discriminant: usize,
    pub sqrt_calls: usize,
}

pub fn roots(a: u64, b: u64, c: u64, d: &mut Diagnostics, slope: bool) -> Vec<u64> {
    if a == 0 {
        assert_ne!(b, 0, "zero/constant polynomial outside component");
        return vec![mul(sub(0, c), inv(b))];
    }
    let delta = sub(mul(b, b), mul(4, mul(a, c)));
    if delta == 0 {
        if slope {
            d.zero_slope_discriminant += 1;
        } else {
            d.zero_coordinate_discriminant += 1;
        }
    }
    d.sqrt_calls += 1;
    let Some(t) = Arithmetic::default().sqrt(delta, P) else {
        return Vec::new();
    };
    let denom = inv(add(a, a));
    let mut out = vec![mul(add(sub(0, b), t), denom)];
    let second = mul(sub(sub(0, b), t), denom);
    if second != out[0] {
        out.push(second);
    }
    out.sort_unstable();
    out
}

/// Inputs contain no hidden scalar, witness, factor base or precomputed roots.
/// Output consists of all affine ordered pairs; identity summands are excluded.
pub fn extract(
    e: Curve,
    target: Point,
    selector: Selector,
    d: &mut Diagnostics,
) -> Result<Vec<Pair>, String> {
    if e.p != P || e.a >= P || e.b >= P {
        return Err("requires canonical M31 coefficients".into());
    }
    let Some((w, y)) = target else {
        return Err("identity target unsupported".into());
    };
    if y == 0 || !Arithmetic::default().on_curve(e, target) {
        return Err("requires affine on-curve target with nonzero ordinate".into());
    }
    let ms = match selector {
        Selector::Identity => {
            d.identity_selector += 1;
            roots(1, 0, sub(0, w), d, true)
        }
        Selector::Lambda(l) => {
            if l >= P {
                return Err("noncanonical lambda".into());
            }
            let aa = add(w, l);
            let bb = add(y, y);
            let cc = sub(sub(add(mul(w, w), e.a), 1), mul(l, w));
            if aa == 0 {
                d.first_linear += 1;
            } else {
                d.first_quadratic += 1;
            }
            let half_disc = add(add(mul(w, mul(l, l)), mul(sub(1, e.a), l)), add(w, e.b));
            assert_eq!(sub(mul(bb, bb), mul(4, mul(aa, cc))), mul(4, half_disc));
            roots(aa, bb, cc, d, true)
        }
    };
    let mut result = Vec::new();
    for m in ms {
        let m2 = mul(m, m);
        let s = sub(m2, w);
        // Vieta for the chord through -R after removing the intersection -R.
        let q = add(add(add(mul(w, m2), mul(w, w)), e.a), mul(add(y, y), m));
        match selector {
            Selector::Identity => assert_eq!(s, 0),
            Selector::Lambda(l) => assert_eq!(add(q, mul(l, s)), 1),
        }
        for x1 in roots(1, sub(0, s), q, d, false) {
            let x2 = sub(s, x1);
            let p1 = (x1, sub(mul(m, sub(x1, w)), y));
            let p2 = (x2, sub(mul(m, sub(x2, w)), y));
            result.push((p1, p2));
        }
    }
    result.sort_unstable();
    result.dedup();
    Ok(result)
}

pub fn selector_for(p1: Affine, p2: Affine) -> Selector {
    let s = add(p1.0, p2.0);
    if s == 0 {
        Selector::Identity
    } else {
        Selector::Lambda(mul(sub(1, mul(p1.0, p2.0)), inv(s)))
    }
}

// Direct extension-field evaluation checks the convention independently of s,q.
pub fn z_mul(z: Affine, w: Affine) -> Affine {
    (
        sub(mul(z.0, w.0), mul(z.1, w.1)),
        add(mul(z.0, w.1), mul(z.1, w.0)),
    )
}
fn z_div(z: Affine, w: Affine) -> Affine {
    let norm = add(mul(w.0, w.0), mul(w.1, w.1));
    assert_ne!(norm, 0);
    let zi = inv(norm);
    z_mul(z, (mul(w.0, zi), mul(sub(0, w.1), zi)))
}
pub fn cayley(x: u64) -> Affine {
    z_div((x, 1), (x, P - 1))
}
pub fn selector_z(s: Selector) -> Affine {
    match s {
        Selector::Identity => (1, 0),
        Selector::Lambda(l) => z_div((l, P - 1), (l, 1)),
    }
}
