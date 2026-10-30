//! Local SOFTWARE workflow extension of the existing native fixture controller.
//! This stage never grants protocol approval or reports full acceptance/corpus work.
use super::*;
use crypto_lib::ml_dataset::types::plan_v2::{FactorBasePolicy, PlanDocument};
use serde::de::{MapAccess, Visitor};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

const MODULE_SOURCE: &str = include_str!("acceptance.rs");
const CUSTODY_SOURCE: &str = include_str!("control_custody.rs");
const MAX_JSON: u64 = 32 * 1024 * 1024;
const MAX_STORAGE: u64 = 8 * 1024 * 1024 * 1024;
const IDS: [&str; 7] = [
    "v1-bsgs",
    "v2-bsgs",
    "v2-empty-bsgs",
    "v2-empty-pdp",
    "v2-pkm-cap",
    "v2-lp-smallwork",
    "planner-coverage",
];
const METADATA: [(&str, &str); 7] = [
    (
        "cohorts.json",
        "47c173a65e8bd6e78e54ff9da1ea7977721882cb59234f1de64aa61cf24d45cf",
    ),
    (
        "native-cells.json",
        "5260171870d74e0ffa93a5f59cba254ccb71877e524f61db66308b97a6326172",
    ),
    (
        "limits.json",
        "3b8ba1ea191a66d97ac2c99dfbee57a2bec40c518dd7f5344c19177957628f03",
    ),
    (
        "planner-coverage.json",
        "0cdb694decd35099daab8e80333bebea2d9b06c5240820919c23ac9d64cff522",
    ),
    (
        "normal-policy.json",
        "5c13b42a0787a50d5a113f423149ba3b0cc741049092382990fe641b14a0f88c",
    ),
    (
        "empty-policy.json",
        "65a0a64c8dde6ee8da09b83c200920d9ba73a080b149be97c4bf49a78dce7434",
    ),
    (
        "paired-policy.json",
        "fcd4aecd9702a92e68d320314dd06d08010d67851121343001ea854205240101",
    ),
];
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn field<'a>(value: &'a Value, name: &str) -> Result<&'a str> {
    value[name]
        .as_str()
        .ok_or_else(|| format!("missing string {name}").into())
}
fn number(value: &Value, name: &str) -> Result<u64> {
    value[name]
        .as_u64()
        .ok_or_else(|| format!("missing integer {name}").into())
}
fn bounded(path: &Path) -> Result<Vec<u8>> {
    bounded_limit(path, MAX_JSON)
}
fn bounded_hash(path: &Path) -> Result<String> {
    Ok(digest(&bounded(path)?))
}
fn bounded_limit(path: &Path, cap: u64) -> Result<Vec<u8>> {
    require(
        fs::symlink_metadata(path)?.is_file()
            && !fs::symlink_metadata(path)?.file_type().is_symlink(),
        "JSON must be a regular file",
    )?;
    require(fs::metadata(path)?.len() <= cap, "JSON input limit")?;
    read_bounded(File::open(path)?, cap)
}
fn read_bounded(reader: impl Read, cap: u64) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader.take(cap + 1).read_to_end(&mut bytes)?;
    require(bytes.len() as u64 <= cap, "JSON grew beyond input limit")?;
    Ok(bytes)
}
// Preserve original bytes and reject duplicate keys at every nesting level.
fn unique(bytes: &[u8], depth: usize) -> Result<Value> {
    require(depth <= 64, "JSON nesting limit")?;
    struct Pairs;
    impl<'de> Visitor<'de> for Pairs {
        type Value = Vec<(String, Box<serde_json::value::RawValue>)>;
        fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("JSON object")
        }
        fn visit_map<A: MapAccess<'de>>(
            self,
            mut map: A,
        ) -> std::result::Result<Self::Value, A::Error> {
            let mut rows = Vec::new();
            let mut seen = BTreeSet::new();
            while let Some((key, value)) =
                map.next_entry::<String, Box<serde_json::value::RawValue>>()?
            {
                if !seen.insert(key.clone()) {
                    return Err(serde::de::Error::custom("duplicate JSON key"));
                }
                rows.push((key, value));
            }
            Ok(rows)
        }
    }
    let first = bytes.iter().copied().find(|b| !b.is_ascii_whitespace());
    if first == Some(b'{') {
        let mut decoder = serde_json::Deserializer::from_slice(bytes);
        let rows = serde::de::Deserializer::deserialize_map(&mut decoder, Pairs)?;
        decoder.end()?;
        let mut map = serde_json::Map::new();
        for (key, raw) in rows {
            map.insert(key, unique(raw.get().as_bytes(), depth + 1)?);
        }
        Ok(Value::Object(map))
    } else if first == Some(b'[') {
        let rows: Vec<Box<serde_json::value::RawValue>> = serde_json::from_slice(bytes)?;
        Ok(Value::Array(
            rows.iter()
                .map(|v| unique(v.get().as_bytes(), depth + 1))
                .collect::<Result<Vec<_>>>()?,
        ))
    } else {
        Ok(serde_json::from_slice(bytes)?)
    }
}
fn json_file(root: &Path, name: &str) -> Result<Value> {
    unique(&bounded(&checked_file(root, name)?)?, 0)
}
fn frozen(value: &Value) -> Result<()> {
    require(
        value["schema"] == "experiment.protocol/v1",
        "unknown protocol schema",
    )?;
    let mut body = value.clone();
    let freeze = body
        .as_object_mut()
        .ok_or("protocol object required")?
        .remove("freeze")
        .ok_or("missing freeze")?;
    require(
        freeze["status"] == "frozen" && freeze["filed_before_affected_data"] == true,
        "protocol is not frozen before affected data",
    )?;
    require(
        freeze["protocol_sha256"]
            == digest(&crypto_lib::ml_dataset::protocol_canonical_bytes(&body)?),
        "frozen protocol body mismatch",
    )
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileBinding {
    sha256: String,
    bytes: u64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema: String,
    source_commit: String,
    binary_sha256: String,
    files: BTreeMap<String, FileBinding>,
}
struct Admission {
    plans: Vec<PlanDocument>,
    cells: Vec<Value>,
    manifest_sha256: String,
    inputs: Value,
}
fn checked_files(root: &Path, manifest: &Manifest) -> Result<()> {
    require(
        (26..=64).contains(&manifest.files.len()),
        "fixture file-count bound",
    )?;
    let mut total = 0u64;
    for (name, row) in &manifest.files {
        require(row.bytes <= MAX_JSON, "fixture individual file limit")?;
        total = total
            .checked_add(row.bytes)
            .ok_or("fixture input byte overflow")?;
        require(total <= 128 * 1024 * 1024, "fixture aggregate input limit")?;
        let path = checked_file(root, name)?;
        require(
            fs::metadata(&path)?.len() == row.bytes
                && digest(&bounded_limit(&path, row.bytes)?) == row.sha256,
            "frozen fixture file changed",
        )?;
    }
    Ok(())
}
fn plan_scope(plan: &PlanDocument, protocol: &Value) -> Result<()> {
    if let PlanDocument::V2(v2) = plan {
        let mut scope = v2.clone();
        scope.axes.protocol_sha256 = None;
        let mut bytes = b"crypto.ml-dataset.protocol-plan-scope/v1\0".to_vec();
        bytes.extend(serde_json::to_vec(&scope)?);
        require(
            protocol["extensions"]["ml_dataset_policy_plan"]
                == json!({"schema":"crypto.ml-dataset.protocol-plan-scope/v1","scope_sha256":digest(&bytes),"planned_cells":plan.cell_count()?}),
            "protocol scope does not bind complete V2 plan",
        )?;
    }
    Ok(())
}
fn check_population(
    plans: &[PlanDocument],
    cells: &[Value],
    cohorts: &Value,
    broad: &Value,
) -> Result<()> {
    require(
        plans.len() == 7 && cells.len() == 26,
        "wrong deterministic fixture population",
    )?;
    for (i, plan) in plans.iter().enumerate() {
        let source = if i < 6 { &cohorts[i] } else { broad };
        let axes = plan.axes();
        require(
            serde_json::to_value(&axes.field_sizes)? == source["n"]
                && serde_json::to_value(&axes.arities)? == source["m"]
                && serde_json::to_value(&axes.families)? == source["families"]
                && serde_json::to_value(&axes.solvers)? == source["solvers"]
                && axes.curve_seeds == number(source, "curve_seeds")?
                && axes.generators == number(source, "generators")?
                && axes.keys == number(source, "keys")?
                && axes.twists,
            "plan axes differ from frozen population",
        )?;
        require(
            plan.cell_count()?
                == number(
                    source,
                    if i < 6 {
                        "planned_cell_count"
                    } else {
                        "planned_cells"
                    },
                )?,
            "plan count differs",
        )?;
        let expected_policies = &source["policies"];
        match plan {
            PlanDocument::V1(_) => require(
                i == 0 && expected_policies.is_null(),
                "unexpected legacy origin",
            )?,
            PlanDocument::V2(v2) => require(
                serde_json::to_value(&v2.factor_base_policies)? == *expected_policies,
                "policy order/caps differ",
            )?,
        }
        if i < 6 {
            require(
                axes.seed == 42
                    && axes.factor_base_size as u64 == number(source, "factor_base_size")?
                    && axes.degree_bound as u64 == number(source, "degree_bound")?
                    && axes.work_limit == number(source, "work_limit")?,
                "native scalar parameters differ",
            )?;
        }
    }
    let mut seen = BTreeSet::new();
    for (ordinal, row) in cells.iter().enumerate() {
        let id = field(row, "plan")?;
        let i = IDS.iter().position(|x| *x == id).ok_or("unknown cohort")?;
        require(i < 6, "planning-only plan cannot enter native generation")?;
        let index = number(row, "global_index")?;
        let spec = plans[i].cell(index)?;
        require(
            number(row, "ordinal")? == ordinal as u64
                && seen.insert((i, index))
                && index == number(row, "shard_index")?
                && number(row, "shard_count")? == plans[i].cell_count()?
                && spec.policy_index == number(row, "policy_index")?
                && spec.axes_cell_index == number(row, "axes_index")?,
            "ordinal/global shard/policy identity mismatch",
        )?;
        let config = serde_json::to_value(&spec.config)?;
        for name in [
            "family",
            "n",
            "m",
            "twist",
            "curve_seed",
            "generator_index",
            "key_index",
            "solver",
            "work_limit",
        ] {
            require(config[name] == row[name], "native selected config differs")?;
        }
        require(
            spec.config.n == 7
                && spec.config.m == 2
                && spec.config.seed == 42
                && spec.config.curve_seed == 7
                && spec.config.generator_index == 1
                && spec.config.key_index == 1,
            "unexpected native fixture size/seed/key",
        )?;
    }
    Ok(())
}
fn admit(args: &Args, root: &Path) -> Result<Admission> {
    let manifest_path = checked_file(root, "acceptance-manifest.json")?;
    let manifest_bytes = bounded(&manifest_path)?;
    let manifest_sha256 = digest(&manifest_bytes);
    require(
        args.acceptance_manifest_sha256.as_deref() == Some(&manifest_sha256),
        "explicit frozen acceptance manifest digest required",
    )?;
    let manifest: Manifest = serde_json::from_value(unique(&manifest_bytes, 0)?)?;
    require(
        manifest.schema == "ml-dataset.native-policy-acceptance-inputs/v1"
            && manifest.source_commit == args.source_commit
            && manifest.binary_sha256 == args.binary_sha256,
        "manifest source/binary mismatch",
    )?;
    checked_files(root, &manifest)?;
    for (name, sha) in METADATA {
        require(
            manifest.files.get(name).is_some_and(|v| v.sha256 == sha)
                && bounded_hash(&checked_file(root, name)?)? == sha,
            "reviewed population/limits bytes changed",
        )?;
    }
    for (name, source) in [
        ("run.rs", RUNNER_SOURCE),
        ("acceptance.rs", MODULE_SOURCE),
        ("control_custody.rs", CUSTODY_SOURCE),
        ("controls.rs", controls::source()),
    ] {
        require(
            manifest
                .files
                .get(name)
                .is_some_and(|v| v.sha256 == digest(source.as_bytes())),
            "compiled runner/module differs from manifest",
        )?;
    }
    let umbrella = json_file(root, "umbrella.json")?;
    frozen(&umbrella)?;
    let mut plans = Vec::new();
    let mut children = Vec::new();
    for id in IDS {
        let plan_name = format!("plans/{id}.json");
        let protocol_name = format!("protocols/{id}.json");
        require(
            manifest.files.contains_key(&plan_name) && manifest.files.contains_key(&protocol_name),
            "missing child file binding",
        )?;
        let plan = PlanDocument::parse_json(&bounded(&checked_file(root, &plan_name)?)?)?;
        let protocol = json_file(root, &protocol_name)?;
        frozen(&protocol)?;
        plan_scope(&plan, &protocol)?;
        let protocol_sha = bounded_hash(&checked_file(root, &protocol_name)?)?;
        require(
            plan.axes().protocol_sha256.as_deref() == Some(protocol_sha.as_str()),
            "plan lacks exact child protocol hash",
        )?;
        children.push(json!({"id":id,"plan_sha256":plan.binding_sha256()?,"plan_file_sha256":bounded_hash(&checked_file(root,&plan_name)?)?,"protocol_file_sha256":protocol_sha,"protocol_body_sha256":protocol["freeze"]["protocol_sha256"]}));
        plans.push(plan);
    }
    let limits = json_file(root, "limits.json")?;
    let scope = &umbrella["extensions"]["native_policy_acceptance"];
    bindings(
        scope,
        &json!({"schema":"ml-dataset.native-policy-acceptance-scope/v1","classification":"SOFTWARE","source_commit":args.source_commit,"binary_sha256":args.binary_sha256,
        "runner_sha256":digest(RUNNER_SOURCE.as_bytes()),"module_sha256":digest(MODULE_SOURCE.as_bytes()),"custody_sha256":digest(CUSTODY_SOURCE.as_bytes()),"controls_sha256":digest(controls::source().as_bytes()),"children":children,"limits":limits,"native_workers":26,"workflow_count":2}),
    )?;
    require(
        umbrella["analysis_plan"]["script_sha256"] == digest(RUNNER_SOURCE.as_bytes()),
        "umbrella does not bind native runner",
    )?;
    let ready = json_file(root, "readiness.json")?;
    require(
        manifest.files.contains_key("readiness.json")
            && manifest.files.contains_key("umbrella.json"),
        "missing readiness/umbrella custody",
    )?;
    bindings(
        &ready,
        &json!({"schema":"ml-dataset.native-policy-readiness/v1","source_commit":args.source_commit,"binary_sha256":args.binary_sha256,"umbrella_sha256":bounded_hash(&checked_file(root,"umbrella.json")?)?,"limits":limits,"approved_for_local_software":true,"cloud_execution_authorized":false}),
    )?;
    let gates = ready["gates"]
        .as_object()
        .ok_or("missing pre-freeze gates")?;
    let expected = [
        "native-fixture-unit-tests",
        "unified-dataset-tests",
        "maintained-clippy",
        "official-umbrella-schema",
        "official-child-schemas",
        "runtime-preflight",
    ];
    require(gates.len() == expected.len(), "readiness gate set differs")?;
    for gate in expected {
        let record = gates.get(gate).ok_or("required readiness gate is absent")?;
        require(
            record["status"] == "passed",
            "required readiness gate is not passed",
        )?;
        let path = field(record, "receipt")?;
        require(
            manifest.files.contains_key(path),
            "gate evidence absent from manifest",
        )?;
        let receipt = json_file(root, path)?;
        bindings(
            &receipt,
            &json!({"source_commit":args.source_commit,"binary_sha256":args.binary_sha256,"status":"passed","umbrella_sha256":bounded_hash(&checked_file(root,"umbrella.json")?)?}),
        )?;
        require(
            record["sha256"] == bounded_hash(&checked_file(root, path)?)?,
            "gate receipt digest differs",
        )?;
        let evidence = receipt["evidence"]
            .as_array()
            .ok_or("gate lacks retained evidence")?;
        require(!evidence.is_empty(), "gate evidence is empty")?;
        for row in evidence {
            let name = field(row, "path")?;
            require(
                manifest.files.contains_key(name)
                    && row["sha256"] == bounded_hash(&checked_file(root, name)?)?,
                "gate evidence is not retained and hash-bound",
            )?;
        }
        if gate == "runtime-preflight" {
            require(
                receipt["observed_commands"].as_u64().is_some_and(|n| n > 0)
                    && receipt["observed_wall_nanoseconds"]
                        .as_str()
                        .and_then(|n| n.parse::<u128>().ok())
                        .is_some_and(|n| n > 0)
                    && receipt["measurement_source"] == "retained_command_receipts",
                "runtime evidence must be measured, not budget-derived",
            )?;
        }
    }
    let cells = json_file(root, "native-cells.json")?
        .as_array()
        .ok_or("missing cell table")?
        .clone();
    check_population(
        &plans,
        &cells,
        &json_file(root, "cohorts.json")?,
        &json_file(root, "planner-coverage.json")?,
    )?;
    Ok(Admission {
        plans,
        cells,
        manifest_sha256,
        inputs: json!({"children":children,"limits":limits,"readiness_sha256":bounded_hash(&checked_file(root,"readiness.json")?)?,"readiness_is_supplied_attestation":true,"official_checker_reexecuted_by_fixture":false}),
    })
}

fn tree_bytes(root: &Path) -> Result<u64> {
    let mut sum = 0u64;
    for path in entries(root)? {
        let meta = fs::symlink_metadata(&path)?;
        require(!meta.file_type().is_symlink(), "symlink in fixture storage")?;
        let bytes = if meta.is_dir() {
            tree_bytes(&path)?
        } else {
            require(meta.is_file(), "special fixture output")?;
            meta.len()
        };
        sum = sum.checked_add(bytes).ok_or("fixture byte overflow")?;
    }
    Ok(sum)
}
fn command_limit(action: &str) -> Result<u64> {
    match action {
        "plan" | "status" => Ok(15),
        "preflight" => Ok(30),
        "generate" => Ok(50),
        "verify" | "export" | "report" => Ok(60),
        _ => Err("command outside frozen local SOFTWARE surface".into()),
    }
}
pub(super) fn before_command(runner: &Runner<'_>, args: &[String]) -> Result<u64> {
    admit_resources(runner.commands.len(), tree_bytes(&runner.root)?)?;
    // Includes source snapshots, readiness and exact protocol hashes on every call.
    admit(runner.args, &runner.root.join("fixture"))?;
    command_limit(args.first().ok_or("missing command")?)
}
fn admit_resources(commands: usize, bytes: u64) -> Result<()> {
    require(commands < 256, "command count exhausted")?;
    require(
        bytes <= MAX_STORAGE,
        "before-next-command storage admission exceeded",
    )
}
fn snapshot(root: &Path) -> Result<Vec<Value>> {
    let mut rows = vec![];
    inventory(root, root, &mut rows)?;
    Ok(rows)
}
fn csv<T: serde::Serialize>(values: &[T]) -> Result<String> {
    Ok(serde_json::to_value(values)?
        .as_array()
        .ok_or("axis array")?
        .iter()
        .map(|x| {
            x.as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| x.to_string())
        })
        .collect::<Vec<_>>()
        .join(","))
}
fn plan_argv(
    plan: &PlanDocument,
    out: &Path,
    protocol_sha: &str,
    policy: Option<&Path>,
) -> Result<Vec<String>> {
    let a = plan.axes();
    let mut args = strings(&[
        "plan",
        "--out",
        &path_text(out)?,
        "--n",
        &csv(&a.field_sizes)?,
        "--m",
        &csv(&a.arities)?,
        "--families",
        &csv(&a.families)?,
        "--solvers",
        &csv(&a.solvers)?,
        "--curve-seeds",
        &a.curve_seeds.to_string(),
        "--generators",
        &a.generators.to_string(),
        "--keys",
        &a.keys.to_string(),
        "--seed",
        &a.seed.to_string(),
        "--factor-base-size",
        &a.factor_base_size.to_string(),
        "--degree-bound",
        &a.degree_bound.to_string(),
        "--work-limit",
        &a.work_limit.to_string(),
        "--protocol-sha256",
        protocol_sha,
    ]);
    if let Some(path) = policy {
        args.extend(strings(&["--factor-base-policy-file", &path_text(path)?]));
    }
    Ok(args)
}
fn generate_argv(plan: &Path, protocol: &Path, out: &Path, row: &Value) -> Result<Vec<String>> {
    require(
        field(row, "plan")? != "planner-coverage"
            && number(row, "n")? == 7
            && number(row, "m")? == 2,
        "planning-only/large native generation forbidden",
    )?;
    Ok(strings(&[
        "generate",
        "--plan",
        &path_text(plan)?,
        "--protocol",
        &path_text(protocol)?,
        "--out",
        &path_text(out)?,
        "--max-cases",
        "1",
        "--shard-index",
        &number(row, "global_index")?.to_string(),
        "--shard-count",
        &number(row, "shard_count")?.to_string(),
        "--seconds-per-case",
        "30",
        "--memory-mib",
        "2048",
        "--trace-mib",
        "64",
    ]))
}
fn check_status(value: &Value, total: u64, replay: bool) -> Result<()> {
    bindings(
        value,
        &json!({"planned_cells":total,"assigned_shard_cells":1,"completed_shard_receipts":1,"artifact_hashes_verified":true,"known_answer_replay_performed":replay,"shard_attempts_complete":true,"campaign_attempts_complete":false,"full_scope_complete":false,"published_cells":0,"discoverable_catalog_cells":0,"incomplete_attempts":[]}),
    )
}
fn artifact_rows(cell: &Path, manifest: &Value) -> Result<Vec<Value>> {
    let rows = manifest["artifacts"]
        .as_array()
        .ok_or("cell artifacts missing")?;
    let mut seen = BTreeSet::new();
    for row in rows {
        let name = field(row, "path")?;
        require(seen.insert(name), "duplicate artifact path")?;
        let path = checked_file(cell, name)?;
        require(
            row["sha256"] == hash(&path)? && row["bytes"] == fs::metadata(path)?.len(),
            "artifact digest/size mismatch",
        )?;
        require(
            !field(row, "role")?.is_empty() && !field(row, "encoding")?.is_empty(),
            "artifact classification missing",
        )?;
    }
    Ok(rows.clone())
}
fn no_positive(value: &Value) -> Result<()> {
    for field in [
        "solver_completed",
        "pdp_witness_verified",
        "scalar_recovered_without_witness",
        "scalar_replay_verified",
        "ecdlp_pipeline_completed",
        "exhaustive_no_decomposition",
    ] {
        require(
            value[field] != true,
            "capped/unsupported outcome carries positive claim",
        )?;
    }
    Ok(())
}
fn inspect_cell(args: &Args, root: &Path, plan: &PlanDocument, row: &Value) -> Result<Value> {
    let index = number(row, "global_index")?;
    let cell = root.join(format!("cells/{index:012}"));
    let expected = plan.cell(index)?;
    let config = json_file(&cell, "config.json")?;
    require(
        config == serde_json::to_value(&expected.config)?,
        "retained config differs from selected occurrence",
    )?;
    let run = json_file(&cell, "run.json")?;
    bindings(
        &run,
        &json!({"source_commit":args.source_commit,"binary_sha256":args.binary_sha256,"plan_sha256":plan.binding_sha256()?,"shard_index":index,"shard_count":plan.cell_count()?,"seconds_per_case":30,"memory_mib":2048,"trace_mib":64}),
    )?;
    let manifest = json_file(&cell, "manifest.json")?;
    require(
        manifest["cell_index"] == index
            && manifest["config"] == config
            && manifest["plan_sha256"] == plan.binding_sha256()?,
        "cell manifest identity mismatch",
    )?;
    let artifacts = artifact_rows(&cell, &manifest)?;
    let outcome = &manifest["outcome"];
    require(
        !matches!(
            outcome["status"].as_str(),
            Some("error" | "worker_failed" | "timeout" | "trace_budget_exhausted")
        ),
        "unexpected failed native worker",
    )?;
    let base = json_file(&cell, "factor-base.json")?;
    let signed = matches!(
        expected.policy,
        FactorBasePolicy::SignedFrobeniusUnionV1 { .. }
    );
    if let PlanDocument::V2(_) = plan {
        let occurrence = json_file(&cell, "case-policy.json")?;
        bindings(
            &occurrence,
            &json!({"cell_index":index,"policy_index":expected.policy_index,"axes_cell_index":expected.axes_cell_index,"plan_sha256":plan.binding_sha256()?}),
        )?;
        let attempt = json_file(&cell, "attempt.json")?;
        require(
            attempt["plan_file_sha256"] == hash(&checked_file(&cell, "plan.json")?)?
                && attempt["case_policy_file_sha256"]
                    == hash(&checked_file(&cell, "case-policy.json")?)?,
            "attempt does not bind exact policy/plan bytes",
        )?;
        for name in [
            "capture-binding.json",
            "capture-limits.json",
            "execution-context.json",
        ] {
            require(
                artifacts.iter().any(|a| a["path"] == name),
                "missing mandatory V2 custody artifact",
            )?;
        }
        require(
            fs::read(checked_file(root, "plan.json")?)?
                == fs::read(checked_file(&cell, "plan.json")?)?,
            "root/cell V2 plan bytes differ",
        )?;
    }
    if signed {
        require(
            base["schema"] == "ml-ecdlp-factor-base/v2",
            "signed source reinterpreted as legacy",
        )?;
        let domain = json_file(&cell, "solver-domain.json")?;
        require(
            domain["points"] == base["points"]
                && domain["source_domain_digest"] == base["domain_digest"],
            "solver source domain grew/reordered",
        )?;
        let witness = json_file(&cell, "witness.json")?;
        require(
            outcome["factor_base_construction_status"]
                == witness["factor_base_policy_construction"]["terminal_status"],
            "constructor terminal mismatch",
        )?;
    } else {
        require(
            base["schema"] == "ml-ecdlp-factor-base/v1",
            "legacy source schema changed",
        )?;
    }
    let id = field(row, "plan")?;
    if id.contains("empty") {
        require(
            base["points"].as_array().is_some_and(Vec::is_empty) && base["domain_digest"].is_null(),
            "empty construction fabricated a population/domain",
        )?;
    }
    if expected.config.solver == "bsgs" {
        for field in [
            "solver_completed",
            "scalar_recovered_without_witness",
            "scalar_replay_verified",
        ] {
            require(
                outcome[field] == true,
                "required BSGS scalar recovery absent",
            )?;
        }
        let labels: Vec<_> = artifacts
            .iter()
            .filter(|a| a["role"] == "restricted-independent-scalar-label")
            .collect();
        require(
            labels.len() == 1 && labels[0]["encoding"] == "json",
            "ambiguous or absent checked scalar artifact",
        )?;
        let certificate = json_file(&cell, field(labels[0], "path")?)?;
        require(
            certificate["method"] == "native-bsgs" && certificate["read_private_witness"] == false,
            "wrong scalar certificate provenance",
        )?;
        // The binary-bound CLI verify already checked its equation/range. These
        // descriptive provenance flags alone are never an independence proof.
        if signed {
            require(
                outcome["solver_domain_scope"] == "cyclic_subgroup_discrete_log"
                    && outcome["factor_base_required_by_solver"] == false,
                "BSGS incorrectly requires PDP source",
            )?;
        }
    } else if id == "v2-empty-pdp" {
        require(
            outcome["status"] == "censored" && outcome["native_solver_invoked"] == false,
            "empty PDP entered native search",
        )?;
        no_positive(outcome)?;
    } else if id == "v2-pkm-cap" {
        require(
            outcome["native_alternative_domain_admission"] == "preflight_budget_exhausted"
                && outcome["status"] == "budget_exhausted"
                && !artifacts
                    .iter()
                    .any(|a| a["path"] == "pkm-factor-base.json"),
            "PKM low-work domain was fabricated",
        )?;
        no_positive(outcome)?;
    } else if id == "v2-lp-smallwork" {
        match outcome["native_alternative_domain_admission"].as_str() {
            Some("preflight_budget_exhausted") => {
                require(
                    outcome["status"] == "budget_exhausted"
                        && !artifacts
                            .iter()
                            .any(|a| a["role"] == "exact-large-prime-discovery-domains"),
                    "capped residual domain fabricated",
                )?;
                no_positive(outcome)?;
            }
            Some("eligible") => require(
                artifacts
                    .iter()
                    .filter(|a| a["role"] == "exact-large-prime-discovery-domains")
                    .count()
                    == 1,
                "eligible generic domain absent",
            )?,
            _ => return Err("unexpected large-prime preflight state".into()),
        }
    }
    Ok(
        json!({"ordinal":row["ordinal"],"plan":id,"cell_index":index,"manifest_sha256":hash(&checked_file(&cell,"manifest.json")?)?,"outcome":outcome,"native_cli_replay_required":true,"included_in_research_corpus":false}),
    )
}
fn frozen_plan_inputs(root: &Path, id: &str) -> Result<(PathBuf, String, Vec<u8>)> {
    require(IDS.contains(&id), "unknown frozen plan id")?;
    let fixture = root.join("fixture");
    let protocol = checked_file(&fixture, &format!("protocols/{id}.json"))?;
    let protocol_sha = bounded_hash(&protocol)?;
    let plan = bounded(&checked_file(&fixture, &format!("plans/{id}.json"))?)?;
    Ok((protocol, protocol_sha, plan))
}

// Controller operation intervals include nested command time where applicable;
// they are not additive to command timings and exclude the phase receipt writes.
fn recorded_step<T>(
    root: &Path,
    ordinal: u64,
    index: &mut usize,
    name: &str,
    kind: &str,
    operation: impl FnOnce() -> Result<T>,
) -> Result<T> {
    let dir = root.join("steps").join(format!("cell-{ordinal:02}"));
    fs::create_dir_all(&dir)?;
    let stem = format!("{:03}", *index);
    *index += 1;
    let start = json!({"ordinal":ordinal,"phase":name,"attempt_kind":kind,
        "started_unix_seconds":now()?,"intent_is_not_process_spawn":true});
    write_new(&dir.join(format!("{stem}.started.json")), &start)?;
    let began = Instant::now();
    let result = operation();
    let elapsed = began.elapsed().as_nanos().to_string();
    write_new(
        &dir.join(format!("{stem}.receipt.json")),
        &json!({"ordinal":ordinal,"phase":name,
        "attempt_kind":kind,"completed":result.is_ok(),"error":result.as_ref().err().map(|e|e.to_string()),
        "observed_controller_operation_nanoseconds":elapsed,"timing_scope":"operation_only_excludes_phase_receipt_writes",
        "nested_command_timings_are_not_additive":true,"clock_valid_for_comparison":false}),
    )?;
    result
}
fn latest_step(root: &Path, ordinal: u64) -> Result<Value> {
    let dir = root.join("steps").join(format!("cell-{ordinal:02}"));
    if !dir.try_exists()? {
        return Ok(Value::Null);
    }
    let mut starts: Vec<_> = entries(&dir)?
        .into_iter()
        .filter(|p| {
            p.file_name()
                .and_then(|x| x.to_str())
                .is_some_and(|s| s.ends_with(".started.json"))
        })
        .collect();
    starts.sort();
    let Some(start) = starts.last() else {
        return Ok(Value::Null);
    };
    let intent = unique(&bounded(start)?, 0)?;
    let stem = start
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("phase start filename")?
        .strip_suffix(".started.json")
        .ok_or("phase start suffix")?;
    let finish = dir.join(format!("{stem}.receipt.json"));
    let receipt = if finish.try_exists()? {
        unique(&bounded(&finish)?, 0)?
    } else {
        Value::Null
    };
    Ok(
        json!({"phase":intent["phase"],"attempt_kind":intent["attempt_kind"],
        "started_evidence":start.strip_prefix(root)?,"started_sha256":bounded_hash(start)?,
        "receipt":receipt,"receipt_sha256":if finish.try_exists()? {Some(bounded_hash(&finish)?)}else{None},
        "completion_known":!receipt.is_null()}),
    )
}
fn plan_inputs(runner: &mut Runner<'_>, admission: &Admission) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for (i, plan) in admission.plans.iter().enumerate() {
        let id = IDS[i];
        let dir = runner.root.join("plans").join(id);
        fs::create_dir_all(&dir)?;
        let (protocol, protocol_sha, expected_plan) = frozen_plan_inputs(&runner.root, id)?;
        let path = dir.join("plan.json");
        let policy = if let PlanDocument::V2(v2) = plan {
            let p = dir.join("policy.json");
            write_new(&p, &serde_json::to_value(&v2.factor_base_policies)?)?;
            Some(p)
        } else {
            None
        };
        let planned = runner.json_command(
            &format!("plan-{i}"),
            plan_argv(plan, &path, &protocol_sha, policy.as_deref())?,
        )?;
        bindings(
            &planned,
            &json!({"planned_cells":plan.cell_count()?,"generated_cells":0,"plan_sha256":plan.binding_sha256()?,"sha256":hash(&path)?}),
        )?;
        require(
            fs::read(&path)? == expected_plan,
            "CLI plan bytes differ from frozen expected plan",
        )?;
        let receipt = runner.json_command(
            &format!("preflight-{i}"),
            strings(&[
                "preflight",
                "--plan",
                &path_text(&path)?,
                "--protocol",
                &path_text(&protocol)?,
            ]),
        )?;
        bindings(
            &receipt,
            &json!({"source_commit":runner.args.source_commit,"source_dirty":false,"binary_sha256":runner.args.binary_sha256,"plan_sha256":plan.binding_sha256()?,"plan_file_sha256":hash(&path)?,"protocol_file_sha256":protocol_sha,"planned_cells":plan.cell_count()?,"generated_cells":0,"cloud_permissions_checked":false,"lab_receipt_uri":null}),
        )?;
        if matches!(plan, PlanDocument::V2(_)) {
            require(
                receipt["policy_plan"]["execution_enabled"] == true,
                "candidate V2 gates remain disabled",
            )?;
        }
        paths.push(path);
    }
    Ok(paths)
}
fn native_stage(runner: &mut Runner<'_>, admission: &Admission) -> Result<()> {
    let plans = plan_inputs(runner, admission)?;
    let root = runner.root.clone();
    let mut reported = BTreeSet::new();
    let mut legacy = BTreeMap::<(String, bool), (Vec<u8>, Vec<u8>)>::new();
    for row in &admission.cells {
        let ordinal = number(row, "ordinal")?;
        let mut step = 0;
        let (id, i, out, protocol) = recorded_step(
            &root,
            ordinal,
            &mut step,
            "prepare-cell",
            "verification_attempt",
            || {
                let id = field(row, "plan")?.to_owned();
                let i = IDS.iter().position(|x| *x == id).ok_or("missing plan")?;
                require(i < 6, "broad planning fixture must not generate")?;
                let out = root.join(format!("cell-{ordinal:02}"));
                let protocol = root.join("fixture/protocols").join(format!("{id}.json"));
                Ok((id, i, out, protocol))
            },
        )?;
        recorded_step(
            &root,
            ordinal,
            &mut step,
            "generate",
            "execution_attempt",
            || {
                runner.invoke(
                    &format!("cell-{ordinal:02}-generate"),
                    generate_argv(&plans[i], &protocol, &out, row)?,
                    true,
                )
            },
        )?;
        let before = recorded_step(
            &root,
            ordinal,
            &mut step,
            "snapshot-source-before-checks",
            "verification_attempt",
            || snapshot(&out),
        )?;
        for (action, replay) in [("verify", true), ("status", false)] {
            recorded_step(
                &root,
                ordinal,
                &mut step,
                action,
                "verification_attempt",
                || {
                    let result = runner.json_command(
                        &format!("cell-{ordinal:02}-{action}"),
                        strings(&[action, "--out", &path_text(&out)?]),
                    )?;
                    check_status(&result, admission.plans[i].cell_count()?, replay)
                },
            )?;
        }
        let checked = recorded_step(
            &root,
            ordinal,
            &mut step,
            "inspect-cell",
            "verification_attempt",
            || inspect_cell(runner.args, &out, &admission.plans[i], row),
        )?;
        recorded_step(
            &root,
            ordinal,
            &mut step,
            "legacy-byte-compatibility",
            "verification_attempt",
            || {
                let cell = out.join(format!("cells/{:012}", number(row, "global_index")?));
                let key = (
                    field(row, "family")?.to_owned(),
                    row["twist"].as_bool().ok_or("twist")?,
                );
                if id == "v1-bsgs" {
                    legacy.insert(
                        key,
                        (
                            fs::read(cell.join("public.json"))?,
                            fs::read(cell.join("factor-base.json"))?,
                        ),
                    );
                } else if id == "v2-bsgs" && number(row, "policy_index")? == 0 {
                    let prior = legacy
                        .get(&key)
                        .ok_or("legacy compatibility source unavailable")?;
                    require(
                        prior.0 == fs::read(cell.join("public.json"))?
                            && prior.1 == fs::read(cell.join("factor-base.json"))?,
                        "V1/explicit legacy deterministic bytes differ",
                    )?;
                }
                Ok(())
            },
        )?;
        for enabled in [false, true] {
            let destination = root.join(format!(
                "{}-{ordinal:02}",
                if enabled { "ecdlp" } else { "pdp" }
            ));
            let mut argv = strings(&[
                "export",
                "--out",
                &path_text(&out)?,
                "--destination",
                &path_text(&destination)?,
            ]);
            if enabled {
                argv.push("--cyclic-subgroup-ecdlp".into());
            }
            let receipt = recorded_step(
                &root,
                ordinal,
                &mut step,
                &format!("export-{enabled}"),
                "verification_attempt",
                || {
                    let receipt = runner.json_command(
                        &format!("cell-{ordinal:02}-export-{enabled}"),
                        argv.clone(),
                    )?;
                    require(receipt.is_object(), "export receipt missing")?;
                    Ok(receipt)
                },
            )?;
            if enabled {
                let saved = recorded_step(
                    &root,
                    ordinal,
                    &mut step,
                    "snapshot-export-before-retry",
                    "verification_attempt",
                    || snapshot(&destination),
                )?;
                let retry = recorded_step(
                    &root,
                    ordinal,
                    &mut step,
                    "export-retry",
                    "verification_attempt",
                    || runner.json_command(&format!("cell-{ordinal:02}-export-retry"), argv),
                )?;
                recorded_step(
                    &root,
                    ordinal,
                    &mut step,
                    "inspect-export-retry-and-labels",
                    "verification_attempt",
                    || {
                        require(
                            receipt == retry && saved == snapshot(&destination)?,
                            "immutable export retry changed bytes/receipt",
                        )?;
                        let rows = fs::read_to_string(checked_file(
                            &destination,
                            "restricted/ecdlp-records.jsonl",
                        )?)?;
                        let rows: Vec<_> = rows
                            .lines()
                            .map(|line| unique(line.as_bytes(), 0))
                            .collect::<Result<_>>()?;
                        require(rows.len() == 1, "subgroup occurrence count differs")?;
                        if field(row, "solver")? == "bsgs" {
                            require(
                                !rows[0]["recovered_scalar"].is_null()
                                    && rows[0]["scalar_label_verification"]
                                        == "exact-equation-range-and-custody",
                                "subgroup checked scalar join missing",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            } else {
                recorded_step(
                    &root,
                    ordinal,
                    &mut step,
                    "inspect-default-export",
                    "verification_attempt",
                    || {
                        require(
                            !destination.join("ecdlp-features.jsonl").exists(),
                            "default export gained opt-in files",
                        )
                    },
                )?;
            }
            recorded_step(
                &root,
                ordinal,
                &mut step,
                &format!("inspect-empty-domain-{enabled}"),
                "verification_attempt",
                || {
                    if id.contains("empty") {
                        let features =
                            fs::read_to_string(checked_file(&destination, "features.jsonl")?)?;
                        require(
                            features.trim().is_empty(),
                            "empty PDP domain became a feature",
                        )?;
                    }
                    Ok(())
                },
            )?;
        }
        if reported.insert(id.to_owned()) {
            let destination = root.join(format!("report-{i}"));
            recorded_step(
                &root,
                ordinal,
                &mut step,
                "report",
                "verification_attempt",
                || {
                    runner.json_command(
                        &format!("report-{i}"),
                        strings(&[
                            "report",
                            "--out",
                            &path_text(&out)?,
                            "--destination",
                            &path_text(&destination)?,
                        ]),
                    )
                },
            )?;
        }
        recorded_step(
            &root,
            ordinal,
            &mut step,
            "inspect-source-immutability",
            "verification_attempt",
            || {
                require(
                    before == snapshot(&out)?,
                    "read/replay/export/report changed source files",
                )
            },
        )?;
        runner.cells.push(checked);
    }
    Ok(())
}

// The start record is durable intent, not proof of spawn. Absence means no generate
// invocation crossed the command-start boundary; a successful command is still
// distinct from later custody, replay and export completion.
fn progress_one(root: &Path, row: &Value, checked: &[Value]) -> Result<Value> {
    let ordinal = number(row, "ordinal")?;
    let name = format!("commands/cell-{ordinal:02}-generate");
    let start = root.join(format!("{name}.started.json"));
    let receipt = root.join(format!("{name}.receipt.json"));
    let attempted = start.try_exists()?;
    let finished = receipt.try_exists()?;
    require(
        !finished || attempted,
        "command receipt without start evidence",
    )?;
    let generated = if finished {
        let result = unique(&bounded(&receipt)?, 0)?;
        result["exit_status"] == 0
            && result["timed_out"] == false
            && result["cancelled"] == false
            && result.get("spawn_or_wait_error").is_none()
    } else {
        false
    };
    let complete = checked.iter().any(|done| done["ordinal"] == row["ordinal"]);
    let last_step = latest_step(root, ordinal)?;
    let failed_step =
        if !complete && !last_step.is_null() && last_step["receipt"]["completed"] != true {
            last_step["phase"].clone()
        } else {
            Value::Null
        };
    require(
        !complete || generated,
        "checked cell without successful generation receipt",
    )?;
    let state = if complete {
        "checked_complete"
    } else if generated {
        "generated_check_incomplete"
    } else if attempted {
        "attempted_generation_incomplete"
    } else {
        "never_started"
    };
    Ok(
        json!({"cell":row,"state":state,"generation_attempted":attempted,"generation_command_succeeded":generated,
            "failed_step":failed_step,"last_step":last_step,
            "checked_complete":complete,"start_evidence":if attempted { Some(format!("{name}.started.json")) } else {None},
            "start_evidence_sha256":if attempted {Some(bounded_hash(&start)?)} else {None},
            "generation_receipt":if finished {Some(format!("{name}.receipt.json"))}else{None},
            "generation_receipt_sha256":if finished {Some(bounded_hash(&receipt)?)}else{None}}),
    )
}
// Terminal reporting must tolerate damaged partial evidence. Preserve each row
// and every readable row, but never turn an unreadable attempt into "unrun".
fn progress(root: &Path, rows: &[Value], checked: &[Value]) -> Vec<Value> {
    rows.iter()
        .map(|row| match progress_one(root, row, checked) {
            Ok(value) => value,
            Err(error) => json!({"cell":row,"state":"unknown_due_to_evidence_error",
            "progress_error":error.to_string(),"generation_attempted":null,
            "generation_command_succeeded":null,"checked_complete":null,
            "failed_step":null,"last_step":null}),
        })
        .collect()
}
fn progress_errors(rows: &[Value]) -> Vec<Value> {
    rows.iter().filter(|row| !row["progress_error"].is_null()).map(|row|
        json!({"ordinal":row["cell"]["ordinal"],"scope":"cell_progress_readback","error":row["progress_error"]})).collect()
}

pub(super) fn run(mut args: Args) -> Result<bool> {
    use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
    let started = Instant::now();
    for (value, width) in [
        (&args.binary_sha256, 64),
        (&args.source_commit, 40),
        (&args.fixture_commit, 40),
    ] {
        require(
            value.len() == width
                && value
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
            "exact lowercase hexadecimal identities required",
        )?;
    }
    args.binary = args.binary.canonicalize()?;
    args.fixture = args.fixture.canonicalize()?;
    require(
        hash(&args.binary)? == args.binary_sha256,
        "binary differs from explicit digest",
    )?;
    let original = admit(&args, &args.fixture)?;
    let parent = args
        .scratch
        .parent()
        .ok_or("scratch parent required")?
        .canonicalize()?;
    let root = parent.join(args.scratch.file_name().ok_or("scratch name required")?);
    fs::DirBuilder::new().mode(0o700).create(&root)?;
    let fixture = root.join("fixture");
    fs::create_dir(&fixture)?;
    let source_manifest: Manifest =
        serde_json::from_value(json_file(&args.fixture, "acceptance-manifest.json")?)?;
    let mut names: Vec<_> = source_manifest.files.keys().cloned().collect();
    names.push("acceptance-manifest.json".into());
    for name in names {
        let source = checked_file(&args.fixture, &name)?;
        let destination = fixture.join(&name);
        fs::create_dir_all(destination.parent().ok_or("destination parent")?)?;
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&destination)?;
        let bytes = bounded(&source)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        fs::set_permissions(destination, fs::Permissions::from_mode(0o444))?;
    }
    let admission = admit(&args, &fixture)?;
    require(
        original.manifest_sha256 == admission.manifest_sha256,
        "fixture changed while copying",
    )?;
    fs::create_dir(root.join("bin"))?;
    let binary = root.join("bin/crypto");
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&binary)?;
    std::io::copy(&mut File::open(&args.binary)?, &mut file)?;
    file.sync_all()?;
    fs::set_permissions(&binary, fs::Permissions::from_mode(0o555))?;
    require(
        hash(&binary)? == args.binary_sha256,
        "binary changed while copying",
    )?;
    args.binary = binary;
    for name in ["commands", "tmp", "plans"] {
        fs::create_dir(root.join(name))?;
    }
    let inputs = json!({"schema":"ml-dataset.native-policy-stage-inputs/v1","classification":"SOFTWARE","source_commit":args.source_commit,"binary_sha256":args.binary_sha256,"fixture_commit":args.fixture_commit,"fixture_commit_is_supplied_not_git_verified":true,"fixture_manifest_sha256":admission.manifest_sha256,"admission":admission.inputs,"started_unix_seconds":now()?,"native_worker_ceiling":26,"large_plan_generate_forbidden":true,"scientific_samples_added":0,"verified_cloud_dataset_cells":0});
    write_new(&root.join("execution-inputs.json"), &inputs)?;
    unsafe {
        libc::signal(libc::SIGINT, cancelled as *const () as libc::sighandler_t);
        libc::signal(libc::SIGTERM, cancelled as *const () as libc::sighandler_t);
    }
    let mut runner = Runner {
        args: &args,
        root: root.clone(),
        deadline: started + Duration::from_secs(3600),
        commands: vec![],
        cells: vec![],
        tamper: Value::Null,
    };
    let native_intent_error = (|| -> Result<()> {
        write_new(
            &root.join("native-workflow.started.json"),
            &json!({"workflow":"native-cells","started_unix_seconds":now()?,"first_command":0}),
        )
    })()
    .err()
    .map(|e| e.to_string());
    let execution_failure = if let Some(error) = &native_intent_error {
        Some(format!("native workflow intent: {error}"))
    } else {
        native_stage(&mut runner, &admission)
            .err()
            .map(|e| e.to_string())
    };
    let native_end_command = runner.commands.len();
    let native_terminal_error = (|| -> Result<()> {
        write_new(&root.join("native-workflow.receipt.json"), &json!({"workflow":"native-cells","finished_unix_seconds":now()?,"first_command":0,"end_command":native_end_command,"failure":execution_failure,"intent_error":native_intent_error,"checked_cells":runner.cells.len()}))
    })().err().map(|e|e.to_string());
    let native_stage_complete =
        execution_failure.is_none() && native_terminal_error.is_none() && runner.cells.len() == 26;
    let controls_result = controls::run(&mut runner, &admission, native_stage_complete);
    let controls_error = controls_result
        .as_ref()
        .err()
        .map(ToString::to_string)
        .or_else(|| {
            controls_result
                .as_ref()
                .ok()
                .and_then(|v| v["failure"].as_str())
                .map(str::to_owned)
        });
    let mut failure = execution_failure
        .clone()
        .or_else(|| native_terminal_error.clone());
    if failure.is_none() {
        failure = controls_error.clone();
    }
    let fixture_readback_error = admit(&args, &fixture).err().map(|e| e.to_string());
    if failure.is_none() {
        failure = fixture_readback_error
            .as_ref()
            .map(|error| format!("final fixture readback: {error}"));
    }
    let final_binary = hash(&args.binary);
    let binary_readback_error = match &final_binary {
        Ok(observed) if observed == &args.binary_sha256 => None,
        Ok(_) => Some("binary failed final digest readback".to_owned()),
        Err(error) => Some(format!("binary final readback: {error}")),
    };
    if failure.is_none() {
        failure = binary_readback_error.clone();
    }
    let mut retained = Vec::new();
    let inventory_error = inventory(&root, &root, &mut retained)
        .err()
        .map(|e| e.to_string());
    if failure.is_none() {
        failure = inventory_error.clone();
    }
    let cell_progress = progress(&runner.root, &admission.cells, &runner.cells);
    let mut retention_errors = progress_errors(&cell_progress);
    for (scope, error) in [
        ("native_workflow_terminal", &native_terminal_error),
        ("copied_controls", &controls_error),
        ("fixture_readback", &fixture_readback_error),
        ("binary_readback", &binary_readback_error),
        ("inventory_readback", &inventory_error),
    ] {
        if let Some(error) = error {
            retention_errors.push(json!({"scope":scope,"error":error}));
        }
    }
    if failure.is_none() && !retention_errors.is_empty() {
        failure = Some(
            "one or more cell progress records could not be read; retained unknown rows".into(),
        );
    }
    let unrun: Vec<_> = cell_progress
        .iter()
        .filter(|row| row["state"] == "never_started")
        .cloned()
        .collect();
    let native_stage_passed = native_stage_complete
        && fixture_readback_error.is_none()
        && binary_readback_error.is_none()
        && inventory_error.is_none()
        && progress_errors(&cell_progress).is_empty();
    let workflow_stages_passed = failure.is_none() && native_stage_passed;
    let receipt = json!({"schema":"ml-dataset.native-policy-stage-receipt/v1","classification":"SOFTWARE","inputs":inputs,"finished_unix_seconds":now()?,"native_stage_passed":native_stage_passed,"full_acceptance":false,"pending_gates":["V2-startup-budget-restore-API-fixtures","official-two-workflow-export-and-final-checker"],"failure":failure,"execution_failure":execution_failure,"controls_error":controls_error,"controls_workflow":controls_result.ok(),"native_end_command":native_end_command,"retention_errors":retention_errors,"commands":runner.commands,"checked_local_cells":runner.cells,"cell_progress":cell_progress,"unrun_cells":unrun,"binary_sha256_after":final_binary.ok(),"inventory":retained,"inventory_error":inventory_error,"retired_hardware_instructions":null,"whole_worker_instruction_scope_complete":false,"clock_valid_for_comparison":false,"descendant_cleanup":"attempted_not_verified; inherited supervisor reaps direct child","storage_limit_scope":"before-next-command admission; command/poll overshoot retained","scientific_samples_added":0,"verified_cloud_dataset_cells":0,"cloud_permissions_checked":false});
    let mut receipt = receipt;
    receipt["workflow_stages_passed"] = json!(workflow_stages_passed);
    write_new(&root.join("receipt.json"), &receipt)?;
    // Receipt inventory is sealed before the separate pointer, avoiding cycles.
    write_new(
        &root.join("result.json"),
        &json!({"schema":"ml-dataset.native-policy-stage-result/v1","receipt_sha256":hash(&root.join("receipt.json"))?,"native_stage_passed":native_stage_passed,"workflow_stages_passed":workflow_stages_passed,"full_acceptance":false,"retention_verified":false}),
    )?;
    println!(
        "{}",
        json!({"receipt":root.join("receipt.json"),"native_stage_passed":native_stage_passed,"workflow_stages_passed":workflow_stages_passed,"full_acceptance":false})
    );
    Ok(workflow_stages_passed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::ml_dataset::{types::plan_v2::DatasetPlanV2, DatasetPlan};
    use std::sync::atomic::AtomicU64;
    static NEXT: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn failed_native_workflow_retains_all_control_slots_without_commands() {
        let s = Scratch::new();
        let args = admitted_fixture(&s.0);
        let admission = admit(&args, &s.0).unwrap();
        let root = s.0.join("no-native-source");
        fs::create_dir(&root).unwrap();
        let mut runner = Runner {
            args: &args,
            root,
            deadline: Instant::now() + Duration::from_secs(30),
            commands: vec![],
            cells: vec![],
            tamper: Value::Null,
        };
        let result = controls::run(&mut runner, &admission, false).unwrap();
        let rows = result["controls"].as_array().unwrap();
        assert_eq!(rows.len(), 16);
        for (index, row) in rows.iter().enumerate() {
            assert_eq!(row["slot"], index + 1);
            assert_eq!(row["status"], "stopped_before_attempt");
            assert_eq!(row["command_invocation_attempted"], false);
            assert_eq!(row["command_receipt_present"], false);
            assert_eq!(row["semantic_rejection_verified"], false);
            assert!(row["definition"]["expected_stage"].is_string());
        }
        assert!(runner.commands.is_empty());
        assert_eq!(result["first_command"], 0);
        assert_eq!(result["end_command"], 0);
        assert_eq!(result["full_acceptance"], false);
    }

    #[test]
    fn rehashed_control_source_still_must_match_the_compiled_module() {
        let s = Scratch::new();
        let mut args = admitted_fixture(&s.0);
        admit(&args, &s.0).unwrap();
        let bytes = b"// changed control implementation\n";
        fs::write(s.0.join("controls.rs"), bytes).unwrap();
        let mut manifest = json_file(&s.0, "acceptance-manifest.json").unwrap();
        manifest["files"]["controls.rs"]["sha256"] = json!(digest(bytes));
        manifest["files"]["controls.rs"]["bytes"] = json!(bytes.len());
        fs::write(
            s.0.join("acceptance-manifest.json"),
            serde_json::to_vec(&manifest).unwrap(),
        )
        .unwrap();
        args.acceptance_manifest_sha256 =
            Some(hash(&s.0.join("acceptance-manifest.json")).unwrap());
        let error = admit(&args, &s.0)
            .err()
            .expect("rehashed module was accepted");
        assert!(error
            .to_string()
            .contains("compiled runner/module differs from manifest"));
    }
    struct Scratch(PathBuf);
    impl Scratch {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "native-policy-acceptance-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&p).unwrap();
            Self(p)
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn tables() -> (Value, Value, Vec<Value>) {
        (
            serde_json::from_str(include_str!("cohorts.json")).unwrap(),
            serde_json::from_str(include_str!("planner-coverage.json")).unwrap(),
            serde_json::from_str(include_str!("native-cells.json")).unwrap(),
        )
    }
    fn plans(cohorts: &Value, broad: &Value) -> Vec<PlanDocument> {
        (0..7)
            .map(|i| {
                let v = if i < 6 { &cohorts[i] } else { broad };
                let axes = DatasetPlan {
                    schema: "crypto.ml-dataset.plan/v1".into(),
                    field_sizes: serde_json::from_value(v["n"].clone()).unwrap(),
                    arities: serde_json::from_value(v["m"].clone()).unwrap(),
                    families: serde_json::from_value(v["families"].clone()).unwrap(),
                    solvers: serde_json::from_value(v["solvers"].clone()).unwrap(),
                    curve_seeds: v["curve_seeds"].as_u64().unwrap(),
                    generators: v["generators"].as_u64().unwrap(),
                    keys: v["keys"].as_u64().unwrap(),
                    seed: 42,
                    factor_base_size: v["factor_base_size"].as_u64().unwrap_or(2) as usize,
                    degree_bound: 4,
                    work_limit: v["work_limit"].as_u64().unwrap_or(1),
                    protocol_sha256: None,
                    twists: true,
                    split_rule: "curve-twist-component-sha256-v1".into(),
                };
                if i == 0 {
                    PlanDocument::V1(axes)
                } else {
                    PlanDocument::V2(
                        DatasetPlanV2::new(
                            axes,
                            serde_json::from_value(v["policies"].clone()).unwrap(),
                        )
                        .unwrap(),
                    )
                }
            })
            .collect()
    }
    #[test]
    fn frozen_population_matches_native_global_indices_and_rejects_policy_substitution() {
        let (cohorts, broad, cells) = tables();
        let mut plans = plans(&cohorts, &broad);
        check_population(&plans, &cells, &cohorts, &broad).unwrap();
        assert_eq!(plans[6].cell_count().unwrap(), 580608);
        let mut changed = cells.clone();
        changed[6]["policy_index"] = json!(1);
        assert!(check_population(&plans, &changed, &cohorts, &broad).is_err());
        if let PlanDocument::V2(v) = &mut plans[1] {
            v.factor_base_policies.reverse();
        }
        assert!(check_population(&plans, &cells, &cohorts, &broad).is_err());
    }
    #[test]
    fn nested_duplicate_inputs_and_unfrozen_protocols_fail() {
        assert!(unique(br#"{"gates":{"passed":true,"passed":false}}"#, 0).is_err());
        assert!(
            frozen(&json!({"schema":"experiment.protocol/v1","freeze":{"status":"draft"}}))
                .is_err()
        );
        let mut body = json!({"schema":"experiment.protocol/v1","extensions":{"n":7}});
        let sha = digest(&crypto_lib::ml_dataset::protocol_canonical_bytes(&body).unwrap());
        body["freeze"] =
            json!({"status":"frozen","filed_before_affected_data":true,"protocol_sha256":sha});
        frozen(&body).unwrap();
        body["extensions"]["n"] = json!(61);
        assert!(frozen(&body).is_err());
    }
    #[test]
    fn broad_population_never_becomes_generate_argv_and_network_actions_fail() {
        let (_, _, mut cells) = tables();
        cells[0]["plan"] = json!("planner-coverage");
        assert!(generate_argv(
            Path::new("plan"),
            Path::new("protocol"),
            Path::new("out"),
            &cells[0]
        )
        .is_err());
        assert!(command_limit("publish").is_err());
        assert!(command_limit("start").is_err());
        assert_eq!(command_limit("generate").unwrap(), 50);
    }
    #[test]
    fn artifact_digest_and_duplicate_path_controls_reject_rehashed_metadata_errors() {
        let t = Scratch::new();
        write_new(&t.0.join("public.json"), &json!({"public":true})).unwrap();
        let a = json!({"path":"public.json","role":"public-instance","encoding":"json","bytes":fs::metadata(t.0.join("public.json")).unwrap().len(),"sha256":hash(&t.0.join("public.json")).unwrap()});
        artifact_rows(&t.0, &json!({"artifacts":[a.clone()]})).unwrap();
        assert!(artifact_rows(&t.0, &json!({"artifacts":[a.clone(),a.clone()]})).is_err());
        fs::write(t.0.join("public.json"), b"changed").unwrap();
        assert!(artifact_rows(&t.0, &json!({"artifacts":[a]})).is_err());
    }
    #[test]
    fn copied_output_retry_comparison_detects_changed_bytes_and_caps_never_gain_success() {
        let t = Scratch::new();
        write_new(&t.0.join("receipt.json"), &json!({"ok":true})).unwrap();
        let initial = snapshot(&t.0).unwrap();
        assert_eq!(initial, snapshot(&t.0).unwrap());
        assert!(write_new(&t.0.join("receipt.json"), &json!({"ok":false})).is_err());
        fs::write(t.0.join("receipt.json"), b"changed").unwrap();
        assert_ne!(initial, snapshot(&t.0).unwrap());
        no_positive(&json!({"status":"budget_exhausted","solver_completed":false})).unwrap();
        assert!(
            no_positive(&json!({"status":"budget_exhausted","scalar_replay_verified":true}))
                .is_err()
        );
    }
    #[test]
    fn absent_manifest_digest_refuses_before_any_command() {
        let t = Scratch::new();
        write_new(&t.0.join("acceptance-manifest.json"), &json!({})).unwrap();
        let args = Args {
            policy_acceptance: true,
            acceptance_manifest_sha256: None,
            binary: PathBuf::from("unused"),
            binary_sha256: "a".repeat(64),
            source_commit: "b".repeat(40),
            fixture_commit: "c".repeat(40),
            fixture: t.0.clone(),
            scratch: t.0.join("never-created"),
        };
        assert!(admit(&args, &t.0).is_err());
        assert!(!args.scratch.exists());
    }
    #[test]
    fn exact_resource_caps_reject_the_next_command_without_truncating_evidence() {
        admit_resources(255, MAX_STORAGE).unwrap();
        assert!(admit_resources(256, 0).is_err());
        assert!(admit_resources(0, MAX_STORAGE + 1).is_err());
    }
    #[test]
    fn child_protocol_scope_rejects_a_valid_but_different_policy_population() {
        let (cohorts, broad, _) = tables();
        let p = plans(&cohorts, &broad);
        let PlanDocument::V2(v2) = &p[1] else {
            panic!("V2 fixture")
        };
        let mut scope = v2.clone();
        scope.axes.protocol_sha256 = None;
        let mut bytes = b"crypto.ml-dataset.protocol-plan-scope/v1\0".to_vec();
        bytes.extend(serde_json::to_vec(&scope).unwrap());
        let protocol = json!({"extensions":{"ml_dataset_policy_plan":{"schema":"crypto.ml-dataset.protocol-plan-scope/v1","scope_sha256":digest(&bytes),"planned_cells":p[1].cell_count().unwrap()}}});
        plan_scope(&p[1], &protocol).unwrap();
        scope.factor_base_policies.reverse();
        assert!(plan_scope(&PlanDocument::V2(scope), &protocol).is_err());
    }
    fn save(root: &Path, name: &str, value: &Value) {
        let path = root.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
    }
    fn freeze_test(mut value: Value) -> Value {
        value.as_object_mut().unwrap().remove("freeze");
        let sha = digest(&crypto_lib::ml_dataset::protocol_canonical_bytes(&value).unwrap());
        value["freeze"] =
            json!({"status":"frozen","filed_before_affected_data":true,"protocol_sha256":sha});
        value
    }
    fn refresh_manifest(args: &mut Args) {
        fn visit(root: &Path, here: &Path, rows: &mut serde_json::Map<String, Value>) {
            for path in entries(here).unwrap() {
                if path.is_dir() {
                    visit(root, &path, rows);
                } else {
                    let name = path.strip_prefix(root).unwrap().to_str().unwrap();
                    if name != "acceptance-manifest.json" {
                        let bytes = bounded(&path).unwrap();
                        rows.insert(
                            name.into(),
                            json!({"bytes":bytes.len(),"sha256":digest(&bytes)}),
                        );
                    }
                }
            }
        }
        let mut files = serde_json::Map::new();
        visit(&args.fixture, &args.fixture, &mut files);
        save(
            &args.fixture,
            "acceptance-manifest.json",
            &json!({"schema":"ml-dataset.native-policy-acceptance-inputs/v1","source_commit":args.source_commit,"binary_sha256":args.binary_sha256,"files":files}),
        );
        args.acceptance_manifest_sha256 =
            Some(bounded_hash(&args.fixture.join("acceptance-manifest.json")).unwrap());
    }
    fn refresh_receipt(args: &mut Args, gate: &str, receipt: &Value) {
        let name = format!("receipts/{gate}.json");
        save(&args.fixture, &name, receipt);
        let mut ready = json_file(&args.fixture, "readiness.json").unwrap();
        ready["gates"][gate]["sha256"] = json!(bounded_hash(&args.fixture.join(&name)).unwrap());
        save(&args.fixture, "readiness.json", &ready);
        refresh_manifest(args);
    }
    // Synthetic admission-only attestations, never official checker approval or
    // runtime evidence. Tests exercise custody predicates without spawning code.
    fn admitted_fixture(root: &Path) -> Args {
        let mut args = Args {
            policy_acceptance: true,
            acceptance_manifest_sha256: None,
            binary: PathBuf::from("unused"),
            binary_sha256: "a".repeat(64),
            source_commit: "b".repeat(40),
            fixture_commit: "c".repeat(40),
            fixture: root.to_owned(),
            scratch: root.join("never-created"),
        };
        for (name, bytes) in [
            ("cohorts.json", include_str!("cohorts.json")),
            ("native-cells.json", include_str!("native-cells.json")),
            (
                "planner-coverage.json",
                include_str!("planner-coverage.json"),
            ),
            ("limits.json", include_str!("limits.json")),
            ("normal-policy.json", include_str!("normal-policy.json")),
            ("empty-policy.json", include_str!("empty-policy.json")),
            ("paired-policy.json", include_str!("paired-policy.json")),
            ("run.rs", RUNNER_SOURCE),
            ("acceptance.rs", MODULE_SOURCE),
            ("control_custody.rs", CUSTODY_SOURCE),
            ("controls.rs", controls::source()),
        ] {
            fs::write(root.join(name), bytes).unwrap();
        }
        let (cohorts, broad, _) = tables();
        let mut children = Vec::new();
        for (id, mut plan) in IDS.iter().zip(plans(&cohorts, &broad)) {
            let mut protocol = json!({"schema":"experiment.protocol/v1","sample_size":{"power":{"alpha":0.05}},"synthetic_unit_test_only":true});
            if let PlanDocument::V2(v2) = &plan {
                let mut scope = v2.clone();
                scope.axes.protocol_sha256 = None;
                let mut bytes = b"crypto.ml-dataset.protocol-plan-scope/v1\0".to_vec();
                bytes.extend(serde_json::to_vec(&scope).unwrap());
                protocol["extensions"] = json!({"ml_dataset_policy_plan":{"schema":"crypto.ml-dataset.protocol-plan-scope/v1","scope_sha256":digest(&bytes),"planned_cells":plan.cell_count().unwrap()}});
            }
            let protocol = freeze_test(protocol);
            let protocol_name = format!("protocols/{id}.json");
            save(root, &protocol_name, &protocol);
            let protocol_sha = bounded_hash(&root.join(&protocol_name)).unwrap();
            match &mut plan {
                PlanDocument::V1(p) => p.protocol_sha256 = Some(protocol_sha.clone()),
                PlanDocument::V2(p) => p.axes.protocol_sha256 = Some(protocol_sha.clone()),
            }
            let plan_name = format!("plans/{id}.json");
            save(root, &plan_name, &serde_json::to_value(&plan).unwrap());
            children.push(json!({"id":id,"plan_sha256":plan.binding_sha256().unwrap(),"plan_file_sha256":bounded_hash(&root.join(plan_name)).unwrap(),"protocol_file_sha256":protocol_sha,"protocol_body_sha256":protocol["freeze"]["protocol_sha256"]}));
        }
        let limits = json_file(root, "limits.json").unwrap();
        let scope = json!({"schema":"ml-dataset.native-policy-acceptance-scope/v1","classification":"SOFTWARE","source_commit":args.source_commit,"binary_sha256":args.binary_sha256,"runner_sha256":digest(RUNNER_SOURCE.as_bytes()),"module_sha256":digest(MODULE_SOURCE.as_bytes()),"custody_sha256":digest(CUSTODY_SOURCE.as_bytes()),"controls_sha256":digest(controls::source().as_bytes()),"children":children,"limits":limits,"native_workers":26,"workflow_count":2});
        save(
            root,
            "umbrella.json",
            &freeze_test(
                json!({"schema":"experiment.protocol/v1","sample_size":{"power":{"alpha":0.05}},"synthetic_unit_test_only":true,"analysis_plan":{"script_sha256":digest(RUNNER_SOURCE.as_bytes())},"extensions":{"native_policy_acceptance":scope}}),
            ),
        );
        let umbrella_sha = bounded_hash(&root.join("umbrella.json")).unwrap();
        save(
            root,
            "evidence/synthetic.json",
            &json!({"synthetic_unit_test_only":true,"execution_performed":false}),
        );
        let mut gates = serde_json::Map::new();
        for gate in [
            "native-fixture-unit-tests",
            "unified-dataset-tests",
            "maintained-clippy",
            "official-umbrella-schema",
            "official-child-schemas",
            "runtime-preflight",
        ] {
            let name = format!("receipts/{gate}.json");
            let mut receipt = json!({"source_commit":args.source_commit,"binary_sha256":args.binary_sha256,"umbrella_sha256":umbrella_sha,"status":"passed","synthetic_unit_test_only":true,"evidence":[{"path":"evidence/synthetic.json","sha256":bounded_hash(&root.join("evidence/synthetic.json")).unwrap()}]});
            if gate == "runtime-preflight" {
                receipt["observed_commands"] = json!(1);
                receipt["observed_wall_nanoseconds"] = json!("1");
                receipt["measurement_source"] = json!("retained_command_receipts");
            }
            save(root, &name, &receipt);
            gates.insert(gate.into(),json!({"status":"passed","receipt":name,"sha256":bounded_hash(&root.join(&name)).unwrap()}));
        }
        save(
            root,
            "readiness.json",
            &json!({"schema":"ml-dataset.native-policy-readiness/v1","source_commit":args.source_commit,"binary_sha256":args.binary_sha256,"umbrella_sha256":umbrella_sha,"limits":limits,"approved_for_local_software":true,"cloud_execution_authorized":false,"gates":gates}),
        );
        refresh_manifest(&mut args);
        let admitted = admit(&args, root).unwrap();
        assert_eq!(admitted.cells.len(), 26);
        assert_eq!(admitted.inputs["readiness_is_supplied_attestation"], true);
        assert_eq!(
            admitted.inputs["official_checker_reexecuted_by_fixture"],
            false
        );
        args
    }
    fn rejected(args: &Args, expected: &str) {
        let error = admit(args, &args.fixture)
            .err()
            .expect("must reject")
            .to_string();
        assert!(
            error.contains(expected),
            "expected {expected}, observed {error}"
        );
        assert!(!args.scratch.exists());
    }
    #[test]
    fn synthetic_complete_admission_reaches_each_readiness_gate() {
        for mutation in [
            "missing-gate",
            "renamed-gate",
            "failed-gate",
            "receipt-source",
            "receipt-missing",
            "receipt-digest",
            "evidence-digest",
            "evidence-missing",
            "umbrella",
            "runtime-count",
            "runtime-clock",
            "runtime-source",
        ] {
            let t = Scratch::new();
            let mut args = admitted_fixture(&t.0);
            let gate = "runtime-preflight";
            let mut ready = json_file(&t.0, "readiness.json").unwrap();
            let mut receipt = json_file(&t.0, "receipts/runtime-preflight.json").unwrap();
            let expected = match mutation {
                "missing-gate" => {
                    ready["gates"]
                        .as_object_mut()
                        .unwrap()
                        .remove("maintained-clippy");
                    "readiness gate set differs"
                }
                "renamed-gate" => {
                    let value = ready["gates"]
                        .as_object_mut()
                        .unwrap()
                        .remove("maintained-clippy")
                        .unwrap();
                    ready["gates"]["unrecognized-gate"] = value;
                    "required readiness gate is absent"
                }
                "failed-gate" => {
                    ready["gates"]["maintained-clippy"]["status"] = json!("failed");
                    "required readiness gate is not passed"
                }
                "receipt-missing" => {
                    fs::remove_file(t.0.join("receipts/runtime-preflight.json")).unwrap();
                    "gate evidence absent from manifest"
                }
                "receipt-digest" => {
                    ready["gates"][gate]["sha256"] = json!("0".repeat(64));
                    "gate receipt digest differs"
                }
                "umbrella" => {
                    ready["umbrella_sha256"] = json!("0".repeat(64));
                    "binding"
                }
                "receipt-source" => {
                    receipt["source_commit"] = json!("d".repeat(40));
                    "binding"
                }
                "evidence-digest" => {
                    receipt["evidence"][0]["sha256"] = json!("0".repeat(64));
                    "gate evidence is not retained"
                }
                "evidence-missing" => {
                    receipt["evidence"][0]["path"] = json!("absent.json");
                    "gate evidence is not retained"
                }
                "runtime-count" => {
                    receipt["observed_commands"] = json!(0);
                    "runtime evidence must be measured"
                }
                "runtime-clock" => {
                    receipt["observed_wall_nanoseconds"] = json!("0");
                    "runtime evidence must be measured"
                }
                "runtime-source" => {
                    receipt["measurement_source"] = json!("budget");
                    "runtime evidence must be measured"
                }
                _ => unreachable!(),
            };
            save(&t.0, "readiness.json", &ready);
            if ![
                "missing-gate",
                "renamed-gate",
                "failed-gate",
                "receipt-digest",
                "receipt-missing",
                "umbrella",
            ]
            .contains(&mutation)
            {
                refresh_receipt(&mut args, gate, &receipt);
            } else {
                refresh_manifest(&mut args);
            }
            rejected(&args, expected);
        }
    }
    #[test]
    fn fractional_official_power_shape_uses_shared_native_canonicalization() {
        let mut protocol = freeze_test(
            json!({"schema":"experiment.protocol/v1","sample_size":{"power":{"alpha":0.05,"target":0.8}},"synthetic_unit_test_only":true}),
        );
        frozen(&protocol).unwrap();
        protocol["sample_size"]["power"]["alpha"] = json!(0.01);
        assert!(frozen(&protocol).is_err());
        // This tests numeric canonicalization, not official schema validation.
    }
    #[test]
    fn actual_file_size_precedes_hash_and_bounded_read_rejects_growth() {
        let t = Scratch::new();
        let path = t.0.join("large.json");
        File::create(&path).unwrap().set_len(MAX_JSON + 1).unwrap();
        assert!(bounded_hash(&path)
            .unwrap_err()
            .to_string()
            .contains("JSON input limit"));
        let mut files = BTreeMap::new();
        for i in 0..26 {
            files.insert(
                format!("{i:02}.json"),
                FileBinding {
                    sha256: "0".repeat(64),
                    bytes: 0,
                },
            );
        }
        fs::rename(&path, t.0.join("00.json")).unwrap();
        let manifest = Manifest {
            schema: String::new(),
            source_commit: String::new(),
            binary_sha256: String::new(),
            files,
        };
        assert!(checked_files(&t.0, &manifest)
            .unwrap_err()
            .to_string()
            .contains("frozen fixture file changed"));
        // A bounded reader with a cap smaller than the observed bytes fails without hashing.
        fs::write(&path, b"12345").unwrap();
        assert!(bounded_limit(&path, 4).is_err());
        assert!(read_bounded(&b"12345"[..], 4)
            .unwrap_err()
            .to_string()
            .contains("JSON grew beyond input limit"));
        assert_eq!(read_bounded(&b"1234"[..], 4).unwrap(), b"1234");
    }
    #[test]
    fn generation_start_and_success_are_distinct_from_completed_checks() {
        let t = Scratch::new();
        fs::create_dir(t.0.join("commands")).unwrap();
        let rows = vec![
            json!({"ordinal":0}),
            json!({"ordinal":1}),
            json!({"ordinal":2}),
            json!({"ordinal":3}),
        ];
        for i in 0..3 {
            save(
                &t.0,
                &format!("commands/cell-{i:02}-generate.started.json"),
                &json!({"intent":true}),
            );
        }
        for i in 0..2 {
            save(
                &t.0,
                &format!("commands/cell-{i:02}-generate.receipt.json"),
                &json!({"exit_status":0,"timed_out":false,"cancelled":false}),
            );
        }
        let states = progress(&t.0, &rows, &[json!({"ordinal":0})]);
        assert_eq!(states[0]["state"], "checked_complete");
        assert_eq!(states[1]["state"], "generated_check_incomplete");
        assert_eq!(states[2]["state"], "attempted_generation_incomplete");
        assert_eq!(states[3]["state"], "never_started");
        assert_eq!(
            progress(&t.0, &rows, &[json!({"ordinal":3})])[3]["state"],
            "unknown_due_to_evidence_error"
        );
    }
    #[test]
    fn frozen_plan_paths_enforce_limits_before_native_plan_command() {
        let t = Scratch::new();
        fs::create_dir_all(t.0.join("fixture/protocols")).unwrap();
        fs::create_dir_all(t.0.join("fixture/plans")).unwrap();
        let protocol = t.0.join("fixture/protocols/v1-bsgs.json");
        let plan = t.0.join("fixture/plans/v1-bsgs.json");
        fs::write(&protocol, b"{}").unwrap();
        fs::write(&plan, b"{}").unwrap();
        let (_, hash, bytes) = frozen_plan_inputs(&t.0, "v1-bsgs").unwrap();
        assert_eq!(hash, digest(b"{}"));
        assert_eq!(bytes, b"{}");
        File::create(&protocol)
            .unwrap()
            .set_len(MAX_JSON + 1)
            .unwrap();
        assert!(frozen_plan_inputs(&t.0, "v1-bsgs")
            .unwrap_err()
            .to_string()
            .contains("JSON input limit"));
        fs::write(&protocol, b"{}").unwrap();
        File::create(&plan).unwrap().set_len(MAX_JSON + 1).unwrap();
        assert!(frozen_plan_inputs(&t.0, "v1-bsgs")
            .unwrap_err()
            .to_string()
            .contains("JSON input limit"));
        assert!(!t.0.join("commands").exists());
    }
    #[test]
    fn phase_receipts_attribute_local_failures_and_do_not_call_generated_cells_unrun() {
        for phase in ["inspect-cell", "inspect-export-retry-and-labels"] {
            let t = Scratch::new();
            save(
                &t.0,
                "commands/cell-00-generate.started.json",
                &json!({"intent":true}),
            );
            save(
                &t.0,
                "commands/cell-00-generate.receipt.json",
                &json!({"exit_status":0,"timed_out":false,"cancelled":false}),
            );
            let mut index = 0;
            recorded_step(&t.0, 0, &mut index, "generate", "execution_attempt", || {
                Ok(())
            })
            .unwrap();
            let result: Result<()> =
                recorded_step(&t.0, 0, &mut index, phase, "verification_attempt", || {
                    Err("synthetic local inspection failure".into())
                });
            assert!(result.is_err());
            let rows = progress(&t.0, &[json!({"ordinal":0})], &[]);
            assert_eq!(rows[0]["state"], "generated_check_incomplete");
            assert_eq!(rows[0]["failed_step"], phase);
            assert_eq!(rows[0]["last_step"]["attempt_kind"], "verification_attempt");
            assert_eq!(
                rows[0]["last_step"]["receipt"]["error"],
                "synthetic local inspection failure"
            );
            assert!(
                rows[0]["last_step"]["receipt"]["observed_controller_operation_nanoseconds"]
                    .as_str()
                    .unwrap()
                    .parse::<u128>()
                    .is_ok()
            );
            assert_eq!(
                rows[0]["last_step"]["receipt"]["nested_command_timings_are_not_additive"],
                true
            );
        }
    }
    #[test]
    fn unfinished_phase_has_unknown_timing_and_generation_intent_is_not_spawn() {
        let t = Scratch::new();
        save(
            &t.0,
            "steps/cell-00/000.started.json",
            &json!({"phase":"generate","attempt_kind":"execution_attempt"}),
        );
        save(
            &t.0,
            "commands/cell-00-generate.started.json",
            &json!({"intent":true}),
        );
        let rows = progress(&t.0, &[json!({"ordinal":0})], &[]);
        assert_eq!(rows[0]["state"], "attempted_generation_incomplete");
        assert_eq!(rows[0]["failed_step"], "generate");
        assert_eq!(rows[0]["last_step"]["completion_known"], false);
        assert!(rows[0]["last_step"]["receipt"].is_null());
        assert_eq!(rows[0]["generation_command_succeeded"], false);
    }
    #[test]
    fn torn_phase_receipt_preserves_readable_rows_and_original_failure_for_sealing() {
        for malformed in [true, false] {
            let t = Scratch::new();
            save(
                &t.0,
                "steps/cell-00/000.started.json",
                &json!({"phase":"inspect-cell","attempt_kind":"verification_attempt"}),
            );
            let bad = t.0.join("steps/cell-00/000.receipt.json");
            if malformed {
                fs::write(&bad, b"{\"completed\":").unwrap();
            } else {
                fs::create_dir(&bad).unwrap();
            }
            save(
                &t.0,
                "commands/cell-00-generate.started.json",
                &json!({"intent":true}),
            );
            save(
                &t.0,
                "commands/cell-00-generate.receipt.json",
                &json!({"exit_status":0,"timed_out":false,"cancelled":false}),
            );
            let rows = progress(&t.0, &[json!({"ordinal":0}), json!({"ordinal":1})], &[]);
            assert_eq!(rows[0]["state"], "unknown_due_to_evidence_error");
            assert_eq!(rows[1]["state"], "never_started");
            assert!(!rows[0]["progress_error"].is_null());
            let errors = progress_errors(&rows);
            assert_eq!(errors.len(), 1);
            // Terminal reporting is still writable; original operation failure is
            // separate from malformed/nonregular evidence and never overwritten.
            let receipt = json!({"execution_failure":"synthetic original failure","cell_progress":rows,"retention_errors":errors,"native_stage_passed":false});
            write_new(&t.0.join("receipt.json"), &receipt).unwrap();
            let saved = json_file(&t.0, "receipt.json").unwrap();
            assert_eq!(saved["execution_failure"], "synthetic original failure");
            assert_eq!(saved["retention_errors"].as_array().unwrap().len(), 1);
        }
    }
}

#[path = "control_custody.rs"]
mod control_custody;
#[path = "controls.rs"]
mod controls;
