//! Fail-closed admission for the supported provenance-only post-data correction.
use super::*;

const SCHEMA: &str = "pencil-analysis-provenance-erratum/v1";
const PREFIX: &str = "research/pencil_large_prime_20261029/";
const REVIEWED_PARENT_FILE_SHA256: &str =
    "7d9f3a299c8d2e39f0a7b69aaab34a922dcc192102ab8d0ee1971ab6e635efee";
const REVIEWED_PARENT_PROTOCOL_SHA256: &str =
    "cf8f76eadc42ff487644ea13b3392af1fd31c94e4629eecd6ad7b9a18820badb";
const REVIEWED_SOURCE_MANIFEST_SHA256: &str =
    "9618895ede7542e314167822e25ad1ecaf1bdc9da6da84cceb2499ec8a1acb24";
const REVIEWED_BINARY_MANIFEST_SHA256: &str =
    "581f4f3f103f3c69af738189f7811eda12d7ae0653fc87e4fe2d64f3288e5742";
const REVIEWED_RECEIPT_SHA256: &str =
    "07e7359d4adfe9b8ab74ba19b313c534f9962f756209ea9059b9f65ac0fb3523";
pub(super) const FIELDS: [&str; 4] = [
    "/extensions/analysis_build/receipt",
    "/extensions/analysis_build/binaries/*/bytes",
    "/extensions/finite_cost_model/analyzer_binaries_sha256",
    "/extensions/analysis_support_sha256",
];

#[derive(Clone)]
pub(super) struct Admitted {
    pub amendment_id: String,
    pub parent_protocol_sha256: String,
    pub attachments: Vec<(PathBuf, &'static str)>,
}

#[cfg(test)]
type ReviewedTuple = (String, String, String, String, String);
#[cfg(test)]
static TEST_REVIEWED: std::sync::OnceLock<std::sync::Mutex<BTreeSet<ReviewedTuple>>> =
    std::sync::OnceLock::new();
#[cfg(test)]
pub(super) fn register_test_reviewed(values: ReviewedTuple) {
    TEST_REVIEWED
        .get_or_init(Default::default)
        .lock()
        .unwrap()
        .insert(values);
}

fn exact_reviewed_binding(ext: &V) -> Result<()> {
    let values = (
        s(&ext["parent_protocol"]["sha256"])?,
        s(&ext["parent_protocol"]["protocol_sha256"])?,
        s(&ext["predata_source_manifest"]["sha256"])?,
        s(&ext["predata_binary_manifest"]["sha256"])?,
        s(&ext["reconciled_build_receipt"]["sha256"])?,
    );
    #[cfg(test)]
    if TEST_REVIEWED
        .get_or_init(Default::default)
        .lock()
        .unwrap()
        .contains(&(
            values.0.to_owned(),
            values.1.to_owned(),
            values.2.to_owned(),
            values.3.to_owned(),
            values.4.to_owned(),
        ))
    {
        return Ok(());
    }
    check(
        values
            == (
                REVIEWED_PARENT_FILE_SHA256,
                REVIEWED_PARENT_PROTOCOL_SHA256,
                REVIEWED_SOURCE_MANIFEST_SHA256,
                REVIEWED_BINARY_MANIFEST_SHA256,
                REVIEWED_RECEIPT_SHA256,
            ),
        "erratum evidence is not the exact independently reviewed binding",
    )
}

pub(super) fn prepare(
    parent_path: &Path,
    source_manifest: &Path,
    binary_manifest: &Path,
    receipt_path: &Path,
) -> Result<V> {
    let parent_path = fs::canonicalize(parent_path).map_err(|e| e.to_string())?;
    let source_manifest = fs::canonicalize(source_manifest).map_err(|e| e.to_string())?;
    let binary_manifest = fs::canonicalize(binary_manifest).map_err(|e| e.to_string())?;
    let receipt_path = fs::canonicalize(receipt_path).map_err(|e| e.to_string())?;
    let parent = load(&parent_path)?;
    let receipt = load(&receipt_path)?;
    let mut child = parent.clone();
    child["version"] = json!(n(&parent["version"])?
        .checked_add(1)
        .ok_or("version overflow")?);
    let mut normalized = serde_json::Map::new();
    for (name, sha) in parent["extensions"]["analysis_support_sha256"]
        .as_object()
        .ok_or("parent analysis source pins absent")?
    {
        normalized.insert(
            name.strip_prefix(PREFIX)
                .ok_or("parent analysis source key lacks reviewed prefix")?
                .to_owned(),
            sha.clone(),
        );
    }
    child["extensions"]["analysis_support_sha256"] = V::Object(normalized);
    child["extensions"]["analysis_build"]["receipt"] = json!(receipt_path);
    child["extensions"]["analysis_build"]["receipt_sha256"] = json!(digest(&receipt_path)?);
    for binary in arr(&receipt["binaries"])? {
        let name = s(&binary["name"])?;
        check(
            matches!(name, "analyze_pair" | "analyze_cohort"),
            "unexpected reconciled binary",
        )?;
        child["extensions"]["analysis_build"]["binaries"][name]["bytes"] = binary["bytes"].clone();
        child["extensions"]["finite_cost_model"]["analyzer_binaries_sha256"][name] =
            binary["sha256"].clone();
    }
    child["amendments"]
        .as_array_mut()
        .ok_or("amendments array absent")?
        .push(json!({
            "id":"A-analysis-provenance-erratum-20261030",
            "filed":"2026-10-30",
            "filed_before_affected_data":false,
            "reason":"Truthful post-data correction of four independently reviewed analysis provenance metadata defects only; frozen parent, observations and scientific design remain unchanged.",
            "changes":["Bind the exact pre-data source and binary manifests and reconciled native analysis build receipt, including binary byte sizes and normalized source keys."],
            "supersedes_fields":FIELDS
        }));
    child["extensions"]["analysis_provenance_erratum"] = json!({
        "schema":SCHEMA,
        "amendment_id":"A-analysis-provenance-erratum-20261030",
        "filed_before_affected_data":false,
        "adapter_sha256":adapter_hash(),
        "parent_protocol":{"path":parent_path,"sha256":digest(&parent_path)?,"protocol_sha256":parent["freeze"]["protocol_sha256"]},
        "predata_source_manifest":{"path":source_manifest,"sha256":digest(&source_manifest)?},
        "predata_binary_manifest":{"path":binary_manifest,"sha256":digest(&binary_manifest)?},
        "reconciled_build_receipt":{"path":receipt_path,"sha256":digest(&receipt_path)?}
    });
    child["freeze"]["frozen_at"] =
        json!(chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
    child["freeze"]["filed_before_affected_data"] = json!(false);
    child["freeze"]["protocol_sha256"] = json!(protocol_hash(&child)?);
    admit(&child)?.ok_or("prepared erratum did not enter reviewed admission path")?;
    Ok(child)
}

fn manifest(path: &Path) -> Result<BTreeMap<String, String>> {
    let mut out = BTreeMap::new();
    for (i, line) in fs::read_to_string(path)
        .map_err(|e| e.to_string())?
        .lines()
        .enumerate()
    {
        if line.trim().is_empty() {
            continue;
        }
        let (sha, name) = line
            .split_once("  ")
            .ok_or_else(|| format!("malformed SHA256SUMS line {}", i + 1))?;
        check(
            sha.len() == 64 && sha.bytes().all(|b| b.is_ascii_hexdigit()),
            "malformed SHA256SUMS digest",
        )?;
        check(!name.is_empty(), "empty SHA256SUMS path")?;
        check(
            out.insert(name.to_owned(), sha.to_ascii_lowercase())
                .is_none(),
            "duplicate SHA256SUMS path",
        )?;
    }
    Ok(out)
}

fn exact_object(v: &V, fields: &[&str]) -> Result<()> {
    let o = v.as_object().ok_or("erratum extension object absent")?;
    check(
        o.keys().map(String::as_str).collect::<BTreeSet<_>>() == fields.iter().copied().collect(),
        "unsupported erratum extension field",
    )
}

pub(super) fn admit(child: &V) -> Result<Option<Admitted>> {
    let amendments = arr(&child["amendments"])?;
    let false_ams: Vec<_> = amendments
        .iter()
        .filter(|a| a["filed_before_affected_data"] == false)
        .collect();
    if false_ams.is_empty() {
        return Ok(None);
    }
    check(
        false_ams.len() == 1,
        "multiple post-data amendments unsupported",
    )?;
    let ext = &child["extensions"]["analysis_provenance_erratum"];
    exact_object(
        ext,
        &[
            "schema",
            "amendment_id",
            "filed_before_affected_data",
            "adapter_sha256",
            "parent_protocol",
            "predata_source_manifest",
            "predata_binary_manifest",
            "reconciled_build_receipt",
        ],
    )?;
    check(
        ext["schema"] == SCHEMA,
        "unsupported provenance erratum schema",
    )?;
    check(
        ext["filed_before_affected_data"] == false
            && child["freeze"]["filed_before_affected_data"] == false,
        "post-data erratum timing must remain false",
    )?;
    check(
        ext["adapter_sha256"] == adapter_hash(),
        "post-data erratum adapter not pinned",
    )?;
    exact_reviewed_binding(ext)?;
    let amendment = false_ams[0];
    check(
        amendment["id"] == ext["amendment_id"],
        "post-data amendment identity mismatch",
    )?;
    let actual_fields: BTreeSet<_> = arr(&amendment["supersedes_fields"])?
        .iter()
        .map(s)
        .collect::<Result<_>>()?;
    check(
        actual_fields == FIELDS.into_iter().collect(),
        "post-data amendment field allowlist mismatch",
    )?;

    let parent_path = bound(&ext["parent_protocol"])?;
    let parent = load(&parent_path)?;
    super::basic_protocol_admission(&parent, false)?;
    check(
        ext["parent_protocol"]["protocol_sha256"] == parent["freeze"]["protocol_sha256"],
        "parent internal protocol identity mismatch",
    )?;
    check(
        n(&child["version"])?
            == n(&parent["version"])?
                .checked_add(1)
                .ok_or("version overflow")?,
        "erratum version must increment frozen parent",
    )?;
    let parent_ams = arr(&parent["amendments"])?;
    check(
        amendments.len() == parent_ams.len() + 1
            && &amendments[..parent_ams.len()] == parent_ams
            && amendments.last() == Some(amendment),
        "erratum must append exactly one amendment",
    )?;

    let source_manifest_path = bound(&ext["predata_source_manifest"])?;
    let binary_manifest_path = bound(&ext["predata_binary_manifest"])?;
    let receipt_path = bound(&ext["reconciled_build_receipt"])?;
    let source_manifest = manifest(&source_manifest_path)?;
    let binary_manifest = manifest(&binary_manifest_path)?;
    let receipt = load(&receipt_path)?;
    check(
        receipt["schema"] == "pencil-native-analysis-build/v1"
            && receipt["source_files_match_commit"] == true
            && receipt["build_exit_code"] == 0
            && receipt["source_revision"]
                == parent["extensions"]["analysis_build"]["source_commit"]
            && receipt["mapping_sha256"]
                == parent["extensions"]["finite_cost_model"]["mapping_sha256"],
        "reconciled build receipt identity invalid",
    )?;

    let parent_sources = parent["extensions"]["analysis_support_sha256"]
        .as_object()
        .ok_or("parent analysis source pins absent")?;
    check(
        parent_sources.len() == 13,
        "expected exact 13 pre-data analysis sources",
    )?;
    let mut normalized = serde_json::Map::new();
    for (full, sha) in parent_sources {
        let short = full
            .strip_prefix(PREFIX)
            .ok_or("parent analysis source key lacks reviewed prefix")?;
        check(
            source_manifest.get(full).map(String::as_str) == sha.as_str(),
            "pre-data source manifest differs from frozen parent",
        )?;
        normalized.insert(short.to_owned(), sha.clone());
    }
    check(
        source_manifest.len() == parent_sources.len(),
        "pre-data source manifest has unexpected entries",
    )?;
    let receipt_source_rows = arr(&receipt["sources"])?;
    check(
        receipt_source_rows.len() == 13,
        "reconciled receipt source inventory differs",
    )?;
    let receipt_sources: BTreeMap<_, _> = receipt_source_rows
        .iter()
        .map(|x| Ok((s(&x["path"])?.to_owned(), s(&x["sha256"])?.to_owned())))
        .collect::<Result<_>>()?;
    check(
        receipt_sources.len() == 13
            && normalized
                .iter()
                .all(|(k, v)| receipt_sources.get(k).map(String::as_str) == v.as_str()),
        "reconciled receipt source set differs",
    )?;

    let mut expected = parent.clone();
    expected["version"] = child["version"].clone();
    expected["freeze"] = child["freeze"].clone();
    expected["amendments"] = child["amendments"].clone();
    expected["extensions"]["analysis_provenance_erratum"] = ext.clone();
    expected["extensions"]["analysis_support_sha256"] = V::Object(normalized);
    expected["extensions"]["analysis_build"]["receipt"] =
        ext["reconciled_build_receipt"]["path"].clone();
    expected["extensions"]["analysis_build"]["receipt_sha256"] =
        ext["reconciled_build_receipt"]["sha256"].clone();
    check(
        arr(&receipt["binaries"])?.len() == 2,
        "reconciled receipt binary inventory differs",
    )?;
    for name in ["analyze_pair", "analyze_cohort"] {
        let entries: Vec<_> = arr(&receipt["binaries"])?
            .iter()
            .filter(|x| x["name"] == name)
            .collect();
        check(entries.len() == 1, "reconciled receipt binary set invalid")?;
        let rb = entries[0];
        let parent_bin = &parent["extensions"]["analysis_build"]["binaries"][name];
        check(
            rb["sha256"] == parent_bin["sha256"],
            "reconciled binary differs from frozen pre-data pin",
        )?;
        let rb_sha = s(&rb["sha256"])?;
        check(
            binary_manifest
                .iter()
                .filter(|(path, sha)| path.ends_with(name) && sha.as_str() == rb_sha)
                .count()
                == 1,
            "pre-data binary manifest mismatch",
        )?;
        expected["extensions"]["analysis_build"]["binaries"][name]["bytes"] = rb["bytes"].clone();
        expected["extensions"]["finite_cost_model"]["analyzer_binaries_sha256"][name] =
            rb["sha256"].clone();
    }
    check(
        expected == *child,
        "post-data erratum changes fields outside reviewed provenance allowlist",
    )?;
    Ok(Some(Admitted {
        amendment_id: s(&amendment["id"])?.to_owned(),
        parent_protocol_sha256: s(&parent["freeze"]["protocol_sha256"])?.to_owned(),
        attachments: vec![
            (parent_path, "protocol"),
            (source_manifest_path, "receipt"),
            (binary_manifest_path, "receipt"),
            (receipt_path, "receipt"),
        ],
    }))
}
