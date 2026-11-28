mod field_f;
use field_f::*;
use std::collections::BTreeMap;
const E_INPUT: &str = include_str!("first-E-native/actual-functions.stdout");
const A_INPUT: &str = include_str!("first-A-native/actual-fibres.stdout");
const TERMS: usize = 25;
type Series = [u8; TERMS];
type Poly = BTreeMap<(usize, usize), u8>;
type IntegerPoly = BTreeMap<(usize, usize), i128>;
fn numbers(s: &str) -> Vec<usize> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
fn field<'a>(s: &'a str, prefix: &str) -> &'a str {
    s.split('\t').find_map(|s| s.strip_prefix(prefix)).unwrap()
}
fn parse_poly(s: &str, degree: usize) -> Poly {
    assert!(s.starts_with('{') && s.ends_with('}'));
    let n = numbers(s);
    assert_eq!(n.len() % 3, 0);
    let mut p = Poly::new();
    for a in n.as_chunks::<3>().0 {
        assert!(a[0] + a[1] <= degree && a[2] > 0 && a[2] < 7);
        assert!(p
            .insert((a[0], a[1]), u8::try_from(a[2]).unwrap())
            .is_none());
    }
    assert!(!p.is_empty());
    p
}
struct Functions {
    q: Poly,
    x: Poly,
    y: Poly,
    curve: Poly,
    quotient: Poly,
}
fn read_functions() -> Functions {
    assert!(E_INPUT
        .lines()
        .any(|l| l.starts_with("SUMMARY\tfull-weighted-pole-divisor=4\t14-first-jets")));
    let q = E_INPUT
        .lines()
        .find(|l| l.starts_with("ACTUAL_DENOMINATOR_CONIC\t"))
        .unwrap();
    let x = E_INPUT
        .lines()
        .find(|l| l.starts_with("ACTUAL_NUMERATOR\tdegree=4\t"))
        .unwrap();
    let y = E_INPUT
        .lines()
        .find(|l| l.starts_with("ACTUAL_NUMERATOR\tdegree=6\t"))
        .unwrap();
    let identity = E_INPUT
        .lines()
        .find(|l| l.starts_with("COMPLETE_ELLIPTIC_POLYNOMIAL_IDENTITY\t"))
        .unwrap();
    assert_eq!(field(identity, "remainder="), "{}");
    Functions {
        q: parse_poly(field(q, "terms-Y-Z="), 2),
        x: parse_poly(field(x, "terms-Y-Z="), 4),
        y: parse_poly(field(y, "terms-Y-Z="), 6),
        curve: parse_poly(field(identity, "curve="), 4),
        quotient: parse_poly(field(identity, "quotient="), 8),
    }
}
fn constant(c: u8) -> Series {
    let mut a = [0; TERMS];
    a[0] = c;
    a
}
fn product(a: &Series, b: &Series) -> Series {
    std::array::from_fn(|i| (0..=i).map(|j| mul(a[j], b[i - j])).fold(0, add))
}
fn power(a: &Series, n: usize) -> Series {
    (0..n).fold(constant(1), |r, _| product(&r, a))
}
fn evaluate_poly(p: &Poly, y: &Series, z: &Series) -> Series {
    let mut r = [0; TERMS];
    for (&(a, b), &c) in p {
        let monomial = product(&power(y, a), &power(z, b));
        for (r, &v) in r.iter_mut().zip(&monomial) {
            *r = add(*r, mul(c, v));
        }
    }
    r
}
fn actual_local_series(p: Point, curve: &Poly) -> (Series, Series) {
    assert_eq!(p[0], 1);
    let mut y = constant(p[1]);
    let mut z = constant(p[2]);
    assert_eq!(evaluate_poly(curve, &y, &z)[0], 0);
    y[1] = 1;
    let fy = evaluate_poly(curve, &y, &z)[1];
    y[1] = 0;
    z[1] = 1;
    let fz = evaluate_poly(curve, &y, &z)[1];
    z[1] = 0;
    if fy != 0 {
        z[1] = 1;
        for n in 1..TERMS {
            let error = evaluate_poly(curve, &y, &z)[n];
            y[n] = mul(sub(0, error), inv(fy));
        }
    } else {
        assert_ne!(fz, 0);
        y[1] = 1;
        for n in 1..TERMS {
            let error = evaluate_poly(curve, &y, &z)[n];
            z[n] = mul(sub(0, error), inv(fz));
        }
    }
    assert_eq!(evaluate_poly(curve, &y, &z), [0; TERMS]);
    (y, z)
}
fn order(a: &Series, bound: usize) -> usize {
    let n = a.iter().position(|a| *a != 0).unwrap();
    assert!(n <= bound);
    n
}
struct Evaluation {
    value: EPoint,
    vq: usize,
    vx: usize,
    vy: usize,
    q: Series,
    x: Series,
    y: Series,
}
fn evaluate(p: Point, f: &Functions) -> Evaluation {
    // No historical image, pole-divisor or case-index input enters this function.
    let (local_y, local_z) = actual_local_series(p, &f.curve);
    let q = evaluate_poly(&f.q, &local_y, &local_z);
    let x = evaluate_poly(&f.x, &local_y, &local_z);
    let y = evaluate_poly(&f.y, &local_y, &local_z);
    let vq = order(&q, 8);
    let vx = order(&x, 16);
    let vy = order(&y, 24);
    let value = if vx >= 2 * vq && vy >= 3 * vq {
        let xx = if vx == 2 * vq {
            mul(x[vx], inv(pow(q[vq], 2)))
        } else {
            0
        };
        let yy = if vy == 3 * vq {
            mul(y[vy], inv(pow(q[vq], 3)))
        } else {
            0
        };
        Some((xx, yy))
    } else {
        assert!(vx < 2 * vq && vy < 3 * vq);
        let ex = 2 * vq - vx;
        assert_eq!(ex % 2, 0);
        let e = ex / 2;
        assert!(e > 0);
        assert_eq!(3 * vq - vy, 3 * e);
        None
    };
    assert!(valid(value));
    Evaluation {
        value,
        vq,
        vx,
        vy,
        q,
        x,
        y,
    }
}
fn parse_e(s: &str) -> EPoint {
    if s == "None" {
        return None;
    }
    assert!(s.starts_with("Some((") && s.ends_with("))"));
    let n = numbers(s);
    assert_eq!(n.len(), 2);
    let p = Some((u8::try_from(n[0]).unwrap(), u8::try_from(n[1]).unwrap()));
    assert!(valid(p));
    p
}
fn integer_lift(p: &Poly) -> IntegerPoly {
    p.iter().map(|(&k, &v)| (k, i128::from(v))).collect()
}
fn integer_product(a: &IntegerPoly, b: &IntegerPoly) -> IntegerPoly {
    let mut r = IntegerPoly::new();
    for (&(y, z), &c) in a {
        for (&(yy, zz), &d) in b {
            *r.entry((y + yy, z + zz)).or_default() += c * d;
        }
    }
    r
}
fn integer_power(a: &IntegerPoly, n: usize) -> IntegerPoly {
    (0..n).fold(IntegerPoly::from([((0, 0), 1)]), |r, _| {
        integer_product(&r, a)
    })
}
fn integer_accumulate(a: &mut IntegerPoly, b: &IntegerPoly, c: i128) {
    for (&k, &v) in b {
        *a.entry(k).or_default() += c * v;
    }
}
fn coefficient_certificate(f: &Functions) {
    let q = integer_lift(&f.q);
    let x = integer_lift(&f.x);
    let y = integer_lift(&f.y);
    let mut lhs = integer_power(&y, 2);
    integer_accumulate(&mut lhs, &integer_power(&x, 3), -1);
    integer_accumulate(&mut lhs, &integer_product(&x, &integer_power(&q, 4)), -2);
    integer_accumulate(&mut lhs, &integer_power(&q, 6), -6);
    let rhs = integer_product(&integer_lift(&f.curve), &integer_lift(&f.quotient));
    assert!(lhs.keys().chain(rhs.keys()).all(|(a, b)| a + b <= 12));
    let mut lean=String::from("import Init\nnamespace ActualCM19CoefficientLeavesG\n/- Closed standard Int coefficient arithmetic only. Polynomial expansion and actual-geometry bridges remain explicit native/written obligations. -/\n");
    let mut count = 0;
    for a in 0..=12 {
        for b in 0..=12 - a {
            let left = lhs.get(&(a, b)).copied().unwrap_or(0);
            let right = rhs.get(&(a, b)).copied().unwrap_or(0);
            assert_eq!((left - right).rem_euclid(7), 0);
            println!("ACTUAL_INTEGER_COEFFICIENT\tY={a}\tZ={b}\tX={}\tinteger-left={left}\tinteger-right={right}\tdifference={}\tmod7=0",12-a-b,left-right);
            lean.push_str(&format!("theorem coefficient_{a}_{b} : ((({left}) - ({right}) : Int) % 7) = 0 := by decide\n#print axioms coefficient_{a}_{b}\n"));
            count += 1;
        }
    }
    assert_eq!(count, 91);
    lean.push_str("end ActualCM19CoefficientLeavesG\n");
    println!("LEAN_SOURCE_BEGIN\n{lean}LEAN_SOURCE_END");
    println!("COMPLETE_INTEGER_COEFFICIENT_CERTIFICATE\tall91-degree12-monomials\tfull-unreduced-integer-convolutions\tall-mod7-zero\tLean-leaves-generated-not-yet-compiled");
}
fn main() {
    println!("PROFILE\tactual-CM19-polynomial-exceptional-evaluation-F\tfixed-proof-correctness\tunsigned\tno-performance-claim");
    let f = read_functions();
    let target = emul(4, Some((1, 3)));
    assert_eq!(target, Some((3, 5)));
    let mut count = 0;
    let mut poles = 0;
    let mut residuals = 0;
    let mut control = None;
    for line in A_INPUT.lines().filter(|l| l.starts_with("ACTUAL_POINT\t")) {
        let n = numbers(field(line, "C="));
        assert_eq!(n.len(), 3);
        let point: Point = n
            .into_iter()
            .map(|n| u8::try_from(n).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let r = evaluate(point, &f);
        // The immutable image oracle is read only after independent evaluation.
        let recovered = eadd(r.value, target);
        let oracle = parse_e(field(line, "f4="));
        assert_eq!(recovered, oracle);
        assert_eq!(r.value, eadd(oracle, eneg(target)));
        if r.vq > 0 {
            if r.value.is_none() {
                poles += 1;
            } else {
                residuals += 1;
                if control.is_none() {
                    control = Some((r.q, r.y, r.vq, r.vy, r.value));
                }
            }
            println!("COMPLETE_EXCEPTIONAL_LOCAL_CERTIFICATE\tindex={count}\tC={point:?}\tq-series={:?}\tp4-series={:?}\tp6-series={:?}",r.q,r.x,r.y);
        }
        println!("ACTUAL_FUNCTION_EVALUATION\tindex={count}\tC={point:?}\tvaluations-q-p4-p6={:?}\tg={:?}\tf4={recovered:?}\toracle-agrees=true",(r.vq,r.vx,r.vy),r.value);
        count += 1;
    }
    assert_eq!(count, 17);
    assert_eq!(poles, 3);
    assert_eq!(residuals, 2);
    let (q, mut y, vq, vy, value) = control.unwrap();
    assert_eq!(vy, 3 * vq);
    let scale = pow(q[vq], 3);
    let pristine = y[vy];
    y[vy] = add(y[vy], scale);
    let mutant = value.map(|(x, _)| (x, mul(y[vy], inv(scale))));
    assert_ne!(mutant, value);
    println!("ACTUAL_EXCEPTIONAL_CONTROL\tfirst-residual-real-leading-y-coefficient-mutated\tpristine={value:?}\tmutant={mutant:?}\tmutant-on-E={}\tactual-image-equality-rejects=true",valid(mutant));
    y[vy] = pristine;
    assert_eq!(value.map(|(_, v)| v), Some(mul(y[vy], inv(scale))));
    coefficient_certificate(&f);
    println!("SUMMARY\tall17-images-independent-polynomial-evaluation\t3-genuine-pole-points\t2-removable-rational-conic-basepoints\torder24-exact-local-series\t91-complete-integer-coefficients\tpristine-control-restored\tfull-geometric-formalization=OPEN\tECDLP-speedup=UNMEASURED");
}
