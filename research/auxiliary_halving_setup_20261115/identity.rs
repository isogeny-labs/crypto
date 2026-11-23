// Reuse the repository's unchanged native certificate issuer and checker.
#[allow(dead_code)]
#[path = "../../src/hash/sha256.rs"]
mod sha256_impl;
mod hash {
    pub mod sha256 {
        pub use crate::sha256_impl::sha256;
    }
}
#[allow(dead_code)]
#[path = "../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;

use identity_certificate::{Expr, IdentityCertificate, IdentityStatement, Verdict};
fn product(items: Vec<Expr>) -> Expr { Expr::Mul(items) }
fn sum(items: Vec<Expr>) -> Expr { Expr::Add(items) }
fn power(value: &Expr, degree: usize) -> Expr { product(vec![value.clone(); degree]) }
fn c(value: i64) -> Expr { Expr::Const(value) }
fn statement() -> IdentityStatement {
    let u = Expr::var("u");
    let a = Expr::var("a");
    let b = Expr::var("b");
    let x = Expr::var("xp");
    let numerator = sum(vec![product(vec![c(3), power(&u,2)]), a.clone()]);
    let cubic = sum(vec![power(&u,3),product(vec![a.clone(),u.clone()]),b.clone()]);
    IdentityStatement {
        name: "Cleared short-Weierstrass doubling abscissa quartic".into(),
        source: "MIT18.783 Fall2023 Lecture2 section2.2, https://math.mit.edu/classes/18.783/2023/LectureNotes2.pdf; clear 4*(u^3+a*u+b) in x(2H)=xp".into(),
        variables: vec!["u".into(),"a".into(),"b".into(),"xp".into()],
        degree_bound: 4,
        lhs: sum(vec![power(&numerator,2),product(vec![c(-4),cubic,sum(vec![product(vec![c(2),u.clone()]),x.clone()])])]),
        rhs: sum(vec![
            power(&u,4),
            product(vec![c(-4),x.clone(),power(&u,3)]),
            product(vec![c(-2),a.clone(),power(&u,2)]),
            product(vec![c(-8),b.clone(),u.clone()]),
            product(vec![c(-4),a.clone(),x.clone(),u]),
            power(&a,2),
            product(vec![c(-4),b,x]),
        ]),
    }
}
fn run() -> Result<(),Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len()!=3 {return Err("usage: halving-identity --issue NEW_JSON | --check JSON".into());}
    if args[1]=="--issue" {
        let certificate=identity_certificate::issue(
            statement(),1234,32,
            "Producer-created evaluated record; unchanged native issuer/checker. No signed PCC standing, integer identity proof or program-correctness proof.",
            None)?;
        let mut file=std::fs::OpenOptions::new().write(true).create_new(true).open(&args[2])?;
        serde_json::to_writer_pretty(&mut file,&certificate)?;
        use std::io::Write;
        file.write_all(b"\n")?;
        println!("ISSUED {}",certificate.id);
    } else if args[1]=="--check" {
        let certificate: IdentityCertificate=serde_json::from_slice(&std::fs::read(&args[2])?)?;
        let verdict=identity_certificate::check_with_extra(&certificate,Some((424242,128)))?;
        if !matches!(verdict,Verdict::Accept{..}) {return Err(format!("certificate rejected: {verdict:?}").into());}
        println!("PASS {} {verdict:?}; evaluated over F_(2^61-1) only",certificate.id);
    } else {return Err("unknown identity mode".into());}
    Ok(())
}
fn main() {if let Err(error)=run(){eprintln!("REJECT: {error}");std::process::exit(2);}}
