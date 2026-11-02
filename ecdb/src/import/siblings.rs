//! The sibling repositories' curve records and measurements:
//!
//! - isogeny-labs/cryptanalysis: the IC candidate catalogue (`curves.yaml`,
//!   the source crypto mirrors), factor-base cells, identity fixtures and the
//!   ECC challenge index;
//! - isogeny-labs/crypto-autoresearcher: the UI curve capsules and trait
//!   snapshot, the NIST-seed candidates and the endomorphism sweep of named
//!   curves, and the classical-solver fixture curves;
//! - isogeny-labs/cryptanalysis-rl: the ECDLP tool-policy census and stage-0b
//!   rows;
//! - aburan28/ml-cryptanalysis: the curve corpora with measured rho cost.
//!
//! Records are kept verbatim in `external_curve_record` and linked to an
//! ecdb curve only by an exact identifier, in this order: EC1 alias,
//! curve_uid, ICV1 slug (a model match: the representation may differ),
//! a unique standard name. Prime-field dataset curves given as (p, a, b, #E)
//! become ecdb curves under the ICV1 rule (model JSON, its hash, the slug);
//! their measurements go to `measurement` in their producers' units.

use super::registry::{
    class_row, invariant_row, prime_field, ALIAS_SQL, CLASS_SQL, CURVE_SQL, FIELD_SQL, INVARIANT_SQL, LINK_CLASS_SQL,
    ZETA_SQL,
};
use super::{bulk, bulk1, read_bytes, Ctx, Report};
use crate::math::{hasse_ok, sha256_hex};
use anyhow::{bail, Context, Result};
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub struct Sibling {
    pub dir: &'static str,
    pub repository: &'static str,
}

pub const SIBLINGS: &[Sibling] = &[
    Sibling { dir: "cryptanalysis", repository: "isogeny-labs/cryptanalysis" },
    Sibling { dir: "crypto-autoresearcher", repository: "isogeny-labs/crypto-autoresearcher" },
    Sibling { dir: "cryptanalysis-rl", repository: "isogeny-labs/cryptanalysis-rl" },
    Sibling { dir: "ml-cryptanalysis", repository: "aburan28/ml-cryptanalysis" },
];

/// Files read from each sibling (the Lambda package copies exactly these to data/ext/<dir>/).
pub const FILES: &[(&str, &str)] = &[
    ("cryptanalysis", "experiments/ic-candidate-catalog/curves.yaml"),
    ("cryptanalysis", "experiments/frobenius-quotient-m4/results/factor_bases.json"),
    ("cryptanalysis", "experiments/linearized-half-decomposition/results/factor_bases.json"),
    ("cryptanalysis", "experiments/sage-ic-campaign/pb-bulk-halftrace-20260924/factor_bases.json"),
    ("cryptanalysis", "docs/known-label-pair-index-20261029/CURVE_FIXTURE.json"),
    ("cryptanalysis", "docs/ecdlp-surfaces/evidence/curve_record.json"),
    ("cryptanalysis", "challenges/ecc/catalog.json"),
    ("crypto-autoresearcher", "ui/curves/binary13.json"),
    ("crypto-autoresearcher", "ui/curves/binary19.json"),
    ("crypto-autoresearcher", "ui/curves/crypto-EC1N13Ce0h1dfa32cdf623.json"),
    ("crypto-autoresearcher", "ui/curves/crypto-EC1N19Ce0h1e2a8f558313.json"),
    ("crypto-autoresearcher", "ui/curves/crypto-traits.json"),
    ("crypto-autoresearcher", "research/nist_seed_skipped_curves_20261016/traits.json"),
    ("crypto-autoresearcher", "research/endosweep_curves_20261006/curves.json"),
    ("crypto-autoresearcher", "harness/classical_ecdlp_solver_validation/cohort/public/fixture-history-part-000.jsonl"),
    ("crypto-autoresearcher", "harness/classical_ecdlp_solver_validation/cohort/public/fixture-history-part-001.jsonl"),
    ("crypto-autoresearcher", "harness/classical_ecdlp_solver_validation/cohort/public/fixture-history-part-002.jsonl"),
    ("crypto-autoresearcher", "harness/classical_ecdlp_solver_validation/cohort/public/fixture-history-part-003.jsonl"),
    ("crypto-autoresearcher", "harness/classical_ecdlp_solver_validation/cohort/public/fixture-history-part-004.jsonl"),
    ("cryptanalysis-rl", "experiments/e7-ecdlp-tool-policy/reports/census-20261027T0659Z/rows.jsonl"),
    ("cryptanalysis-rl", "experiments/e7-ecdlp-tool-policy/reports/stage0b-20261027T1350Z/rows.jsonl"),
    ("ml-cryptanalysis", "data/curves.jsonl"),
    ("ml-cryptanalysis", "data/curves_holdout.jsonl"),
    ("ml-cryptanalysis", "data/enumeration.jsonl"),
];

const RECORD_SQL: &str = "
WITH r AS (
  SELECT * FROM jsonb_to_recordset($1::jsonb) AS r(repository text, path text, record_key text, kind text, slug text,
         ec1 text, curve_uid text, std_name text, computed boolean, record jsonb)
), m AS (
  SELECT r.*,
         (SELECT x.curve_id FROM ec.curve_representation x WHERE x.ec1_alias = r.ec1) AS by_ec1,
         (SELECT x.curve_id FROM ec.curve_representation x WHERE x.curve_uid = r.curve_uid) AS by_uid,
         (SELECT c.id FROM ec.curve c WHERE c.icv1_slug = r.slug) AS by_slug,
         (SELECT min(a.curve_id) FROM ec.curve_alias a
           WHERE r.std_name IS NOT NULL AND a.namespace IN ('standard', 'registry_alias') AND lower(a.alias) = lower(r.std_name)
          HAVING count(DISTINCT a.curve_id) = 1) AS by_std
  FROM r
)
INSERT INTO ec.external_curve_record (repository, path, record_key, kind, icv1_slug, ec1, curve_uid, curve_id, match,
       record, provenance_id)
SELECT m.repository, m.path, m.record_key, m.kind, m.slug, m.ec1, m.curve_uid,
       coalesce(m.by_ec1, m.by_uid, m.by_slug, m.by_std),
       CASE WHEN m.by_ec1 IS NOT NULL THEN 'ec1' WHEN m.by_uid IS NOT NULL THEN 'curve_uid'
            WHEN m.by_slug IS NOT NULL THEN (CASE WHEN m.computed THEN 'computed_icv1' ELSE 'icv1_slug' END)
            WHEN m.by_std IS NOT NULL THEN 'standard_name' ELSE 'none' END,
       m.record, $2::bigint
FROM m
ON CONFLICT (repository, path, record_key) DO UPDATE SET kind = EXCLUDED.kind, icv1_slug = EXCLUDED.icv1_slug,
  ec1 = EXCLUDED.ec1, curve_uid = EXCLUDED.curve_uid, curve_id = EXCLUDED.curve_id, match = EXCLUDED.match,
  record = EXCLUDED.record, provenance_id = EXCLUDED.provenance_id";

const MEASUREMENT_SQL: &str = "
INSERT INTO ec.measurement (repository, dataset, record_key, curve_id, curve_slug, method, metric, unit, value, sem, n,
       status, record, provenance_id)
SELECT r.repository, r.dataset, r.record_key, (SELECT c.id FROM ec.curve c WHERE c.icv1_slug = r.slug), r.slug,
       r.method, r.metric, r.unit, r.value, r.sem, r.n, r.status, r.record, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(repository text, dataset text, record_key text, slug text, method text,
     metric text, unit text, value double precision, sem double precision, n int, status text, record jsonb)
ON CONFLICT (repository, dataset, record_key, method, metric) DO UPDATE SET curve_id = EXCLUDED.curve_id,
  curve_slug = EXCLUDED.curve_slug, unit = EXCLUDED.unit, value = EXCLUDED.value, sem = EXCLUDED.sem, n = EXCLUDED.n,
  status = EXCLUDED.status, record = EXCLUDED.record, provenance_id = EXCLUDED.provenance_id";

/// Where a sibling's files are: `<repo_root>/ext/<dir>` (the Lambda package)
/// or the checkout beside this one (`<repo_root>/../<dir>`).
pub fn sibling_root(repo_root: &Path, dir: &str) -> Option<PathBuf> {
    [repo_root.join("ext").join(dir), repo_root.join("..").join(dir)].into_iter().find(|p| p.is_dir())
}

fn sibling_commit(root: &Path) -> Option<String> {
    if let Ok(c) = std::fs::read_to_string(root.join("SOURCE_COMMIT")) {
        return Some(c.trim().to_string());
    }
    let out = std::process::Command::new("git").arg("-C").arg(root).args(["rev-parse", "HEAD"]).output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// ICV1 for y^2 = x^3 + a x + b over F_p with #E = order: (slug, model JSON, its SHA-256).
/// docs/curves/ICV1.md: model JSON with sorted keys and canonical residues; the
/// slug is icv1-fp<bits of p>-t<trace>-<first 8 hex of the model hash>, a
/// negative trace written tm<|t|>.
pub fn prime_icv1(p: &BigInt, a: &BigInt, b: &BigInt, order: &BigInt) -> Result<(String, String, String, BigInt)> {
    let modp = |x: &BigInt| ((x % p) + p) % p;
    let (a, b) = (modp(a), modp(b));
    let disc = (BigInt::from(4) * &a * &a * &a + BigInt::from(27) * &b * &b) % p;
    if disc.is_zero() {
        bail!("singular: 4a^3 + 27b^2 = 0 mod p");
    }
    let t = p + BigInt::one() - order;
    if !hasse_ok(&t, p) {
        bail!("order {order} violates the Hasse bound for p = {p}");
    }
    let model_json = format!(
        "{{\"a\":\"{a}\",\"b\":\"{b}\",\"field\":\"fp-{p}\",\"form\":\"y^2=x^3+a*x+b\",\"p\":\"{p}\",\"v\":\"1\"}}"
    );
    let sha = sha256_hex(&model_json);
    let tr = if t.is_negative() { format!("m{}", t.abs()) } else { t.to_string() };
    let slug = format!("icv1-fp{}-t{}-{}", p.bits(), tr, &sha[..8]);
    Ok((slug, model_json, sha, t))
}

fn big(v: &Value) -> Option<BigInt> {
    match v {
        Value::Number(n) => n.to_string().parse().ok(),
        Value::String(s) => {
            let s = s.trim();
            if let Some(h) = s.strip_prefix("0x") {
                BigInt::parse_bytes(h.as_bytes(), 16)
            } else {
                s.parse().ok()
            }
        }
        _ => None,
    }
}

fn num(v: &Value) -> Value {
    match v {
        Value::Number(_) => v.clone(),
        _ => Value::Null,
    }
}

/// Dataset curves collected for insertion, keyed by slug.
#[derive(Default)]
struct Curves {
    fields: BTreeMap<String, Value>,
    classes: BTreeMap<String, Value>,
    curves: BTreeMap<String, Value>,
    invariants: Vec<Value>,
    links: Vec<Value>,
    zeta: Vec<Value>,
    aliases: Vec<Value>,
}

impl Curves {
    /// Add y^2 = x^3 + a x + b over F_p with #E = order; returns its slug.
    fn add(&mut self, p: &BigInt, a: &BigInt, b: &BigInt, order: &BigInt, family: &str) -> Result<String> {
        let (slug, model_json, sha, t) = prime_icv1(p, a, b, order)?;
        if self.curves.contains_key(&slug) {
            return Ok(slug);
        }
        let field = format!("fp-{p}");
        let spec = prime_field(&field, p)?;
        let modp = |x: &BigInt| (((x % p) + p) % p).to_string();
        let mut coeff = Map::new();
        coeff.insert("a".into(), json!(modp(a)));
        coeff.insert("b".into(), json!(modp(b)));
        let mut inv = invariant_row(&slug, &spec.fq, "short_weierstrass", &coeff, None)?;
        inv["status"] = json!("proved");
        let (label, class) = class_row(&spec, &t)?;
        self.classes.entry(label.clone()).or_insert(class);
        self.fields.entry(field.clone()).or_insert_with(|| spec.row.clone());
        self.curves.insert(
            slug.clone(),
            json!({
                "field": field, "model": "short_weierstrass", "form": "y^2=x^3+a*x+b", "coefficients": coeff,
                "equation": format!("y^2 = x^3 + {}*x + {}", modp(a), modp(b)), "family": family,
                "model_json": model_json, "model_sha256": sha, "icv1": null, "slug": slug,
                "is_smooth": true, "smoothness_status": "proved",
            }),
        );
        self.invariants.push(inv);
        self.links.push(json!({ "key": slug, "label": label }));
        self.zeta.push(json!({ "key": slug, "point_counts": [order.to_string()] }));
        self.aliases.push(json!({ "key": slug, "namespace": "icv1_slug", "alias": slug, "retired": false }));
        Ok(slug)
    }

    async fn insert(self, ctx: &Ctx<'_>, prov: i64, report: &mut Report) -> Result<()> {
        let c = ctx.client;
        let fields: Vec<Value> = self.fields.into_values().collect();
        report.count("dataset_fields", bulk(c, FIELD_SQL, &fields, prov).await?);
        let curves: Vec<Value> = self.curves.into_values().collect();
        report.count("dataset_curves_new", bulk(c, CURVE_SQL, &curves, prov).await?);
        report.count("dataset_curves_seen", curves.len() as u64);
        report.count("dataset_invariants", bulk(c, INVARIANT_SQL, &self.invariants, prov).await?);
        report.count("dataset_aliases", bulk1(c, ALIAS_SQL, &self.aliases).await?);
        let classes: Vec<Value> = self.classes.into_values().collect();
        report.count("dataset_isogeny_classes", bulk(c, CLASS_SQL, &classes, prov).await?);
        report.count("dataset_jacobians_linked", bulk1(c, LINK_CLASS_SQL, &self.links).await?);
        report.count("dataset_zeta", bulk(c, ZETA_SQL, &self.zeta, prov).await?);
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn rec(
    repo: &str,
    path: &str,
    key: &str,
    kind: &str,
    slug: Option<&str>,
    ec1: Option<&str>,
    uid: Option<&str>,
    std_name: Option<&str>,
    computed: bool,
    record: &Value,
) -> Value {
    json!({
        "repository": repo, "path": path, "record_key": key, "kind": kind, "slug": slug, "ec1": ec1,
        "curve_uid": uid, "std_name": std_name, "computed": computed, "record": record,
    })
}

#[allow(clippy::too_many_arguments)]
fn meas(
    repo: &str,
    dataset: &str,
    key: &str,
    slug: Option<&str>,
    method: &str,
    metric: &str,
    unit: &str,
    value: &Value,
    sem: &Value,
    n: &Value,
    status: &Value,
    record: &Value,
) -> Value {
    json!({
        "repository": repo, "dataset": dataset, "record_key": key, "slug": slug, "method": method, "metric": metric,
        "unit": unit, "value": num(value), "sem": num(sem), "n": num(n), "status": status, "record": record,
    })
}

/// A dataset row that is not an elliptic curve: singular rows are the
/// producers' deliberate negative controls and are only counted.
fn invalid(report: &mut Report, path: &str, key: &str, e: anyhow::Error) {
    if e.to_string().starts_with("singular") {
        report.count("dataset_rows_singular_controls", 1);
    } else {
        report.problem(format!("{path}#{key}: {e}"));
    }
}

fn jsonl(bytes: &[u8]) -> Vec<Value> {
    bytes.split(|&b| b == b'\n').filter(|l| !l.is_empty()).filter_map(|l| serde_json::from_slice(l).ok()).collect()
}

/// Turn one sibling file into external records, measurements and dataset curves.
fn parse_file(
    repo: &str,
    path: &str,
    bytes: &[u8],
    out: &mut (Vec<Value>, Vec<Value>),
    curves: &mut Curves,
    report: &mut Report,
) -> Result<()> {
    let (records, measures) = out;
    let name = Path::new(path).file_name().and_then(|n| n.to_str()).unwrap_or_default();
    let s = |v: &Value| v.as_str().map(str::to_string);
    if path.ends_with("curves.yaml") {
        let doc: Value = serde_yaml::from_slice(bytes).context("curves.yaml")?;
        for (key, c) in doc["curves"].as_object().into_iter().flatten() {
            records.push(rec(
                repo,
                path,
                key,
                "ic_catalog",
                c["icv1_identity"]["slug"].as_str(),
                c["curve_id"].as_str(),
                c["curve_uid"].as_str(),
                None,
                false,
                c,
            ));
        }
    } else if name == "factor_bases.json" {
        let doc: Value = serde_json::from_slice(bytes)?;
        for c in doc["cells"].as_array().into_iter().flatten() {
            let key = s(&c["factor_base_sha256"]).unwrap_or_else(|| c.to_string());
            records.push(rec(repo, path, &key, "factor_base", None, c["curve_id"].as_str(), None, None, false, c));
        }
    } else if name == "CURVE_FIXTURE.json" || name == "curve_record.json" {
        let doc: Value = serde_json::from_slice(bytes)?;
        let key = s(&doc["curve_uid"]).or_else(|| s(&doc["curve_id"])).unwrap_or_else(|| name.to_string());
        records.push(rec(
            repo,
            path,
            &key,
            "identity_fixture",
            None,
            doc["curve_id"].as_str(),
            doc["curve_uid"].as_str(),
            None,
            false,
            &doc,
        ));
    } else if path.ends_with("challenges/ecc/catalog.json") {
        let doc: Value = serde_json::from_slice(bytes)?;
        for c in doc["curves"].as_array().into_iter().flatten() {
            if let Some(id) = c["id"].as_str() {
                records.push(rec(repo, path, id, "challenge_index", None, None, None, None, false, c));
            }
        }
    } else if path.starts_with("ui/curves/") && name == "crypto-traits.json" {
        let doc: Value = serde_json::from_slice(bytes)?;
        for c in doc["records"].as_array().into_iter().flatten() {
            let rep = &c["representations"][0];
            records.push(rec(
                repo,
                path,
                c["slug"].as_str().unwrap_or_default(),
                "traits_snapshot",
                c["slug"].as_str(),
                rep["ec1"].as_str(),
                rep["curve_uid"].as_str(),
                None,
                false,
                c,
            ));
        }
    } else if path.starts_with("ui/curves/") {
        let doc: Value = serde_json::from_slice(bytes)?;
        let key = s(&doc["curve_uid"]).unwrap_or_else(|| name.to_string());
        records.push(rec(
            repo,
            path,
            &key,
            "curve_capsule",
            None,
            doc["curve"]["curve_id"].as_str(),
            doc["curve_uid"].as_str(),
            None,
            false,
            &doc,
        ));
    } else if path.contains("nist_seed_skipped_curves") {
        let doc: Value = serde_json::from_slice(bytes)?;
        for (k, c) in doc["published_controls"].as_object().into_iter().flatten() {
            records.push(rec(
                repo,
                path,
                k,
                "nist_seed_control",
                None,
                c["ec1"].as_str(),
                c["curve_uid"].as_str(),
                Some(k),
                false,
                c,
            ));
        }
        for (i, c) in doc["unpublished_candidates"].as_array().into_iter().flatten().enumerate() {
            let key = s(&c["name"]).or_else(|| s(&c["ec1"])).unwrap_or_else(|| format!("candidate-{i}"));
            records.push(rec(
                repo,
                path,
                &key,
                "nist_seed_candidate",
                None,
                c["ec1"].as_str(),
                c["curve_uid"].as_str(),
                None,
                false,
                c,
            ));
        }
    } else if path.contains("endosweep_curves") {
        let doc: Value = serde_json::from_slice(bytes)?;
        for c in doc["results"].as_array().into_iter().flatten() {
            let Some(full) = c["name"].as_str() else { continue };
            let short = full.rsplit('/').next().unwrap_or(full);
            records.push(rec(repo, path, full, "endosweep_named_curve", None, None, None, Some(short), false, c));
        }
    } else if path.contains("fixture-history") {
        for r in jsonl(bytes) {
            let key = format!("{}", r["i"]);
            let slug = match (big(&r["p"]), big(&r["a"]), big(&r["b"]), big(&r["N"])) {
                (Some(p), Some(a), Some(b), Some(n)) => {
                    curves.add(&p, &a, &b, &n, "dataset").map_err(|e| invalid(report, path, &key, e)).ok()
                }
                _ => None,
            };
            records.push(rec(repo, path, &key, "dataset_curve", slug.as_deref(), None, None, None, true, &r));
        }
    } else if path.contains("e7-ecdlp-tool-policy") {
        let census = path.contains("census");
        for r in jsonl(bytes) {
            let key = format!("{}:{}:{}:{}", r["arm"].as_str().unwrap_or("-"), r["bits"], r["index"], r["seed"]);
            let slug = match (big(&r["p"]), big(&r["a"]), big(&r["b"]), big(&r["group_order"])) {
                (Some(p), Some(a), Some(b), Some(n)) => {
                    curves.add(&p, &a, &b, &n, "dataset").map_err(|e| invalid(report, path, &key, e)).ok()
                }
                _ => None,
            };
            if census {
                records.push(rec(repo, path, &key, "dataset_curve", slug.as_deref(), None, None, None, true, &r));
            }
            let method = r["method"].as_str().or(r["arm"].as_str()).unwrap_or("unknown");
            let ops = if r["group_ops"].is_null() { &r["cost"]["group_ops"] } else { &r["group_ops"] };
            measures.push(meas(
                repo,
                path,
                &key,
                slug.as_deref(),
                method,
                "group_ops",
                "group operations",
                ops,
                &Value::Null,
                &json!(1),
                &r["status"],
                &r,
            ));
            let ratio = &r["ratio"]["0.0"];
            if ratio.is_number() {
                measures.push(meas(
                    repo,
                    path,
                    &key,
                    slug.as_deref(),
                    method,
                    "ratio_to_rho_expected_gae0",
                    "ratio (0 GAE per unpriced op)",
                    ratio,
                    &Value::Null,
                    &json!(1),
                    &r["status"],
                    &Value::Null,
                ));
            }
        }
    } else if repo == "aburan28/ml-cryptanalysis" {
        for r in jsonl(bytes) {
            let (Some(p), Some(a), Some(b), Some(n)) = (big(&r["p"]), big(&r["a"]), big(&r["b"]), big(&r["N"])) else {
                continue;
            };
            let key = format!("{p}:{a}:{b}");
            let slug = match curves.add(&p, &a, &b, &n, "dataset") {
                Ok(s) => Some(s),
                Err(e) => {
                    invalid(report, path, &key, e);
                    None
                }
            };
            records.push(rec(repo, path, &key, "dataset_curve", slug.as_deref(), None, None, None, true, &r));
            for arm in ["aut", "plain"] {
                for m in ["S_walk", "S_online", "S_total"] {
                    let v = &r[format!("{arm}_{m}_mean")];
                    if v.is_number() {
                        let method = if arm == "aut" { "rho.automorphism_classes" } else { "rho.plain" };
                        measures.push(meas(
                            repo,
                            path,
                            &key,
                            slug.as_deref(),
                            method,
                            m,
                            "S: rho cost / sqrt(r), as recorded by ml-cryptanalysis",
                            v,
                            &r[format!("{arm}_{m}_sem")],
                            &r["n_seeds"],
                            &r["status"],
                            &Value::Null,
                        ));
                    }
                }
            }
        }
    } else {
        bail!("no reader for {repo}:{path}");
    }
    Ok(())
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let mut report = Report { input: "sibling repositories".into(), ..Default::default() };
    for sib in SIBLINGS {
        let Some(root) = sibling_root(&ctx.repo_root, sib.dir) else {
            report.problem(format!("{}: no checkout found beside this one or under ext/", sib.repository));
            continue;
        };
        let commit = sibling_commit(&root);
        for (dir, path) in FILES.iter().filter(|(d, _)| *d == sib.dir) {
            let _ = dir;
            let bytes = match read_bytes(&root.join(path)) {
                Ok(b) => b,
                Err(_) => {
                    report.problem(format!("{}:{path} not found", sib.repository));
                    continue;
                }
            };
            let sha = hex::encode(<sha2::Sha256 as sha2::Digest>::digest(&bytes));
            let src = ctx
                .source_in(sib.repository, path, &sha, commit.as_deref(), &format!("{} {path}", sib.repository))
                .await?;
            let prov = ctx.provenance(src, "sibling repository record", None).await?;
            let mut out = (Vec::new(), Vec::new());
            let mut curves = Curves::default();
            if let Err(e) = parse_file(sib.repository, path, &bytes, &mut out, &mut curves, &mut report) {
                report.problem(format!("{}:{path}: {e}", sib.repository));
                continue;
            }
            curves.insert(ctx, prov, &mut report).await?;
            report.count("external_records", bulk(ctx.client, RECORD_SQL, &out.0, prov).await?);
            report.count("measurements", bulk(ctx.client, MEASUREMENT_SQL, &out.1, prov).await?);
            report.count("files", 1);
        }
    }
    for row in ctx
        .client
        .query("SELECT kind || ':' || match, count(*) FROM ec.external_curve_record GROUP BY 1 ORDER BY 1", &[])
        .await?
    {
        let k: String = row.get(0);
        let n: i64 = row.get(1);
        report.count(&format!("match:{k}"), n as u64);
    }
    Ok(report)
}

/// Copy the sibling files into `<out>/ext/<dir>/` with each checkout's commit (for the Lambda package).
pub fn stage(repo_root: &Path, out: &Path) -> Result<Vec<String>> {
    let mut staged = Vec::new();
    for sib in SIBLINGS {
        let Some(root) = sibling_root(repo_root, sib.dir) else {
            eprintln!(
                "ecdb pack: {} has no checkout beside {}; its records are not packed",
                sib.repository,
                repo_root.display()
            );
            continue;
        };
        let dest = out.join("ext").join(sib.dir);
        std::fs::create_dir_all(&dest)?;
        if let Some(c) = sibling_commit(&root) {
            std::fs::write(dest.join("SOURCE_COMMIT"), c)?;
        }
        for (_, path) in FILES.iter().filter(|(d, _)| *d == sib.dir) {
            let bytes = read_bytes(&root.join(path)).with_context(|| format!("{}:{path}", sib.repository))?;
            let target = dest.join(format!("{path}.gz"));
            std::fs::create_dir_all(target.parent().unwrap())?;
            let mut enc = flate2::write::GzEncoder::new(std::fs::File::create(&target)?, flate2::Compression::best());
            std::io::Write::write_all(&mut enc, &bytes)?;
            enc.finish()?;
            staged.push(format!("{}:{path}", sib.repository));
        }
    }
    Ok(staged)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_icv1_reproduces_registry_slugs() {
        let b = |s: &str| s.parse::<BigInt>().unwrap();
        let (slug, _, _, t) = prime_icv1(&b("827"), &b("1"), &b("15"), &b("823")).unwrap();
        assert_eq!((slug.as_str(), t), ("icv1-fp10-t5-192cb216", b("5")));
        let (slug, _, _, _) = prime_icv1(&b("10935329"), &b("5320418"), &b("8535318"), &b("10933753")).unwrap();
        assert_eq!(slug, "icv1-fp24-t1577-77336155");
        // a and b are reduced mod p before hashing
        let (again, _, _, _) = prime_icv1(&b("827"), &b("828"), &b("-812"), &b("823")).unwrap();
        assert_eq!(again, "icv1-fp10-t5-192cb216");
        assert!(prime_icv1(&b("827"), &b("1"), &b("15"), &b("2000")).is_err());
    }
}
