#![allow(dead_code)]
#[path = "../../../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
#[path = "../../../../../../examples/quartic_twist_s4_integer.rs"]
mod integer;
mod hash {
    pub mod sha256 {
        use sha2::{Digest, Sha256};
        pub fn sha256(b: &[u8]) -> [u8; 32] {
            Sha256::digest(b).into()
        }
    }
}
use identity_certificate::{check_with_extra, issue, Expr, IdentityStatement, Verdict};
use integer::{Exponent, ZPoly};
use serde_json::{json, Value};
use std::path::Path;
fn expression(p: &ZPoly) -> Expr {
    let names = ["s1", "s2", "s3", "s4", "unused"];
    Expr::sum(
        p.0.iter()
            .map(|(e, &c)| {
                let mut fs = vec![Expr::Const(c.try_into().unwrap())];
                for i in 0..5 {
                    for _ in 0..e[i] {
                        fs.push(Expr::var(names[i]));
                    }
                }
                Expr::product(fs)
            })
            .collect(),
    )
}
fn decode(v: &Value) -> ZPoly {
    let terms: Vec<(Exponent, i128)> = serde_json::from_value(v.clone()).unwrap();
    let mut p = ZPoly::default();
    for (e, c) in terms {
        assert_eq!(e[4], 0);
        assert_ne!(c, 0);
        assert!(!p.0.contains_key(&e));
        p.add_term(e, c);
    }
    p
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let square = args.len() == 5 && args[1] == "--square";
    assert!(args.len() == 4 || square);
    let offset = if square { 1 } else { 0 };
    let input_sha = &args[2 + offset];
    let bytes = std::fs::read(&args[1 + offset]).unwrap();
    assert_eq!(hex::encode(hash::sha256::sha256(&bytes)), *input_sha);
    let v: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["status"], "integer-identity-pass");
    let out = Path::new(&args[3 + offset]);
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let (raw, _) = integer::derive();
    let h = integer::derive_full(&raw);
    let supplied: Vec<(Exponent, i128)> = serde_json::from_value(v["h4"].clone()).unwrap();
    assert_eq!(integer::ZPoly(supplied.into_iter().collect()), h);
    let mut cs: [ZPoly; 4] = std::array::from_fn(|_| ZPoly::default());
    for (&e, &c) in &h.0 {
        let mut f = e;
        f[4] = 0;
        cs[e[4] as usize].add_term(f, c);
    }
    for (i, c) in cs.iter().enumerate() {
        assert_eq!(*c, decode(&v["coefficients"][i]));
    }
    let ds = decode(
        &v[if square {
            "square_defect_cube"
        } else {
            "discriminant"
        }],
    );
    // Fixed standard formula, independently pinned by the checked quartic
    // factorization in proof/orbit-v3. Do not trust a caller's target label.
    let classical = ZPoly(
        [
            ([0, 0, 0, 3, 0], 256),
            ([0, 0, 4, 0, 0], -27),
            ([0, 1, 2, 1, 0], 144),
            ([0, 2, 0, 2, 0], -128),
            ([0, 3, 2, 0, 0], -4),
            ([0, 4, 0, 1, 0], 16),
            ([1, 0, 1, 2, 0], -192),
            ([1, 1, 3, 0, 0], 18),
            ([1, 2, 1, 1, 0], -80),
            ([2, 0, 2, 1, 0], -6),
            ([2, 1, 0, 2, 0], 144),
            ([2, 2, 2, 0, 0], 1),
            ([2, 3, 0, 1, 0], -4),
            ([3, 0, 3, 0, 0], -4),
            ([3, 1, 1, 1, 0], 18),
            ([4, 0, 0, 2, 0], -27),
        ]
        .into_iter()
        .collect(),
    );
    if square {
        let [a, b, _c, d, _] = std::array::from_fn(ZPoly::var);
        let defect = d.scale(64).sub(&b.scale(4).sub(&a.pow(2)).pow(2));
        assert_eq!(decode(&v["square_defect"]), defect);
        assert_eq!(v["target_power"], 3);
        assert_eq!(
            ds,
            defect.pow(3),
            "target must be the actual square-defect cube"
        );
    } else {
        assert_eq!(
            ds, classical,
            "target must be the actual quartic discriminant"
        );
    }
    let denominator = v["denominator"].as_i64().unwrap();
    assert_eq!(denominator, if square { 64 } else { 2048 });
    let multipliers: Vec<_> = v["multipliers"]
        .as_array()
        .unwrap()
        .iter()
        .map(decode)
        .collect();
    assert_eq!(multipliers.len(), 4);
    let rhs = cs
        .iter()
        .zip(&multipliers)
        .fold(ZPoly::default(), |s, (c, a)| s.add(&c.mul(a)));
    let lhs = ds.scale(denominator as i128);
    assert_eq!(lhs, rhs);
    let lhs_expr = Expr::times(Expr::Const(denominator), expression(&ds));
    let rhs_expr = Expr::sum(
        cs.iter()
            .zip(&multipliers)
            .map(|(c, a)| Expr::times(expression(c), expression(a)))
            .collect(),
    );
    let statement=IdentityStatement{name:if square {"Pure zero S4 component ideal contains 64 times square-defect cube"}else{"Pure zero S4 component ideal contains 2048 times quartic discriminant"}.into(),source:"Frozen native rational search; independently reconstructed H4 components, exact integer replay and IDC expression checker".into(),variables:["s1","s2","s3","s4","unused"].map(str::to_string).to_vec(),degree_bound:12,lhs:lhs_expr,rhs:rhs_expr};
    let cert = issue(
        statement.clone(),
        0x505552455a45524f + u64::from(square),
        32,
        "Exact Z reconstruction; independent IDC expansion and 32 extra points",
        None,
    )
    .unwrap();
    assert!(matches!(
        check_with_extra(&cert, Some((0x4f42535452554354 + u64::from(square), 32))).unwrap(),
        Verdict::Accept { .. }
    ));
    let mut bad = statement.clone();
    bad.rhs = Expr::plus(bad.rhs, Expr::Const(1));
    assert!(issue(bad, 1, 32, "+1 mutation", None).is_err());
    let mut wrong_multipliers = multipliers.clone();
    wrong_multipliers[0] = wrong_multipliers[0].add(&ZPoly::constant(1));
    let wrong = cs
        .iter()
        .zip(&wrong_multipliers)
        .fold(ZPoly::default(), |s, (c, a)| s.add(&c.mul(a)));
    assert_ne!(lhs, wrong);
    std::fs::write(
        out.join(if square {
            "square-zero-obstruction.identity.json"
        } else {
            "pure-zero-obstruction.identity.json"
        }),
        serde_json::to_vec_pretty(&cert).unwrap(),
    )
    .unwrap();
    std::fs::write(out.join("summary.json"),serde_json::to_vec_pretty(&json!({"schema":"pure-zero-independent-verifier/v1","status":"pass","input_sha256":input_sha,"target":if square {"square-defect-cube"}else{"quartic-discriminant"},"denominator":denominator,"certificate_id":cert.id,"integer_remainder_terms":0,"constant_mutation_rejected":true,"multiplier_mutation_rejected":true,"additional_points":32,"scope":"Exact integer coefficient identity and independent IDC check. Kummer coefficient independence and actual curve/group S4 necessity are separate bridges; no cost or asymptotic claim."})).unwrap()).unwrap();
    println!("Independent integer replay and IDC accepted {}", cert.id);
}
