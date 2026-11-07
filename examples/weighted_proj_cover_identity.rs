//! Issue and independently replay the sextic factorization used by the
//! weighted-Proj chart-cover lemma.
//! `cargo run --release --example weighted_proj_cover_identity -- issue`
//! `cargo run --release --example weighted_proj_cover_identity -- check PATH.json`

use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityCertificate, IdentityStatement, ProofRef, Verdict,
};
use std::{env, fs};

fn v(name: &str) -> Expr {
    Expr::var(name)
}

fn sum(terms: Vec<Expr>) -> Expr {
    Expr::sum(terms)
}

fn product(terms: Vec<Expr>) -> Expr {
    Expr::product(terms)
}

fn pow(x: Expr, n: usize) -> Expr {
    product(vec![x; n])
}

fn minus(a: Expr, b: Expr) -> Expr {
    sum(vec![a, Expr::minus(b)])
}

fn statement() -> IdentityStatement {
    let x = v("x");
    let y = v("y");
    let z = v("z");
    let lhs = minus(
        pow(y.clone(), 2),
        sum(vec![
            product(vec![pow(x.clone(), 5), z.clone()]),
            product(vec![v("a3"), pow(x.clone(), 3), pow(z.clone(), 3)]),
            product(vec![v("a2"), pow(x.clone(), 2), pow(z.clone(), 4)]),
            product(vec![v("a1"), x.clone(), pow(z.clone(), 5)]),
            product(vec![v("a0"), pow(z.clone(), 6)]),
        ]),
    );
    let tail = sum(vec![
        pow(x.clone(), 5),
        product(vec![v("a3"), pow(x.clone(), 3), pow(z.clone(), 2)]),
        product(vec![v("a2"), pow(x.clone(), 2), pow(z.clone(), 3)]),
        product(vec![v("a1"), x, pow(z.clone(), 4)]),
        product(vec![v("a0"), pow(z.clone(), 5)]),
    ]);
    let rhs = minus(pow(y, 2), product(vec![z, tail]));
    IdentityStatement {
        name: "weighted genus-2 sextic factorization for chart cover".into(),
        source: "research/projective_law_followup_20261022/formal/Common22WeightedProjCover.lean"
            .into(),
        variables: ["x", "y", "z", "a3", "a2", "a1", "a0"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        degree_bound: 7,
        lhs,
        rhs,
    }
}

fn proof() -> ProofRef {
    ProofRef {
        kind: "lean4".into(),
        path: "research/projective_law_followup_20261022/formal/Common22WeightedProjCover.lean"
            .into(),
        sha256: "8fb14500cc661695aec938b933ef740048d82b47933b90ba7f2bcd20ca4f947f".into(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, mode] if mode == "issue" => {
            let certificate = issue(
                statement(),
                0x2026_1025_2304,
                64,
                "issued by the native weighted_proj_cover_identity example; the sealed statement is replayed with an independent checker seed and the cited Lean source is kernel-checked separately",
                Some(proof()),
            )
            .expect("weighted sextic factorization");
            println!("{}", serde_json::to_string_pretty(&certificate).unwrap());
        }
        [_, mode, path] if mode == "check" => {
            let certificate: IdentityCertificate =
                serde_json::from_slice(&fs::read(path).expect("certificate file")).unwrap();
            assert_eq!(certificate.statement, statement());
            assert_eq!(certificate.proof, Some(proof()));
            match check_with_extra(&certificate, Some((0x2026_1025_2305, 64))).unwrap() {
                Verdict::Accept { .. } => println!("CHECKED"),
                _ => panic!("certificate rejected"),
            }
        }
        _ => panic!("usage: weighted_proj_cover_identity issue | check CERTIFICATE.json"),
    }
}
