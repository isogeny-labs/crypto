//! Concrete identity exporter. No collector, F4 solver, point generation or DLP.
#![allow(dead_code)]
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::io::Write;
use std::path::Path;

pub mod hash {
    pub mod sha256 {
        use sha2::{Digest, Sha256};
        pub fn sha256(bytes: &[u8]) -> [u8; 32] {
            Sha256::digest(bytes).into()
        }
    }
}
#[path = "../../../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity;
use identity::{Expr as E, IdentityStatement, ProofRef, Verdict};

type Result<T> = std::result::Result<T, String>;
type M = [u8; 3];
type Poly = Vec<(M, i128)>;
const ID: &str = "p37-seed0-phi-l0-active-sys0-raw16";
const WORKLOAD: &str = "254581dfebfcb992e8ce0c6b7896cbd7cb402259b0a5b72e34fd8c711cc0c88a";
const CANDIDATE: &str = "35d1890ef4aa04d3a7568dca950f85274097079a4813db64e8022025a57ba2ec";
const PREFIX: &str = "fe7fbef221b2d391f3793976fd3babbe14673ef08b0ddaa3b5ae74664764773c";
const D: usize = 12;
const SIDE: usize = D + 1;
const SIZE: usize = SIDE * SIDE * SIDE;

fn require(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn uint(v: &Value) -> Result<u64> {
    v.as_u64().ok_or("expected unsigned number".into())
}
fn string(v: &Value) -> Result<&str> {
    v.as_str().ok_or("expected string".into())
}
fn array(v: &Value) -> Result<&Vec<Value>> {
    v.as_array().ok_or("expected array".into())
}
fn read(path: &Path) -> Result<Value> {
    serde_json::from_slice(&std::fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}
fn sha(path: &Path) -> Result<String> {
    Ok(hex::encode(Sha256::digest(
        std::fs::read(path).map_err(|e| e.to_string())?,
    )))
}
fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut f = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.write_all(bytes).map_err(|e| e.to_string())?;
    f.sync_all().map_err(|e| e.to_string())
}
fn json_new(path: &Path, v: &Value) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(v).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    write_new(path, &bytes)
}
fn bound(row: &Value, key: &str) -> Result<Value> {
    let path = Path::new(string(&row[format!("{key}_path")])?);
    require(
        sha(path)? == string(&row[format!("{key}_sha256")])?,
        "sealed reference hash mismatch",
    )?;
    read(path)
}
fn degree(m: M) -> usize {
    m.iter().map(|&x| x as usize).sum()
}
fn parse(v: &Value) -> Result<Poly> {
    let terms: Vec<(M, u64)> = serde_json::from_value(v.clone()).map_err(|e| e.to_string())?;
    let mut seen = BTreeSet::new();
    terms
        .into_iter()
        .map(|(m, c)| {
            require(degree(m) <= D, "monomial degree exceeds 12")?;
            require(c > 0 && c < 37, "noncanonical coefficient")?;
            require(seen.insert(m), "duplicate monomial")?;
            Ok((m, c as i128))
        })
        .collect()
}
fn at(m: M) -> usize {
    (m[0] as usize * SIDE + m[1] as usize) * SIDE + m[2] as usize
}
fn monomial(i: usize) -> M {
    [
        (i / (SIDE * SIDE)) as u8,
        ((i / SIDE) % SIDE) as u8,
        (i % SIDE) as u8,
    ]
}
fn accumulate(out: &mut [i128], a: &Poly, b: &Poly, sign: i128) -> Result<()> {
    for &(m, c) in a {
        for &(n, d) in b {
            let t = std::array::from_fn(|j| m[j] + n[j]);
            require(degree(t) <= D, "product degree exceeds 12")?;
            let value = c
                .checked_mul(d)
                .and_then(|v| v.checked_mul(sign))
                .ok_or("coefficient overflow")?;
            out[at(t)] = out[at(t)]
                .checked_add(value)
                .ok_or("coefficient overflow")?;
        }
    }
    Ok(())
}
fn q(a: u64) -> Poly {
    let a = a as i128;
    [
        ([0, 0, 0], a * a * a),
        ([1, 0, 0], -a * a),
        ([0, 1, 0], a),
        ([0, 0, 1], -1),
    ]
    .into_iter()
    .filter(|(_, c)| *c != 0)
    .collect()
}
fn lift(f: &[Poly], h: &[Poly], a: u64) -> Result<Poly> {
    require(
        f.len() == 4 && h.len() == 4,
        "four original equations and multipliers required",
    )?;
    let mut delta = vec![0i128; SIZE];
    for (fi, hi) in f.iter().zip(h) {
        accumulate(&mut delta, hi, fi, 1)?;
    }
    accumulate(&mut delta, &q(a), &q(a), -1)?;
    let mut remainder = Vec::new();
    for (i, c) in delta.into_iter().enumerate() {
        require(c % 37 == 0, "integer coefficient not divisible by 37")?;
        if c != 0 {
            remainder.push((monomial(i), c / 37));
        }
    }
    Ok(remainder)
}
fn eval(p: &Poly, e: [u64; 3]) -> u64 {
    p.iter().fold(0i128, |sum, (m, c)| {
        let mut v = *c;
        for j in 0..3 {
            for _ in 0..m[j] {
                v = (v * e[j] as i128).rem_euclid(37);
            }
        }
        (sum + v).rem_euclid(37)
    }) as u64
}
fn expression(poly: &Poly) -> String {
    if poly.is_empty() {
        return "(0 : R)".into();
    }
    poly.iter()
        .map(|(m, c)| {
            let mut pieces = vec![format!("({c} : R)")];
            for (i, &power) in m.iter().enumerate() {
                if power > 0 {
                    pieces.push(format!("s{}^{power}", i + 1));
                }
            }
            format!("({})", pieces.join(" * "))
        })
        .collect::<Vec<_>>()
        .join(" + ")
}
fn sdk_poly(poly: &Poly) -> E {
    E::sum(
        poly.iter()
            .map(|(m, c)| {
                let mut factors = vec![E::Const(
                    i64::try_from(*c).expect("bounded integer-lift coefficient"),
                )];
                for (i, &power) in m.iter().enumerate() {
                    for _ in 0..power {
                        factors.push(E::var(&format!("s{}", i + 1)));
                    }
                }
                E::product(factors)
            })
            .collect(),
    )
}
fn lean(prefix: &str, f: &[Poly], h: &[Poly], rem: &Poly, a: u64) -> String {
    let mut text = prefix.to_string();
    text.push_str(
        "\n-- Literal coefficients exported from the sealed instance; no assumed identity.\n",
    );
    text.push_str("set_option maxRecDepth 100000\nset_option maxHeartbeats 20000000\nnamespace ConcreteCertificate\nvariable {R : Type} [Lean.Grind.CommRing R]\n");
    for (name, polys) in [("f", f), ("h", h)] {
        for (i, poly) in polys.iter().enumerate() {
            text.push_str(&format!(
                "def {name}{i} (s1 s2 s3 : R) : R :=\n  {}\n",
                expression(poly)
            ));
        }
    }
    text.push_str(&format!(
        "def remainder (s1 s2 s3 : R) : R :=\n  {}\n",
        expression(rem)
    ));
    text.push_str(&format!(
        "def q (s1 s2 s3 : R) : R := ({a} : R)^3 - ({a} : R)^2*s1 + ({a} : R)*s2 - s3\n"
    ));
    let rhs = (0..4)
        .map(|i| format!("h{i} s1 s2 s3 * f{i} s1 s2 s3"))
        .collect::<Vec<_>>()
        .join(" + ");
    text.push_str(&format!("\ntheorem integer_lift (s1 s2 s3 : R) :\n    (q s1 s2 s3)^2 + 37 * remainder s1 s2 s3 =\n      {rhs} := by\n  unfold q remainder h0 h1 h2 h3 f0 f1 f2 f3\n  grind\n"));
    text.push_str(&format!("\ntheorem certificate_identity [Lean.Grind.IsCharP R 37] (s1 s2 s3 : R) :\n    (q s1 s2 s3)^2 = {rhs} := by\n  have hlift := integer_lift s1 s2 s3\n  grind\n"));
    text.push_str("\ntheorem repeated_factor_of_roots [Lean.Grind.IsCharP R 37]\n    (noZero : ∀ u v : R, u*v=0 → u=0 ∨ v=0)\n    (x y z : R)\n    (roots : f0 (x+y+z) (x*y+x*z+y*z) (x*y*z)=0 ∧\n      f1 (x+y+z) (x*y+x*z+y*z) (x*y*z)=0 ∧\n      f2 (x+y+z) (x*y+x*z+y*z) (x*y*z)=0 ∧\n      f3 (x+y+z) (x*y+x*z+y*z) (x*y*z)=0) :\n");
    text.push_str(&format!(
        "    ({a} : R)=x ∨ ({a} : R)=y ∨ ({a} : R)=z := by\n"
    ));
    text.push_str(
        "  have hcert := certificate_identity (x+y+z) (x*y+x*z+y*z) (x*y*z)\n  unfold q at hcert\n",
    );
    text.push_str(&format!(
        "  exact RepeatedFactor.rejection_of_certificate noZero ({a} : R) x y z\n"
    ));
    for name in ["f", "h"] {
        for i in 0..4 {
            text.push_str(&format!("    ({name}{i} (x+y+z) (x*y+x*z+y*z) (x*y*z))\n"));
        }
    }
    text.push_str("    2 roots hcert\n\n#print axioms integer_lift\n#print axioms certificate_identity\n#print axioms repeated_factor_of_roots\nend ConcreteCertificate\n");
    text
}

fn export(workload: &Path, candidate: &Path, prefix: &Path, out: &Path) -> Result<()> {
    require(sha(workload)? == WORKLOAD, "workload hash")?;
    require(sha(candidate)? == CANDIDATE, "candidate hash")?;
    require(sha(prefix)? == PREFIX, "generic proof dependency hash")?;
    require(!out.exists(), "fresh export directory required")?;
    let w = read(workload)?;
    let row = array(&w["workloads"])?
        .iter()
        .find(|r| r["id"] == ID)
        .ok_or("fixed case absent")?;
    let input = bound(row, "input")?;
    let original = bound(row, "original_input")?;
    let reference = bound(row, "reference")?;
    let audit = bound(row, "audit")?;
    let public = bound(row, "public")?;
    let prior = bound(row, "prior")?;
    let c = read(candidate)?;
    require(
        input["p"] == 37
            && input["seed"] == 0
            && input["arm"] == "phi-l0-active"
            && input["original_system_index"] == 0,
        "fixed instance identity",
    )?;
    require(
        c["status"] == "complete" && c["id"] == ID && c["input_sha256"] == row["input_sha256"],
        "candidate identity",
    )?;
    require(
        c["certificate"]["power"] == 2
            && c["certificate"]["degree_cap"] == 12
            && c["actual_certificate_product_degree"] == 12,
        "registered concrete certificate shape",
    )?;
    require(
        audit["status"] == "pass"
            && audit["candidate_sha256"] == row["reference_sha256"]
            && audit["public_input_sha256"] == row["public_sha256"],
        "source audit binding",
    )?;
    require(
        reference["status"] == "complete-polynomial-enumeration",
        "original complete reference",
    )?;
    let sys = &reference["systems"][0];
    require(
        input["equations"] == sys["equations"] && input["pair"] == sys["pair"],
        "source equation/pair binding",
    )?;
    require(
        input["equations"] == original["systems"][0]["equations"]
            && input["peeled_abscissa"] == original["systems"][0]["peeled_abscissa"],
        "original equation-only input binding",
    )?;
    let ordinary = uint(&input["pair"][0])? as usize;
    require(
        input["peeled_abscissa"] == public["base_coordinates"][ordinary][0][0],
        "peeled abscissa public binding",
    )?;
    require(
        prior["status"] == "complete" && prior["input_sha256"] == row["original_input_sha256"],
        "raw8 binding",
    )?;
    require(
        array(&c["attempts"])?[..5] == array(&prior["systems"][0]["attempts"])?[..],
        "raw4..8 prefix mismatch",
    )?;
    require(
        array(&sys["accepted_representative_relations"])?.is_empty(),
        "source has accepted physical relation",
    )?;
    let a = uint(&input["peeled_abscissa"])?;
    require(a < 37, "canonical a")?;
    let f = array(&input["equations"])?
        .iter()
        .map(parse)
        .collect::<Result<Vec<_>>>()?;
    let h = array(&c["certificate"]["multipliers"])?
        .iter()
        .map(parse)
        .collect::<Result<Vec<_>>>()?;
    let rem = lift(&f, &h, a)?;
    let mut roots_checked = 0;
    for root in array(&sys["field_roots"])? {
        let e: [u64; 3] = serde_json::from_value(root.clone()).map_err(|e| e.to_string())?;
        require(
            e.iter().all(|&v| v < 37) && f.iter().all(|p| eval(p, e) == 0) && eval(&q(a), e) == 0,
            "original finite root check",
        )?;
        roots_checked += 1;
    }
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let prefix_text = std::fs::read_to_string(prefix).map_err(|e| e.to_string())?;
    write_new(
        &out.join("ConcreteCertificate.lean"),
        lean(&prefix_text, &f, &h, &rem, a).as_bytes(),
    )?;
    for (name, path) in [
        ("source-input.json", Path::new(string(&row["input_path"])?)),
        ("source-candidate.json", candidate),
        ("source-workload.json", workload),
    ] {
        write_new(
            &out.join(name),
            &std::fs::read(path).map_err(|e| e.to_string())?,
        )?;
    }
    json_new(
        &out.join("statement.json"),
        &json!({"schema":"twist-certificate-integer-lift/v1","prime":37,"peeled_abscissa":a,"power":2,"degree":12,"equations":f,"multipliers":h,"remainder":rem,
        "identity":"sum(h_i*f_i) = q^2 + 37*remainder","candidate_sha256":CANDIDATE,"input_sha256":row["input_sha256"],"workload_sha256":WORKLOAD,"generic_prefix_sha256":PREFIX}),
    )?;
    json_new(
        &out.join("summary.json"),
        &json!({"schema":"twist-certificate-instance-export/v1","status":"pass","id":ID,"candidate_sha256":CANDIDATE,"input_sha256":row["input_sha256"],"original_reference_sha256":row["reference_sha256"],"dense_coefficient_positions_checked":SIZE,
        "nonzero_integer_remainder_coefficients":rem.len(),"all_integer_division_remainders_zero":true,"retained_roots_checked":roots_checked,
        "lean_source_sha256":sha(&out.join("ConcreteCertificate.lean"))?,"statement_sha256":sha(&out.join("statement.json"))?,"lean_proof_checked":false,"scope":"Fresh dense integer expansion and literal Lean export; formal proof, IDC, group/decoder bridge and any cost claim are separate."}),
    )
}
fn check_lean(stdout: &Path, proof: &Path, statement: &Path, out: &Path) -> Result<()> {
    let text = std::fs::read_to_string(stdout).map_err(|e| e.to_string())?;
    let names = [
        "RepeatedFactor.cubic_identity",
        "RepeatedFactor.power_zero",
        "RepeatedFactor.rejection_of_certificate",
        "ConcreteCertificate.integer_lift",
        "ConcreteCertificate.certificate_identity",
        "ConcreteCertificate.repeated_factor_of_roots",
    ];
    for name in names {
        require(text.contains(name), "missing theorem axiom report")?;
    }
    require(
        !text.contains("sorryAx")
            && !text.contains("error:")
            && !text.contains("Lean.ofReduceBool"),
        "invalid proof output",
    )?;
    for axioms in text.split("axioms:").skip(1) {
        let list = axioms
            .trim()
            .split_once('[')
            .ok_or("axiom list")?
            .1
            .split_once(']')
            .ok_or("axiom list end")?
            .0;
        for axiom in list.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            require(
                ["propext", "Classical.choice", "Quot.sound"].contains(&axiom),
                "nonstandard axiom",
            )?;
        }
    }
    json_new(
        out,
        &json!({"schema":"twist-instance-lean-axiom-audit/v1","status":"pass","theorems":names,"allowed_foundations":["propext","Classical.choice","Quot.sound"],"lean_stdout_sha256":sha(stdout)?,"proof_sha256":sha(proof)?,"statement_sha256":sha(statement)?}),
    )
}
fn idc(statement: &Path, proof: &Path, lean_check: &Path, out: &Path) -> Result<()> {
    let checked = read(lean_check)?;
    require(
        checked["status"] == "pass"
            && checked["proof_sha256"] == sha(proof)?
            && checked["statement_sha256"] == sha(statement)?,
        "Lean audit and source/statement binding required before IDC",
    )?;
    require(!out.exists(), "fresh IDC directory required")?;
    let s = read(statement)?;
    let f: Vec<Poly> = serde_json::from_value(s["equations"].clone()).map_err(|e| e.to_string())?;
    let h: Vec<Poly> =
        serde_json::from_value(s["multipliers"].clone()).map_err(|e| e.to_string())?;
    let rem: Poly = serde_json::from_value(s["remainder"].clone()).map_err(|e| e.to_string())?;
    require(
        lift(&f, &h, uint(&s["peeled_abscissa"])?)? == rem,
        "integer lift replay",
    )?;
    let q = sdk_poly(&q(uint(&s["peeled_abscissa"])?));
    let statement=IdentityStatement{name:"Sealed p37 phi system0 repeated-factor certificate, exact integer lift".into(),source:"ConcreteCertificate.integer_lift; literal coefficient export bound to the registered raw16 record".into(),variables:vec!["s1".into(),"s2".into(),"s3".into()],degree_bound:12,
        lhs:E::sum(f.iter().zip(&h).map(|(f,h)|E::times(sdk_poly(h),sdk_poly(f))).collect()),
        rhs:E::plus(E::times(q.clone(),q),E::times(E::Const(37),sdk_poly(&rem)))};
    let proof_sha = sha(proof)?;
    let certificate=identity::issue(statement.clone(),20261025,64,"One native issue/check process with a disjoint extra-point domain; separate kernel-checked literal Lean identity, no independent-model attestation claimed.",Some(ProofRef{kind:"lean4".into(),path:proof.display().to_string(),sha256:proof_sha.clone()})).map_err(|e|format!("IDC issue: {e:?}"))?;
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    json_new(
        &out.join("identity.json"),
        &serde_json::to_value(&certificate).map_err(|e| e.to_string())?,
    )?;
    let stored: identity::IdentityCertificate =
        serde_json::from_value(read(&out.join("identity.json"))?).map_err(|e| e.to_string())?;
    let (points, extra) = match identity::check_with_extra(&stored, Some((0x494e53545f503337, 64)))
        .map_err(|e| format!("IDC check: {e:?}"))?
    {
        Verdict::Accept {
            points,
            extra_points,
            ..
        } => (points, extra_points),
        other => return Err(format!("IDC rejected: {other:?}")),
    };
    let mut false_statement = statement;
    false_statement.lhs = E::plus(false_statement.lhs, E::Const(1));
    let control = identity::issue(
        false_statement,
        20261025,
        64,
        "False integer-lift +1 rejection control",
        None,
    );
    require(
        matches!(control, Err(identity::IssueError::False(_))),
        "false IDC not refused",
    )?;
    write_new(
        &out.join("false-identity-control.txt"),
        format!("{control:?}\n").as_bytes(),
    )?;
    json_new(
        &out.join("summary.json"),
        &json!({"schema":"twist-certificate-instance-idc/v1","status":"pass","id":certificate.id,"points":points,"extra_points":extra,"false_plus_one_refused":true,"proof_sha256":proof_sha,"scope":"Exact integer lift IDC; the characteristic-37 consequence and repeated-factor theorem are separately kernel checked. No cost, parametric-family or curve/decoder proof is implied."}),
    )
}
fn run() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice(){
        ["export",w,c,p,o]=>export(Path::new(w),Path::new(c),Path::new(p),Path::new(o)),
        ["check-lean",s,p,t,o]=>check_lean(Path::new(s),Path::new(p),Path::new(t),Path::new(o)),
        ["idc",s,p,l,o]=>idc(Path::new(s),Path::new(p),Path::new(l),Path::new(o)),
        _=>Err("usage: export WORKLOAD CANDIDATE PREFIX FRESH_OUT | check-lean STDOUT PROOF STATEMENT FRESH_JSON | idc STATEMENT PROOF LEAN_AUDIT FRESH_OUT".into())
    }
}
fn main() {
    if let Err(e) = run() {
        eprintln!("certificate instance verification rejected: {e}");
        std::process::exit(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn poly(v: &[(M, i128)]) -> Poly {
        v.to_vec()
    }
    #[test]
    fn exact_square_and_integer_lift() {
        let f = vec![poly(&[([0, 0, 2], 1)]), vec![], vec![], vec![]];
        let h = vec![poly(&[([0, 0, 0], 1)]), vec![], vec![], vec![]];
        assert!(lift(&f, &h, 0).unwrap().is_empty());
        let h = vec![poly(&[([0, 0, 0], 38)]), vec![], vec![], vec![]];
        assert_eq!(lift(&f, &h, 0).unwrap(), poly(&[([0, 0, 2], 1)]));
    }
    #[test]
    fn false_modular_identity_refused() {
        let f = vec![poly(&[([0, 0, 2], 1)]), vec![], vec![], vec![]];
        let h = vec![poly(&[([0, 0, 0], 2)]), vec![], vec![], vec![]];
        assert!(lift(&f, &h, 0).unwrap_err().contains("not divisible"));
    }
    #[test]
    fn malformed_terms_refused() {
        assert!(parse(&json!([[[0, 0, 0], 1], [[0, 0, 0], 2]])).is_err());
        assert!(parse(&json!([[[0, 0, 0], 37]])).is_err());
        assert!(parse(&json!([[[13, 0, 0], 1]])).is_err());
    }
    #[test]
    fn product_cap_refused() {
        let mut out = vec![0; SIZE];
        assert!(accumulate(
            &mut out,
            &poly(&[([12, 0, 0], 1)]),
            &poly(&[([1, 0, 0], 1)]),
            1
        )
        .is_err());
    }
}
