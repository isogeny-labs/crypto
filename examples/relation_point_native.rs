//! Native-size timing of the relation point R = aG + bQ on the real E-222 and
//! M-221 (Weierstrass images of the registry curves; see
//! research/ic_e222_m221_20261106/native/native_params.py).  Three paths, all
//! returning the same point: the former default (two `scalar_mul` ladders and
//! `add`), two `scalar_mul_vartime` ladders and `add_vartime`, and
//! `Point::double_scalar_mul_vartime` (one interleaved Jacobian pass).
//! Usage: relation_point_native NAME P A B GX GY N H PAIRS REPS   (hex except H, PAIRS, REPS)
use crypto_lib::ecc::curve::CurveParams;
use num_bigint::BigUint;
use num_traits::Num;
use std::time::Instant;

fn hex(s: &str) -> BigUint {
    BigUint::from_str_radix(s, 16).expect("hex")
}

fn main() {
    let v: Vec<String> = std::env::args().collect();
    let name: &'static str = Box::leak(v[1].clone().into_boxed_str());
    let curve = CurveParams {
        name,
        p: hex(&v[2]),
        a: hex(&v[3]),
        b: hex(&v[4]),
        gx: hex(&v[5]),
        gy: hex(&v[6]),
        n: hex(&v[7]),
        h: v[8].parse().unwrap(),
    };
    let pairs: usize = v[9].parse().unwrap();
    let reps: usize = v[10].parse().unwrap();
    let a_fe = curve.a_fe();
    let g = curve.generator();
    let q = g.scalar_mul(&BigUint::from(0x0123_4567_89ab_cdef_u64), &a_fe);
    // xorshift64* scalars below n
    let mut st = 0x9e37_79b9_7f4a_7c15_u64;
    let mut next = || {
        st ^= st >> 12;
        st ^= st << 25;
        st ^= st >> 27;
        st.wrapping_mul(0x2545_f491_4f6c_dd1d)
    };
    let mut scalar = || {
        let mut x = BigUint::from(0u32);
        for _ in 0..5 {
            x = (x << 64u32) + BigUint::from(next());
        }
        x % &curve.n
    };
    let ks: Vec<(BigUint, BigUint)> = (0..pairs).map(|_| (scalar(), scalar())).collect();
    // equivalence on every pair
    for (k, l) in &ks {
        let w = g.scalar_mul(k, &a_fe).add(&q.scalar_mul(l, &a_fe), &a_fe);
        let c = g
            .scalar_mul_vartime(k, &a_fe)
            .add_vartime(&q.scalar_mul_vartime(l, &a_fe), &a_fe);
        let j = g.double_scalar_mul_vartime(k, &q, l, &a_fe);
        assert_eq!(w, c, "control differs");
        assert_eq!(w, j, "joint differs");
    }
    let mut t = [0f64; 3];
    let mut sink = 0usize;
    for r in 0..reps {
        for path in 0..3 {
            let path = (path + r) % 3; // rotate the order each rep
            let t0 = Instant::now();
            for (k, l) in &ks {
                let p = match path {
                    0 => g.scalar_mul(k, &a_fe).add(&q.scalar_mul(l, &a_fe), &a_fe),
                    1 => g
                        .scalar_mul_vartime(k, &a_fe)
                        .add_vartime(&q.scalar_mul_vartime(l, &a_fe), &a_fe),
                    _ => g.double_scalar_mul_vartime(k, &q, l, &a_fe),
                };
                sink += matches!(p, crypto_lib::ecc::point::Point::Affine { .. }) as usize;
            }
            t[path] += t0.elapsed().as_secs_f64();
        }
    }
    let per = |x: f64| x / (reps * pairs) as f64 * 1e6;
    println!(
        "{} p_bits={} pairs={} reps={} sink={} | us per relation point: former={:.1} vartime-two-ladders={:.1} joint={:.1} | joint/former={:.3} joint/two-ladders={:.3}",
        name,
        curve.p.bits(),
        pairs,
        reps,
        sink,
        per(t[0]),
        per(t[1]),
        per(t[2]),
        t[2] / t[0],
        t[2] / t[1]
    );
}
