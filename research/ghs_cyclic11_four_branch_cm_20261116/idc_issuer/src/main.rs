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
fn main() {
    let name=std::env::args().nth(1).expect("choose e2 or e24");
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
    }else {panic!("unknown candidate")};
    let cert=issue(statement,seed,32,
        "issued by standalone Rust using repository identity_certificate module; checker replay uses 32 independent extra points",None)
        .expect("identity issue");
    assert!(matches!(check_with_extra(&cert,Some((seed^0x55aa,32))).unwrap(),Verdict::Accept{..}));
    println!("{}",serde_json::to_string_pretty(&cert).unwrap());
}
