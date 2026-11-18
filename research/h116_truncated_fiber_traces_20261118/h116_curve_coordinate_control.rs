use std::fmt::Write;

const PRECISION: usize = 24;

#[derive(Clone, Copy)]
struct Fixture {
    slug: &'static str,
    p: u64,
    a: u64,
    b: u64,
    trace: i64,
}

const FIXTURES: [Fixture; 5] = [
    Fixture { slug: "fp31-r37", p: 31, a: 1, b: 19, trace: -5 },
    Fixture { slug: "fp211-r197", p: 211, a: 2, b: 24, trace: 15 },
    Fixture { slug: "fp241-r257", p: 241, a: 2, b: 4, trace: -15 },
    Fixture { slug: "fp431-r397", p: 431, a: 1, b: 225, trace: 35 },
    Fixture { slug: "fp587-r547", p: 587, a: 1, b: 342, trace: 41 },
];

#[derive(Default)]
struct Counter { additions: u64, multiplications: u64, inversions: u64 }

fn add_mod(x: u64, y: u64, p: u64, c: &mut Counter) -> u64 {
    c.additions += 1;
    (x + y) % p
}

fn mul_mod(x: u64, y: u64, p: u64, c: &mut Counter) -> u64 {
    c.multiplications += 1;
    x * y % p
}

fn pow_mod(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut out = 1;
    while n > 0 {
        if n & 1 == 1 { out = out * x % p; }
        x = x * x % p;
        n >>= 1;
    }
    out
}

fn inv_mod(x: u64, p: u64, c: &mut Counter) -> u64 {
    assert!(x % p != 0);
    c.inversions += 1;
    pow_mod(x, p - 2, p)
}

fn add_series(x: &[u64], y: &[u64], p: u64, c: &mut Counter) -> Vec<u64> {
    (0..PRECISION).map(|i| add_mod(x[i], y[i], p, c)).collect()
}

fn mul_series(x: &[u64], y: &[u64], p: u64, c: &mut Counter) -> Vec<u64> {
    let mut out = vec![0; PRECISION];
    for i in 0..PRECISION {
        for j in 0..(PRECISION - i) {
            let term = mul_mod(x[i], y[j], p, c);
            out[i + j] = add_mod(out[i + j], term, p, c);
        }
    }
    out
}

fn scale_series(x: &[u64], scalar: u64, p: u64, c: &mut Counter) -> Vec<u64> {
    x.iter().map(|&v| mul_mod(v, scalar, p, c)).collect()
}

fn shift_scale(x: &[u64], shift: usize, scalar: u64, p: u64, c: &mut Counter) -> Vec<u64> {
    let mut out = vec![0; PRECISION];
    for i in 0..(PRECISION - shift) {
        out[i + shift] = mul_mod(x[i], scalar, p, c);
    }
    out
}

fn inverse_series(x: &[u64], p: u64, c: &mut Counter) -> Vec<u64> {
    let mut out = vec![0; PRECISION];
    out[0] = inv_mod(x[0], p, c);
    for n in 1..PRECISION {
        let mut total = 0;
        for i in 1..=n {
            let term = mul_mod(x[i], out[n - i], p, c);
            total = add_mod(total, term, p, c);
        }
        out[n] = (p - mul_mod(out[0], total, p, c)) % p;
    }
    out
}

fn compose(outer: &[u64], inner: &[u64], p: u64, c: &mut Counter) -> Vec<u64> {
    assert_eq!(inner[0], 0);
    let mut out = vec![0; PRECISION];
    let mut power = vec![0; PRECISION];
    power[0] = 1;
    for &coefficient in outer {
        out = add_series(&out, &scale_series(&power, coefficient, p, c), p, c);
        power = mul_series(&power, inner, p, c);
    }
    out
}

fn curve_q_fixed_point(a: u64, b: u64, p: u64, c: &mut Counter) -> Vec<u64> {
    let mut q = vec![0; PRECISION];
    q[0] = 1;
    for _ in 0..PRECISION {
        let q2 = mul_series(&q, &q, p, c);
        let q3 = mul_series(&q2, &q, p, c);
        let mut next = vec![0; PRECISION];
        next[0] = 1;
        next = add_series(&next, &shift_scale(&q2, 4, a, p, c), p, c);
        next = add_series(&next, &shift_scale(&q3, 6, b, p, c), p, c);
        q = next;
    }
    q
}

fn curve_identity(q: &[u64], a: u64, b: u64, p: u64, c: &mut Counter) -> bool {
    let q2 = mul_series(q, q, p, c);
    let q3 = mul_series(&q2, q, p, c);
    let mut rhs = vec![0; PRECISION];
    rhs[0] = 1;
    rhs = add_series(&rhs, &shift_scale(&q2, 4, a, p, c), p, c);
    rhs = add_series(&rhs, &shift_scale(&q3, 6, b, p, c), p, c);
    if q != rhs { return false; }
    let q_inv = inverse_series(q, p, c);
    let mut cleared = q_inv;
    cleared = add_series(&cleared, &shift_scale(q, 4, a, p, c), p, c);
    cleared = add_series(&cleared, &shift_scale(&q2, 6, b, p, c), p, c);
    cleared[0] == 1 && cleared[1..].iter().all(|&x| x == 0)
}

fn rescale_series(q: &[u64], scale: u64, p: u64, c: &mut Counter) -> Vec<u64> {
    let mut power = 1;
    let mut out = vec![0; PRECISION];
    for i in 0..PRECISION {
        out[i] = mul_mod(q[i], power, p, c);
        power = mul_mod(power, scale, p, c);
    }
    out
}

fn normalized_parameter(p: u64) -> Vec<u64> {
    let mut z = vec![0; PRECISION];
    z[1] = 1;
    for (i, slot) in z.iter_mut().enumerate().skip(2) {
        let n = i as u64;
        *slot = (n * n + 3 * n + 1) % p;
    }
    z
}

fn first_difference(x: &[u64], y: &[u64]) -> i64 {
    x.iter().zip(y).position(|(a, b)| a != b).map_or(-1, |i| i as i64)
}

fn csv(values: &[u64]) -> String {
    let mut out = String::new();
    for (i, value) in values.iter().enumerate() {
        if i > 0 { out.push(','); }
        write!(&mut out, "{value}").unwrap();
    }
    out
}

fn main() {
    println!("schema\th116.curve-coordinate-control/v1");
    println!("slug\tp\tcurve_a\tcurve_b\ttrace\tscale\tprecision\tsource_q\ttarget_q\tz_normalized\ttarget_at_normalized\tsource_at_scalar\tcurve_identity\trescale_match\tdouble_subst_match\tratio_match\twrong_scale_first_diff\twrong_parameter_first_diff\tadditions\tmultiplications\tinversions");
    for fixture in FIXTURES {
        let mut counter = Counter::default();
        let p = fixture.p;
        let scale = fixture.trace.rem_euclid(p as i64) as u64;
        let scale = (scale + p - 1) % p;
        assert_ne!(scale, 0);
        let target_a = fixture.a * pow_mod(scale, 4, p) % p;
        let target_b = fixture.b * pow_mod(scale, 6, p) % p;
        let source_q = curve_q_fixed_point(fixture.a, fixture.b, p, &mut counter);
        let target_q = curve_q_fixed_point(target_a, target_b, p, &mut counter);
        let rescaled_q = rescale_series(&source_q, scale, p, &mut counter);
        let z_normalized = normalized_parameter(p);
        let z_scalar = scale_series(&z_normalized, scale, p, &mut counter);
        let target_at = compose(&target_q, &z_normalized, p, &mut counter);
        let source_at = compose(&source_q, &z_scalar, p, &mut counter);
        let parameter_unit: Vec<u64> = z_normalized[1..].iter().copied().chain(std::iter::once(0)).collect();
        let parameter_inv = inverse_series(&parameter_unit, p, &mut counter);
        let pole = mul_series(&parameter_inv, &parameter_inv, p, &mut counter);
        let common = mul_series(&source_q, &pole, p, &mut counter);
        let ratio_left = mul_series(&common, &inverse_series(&target_at, p, &mut counter), p, &mut counter);
        let ratio_right = mul_series(&common, &inverse_series(&source_at, p, &mut counter), p, &mut counter);

        let wrong_a = fixture.a * pow_mod(scale, 2, p) % p;
        let wrong_b = fixture.b * pow_mod(scale, 4, p) % p;
        let wrong_q = curve_q_fixed_point(wrong_a, wrong_b, p, &mut counter);
        let wrong_at = compose(&wrong_q, &z_normalized, p, &mut counter);
        let wrong_parameter = scale_series(&z_normalized, (scale + 1) % p, p, &mut counter);
        let wrong_source_at = compose(&source_q, &wrong_parameter, p, &mut counter);

        println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            fixture.slug, p, fixture.a, fixture.b, fixture.trace, scale, PRECISION,
            csv(&source_q), csv(&target_q), csv(&z_normalized), csv(&target_at), csv(&source_at),
            curve_identity(&source_q, fixture.a, fixture.b, p, &mut counter) && curve_identity(&target_q, target_a, target_b, p, &mut counter),
            target_q == rescaled_q, target_at == source_at, ratio_left == ratio_right,
            first_difference(&wrong_at, &source_at), first_difference(&target_at, &wrong_source_at),
            counter.additions, counter.multiplications, counter.inversions);
    }
}
