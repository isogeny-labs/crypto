mod hash { pub mod sha256 {
    use sha2::{Digest,Sha256};
    pub fn sha256(data:&[u8])->[u8;32] { Sha256::digest(data).into() }
}}
#[path="identity_certificate.rs"]
mod identity_certificate;
use identity_certificate::{Expr,IdentityStatement,Verdict,issue,check_with_extra};
fn c(n:i64)->Expr {Expr::Const(n)}
fn plus(a:Expr,b:Expr)->Expr {Expr::plus(a,b)}
fn prod(xs:Vec<Expr>)->Expr {Expr::product(xs)}
fn pow(x:Expr,n:usize)->Expr {prod(vec![x;n])}
fn term(k:i64,x:&Expr,n:usize)->Expr {prod(vec![c(k),pow(x.clone(),n)])}
fn main() {
    let name=std::env::args().nth(1).expect("choose e2, e24, q2 or h2");
    let x=Expr::var("x");
    let xm1=plus(x.clone(),c(-1));
    let xm2=plus(x.clone(),c(-2));
    let two_minus_x=plus(c(2),Expr::minus(x.clone()));
    let one_minus_x=plus(c(1),Expr::minus(x.clone()));
    let (statement,seed)=if name=="e2" {
        (IdentityStatement{name:"F5 cyclic11 trace-2 involution numerator identity".into(),
            source:"research/ghs_cyclic11_four_branch_cm_20261116/RESULT.md".into(),
            variables:vec!["x".into()],degree_bound:3,
            lhs:prod(vec![x.clone(),xm1,c(2),two_minus_x]),
            rhs:prod(vec![c(2),x.clone(),xm2,one_minus_x])},
            0x2026_1116_0000_0002)
    }else if name=="e24" {
        (IdentityStatement{name:"F5 cyclic11 trace-24 involution numerator identity".into(),
            source:"research/ghs_cyclic11_four_branch_cm_20261116/RESULT.md".into(),
            variables:vec!["x".into()],degree_bound:19,
            lhs:prod(vec![x.clone(),pow(xm1,9),pow(two_minus_x,9)]),
            rhs:prod(vec![x.clone(),pow(xm2,9),pow(one_minus_x,9)])},
            0x2026_1116_0000_0024)
    }else if name=="q2" {
        let x2=pow(x.clone(),2);
        let u=plus(x2.clone(),c(2));
        let numerator=plus(prod(vec![x2.clone(),pow(xm1.clone(),2)]),pow(xm2.clone(),2));
        let denominator=prod(vec![x.clone(),xm1.clone(),xm2.clone()]);
        let left=prod(vec![numerator,x.clone(),plus(u.clone(),
            Expr::minus(prod(vec![c(3),x.clone()]))) ]);
        let bracket=Expr::sum(vec![pow(u.clone(),2),
            Expr::minus(prod(vec![c(2),x.clone(),u])),prod(vec![c(3),x2])]);
        let right=prod(vec![denominator,bracket]);
        let correction=prod(vec![c(5),pow(x.clone(),3),xm1,xm2]);
        (IdentityStatement{name:"F5 cyclic11 trace-2 quotient rational identity correction".into(),
            source:"research/ghs_cyclic11_four_branch_cm_20261116/RESULT.md".into(),
            variables:vec!["x".into()],degree_bound:7,
            lhs:right,rhs:plus(left,correction)},
            0x2026_1116_0000_0202)
    }else if name=="h2" {
        let v=Expr::var("v");
        let d=Expr::sum(vec![term(1,&v,11),term(-11,&v,9),term(44,&v,7),
            term(-77,&v,5),term(55,&v,3),term(-11,&v,1)]);
        let p=Expr::sum(vec![term(1,&v,5),term(1,&v,4),term(1,&v,3),
            term(2,&v,2),term(3,&v,1),c(1)]);
        let k=Expr::sum(vec![term(-2,&v,9),term(9,&v,7),term(2,&v,6),
            term(-13,&v,5),term(2,&v,4),term(14,&v,3),term(4,&v,2)]);
        (IdentityStatement{name:"F5 cyclic11 trace-2 Dickson square factor".into(),
            source:"research/ghs_cyclic11_four_branch_cm_20261116/HYPERELL_PROTOCOL.md".into(),
            variables:vec!["v".into()],degree_bound:11,
            lhs:plus(d,c(-2)),
            rhs:plus(prod(vec![plus(v,c(-2)),pow(p,2)]),prod(vec![c(5),k]))},
            0x2026_1116_0000_0205)
    }else {panic!("unknown candidate")};
    let cert=issue(statement,seed,32,
        "issued by standalone Rust using repository identity_certificate module; checker replay uses 32 independent extra points",None)
        .expect("identity issue");
    assert!(matches!(check_with_extra(&cert,Some((seed^0x55aa,32))).unwrap(),Verdict::Accept{..}));
    println!("{}",serde_json::to_string_pretty(&cert).unwrap());
}
