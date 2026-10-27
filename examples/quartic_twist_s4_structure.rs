//! Exact integer derivation of the j=0 symmetrised fourth summation polynomial.
use std::path::Path;

use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityCertificate, IdentityStatement, Verdict,
};
use serde_json::json;

#[allow(dead_code)]
#[path = "quartic_twist_s4_integer.rs"]
mod integer;
use integer::{derive, derive_full, elementary, full_elementary, ZPoly};

fn expression(poly: &ZPoly, values: &[Expr; 5]) -> Expr {
    Expr::sum(
        poly.0
            .iter()
            .map(|(exps, coefficient)| {
                let mut factors = vec![Expr::Const(i64::try_from(*coefficient).unwrap())];
                for i in 0..5 {
                    factors.extend((0..exps[i]).map(|_| values[i].clone()));
                }
                Expr::product(factors)
            })
            .collect(),
    )
}

fn certificate(original: &ZPoly, symmetric: &ZPoly, full: bool) -> IdentityCertificate {
    let names = ["x1", "x2", "x3", "X", "B"];
    let values = names.map(Expr::var);
    let e = if full {
        full_elementary()
    } else {
        elementary()
    }
    .map(|poly| expression(&poly, &values));
    let statement = IdentityStatement {
        name: if full { "exact integer j0 S4 full symmetrisation" } else { "exact integer j0 S4 symmetrisation" }.to_string(),
        source: "examples/quartic_twist_s4_structure.rs; exact integer expansion of the two-quadratic S3 resultant and lexicographic symmetric reduction".to_string(),
        variables: names.map(str::to_string).to_vec(),
        degree_bound: 12,
        lhs: expression(original, &values),
        rhs: expression(symmetric, &e),
    };
    let cert = issue(statement.clone(), 0x5334_5457_4953_5431, 32,
        "native exact coefficient expansion; certificate checked with 32 further domain-separated points", None).unwrap();
    assert!(matches!(
        check_with_extra(&cert, Some((0x5345_5041_5241_5445, 32))).unwrap(),
        Verdict::Accept { .. }
    ));
    let mut corrupt = statement;
    corrupt.rhs = Expr::plus(corrupt.rhs, Expr::Const(1));
    assert!(issue(corrupt, 0x5334_5457_4953_5431, 32, "negative control", None).is_err());
    cert
}

pub(crate) fn write_record(out: &Path) {
    assert!(!out.exists(), "output directory is write-once");
    std::fs::create_dir_all(out).unwrap();
    let (original, symmetric) = derive();
    let cert = certificate(&original, &symmetric, false);
    let full = derive_full(&original);
    let full_cert = certificate(&original, &full, true);
    let terms: Vec<_> = symmetric
        .0
        .iter()
        .map(|(e, c)| {
            json!({
                "exponents":e,"coefficient":i64::try_from(*c).unwrap()
            })
        })
        .collect();
    let record = json!({"schema":"quartic-twist-s4-exact-structure/v1","status":"pass",
        "variables":["e1","e2","e3","X","B"],"terms":terms,
        "expanded_resultant_terms":original.0.len(),"symmetric_terms":symmetric.0.len(),
        "exact_integer_remainder_terms":0,"max_abs_coefficient":symmetric.0.values().map(|c| c.abs()).max(),
        "weighted_degree":12,"weights":[1,2,3,1,3],
        "certificate_id":cert.id,"mutated_identity_rejected":true,
        "fully_symmetric_terms":full.0.len(),"full_certificate_id":full_cert.id,
        "full_exact_integer_remainder_terms":0,"full_mutated_identity_rejected":true});
    std::fs::write(
        out.join("structure.json"),
        serde_json::to_vec_pretty(&record).unwrap(),
    )
    .unwrap();
    std::fs::write(
        out.join("identity.json"),
        serde_json::to_vec_pretty(&cert).unwrap(),
    )
    .unwrap();
    std::fs::write(
        out.join("full-identity.json"),
        serde_json::to_vec_pretty(&full_cert).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string(&record).unwrap());
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(
        args.len(),
        2,
        "usage: quartic_twist_s4_structure OUTPUT_DIRECTORY"
    );
    write_record(Path::new(&args[1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_symmetric_reconstruction() {
        let (original, symmetric) = derive();
        assert_eq!(symmetric.substitute(&elementary()), original);
        assert!(symmetric
            .0
            .keys()
            .all(|e| e[0] + 2 * e[1] + 3 * e[2] + e[3] + 3 * e[4] == 12));
    }

    #[test]
    fn independent_certificate_points_and_corruption() {
        let (original, symmetric) = derive();
        let cert = certificate(&original, &symmetric, false);
        assert!(matches!(
            check_with_extra(&cert, Some((0x5445_5354_4449_4646, 16))).unwrap(),
            Verdict::Accept { .. }
        ));
        let full = derive_full(&original);
        let full_cert = certificate(&original, &full, true);
        assert!(matches!(
            check_with_extra(&full_cert, Some((0x5445_5354_4449_4646, 16))).unwrap(),
            Verdict::Accept { .. }
        ));
    }
}
