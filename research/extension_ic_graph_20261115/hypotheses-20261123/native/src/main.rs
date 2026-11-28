//! Finite identity qualification only: no relation search, target, or timing.
use std::{collections::BTreeMap, error::Error, io};

mod hash {
    pub mod sha256 {
        use sha2::{Digest, Sha256};
        pub fn sha256(data: &[u8]) -> [u8; 32] {
            Sha256::digest(data).into()
        }
    }
}

// Reuse the repository checker rather than making a new certificate format.
#[allow(dead_code)]
#[path = "../../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
use identity_certificate::{Expr, IdentityCertificate, IdentityStatement, Verdict};

fn statement() -> IdentityStatement {
    let var = Expr::var;
    let sq = |name| Expr::product(vec![var(name), var(name)]);
    let a = Expr::sum(vec![
        Expr::product(vec![var("u"), var("L")]),
        Expr::minus(Expr::product(vec![var("C"), var("v")])),
    ]);
    IdentityStatement {
        name: "Nagao norm quotient with H = v^2 + u W".into(),
        source: "Joux–Vitse 2012 section 3.1; hypotheses-20261123 H4, direct expansion".into(),
        variables: ["u", "v", "W", "C", "L"].map(str::to_owned).to_vec(),
        degree_bound: 4,
        lhs: Expr::sum(vec![
            Expr::product(vec![
                sq("C"),
                Expr::sum(vec![sq("v"), Expr::product(vec![var("u"), var("W")])]),
            ]),
            Expr::minus(Expr::product(vec![a.clone(), a])),
        ]),
        rhs: Expr::product(vec![
            var("u"),
            Expr::sum(vec![
                Expr::product(vec![sq("C"), var("W")]),
                Expr::product(vec![Expr::Const(2), var("C"), var("v"), var("L")]),
                Expr::minus(Expr::product(vec![var("u"), sq("L")])),
            ]),
        ]),
    }
}

type Polynomial = BTreeMap<[u32; 5], i128>;

/// Exact coefficient expansion over Z, independent of sampled field values.
fn expand(e: &Expr) -> Polynomial {
    match e {
        Expr::Var(name) => {
            let k = ["u", "v", "W", "C", "L"]
                .iter()
                .position(|&v| v == name)
                .expect("declared variable");
            let mut monomial = [0; 5];
            monomial[k] = 1;
            BTreeMap::from([(monomial, 1)])
        }
        Expr::Const(c) => BTreeMap::from([([0; 5], i128::from(*c))]),
        Expr::Neg(a) => expand(a).into_iter().map(|(m, c)| (m, -c)).collect(),
        Expr::Add(args) => {
            let mut out = Polynomial::new();
            for a in args {
                for (m, c) in expand(a) {
                    let entry = out.entry(m).or_default();
                    *entry = entry.checked_add(c).expect("coefficient addition overflow");
                }
            }
            out.retain(|_, c| *c != 0);
            out
        }
        Expr::Mul(args) => {
            let mut out = BTreeMap::from([([0u32; 5], 1i128)]);
            for a in args {
                let next = expand(a);
                let mut product = Polynomial::new();
                for (m, c) in &out {
                    for (n, d) in &next {
                        let mut monomial = [0; 5];
                        for k in 0..5 {
                            monomial[k] = m[k].checked_add(n[k]).expect("degree overflow");
                        }
                        let coefficient = c.checked_mul(*d).expect("coefficient product overflow");
                        let entry = product.entry(monomial).or_default();
                        *entry = entry
                            .checked_add(coefficient)
                            .expect("coefficient sum overflow");
                    }
                }
                product.retain(|_, c| *c != 0);
                out = product;
            }
            out
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [mode] if mode == "issue" => {
            let s = statement();
            assert_eq!(expand(&s.lhs), expand(&s.rhs));
            let cert = identity_certificate::issue(
                s,
                0x2026_1123_4859_5034,
                32,
                "Producer-issued; reused repository checker and producer integer expansion. No independent or signed verdict.",
                None,
            )?;
            assert!(matches!(
                identity_certificate::check(&cert)?,
                Verdict::Accept { .. }
            ));
            println!("{}", serde_json::to_string_pretty(&cert)?);
        }
        [mode, path] if mode == "check" => {
            let cert: IdentityCertificate = serde_json::from_slice(&std::fs::read(path)?)?;
            if cert.statement != statement() {
                return Err(io::Error::other("unexpected identity statement").into());
            }
            let exact = expand(&cert.statement.lhs) == expand(&cert.statement.rhs);
            if !exact {
                return Err(io::Error::other("integer identity differs").into());
            }
            let verdict = identity_certificate::check_with_extra(&cert, Some((20261123, 16)))?;
            if !matches!(verdict, Verdict::Accept { .. }) {
                return Err(io::Error::other(format!("certificate rejected: {verdict:?}")).into());
            }
            println!(
                "{}",
                serde_json::json!({
                    "id": cert.id,
                    "integer_coefficients_equal": exact,
                    "nonzero_monomials": expand(&cert.statement.lhs).len(),
                    "sampled_points": 48,
                    "standing": "producer-only; unverified",
                    "scope": "polynomial identity, not dimension, parametrization, yield, or performance"
                })
            );
        }
        _ => {
            return Err(
                io::Error::other("usage: graph-ic-hypothesis-identity issue | check PATH").into(),
            )
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_has_exact_integer_coefficients() {
        let s = statement();
        let lhs = expand(&s.lhs);
        assert_eq!(lhs, expand(&s.rhs));
        assert_eq!(lhs.len(), 3);
    }

    #[test]
    fn omitted_cross_term_is_false() {
        let mut s = statement();
        s.rhs = Expr::product(vec![
            Expr::var("u"),
            Expr::sum(vec![
                Expr::product(vec![Expr::var("C"), Expr::var("C"), Expr::var("W")]),
                Expr::minus(Expr::product(vec![
                    Expr::var("u"),
                    Expr::var("L"),
                    Expr::var("L"),
                ])),
            ]),
        ]);
        assert_ne!(expand(&s.lhs), expand(&s.rhs));
        assert!(matches!(
            identity_certificate::issue(s, 7, 32, "negative control", None),
            Err(identity_certificate::IssueError::False(_))
        ));
    }
}
