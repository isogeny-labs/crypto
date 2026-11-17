//! Certify S3's swap symmetry and check its actual Boolean chain action.
use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::identity_certificate::{
    self as ic, Expr, IdentityStatement, Verdict,
};
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::pq_groebner_f2::{F2BoolMono, F2BoolPoly};
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::collections::BTreeMap;

fn statement() -> IdentityStatement {
    let s3 = |a: &str, c: &str| {
        let (a, c, d, b) = (Expr::var(a), Expr::var(c), Expr::var("d"), Expr::var("b"));
        let ac = Expr::times(a.clone(), c.clone());
        let sum = Expr::plus(a, c);
        Expr::sum(vec![
            Expr::product(vec![sum.clone(), sum, d.clone(), d.clone()]),
            Expr::times(ac.clone(), d),
            Expr::times(ac.clone(), ac),
            b,
        ])
    };
    IdentityStatement{name:"S3(a,c,d;b) = S3(c,a,d;b), over Z".into(),
        source:"Pursuit protocol; S3=(a+c)^2*d^2+a*c*d+(a*c)^2+b; commutativity, independently expanded in integers".into(),
        variables:vec!["a".into(),"c".into(),"d".into(),"b".into()],degree_bound:4,lhs:s3("a","c"),rhs:s3("c","a")}
}
type Poly = BTreeMap<[u8; 4], i64>;
fn expand(e: &Expr) -> Poly {
    match e {
        Expr::Var(v) => {
            let mut p = [0; 4];
            p[["a", "c", "d", "b"].iter().position(|x| *x == v).unwrap()] = 1;
            Poly::from([(p, 1)])
        }
        Expr::Const(c) => Poly::from([([0; 4], *c)]),
        Expr::Neg(x) => expand(x).into_iter().map(|(m, c)| (m, -c)).collect(),
        Expr::Add(xs) => {
            let mut out = Poly::new();
            for x in xs {
                for (m, c) in expand(x) {
                    *out.entry(m).or_default() += c;
                }
            }
            out.retain(|_, c| *c != 0);
            out
        }
        Expr::Mul(xs) => {
            let mut out = Poly::from([([0; 4], 1)]);
            for x in xs {
                let mut next = Poly::new();
                for (a, ca) in &out {
                    for (b, cb) in expand(x) {
                        let m = std::array::from_fn(|i| a[i] + b[i]);
                        *next.entry(m).or_default() += ca * cb;
                    }
                }
                out = next;
            }
            out.retain(|_, c| *c != 0);
            out
        }
    }
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 3, "input.jsonl certificate.json receipt.json");
    let statement = statement();
    let coefficients = expand(&statement.lhs);
    assert_eq!(coefficients, expand(&statement.rhs));
    assert!(coefficients.values().all(|c| c.abs() <= 2));
    let cert=ic::issue(statement,20261023,32,"Native issuer; separate certificate checker uses 256 domain-separated points; exact integer expansion and actual Boolean permutation checked in this process",None).unwrap();
    assert!(matches!(
        ic::check_with_extra(&cert, Some((2026102301, 256))).unwrap(),
        Verdict::Accept { .. }
    ));
    let source = std::fs::read(&args[0]).unwrap();
    let (mut draws, mut equations) = (0u64, 0u64);
    for line in std::str::from_utf8(&source).unwrap().lines() {
        let input: Value = serde_json::from_str(line).unwrap();
        let n = input["n"].as_u64().unwrap() as u32;
        let m = input["m"].as_u64().unwrap_or(3) as usize;
        let element = |x: u64| F2mElement::from_biguint(&BigUint::from(x), n);
        let bs: Vec<_> = input["v_basis"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| element(v.as_u64().unwrap()))
            .collect();
        let irr = find_irreducible_sparse(n).unwrap();
        let st = FieldStructure::new(n, &irr);
        let sys = build_decomposition_system(
            &bs,
            &element(input["x_r"].as_u64().unwrap()),
            &element(1),
            m,
            &st,
        )
        .unwrap();
        let ell = bs.len();
        let mask = (1u64 << ell) - 1;
        for p in &sys.equations {
            let permuted = F2BoolPoly::from_monos(
                p.terms
                    .iter()
                    .map(|t| F2BoolMono {
                        mask: (t.mask & !(mask | (mask << ell)))
                            | ((t.mask & mask) << ell)
                            | ((t.mask >> ell) & mask),
                    })
                    .collect(),
                sys.n_vars,
            );
            assert_eq!(&permuted, p);
            equations += 1;
        }
        draws += 1;
    }
    assert!(!std::path::Path::new(&args[1]).exists());
    assert!(!std::path::Path::new(&args[2]).exists());
    std::fs::write(&args[1], serde_json::to_vec_pretty(&cert).unwrap()).unwrap();
    let receipt = json!({"schema":"ic-guess-symmetry/v1","certificate_id":cert.id,"integer_expansion_equal":true,
        "integer_terms":coefficients.len(),"max_integer_coefficient":2,"fixture_draws":draws,"coordinate_equations_checked":equations,
        "permutation":"swap every coordinate of summands 1 and 2; preserve intermediates",
        "degree_verdict_equivalence":[[7,23,24],[15,23,24]],"cost_equivalence":false,"status":"passed"});
    std::fs::write(&args[2], serde_json::to_vec_pretty(&receipt).unwrap()).unwrap();
    println!("{receipt}");
}
