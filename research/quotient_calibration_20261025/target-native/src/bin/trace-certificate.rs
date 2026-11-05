//! Certificate for the all-distinct-slot trace identity; no target data.
pub mod hash {
    // Adapt the canonical checker to this standalone crate's existing SHA-256
    // dependency without importing the unrelated root SIMD implementation.
    pub mod sha256 {
        use sha2::{Digest, Sha256};
        pub fn sha256(data: &[u8]) -> [u8; 32] {
            Sha256::digest(data).into()
        }
    }
}
#[path = "../../../../../src/cryptanalysis/identity_certificate.rs"]
pub mod identity_certificate;

use identity_certificate::{Expr, IdentityStatement};
use serde_json::json;
use std::{collections::BTreeMap, fs::OpenOptions, io::Write, path::Path};

fn variable(group: usize, index: usize) -> Expr {
    Expr::var(&format!("{}{}", ["f", "g", "h"][group], index + 1))
}
fn trace(groups: &[usize]) -> Expr {
    Expr::sum(
        (0..3)
            .map(|i| Expr::product(groups.iter().map(|g| variable(*g, i)).collect()))
            .collect(),
    )
}
fn statement() -> IdentityStatement {
    let lhs = Expr::sum(vec![
        Expr::product(vec![trace(&[0]), trace(&[1]), trace(&[2])]),
        Expr::minus(Expr::times(trace(&[0, 1]), trace(&[2]))),
        Expr::minus(Expr::times(trace(&[0, 2]), trace(&[1]))),
        Expr::minus(Expr::times(trace(&[1, 2]), trace(&[0]))),
        Expr::times(Expr::Const(2), trace(&[0, 1, 2])),
    ]);
    let permutations = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    let rhs = Expr::sum(
        permutations
            .iter()
            .map(|p| Expr::product((0..3).map(|i| variable(i, p[i])).collect()))
            .collect(),
    );
    IdentityStatement{name:"Three-slot inclusion-exclusion for symmetric Kummer tensor evaluation".into(),source:"Elementary inclusion-exclusion; trace evaluation of Tian arXiv:2012.07173v1 section 3.2 symmetric products".into(),variables:(0..3).flat_map(|g|(0..3).map(move|i|format!("{}{}",["f","g","h"][g],i+1))).collect(),degree_bound:3,lhs,rhs}
}
type Polynomial = BTreeMap<Vec<String>, i64>;
fn expand(e: &Expr) -> Polynomial {
    match e {
        Expr::Var(v) => BTreeMap::from([(vec![v.clone()], 1)]),
        Expr::Const(c) => BTreeMap::from([(vec![], *c)]),
        Expr::Neg(a) => expand(a).into_iter().map(|(m, c)| (m, -c)).collect(),
        Expr::Add(terms) => {
            let mut out = BTreeMap::new();
            for term in terms {
                for (m, c) in expand(term) {
                    *out.entry(m).or_insert(0) += c;
                }
            }
            out.retain(|_, c| *c != 0);
            out
        }
        Expr::Mul(terms) => {
            let mut out = BTreeMap::from([(vec![], 1)]);
            for term in terms {
                let right = expand(term);
                let mut next = BTreeMap::new();
                for (m, c) in out {
                    for (n, d) in &right {
                        let mut mon = m.clone();
                        mon.extend(n.clone());
                        mon.sort();
                        *next.entry(mon).or_insert(0) += c * d;
                    }
                }
                out = next;
            }
            out.retain(|_, c| *c != 0);
            out
        }
    }
}
fn fresh(path: &Path, value: &impl serde::Serialize) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("fresh certificate output");
    serde_json::to_writer_pretty(&mut file, value).unwrap();
    file.write_all(b"\n").unwrap();
}
fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("fresh certificate JSON path");
    let statement = statement();
    let left = expand(&statement.lhs);
    let right = expand(&statement.rhs);
    assert_eq!(left, right);
    // This is a public polynomial-identity proof record, not a credential or
    // TLS certificate. Its full contents are intentionally published and replayed.
    let public_identity_proof = identity_certificate::issue(
        statement,
        20261026,
        32,
        "One-session issue and canonical native check; independent runner review not performed.",
        None,
    )
    .unwrap();
    assert!(matches!(
        identity_certificate::check_with_extra(&public_identity_proof, Some((0x747261636533, 16)))
            .unwrap(),
        identity_certificate::Verdict::Accept { .. }
    ));
    fresh(Path::new(&path), &public_identity_proof);
    fresh(
        &Path::new(&path).with_extension("expansion.json"),
        &json!({"certificate_id":public_identity_proof.id,"exact_integer_coefficient_comparison":true,"nonzero_monomials":left.len(),"coefficients":left.into_iter().map(|(m,c)|json!({"monomial":m,"coefficient":c})).collect::<Vec<_>>(),"formal_prover_used":false}),
    );
    println!(
        "{}: exact integer expansion and canonical field certificate checked",
        public_identity_proof.id
    );
}
