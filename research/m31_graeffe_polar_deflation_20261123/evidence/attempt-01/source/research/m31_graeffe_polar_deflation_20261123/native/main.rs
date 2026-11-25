//! Fixed M31 development checks for total polar derivatives of a Graeffe circuit.
//!
//! This program evaluates supplied, deterministic witnesses. It does not find
//! relations, invoke a polynomial solver, enumerate supports, or measure speed.
//! Taylor coefficients are ordinary coefficients, not divided differences:
//! conversion to actual partial derivatives includes every factorial.

use prime31_algebraic_ic::{check_curve, Arithmetic, Base, PRIME31};
use serde_json::{json, Value};

const P: u64 = 2_147_483_647;
const EXPS: [[u8; 3]; 20] = [
    [0, 0, 0],
    [1, 0, 0], [0, 1, 0], [0, 0, 1],
    [2, 0, 0], [1, 1, 0], [1, 0, 1], [0, 2, 0], [0, 1, 1], [0, 0, 2],
    [3, 0, 0], [2, 1, 0], [2, 0, 1], [1, 2, 0], [1, 1, 1],
    [1, 0, 2], [0, 3, 0], [0, 2, 1], [0, 1, 2], [0, 0, 3],
];
const HESSIAN_PAIRS: [(usize, usize); 6] = [(0, 0), (0, 1), (0, 2), (1, 1), (1, 2), (2, 2)];

fn add(a: u64, b: u64) -> u64 { (a + b) % P }
fn sub(a: u64, b: u64) -> u64 { (a + P - b) % P }
fn neg(a: u64) -> u64 { (P - a) % P }
fn mul(a: u64, b: u64) -> u64 { ((a as u128 * b as u128) % P as u128) as u64 }
fn pow(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1;
    while e != 0 {
        if e & 1 != 0 { r = mul(r, a); }
        a = mul(a, a);
        e >>= 1;
    }
    r
}
fn inv(a: u64) -> u64 {
    assert_ne!(a, 0, "an independently checked unit is required");
    pow(a, P - 2)
}
fn need(condition: bool, message: &str) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message.to_owned()) }
}
fn exponent_index(e: [u8; 3]) -> usize {
    EXPS.iter().position(|&v| v == e).expect("degree at most three")
}
fn degree(e: [u8; 3]) -> u8 { e[0] + e[1] + e[2] }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Jet { c: [u64; 20] }

impl Jet {
    fn zero() -> Self { Self { c: [0; 20] } }
    fn constant(a: u64) -> Self {
        let mut r = Self::zero();
        r.c[0] = a % P;
        r
    }
    fn variable(a: u64, coordinate: usize) -> Self {
        let mut r = Self::constant(a);
        let mut e = [0; 3];
        e[coordinate] = 1;
        r.c[exponent_index(e)] = 1;
        r
    }
    fn add(self, rhs: Self) -> Self {
        let mut r = Self::zero();
        for (i, v) in r.c.iter_mut().enumerate() { *v = add(self.c[i], rhs.c[i]); }
        r
    }
    fn sub(self, rhs: Self) -> Self {
        let mut r = Self::zero();
        for (i, v) in r.c.iter_mut().enumerate() { *v = sub(self.c[i], rhs.c[i]); }
        r
    }
    fn scale(self, scalar: u64) -> Self {
        let mut r = Self::zero();
        for (i, v) in r.c.iter_mut().enumerate() { *v = mul(self.c[i], scalar); }
        r
    }
    fn mul(self, rhs: Self) -> Self {
        let mut r = Self::zero();
        for (i, &a) in self.c.iter().enumerate() {
            if a == 0 { continue; }
            for (j, &b) in rhs.c.iter().enumerate() {
                if b == 0 { continue; }
                let e = [EXPS[i][0] + EXPS[j][0], EXPS[i][1] + EXPS[j][1], EXPS[i][2] + EXPS[j][2]];
                if degree(e) <= 3 {
                    let k = exponent_index(e);
                    r.c[k] = add(r.c[k], mul(a, b));
                }
            }
        }
        r
    }
    fn square(self) -> Self { self.mul(self) }
    fn partial(self, e: [u8; 3]) -> u64 {
        let mut factor = 1;
        for n in e {
            factor = mul(factor, match n { 0 | 1 => 1, 2 => 2, 3 => 6, _ => unreachable!() });
        }
        mul(self.c[exponent_index(e)], factor)
    }
    fn gradient(self) -> [u64; 3] {
        std::array::from_fn(|i| { let mut e = [0; 3]; e[i] = 1; self.partial(e) })
    }
    fn hessian(self) -> [[u64; 3]; 3] {
        std::array::from_fn(|i| std::array::from_fn(|j| {
            let mut e = [0; 3]; e[i] += 1; e[j] += 1; self.partial(e)
        }))
    }
    fn hessian_values(self) -> [u64; 6] {
        std::array::from_fn(|k| {
            let (i, j) = HESSIAN_PAIRS[k];
            let mut e = [0; 3]; e[i] += 1; e[j] += 1; self.partial(e)
        })
    }
    fn hessian_jacobian(self) -> [[u64; 3]; 6] {
        std::array::from_fn(|k| std::array::from_fn(|l| {
            let (i, j) = HESSIAN_PAIRS[k];
            let mut e = [0; 3]; e[i] += 1; e[j] += 1; e[l] += 1; self.partial(e)
        }))
    }
}

fn rank(rows: &[[u64; 3]]) -> usize {
    let mut a = rows.to_vec();
    let mut next = 0;
    for column in 0..3 {
        let Some(pivot) = (next..a.len()).find(|&i| a[i][column] != 0) else { continue; };
        a.swap(next, pivot);
        let divisor = inv(a[next][column]);
        for k in column..3 { a[next][k] = mul(a[next][k], divisor); }
        let pivot_row = a[next];
        for (i, row) in a.iter_mut().enumerate() {
            if i == next { continue; }
            let scalar = row[column];
            for k in column..3 { row[k] = sub(row[k], mul(scalar, pivot_row[k])); }
        }
        next += 1;
        if next == 3 { break; }
    }
    next
}

fn poly_product(a: &[Jet], b: &[Jet]) -> Vec<Jet> {
    let mut out = vec![Jet::zero(); a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() { out[i + j] = out[i + j].add(x.mul(y)); }
    }
    out
}
fn poly_eval(a: &[Jet], x: Jet) -> Jet {
    a.iter().rev().fold(Jet::zero(), |acc, &c| acc.mul(x).add(c))
}

/// Independent norm assembly, using polynomial products rather than expanded
/// quartic formulas or the Graeffe coefficient map.
fn rr5_quartic(parameters: [u64; 3], target: (u64, u64)) -> [Jet; 5] {
    let [b, c, d] = std::array::from_fn(|i| Jet::variable(parameters[i], i));
    let (r, s) = target;
    let v = [b, Jet::constant(1)];
    let l = [d, c];
    let dr = [Jet::constant(add(mul(r, r), PRIME31.a)), Jet::constant(r), Jet::constant(1)];
    let mut u = poly_product(&dr, &poly_product(&v, &v));
    let ll = poly_product(&[Jet::constant(neg(r)), Jet::constant(1)], &poly_product(&l, &l));
    let lv = poly_product(&l, &v);
    for (i, &value) in ll.iter().enumerate() { u[i] = u[i].sub(value); }
    for (i, &value) in lv.iter().enumerate() { u[i] = u[i].sub(value.scale(add(s, s))); }
    u.try_into().expect("monic quartic norm inventory")
}

/// Characteristic polynomial of x_i^2-2, in ascending monic coefficients.
/// Derived independently from E(T+2)^2-(T+2)O(T+2)^2 for U(X)=E(X^2)+XO(X^2).
fn graeffe(u: [Jet; 5]) -> [Jet; 5] {
    let [a0, a1, a2, a3, leading] = u;
    assert_eq!(leading, Jet::constant(1));
    let w3 = a2.scale(2).sub(a3.square());
    let w2 = a2.square().add(a0.scale(2)).sub(a3.mul(a1).scale(2));
    let w1 = a2.mul(a0).scale(2).sub(a1.square());
    let w0 = a0.square();
    [
        Jet::constant(16).add(w3.scale(8)).add(w2.scale(4)).add(w1.scale(2)).add(w0),
        Jet::constant(32).add(w3.scale(12)).add(w2.scale(4)).add(w1),
        Jet::constant(24).add(w3.scale(6)).add(w2),
        Jet::constant(8).add(w3),
        Jet::constant(1),
    ]
}

/// e_k(D_1024(x_i)-tau), for k=1,2,3,4. The third residual is
/// -g1-2*tau*g2-3*tau^2*g3-4*tau^3, not an uncentered coefficient.
fn centered(g: [Jet; 5], tau: u64) -> [Jet; 4] {
    let t2 = mul(tau, tau);
    let t3 = mul(t2, tau);
    let t4 = mul(t3, tau);
    [
        g[3].scale(P - 1).sub(Jet::constant(mul(4, tau))),
        g[2].add(g[3].scale(mul(3, tau))).add(Jet::constant(mul(6, t2))),
        g[1].scale(P - 1).sub(g[2].scale(mul(2, tau))).sub(g[3].scale(mul(3, t2))).sub(Jet::constant(mul(4, t3))),
        g[0].add(g[1].scale(tau)).add(g[2].scale(t2)).add(g[3].scale(t3)).add(Jet::constant(t4)),
    ]
}

fn elementary(roots: &[Jet; 4]) -> [Jet; 4] {
    let mut e = [Jet::constant(1), Jet::zero(), Jet::zero(), Jet::zero(), Jet::zero()];
    for (i, &x) in roots.iter().enumerate() {
        for j in (1..=i + 1).rev() { e[j] = e[j].add(e[j - 1].mul(x)); }
    }
    [e[1], e[2], e[3], e[4]]
}

fn dickson(mut x: u64) -> u64 {
    for _ in 0..10 { x = sub(mul(x, x), 2); }
    x
}
fn dickson_derivative(mut x: u64) -> u64 {
    let mut derivative = 1;
    for _ in 0..10 {
        derivative = mul(mul(2, x), derivative);
        x = sub(mul(x, x), 2);
    }
    derivative
}
fn dickson_jet(mut x: Jet) -> Jet {
    for _ in 0..10 { x = x.square().sub(Jet::constant(2)); }
    x
}

/// Independent formal implicit-function solve, degree by degree. Its only
/// division is by the separately checked scalar U'(x_i), never by a jet.
fn root_jet(u: &[Jet; 5], x: u64, derivative: u64) -> Result<Jet, String> {
    need(derivative != 0, "root lift has zero U prime")?;
    let mut out = Jet::constant(x);
    need(poly_eval(u, out).c[0] == 0, "root lift does not start at a quartic root")?;
    let reciprocal = inv(derivative);
    for d in 1..=3 {
        let residual = poly_eval(u, out);
        for (i, &e) in EXPS.iter().enumerate() {
            if degree(e) == d { out.c[i] = neg(mul(residual.c[i], reciprocal)); }
        }
    }
    need(poly_eval(u, out) == Jet::zero(), "implicit root jet fails the full norm through order three")?;
    Ok(out)
}

fn interpolate(points: &[(u64, u64); 4], target: (u64, u64)) -> Result<[u64; 3], String> {
    let (r, s) = target;
    let mut rows: Vec<[u64; 4]> = points.iter().map(|&(x, y)| {
        let sy = add(s, y);
        let xr = sub(x, r);
        [sy, mul(xr, x), xr, neg(mul(sy, x))]
    }).collect();
    let mut next = 0;
    for column in 0..3 {
        let pivot = (next..rows.len()).find(|&i| rows[i][column] != 0)
            .ok_or_else(|| "RR5 interpolation has rank below three".to_string())?;
        rows.swap(next, pivot);
        let reciprocal = inv(rows[next][column]);
        for k in column..4 { rows[next][k] = mul(rows[next][k], reciprocal); }
        let pivot_row = rows[next];
        for (i, row) in rows.iter_mut().enumerate() {
            if i == next { continue; }
            let scalar = row[column];
            for k in column..4 { row[k] = sub(row[k], mul(scalar, pivot_row[k])); }
        }
        next += 1;
    }
    need(rows.iter().all(|r| r[..3].iter().any(|&x| x != 0) || r[3] == 0), "fourth RR5 interpolation row is inconsistent")?;
    let parameters = [rows[0][3], rows[1][3], rows[2][3]];
    let [b, c, d] = parameters;
    for &(x, y) in points {
        let value = add(mul(sub(x, r), add(mul(c, x), d)), mul(add(s, y), add(x, b)));
        need(value == 0, "interpolated RR5 function misses a supplied signed point")?;
    }
    Ok(parameters)
}

/// Direct derivatives of the explicit RR5 coefficient formulas, ascending.
/// This path is independent of polynomial convolution and jet multiplication.
fn coefficient_derivatives(parameters: [u64; 3], target: (u64, u64)) -> [[u64; 3]; 4] {
    let [b, c, d] = parameters;
    let (r, s) = target;
    let ra = add(mul(r, r), PRIME31.a);
    [
        [sub(mul(mul(2, ra), b), mul(mul(2, s), d)), 0, sub(mul(mul(2, r), d), mul(mul(2, s), b))],
        [sub(add(mul(mul(2, r), b), mul(2, ra)), mul(mul(2, s), c)), sub(mul(mul(2, r), d), mul(mul(2, s), b)), sub(sub(mul(mul(2, r), c), mul(2, d)), mul(2, s))],
        [mul(2, add(b, r)), sub(sub(mul(mul(2, r), c), mul(2, d)), mul(2, s)), neg(mul(2, c))],
        [2, neg(mul(2, c)), 0],
    ]
}

fn f2_rank(f: &[Jet; 4]) -> usize {
    let mut rows = vec![f[0].gradient()];
    rows.extend(f[1].hessian());
    rank(&rows)
}

fn check_case(index: usize, base: &Base, arithmetic: &mut Arithmetic) -> Result<Value, String> {
    let indices = [4 * index, 4 * index + 1, 4 * index + 2, 4 * index + 3];
    let points: [(u64, u64); 4] = std::array::from_fn(|j| {
        let (x, y) = base.points[indices[j]];
        (x, if (index >> j) & 1 != 0 { neg(y) } else { y })
    });
    let mut target = None;
    for &point in &points {
        need(arithmetic.on_curve(PRIME31, Some(point)), "supplied signed leaf is not on the pinned curve")?;
        target = arithmetic.add(PRIME31, target, Some(point));
    }
    let target = target.ok_or_else(|| "frozen quartet gives the identity target; no replacement is allowed".to_string())?;
    need(arithmetic.on_curve(PRIME31, Some(target)), "target is not a canonical curve point")?;
    need(target.1 != 0 && dickson(target.0) != base.terminal, "frozen quartet leaves the regular target chart; no replacement is allowed")?;
    let parameters = interpolate(&points, target)?;
    let u = rr5_quartic(parameters, target);
    let mut product = vec![Jet::constant(1)];
    for &(x, _) in &points { product = poly_product(&product, &[Jet::constant(neg(x)), Jet::constant(1)]); }
    need((0..5).all(|j| u[j].c[0] == product[j].c[0]), "RR5 quartic disagrees with the independent four-factor product")?;
    let derivatives = coefficient_derivatives(parameters, target);
    need((0..4).all(|j| u[j].gradient() == derivatives[j]), "RR5 coefficient derivative formulas disagree with norm jets")?;
    let mut g = u;
    for _ in 0..10 { g = graeffe(g); }
    let f = centered(g, base.terminal);
    need(f.iter().all(|v| v.c[0] == 0), "collapsed Graeffe terminal residual is nonzero")?;
    let original_collapsed_rank = rank(&f.map(|residual| residual.gradient()));
    need(original_collapsed_rank <= 1, "original collapsed residual Jacobian exceeds the stated rank bound")?;

    let mut z = [Jet::zero(); 4];
    let mut tangent = [[0u64; 3]; 4];
    let mut normal = [0u64; 4];
    for i in 0..4 {
        let (x, y) = points[i];
        need(y != 0 && dickson(x) == base.terminal, "supplied point violates Dickson or nonzero-y admission")?;
        need(add(x, parameters[0]) != 0, "RR5 denominator vanishes at a supplied leaf")?;
        let mut up = 1;
        for (j, &(other, _)) in points.iter().enumerate() {
            if j != i { up = mul(up, sub(x, other)); }
        }
        need(up != 0, "frozen quartet has repeated abscissae")?;
        let dp = dickson_derivative(x);
        need(dp != 0, "Dickson derivative vanishes at a supplied leaf")?;
        z[i] = dickson_jet(root_jet(&u, x, up)?).sub(Jet::constant(base.terminal));
        for column in 0..3 {
            let du = derivatives.iter().rev().fold(0, |acc, row| add(mul(acc, x), row[column]));
            tangent[i][column] = neg(mul(mul(dp, du), inv(up)));
        }
        need(z[i].gradient() == tangent[i], "root-jet tangent disagrees with implicit differentiation")?;
        normal[i] = inv(mul(mul(2, y), dp));
    }
    need(rank(&tangent) == 3, "RR5 leaf-residual tangent does not have rank three")?;
    for column in 0..3 {
        let dot = (0..4).fold(0, |acc, i| add(acc, mul(normal[i], tangent[i][column])));
        need(dot == 0, "Abel tangent normal does not annihilate the reconstructed tangent")?;
    }
    let independent = elementary(&z);
    need(f == independent, "Graeffe and implicit-root paths disagree in a centered jet coefficient")?;
    need(f[1].gradient() == [0; 3], "total gradient of centered F2 does not vanish")?;
    need(f[2].hessian_values() == [0; 6], "total Hessian of centered F3 does not vanish")?;
    let hessian_jacobian = f[2].hessian_jacobian();
    let f3_rank = rank(&hessian_jacobian);
    need(f3_rank == 3, "the six F3 polar equations do not have Jacobian rank three")?;
    let normal_sum = normal.iter().copied().fold(0, add);
    let normal_square_sum = normal.iter().copied().map(|a| mul(a, a)).fold(0, add);
    let expected_f2_rank = if normal_sum == 0 && normal_square_sum == 0 { 2 } else { 3 };
    let actual_f2_rank = f2_rank(&f);
    need(actual_f2_rank == expected_f2_rank, "F1 plus gradient-F2 rank disagrees with the normal criterion")?;

    // Wrong ambient differentiation: terminal coefficients are treated as
    // independent variables instead of total functions of b,c,d. F3 is then
    // affine, so its six second derivatives have a zero Jacobian.
    let ambient = Jet::variable(g[1].c[0], 0).scale(P - 1)
        .sub(Jet::variable(g[2].c[0], 1).scale(mul(2, base.terminal)))
        .sub(Jet::variable(g[3].c[0], 2).scale(mul(3, mul(base.terminal, base.terminal))))
        .sub(Jet::constant(mul(4, pow(base.terminal, 3))));
    need(rank(&ambient.hessian_jacobian()) == 0, "ambient negative control failed to expose missing total derivatives")?;
    let shifted = centered(g, add(base.terminal, 1));
    need(shifted[2].c[0] == P - 4 && shifted[2] != independent[2], "wrong-centering control was not detected")?;

    Ok(json!({
        "case": index, "status": "pass", "base_indices_zero_based": indices,
        "negative_sign_mask": index, "signed_points": points, "public_target": target,
        "rr5_parameters_b_c_d": parameters,
        "quartic_ascending": u.map(|v| v.c[0]), "normal": normal,
        "normal_sum": normal_sum, "normal_square_sum": normal_square_sum,
        "leaf_tangent": tangent, "leaf_tangent_rank": 3,
        "centered_jet_coefficients": f.map(|v| v.c),
        "independent_centered_jet_coefficients": independent.map(|v| v.c),
        "original_collapsed_rank": original_collapsed_rank,
        "f2_plus_f1_rank": actual_f2_rank, "f3_hessian_jacobian": hessian_jacobian,
        "f3_hessian_jacobian_rank": f3_rank,
        "ambient_derivative_rank": 0, "wrong_center_f3_value": shifted[2].c[0],
        "all_80_centered_coefficients_equal": true
    }))
}

/// Exact synthetic M31 normal (1,5,c,-6-c), where c^2+6c+31=0.
/// The discriminant is -88; no parameter search or replacement is performed.
fn exceptional_normal(arithmetic: &mut Arithmetic) -> Result<[u64; 4], String> {
    let root = arithmetic.sqrt(P - 88, P)
        .ok_or_else(|| "the frozen synthetic discriminant -88 has no M31 square root".to_string())?;
    let root = root.min(neg(root));
    let c = mul(sub(root, 6), inv(2));
    let n = [1, 5, c, neg(add(6, c))];
    need(n.iter().all(|&x| x != 0), "the frozen synthetic normal has a zero coordinate")?;
    need(add(add(mul(c, c), mul(6, c)), 31) == 0, "synthetic c fails its frozen quadratic")?;
    need(n.iter().copied().fold(0, add) == 0, "synthetic normal does not sum to zero")?;
    need(n.iter().copied().map(|x| mul(x, x)).fold(0, add) == 0, "synthetic normal is not isotropic")?;
    Ok(n)
}

fn synthetic_case(name: &str, n: [u64; 4], expected_f3: usize) -> Result<Value, String> {
    need(n[3] != 0, "synthetic hyperplane needs a nonzero fourth normal coordinate")?;
    let mut z = [Jet::zero(); 4];
    for i in 0..3 { z[i] = Jet::variable(0, i); }
    for i in 0..3 { z[3] = z[3].sub(z[i].scale(mul(n[i], inv(n[3])))); }
    let f = elementary(&z);
    let hessian = f[2].hessian_jacobian();
    let actual_f3 = rank(&hessian);
    need(actual_f3 == expected_f3, "synthetic F3 polar rank differs from the declared control")?;
    need(f[1].gradient() == [0; 3] && f[2].hessian_values() == [0; 6], "synthetic polar equations do not vanish at the origin")?;
    let sum = n.iter().copied().fold(0, add);
    let squares = n.iter().copied().map(|x| mul(x, x)).fold(0, add);
    let expected_f2 = if sum == 0 && squares == 0 { 2 } else { 3 };
    let actual_f2 = f2_rank(&f);
    need(actual_f2 == expected_f2, "synthetic F2 rank criterion failed")?;
    Ok(json!({
        "name": name, "status": "pass", "normal": n,
        "normal_sum": sum, "normal_square_sum": squares,
        "normal_all_nonzero": n.iter().all(|&x| x != 0),
        "f2_plus_f1_rank": actual_f2,
        "f3_hessian_jacobian": hessian, "f3_hessian_jacobian_rank": actual_f3
    }))
}

fn check() -> Result<Value, String> {
    need(
        (PRIME31.p, PRIME31.a, PRIME31.b, PRIME31.n, PRIME31.g)
            == (P, 847_514_846, 2_087_444_724, 2_147_446_831, (625_037_715, 2_100_627_251)),
        "the fixed original M31 curve or generator changed",
    )?;
    let mut arithmetic = Arithmetic::default();
    check_curve(PRIME31, &mut arithmetic)?;
    let base = Base::build(PRIME31, 10, &mut arithmetic)?;
    need(base.values.len() == 1024 && base.points.len() == 509, "original depth-ten B509 inventory changed")?;
    need(base.terminal != 2 && base.terminal != P - 2, "exceptional Dickson terminal")?;
    let mut all_pass = true;
    let mut cases = Vec::new();
    for index in 0..16 {
        match check_case(index, &base, &mut arithmetic) {
            Ok(value) => cases.push(value),
            Err(error) => { all_pass = false; cases.push(json!({"case":index,"status":"fail","error":error})); }
        }
    }
    let mut synthetic = Vec::new();
    let mut normals = vec![
        ("balanced_nonzero", [1, 1, 1, 1], 3),
        ("generic_nonzero", [1, 2, 3, 4], 3),
        ("zero_normal_coordinates_out_of_scope", [1, 0, 0, 1], 2),
    ];
    match exceptional_normal(&mut arithmetic) {
        Ok(n) => normals.push(("f2_rank_failure_nonzero_normal", n, 3)),
        Err(error) => { all_pass = false; synthetic.push(json!({"name":"f2_rank_failure_nonzero_normal","status":"fail","error":error})); }
    }
    for (name, normal, expected) in normals {
        match synthetic_case(name, normal, expected) {
            Ok(value) => synthetic.push(value),
            Err(error) => { all_pass = false; synthetic.push(json!({"name":name,"status":"fail","error":error})); }
        }
    }
    Ok(json!({
        "schema": "m31-graeffe-polar-development/v1",
        "scope": "fixed supplied-witness Taylor and tangent checks; no extraction, F4 solve, or performance claim",
        "standing": "pcc:unverified", "all_pass": all_pass,
        "curve": PRIME31, "dickson_depth":10, "dickson_terminal":base.terminal,
        "raw_dickson_roots":base.values.len(), "curve_base_points":base.points.len(),
        "jet_monomials":EXPS, "hessian_coordinate_pairs":HESSIAN_PAIRS,
        "frozen_case_count":16, "case_replacement_allowed":false,
        "cases":cases, "synthetic_controls":synthetic,
        "synthetic_exception_fixed_b":5,
        "synthetic_exception_discriminant":P-88,
        "synthetic_exception_square_root_choice":"smaller canonical root",
        "original_fixture_arithmetic_counts":arithmetic.counts
    }))
}

fn main() {
    let report = match check() {
        Ok(report) => report,
        Err(error) => json!({"schema":"m31-graeffe-polar-development/v1","all_pass":false,"setup_error":error}),
    };
    println!("{}", serde_json::to_string(&report).expect("serialize finite development report"));
    if report["all_pass"] != true { std::process::exit(1); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jets_preserve_mixed_and_repeated_derivative_factors() {
        let x = Jet::variable(0, 0);
        let y = Jet::variable(0, 1);
        let z = Jet::variable(0, 2);
        let linear = x.add(y.scale(2)).add(z.scale(3));
        let cube = linear.square().mul(linear);
        let weights = [1, 2, 3];
        for i in 0..3 { for j in 0..3 { for k in 0..3 {
            let mut e = [0; 3]; e[i] += 1; e[j] += 1; e[k] += 1;
            assert_eq!(cube.partial(e), 6 * weights[i] * weights[j] * weights[k]);
        } } }
        assert_eq!(cube.hessian_values(), [0; 6]);
    }

    #[test]
    fn graeffe_matches_independent_root_product_on_arbitrary_m31_jets() {
        let roots = [Jet::variable(3, 0), Jet::variable(5, 1), Jet::variable(7, 2), Jet::constant(11)];
        let mut input = vec![Jet::constant(1)];
        let mut expected = vec![Jet::constant(1)];
        for x in roots {
            input = poly_product(&input, &[x.scale(P - 1), Jet::constant(1)]);
            expected = poly_product(&expected, &[x.square().sub(Jet::constant(2)).scale(P - 1), Jet::constant(1)]);
        }
        assert_eq!(graeffe(input.try_into().unwrap()).to_vec(), expected);
    }

    #[test]
    fn implicit_lift_recovers_a_prescribed_nonlinear_root_through_order_three() {
        let x = Jet::variable(0, 0);
        let y = Jet::variable(0, 1);
        let z = Jet::variable(0, 2);
        let known_root = Jet::constant(3).add(x).add(y.square()).add(x.mul(y).mul(z));
        let mut polynomial = vec![Jet::constant(1)];
        for root in [known_root, Jet::constant(5), Jet::constant(7), Jet::constant(11)] {
            polynomial = poly_product(&polynomial, &[root.scale(P - 1), Jet::constant(1)]);
        }
        let quartic: [Jet; 5] = polynomial.try_into().unwrap();
        assert_eq!(root_jet(&quartic, 3, P - 64).unwrap(), known_root);
    }

    #[test]
    fn rank_deficient_normal_control_is_not_mistaken_for_regular_scope() {
        let report = synthetic_case("outside", [1, 0, 0, 1], 2).unwrap();
        assert_eq!(report["normal_all_nonzero"], false);
        assert_eq!(report["f3_hessian_jacobian_rank"], 2);
    }
}
