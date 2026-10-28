//! Native evidence admission, canonical model metadata and lossless exports.
#![allow(dead_code)]
#[path = "../../../../../src/hash/sha256.rs"]
pub mod source_sha256;
mod hash {
    pub use crate::source_sha256 as sha256;
}
mod binary_ecc {
    pub struct IrreduciblePoly {
        pub degree: u32,
        pub low_terms: Vec<u32>,
    }
}
#[path = "../../../../../src/cryptanalysis/curve_id.rs"]
mod curve_id;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
const S: &str = "research/twist_routes_1_3_20261021";
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn write(p: impl AsRef<Path>, v: &Value) -> Result<()> {
    if p.as_ref().exists() {
        return Err("fresh output required".into());
    }
    fs::write(p, serde_json::to_string_pretty(v)? + "\n")?;
    Ok(())
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn file(p: &Path, base: &Path, kind: &str) -> Result<Value> {
    let a = fs::canonicalize(p)?;
    let b = fs::canonicalize(base)?;
    let a = a.components().collect::<Vec<_>>();
    let b = b.components().collect::<Vec<_>>();
    let n = a.iter().zip(&b).take_while(|(a, b)| a == b).count();
    let mut rel = PathBuf::new();
    for _ in n..b.len() {
        rel.push("..");
    }
    for c in &a[n..] {
        rel.push(c.as_os_str());
    }
    Ok(json!({"path":rel,"sha256":sha(p)?,"bytes":fs::metadata(p)?.len(),"kind":kind}))
}
fn admit() -> Result<()> {
    let v = Path::new(S).join("execution-v9");
    let r = v.join("runs/rank-v1");
    let p = v.join("runs/pari-v2");
    let f = read(v.join("PARI_V2_FREEZE.json"))?;
    let receipt = read(p.join("RECEIPT.json"))?;
    assert_eq!(receipt["exit_code"], 0);
    assert!(receipt["signal"].is_null());
    assert_eq!(receipt["script_sha256"], sha(r.join("verify.gp"))?);
    assert_eq!(receipt["script_sha256"], f["main_script"]["sha256"]);
    assert_eq!(receipt["preload"]["sha256"], sha(v.join("PRELOAD_V2.gp"))?);
    assert_eq!(receipt["preload"]["sha256"], f["preload"]["sha256"]);
    assert_eq!(
        fs::read_to_string(p.join("pari.stdout"))?.trim(),
        "ALL_TWIST_RELATION_RANKS_OK 18"
    );
    assert!(fs::read(p.join("pari.stderr"))?.is_empty());
    for k in ["stdout", "stderr"] {
        assert_eq!(
            receipt[format!("{k}_sha256")],
            sha(p.join(format!("pari.{k}")))?
        );
    }
    let native = read(v.join("runs/producer-transport.json"))?;
    assert_eq!(native["exit_code"], 0);
    let data = read(r.join("RESULTS.json"))?;
    assert_eq!(
        data["input_sha256"],
        sha(Path::new(S).join("execution-v8/runs/kernel-v1/RESULTS.json"))?
    );
    write(
        v.join("ADMISSION.json"),
        &json!({"schema":"twist.relation-rank-admission/v1","status":"native-nullspace-and-independent-PARI-verified","summary":data["summary"],"cases":data["cases"],"native_result_sha256":sha(r.join("RESULTS.json"))?,"pari_success_receipt_sha256":sha(p.join("RECEIPT.json"))?,"failed_original_reference_preserved":true,"original_reference_admitted":false,"same_native_attempt_reused_after_reference_amendment":true,"speedup_established":false}),
    )
}
fn export(version: u64) -> Result<()> {
    let v = Path::new(S).join("execution-v9");
    let name = if version == 1 {
        "protocol.json"
    } else {
        "protocol-v2.json"
    };
    let p = read(v.join(name))?;
    let f = read(v.join("FROZEN.json"))?;
    let h = read(v.join("HOST.json"))?;
    let transport = read(v.join("runs/producer-transport.json"))?;
    let mut e = read(Path::new(S).join("execution-v8/experiment.export.json"))?;
    e["export_id"] = json!(format!("twist-relation-rank-20261027-v{version}"));
    e["protocol"] = json!({"path":name,"sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":p["version"]});
    e["source"]["commit"] = f["source_revision"].clone();
    e["host"] = json!({"path":"HOST.json","sha256":sha(v.join("HOST.json"))?,"class_id":null,"summary":h["summary"]});
    e["arms"]=json!(p["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":a["instrument"]["commit"]})).collect::<Vec<_>>());
    let artifacts = vec![
        file(&v.join("runs.tar.xz"), &v, "raw")?,
        file(&v.join("RAW_MEMBERS.json"), &v, "receipt")?,
        file(&v.join("runs/pari-v1/RECEIPT.json"), &v, "receipt")?,
        file(&v.join("runs/pari-v2/RECEIPT.json"), &v, "receipt")?,
    ];
    let mut runs = e["runs"].as_array().unwrap().clone();
    for (i, run) in runs.iter_mut().enumerate() {
        let arm = &p["arms"][i];
        let failed = i == 1 && version == 1;
        run["run_id"] = json!(if i == 0 {
            "rank-native-original-attempt".to_owned()
        } else {
            format!("rank-pari-attempt-{version}")
        });
        run["instance_id"] = json!("same-eighteen-archived-matrices");
        run["worker"] = arm["instrument"]["binary_sha256"].clone();
        run["measurements"]["counted"] = if failed {
            json!({})
        } else {
            json!({"systems":18})
        };
        run["measurements"]["counters"] = json!({});
        run["artifacts"] = json!(artifacts);
        run["status"] = json!(if failed { "failed" } else { "succeeded" });
        run["outcome_class"] = json!("completed");
        run["verification"] = json!({"status":if failed{"error"}else{"verified"},"verifier":"native RREF/nullspace and independent GP matker/matinverseimage; strict stderr gate"});
        run["backend_ref"] = json!(if i == 0 {
            "runs/rank-v1".into()
        } else {
            format!("runs/pari-v{version}")
        });
        run["started_at"] = if i == 0 {
            transport["started"].clone()
        } else {
            Value::Null
        };
        run["finished_at"] = if i == 0 {
            transport["finished"].clone()
        } else {
            Value::Null
        };
    }
    e["runs"] = json!(runs);
    e["flow"] = json!({"planned":2,"attempted":2,"completed":2,"not_completed":0,"interrupted":0,"resumed":0,"excluded":[]});
    e["files"] = json!([
        file(&v.join("FROZEN.json"), &v, "receipt")?,
        file(&v.join(name), &v, "protocol")?,
        file(&v.join("PARI_V2_FREEZE.json"), &v, "receipt")?
    ]);
    e["analysis"] = json!({"script":file(&Path::new(S).join("runner/src/bin/twist-relation-rank.rs"),&v,"analysis")?,"blinded":false,"blinding_key_sha256":null,"outputs":[file(&v.join("ADMISSION.json"),&v,"analysis")?],"unmasked_at":null});
    e["notes"] = json!([{ "kind":"observation","text":"Original export retains failed GP verification despite process0/sentinel. Version2 uses the same actual native attempt plus a separately frozen reference preload replay; do not pool exports as independent runs. No matrix, expected rank, witness, target or scientific native output changed.","by":"native publication tool"}]);
    e["producer"]["session"] = json!("twist-relation-rank-20261027");
    write(
        v.join(if version == 1 {
            "experiment.original.export.json"
        } else {
            "experiment.export.json"
        }),
        &e,
    )
}
fn models() -> Result<()> {
    let v = Path::new(S).join("execution-v8");
    let data = read(v.join("runs/kernel-v1/RESULTS.json"))?;
    let mut additions = vec![];
    let mut global = vec![];
    let field = json!({"characteristic":827,"degree":1,"representation":"prime-field","prime_modulus":827,"element_encoding":"canonical nonnegative integer modulo827"});
    global.push(json!({"icv1":data["source"]["slug"],"field":field,"curve":{"model":"short-weierstrass","a":1,"b":15,"subgroup_order":823,"cofactor":1,"generator":data["generator"],"target_group":"E(F827)"}}));
    for m in data["maps"].as_array().unwrap() {
        let mut id = m["destination"].clone();
        let word = |k: &str| {
            id["params"][k]
                .as_str()
                .unwrap()
                .parse::<BigUint>()
                .unwrap()
        };
        let canonical =
            curve_id::prime(&word("p"), &word("a"), &word("b"), &BigUint::from(823u64)).unwrap();
        assert_eq!(canonical.slug, id["slug"]);
        assert_eq!(canonical.icv1, id["icv1"]);
        assert_eq!(canonical.model_json, id["model_json"]);
        id["sources"] = json!([format!("{S}/execution-v8/ADMISSION.json")]);
        additions.push(id.clone());
        global.push(json!({"icv1":id["slug"],"field":field,"curve":{"model":"short-weierstrass","a":m["map"]["a"],"b":m["map"]["b"],"subgroup_order":823,"cofactor":1,"generator":m["mapped_generator"],"target_group":"E(F827)"}}));
    }
    for (p, value) in [
        (v.join("registry-additions.json"), json!(additions)),
        (v.join("GLOBAL_MODELS_INPUT.json"), json!(global)),
    ] {
        if p.exists() {
            assert_eq!(read(&p)?, value);
        } else {
            write(&p, &value)?;
        }
    }
    // Existing registry integers exceed u64. Keep their original bytes rather
    // than round-tripping them through serde_json::Number (which uses f64).
    let old_registry = fs::read_to_string("docs/curves/registry.json")?;
    let registry = read("docs/curves/registry.json")?;
    let mut aliases = read("src/cryptanalysis/curve_aliases.json")?;
    let rows = registry["curves"].as_array().unwrap();
    let mut append = String::new();
    let mut all_additions = additions.clone();
    for path in [
        "execution-v1/registry-additions.json",
        "execution-v1/runs/isogeny-20261021T2320Z/E14-registry-additions.json",
        "execution-v1/pairing-fixture/registry-additions.json",
    ] {
        all_additions.extend(
            read(Path::new(S).join(path))?
                .as_array()
                .unwrap()
                .iter()
                .cloned(),
        );
    }
    if let Ok(extra) = read(Path::new(S).join("execution-v10/registry-additions.json")) {
        all_additions.extend(extra.as_array().unwrap().iter().cloned());
    }
    let mut appended = std::collections::BTreeSet::new();
    for id in &all_additions {
        let slug = id["slug"].as_str().unwrap();
        if let Some(old) = rows.iter().find(|v| v["slug"] == slug) {
            assert_eq!(old["model_json"], id["model_json"]);
        } else if appended.insert(slug.to_owned()) {
            let mut bytes = vec![];
            let mut ser = serde_json::Serializer::with_formatter(
                &mut bytes,
                serde_json::ser::PrettyFormatter::with_indent(b" "),
            );
            serde::Serialize::serialize(id, &mut ser)?;
            append.push_str(",\n");
            append.push_str(
                &String::from_utf8(bytes)?
                    .lines()
                    .map(|line| format!("  {line}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
        }
        for key in [id["slug"].as_str().unwrap(), id["icv1"].as_str().unwrap()] {
            aliases["aliases"][curve_id::normalise(key)] = json!(slug);
        }
    }
    let sorted: BTreeMap<_, _> = aliases["aliases"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    aliases["aliases"] = json!(sorted);
    let split = old_registry
        .rfind("\n ]\n}")
        .ok_or("unrecognized registry ending")?;
    fs::write(
        "docs/curves/registry.json",
        format!(
            "{}{}{}",
            &old_registry[..split],
            append,
            &old_registry[split..]
        ),
    )?;
    for (p, v, indent) in [(
        "src/cryptanalysis/curve_aliases.json",
        aliases,
        b"".as_slice(),
    )] {
        let mut bytes = vec![];
        let mut ser = serde_json::Serializer::with_formatter(
            &mut bytes,
            serde_json::ser::PrettyFormatter::with_indent(indent),
        );
        serde::Serialize::serialize(&v, &mut ser)?;
        bytes.push(b'\n');
        fs::write(p, bytes)?;
    }
    Ok(())
}
fn endo_models() -> Result<()> {
    let v = Path::new(S).join("execution-v10");
    let data = read(v.join("ADMISSION.json"))?;
    let mut id = data["intermediate_model"].clone();
    let word = |k: &str| {
        id["params"][k]
            .as_str()
            .unwrap()
            .parse::<BigUint>()
            .unwrap()
    };
    let canonical =
        curve_id::prime(&word("p"), &word("a"), &word("b"), &BigUint::from(823u64)).unwrap();
    assert_eq!(canonical.slug, id["slug"]);
    assert_eq!(canonical.icv1, id["icv1"]);
    assert_eq!(canonical.model_json, id["model_json"]);
    id["sources"] = json!([format!("{S}/execution-v10/ADMISSION.json")]);
    write(v.join("registry-additions.json"), &json!([id]))?;
    let old = read(Path::new(S).join("execution-v8/GLOBAL_MODELS_INPUT.json"))?;
    let field = old[0]["field"].clone();
    let mut destination = old[0].clone();
    destination["curve"]["generator"] = data["mapped_generator"].clone();
    write(
        v.join("GLOBAL_MODELS_INPUT.json"),
        &json!([old[0].clone(),{"icv1":id["slug"],"field":field,"curve":{"model":"short-weierstrass","a":data["velu"]["a"],"b":data["velu"]["b"],"subgroup_order":823,"cofactor":1,"generator":data["intermediate_generator"],"target_group":"E(F827)"}},destination]),
    )?;
    models()
}
fn endo_export() -> Result<()> {
    let v = Path::new(S).join("execution-v10");
    let p = read(v.join("protocol.json"))?;
    let f = read(v.join("FROZEN.json"))?;
    let a = read(v.join("ADMISSION.json"))?;
    assert_eq!(
        a["status"],
        "native-generic-and-independent-PARI-endomorphism-verified"
    );
    let h = read(v.join("HOST.json"))?;
    let transport = read(v.join("runs/producer-transport.json"))?;
    let mut e = read(Path::new(S).join("execution-v8/experiment.export.json"))?;
    e["export_id"] = json!("twist-endo-symmetry-20261027-v1");
    e["protocol"] = json!({"path":"protocol.json","sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":p["version"]});
    e["source"]["commit"] = f["source_revision"].clone();
    e["host"] = json!({"path":"HOST.json","sha256":sha(v.join("HOST.json"))?,"class_id":null,"summary":h["summary"]});
    e["arms"]=json!(p["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":a["instrument"]["commit"]})).collect::<Vec<_>>());
    let artifacts = vec![
        file(&v.join("runs.tar.xz"), &v, "raw")?,
        file(&v.join("RAW_MEMBERS.json"), &v, "receipt")?,
        file(&v.join("ADMISSION.json"), &v, "receipt")?,
        file(&v.join("runs/pari-v1/RECEIPT.json"), &v, "receipt")?,
    ];
    let mut runs = e["runs"].as_array().unwrap().clone();
    for (i, r) in runs.iter_mut().enumerate() {
        r["run_id"] = json!(format!(
            "endo-{}-000",
            if i == 0 { "native" } else { "pari" }
        ));
        r["instance_id"] = json!("p827-fixed-degree17-endomorphism");
        r["worker"] = p["arms"][i]["instrument"]["binary_sha256"].clone();
        r["artifacts"] = json!(artifacts);
        r["measurements"]["counted"] =
            json!({"source_rational_checks":a["source_rational_checks"]});
        r["measurements"]["counters"] = json!({"twist_rational_checks":a["twist_rational_checks"],"generic_norm_compositions":a["generic_norm_compositions_verified"],"scalar_multiplicative_order":a["scalar_multiplicative_order"]});
        r["verification"] = json!({"status":"verified","verifier":"native exact generic/finite endomorphism identities and independent PARI construction/group replay"});
        r["backend_ref"] = json!(if i == 0 {
            "runs/endo-v1"
        } else {
            "runs/pari-v1"
        });
        r["started_at"] = if i == 0 {
            transport["started"].clone()
        } else {
            Value::Null
        };
        r["finished_at"] = if i == 0 {
            transport["finished"].clone()
        } else {
            Value::Null
        };
    }
    e["runs"] = json!(runs);
    e["analysis"] = json!({"script":file(&Path::new(S).join("runner/src/endo_experiment_20261027.rs"),&v,"analysis")?,"blinded":false,"blinding_key_sha256":null,"outputs":[file(&v.join("ADMISSION.json"),&v,"analysis")?],"unmasked_at":null});
    e["files"] = json!([
        file(&v.join("FROZEN.json"), &v, "receipt")?,
        file(&v.join("protocol.json"), &v, "protocol")?
    ]);
    e["notes"] = json!([{"kind":"observation","text":"One fixed public degree17 endomorphism: exact generic Frobenius/norm identities, full source/twist finite checks and scalar action. Counts are exact, not independent DLP samples or calibrated cost. No novelty, unknown-log, NIST or speed claim.","by":"native publication tool"}]);
    e["producer"]["session"] = json!("twist-endo-symmetry-20261027");
    write(v.join("experiment.export.json"), &e)
}
fn symmetry_export() -> Result<()> {
    let v = Path::new(S).join("execution-v11");
    let p = read(v.join("protocol.json"))?;
    let f = read(v.join("FROZEN.json"))?;
    let a = read(v.join("ADMISSION.json"))?;
    assert_eq!(
        a["status"],
        "native-and-independent-PARI-kernel-symmetry-verified"
    );
    let h = read(v.join("HOST.json"))?;
    let transport = read(v.join("runs/producer-transport.json"))?;
    assert_eq!(transport["exit_code"], 0);
    assert_eq!(transport["binary_sha256"], f["binary"]["sha256"]);
    let mut e = read(Path::new(S).join("execution-v10/experiment.export.json"))?;
    e["export_id"] = json!("twist-kernel-symmetry-20261028-v1");
    e["created"] = transport["finished"].clone();
    e["protocol"] = json!({"path":"protocol.json","sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":p["version"]});
    e["source"]["commit"] = f["source_revision"].clone();
    e["host"] = json!({"path":"HOST.json","sha256":sha(v.join("HOST.json"))?,"class_id":null,"summary":h["summary"]});
    e["arms"]=json!(p["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":a["instrument"]["commit"]})).collect::<Vec<_>>());
    let artifacts = vec![
        file(&v.join("runs.tar.xz"), &v, "raw")?,
        file(&v.join("RAW_MEMBERS.json"), &v, "receipt")?,
        file(&v.join("ADMISSION.json"), &v, "receipt")?,
        file(&v.join("runs/pari-v1/RECEIPT.json"), &v, "receipt")?,
    ];
    let mut runs = e["runs"].as_array().unwrap().clone();
    assert_eq!(runs.len(), 2);
    for (i, r) in runs.iter_mut().enumerate() {
        r["run_id"] = json!(format!(
            "symmetry-{}-000",
            if i == 0 { "native" } else { "pari" }
        ));
        r["instance_id"] = json!("p827-archived-twist-seven-torsion");
        r["worker"] = p["arms"][i]["instrument"]["binary_sha256"].clone();
        r["artifacts"] = json!(artifacts);
        r["measurements"]["counted"] = json!({"point_checks":a["point_checks"]});
        r["measurements"]["counters"] = json!({"addition_checks":a["addition_checks"],"kernel_checks":a["kernel_checks"],"point_order":a["point_order"],"projective_order":a["projective_order"],"corrupt_matrix_rejections":a["corrupt_matrix_rejections"],"corrupt_line_rejections":a["corrupt_line_rejections"]});
        r["verification"] = json!({"status":"verified","verifier":"Native direct map and independent PARI ellisogeny/group/matrix/orbit/root-set replay"});
        r["backend_ref"] = json!(if i == 0 {
            "runs/symmetry-v1"
        } else {
            "runs/pari-v1"
        });
        r["started_at"] = if i == 0 {
            transport["started"].clone()
        } else {
            Value::Null
        };
        r["finished_at"] = if i == 0 {
            transport["finished"].clone()
        } else {
            Value::Null
        };
    }
    e["runs"] = json!(runs);
    e["analysis"] = json!({"script":file(&Path::new(S).join("runner/src/kernel_symmetry_20261028.rs"),&v,"analysis")?,"blinded":false,"blinding_key_sha256":null,"outputs":[file(&v.join("ADMISSION.json"),&v,"analysis")?],"unmasked_at":null});
    e["files"] = json!([
        file(&v.join("FROZEN.json"), &v, "receipt")?,
        file(&v.join("protocol.json"), &v, "protocol")?
    ]);
    e["notes"] = json!([{"kind":"observation","text":"Exact fixed archived seven-torsion module and eight kernels; complete action/cycles and controls. No performance, novelty, unknown-log or NIST claim. Shared author/host independent CAS implementation, no separate-runner review.","by":"native publication tool"}]);
    e["producer"]["session"] = json!("twist-kernel-symmetry-20261028");
    write(v.join("experiment.export.json"), &e)
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("admit") => admit(),
        Some("export-original") => export(1),
        Some("export") => export(2),
        Some("models") => models(),
        Some("endo-models") => endo_models(),
        Some("endo-export") => endo_export(),
        Some("symmetry-export") => symmetry_export(),
        _ => Err("admit|export-original|export|models".into()),
    }
}
