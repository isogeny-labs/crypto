//! Native structural check for the frozen H115 base-field fixtures.
//! This does not construct extension fields, moments, predicates or scores.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Clone, Copy)]
struct Curve {
    slug: &'static str,
    p: u64,
    a: u64,
    b: u64,
    r: u64,
    trace: i64,
    d: u64,
    extension_size: u64,
}

const CURVES_SHA256: &str = "babd2556951ecfcaee66a5c6966364ca136bf1f62fb0a51511f391768255d295";

const CURVES: [Curve; 5] = [
    Curve {
        slug: "h115-fp7-a1-b1-r5-d4",
        p: 7,
        a: 1,
        b: 1,
        r: 5,
        trace: 3,
        d: 4,
        extension_size: 2_401,
    },
    Curve {
        slug: "h115-fp5-a2-b1-r7-d6",
        p: 5,
        a: 2,
        b: 1,
        r: 7,
        trace: -1,
        d: 6,
        extension_size: 15_625,
    },
    Curve {
        slug: "h115-fp13-a2-b4-r17-d4",
        p: 13,
        a: 2,
        b: 4,
        r: 17,
        trace: -3,
        d: 4,
        extension_size: 28_561,
    },
    Curve {
        slug: "h115-fp31-a1-b19-r37-d4",
        p: 31,
        a: 1,
        b: 19,
        r: 37,
        trace: -5,
        d: 4,
        extension_size: 923_521,
    },
    Curve {
        slug: "h115-fp47-a1-b38-r61-d3",
        p: 47,
        a: 1,
        b: 38,
        r: 61,
        trace: -13,
        d: 3,
        extension_size: 103_823,
    },
];

fn add_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}

fn sub_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + p as u128 - b as u128) % p as u128) as u64
}

fn mul_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn pow_mod(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut out = 1;
    while n != 0 {
        if n & 1 == 1 {
            out = mul_mod(out, a, p);
        }
        a = mul_mod(a, a, p);
        n >>= 1;
    }
    out
}

fn inv_mod(a: u64, p: u64) -> u64 {
    assert_ne!(a, 0);
    pow_mod(a, p - 2, p)
}

fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut q = 2;
    while q * q <= n {
        if n % q == 0 {
            return n == q;
        }
        q += 1;
    }
    true
}

fn rhs(curve: Curve, x: u64) -> u64 {
    add_mod(
        add_mod(
            mul_mod(mul_mod(x, x, curve.p), x, curve.p),
            mul_mod(curve.a, x, curve.p),
            curve.p,
        ),
        curve.b,
        curve.p,
    )
}

fn on_curve(curve: Curve, point: Point) -> bool {
    mul_mod(point.y, point.y, curve.p) == rhs(curve, point.x)
}

fn points(curve: Curve) -> Vec<Point> {
    let mut out = Vec::new();
    for x in 0..curve.p {
        let value = rhs(curve, x);
        for y in 0..curve.p {
            if mul_mod(y, y, curve.p) == value {
                out.push(Point { x, y });
            }
        }
    }
    out
}

fn add(curve: Curve, left: Option<Point>, right: Option<Point>) -> Option<Point> {
    let (p1, p2) = match (left, right) {
        (None, other) | (other, None) => return other,
        (Some(p1), Some(p2)) => (p1, p2),
    };
    if p1.x == p2.x && add_mod(p1.y, p2.y, curve.p) == 0 {
        return None;
    }
    let slope = if p1 == p2 {
        let numerator = add_mod(
            mul_mod(3, mul_mod(p1.x, p1.x, curve.p), curve.p),
            curve.a,
            curve.p,
        );
        let denominator = mul_mod(2, p1.y, curve.p);
        if denominator == 0 {
            return None;
        }
        mul_mod(numerator, inv_mod(denominator, curve.p), curve.p)
    } else {
        mul_mod(
            sub_mod(p2.y, p1.y, curve.p),
            inv_mod(sub_mod(p2.x, p1.x, curve.p), curve.p),
            curve.p,
        )
    };
    let x3 = sub_mod(
        sub_mod(mul_mod(slope, slope, curve.p), p1.x, curve.p),
        p2.x,
        curve.p,
    );
    let y3 = sub_mod(
        mul_mod(slope, sub_mod(p1.x, x3, curve.p), curve.p),
        p1.y,
        curve.p,
    );
    let result = Point { x: x3, y: y3 };
    assert!(on_curve(curve, result));
    Some(result)
}

fn scalar_mul(curve: Curve, mut scalar: u64, mut point: Option<Point>) -> Option<Point> {
    let mut out = None;
    while scalar != 0 {
        if scalar & 1 == 1 {
            out = add(curve, out, point);
        }
        point = add(curve, point, point);
        scalar >>= 1;
    }
    out
}

fn multiplicative_order(a: u64, modulus: u64) -> u64 {
    let mut value = 1;
    for d in 1..modulus {
        value = mul_mod(value, a % modulus, modulus);
        if value == 1 {
            return d;
        }
    }
    panic!("no multiplicative order");
}

fn checked_pow(mut base: u64, mut exponent: u64) -> u64 {
    let mut out = 1_u64;
    while exponent != 0 {
        if exponent & 1 == 1 {
            out = out.checked_mul(base).expect("extension size overflow");
        }
        exponent >>= 1;
        if exponent != 0 {
            base = base.checked_mul(base).expect("extension size overflow");
        }
    }
    out
}

fn verify(curve: Curve) {
    assert!(prime(curve.p));
    assert!(prime(curve.r));
    assert!(curve.p >= 5);
    let discriminant_term = add_mod(
        mul_mod(
            4,
            mul_mod(mul_mod(curve.a, curve.a, curve.p), curve.a, curve.p),
            curve.p,
        ),
        mul_mod(27 % curve.p, mul_mod(curve.b, curve.b, curve.p), curve.p),
        curve.p,
    );
    assert_ne!(discriminant_term, 0);

    let affine = points(curve);
    assert_eq!(affine.len() as u64 + 1, curve.r);
    assert_eq!(curve.p as i64 + 1 - curve.r as i64, curve.trace);
    assert!(curve.trace * curve.trace <= 4 * curve.p as i64);
    assert_eq!(multiplicative_order(curve.p % curve.r, curve.r), curve.d);
    assert_eq!(checked_pow(curve.p, curve.d), curve.extension_size);
    assert!(curve.extension_size <= 1_000_000);

    let base = *affine.first().expect("nonempty affine group");
    for point in &affine {
        assert!(on_curve(curve, *point));
        assert_eq!(scalar_mul(curve, curve.r, Some(*point)), None);
    }

    let lambda = 2 % curve.p;
    let lambda2 = mul_mod(lambda, lambda, curve.p);
    let lambda3 = mul_mod(lambda2, lambda, curve.p);
    let iso = Curve {
        a: mul_mod(curve.a, mul_mod(lambda2, lambda2, curve.p), curve.p),
        b: mul_mod(curve.b, mul_mod(lambda3, lambda3, curve.p), curve.p),
        ..curve
    };
    let mapped = Point {
        x: mul_mod(lambda2, base.x, curve.p),
        y: mul_mod(lambda3, base.y, curve.p),
    };
    assert!(on_curve(iso, mapped));
    assert_eq!(points(iso).len() as u64 + 1, curve.r);
    assert_eq!(scalar_mul(iso, curve.r, Some(mapped)), None);

    println!(
        "{} p={} r={} t={} d={} field={} base=({}, {}) affine={} iso_base=({}, {})",
        curve.slug,
        curve.p,
        curve.r,
        curve.trace,
        curve.d,
        curve.extension_size,
        base.x,
        base.y,
        affine.len(),
        mapped.x,
        mapped.y
    );
}

fn main() {
    for curve in CURVES {
        verify(curve);
    }
    let targets: u64 = CURVES.iter().map(|curve| curve.r - 1).sum::<u64>() * 3;
    assert_eq!(targets, 366);
    println!(
        "fixture-check ok curves={} rows={} targets={} curves_sha256={}",
        CURVES.len(),
        CURVES.len() * 3,
        targets,
        CURVES_SHA256
    );
}
