//! Native evaluated-tier certificate for the oriented-node factorization.
//! Uses unchanged crypto_lib identity_certificate; no formal kernel proof.
use crypto_lib::cryptanalysis::identity_certificate::{
    check_with_extra, issue, Expr, IdentityCertificate, IdentityStatement, Verdict,
};
use serde_json::json;
use std::error::Error;
use std::path::Path;

fn v(s: &str) -> Expr {
    Expr::var(s)
}
fn c(n: i64) -> Expr {
    Expr::Const(n)
}
fn add(xs: Vec<Expr>) -> Expr {
    Expr::sum(xs)
}
fn mul(xs: Vec<Expr>) -> Expr {
    Expr::product(xs)
}
fn sub(x: Expr, y: Expr) -> Expr {
    add(vec![x, Expr::minus(y)])
}
fn sq(x: Expr) -> Expr {
    mul(vec![x.clone(), x])
}
fn rhs(x: Expr) -> Expr {
    add(vec![
        mul(vec![x.clone(), x.clone(), x.clone()]),
        mul(vec![v("a"), x]),
        v("b"),
    ])
}

fn statement() -> IdentityStatement {
    IdentityStatement {
        name: "Oriented S3 node cubic factorization with explicit residuals".into(),
        source: "Prime31 iteration6 RATIONAL_CHAIN.md; residuals encode the root curve equation and the two quadratic node equations. Standard chord-tangent arithmetic; no novelty claim.".into(),
        variables: ["r", "T", "lambda", "s", "a", "b", "u", "v"].iter().map(|x| x.to_string()).collect(),
        degree_bound: 4,
        lhs: sub(rhs(add(vec![v("r"), v("T")])), sq(sub(mul(vec![v("lambda"), v("T")]), v("s")))),
        rhs: add(vec![
            mul(vec![v("T"), sub(v("T"), v("u")), sub(v("T"), v("v"))]),
            sub(rhs(v("r")), sq(v("s"))),
            mul(vec![add(vec![mul(vec![c(3), v("r")]), Expr::minus(sq(v("lambda"))), v("u"), v("v")]), sq(v("T"))]),
            mul(vec![add(vec![mul(vec![c(3), sq(v("r"))]), v("a"), mul(vec![c(2), v("s"), v("lambda")]), Expr::minus(mul(vec![v("u"), v("v")]))]), v("T")]),
        ]),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (checking, directory, checker_seed) = match args.as_slice() {
        [directory] => (false, Path::new(directory), 0x0126_0613_5042_07AB),
        [flag, directory, seed] if flag == "--check" => {
            (true, Path::new(directory), seed.parse::<u64>()?)
        }
        _ => {
            return Err(
                "usage: identity_certificate_rational_chain OUT_DIR | --check OUT_DIR SEED".into(),
            )
        }
    };
    let path = directory.join("oriented-node-factorization.json");
    let s = statement();
    let cert: IdentityCertificate = if checking {
        let cert: IdentityCertificate = serde_json::from_slice(&std::fs::read(&path)?)?;
        if cert.statement != s {
            return Err("stored statement differs from the frozen issuer".into());
        }
        cert
    } else {
        std::fs::create_dir_all(directory)?;
        let cert = issue(s, 0xD1C5_0A06_2026_1122, 32,
            "Native evaluated-tier producer certificate; additional replay recorded separately. No formal proof or signed PCC verdict.", None)?;
        std::fs::write(&path, format!("{}\n", serde_json::to_string_pretty(&cert)?))?;
        cert
    };
    let verdict = check_with_extra(&cert, Some((checker_seed, 64)))?;
    let (points, extra_points) = match verdict {
        Verdict::Accept {
            points,
            extra_points,
            ..
        } => (points, extra_points),
        other => return Err(format!("native identity check rejected: {other:?}").into()),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema":"rational-chain-native-identity-v1", "mode":if checking {"replay"} else {"issue_and_replay"},
            "file":"oriented-node-factorization.json", "id":cert.id, "modulus":cert.modulus,
            "points":points, "extra_points":extra_points, "checker_seed":checker_seed,
            "proof":cert.proof, "scope":"Native evaluated tier; explicit residual identity only; no solver or asymptotic certificate."
        }))?
    );
    Ok(())
}
