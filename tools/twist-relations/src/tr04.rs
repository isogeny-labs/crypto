//! Frozen finite TR-04 orchestration. Audit acceleration is not algorithm time.
use crate::core::{prime, Curve, Parameters, Point};
use crate::evidence::write_json_atomic;
use crate::tr04_relation::{self, RankMilestone, RelationAudit};
use crate::tr04_select::{self, decimal_u128, increment, SelectionAudit, Work};
use crate::tr04_verify;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub const PRIMES: [u64; 3] = [7, 11, 19];
pub const BASE_SIZES: [usize; 2] = [2, 3];
pub const METHODS: [&str; 7] = [
    "random",
    "progression",
    "collision_avoid",
    "twist_l1",
    "twist_l2",
    "twist_l3",
    "twist_l4",
];
pub const CANDIDATE_BUDGET: usize = 8;
pub const WALK_LENGTH: usize = 8;
pub const PROTOCOL_PATH: &str = "research/twist_relations_tr04_20261029/protocol.json";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub path: String,
    pub sha256: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    pub compiled_source_sha256: BTreeMap<String, String>,
    pub protocol_path: String,
    pub protocol_sha256: Option<String>,
    pub protocol_canonical_sha256: Option<String>,
    pub working_tree_revision: Option<String>,
    pub tracked_worktree_clean: Option<bool>,
    pub declared_build_revision: Option<String>,
    pub binary_sha256: Option<String>,
    pub execution_context: BTreeMap<String, String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub schema: String,
    pub status: String,
    pub kind: String,
    pub prime: Option<u64>,
    pub source: Source,
    pub started_unix_utc: u64,
    pub finished_unix_utc: Option<u64>,
    pub candidate_budget: usize,
    pub walk_length: usize,
    pub expected_model_dispositions: u64,
    pub artifacts: Vec<ArtifactRef>,
    pub error: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Combined {
    #[serde(with = "decimal_u128")]
    pub denominator: u128,
    #[serde(with = "decimal_u128")]
    pub selector_success_words: u128,
    #[serde(with = "decimal_u128")]
    pub selector_failure_words: u128,
    #[serde(with = "matrix_decimal")]
    pub rank_hist_by_prefix: Vec<Vec<u128>>,
    #[serde(with = "vec_decimal")]
    pub hits_sum_by_prefix: Vec<u128>,
    #[serde(with = "vec_decimal")]
    pub dependent_hits_sum_by_prefix: Vec<u128>,
    #[serde(with = "decimal_u128")]
    pub full_rank_observations: u128,
    #[serde(with = "decimal_u128")]
    pub budget_exhausted_observations: u128,
    pub selection_work_sum: Work,
    pub pair_setup_work_sum: Work,
    pub search_work_sum: Work,
    pub linear_work_sum: Work,
    pub verification_work_sum: Work,
    pub total_work_sum: Work,
    pub milestones: Vec<RankMilestone>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelationReference {
    pub outcome_index: usize,
    pub artifact: ArtifactRef,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CaseRecord {
    pub schema: String,
    pub parameters: Parameters,
    pub group_order: u64,
    pub g: Point,
    pub base_size: usize,
    pub method: String,
    pub selection: SelectionAudit,
    pub relation_refs: Vec<RelationReference>,
    pub aggregate: Combined,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CachedRelation {
    pub schema: String,
    pub parameters: Parameters,
    pub group_order: u64,
    pub g: Point,
    pub base: Vec<Point>,
    pub setup_prepaid: bool,
    pub audit: RelationAudit,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelRecord {
    pub schema: String,
    pub parameters: Parameters,
    pub nonsingular: bool,
    pub group_order: Option<u64>,
    pub eligible: bool,
    pub canonical_generator: Option<Point>,
    pub cases: Vec<ArtifactRef>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Controls {
    schema: String,
    parameters: Parameters,
    group_order: u64,
    g: Point,
    base: Vec<Point>,
    known_word: String,
    known_prefixes: Vec<usize>,
    known_rows: Vec<Vec<u64>>,
    determinant_mod_order: u64,
    audit: RelationAudit,
    independent_scope: Value,
    negative_control: String,
}

pub fn cli(args: Vec<String>) -> Result<(), String> {
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        println!("twist-tr04 --prime 7|11|19 --output-dir DIR\ntwist-tr04 verify DIR\ntwist-tr04 summarize --input DIR --input DIR --input DIR --output-dir DIR\ntwist-tr04 preflight --output-dir DIR\nFixed B=2,3; seven methods; candidate and binary-walk budgets 8. Existing output directories are rejected.");
        return Ok(());
    }
    if args[0] == "verify" {
        if args.len() != 2 {
            return Err("verify requires exactly one directory".into());
        }
        verify(Path::new(&args[1]))?;
        return Ok(());
    }
    let kind = if args[0] == "summarize" || args[0] == "preflight" {
        args[0].as_str()
    } else {
        "shard"
    };
    let mut at = usize::from(kind != "shard");
    let (mut prime_value, mut output, mut inputs) = (None, None, Vec::new());
    while at < args.len() {
        let flag = &args[at];
        at += 1;
        let value = args
            .get(at)
            .ok_or_else(|| format!("missing value for {flag}"))?;
        at += 1;
        match flag.as_str() {
            "--prime" if kind == "shard" && prime_value.is_none() => {
                prime_value = Some(value.parse::<u64>().map_err(|_| "invalid prime")?)
            }
            "--output-dir" if output.is_none() => output = Some(PathBuf::from(value)),
            "--input" if kind == "summarize" => inputs.push(PathBuf::from(value)),
            _ => return Err(format!("unknown or duplicate argument {flag}")),
        }
    }
    let out = output.ok_or("--output-dir is required")?;
    match kind {
        "preflight" => {
            execute(&out, "preflight", None, None, |writer| {
                let control_started = Instant::now();
                let controls = controls()?;
                let control_elapsed_ns = control_started.elapsed().as_nanos();
                writer.publish("controls.json", &controls)?;
                let parameters = Parameters { p: 7, a: 0, b: 3 };
                let mut rows = Vec::new();
                for method in METHODS {
                    let started = Instant::now();
                    let _calibration_only = tr04_select::selection(parameters, 2, method, 3)?;
                    rows.push(json!({"method":method,"elapsed_ns":started.elapsed().as_nanos().to_string()}));
                }
                writer.publish("calibration.json", &json!({
                "schema":"twist-tr04/development-calibration/v1",
                "parameters":parameters,"base_size":2,"candidate_budget":3,
                "control_audit_elapsed_ns":control_elapsed_ns.to_string(),"methods":rows,
                "scope":"Fixed known-answer development calibration only. No selection distributions, arm comparison, outcome tuning, or admitted timing result. Primary candidate budget remains 8."
            }))?;
                Ok(())
            })
        }
        "summarize" => summarize(&inputs, &out),
        _ => {
            let p = prime_value.ok_or("--prime is required")?;
            if !PRIMES.contains(&p) {
                return Err("prime is outside frozen {7,11,19} scope".into());
            }
            execute(&out, "shard", Some(p), None, |writer| run_shard(p, writer))
        }
    }
}

fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    serde_json::from_slice(&fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?)
        .map_err(|e| format!("{}: {e}", path.display()))
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn source() -> Source {
    let files = [
        ("core.rs", include_str!("core.rs")),
        ("benchmark.rs", include_str!("benchmark.rs")),
        ("exhaustive.rs", include_str!("exhaustive.rs")),
        ("evidence.rs", include_str!("evidence.rs")),
        ("tr04_select.rs", include_str!("tr04_select.rs")),
        ("tr04_relation.rs", include_str!("tr04_relation.rs")),
        ("tr04_verify.rs", include_str!("tr04_verify.rs")),
        ("tr04_protocol.rs", include_str!("tr04_protocol.rs")),
        ("tr04_report.rs", include_str!("tr04_report.rs")),
        ("tr04.rs", include_str!("tr04.rs")),
        ("bin/twist-tr04.rs", include_str!("bin/twist-tr04.rs")),
        ("bin/tr04-protocol.rs", include_str!("bin/tr04-protocol.rs")),
        ("bin/tr04-report.rs", include_str!("bin/tr04-report.rs")),
        ("lib.rs", include_str!("lib.rs")),
        ("Cargo.toml", include_str!("../Cargo.toml")),
        ("Cargo.lock", include_str!("../Cargo.lock")),
    ];
    let revision = Command::new("git")
        .arg("-C")
        .arg(repo_root())
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
    Source {
        compiled_source_sha256: files
            .into_iter()
            .map(|(name, text)| (name.into(), hash(text.as_bytes())))
            .collect(),
        protocol_path: PROTOCOL_PATH.into(),
        protocol_sha256: fs::read(repo_root().join(PROTOCOL_PATH))
            .ok()
            .map(|b| hash(&b)),
        protocol_canonical_sha256: fs::read(repo_root().join(PROTOCOL_PATH))
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
            .and_then(|value| {
                value["freeze"]["protocol_sha256"]
                    .as_str()
                    .map(str::to_string)
            }),
        working_tree_revision: revision,
        tracked_worktree_clean: tracked_clean(&repo_root()),
        declared_build_revision: option_env!("TR04_SOURCE_REVISION").map(str::to_string),
        binary_sha256: std::env::current_exe()
            .ok()
            .and_then(|p| fs::read(p).ok())
            .map(|b| hash(&b)),
        execution_context: [
            "GITHUB_REPOSITORY",
            "GITHUB_RUN_ID",
            "GITHUB_RUN_ATTEMPT",
            "GITHUB_SHA",
            "GITHUB_WORKFLOW_REF",
            "TR04_SOURCE_REVISION",
            "TR04_WORKFLOW_REVISION",
            "TR04_REQUESTED_RUNNER",
            "RUNNER_NAME",
            "RUNNER_OS",
            "RUNNER_ARCH",
        ]
        .into_iter()
        .filter_map(|name| std::env::var(name).ok().map(|value| (name.into(), value)))
        .collect(),
    }
}
fn safe_path(path: &str) -> Result<PathBuf, String> {
    let p = Path::new(path);
    if path.is_empty() || p.components().any(|c| !matches!(c, Component::Normal(_))) {
        return Err("unsafe artifact path".into());
    }
    Ok(p.to_path_buf())
}

fn tracked_clean(root: &Path) -> Option<bool> {
    Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| o.stdout.is_empty())
}

fn verify_source_blobs(root: &Path, source: &Source, revision: &str) -> Result<(), String> {
    for (name, expected) in &source.compiled_source_sha256 {
        safe_path(name)?;
        let directory = if name == "Cargo.toml" || name == "Cargo.lock" {
            "tools/twist-relations"
        } else {
            "tools/twist-relations/src"
        };
        let blob = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["show", &format!("{revision}:{directory}/{name}")])
            .output()
            .map_err(|e| format!("cannot read committed source {name}: {e}"))?;
        if !blob.status.success() || hash(&blob.stdout) != *expected {
            return Err(format!(
                "compiled source differs from declared commit: {name}"
            ));
        }
    }
    Ok(())
}
struct Writer {
    out: PathBuf,
    manifest: Manifest,
    journal: File,
    sequence: u64,
    paths: BTreeSet<String>,
}
impl Writer {
    fn start(out: &Path, kind: &str, p: Option<u64>) -> Result<Self, String> {
        fs::create_dir(out)
            .map_err(|e| format!("fresh output directory {} required: {e}", out.display()))?;
        let manifest = Manifest {
            schema: "twist-tr04/manifest/v1".into(),
            status: "running".into(),
            kind: kind.into(),
            prime: p,
            source: source(),
            started_unix_utc: now(),
            finished_unix_utc: None,
            candidate_budget: CANDIDATE_BUDGET,
            walk_length: WALK_LENGTH,
            expected_model_dispositions: p.map_or(0, |p| p * p),
            artifacts: Vec::new(),
            error: None,
        };
        write_json_atomic(&out.join("manifest.json"), &manifest)?;
        let journal = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out.join("events.jsonl"))
            .map_err(|e| e.to_string())?;
        Ok(Self {
            out: out.into(),
            manifest,
            journal,
            sequence: 0,
            paths: BTreeSet::new(),
        })
    }
    fn event(&mut self, event: Value) -> Result<(), String> {
        let value = json!({"schema":"twist-tr04/event/v1","sequence":self.sequence,"event":event});
        let mut bytes = serde_json::to_vec(&value).map_err(|e| e.to_string())?;
        bytes.push(b'\n');
        self.journal
            .write_all(&bytes)
            .and_then(|_| self.journal.sync_data())
            .map_err(|e| e.to_string())?;
        write_json_atomic(&self.out.join("active-case.json"), &value)?;
        self.sequence = self
            .sequence
            .checked_add(1)
            .ok_or("event sequence overflow")?;
        Ok(())
    }
    fn publish(&mut self, path: &str, value: &impl Serialize) -> Result<ArtifactRef, String> {
        if !self.paths.insert(path.into()) {
            return Err(format!("duplicate artifact {path}"));
        }
        let local = self.out.join(safe_path(path)?);
        fs::create_dir_all(local.parent().ok_or("missing artifact parent")?)
            .map_err(|e| e.to_string())?;
        if local.exists() {
            return Err("refusing to replace completed evidence".into());
        }
        write_json_atomic(&local, value)?;
        let reference = ArtifactRef {
            path: path.into(),
            sha256: hash(&fs::read(&local).map_err(|e| e.to_string())?),
        };
        self.manifest.artifacts.push(reference.clone());
        self.event(json!({"type":"artifact_completed","artifact":reference}))?;
        Ok(reference)
    }
    fn publish_bytes(&mut self, path: &str, bytes: &[u8]) -> Result<ArtifactRef, String> {
        if !self.paths.insert(path.into()) {
            return Err(format!("duplicate artifact {path}"));
        }
        let local = self.out.join(safe_path(path)?);
        fs::create_dir_all(local.parent().ok_or("missing artifact parent")?)
            .map_err(|e| e.to_string())?;
        if local.exists() {
            return Err("refusing to replace completed evidence".into());
        }
        let temporary = local.with_extension("raw.tmp");
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|e| e.to_string())?;
        file.write_all(bytes)
            .and_then(|_| file.sync_all())
            .map_err(|e| e.to_string())?;
        fs::rename(&temporary, &local).map_err(|e| e.to_string())?;
        #[cfg(unix)]
        File::open(local.parent().ok_or("missing parent")?)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())?;
        let reference = ArtifactRef {
            path: path.into(),
            sha256: hash(bytes),
        };
        self.manifest.artifacts.push(reference.clone());
        self.event(json!({"type":"artifact_completed","artifact":reference}))?;
        Ok(reference)
    }
    fn finish(&mut self, error: Option<String>) -> Result<(), String> {
        self.event(json!({"type":if error.is_some(){"failed"}else{"completed"},"error":error}))?;
        for path in ["events.jsonl", "active-case.json"] {
            self.manifest.artifacts.push(ArtifactRef {
                path: path.into(),
                sha256: hash(&fs::read(self.out.join(path)).map_err(|e| e.to_string())?),
            });
        }
        self.manifest.artifacts.sort_by(|a, b| a.path.cmp(&b.path));
        self.manifest.status = if error.is_some() {
            "failed"
        } else {
            "complete"
        }
        .into();
        self.manifest.finished_unix_utc = Some(now());
        self.manifest.error = error;
        write_json_atomic(&self.out.join("manifest.json"), &self.manifest)
    }
}
fn execute(
    out: &Path,
    kind: &str,
    p: Option<u64>,
    initial_index: Option<Value>,
    run: impl FnOnce(&mut Writer) -> Result<(), String>,
) -> Result<(), String> {
    let mut writer = Writer::start(out, kind, p)?;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if let Some(index) = initial_index {
            writer.publish("shard-inputs.json", &index)?;
        }
        validate_source_identity(&writer.manifest.source, kind == "shard")?;
        if let (Some(declared), Ok(runtime)) = (
            &writer.manifest.source.declared_build_revision,
            std::env::var("TR04_SOURCE_REVISION"),
        ) {
            if declared != &runtime {
                return Err("runtime source revision differs from compiled build revision".into());
            }
        }
        crate::tr04_protocol::validate(&repo_root().join(PROTOCOL_PATH), kind != "preflight")?;
        let protocol = fs::read(repo_root().join(PROTOCOL_PATH))
            .map_err(|e| format!("frozen protocol unavailable: {e}"))?;
        let _: Value = serde_json::from_slice(&protocol).map_err(|e| e.to_string())?;
        writer.publish_bytes("protocol.json", &protocol)?;
        run(&mut writer)
    }))
    .unwrap_or_else(|_| Err("panic during bounded audit; completed prefix retained".into()));
    writer.finish(result.clone().err())?;
    result
}
fn validate_source_identity(source: &Source, required: bool) -> Result<(), String> {
    if required && source.tracked_worktree_clean != Some(true) {
        return Err("primary shard requires a recorded clean tracked worktree".into());
    }
    match &source.declared_build_revision {
        Some(revision) => {
            if revision.len() != 40
                || !revision.bytes().all(|b| b.is_ascii_hexdigit())
                || source.working_tree_revision.as_ref() != Some(revision)
            {
                return Err("declared and working-tree source revisions are inconsistent".into());
            }
            if required {
                verify_source_blobs(&repo_root(), source, revision)?;
            }
        }
        None if required => {
            return Err("primary shard requires immutable TR04_SOURCE_REVISION build pin".into())
        }
        None => {}
    }
    Ok(())
}

fn controls() -> Result<Controls, String> {
    let parameters = Parameters { p: 7, a: 0, b: 3 };
    let n = 13;
    let g = Point { x: 1, y: 2 };
    let base = vec![g, Point { x: 2, y: 2 }];
    let audit = tr04_relation::audit(parameters, n, g, &base, WALK_LENGTH, false)?;
    if audit.pair_table_entries != 3 || audit.rank_hist_by_prefix[6][3] < 4 {
        return Err("positive rank calibration or B2 pair control failed".into());
    }
    let scope = tr04_verify::verify_relation(parameters, n, g, &base, WALK_LENGTH, &audit)?;
    tr04_verify::verify_fixed_controls()?;
    Ok(Controls {schema:"twist-tr04/controls/v1".into(),parameters,group_order:n,g,base,known_word:"GGQQGGGG".into(),known_prefixes:vec![2,4,6],known_rows:vec![vec![2,0,0],vec![1,1,11],vec![0,2,11]],determinant_mod_order:4,audit,independent_scope:serde_json::to_value(scope).map_err(|e|e.to_string())?,negative_control:"Every B=2 canonical distinct-point base in odd prime order has exactly three distinct i<=j sums; asserted for every cached B2 audit".into()})
}
fn production_model(parameters: Parameters) -> Result<ModelRecord, String> {
    let mut curve = Curve::new(parameters.p, parameters.a, parameters.b);
    let nonsingular = curve.nonsingular();
    let group_order = if nonsingular {
        Some(
            (parameters.p as i64 + 1 + (0..parameters.p).map(|x| curve.character(x)).sum::<i64>())
                as u64,
        )
    } else {
        None
    };
    let eligible = group_order.is_some_and(|n| n >= 7 && prime(n));
    let generator = if eligible {
        (0..parameters.p).find_map(|x| curve.point(x, false))
    } else {
        None
    };
    if eligible && generator.is_none() {
        return Err("eligible model has no canonical generator".into());
    }
    Ok(ModelRecord {
        schema: "twist-tr04/model/v1".into(),
        parameters,
        nonsingular,
        group_order,
        eligible,
        canonical_generator: generator,
        cases: Vec::new(),
    })
}
fn model_name(parameters: Parameters) -> String {
    format!("p{}-a{}-b{}", parameters.p, parameters.a, parameters.b)
}
fn cache_path(parameters: Parameters, base: &[Point], prepaid: bool) -> Result<String, String> {
    Ok(format!(
        "relations/{}-{}-{}.json",
        model_name(parameters),
        hash(&serde_json::to_vec(base).map_err(|e| e.to_string())?),
        u8::from(prepaid)
    ))
}
fn run_shard(p: u64, writer: &mut Writer) -> Result<(), String> {
    writer.publish("controls.json", &controls()?)?;
    for a in 0..p {
        for b in 0..p {
            let parameters = Parameters { p, a, b };
            writer.event(json!({"type":"model_started","parameters":parameters}))?;
            let mut model = production_model(parameters)?;
            if model.eligible {
                let n = model.group_order.ok_or("missing order")?;
                let g = model.canonical_generator.ok_or("missing generator")?;
                let mut cache: BTreeMap<String, (ArtifactRef, CachedRelation)> = BTreeMap::new();
                for size in BASE_SIZES {
                    for method in METHODS {
                        writer.event(json!({"type":"case_started","parameters":parameters,"base_size":size,"method":method}))?;
                        let selection =
                            tr04_select::selection(parameters, size, method, CANDIDATE_BUDGET)?;
                        let mut refs = Vec::new();
                        let mut audits = BTreeMap::new();
                        for (index, outcome) in selection.outcomes.iter().enumerate() {
                            if outcome.base.len() != size {
                                continue;
                            }
                            let prepaid = tr04_select::reuses_pair_table(method);
                            let path = cache_path(parameters, &outcome.base, prepaid)?;
                            if !cache.contains_key(&path) {
                                writer.event(json!({"type":"relation_cache_started","parameters":parameters,"base":outcome.base,"setup_prepaid":prepaid}))?;
                                let audit = tr04_relation::audit(
                                    parameters,
                                    n,
                                    g,
                                    &outcome.base,
                                    WALK_LENGTH,
                                    prepaid,
                                )?;
                                if size == 2 && audit.pair_table_entries != 3 {
                                    return Err("B2 negative control failed".into());
                                }
                                let cached = CachedRelation {
                                    schema: "twist-tr04/relation-cache/v1".into(),
                                    parameters,
                                    group_order: n,
                                    g,
                                    base: outcome.base.clone(),
                                    setup_prepaid: prepaid,
                                    audit,
                                };
                                let reference = writer.publish(&path, &cached)?;
                                cache.insert(path.clone(), (reference, cached));
                            }
                            let (reference, cached) =
                                cache.get(&path).ok_or("missing cached audit")?;
                            refs.push(RelationReference {
                                outcome_index: index,
                                artifact: reference.clone(),
                            });
                            audits.insert(index, cached.audit.clone());
                        }
                        let aggregate = combine(n, size, &selection, &audits)?;
                        let record = CaseRecord {
                            schema: "twist-tr04/case/v1".into(),
                            parameters,
                            group_order: n,
                            g,
                            base_size: size,
                            method: method.into(),
                            selection,
                            relation_refs: refs,
                            aggregate,
                        };
                        let path =
                            format!("cases/{}-B{size}-{method}.json", model_name(parameters));
                        model.cases.push(writer.publish(&path, &record)?);
                    }
                }
            }
            writer.publish(&format!("models/{}.json", model_name(parameters)), &model)?;
        }
    }
    Ok(())
}

fn times(a: u128, b: u128) -> Result<u128, String> {
    a.checked_mul(b)
        .ok_or("weighted census multiplication overflow".into())
}
fn add_scaled(dst: &mut u128, value: u128, scale: u128) -> Result<(), String> {
    increment(dst, times(value, scale)?)
}
fn zero_milestones(size: usize) -> Vec<RankMilestone> {
    (1..=size + 1)
        .map(|rank| RankMilestone {
            rank,
            attained: 0,
            missing: 0,
            first_prefix_hist: BTreeMap::new(),
            work_sum: Work::default(),
            attained_work_sum: Work::default(),
            missing_work_sum: Work::default(),
        })
        .collect()
}
/// Reconcile weighted exact distributions. Replaying this reducer is aggregate
/// arithmetic reconciliation, not a second independent work-counter producer.
pub fn combine(
    n: u64,
    size: usize,
    selection: &SelectionAudit,
    audits: &BTreeMap<usize, RelationAudit>,
) -> Result<Combined, String> {
    if !(2..=3).contains(&size) || n < 7 {
        return Err("invalid aggregate dimension".into());
    }
    let relation_observations = times(n as u128 - 1, 1u128 << WALK_LENGTH)?;
    let denominator = times(selection.total_words, relation_observations)?;
    let mut result = Combined {
        denominator,
        selector_success_words: 0,
        selector_failure_words: 0,
        rank_hist_by_prefix: vec![vec![0; size + 2]; WALK_LENGTH + 1],
        hits_sum_by_prefix: vec![0; WALK_LENGTH + 1],
        dependent_hits_sum_by_prefix: vec![0; WALK_LENGTH + 1],
        full_rank_observations: 0,
        budget_exhausted_observations: 0,
        selection_work_sum: Work::default(),
        pair_setup_work_sum: Work::default(),
        search_work_sum: Work::default(),
        linear_work_sum: Work::default(),
        verification_work_sum: Work::default(),
        total_work_sum: Work::default(),
        milestones: zero_milestones(size),
    };
    let mut used = BTreeSet::new();
    for (index, outcome) in selection.outcomes.iter().enumerate() {
        let mass = times(outcome.ways, relation_observations)?;
        let paid_selection = outcome.work_sum.scaled(relation_observations)?;
        result.selection_work_sum.accumulate(&paid_selection)?;
        result.total_work_sum.accumulate(&paid_selection)?;
        if outcome.base.len() < size {
            if audits.contains_key(&index) {
                return Err("failed selector received a relation audit".into());
            }
            increment(&mut result.selector_failure_words, outcome.ways)?;
            increment(&mut result.budget_exhausted_observations, mass)?;
            for hist in &mut result.rank_hist_by_prefix {
                increment(&mut hist[0], mass)?;
            }
            for milestone in &mut result.milestones {
                increment(&mut milestone.missing, mass)?;
                milestone.work_sum.accumulate(&paid_selection)?;
                milestone.missing_work_sum.accumulate(&paid_selection)?;
            }
            continue;
        }
        if outcome.base.len() != size {
            return Err("selector base exceeds declared size".into());
        }
        increment(&mut result.selector_success_words, outcome.ways)?;
        let audit = audits
            .get(&index)
            .ok_or("completed selector missing relation audit")?;
        used.insert(index);
        tr04_relation::reconcile(audit)?;
        if audit.base_size != size
            || audit.walk_length != WALK_LENGTH
            || audit.observations != relation_observations
            || audit.setup_prepaid != tr04_select::reuses_pair_table(&selection.method)
        {
            return Err("relation audit domain/provenance mismatch".into());
        }
        result
            .pair_setup_work_sum
            .add_scaled(&audit.pair_setup_work_sum, outcome.ways)?;
        result
            .search_work_sum
            .add_scaled(&audit.search_work_sum, outcome.ways)?;
        result
            .linear_work_sum
            .add_scaled(&audit.linear_work_sum, outcome.ways)?;
        result
            .verification_work_sum
            .add_scaled(&audit.verification_work_sum, outcome.ways)?;
        result
            .total_work_sum
            .add_scaled(&audit.total_work_sum, outcome.ways)?;
        add_scaled(
            &mut result.full_rank_observations,
            audit.full_rank_words,
            outcome.ways,
        )?;
        add_scaled(
            &mut result.budget_exhausted_observations,
            audit.budget_exhausted_words,
            outcome.ways,
        )?;
        for depth in 0..=WALK_LENGTH {
            for rank in 0..size + 2 {
                add_scaled(
                    &mut result.rank_hist_by_prefix[depth][rank],
                    audit.rank_hist_by_prefix[depth][rank],
                    outcome.ways,
                )?;
            }
            add_scaled(
                &mut result.hits_sum_by_prefix[depth],
                audit.hits_sum_by_prefix[depth],
                outcome.ways,
            )?;
            add_scaled(
                &mut result.dependent_hits_sum_by_prefix[depth],
                audit.dependent_hits_sum_by_prefix[depth],
                outcome.ways,
            )?;
        }
        for (dst, src) in result.milestones.iter_mut().zip(&audit.milestones) {
            add_scaled(&mut dst.attained, src.attained, outcome.ways)?;
            add_scaled(&mut dst.missing, src.missing, outcome.ways)?;
            for (&prefix, &count) in &src.first_prefix_hist {
                add_scaled(
                    dst.first_prefix_hist.entry(prefix).or_default(),
                    count,
                    outcome.ways,
                )?;
            }
            dst.work_sum.add_scaled(&src.work_sum, outcome.ways)?;
            dst.work_sum.accumulate(&paid_selection)?;
            dst.attained_work_sum
                .add_scaled(&src.attained_work_sum, outcome.ways)?;
            dst.attained_work_sum
                .add_scaled(&outcome.work_sum, src.attained)?;
            dst.missing_work_sum
                .add_scaled(&src.missing_work_sum, outcome.ways)?;
            dst.missing_work_sum
                .add_scaled(&outcome.work_sum, src.missing)?;
        }
    }
    if used.len() != audits.len() {
        return Err("extra relation audits in aggregate".into());
    }
    if result
        .selector_success_words
        .checked_add(result.selector_failure_words)
        != Some(selection.total_words)
        || result
            .full_rank_observations
            .checked_add(result.budget_exhausted_observations)
            != Some(denominator)
    {
        return Err("weighted outcome census mismatch".into());
    }
    for hist in &result.rank_hist_by_prefix {
        if sum(hist.iter().copied())? != denominator {
            return Err("weighted rank census mismatch".into());
        }
    }
    let mut total = result.selection_work_sum.clone();
    for phase in [
        &result.pair_setup_work_sum,
        &result.search_work_sum,
        &result.linear_work_sum,
        &result.verification_work_sum,
    ] {
        total.accumulate(phase)?;
    }
    if total != result.total_work_sum {
        return Err("weighted phase totals mismatch".into());
    }
    for milestone in &result.milestones {
        if milestone.attained.checked_add(milestone.missing) != Some(denominator)
            || sum(milestone.first_prefix_hist.values().copied())? != milestone.attained
            || milestone
                .attained_work_sum
                .add(&milestone.missing_work_sum)?
                != milestone.work_sum
        {
            return Err("weighted milestone census mismatch".into());
        }
    }
    Ok(result)
}
fn sum(mut values: impl Iterator<Item = u128>) -> Result<u128, String> {
    values.try_fold(0u128, |acc, v| {
        acc.checked_add(v).ok_or("census sum overflow".into())
    })
}

fn checked_artifact<T: DeserializeOwned>(
    dir: &Path,
    reference: &ArtifactRef,
    available: &BTreeMap<String, String>,
) -> Result<T, String> {
    if available.get(&reference.path) != Some(&reference.sha256) {
        return Err("reference absent from exact manifest inventory".into());
    }
    read_json(&dir.join(safe_path(&reference.path)?))
}
fn files_under(dir: &Path, prefix: &str, files: &mut BTreeSet<String>) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let relative = if prefix.is_empty() {
            name
        } else {
            format!("{prefix}/{name}")
        };
        let kind = entry.file_type().map_err(|e| e.to_string())?;
        if kind.is_symlink() {
            return Err("symbolic links are not raw evidence".into());
        }
        if kind.is_dir() {
            files_under(&entry.path(), &relative, files)?;
        } else if kind.is_file() {
            files.insert(relative);
        } else {
            return Err("nonregular evidence entry".into());
        }
    }
    Ok(())
}
fn verified_manifest(dir: &Path) -> Result<(Manifest, BTreeMap<String, String>, String), String> {
    let bytes = fs::read(dir.join("manifest.json")).map_err(|e| e.to_string())?;
    let manifest: Manifest = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if manifest.schema != "twist-tr04/manifest/v1"
        || manifest.status != "complete"
        || manifest.error.is_some()
        || manifest.finished_unix_utc.is_none()
        || manifest.candidate_budget != CANDIDATE_BUDGET
        || manifest.walk_length != WALK_LENGTH
    {
        return Err("manifest is incomplete or incompatible".into());
    }
    let current = source();
    validate_source_identity(&manifest.source, manifest.kind == "shard")?;
    if manifest.source.compiled_source_sha256 != current.compiled_source_sha256
        || manifest.source.protocol_path != PROTOCOL_PATH
        || manifest.source.protocol_sha256.is_none()
        || (manifest.kind != "preflight"
            && (manifest.source.protocol_sha256 != current.protocol_sha256
                || manifest.source.protocol_canonical_sha256 != current.protocol_canonical_sha256))
    {
        return Err("compiled source or frozen protocol identity mismatch".into());
    }
    let mut inventory = BTreeMap::new();
    for reference in &manifest.artifacts {
        if inventory
            .insert(reference.path.clone(), reference.sha256.clone())
            .is_some()
        {
            return Err("duplicate manifest artifact".into());
        }
        let path = dir.join(safe_path(&reference.path)?);
        if hash(&fs::read(path).map_err(|e| e.to_string())?) != reference.sha256 {
            return Err(format!("artifact hash mismatch: {}", reference.path));
        }
    }
    let expected = inventory
        .keys()
        .cloned()
        .chain(["manifest.json".to_string()])
        .collect::<BTreeSet<_>>();
    let mut actual = BTreeSet::new();
    files_under(dir, "", &mut actual)?;
    actual.remove("verification.json");
    if actual != expected {
        return Err("missing or unexpected raw evidence files".into());
    }
    if inventory.get("protocol.json") != manifest.source.protocol_sha256.as_ref() {
        return Err("copied frozen protocol hash mismatch".into());
    }
    crate::tr04_protocol::validate_with_analysis_protocol(
        &dir.join("protocol.json"),
        &repo_root().join(PROTOCOL_PATH),
        manifest.kind != "preflight",
    )?;
    if inventory.keys().any(|path| {
        ![
            "protocol.json",
            "controls.json",
            "calibration.json",
            "events.jsonl",
            "active-case.json",
        ]
        .contains(&path.as_str())
            && !path.starts_with("models/")
            && !path.starts_with("cases/")
            && !path.starts_with("relations/")
    }) {
        return Err("unexpected artifact category in raw shard".into());
    }
    if !inventory.contains_key("controls.json")
        || !inventory.contains_key("events.jsonl")
        || !inventory.contains_key("active-case.json")
    {
        return Err("missing control or prefix-retention evidence".into());
    }
    Ok((manifest, inventory, hash(&bytes)))
}
fn verify_controls(dir: &Path, inventory: &BTreeMap<String, String>) -> Result<(), String> {
    let reference = ArtifactRef {
        path: "controls.json".into(),
        sha256: inventory
            .get("controls.json")
            .ok_or("missing controls")?
            .clone(),
    };
    let c: Controls = checked_artifact(dir, &reference, inventory)?;
    if c.schema != "twist-tr04/controls/v1"
        || c.parameters != (Parameters { p: 7, a: 0, b: 3 })
        || c.group_order != 13
        || c.g != (Point { x: 1, y: 2 })
        || c.base != vec![Point { x: 1, y: 2 }, Point { x: 2, y: 2 }]
        || c.known_word != "GGQQGGGG"
        || c.known_prefixes != [2, 4, 6]
        || c.known_rows != vec![vec![2, 0, 0], vec![1, 1, 11], vec![0, 2, 11]]
        || c.determinant_mod_order != 4
        || c.audit.pair_table_entries != 3
        || c.audit
            .rank_hist_by_prefix
            .get(6)
            .and_then(|h| h.get(3))
            .is_none_or(|&v| v < 4)
    {
        return Err("fixed control metadata or outcome mismatch".into());
    }
    tr04_verify::verify_fixed_controls()?;
    tr04_verify::verify_relation(
        c.parameters,
        c.group_order,
        c.g,
        &c.base,
        WALK_LENGTH,
        &c.audit,
    )?;
    Ok(())
}
/// Independent arithmetic/selection/rank replay plus explicit aggregate
/// reconciliation. Work-counter independence is deliberately false.
pub fn verify(dir: &Path) -> Result<Value, String> {
    let (manifest, inventory, manifest_hash) = verified_manifest(dir)?;
    verify_controls(dir, &inventory)?;
    let mut checked_cases = 0usize;
    let mut checked_relations = 0usize;
    let mut checked_models = 0usize;
    if manifest.kind == "shard" {
        let p = manifest.prime.ok_or("shard missing prime")?;
        if !PRIMES.contains(&p) || manifest.expected_model_dispositions != p * p {
            return Err("unexpected shard scope".into());
        }
        let mut used_cases = BTreeSet::new();
        let mut used_relations = BTreeSet::new();
        for a in 0..p {
            for b in 0..p {
                let parameters = Parameters { p, a, b };
                let path = format!("models/{}.json", model_name(parameters));
                let reference = ArtifactRef {
                    path: path.clone(),
                    sha256: inventory
                        .get(&path)
                        .ok_or("missing coefficient-model disposition")?
                        .clone(),
                };
                let model: ModelRecord = checked_artifact(dir, &reference, &inventory)?;
                let independent = tr04_verify::model_info(parameters)?;
                if model.schema != "twist-tr04/model/v1"
                    || model.parameters != parameters
                    || model.nonsingular != independent.nonsingular
                    || model.group_order != independent.group_order
                    || model.eligible != independent.eligible
                    || model.canonical_generator != independent.canonical_generator
                {
                    return Err("coefficient-model disposition mismatch".into());
                }
                checked_models += 1;
                if !model.eligible {
                    if !model.cases.is_empty() {
                        return Err("ineligible model has cases".into());
                    }
                    continue;
                }
                if model.cases.len() != BASE_SIZES.len() * METHODS.len() {
                    return Err("eligible model lacks complete case census".into());
                }
                let n = model.group_order.ok_or("eligible model missing order")?;
                let g = model
                    .canonical_generator
                    .ok_or("eligible model missing G")?;
                let mut expected_cases = BTreeSet::new();
                for size in BASE_SIZES {
                    for method in METHODS {
                        expected_cases.insert(format!(
                            "cases/{}-B{size}-{method}.json",
                            model_name(parameters)
                        ));
                    }
                }
                for reference in &model.cases {
                    if !expected_cases.remove(&reference.path)
                        || !used_cases.insert(reference.path.clone())
                    {
                        return Err("duplicate or unexpected case".into());
                    }
                    let case: CaseRecord = checked_artifact(dir, reference, &inventory)?;
                    if case.schema != "twist-tr04/case/v1"
                        || case.parameters != parameters
                        || case.group_order != n
                        || case.g != g
                        || !BASE_SIZES.contains(&case.base_size)
                        || !METHODS.contains(&case.method.as_str())
                        || case.selection.method != case.method
                    {
                        return Err("case identity mismatch".into());
                    }
                    let expected_path = format!(
                        "cases/{}-B{}-{}.json",
                        model_name(parameters),
                        case.base_size,
                        case.method
                    );
                    if reference.path != expected_path {
                        return Err("case path and body identity mismatch".into());
                    }
                    tr04_verify::verify_selection(
                        parameters,
                        case.base_size,
                        &case.method,
                        CANDIDATE_BUDGET,
                        &case.selection,
                    )?;
                    let mut audits = BTreeMap::new();
                    for relation_ref in &case.relation_refs {
                        let outcome = case
                            .selection
                            .outcomes
                            .get(relation_ref.outcome_index)
                            .ok_or("invalid outcome reference")?;
                        if outcome.base.len() != case.base_size
                            || audits.contains_key(&relation_ref.outcome_index)
                        {
                            return Err("invalid or duplicate relation mapping".into());
                        }
                        let cached: CachedRelation =
                            checked_artifact(dir, &relation_ref.artifact, &inventory)?;
                        let prepaid = tr04_select::reuses_pair_table(&case.method);
                        if cached.schema != "twist-tr04/relation-cache/v1"
                            || cached.parameters != parameters
                            || cached.group_order != n
                            || cached.g != g
                            || cached.base != outcome.base
                            || cached.setup_prepaid != prepaid
                            || cached.audit.setup_prepaid != prepaid
                            || relation_ref.artifact.path
                                != cache_path(parameters, &outcome.base, prepaid)?
                        {
                            return Err("cache or prepaid-table provenance mismatch".into());
                        }
                        if used_relations.insert(relation_ref.artifact.path.clone()) {
                            tr04_verify::verify_relation(
                                parameters,
                                n,
                                g,
                                &cached.base,
                                WALK_LENGTH,
                                &cached.audit,
                            )?;
                            checked_relations += 1;
                        }
                        if case.base_size == 2 && cached.audit.pair_table_entries != 3 {
                            return Err("B2 control failure".into());
                        }
                        audits.insert(relation_ref.outcome_index, cached.audit);
                    }
                    if combine(n, case.base_size, &case.selection, &audits)? != case.aggregate {
                        return Err("weighted aggregate arithmetic mismatch".into());
                    }
                    checked_cases += 1;
                }
                if !expected_cases.is_empty() {
                    return Err("missing expected case".into());
                }
            }
        }
        let actual_models = inventory
            .keys()
            .filter(|p| p.starts_with("models/"))
            .count();
        let actual_cases = inventory
            .keys()
            .filter(|p| p.starts_with("cases/"))
            .cloned()
            .collect::<BTreeSet<_>>();
        let actual_relations = inventory
            .keys()
            .filter(|p| p.starts_with("relations/"))
            .cloned()
            .collect::<BTreeSet<_>>();
        if actual_models != checked_models
            || actual_cases != used_cases
            || actual_relations != used_relations
        {
            return Err("extra model, case or unreferenced relation cache".into());
        }
    } else if manifest.kind != "preflight" {
        return Err("verify accepts shard or preflight records only".into());
    }
    let result = json!({"schema":"twist-tr04/verification/v1","status":"passed","manifest_sha256":manifest_hash,"prime":manifest.prime,"checked_model_dispositions":checked_models,"checked_cases":checked_cases,"checked_unique_relation_caches":checked_relations,"group_arithmetic_independent":true,"selector_multiplicities_independent":true,"rank_and_digest_independent":true,"work_counters_independent":false,"aggregate_arithmetic":"same declared reducer replay over independently verified selector/relation records; not an independent work-counter implementation","verified_unix_utc":now()});
    write_json_atomic(&dir.join("verification.json"), &result)?;
    Ok(result)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummaryModel {
    pub parameters: Parameters,
    pub group_order: u64,
    pub g: Point,
    pub case_ref: ArtifactRef,
    pub aggregate: Combined,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummaryCell {
    pub prime: u64,
    pub base_size: usize,
    pub method: String,
    pub model_count: usize,
    pub models: Vec<SummaryModel>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Summary {
    pub schema: String,
    pub shards: Vec<Value>,
    pub cells: Vec<SummaryCell>,
    pub scope_note: String,
}
fn summarize(inputs: &[PathBuf], out: &Path) -> Result<(), String> {
    let index = shard_input_index(inputs);
    execute(out, "summary", None, Some(index), |writer| {
        if inputs.len() != PRIMES.len() {
            return Err("summary requires exactly three frozen field shards".into());
        }
        writer.publish("controls.json", &controls()?)?;
        let mut seen = BTreeSet::new();
        let mut identities = None;
        let mut shards = Vec::new();
        let mut cells = BTreeMap::<(u64, usize, String), SummaryCell>::new();
        for input in inputs {
            let (manifest, inventory, manifest_hash) = verified_manifest(input)?;
            if manifest.kind != "shard" {
                return Err("non-shard summary input".into());
            }
            let p = manifest.prime.ok_or("missing shard prime")?;
            if !PRIMES.contains(&p) || !seen.insert(p) {
                return Err("duplicate or unexpected summary shard".into());
            }
            let verification: Value = read_json(&input.join("verification.json"))?;
            if verification["schema"] != "twist-tr04/verification/v1"
                || verification["status"] != "passed"
                || verification["manifest_sha256"] != manifest_hash
                || verification["work_counters_independent"] != false
                || verification["checked_model_dispositions"].as_u64() != Some(p * p)
                || verification["checked_cases"].as_u64()
                    != Some(
                        inventory
                            .keys()
                            .filter(|path| path.starts_with("cases/"))
                            .count() as u64,
                    )
                || verification["checked_unique_relation_caches"].as_u64()
                    != Some(
                        inventory
                            .keys()
                            .filter(|path| path.starts_with("relations/"))
                            .count() as u64,
                    )
            {
                return Err("shard lacks matching independent verification receipt".into());
            }
            let identity = (
                manifest.source.compiled_source_sha256.clone(),
                manifest.source.protocol_sha256.clone(),
                manifest.source.working_tree_revision.clone(),
                manifest.source.declared_build_revision.clone(),
            );
            if identities.as_ref().is_some_and(|old| old != &identity) {
                return Err("mixed source/protocol identities in summary".into());
            }
            identities = Some(identity);
            let canonical_input = fs::canonicalize(input).map_err(|e| e.to_string())?;
            let provenance = input_entry(input);
            shards.push(json!({"prime":p,"path":canonical_input,"input_path":canonical_input,"manifest_path":canonical_input.join("manifest.json"),"verification_path":canonical_input.join("verification.json"),"execution_path":provenance["execution_path"],"outcome_path":provenance["outcome_path"],"assigned_job_path":provenance["assigned_job_path"],"manifest_sha256":manifest_hash,"verification_sha256":hash(&fs::read(input.join("verification.json")).map_err(|e|e.to_string())?),"source":manifest.source}));
            for size in BASE_SIZES {
                for method in METHODS {
                    cells.insert(
                        (p, size, method.into()),
                        SummaryCell {
                            prime: p,
                            base_size: size,
                            method: method.into(),
                            model_count: 0,
                            models: Vec::new(),
                        },
                    );
                }
            }
            for (path, sha) in &inventory {
                if path.starts_with("cases/") {
                    let case: CaseRecord = checked_artifact(
                        input,
                        &ArtifactRef {
                            path: path.clone(),
                            sha256: sha.clone(),
                        },
                        &inventory,
                    )?;
                    let cell = cells
                        .get_mut(&(p, case.base_size, case.method.clone()))
                        .ok_or("unexpected summary case")?;
                    cell.models.push(SummaryModel {
                        parameters: case.parameters,
                        group_order: case.group_order,
                        g: case.g,
                        case_ref: ArtifactRef {
                            path: path.clone(),
                            sha256: sha.clone(),
                        },
                        aggregate: case.aggregate,
                    });
                }
            }
        }
        if seen != PRIMES.into_iter().collect() {
            return Err("missing summary prime".into());
        }
        for cell in cells.values_mut() {
            cell.models
                .sort_by_key(|m| (m.parameters.a, m.parameters.b));
            cell.model_count = cell.models.len();
            if cell.model_count == 0 {
                return Err("empty summary cell".into());
            }
        }
        shards.sort_by_key(|value| value["prime"].as_u64());
        let summary=Summary {schema:"twist-tr04/summary/v1".into(),shards,cells:cells.into_values().collect(),scope_note:"42 p×B×method cells, with each coefficient model retaining its own exact decimal numerator/denominator. No field/base/model probability pooling or runtime speedup is inferred. Candidate DP and relation-cache memoization are audit acceleration, not free algorithm work.".into()};
        if summary.cells.len() != 42 {
            return Err("summary cell census mismatch".into());
        }
        writer.publish("summary.json", &summary)?;
        Ok(())
    })
}
fn input_entry(input: &Path) -> Value {
    let absolute = fs::canonicalize(input).unwrap_or_else(|_| {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(input)
    });
    let present = |name: &str| -> Option<PathBuf> {
        [
            absolute.join(name),
            absolute.parent().unwrap_or(&absolute).join(name),
        ]
        .into_iter()
        .find(|path| path.is_file())
    };
    json!({"input_path":absolute,"exists":absolute.exists(),"is_directory":absolute.is_dir(),
        "manifest_path":absolute.join("manifest.json").is_file().then(||absolute.join("manifest.json")),
        "verification_path":absolute.join("verification.json").is_file().then(||absolute.join("verification.json")),
        "execution_path":present("execution.json"),"outcome_path":present("outcome.json"),"assigned_job_path":present("assigned-job.json")})
}
fn shard_input_index(inputs: &[PathBuf]) -> Value {
    json!({"schema":"twist-tr04/shard-input-index/v1","inputs":inputs.iter().map(|path|input_entry(path)).collect::<Vec<_>>(),
        "scope":"Provided paths and existing metadata only. An expected or absent path is not evidence that a job attempted execution."})
}
mod vec_decimal {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub fn serialize<S: Serializer>(v: &[u128], s: S) -> Result<S::Ok, S::Error> {
        v.iter()
            .map(u128::to_string)
            .collect::<Vec<_>>()
            .serialize(s)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u128>, D::Error> {
        Vec::<String>::deserialize(d)?
            .into_iter()
            .map(|v| v.parse().map_err(serde::de::Error::custom))
            .collect()
    }
}
mod matrix_decimal {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub fn serialize<S: Serializer>(v: &[Vec<u128>], s: S) -> Result<S::Ok, S::Error> {
        v.iter()
            .map(|r| r.iter().map(u128::to_string).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .serialize(s)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<Vec<u128>>, D::Error> {
        Vec::<Vec<String>>::deserialize(d)?
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .map(|v| v.parse().map_err(serde::de::Error::custom))
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn primary_source_pin_binds_compiled_bytes_even_after_worktree_restoration() {
        let root = std::env::temp_dir().join(format!(
            "tr04-source-pin-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join("tools/twist-relations/src")).unwrap();
        let run = |args: &[&str]| {
            let result = Command::new("git")
                .arg("-C")
                .arg(&root)
                .args(args)
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
            result.stdout
        };
        run(&["init", "-q"]);
        let file = root.join("tools/twist-relations/src/core.rs");
        fs::write(&file, b"committed source\n").unwrap();
        run(&["add", "."]);
        run(&[
            "-c",
            "core.hooksPath=/dev/null",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "user.name=TR04 test",
            "-c",
            "user.email=tr04-test@example.invalid",
            "commit",
            "-qm",
            "source fixture",
        ]);
        let revision = String::from_utf8(run(&["rev-parse", "HEAD"]))
            .unwrap()
            .trim()
            .to_string();
        let mut pinned = source();
        pinned.compiled_source_sha256 =
            BTreeMap::from([("core.rs".into(), hash(b"committed source\n"))]);
        assert_eq!(tracked_clean(&root), Some(true));
        verify_source_blobs(&root, &pinned, &revision).unwrap();
        fs::write(&file, b"edited during compilation\n").unwrap();
        assert_eq!(tracked_clean(&root), Some(false));
        pinned
            .compiled_source_sha256
            .insert("core.rs".into(), hash(b"edited during compilation\n"));
        fs::write(&file, b"committed source\n").unwrap();
        assert_eq!(tracked_clean(&root), Some(true));
        assert!(verify_source_blobs(&root, &pinned, &revision).is_err());
        pinned.tracked_worktree_clean = None;
        assert!(validate_source_identity(&pinned, true).is_err());
        let not_repository = root.join("not-repository");
        fs::create_dir(&not_repository).unwrap();
        // A Git error must not be interpreted as an empty/clean status.
        assert_eq!(tracked_clean(&root.join("absent")), None);
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn selector_failure_retains_all_paid_work_and_zero_rank() {
        let work = Work {
            candidate_reads: 8,
            ..Work::default()
        };
        let selection = SelectionAudit {
            method: "twist_l4".into(),
            total_words: 5,
            outcomes: vec![tr04_select::Selected {
                base: Vec::new(),
                ways: 5,
                work_sum: work,
            }],
        };
        let combined = combine(13, 2, &selection, &BTreeMap::new()).unwrap();
        assert_eq!(combined.denominator, 5 * 12 * 256);
        assert_eq!(combined.selector_failure_words, 5);
        assert_eq!(combined.total_work_sum.candidate_reads, 8 * 12 * 256);
        for row in &combined.rank_hist_by_prefix {
            assert_eq!(row[0], combined.denominator);
        }
        for rank in &combined.milestones {
            assert_eq!(rank.missing, combined.denominator);
            assert_eq!(rank.work_sum, combined.total_work_sum);
        }
    }
    #[test]
    fn command_scope_and_artifact_paths_fail_closed() {
        assert!(cli(vec![
            "--prime".into(),
            "23".into(),
            "--output-dir".into(),
            "unused".into()
        ])
        .is_err());
        for path in ["/tmp/a", "../a", "a/../b", ""] {
            assert!(safe_path(path).is_err());
        }
        assert!(safe_path("cases/model.json").is_ok());
        assert!(cli(vec!["verify".into()]).is_err());
    }
}
