//! Fixed-size, full-field CPU stage measurements on three Certicom curves.
//!
//! `cargo run --release --example certicom_131_stage_bench -- 400 100 5`
//! prints CSV. A rho step includes the partition, a group addition, and
//! coefficient updates. An IC query includes S3 coefficient construction and
//! roots over the full target field. Neither is an end-to-end solve.

use crypto_lib::binary_ecc::curve::{point_add, scalar_mul};
use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::binary_semaev::{
    binary_semaev_s3, binary_semaev_s3_in_x3, solve_quadratic_f2m,
};
use crypto_lib::cryptanalysis::ec_index_calculus::{semaev_s3, semaev_s3_in_x3, sqrt_mod_p};
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::point::Point;
use num_bigint::BigUint;
use std::hint::black_box;
use std::time::Instant;

fn binary_x(p: &BinaryPoint) -> &F2mElement {
    match p {
        BinaryPoint::Affine { x, .. } => x,
        BinaryPoint::Infinity => panic!("unexpected identity point"),
    }
}

fn prime_x(p: &Point) -> &crypto_lib::ecc::field::FieldElement {
    match p {
        Point::Affine { x, .. } => x,
        Point::Infinity => panic!("unexpected identity point"),
    }
}

fn binary_fixture(name: &str) -> (BinaryCurve, Vec<BinaryPoint>, Vec<(u32, u32)>) {
    let (curve, _) = BinaryCurve::certicom_challenge(name).expect("Certicom curve");
    assert!(curve.is_on_curve(&curve.generator));
    assert_eq!(
        scalar_mul(&curve, &curve.generator, &curve.order),
        BinaryPoint::Infinity
    );
    let g = curve.generator.clone();
    let q = scalar_mul(&curve, &g, &BigUint::from(73u32));
    let mut jumps = Vec::new();
    let mut coeffs = Vec::new();
    for i in 0..16u32 {
        let a = i + 1;
        let b = (i * i + 3 * i + 7) % 97 + 1;
        let ag = scalar_mul(&curve, &g, &BigUint::from(a));
        let bq = scalar_mul(&curve, &q, &BigUint::from(b));
        jumps.push(point_add(&curve, &ag, &bq));
        coeffs.push((a, b));
    }
    (curve, jumps, coeffs)
}

fn prime_fixture() -> (CurveParams, Vec<Point>, Vec<(u32, u32)>) {
    let curve = CurveParams::eccp131();
    let g = curve.generator();
    assert!(curve.is_on_curve(&g));
    let a_fe = curve.a_fe();
    assert_eq!(g.scalar_mul(&curve.n, &a_fe), Point::Infinity);
    let q = g.scalar_mul(&BigUint::from(73u32), &a_fe);
    let mut jumps = Vec::new();
    let mut coeffs = Vec::new();
    for i in 0..16u32 {
        let a = i + 1;
        let b = (i * i + 3 * i + 7) % 97 + 1;
        let ag = g.scalar_mul(&BigUint::from(a), &a_fe);
        let bq = q.scalar_mul(&BigUint::from(b), &a_fe);
        jumps.push(ag.add_vartime(&bq, &a_fe));
        coeffs.push((a, b));
    }
    (curve, jumps, coeffs)
}

fn binary_rho(
    curve: &BinaryCurve,
    jumps: &[BinaryPoint],
    coeffs: &[(u32, u32)],
    steps: usize,
) -> u64 {
    let mut p = curve.generator.clone();
    let mut a = BigUint::from(1u32);
    let mut b = BigUint::from(0u32);
    for _ in 0..steps {
        let j = (binary_x(&p).raw_bits()[0] & 15) as usize;
        p = point_add(curve, &p, &jumps[j]);
        a += coeffs[j].0;
        b += coeffs[j].1;
        if a >= curve.order {
            a -= &curve.order;
        }
        if b >= curve.order {
            b -= &curve.order;
        }
        black_box(&p);
    }
    assert!(curve.is_on_curve(&p));
    binary_x(&p).raw_bits()[0]
        ^ a.to_u64_digits().first().copied().unwrap_or(0)
        ^ b.to_u64_digits().first().copied().unwrap_or(0)
}

fn prime_rho(curve: &CurveParams, jumps: &[Point], coeffs: &[(u32, u32)], steps: usize) -> u64 {
    let mut p = curve.generator();
    let mut a = BigUint::from(1u32);
    let mut b = BigUint::from(0u32);
    let a_fe = curve.a_fe();
    for _ in 0..steps {
        let j = (prime_x(&p)
            .value
            .to_u64_digits()
            .first()
            .copied()
            .unwrap_or(0)
            & 15) as usize;
        p = p.add_vartime(&jumps[j], &a_fe);
        a += coeffs[j].0;
        b += coeffs[j].1;
        if a >= curve.n {
            a -= &curve.n;
        }
        if b >= curve.n {
            b -= &curve.n;
        }
        black_box(&p);
    }
    assert!(curve.is_on_curve(&p));
    prime_x(&p)
        .value
        .to_u64_digits()
        .first()
        .copied()
        .unwrap_or(0)
        ^ a.to_u64_digits().first().copied().unwrap_or(0)
        ^ b.to_u64_digits().first().copied().unwrap_or(0)
}

fn binary_ic_fixture(curve: &BinaryCurve) -> Vec<F2mElement> {
    let g = curve.generator.clone();
    let p = scalar_mul(curve, &g, &BigUint::from(37u32));
    let q = scalar_mul(curve, &g, &BigUint::from(73u32));
    let r = point_add(curve, &p, &q);
    assert!(binary_semaev_s3(
        binary_x(&p),
        binary_x(&q),
        binary_x(&r),
        &curve.b,
        &curve.irreducible
    )
    .is_zero());
    let (aa, bb, cc) =
        binary_semaev_s3_in_x3(binary_x(&p), binary_x(&q), &curve.b, &curve.irreducible);
    assert!(solve_quadratic_f2m(&aa, &bb, &cc, curve.m, &curve.irreducible).contains(binary_x(&r)));

    (1..=32u32)
        .map(|i| scalar_mul(curve, &g, &BigUint::from(i)).pipe(|point| binary_x(&point).clone()))
        .collect()
}

fn binary_ic(curve: &BinaryCurve, xs: &[F2mElement], queries: usize) -> u64 {
    let mut checksum = 0u64;
    for k in 0..queries {
        let (aa, bb, cc) = binary_semaev_s3_in_x3(
            &xs[k % 32],
            &xs[(k * 7 + 3) % 32],
            &curve.b,
            &curve.irreducible,
        );
        let roots = solve_quadratic_f2m(&aa, &bb, &cc, curve.m, &curve.irreducible);
        checksum ^= roots.len() as u64;
        for root in roots {
            checksum ^= root.raw_bits()[0];
        }
    }
    checksum
}

fn prime_ic_fixture(curve: &CurveParams) -> Vec<crypto_lib::ecc::field::FieldElement> {
    let g = curve.generator();
    let a_fe = curve.a_fe();
    let b_fe = curve.fe(curve.b.clone());
    let p = g.scalar_mul(&BigUint::from(37u32), &a_fe);
    let q = g.scalar_mul(&BigUint::from(73u32), &a_fe);
    let r = p.add_vartime(&q, &a_fe);
    assert!(semaev_s3(prime_x(&p), prime_x(&q), prime_x(&r), &a_fe, &b_fe).is_zero());
    let (qa, qb, qc) = semaev_s3_in_x3(prime_x(&p), prime_x(&q), &a_fe, &b_fe);
    let disc = qb
        .mul(&qb)
        .sub(&curve.fe(BigUint::from(4u32)).mul(&qa).mul(&qc));
    let s = sqrt_mod_p(&disc.value, &curve.p).expect("positive S3 control has roots");
    let denom_inv = qa.add(&qa).inv().expect("positive S3 control denominator");
    let x1 = qb.neg().add(&curve.fe(s.clone())).mul(&denom_inv);
    let x2 = qb.neg().sub(&curve.fe(s)).mul(&denom_inv);
    assert!(x1 == *prime_x(&r) || x2 == *prime_x(&r));
    (1..=32u32)
        .map(|i| {
            g.scalar_mul(&BigUint::from(i), &a_fe)
                .pipe(|point| prime_x(&point).clone())
        })
        .collect()
}

fn prime_ic(
    curve: &CurveParams,
    xs: &[crypto_lib::ecc::field::FieldElement],
    queries: usize,
) -> u64 {
    let a_fe = curve.a_fe();
    let b_fe = curve.fe(curve.b.clone());
    let four = curve.fe(BigUint::from(4u32));
    let mut checksum = 0u64;
    for k in 0..queries {
        let (aa, bb, cc) = semaev_s3_in_x3(&xs[k % 32], &xs[(k * 7 + 3) % 32], &a_fe, &b_fe);
        if aa.is_zero() {
            if !bb.is_zero() {
                let x = cc.neg().mul(&bb.inv().expect("nonzero coefficient"));
                checksum ^= x.value.to_u64_digits().first().copied().unwrap_or(0);
            }
            continue;
        }
        let disc = bb.mul(&bb).sub(&four.mul(&aa).mul(&cc));
        if let Some(root) = sqrt_mod_p(&disc.value, &curve.p) {
            let denom_inv = aa.add(&aa).inv().expect("nonzero coefficient");
            let x1 = bb.neg().add(&curve.fe(root.clone())).mul(&denom_inv);
            let x2 = bb.neg().sub(&curve.fe(root)).mul(&denom_inv);
            checksum ^= x1.value.to_u64_digits().first().copied().unwrap_or(0);
            checksum ^= x2.value.to_u64_digits().first().copied().unwrap_or(0);
            checksum ^= 2;
        }
    }
    checksum
}

trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}
impl<T> Pipe for T {}

fn emit(name: &str, stage: &str, sample: usize, count: usize, run: impl FnOnce() -> u64) {
    let start = Instant::now();
    let checksum = black_box(run());
    let seconds = start.elapsed().as_secs_f64();
    println!("{name},{stage},{sample},{count},{seconds:.9},{checksum:016x}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parse = |i: usize, default: usize| -> usize {
        args.get(i)
            .map(|s| s.parse().expect("positive integer"))
            .unwrap_or(default)
    };
    let steps = parse(1, 400);
    let queries = parse(2, 100);
    let samples = parse(3, 5);
    assert!(steps > 0 && queries > 0 && samples > 0);
    let (k, kj, kc) = binary_fixture("ecc2k-130");
    let (b, bj, bc) = binary_fixture("ecc2-131");
    let (p, pj, pc) = prime_fixture();
    let kx = binary_ic_fixture(&k);
    let bx = binary_ic_fixture(&b);
    let px = prime_ic_fixture(&p);
    println!("target,stage,sample,operations,seconds,checksum");
    for sample in 0..samples {
        emit("ECC2K-130", "rho_state_step", sample, steps, || {
            binary_rho(&k, &kj, &kc, steps)
        });
        emit("ECC2-131", "rho_state_step", sample, steps, || {
            binary_rho(&b, &bj, &bc, steps)
        });
        emit("ECCp-131", "rho_state_step", sample, steps, || {
            prime_rho(&p, &pj, &pc, steps)
        });
        emit("ECC2K-130", "s3_root_query", sample, queries, || {
            binary_ic(&k, &kx, queries)
        });
        emit("ECC2-131", "s3_root_query", sample, queries, || {
            binary_ic(&b, &bx, queries)
        });
        emit("ECCp-131", "s3_root_query", sample, queries, || {
            prime_ic(&p, &px, queries)
        });
    }
}
