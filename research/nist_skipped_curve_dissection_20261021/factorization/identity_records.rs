//! Issue/check the repository identity format for the nine integer products.
//! PARI certificates separately establish primality. Lean proves exact products.
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityCertificate, IdentityStatement, ProofRef, Verdict,
};
use crypto_lib::hash::sha256::sha256;
use serde_json::Value;
use std::{env, fs, path::Path};

const STUDY: &str = match option_env!("NIST_FACTOR_STUDY") {
    Some(path) => path,
    None => "research/nist_skipped_curve_dissection_20261021/factorization",
};

// Exact decimal syntax as a constant expression; evaluation is the native
// repository checker's modular arithmetic, not a floating-point conversion.
fn decimal(s: &str) -> Expr {
    assert!(!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()));
    s.as_bytes().chunks(9).fold(Expr::Const(0), |a, chunk| {
        let n: i64 = std::str::from_utf8(chunk).unwrap().parse().unwrap();
        Expr::plus(
            Expr::times(a, Expr::Const(10_i64.pow(chunk.len() as u32))),
            Expr::Const(n),
        )
    })
}

fn statement(row: &Value) -> IdentityStatement {
    let id = row["id"].as_str().expect("target id");
    let mut terms = Vec::new();
    for part in ["factors", "residuals"] {
        for pair in row[part].as_array().expect("partition") {
            let e = pair[1].as_u64().expect("positive exponent");
            assert!((1..=512).contains(&e));
            for _ in 0..e {
                terms.push(decimal(pair[0].as_str().expect("decimal factor")));
            }
        }
    }
    IdentityStatement {
        name: format!("{id}: exact integer partition product"),
        source: format!(
            "{STUDY}/results.json; ProductIdentities.lean::SkippedNistFactors.{}_product",
            id.replace('-', "_")
        ),
        variables: Vec::new(),
        degree_bound: 0,
        lhs: decimal(row["value"].as_str().expect("decimal target")),
        rhs: Expr::product(terms),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 4 && ["issue", "check"].contains(&args[1].as_str()),
        "usage: identity-records issue|check RESULTS.json CERTIFICATE_DIRECTORY (from repo root)"
    );
    let proof_path = format!("{STUDY}/ProductIdentities.lean");
    let digest = sha256(&fs::read(&proof_path).expect("Lean source"));
    let proof = ProofRef {
        kind: "lean4-product-identity-only".into(),
        path: proof_path,
        sha256: digest.iter().map(|b| format!("{b:02x}")).collect(),
    };
    let rows: Vec<Value> =
        serde_json::from_slice(&fs::read(&args[2]).expect("results")).expect("JSON");
    assert_eq!(rows.len(), 9);
    let dir = Path::new(&args[3]);
    fs::create_dir_all(dir).expect("certificate directory");
    for row in &rows {
        let id = row["id"].as_str().expect("id");
        assert!(id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-'));
        let path = dir.join(format!("{id}.json"));
        let expected = statement(row);
        let cert: IdentityCertificate = if args[1] == "issue" {
            assert!(!path.exists(), "refuse to overwrite a certificate");
            let cert = issue(expected.clone(), 0x2026_1021, 32,
                "Issued by native identity_records.rs; independently checked in a separate invocation with 32 extra points. Exact integer products are proved by the linked Lean source; primality is checked separately by PARI.",
                Some(proof.clone())).expect("identity issue");
            fs::write(
                &path,
                format!("{}\n", serde_json::to_string_pretty(&cert).unwrap()),
            )
            .unwrap();
            cert
        } else {
            serde_json::from_slice(&fs::read(path).expect("certificate")).expect("certificate JSON")
        };
        assert_eq!(cert.statement, expected, "wrong product statement");
        assert_eq!(cert.proof, Some(proof.clone()), "wrong proof pointer");
        assert!(matches!(
            check_with_extra(&cert, Some((0x91f4_137a, 32))).expect("check"),
            Verdict::Accept { .. }
        ));
        println!("{} {}", id, cert.id);
    }
}
