//! Prime31 field-width admission for the existing native F4 engine.
//! This is a small 31-bit arithmetic smoke gate, NOT blind MQ7/MQ8 extraction.
use crypto_lib::cryptanalysis::f4_fp::{f4, F4Options, Ordering, Poly};

const P: u64 = 2_147_483_647;

fn eval(poly: &Poly, x: &[u64; 2]) -> u64 {
    poly.iter().fold(0u64, |acc, (exp, coeff)| {
        let mut t = *coeff % P;
        for (i, &e) in exp.iter().enumerate() {
            for _ in 0..e {
                t = ((t as u128 * x[i] as u128) % P as u128) as u64;
            }
        }
        (acc + t) % P
    })
}

fn main() {
    let x = P - 2;
    let y = P - 3;
    let xy = ((x as u128 * y as u128) % P as u128) as u64;
    // x = -2, y = -3, xy = +6 modulo p; deliberately exercise near-p coefficients.
    let system: Vec<Poly> = vec![
        vec![(vec![1, 0], 1), (vec![0, 0], P - x)],
        vec![(vec![0, 1], 1), (vec![0, 0], P - y)],
        vec![(vec![1, 1], 1), (vec![0, 0], P - xy)],
    ];
    for eq in &system {
        assert_eq!(eval(eq, &[x, y]), 0, "source equation does not vanish");
    }
    let options = F4Options::new(Ordering::Grevlex, 3);
    let output = f4(&system, 2, P, &options);
    assert!(!output.basis.is_empty(), "F4 returned an empty basis");
    for eq in &output.basis {
        assert_eq!(eval(eq, &[x, y]), 0, "F4 basis changed the common root");
    }
    println!(
        "Prime31 F4 field-width smoke PASS: basis={}, steps={}, rows={}, columns={}, multiplications={}",
        output.basis.len(), output.steps, output.max_rows, output.max_cols, output.field_ops
    );
}
