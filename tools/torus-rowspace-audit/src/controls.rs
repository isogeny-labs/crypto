use crate::{audit, check, Audit, Limits, RawSystem, Result, System, Term};
use flate2::read::GzDecoder;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Read, path::Path};
pub const ORIGINAL:&[u8]=include_bytes!("../../../research/curve25519_torus_native_20261115/target-circuit-v2/native-v2/six-member-system.json.gz");
pub fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn sources() -> Value {
    json!({"Cargo.toml":sha(include_bytes!("../Cargo.toml")),"Cargo.lock":sha(include_bytes!("../Cargo.lock")),"src/lib.rs":sha(include_bytes!("lib.rs")),"src/check.rs":sha(include_bytes!("check.rs")),"src/controls.rs":sha(include_bytes!("controls.rs")),"src/main.rs":sha(include_bytes!("main.rs")),"original_gzip":sha(ORIGINAL)})
}
fn system(rows: Vec<Vec<(Vec<usize>, u32)>>) -> System {
    let equations = rows
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|(monomial, c)| Term {
                    monomial,
                    coefficient: c.to_string(),
                })
                .collect()
        })
        .collect();
    let raw = RawSystem {
        schema: "torus-s4.quadratic/v1".into(),
        p: "29".into(),
        variables: vec!["x".into(), "y".into(), "z".into()],
        equations,
    };
    System::parse(&serde_json::to_vec(&raw).unwrap()).unwrap()
}
pub fn small_cases() -> Vec<(&'static str, System, usize)> {
    vec![
        (
            "new-quadratic",
            system(vec![
                vec![(vec![0], 1), (vec![1], 1)],
                vec![(vec![0, 0], 1), (vec![1], 1)],
            ]),
            1,
        ),
        (
            "private-auxiliary",
            system(vec![
                vec![(vec![0, 0], 28), (vec![1], 1)],
                vec![(vec![0, 1], 28), (vec![2], 1)],
            ]),
            0,
        ),
        (
            "existing-input",
            system(vec![
                vec![(vec![0], 28), (vec![0, 0], 1)],
                vec![(vec![0, 0], 28), (vec![1], 1)],
            ]),
            0,
        ),
        (
            "zero-image",
            system(vec![
                vec![(vec![0], 28), (vec![1], 1)],
                vec![(vec![0], 27), (vec![1], 2)],
            ]),
            0,
        ),
        (
            "constant-conflict",
            system(vec![
                vec![(vec![0], 1), (vec![1], 1)],
                vec![(vec![], 28), (vec![0], 1), (vec![1], 1)],
            ]),
            1,
        ),
    ]
}
pub fn small_contracts() -> Result<Value> {
    let limits = Limits::default();
    for (id, s, expected) in small_cases() {
        let a = audit(&s, vec![0], &limits)?;
        let r = check::verify(&s, &a, &limits)?;
        if r.new_input_relation_dimension != expected {
            return Err(format!("wrong exact control {id}"));
        }
    }
    let (_, s, _) = small_cases().remove(0);
    let a = audit(&s, vec![0], &limits)?;
    let mut mutations: Vec<(&str, Audit)> = vec![];
    let mut b = a.clone();
    b.projection.steps[1].weights[0].1 = "2".into();
    mutations.push(("origin coefficient", b));
    let mut b = a.clone();
    b.projection.steps[0].weights.clear();
    mutations.push(("origin diagonal", b));
    let mut b = a.clone();
    b.projection.steps.pop();
    mutations.push(("source coverage", b));
    let mut b = a.clone();
    b.projection.steps[0].source = 1;
    mutations.push(("source order", b));
    let mut b = a.clone();
    b.projection.steps[0].pivot = Some(vec![0]);
    mutations.push(("invented pivot", b));
    let mut b = a.clone();
    b.images[0][0].coefficient = "3".into();
    mutations.push(("wrong image", b));
    let mut b = a.clone();
    b.keep.push(0);
    mutations.push(("duplicate keep", b));
    let mut b = a.clone();
    b.projection.steps[0].row[0].coefficient = "01".into();
    mutations.push(("noncanonical coefficient", b));
    for (id, b) in &mutations {
        if check::verify(&s, b, &limits).is_ok() {
            return Err(format!("accepted corruption {id}"));
        }
    }
    for l in [
        Limits {
            work: 0,
            ..limits.clone()
        },
        Limits {
            writes: 0,
            ..limits.clone()
        },
        Limits {
            entries: 0,
            ..limits.clone()
        },
    ] {
        if audit(&s, vec![0], &l).is_ok() || check::verify(&s, &a, &l).is_ok() {
            return Err("forced cap admitted".into());
        }
    }
    let mut encoded = serde_json::to_value(&a).map_err(|e| e.to_string())?;
    encoded["extra"] = json!(1);
    if serde_json::from_value::<Audit>(encoded).is_ok() {
        return Err("extra serialization field accepted".into());
    }
    if System::parse(&vec![b' '; (2 << 20) + 1]).is_ok() {
        return Err("input cap admitted".into());
    }
    Ok(
        json!({"matrix_controls":5,"corruptions_refused":mutations.len(),"forced_generator_and_reader_caps":3,"extra_field_refused":true,"oversize_input_refused":true}),
    )
}
fn write(path: &Path, value: &impl serde::Serialize) -> Result<()> {
    fs::write(
        path,
        serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}
fn one(out: &Path, id: &str, s: &System, keep: Vec<usize>) -> Result<Value> {
    let dir = out.join(id);
    fs::create_dir(&dir).map_err(|e| e.to_string())?;
    write(
        &dir.join("input.json"),
        &json!({"system":s.raw,"keep":keep,"limits":Limits::default()}),
    )?;
    let a = match audit(s, keep, &Limits::default()) {
        Ok(a) => a,
        Err(reason) => {
            let v = json!({"id":id,"outcome":"Unknown","reason":reason});
            write(&dir.join("summary.json"), &v)?;
            return Ok(v);
        }
    };
    let summary = check::verify(s, &a, &Limits::default())?;
    write(&dir.join("certificate.json"), &a)?;
    let v = json!({"id":id,"outcome":"CompleteStaticCertificate","summary":summary,"generator_meter":a.generator_meter,"standing":"producer unsigned/unverified","solver_calls":0});
    write(&dir.join("summary.json"), &v)?;
    Ok(v)
}
pub fn run(out: &Path) -> Result<()> {
    fs::create_dir(out).map_err(|e| e.to_string())?;
    write(
        &out.join("pre-call.json"),
        &json!({"schema":"rowspace.static-controls/v1","compiled_sources":sources(),"planned_cases":7,"solver_calls":0,"scope":"fixed static development certificates; no performance/family claim"}),
    )?;
    let contracts = small_contracts()?;
    let mut results = vec![];
    for (id, s, _) in small_cases() {
        results.push(one(out, id, &s, vec![0])?);
    }
    if sha(ORIGINAL) != "5b92bf1e7ef2b468175262d0a77e762386f83e364271a34f830624f1f9e87522" {
        return Err("frozen original input hash".into());
    }
    let mut decoded = vec![];
    GzDecoder::new(ORIGINAL)
        .take((2 << 20) + 1)
        .read_to_end(&mut decoded)
        .map_err(|e| e.to_string())?;
    let s = System::parse(&decoded)?;
    if s.raw.variables.len() != 1059 || s.equations.len() != 1089 {
        return Err("original shape".into());
    }
    let mut raw = vec![];
    let mut inputs = vec![];
    for (i, name) in s.raw.variables.iter().enumerate() {
        let Some(rest) = name.strip_prefix("member") else {
            continue;
        };
        let Some((leaf, stem)) = rest.split_once('_') else {
            continue;
        };
        if !["0", "1", "2", "3", "4", "5"].contains(&leaf) {
            continue;
        }
        let coordinate = stem == "z_a_0" || stem == "z_b_1";
        if coordinate {
            raw.push(i);
        }
        if coordinate
            || ["selector_", "slack_", "branch_", "raw_v_"]
                .iter()
                .any(|tag| stem.starts_with(tag))
        {
            inputs.push(i);
        }
    }
    if raw.len() != 12 || inputs.len() != 204 {
        return Err(format!(
            "frozen role adapter: raw{} inputs{}",
            raw.len(),
            inputs.len()
        ));
    }
    results.push(one(out, "original-raw12", &s, raw)?);
    results.push(one(out, "original-input204", &s, inputs)?);
    write(
        &out.join("controls.json"),
        &json!({"schema":"torus.rowspace-results/v1","contracts":contracts,"planned":7,"attempted":results.len(),"cases":results,"original_input_gzip_sha256":sha(ORIGINAL),"native_aggregation":"Rust","solver_calls":0,"accounting_complete":false,"standing":"producer unsigned/unverified"}),
    )
}
