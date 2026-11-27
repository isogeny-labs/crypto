//! Two fixed algebraic building blocks; no curve, relation-yield or timing experiment.
use crypto_lib::cryptanalysis::identity_certificate::{self as ic, Expr as E, IdentityStatement as S, Verdict, IssueError};
use serde_json::json;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
fn v(n: &str)->E { E::var(n) }
fn add(a:E,b:E)->E { E::plus(a,b) }
fn mul(a:E,b:E)->E { E::times(a,b) }
fn neg(a:E)->E { E::minus(a) }
fn sq(a:E)->E { mul(a.clone(),a) }
fn statements()->Vec<(&'static str,S)> {
    let dot=S {
        name:"Two-by-two finite-sum reassociation (building block only)".into(),
        source:"RESEARCH_402.md section1; distributivity, no group or rank claim".into(),
        variables:["u1","u2","r11","r12","r21","r22","b1","b2"].map(str::to_owned).to_vec(),
        degree_bound:3,
        lhs:add(mul(v("u1"),add(mul(v("r11"),v("b1")),mul(v("r12"),v("b2")))),mul(v("u2"),add(mul(v("r21"),v("b1")),mul(v("r22"),v("b2"))))),
        rhs:add(mul(add(mul(v("u1"),v("r11")),mul(v("u2"),v("r21"))),v("b1")),mul(add(mul(v("u1"),v("r12")),mul(v("u2"),v("r22"))),v("b2")))
    };
    let cubic=add(add(add(mul(sq(v("x")),v("x")),neg(mul(sq(v("m")),sq(v("x"))))),mul(add(v("A"),neg(mul(E::Const(2),mul(v("m"),v("c"))))),v("x"))),add(v("B"),neg(sq(v("c")))));
    let line=S {
        name:"Affine line substitution into short Weierstrass equation".into(),
        source:"RESEARCH_402.md section3; direct substitution y=m*x+c, not a subgroup or splitting claim".into(),
        variables:["x","m","c","A","B"].map(str::to_owned).to_vec(), degree_bound:4,
        lhs:add(sq(add(mul(v("m"),v("x")),v("c"))),neg(add(add(mul(sq(v("x")),v("x")),mul(v("A"),v("x"))),v("B")))), rhs:neg(cubic)
    };
    vec![("sum-reassociation",dot),("line-substitution",line)]
}
fn write_new(p:&Path, value:&impl serde::Serialize) {
    let mut out=OpenOptions::new().write(true).create_new(true).open(p).unwrap();
    serde_json::to_writer_pretty(&mut out,value).unwrap(); writeln!(out).unwrap();
}
fn main() {
    let dest=std::env::args().nth(1).expect("new output directory"); let root=Path::new(&dest);fs::create_dir(root).unwrap();let mut rows=vec![];
    for (name,statement) in statements() {
        let cert=ic::issue(statement.clone(),0,32,"Same local issuer/checker implementation; extra stream domain-separated. No independent-host or signed verification. Fixed correctness statements, not performance input selection.",None).unwrap();
        match ic::check_with_extra(&cert,Some((1,32))).unwrap() {
            Verdict::Accept{points:32,extra_points:32,..}=>{}, other=>panic!("positive check failed: {other:?}")
        }
        write_new(&root.join(format!("{name}.json")),&cert);
        let mut false_statement=statement.clone();false_statement.rhs=add(false_statement.rhs,E::Const(1));
        let counterexample=match ic::issue(false_statement.clone(),0,32,"Negative control",None) {
            Err(IssueError::False(c))=>json!({"index":c.index,"assignment":c.assignment,"lhs":c.lhs,"rhs":c.rhs}), other=>panic!("false statement not refused: {other:?}")
        };
        let mut forged=cert.clone();forged.statement=false_statement;forged.id=ic::statement_id(&forged.statement);
        assert!(matches!(ic::check(&forged).unwrap(),Verdict::Reject(_)),"ID-consistent tamper must fail actual evaluation");
        let mut digest=cert.clone();digest.value_digest.replace_range(0..1,if cert.value_digest.starts_with('0') {"1"} else {"0"});
        assert!(matches!(ic::check(&digest).unwrap(),Verdict::DigestMismatch{..}));
        let mut low=statement;low.degree_bound=0;assert!(matches!(ic::issue(low,0,32,"Malformed degree control",None),Err(IssueError::Malformed(_))));
        rows.push(json!({"name":name,"id":cert.id,"modulus":cert.modulus,"issuer_points":32,"extra_checker_points":32,"proof_pointer":cert.proof,"false_statement_counterexample":counterexample,"id_consistent_tamper_rejected":true,"digest_tamper_rejected":true,"understated_degree_rejected":true}));
    }
    println!("{}",json!({"schema":"relation-idea-identity-checks/v1","cases":rows,"performance_experiment":false,"independent_host":false,"scope":"Two fixed finite-dimensional polynomial identities over2^61-1 only. No arbitrary-size rank theorem, group membership, p31/p53, integer-identity or15x validation."}));
}
