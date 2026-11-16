//! Frozen ALL-NINE public-case observation; no map or timing claim.
use isogeny_algos::field::Field;
use isogeny_algos::fpr::{FpR, ER};
use serde_json::{json, Value};
const CASES: &str =
    include_str!("../../../evidence/genus3-ramification-panel-01/all-historical-cases.json");
fn decode(q: u64, mut n: u64) -> ER {
    let mut a = [0; 16];
    for x in a.iter_mut().take(3) {
        *x = n % q;
        n /= q;
    }
    assert_eq!(n, 0);
    a
}
fn coefficients(a: ER) -> [u64; 3] {
    assert!(a[3..].iter().all(|&x| x == 0));
    [a[0], a[1], a[2]]
}
fn main() {
    let data: Value = serde_json::from_str(CASES).expect("frozen JSON");
    for case in data["cases"].as_array().unwrap() {
        let q = case["q"].as_u64().unwrap();
        let mut modulus: Vec<_> = case["E0_weierstrass_ext"]["modulus"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect();
        modulus.push(1);
        let f = FpR::from_modulus(q, &modulus);
        let b = decode(q, case["u_idx"][0].as_u64().unwrap());
        let c = decode(q, case["u_idx"][1].as_u64().unwrap());
        let bs = [b, f.frobenius(b), f.frobenius(f.frobenius(b))];
        let cs = [c, f.frobenius(c), f.frobenius(f.frobenius(c))];
        let eval = |i: usize, t: ER| f.add(f.add(f.sq(t), f.mul(bs[i], t)), cs[i]);
        let mut roots = Vec::new();
        let mut targets = 0;
        for i in 0..q.pow(3) {
            let t = decode(q, i);
            if !f.is_zero(eval(1, t)) {
                continue;
            }
            let rhs = f.mul(eval(0, t), eval(2, t));
            assert!(!f.is_zero(rhs));
            let root = f.sqrt(rhs);
            let euler = f.pow(rhs, ((q.pow(3) - 1) / 2) as u128);
            assert_eq!(root.is_some(), euler == f.one());
            let norm = f.mul(rhs, f.mul(f.frobenius(rhs), f.frobenius(f.frobenius(rhs))));
            let norm = f.project(norm).expect("norm descends");
            if root.is_some() {
                targets += 2;
            }
            roots.push(json!({"t":coefficients(t),"rhs":coefficients(rhs),"norm_Fq":norm,"Euler_K":coefficients(euler),"K_square_root":root.map(coefficients)}));
        }
        if case["kind"] == "split" {
            assert_eq!(roots.len(), 2);
        } else {
            assert_eq!(roots.len(), 0);
        }
        println!(
            "G3_RAMIFICATION {}",
            json!({"q":q,"kind":case["kind"],"u_idx":case["u_idx"],"source_K_ramification_targets":targets,"u1_roots":roots})
        );
    }
}
