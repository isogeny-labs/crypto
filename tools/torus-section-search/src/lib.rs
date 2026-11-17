//! Fixed identity records for the section-search derivation. No section solver.
pub mod hash {
    pub mod sha256 {
        use sha2::{Digest, Sha256};
        // Thin crate-interface adapter; SHA256 is the pinned sha2 dependency.
        pub fn sha256(bytes: &[u8]) -> [u8; 32] {
            Sha256::digest(bytes).into()
        }
    }
}
#[allow(dead_code)]
#[path = "../../../src/cryptanalysis/identity_certificate.rs"]
pub mod identity;

use identity::{Expr as E, IdentityStatement};
use serde_json::{json, Value};

fn sub(a: E, b: E) -> E {
    E::plus(a, E::minus(b))
}
fn mul(a: E, b: E) -> E {
    E::times(a, b)
}
fn sq(a: E) -> E {
    mul(a.clone(), a)
}
fn scale(n: i64, a: E) -> E {
    mul(E::Const(n), a)
}

pub fn statements() -> Vec<IdentityStatement> {
    let u = E::var("u");
    let c = E::var("c");
    let v = E::var("v");
    let w = E::var("w");
    let wb = E::var("wbar");
    let hv = E::plus(
        sub(
            sq(v.clone()),
            mul(sub(scale(4, sq(u.clone())), E::Const(2)), v.clone()),
        ),
        E::Const(1),
    );
    let ht = sub(
        E::plus(w.clone(), wb.clone()),
        sub(scale(4, sq(c.clone())), E::Const(2)),
    );
    let hp = sub(mul(w.clone(), wb.clone()), E::Const(1));
    let r = E::var("r");
    let s = E::var("s");
    let a = E::var("A");
    let z = E::var("c5");
    let curve = E::sum(vec![
        mul(sq(r.clone()), r.clone()),
        mul(a, sq(r.clone())),
        r,
    ]);
    vec![
        IdentityStatement {
            name:"Section-search geometric trace factor residual".into(),
            source:"section-search-v1/DERIVATION.md section3; classical ring algebra; evaluated only".into(),
            variables:["u","c","v","w","wbar"].map(str::to_string).to_vec(),
            degree_bound:3,
            lhs:sub(mul(sub(v.clone(),w),sub(v.clone(),wb)),scale(4,mul(v.clone(),sub(sq(u),sq(c))))),
            rhs:E::plus(sub(hv,mul(v,ht)),hp),
        },
        IdentityStatement {
            name:"Section-search target norm cleared residual".into(),
            source:"section-search-v1/DERIVATION.md section1; classical norm/distributivity; evaluated only".into(),
            variables:["r","s","A","c5"].map(str::to_string).to_vec(),
            degree_bound:5,
            lhs:sub(sq(mul(z.clone(),s.clone())),mul(curve.clone(),sq(z.clone()))),
            rhs:mul(sq(z),sub(sq(s),curve)),
        },
    ]
}

pub fn run_controls() -> Result<Value, String> {
    let mut records = vec![];
    for (i, s) in statements().into_iter().enumerate() {
        let cert = identity::issue(
            s.clone(),
            202611170845 + i as u64,
            32,
            "Producer fixed native evaluation only, unsigned; no independent verifier",
            None,
        )
        .map_err(|e| e.to_string())?;
        let verdict = identity::check_with_extra(&cert, Some((202611170846 + i as u64, 32)))
            .map_err(|e| e.to_string())?;
        let extra = match verdict {
            identity::Verdict::Accept {
                points,
                extra_points,
                ..
            } => json!({"status":"accept","issuer_points":points,"extra_points":extra_points}),
            _ => return Err("extra stream rejected".into()),
        };
        let mut bad = s;
        bad.rhs = E::plus(bad.rhs, E::Const(1));
        let refusal = identity::issue(bad, 202611170845 + i as u64, 32, "plus-one control", None)
            .err()
            .ok_or("false identity was accepted")?;
        records.push(json!({"certificate":cert,"extra_stream_verdict":extra,
            "plus_one_refusal":refusal.to_string()}));
    }
    Ok(json!({"schema":"section-search.identity-controls/v1",
        "scope":"two polynomial residual identities over the maintained 2^61-1 evaluation field; no quotient query, section search, Curve25519-field or exponent test",
        "records":records,"compiled_source_sha256":source_hashes()}))
}

pub fn source_hashes() -> Value {
    fn digest(b: &[u8]) -> String {
        hash::sha256::sha256(b)
            .iter()
            .map(|x| format!("{x:02x}"))
            .collect()
    }
    json!({
        "Cargo.toml":digest(include_bytes!("../Cargo.toml")),
        "Cargo.lock":digest(include_bytes!("../Cargo.lock")),
        "src/lib.rs":digest(include_bytes!("lib.rs")),
        "src/main.rs":digest(include_bytes!("main.rs")),
        "shared/identity_certificate.rs":digest(include_bytes!("../../../src/cryptanalysis/identity_certificate.rs")),
        "hash_backend":"sha2 0.10 with exact version/checksum in Cargo.lock; adapter in src/lib.rs"
    })
}
