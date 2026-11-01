//! Fixed software fixture, never a campaign draw or producer result.
use anyhow::{ensure, Result};
use ml_dataset_independent_consumer::{audit, common};
use serde_json::{json, Map, Value};
use std::fs;
use std::path::Path;

pub fn add(
    cell: &Path,
    artifacts: &mut Vec<Value>,
    name: &str,
    bytes: &[u8],
    role: &str,
) -> Result<()> {
    let path = cell.join(name);
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, bytes)?;
    artifacts.push(json!({"path":name,"role":role,"encoding":"json","bytes":bytes.len(),"sha256":common::digest(bytes)}));
    Ok(())
}

pub fn fixture(root: &Path, scalar: &str) -> Result<()> {
    ensure!(!root.exists(), "fixture destination must not exist");
    fs::create_dir_all(root)?;
    let protocol = serde_json::to_vec(
        &json!({"schema":"experiment.protocol/v1","software_fixture":true,
        "scientific_observation":false,"description":"Fixed F17 arithmetic and storage validation only"}),
    )?;
    let plan_fields = [
        ("schema", json!("crypto.ml-dataset.plan/v1")),
        ("field_sizes", json!([5])),
        ("arities", json!([2])),
        ("families", json!(["prime"])),
        ("solvers", json!(["bsgs"])),
        ("curve_seeds", json!(1)),
        ("generators", json!(1)),
        ("keys", json!(1)),
        ("seed", json!(20261028)),
        ("factor_base_size", json!(2)),
        ("degree_bound", json!(3)),
        ("work_limit", json!(1000)),
        ("twists", json!(true)),
        ("protocol_sha256", json!(common::digest(&protocol))),
        ("split_rule", json!("curve-twist-component-sha256-v1")),
    ];
    let plan = Value::Object(
        plan_fields
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v))
            .collect::<Map<_, _>>(),
    );
    let plan_bytes = serde_json::to_vec(&plan)?;
    let plan_sha = common::digest(&plan_bytes);
    let run = json!({"schema":"crypto.ml-dataset.run/v1","plan_sha256":plan_sha,
        "source_commit":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","binary_sha256":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "shard_index":0,"shard_count":1});
    let config = audit::config_at(&plan, 0)?;
    let cell = root.join("cells/000000000000");
    fs::create_dir_all(&cell)?;
    let mut artifacts = Vec::new();
    for (name, bytes) in [
        ("plan.json", plan_bytes),
        ("run.json", serde_json::to_vec(&run)?),
        ("protocol.json", protocol),
    ] {
        fs::write(root.join(name), &bytes)?;
        add(&cell, &mut artifacts, name, &bytes, "restricted-provenance")?;
    }
    add(
        &cell,
        &mut artifacts,
        "config.json",
        &serde_json::to_vec(&config)?,
        "restricted-provenance",
    )?;
    for name in ["attempt.json", "worker-process.json"] {
        add(
            &cell,
            &mut artifacts,
            name,
            b"{\"software_fixture\":true}",
            "process-provenance",
        )?;
    }
    for name in ["stdout.log", "stderr.log"] {
        add(&cell, &mut artifacts, name, b"", "process-log")?;
    }
    let public = json!({"schema":"ml-ecdlp-instance/v1","family":"prime","n":5,"m":2,"twist":false,
        "split_component":"field-size-n5","field":{"characteristic":"17","modulus":"17","degree":1,"basis":"canonical-residue"},
        "curve":{"model":"short-weierstrass","a":"2","b":"2","subgroup_order":"19"},
        "generator":{"x":"5","y":"1"},"target":{"x":"6","y":"3"}});
    let private = json!({"schema":"ml-ecdlp-private-witness/v1","scalar":"PRIVATE-SCALAR-NEVER-A-FEATURE",
        "sampling_provenance":{"seed":config["seed"],"curve_seed":config["curve_seed"],
            "generator_index":config["generator_index"],"key_index":config["key_index"]}});
    let base = json!({"schema":"ml-ecdlp-factor-base/v1","requested_size":2,
        "rule":"small-x-cofactor-projection","construction_status":"verified",
        "points":[{"x":"5","y":"1"},{"x":"6","y":"3"}]});
    let certificate =
        json!({"scalar":scalar,"generator":public["generator"],"target":public["target"]});
    for (name, value, role) in [
        ("public.json", public, "public-instance"),
        ("witness.json", private, "restricted-known-answer"),
        ("factor-base.json", base, "factor-base"),
        (
            "recovery.json",
            certificate,
            "restricted-independent-scalar-label",
        ),
    ] {
        add(
            &cell,
            &mut artifacts,
            name,
            &serde_json::to_vec(&value)?,
            role,
        )?;
    }
    let manifest = json!({"schema":"crypto.ml-dataset.cell/v1","cell_index":0,"plan_sha256":plan_sha,
        "config":config,"outcome":{"status":"verified","solver_completed":true,
            "scalar_recovered_without_witness":true,"scalar_replay_verified":true},"artifacts":artifacts});
    fs::write(cell.join("manifest.json"), serde_json::to_vec(&manifest)?)?;
    Ok(())
}

pub fn receipt(root: &Path, remote: &str) -> Result<Value> {
    let cell = root.join("cells/000000000000");
    let manifest_bytes = fs::read(cell.join("manifest.json"))?;
    let manifest = common::read_json_bytes(&manifest_bytes)?;
    let sha = common::digest(&manifest_bytes);
    let mut artifacts = manifest["artifacts"].as_array().unwrap().clone();
    artifacts.push(
        json!({"path":"manifest.json","role":"cell-manifest","encoding":"json",
        "bytes":manifest_bytes.len(),"sha256":sha}),
    );
    let objects: Vec<_> = artifacts
        .into_iter()
        .map(|artifact| {
            let hash = artifact["sha256"].as_str().unwrap();
            json!({"object_key":format!("sha256/{}/{}", &hash[..2],hash),"artifact":artifact})
        })
        .collect();
    Ok(
        json!({"schema":"ml-dataset.publication/v1","remote":remote,"manifest_sha256":sha,"objects":objects,
        "software_fixture":true,"scientific_observation":false}),
    )
}
