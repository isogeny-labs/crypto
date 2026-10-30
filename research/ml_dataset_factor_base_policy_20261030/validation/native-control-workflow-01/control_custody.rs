//! Custody preparation for copied negative controls; never a semantic verdict.
use super::*;

#[derive(Clone, Debug)]
pub(super) enum Recipe {
    ReorderSignedPoints,
    ReplaceSignedDomainDigest(String),
    ChangeConstructionTerminal,
    ChangePolicyOrigin,
    SignedSchemaOnLegacy,
    PromoteCappedPkm,
    RemoveAlternateDomain,
    RemoveCaptureBinding,
    RemoveExecutionContextAndFlag,
}
impl Recipe {
    fn slot(&self) -> u64 {
        match self {
            Self::ReorderSignedPoints => 1,
            Self::ReplaceSignedDomainDigest(_) => 2,
            Self::ChangeConstructionTerminal => 3,
            Self::ChangePolicyOrigin => 4,
            Self::SignedSchemaOnLegacy => 5,
            Self::PromoteCappedPkm => 7,
            Self::RemoveAlternateDomain => 8,
            Self::RemoveCaptureBinding => 9,
            Self::RemoveExecutionContextAndFlag => 10,
        }
    }
}
fn encoded(value: &Value) -> Result<Vec<u8>> {
    let bytes = serde_json::to_vec(value)?;
    require(
        bytes.len() as u64 <= MAX_JSON,
        "control JSON exceeds file cap",
    )?;
    Ok(bytes)
}
fn signed(value: &Value) -> Result<()> {
    require(
        value["schema"] == "ml-ecdlp-factor-base/v2",
        "control prerequisite: signed base required",
    )
}
fn prepare(
    cell: &Path,
    manifest: &Value,
    recipe: &Recipe,
) -> Result<BTreeMap<String, Option<Vec<u8>>>> {
    let mut edits = BTreeMap::new();
    let mut put = |name: &str, value: Value| -> Result<()> {
        edits.insert(name.to_owned(), Some(encoded(&value)?));
        Ok(())
    };
    match recipe {
        Recipe::ReorderSignedPoints => {
            let mut base = json_file(cell, "factor-base.json")?;
            signed(&base)?;
            let points = base["points"]
                .as_array_mut()
                .ok_or("control prerequisite: points")?;
            require(
                points.len() >= 2,
                "control prerequisite: two points required",
            )?;
            points.swap(0, 1);
            put("factor-base.json", base)?;
        }
        Recipe::ReplaceSignedDomainDigest(replacement) => {
            let mut base = json_file(cell, "factor-base.json")?;
            signed(&base)?;
            require(
                replacement.len() == 64
                    && replacement
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
                "control replacement digest is not canonical",
            )?;
            require(
                base["domain_digest"].is_string() && base["domain_digest"] != *replacement,
                "control prerequisite: distinct nonempty domain digest required",
            )?;
            base["domain_digest"] = json!(replacement);
            put("factor-base.json", base)?;
        }
        Recipe::ChangeConstructionTerminal => {
            let mut witness = json_file(cell, "witness.json")?;
            let mut outcome = json_file(cell, "result.json")?;
            let history = witness
                .get_mut("factor_base_policy_construction")
                .and_then(Value::as_object_mut)
                .ok_or("control prerequisite: constructor history")?;
            let old = history
                .get("terminal_status")
                .and_then(Value::as_str)
                .ok_or("control prerequisite: constructor terminal")?;
            let terminal = if old == "requested_size_reached" {
                "candidate_cap"
            } else {
                "requested_size_reached"
            };
            require(
                outcome["factor_base_construction_status"] == old,
                "control prerequisite: source terminals disagree",
            )?;
            history.insert("terminal_status".into(), json!(terminal));
            outcome["factor_base_construction_status"] = json!(terminal);
            put("witness.json", witness)?;
            put("result.json", outcome)?;
        }
        Recipe::ChangePolicyOrigin => {
            let mut policy = json_file(cell, "case-policy.json")?;
            let index = number(&policy, "policy_index")?;
            require(
                index <= 1,
                "control prerequisite: expected two-policy population",
            )?;
            policy["policy_index"] = json!(index ^ 1);
            put("case-policy.json", policy)?;
        }
        Recipe::SignedSchemaOnLegacy => {
            let mut base = json_file(cell, "factor-base.json")?;
            require(
                base["schema"] == "ml-ecdlp-factor-base/v1",
                "control prerequisite: legacy base required",
            )?;
            base["schema"] = json!("ml-ecdlp-factor-base/v2");
            put("factor-base.json", base)?;
        }
        Recipe::PromoteCappedPkm => {
            require(
                manifest["config"]["solver"] == "pkm",
                "control prerequisite: PKM required",
            )?;
            let mut outcome = json_file(cell, "result.json")?;
            require(
                outcome["native_alternative_domain_admission"] == "preflight_budget_exhausted",
                "control prerequisite: capped PKM required",
            )?;
            outcome["native_alternative_domain_admission"] = json!("eligible");
            outcome["status"] = json!("verified");
            for key in ["solver_completed", "exhaustive_no_decomposition"] {
                outcome[key] = json!(true);
            }
            put("result.json", outcome)?;
        }
        Recipe::RemoveAlternateDomain => {
            let rows = manifest["artifacts"]
                .as_array()
                .ok_or("artifact array missing")?;
            let row = rows
                .iter()
                .find(|row| {
                    row["role"] == "explicit-alternative-kummer-factor-base"
                        || row["role"] == "exact-large-prime-discovery-domains"
                })
                .ok_or("control prerequisite: retained alternate domain required")?;
            let name = field(row, "path")?;
            checked_file(cell, name)?;
            edits.insert(name.to_owned(), None);
        }
        Recipe::RemoveCaptureBinding => {
            for name in ["capture-binding.json", "capture-limits.json"] {
                checked_file(cell, name)?;
                edits.insert(name.into(), None);
            }
        }
        Recipe::RemoveExecutionContextAndFlag => {
            let mut run = json_file(cell, "run.json")?;
            run.as_object_mut()
                .ok_or("control prerequisite: run object")?
                .remove("execution_context_required");
            put("run.json", run)?;
            checked_file(cell, "execution-context.json")?;
            edits.insert("execution-context.json".into(), None);
        }
    }
    Ok(edits)
}
fn rewrite_rows(rows: &mut Vec<Value>, edits: &BTreeMap<String, Option<Vec<u8>>>) -> Result<()> {
    let mut next = Vec::new();
    for row in rows.iter() {
        let name = field(row, "path")?;
        match edits.get(name) {
            Some(None) => {}
            Some(Some(bytes)) => {
                let mut changed = row.clone();
                changed["sha256"] = json!(digest(bytes));
                changed["bytes"] = json!(bytes.len());
                next.push(changed);
            }
            None => next.push(row.clone()),
        }
    }
    *rows = next;
    Ok(())
}

/// Caller must supply a mathematically admitted source and an exclusive copied
/// cell. This function proves only unchanged original and resealed byte custody.
pub(super) fn apply(original: &Path, copied: &Path, recipe: Recipe) -> Result<Value> {
    for directory in [original, copied] {
        require(
            fs::symlink_metadata(directory)?.file_type().is_dir(),
            "control source/copy is not an ordinary directory",
        )?;
    }
    let original_path = fs::canonicalize(original)?;
    let copied_path = fs::canonicalize(copied)?;
    require(
        !original_path.starts_with(&copied_path) && !copied_path.starts_with(&original_path),
        "control must use a separate copied cell",
    )?;
    let before_original = snapshot(original)?;
    require(
        snapshot(copied)? == before_original,
        "control copy differs before mutation",
    )?;
    let manifest_path = checked_file(copied, "manifest.json")?;
    let before_manifest_sha256 = hash(&manifest_path)?;
    let mut manifest = json_file(copied, "manifest.json")?;
    require(
        matches!(
            manifest["schema"].as_str(),
            Some("crypto.ml-dataset.cell/v1" | "crypto.ml-dataset.cell/v2")
        ),
        "control cell schema",
    )?;
    let original_rows = artifact_rows(copied, &manifest)?;
    require(
        !original_rows
            .iter()
            .any(|row| row["path"] == "manifest.json"),
        "self-referential manifest",
    )?;
    if original_rows.iter().any(|row| row["path"] == "result.json") {
        require(
            json_file(copied, "result.json")? == manifest["outcome"],
            "source result/outcome mismatch",
        )?;
    }
    let mut registry = if copied.join("worker-artifacts.json").exists() {
        require(
            original_rows.iter().any(|row| {
                row["path"] == "worker-artifacts.json"
                    && row["role"] == "artifact-registry"
                    && row["encoding"] == "json"
            }),
            "worker registry lacks outer binding",
        )?;
        let value = json_file(copied, "worker-artifacts.json")?;
        let rows = artifact_rows(copied, &json!({"artifacts":value}))?;
        require(
            !rows.iter().any(|row| {
                row["path"] == "worker-artifacts.json" || row["path"] == "manifest.json"
            }),
            "self-referential worker registry",
        )?;
        Some(rows)
    } else {
        None
    };
    let mut edits = prepare(copied, &manifest, &recipe)?;
    for path in edits.keys() {
        require(
            original_rows.iter().any(|row| row["path"] == *path),
            "control prerequisite: mutation lacks original manifest entry",
        )?;
        checked_file(copied, path)?;
    }
    let changed_paths: Vec<String> = edits.keys().cloned().collect();
    if let Some(Some(bytes)) = edits.get("result.json") {
        manifest["outcome"] = unique(bytes, 0)?;
    }
    if let Some(rows) = &mut registry {
        rewrite_rows(rows, &edits)?;
        edits.insert("worker-artifacts.json".into(), Some(encoded(&json!(rows))?));
    }
    let rows = manifest["artifacts"]
        .as_array_mut()
        .ok_or("artifact array missing")?;
    rewrite_rows(rows, &edits)?;
    edits.insert("manifest.json".into(), Some(encoded(&manifest)?));
    let total = edits
        .values()
        .filter_map(Option::as_ref)
        .try_fold(0u64, |n, bytes| {
            n.checked_add(bytes.len() as u64)
                .ok_or("control replacement overflow")
        })?;
    require(
        total <= 128 * 1024 * 1024,
        "control aggregate replacement cap",
    )?;
    // Directory separation does not exclude file-level aliases. Reject every
    // mutation target before any write, including the two custody documents.
    for name in edits.keys() {
        use std::os::unix::fs::MetadataExt;
        let path = checked_file(copied, name)?;
        require(
            fs::metadata(path)?.nlink() == 1,
            "control copy contains a hard-linked mutation target",
        )?;
    }
    // Everything above is read-only admission. A later I/O failure is retained
    // as a failed preparation; partial writes never certify a negative control.
    for (name, bytes) in &edits {
        let path = checked_file(copied, name)?;
        if let Some(bytes) = bytes {
            let mut file = OpenOptions::new().write(true).truncate(true).open(&path)?;
            file.write_all(bytes)?;
            file.sync_all()?;
        } else {
            fs::remove_file(path)?;
        }
    }
    artifact_rows(copied, &json_file(copied, "manifest.json")?)?;
    if registry.is_some() {
        artifact_rows(
            copied,
            &json!({"artifacts":json_file(copied, "worker-artifacts.json")?}),
        )?;
    }
    require(
        snapshot(original)? == before_original,
        "original changed while preparing copied control",
    )?;
    Ok(
        json!({"schema":"ml-dataset.copied-control-custody/v1","slot":recipe.slot(),
        "before_manifest_sha256":before_manifest_sha256,"after_manifest_sha256":hash(&manifest_path)?,
        "mutated_source_paths":changed_paths,"resealed_paths":edits.keys().collect::<Vec<_>>(),
        "original_unchanged":true,"outer_and_worker_hashes_verified":true,
        "semantic_rejection_verified":false,"cli_invoked":false,
        "requires_root_run_flag_removal":matches!(recipe, Recipe::RemoveExecutionContextAndFlag)}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "copied-custody-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&root).unwrap();
            fs::create_dir(root.join("original")).unwrap();
            let f = Self(root);
            // Custody fixtures only: these are not mathematically admitted curves.
            for (name, value) in [
                (
                    "factor-base.json",
                    json!({"schema":"ml-ecdlp-factor-base/v2","points":[1,2],"domain_digest":"a".repeat(64)}),
                ),
                (
                    "witness.json",
                    json!({"factor_base_policy_construction":{"terminal_status":"requested_size_reached"}}),
                ),
                (
                    "result.json",
                    json!({"factor_base_construction_status":"requested_size_reached","native_alternative_domain_admission":"preflight_budget_exhausted","status":"budget_exhausted","solver_completed":false}),
                ),
                ("case-policy.json", json!({"policy_index":0})),
                ("capture-binding.json", json!({})),
                ("capture-limits.json", json!({})),
                ("execution-context.json", json!({})),
                ("run.json", json!({"execution_context_required":true})),
                ("alternate.json", json!({"purpose":"custody fixture only"})),
                ("unrelated.json", json!({"unchanged":7})),
            ] {
                fs::write(f.original().join(name), serde_json::to_vec(&value).unwrap()).unwrap();
            }
            f.seal();
            f
        }
        fn original(&self) -> PathBuf {
            self.0.join("original")
        }
        fn copy(&self) -> PathBuf {
            let path = self.0.join("copy");
            copy_tree(&self.original(), &path).unwrap();
            path
        }
        fn row(&self, name: &str, role: &str) -> Value {
            let path = self.original().join(name);
            json!({"path":name,"role":role,"encoding":"json","sha256":hash(&path).unwrap(),"bytes":fs::metadata(path).unwrap().len()})
        }
        fn seal(&self) {
            let mut rows = Vec::new();
            for path in entries(&self.original()).unwrap() {
                let name = path.file_name().unwrap().to_str().unwrap();
                if ["manifest.json", "worker-artifacts.json"].contains(&name) {
                    continue;
                }
                rows.push(self.row(
                    name,
                    if name == "alternate.json" {
                        "exact-large-prime-discovery-domains"
                    } else {
                        "software-fixture"
                    },
                ));
            }
            fs::write(
                self.original().join("worker-artifacts.json"),
                serde_json::to_vec(&rows).unwrap(),
            )
            .unwrap();
            self.seal_outer(rows);
        }
        fn seal_outer(&self, mut rows: Vec<Value>) {
            rows.push(self.row("worker-artifacts.json", "artifact-registry"));
            let manifest = json!({"schema":"crypto.ml-dataset.cell/v2","config":{"solver":"pkm"},
                "outcome":json_file(&self.original(),"result.json").unwrap(),"artifacts":rows});
            fs::write(
                self.original().join("manifest.json"),
                serde_json::to_vec(&manifest).unwrap(),
            )
            .unwrap();
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn independent_custody_check(cell: &Path) {
        use crypto_lib::ml_dataset::{storage, types::Artifact};
        let manifest = json_file(cell, "manifest.json").unwrap();
        let outer: Vec<Artifact> = serde_json::from_value(manifest["artifacts"].clone()).unwrap();
        storage::verify_manifest(cell, &outer).unwrap();
        let inner: Vec<Artifact> =
            serde_json::from_value(json_file(cell, "worker-artifacts.json").unwrap()).unwrap();
        storage::verify_manifest(cell, &inner).unwrap();
    }

    #[test]
    fn every_cell_recipe_reseals_both_levels_and_preserves_original_and_unrelated_bytes() {
        let recipes = [
            Recipe::ReorderSignedPoints,
            Recipe::ReplaceSignedDomainDigest("b".repeat(64)),
            Recipe::ChangeConstructionTerminal,
            Recipe::ChangePolicyOrigin,
            Recipe::SignedSchemaOnLegacy,
            Recipe::PromoteCappedPkm,
            Recipe::RemoveAlternateDomain,
            Recipe::RemoveCaptureBinding,
            Recipe::RemoveExecutionContextAndFlag,
        ];
        for recipe in recipes {
            let f = Fixture::new();
            if matches!(recipe, Recipe::SignedSchemaOnLegacy) {
                let mut base = json_file(&f.original(), "factor-base.json").unwrap();
                base["schema"] = json!("ml-ecdlp-factor-base/v1");
                fs::write(
                    f.original().join("factor-base.json"),
                    serde_json::to_vec(&base).unwrap(),
                )
                .unwrap();
                f.seal();
            }
            let original = snapshot(&f.original()).unwrap();
            let copied = f.copy();
            let unrelated = fs::read(copied.join("unrelated.json")).unwrap();
            let slot = recipe.slot();
            let receipt = apply(&f.original(), &copied, recipe).unwrap();
            independent_custody_check(&copied);
            assert_eq!(snapshot(&f.original()).unwrap(), original);
            assert_eq!(fs::read(copied.join("unrelated.json")).unwrap(), unrelated);
            assert_eq!(receipt["slot"], slot);
            assert_eq!(receipt["semantic_rejection_verified"], false);
            assert_eq!(receipt["cli_invoked"], false);
            assert_ne!(
                receipt["before_manifest_sha256"],
                receipt["after_manifest_sha256"]
            );
            let manifest = json_file(&copied, "manifest.json").unwrap();
            assert_eq!(
                manifest["outcome"],
                json_file(&copied, "result.json").unwrap()
            );
            let removed = match slot {
                8 => Some("alternate.json"),
                9 => Some("capture-binding.json"),
                10 => Some("execution-context.json"),
                _ => None,
            };
            if let Some(name) = removed {
                assert!(!copied.join(name).exists());
                assert!(!manifest["artifacts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|row| row["path"] == name));
                let registry = json_file(&copied, "worker-artifacts.json").unwrap();
                assert!(!registry
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|row| row["path"] == name));
            }
            if slot == 3 {
                assert_eq!(
                    json_file(&copied, "witness.json").unwrap()["factor_base_policy_construction"]
                        ["terminal_status"],
                    manifest["outcome"]["factor_base_construction_status"]
                );
            }
            if slot == 9 {
                assert!(!copied.join("capture-limits.json").exists());
                assert!(!manifest["artifacts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|row| row["path"] == "capture-limits.json"));
                assert!(!json_file(&copied, "worker-artifacts.json")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|row| row["path"] == "capture-limits.json"));
            }
            if slot == 7 {
                assert_eq!(
                    manifest["outcome"]["pdp_witness_verified"],
                    json_file(&f.original(), "result.json").unwrap()["pdp_witness_verified"]
                );
            }
        }
    }

    #[test]
    fn one_point_and_invalid_digest_are_missing_prerequisites_without_writes() {
        let f = Fixture::new();
        let mut base = json_file(&f.original(), "factor-base.json").unwrap();
        base["points"] = json!([1]);
        fs::write(
            f.original().join("factor-base.json"),
            serde_json::to_vec(&base).unwrap(),
        )
        .unwrap();
        f.seal();
        let copy = f.copy();
        let before = snapshot(&copy).unwrap();
        assert!(apply(&f.original(), &copy, Recipe::ReorderSignedPoints)
            .unwrap_err()
            .to_string()
            .contains("two points"));
        assert_eq!(snapshot(&copy).unwrap(), before);
        assert!(apply(
            &f.original(),
            &copy,
            Recipe::ReplaceSignedDomainDigest("../unrelated".into())
        )
        .is_err());
        assert_eq!(snapshot(&copy).unwrap(), before);
    }

    #[test]
    fn stale_outer_hash_is_rejected_before_any_resealing() {
        let f = Fixture::new();
        fs::write(f.original().join("unrelated.json"), b"{}").unwrap();
        let copy = f.copy();
        let before = snapshot(&copy).unwrap();
        assert!(apply(&f.original(), &copy, Recipe::ReorderSignedPoints)
            .unwrap_err()
            .to_string()
            .contains("digest/size mismatch"));
        assert_eq!(snapshot(&copy).unwrap(), before);
    }

    #[test]
    fn duplicate_worker_registry_is_rejected_even_with_valid_outer_hash() {
        let f = Fixture::new();
        let mut rows = json_file(&f.original(), "worker-artifacts.json")
            .unwrap()
            .as_array()
            .unwrap()
            .clone();
        rows.push(rows[0].clone());
        fs::write(
            f.original().join("worker-artifacts.json"),
            serde_json::to_vec(&rows).unwrap(),
        )
        .unwrap();
        let outer = json_file(&f.original(), "manifest.json").unwrap()["artifacts"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| row["path"] != "worker-artifacts.json")
            .cloned()
            .collect();
        f.seal_outer(outer);
        let copy = f.copy();
        let before = snapshot(&copy).unwrap();
        assert!(apply(&f.original(), &copy, Recipe::ReorderSignedPoints)
            .unwrap_err()
            .to_string()
            .contains("duplicate artifact"));
        assert_eq!(snapshot(&copy).unwrap(), before);
    }

    #[test]
    fn hard_linked_mutation_is_rejected_before_original_or_copy_changes() {
        let f = Fixture::new();
        let copy = f.copy();
        let target = copy.join("factor-base.json");
        fs::remove_file(&target).unwrap();
        fs::hard_link(f.original().join("factor-base.json"), &target).unwrap();
        let original_before = snapshot(&f.original()).unwrap();
        let copy_before = snapshot(&copy).unwrap();
        assert_eq!(original_before, copy_before);
        assert!(apply(&f.original(), &copy, Recipe::ReorderSignedPoints)
            .unwrap_err()
            .to_string()
            .contains("hard-linked mutation target"));
        assert_eq!(snapshot(&f.original()).unwrap(), original_before);
        assert_eq!(snapshot(&copy).unwrap(), copy_before);
    }

    #[test]
    fn same_or_nested_source_and_symlink_copy_are_rejected() {
        let f = Fixture::new();
        assert!(apply(&f.original(), &f.original(), Recipe::ReorderSignedPoints).is_err());
        let nested = f.original().join("nested");
        fs::create_dir(&nested).unwrap();
        assert!(apply(&f.original(), &nested, Recipe::ReorderSignedPoints).is_err());
        fs::remove_dir(nested).unwrap();
        let alias = f.0.join("alias");
        std::os::unix::fs::symlink(f.original(), &alias).unwrap();
        assert!(apply(&f.original(), &alias, Recipe::ReorderSignedPoints).is_err());
    }
}
