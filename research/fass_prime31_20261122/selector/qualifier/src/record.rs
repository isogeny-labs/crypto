//! Bounded identity-certificate and source-binding utility; no benchmark path.
use crypto::cryptanalysis::identity_certificate::{self as id, Expr as E, IdentityStatement};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}
fn mul(a: E, b: E) -> E {
    E::product(vec![a, b])
}
fn add(a: E, b: E) -> E {
    E::plus(a, b)
}
fn sub(a: E, b: E) -> E {
    add(a, E::minus(b))
}
fn pow(a: E, n: usize) -> E {
    E::product((0..n).map(|_| a.clone()).collect())
}

fn certificates(out: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(out)?;
    let u = E::var("u");
    let v = E::var("v");
    let a = E::var("a");
    let b = E::var("b");
    let c = E::var("c");
    let y = E::var("y");
    let q = add(
        add(mul(a.clone(), pow(y.clone(), 2)), mul(b.clone(), y.clone())),
        c.clone(),
    );
    let qminus = add(
        sub(mul(a.clone(), pow(y.clone(), 2)), mul(b.clone(), y.clone())),
        c.clone(),
    );
    let statements = vec![
        IdentityStatement {
            name: "Lucas even doubling in root-power coordinates".into(),
            source: "selector/REPORT.md; universal ring derivation".into(),
            variables: vec!["u".into(), "v".into()],
            degree_bound: 2,
            lhs: add(pow(u.clone(), 2), pow(v.clone(), 2)),
            rhs: sub(
                pow(add(u.clone(), v.clone()), 2),
                mul(E::Const(2), mul(u.clone(), v.clone())),
            ),
        },
        IdentityStatement {
            name: "Lucas odd doubling in root-power coordinates".into(),
            source: "selector/REPORT.md; u=alpha^n,v=beta^n".into(),
            variables: vec!["u".into(), "v".into(), "alpha".into(), "beta".into()],
            degree_bound: 3,
            lhs: add(
                mul(E::var("alpha"), pow(u.clone(), 2)),
                mul(E::var("beta"), pow(v.clone(), 2)),
            ),
            rhs: sub(
                mul(
                    add(u.clone(), v.clone()),
                    add(
                        mul(E::var("alpha"), u.clone()),
                        mul(E::var("beta"), v.clone()),
                    ),
                ),
                mul(add(E::var("alpha"), E::var("beta")), mul(u, v)),
            ),
        },
        IdentityStatement {
            name: "Quadratic selector squaring step".into(),
            source: "selector/REPORT.md; q(y)q(-y)".into(),
            variables: vec!["a".into(), "b".into(), "c".into(), "y".into()],
            degree_bound: 6,
            lhs: mul(q, qminus),
            rhs: add(
                add(
                    mul(pow(a.clone(), 2), pow(y.clone(), 4)),
                    mul(
                        sub(mul(E::Const(2), mul(a, c.clone())), pow(b, 2)),
                        pow(y, 2),
                    ),
                ),
                pow(c, 2),
            ),
        },
    ];
    let mut records = Vec::new();
    for (i, statement) in statements.into_iter().enumerate() {
        let cert = id::issue(
            statement,
            20261122 + i as u64,
            32,
            "producer-issued; checked by existing repository checker; no signed PCC or Lean proof",
            None,
        )?;
        let verdict = id::check_with_extra(&cert, Some((0x5e1ec70 + i as u64, 32)))?;
        match verdict {
            id::Verdict::Accept { .. } => {}
            _ => return Err("identity checker rejected certificate".into()),
        }
        let name = format!("{}.json", cert.id);
        write_new(&out.join(&name), &serde_json::to_vec_pretty(&cert)?)?;
        records.push(json!({"path":name,"id":cert.id,"status":"evaluated_not_formally_proved","checker":format!("{verdict:?}")}));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &json!({"schema":"fass-selector.identity-controls/v1","records":records})
        )?
    );
    Ok(())
}
fn manifest(out: &Path, binary: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
    let mut value: Value = serde_json::from_str(crypto::SOURCE_BUILD_MANIFEST)?;
    let root = Path::new(env!("FASS_SOURCE_ROOT"));
    for source in value["sources"].as_array().ok_or("missing sources")? {
        let relative = source["path"].as_str().ok_or("source path")?;
        let bytes = fs::read(root.join(relative))?;
        let actual = format!("{:x}", Sha256::digest(&bytes));
        if source["sha256"] != actual {
            return Err(format!("source changed since build: {relative}").into());
        }
    }
    if let Some(path) = binary {
        let bytes = fs::read(path)?;
        value["binary"] = json!({"filename":path.file_name().unwrap().to_string_lossy(),"sha256":format!("{:x}",Sha256::digest(&bytes)),"bytes":bytes.len()});
    }
    value["binding_checked_at_call"] = json!(true);
    write_new(out, &serde_json::to_vec_pretty(&value)?)?;
    println!("source and binary binding recorded at {}", out.display());
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [command, out] if command == "certificates" => certificates(&PathBuf::from(out)),
        [command, out] if command == "manifest" => manifest(Path::new(out), None),
        [command, out, binary] if command == "manifest" => {
            manifest(Path::new(out), Some(Path::new(binary)))
        }
        _ => Err(
            "usage: selector-record manifest NEW_PATH [BINARY] | certificates NEW_DIRECTORY".into(),
        ),
    }
}
