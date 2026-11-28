//! Issue finite-field evaluation certificates for the nodewise polynomial identity.
//! Uses the repository's existing native issuer, not a new certificate implementation.
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr as E, IdentityStatement, Verdict,
};

fn v(i: usize) -> E {
    E::var(&format!("x{i}"))
}
fn w(i: usize) -> E {
    E::var(&format!("w{i}"))
}
fn delta(i: usize, j: usize) -> E {
    E::plus(v(i), E::minus(v(j)))
}
fn weight(mask: usize, m: usize) -> E {
    let mut terms = Vec::new();
    for i in 0..m {
        if mask & (1 << i) != 0 {
            terms.push(w(i));
            for j in 0..i {
                if mask & (1 << j) != 0 {
                    terms.push(delta(i, j));
                    terms.push(delta(i, j));
                }
            }
        }
    }
    E::product(terms)
}
fn main() {
    let mut certs = Vec::new();
    for g in 2..=4 {
        for d in 1..=3 {
            let m = g + 2 * d;
            let mut lhs = Vec::new();
            let mut rhs = Vec::new();
            for mask in 0usize..1 << m {
                let size = mask.count_ones() as usize;
                if size == d {
                    let mut term = vec![weight(mask, m)];
                    for j in 0..m {
                        if mask & (1 << j) == 0 {
                            term.push(delta(0, j));
                        }
                    }
                    lhs.push(E::product(term));
                }
                if size == d - 1 {
                    let mut term = vec![weight(mask, m)];
                    for j in 0..m {
                        if mask & (1 << j) != 0 {
                            term.push(delta(0, j));
                        }
                    }
                    rhs.push(E::product(term));
                }
            }
            let mut right = vec![w(0), E::sum(rhs)];
            for j in 1..m {
                right.push(delta(0, j));
            }
            let stmt=IdentityStatement {
            name:format!("higher-pole node0 interpolation g{g} d{d}"),
            source:"D'Andrea-Krick-Szanto arXiv:1211.6895, subset specialization; RATIONAL-PROTOCOL.md. Weights arbitrary, yi=wi*A'(xi), signs absorbed in wi. Other nodes follow by permutation.".into(),
            variables:(0..m).map(|i|format!("x{i}")).chain((0..m).map(|i|format!("w{i}"))).collect(),
            degree_bound:(d*d+m-d)as u32,lhs:E::sum(lhs),rhs:E::product(right),
        };
            let cert=issue(stmt,2026111504,32,"Producer-issued and producer-rechecked with 16 additional points; not independent authorship or a formal proof.",None).unwrap();
            assert!(matches!(
                check_with_extra(&cert, Some((2026111505, 16))).unwrap(),
                Verdict::Accept { .. }
            ));
            certs.push(cert);
        }
    }
    println!("{}", serde_json::to_string(&certs).unwrap());
}
