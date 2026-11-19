//! Versioned feature definitions, saved research sweeps and ML dataset recipes.
//!
//! These inputs describe intended coverage and feature semantics. They do not
//! create curve facts, freeze membership or establish measurement evidence.
//! Changed semantics require a version bump; the importer reports drift under
//! an existing identity.

use super::{s, Ctx, Report};
use crate::math::{canonical_json, sha256_hex};
use anyhow::{anyhow, bail, Result};
use serde_json::{json, Value};

pub const FEATURE_FILE: &str = "ecdb/features/catalog.json";
pub const DATASET_FILE: &str = "ecdb/datasets/model-ready.json";
pub const SWEEP_FILES: &[&str] = &[
    "ecdb/sweeps/koblitz-ladder.json",
    "ecdb/sweeps/binary-ladder.json",
    "ecdb/sweeps/prime-ladder.json",
    "ecdb/sweeps/cm-isogeny-strata.json",
];

fn field<'a>(v: &'a Value, key: &str) -> Result<&'a Value> {
    v.get(key).ok_or_else(|| anyhow!("record without {key}"))
}

fn text(v: &Value, key: &str) -> Result<String> {
    field(v, key)?.as_str().map(str::to_owned).ok_or_else(|| anyhow!("{key} is not a string"))
}

fn integer(v: &Value, key: &str) -> Result<i32> {
    let n = field(v, key)?.as_i64().ok_or_else(|| anyhow!("{key} is not an integer"))?;
    i32::try_from(n).map_err(|_| anyhow!("{key} is outside int32"))
}

fn strings(v: &Value, key: &str) -> Result<Vec<String>> {
    field(v, key)?
        .as_array()
        .ok_or_else(|| anyhow!("{key} is not an array"))?
        .iter()
        .map(|x| x.as_str().map(str::to_owned).ok_or_else(|| anyhow!("{key} contains a non-string")))
        .collect()
}

async fn feature_catalog(ctx: &Ctx<'_>) -> Result<Report> {
    let input = ctx.read(FEATURE_FILE)?;
    let mut report = Report::new(&input);
    if input.doc.get("schema").and_then(Value::as_str) != Some("ecdb.feature_catalog/v1") {
        bail!("{} has the wrong schema", input.rel_path);
    }
    let source = ctx.source(&input.rel_path, &input.sha256, "ecdb ML feature catalogue").await?;
    let provenance = ctx
        .provenance(
            source,
            "import versioned ML feature definitions",
            Some(&json!({"schema":"ecdb.feature_catalog/v1"})),
        )
        .await?;
    let features = field(&input.doc, "features")?.as_array().ok_or_else(|| anyhow!("features is not an array"))?;
    for feature in features {
        let key = text(feature, "key")?;
        let version = integer(feature, "version")?;
        let definition_sha = sha256_hex(&canonical_json(feature)?);
        let scope = text(feature, "scope")?;
        let data_type = text(feature, "data_type")?;
        let nullable = feature.get("nullable").and_then(Value::as_bool).unwrap_or(true);
        let description = text(feature, "description")?;
        let unit = feature.get("unit").and_then(Value::as_str);
        let semantics_uri = feature.get("semantics_uri").and_then(Value::as_str);
        let source_expression = feature.get("source_expression").and_then(Value::as_str);
        let inserted = ctx
            .client
            .query_opt(
                "INSERT INTO ec.feature_definition
                 (feature_key,version,scope,data_type,unit,nullable,description,semantics_uri,
                  source_expression,definition_sha256,provenance_id)
                 VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10::text,$11)
                 ON CONFLICT (feature_key,version) DO NOTHING RETURNING id",
                &[
                    &key,
                    &version,
                    &scope,
                    &data_type,
                    &unit,
                    &nullable,
                    &description,
                    &semantics_uri,
                    &source_expression,
                    &definition_sha,
                    &provenance,
                ],
            )
            .await?;
        if inserted.is_some() {
            report.count("feature_definitions", 1);
        } else {
            let have: String = ctx
                .client
                .query_one(
                    "SELECT definition_sha256::text FROM ec.feature_definition WHERE feature_key=$1 AND version=$2",
                    &[&key, &version],
                )
                .await?
                .get(0);
            if have != definition_sha {
                report.problem(format!(
                    "feature {key} v{version} changed without a version bump (database {have}, file {definition_sha})"
                ));
            }
        }
    }
    report.count("feature_definitions_seen", features.len() as u64);
    Ok(report)
}

async fn sweep(ctx: &Ctx<'_>, rel_path: &str) -> Result<Report> {
    let input = ctx.read(rel_path)?;
    let mut report = Report::new(&input);
    if input.doc.get("schema").and_then(Value::as_str) != Some("ecdb.research_sweep/v1") {
        bail!("{} has the wrong schema", input.rel_path);
    }
    let source = ctx.source(&input.rel_path, &input.sha256, "ecdb research sweep").await?;
    let provenance = ctx
        .provenance(source, "import saved research sweep", Some(&json!({"schema":"ecdb.research_sweep/v1"})))
        .await?;
    let name = text(&input.doc, "name")?;
    let version = integer(&input.doc, "version")?;
    let description = text(&input.doc, "description")?;
    let objective = text(&input.doc, "objective")?;
    let selection_policy = field(&input.doc, "selection_policy")?;
    let feature_keys = strings(&input.doc, "feature_keys")?;
    let labels = field(&input.doc, "labels")?;
    let definition_sha = sha256_hex(&canonical_json(&input.doc)?);
    let inserted = ctx
        .client
        .query_opt(
            "INSERT INTO ec.research_sweep
             (name,version,schema_uri,description,objective,selection_policy,feature_keys,labels,
              definition_sha256,provenance_id)
             VALUES ($1,$2,'ecdb.research_sweep/v1',$3,$4,$5,$6,$7,$8::text,$9)
             ON CONFLICT (name,version) DO NOTHING RETURNING id",
            &[
                &name,
                &version,
                &description,
                &objective,
                &selection_policy,
                &feature_keys,
                &labels,
                &definition_sha,
                &provenance,
            ],
        )
        .await?;
    if inserted.is_some() {
        report.count("sweeps", 1);
    }
    let row = ctx
        .client
        .query_one(
            "SELECT id,definition_sha256::text FROM ec.research_sweep WHERE name=$1 AND version=$2",
            &[&name, &version],
        )
        .await?;
    let sweep_id: i64 = row.get(0);
    let have: String = row.get(1);
    if have != definition_sha {
        report.problem(format!(
            "sweep {name} v{version} changed without a version bump (database {have}, file {definition_sha})"
        ));
        return Ok(report);
    }
    let cells = field(&input.doc, "cells")?.as_array().ok_or_else(|| anyhow!("cells is not an array"))?;
    for cell in cells {
        let cell_key = text(cell, "key")?;
        let ordinal = integer(cell, "ordinal")?;
        let field_kind = cell.get("field_kind").and_then(Value::as_str);
        let family = cell.get("family").and_then(Value::as_str);
        let bits_min = cell.get("field_bits_min").and_then(Value::as_i64).map(i32::try_from).transpose()?;
        let bits_max = cell.get("field_bits_max").and_then(Value::as_i64).map(i32::try_from).transpose()?;
        let field_degree = cell.get("field_degree").and_then(Value::as_i64).map(i32::try_from).transpose()?;
        let curve_filter = field(cell, "curve_filter")?;
        let target_curves = i32::try_from(cell.get("target_curves").and_then(Value::as_i64).unwrap_or(1))?;
        let selection_seed = cell.get("selection_seed").and_then(s);
        let factor_base_selectors = field(cell, "factor_base_selectors")?;
        let measurement_plan = field(cell, "measurement_plan")?;
        let required_features = match cell.get("required_features") {
            Some(_) => strings(cell, "required_features")?,
            None => feature_keys.clone(),
        };
        let n = ctx
            .client
            .execute(
                "INSERT INTO ec.research_sweep_cell
                 (sweep_id,cell_key,ordinal,field_kind,family,field_bits_min,field_bits_max,field_degree,
                  curve_filter,target_curves,selection_seed,factor_base_selectors,measurement_plan,
                  required_features,provenance_id)
                 VALUES ($1,$2,$3,$4::text::ec.field_kind,$5,$6,$7,$8,$9,$10,$11::text::numeric,$12,$13,$14,$15)
                 ON CONFLICT (sweep_id,cell_key) DO NOTHING",
                &[
                    &sweep_id,
                    &cell_key,
                    &ordinal,
                    &field_kind,
                    &family,
                    &bits_min,
                    &bits_max,
                    &field_degree,
                    &curve_filter,
                    &target_curves,
                    &selection_seed,
                    &factor_base_selectors,
                    &measurement_plan,
                    &required_features,
                    &provenance,
                ],
            )
            .await?;
        report.count("cells", n);
    }
    report.count("cells_seen", cells.len() as u64);
    Ok(report)
}

async fn datasets(ctx: &Ctx<'_>) -> Result<Report> {
    let input = ctx.read(DATASET_FILE)?;
    let mut report = Report::new(&input);
    if input.doc.get("schema").and_then(Value::as_str) != Some("ecdb.ml_datasets/v1") {
        bail!("{} has the wrong schema", input.rel_path);
    }
    let source = ctx.source(&input.rel_path, &input.sha256, "ecdb model-ready dataset recipes").await?;
    let provenance = ctx
        .provenance(source, "import versioned ML dataset recipes", Some(&json!({"schema":"ecdb.ml_datasets/v1"})))
        .await?;
    let rows = field(&input.doc, "datasets")?.as_array().ok_or_else(|| anyhow!("datasets is not an array"))?;
    for dataset in rows {
        let name = text(dataset, "name")?;
        let version = integer(dataset, "version")?;
        let subject_scope = text(dataset, "subject_scope")?;
        let description = text(dataset, "description")?;
        let selection_query = text(dataset, "selection_query")?;
        let split_policy = field(dataset, "split_policy")?;
        let seed = dataset.get("seed").and_then(s);
        let status = dataset.get("status").and_then(Value::as_str).unwrap_or("not_evaluated");
        let definition_sha = sha256_hex(&canonical_json(dataset)?);
        let inserted = ctx
            .client
            .query_opt(
                "INSERT INTO ec.ml_dataset
                 (name,version,subject_scope,description,selection_query,split_policy,seed,
                  definition_sha256,status,provenance_id)
                 VALUES ($1,$2,$3,$4,$5,$6,$7::text::numeric,$8::text,$9::text::ec.fact_status,$10)
                 ON CONFLICT (name,version) DO NOTHING RETURNING id",
                &[
                    &name,
                    &version,
                    &subject_scope,
                    &description,
                    &selection_query,
                    &split_policy,
                    &seed,
                    &definition_sha,
                    &status,
                    &provenance,
                ],
            )
            .await?;
        if inserted.is_some() {
            report.count("datasets", 1);
        }
        let stored = ctx
            .client
            .query_one(
                "SELECT id,definition_sha256::text FROM ec.ml_dataset WHERE name=$1 AND version=$2",
                &[&name, &version],
            )
            .await?;
        let dataset_id: i64 = stored.get(0);
        let have: String = stored.get(1);
        if have != definition_sha {
            report.problem(format!(
                "dataset {name} v{version} changed without a version bump (database {have}, file {definition_sha})"
            ));
            continue;
        }
        let features = field(dataset, "features")?.as_array().ok_or_else(|| anyhow!("features is not an array"))?;
        for feature in features {
            let key = text(feature, "feature_key")?;
            let feature_version = integer(feature, "version")?;
            let role = text(feature, "role")?;
            let ordinal = integer(feature, "ordinal")?;
            let n = ctx
                .client
                .execute(
                    "INSERT INTO ec.ml_dataset_feature (dataset_id,feature_definition_id,role,ordinal)
                     SELECT $1,d.id,$4,$5 FROM ec.feature_definition d
                      WHERE d.feature_key=$2 AND d.version=$3
                     ON CONFLICT DO NOTHING",
                    &[&dataset_id, &key, &feature_version, &role, &ordinal],
                )
                .await?;
            if n == 0 {
                let exists: bool = ctx
                    .client
                    .query_one(
                        "SELECT EXISTS (SELECT 1 FROM ec.feature_definition WHERE feature_key=$1 AND version=$2)",
                        &[&key, &feature_version],
                    )
                    .await?
                    .get(0);
                if !exists {
                    report.problem(format!("dataset {name} references unknown feature {key} v{feature_version}"));
                }
            }
            report.count("dataset_features", n);
        }
        report.count("dataset_features_seen", features.len() as u64);
    }
    report.count("datasets_seen", rows.len() as u64);
    Ok(report)
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Vec<Report>> {
    let mut reports = vec![feature_catalog(ctx).await?];
    for path in SWEEP_FILES {
        reports.push(sweep(ctx, path).await?);
    }
    reports.push(datasets(ctx).await?);
    Ok(reports)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn doc(root: &std::path::Path, path: &str) -> Value {
        serde_json::from_slice(&std::fs::read(root.join(path)).unwrap()).unwrap()
    }

    #[test]
    fn manifests_are_canonical_and_references_exist() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let features = doc(&root, FEATURE_FILE);
        assert_eq!(features["schema"], "ecdb.feature_catalog/v1");
        let keys: BTreeSet<(String, i64)> = features["features"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| (f["key"].as_str().unwrap().to_string(), f["version"].as_i64().unwrap()))
            .collect();
        assert!(keys.len() >= 75);
        canonical_json(&features).unwrap();
        for path in SWEEP_FILES {
            let sweep = doc(&root, path);
            assert_eq!(sweep["schema"], "ecdb.research_sweep/v1");
            assert!(!sweep["cells"].as_array().unwrap().is_empty(), "{path}");
            for key in sweep["feature_keys"].as_array().unwrap() {
                assert!(keys.contains(&(key.as_str().unwrap().to_string(), 1)), "unknown feature {key} in {path}");
            }
            canonical_json(&sweep).unwrap();
        }
        let datasets = doc(&root, DATASET_FILE);
        assert_eq!(datasets["schema"], "ecdb.ml_datasets/v1");
        for dataset in datasets["datasets"].as_array().unwrap() {
            for feature in dataset["features"].as_array().unwrap() {
                let key = feature["feature_key"].as_str().unwrap().to_string();
                let version = feature["version"].as_i64().unwrap();
                assert!(keys.contains(&(key, version)), "unknown feature in dataset {}", dataset["name"]);
            }
        }
        canonical_json(&datasets).unwrap();
    }
}
