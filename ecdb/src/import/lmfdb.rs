//! LMFDB `av_fq_isog` rows (isogeny classes of abelian varieties over finite
//! fields, any dimension) from a JSON export: a list of rows, or an object
//! with `rows` / `data`. The label is recomputed from the Weil polynomial and
//! must equal LMFDB's; LMFDB columns without an ecdb column stay verbatim in
//! `lmfdb`. Classes are not curves: no curve rows are created.

use super::{bulk, Report};
use crate::math::weil_label;
use anyhow::{anyhow, bail, Result};
use num_bigint::BigInt;
use serde_json::{json, Map, Value};
use std::path::Path;
use tokio_postgres::Client;

const CLASS_SQL: &str = "
INSERT INTO ec.isogeny_class_fq (q, p, r, g, l_polynomial, label, order_value, order_bits, point_counts, curve_counts,
       p_rank, newton_slopes, is_ordinary, is_supersingular, is_simple, is_geometrically_simple, simple_factor_labels,
       simple_multiplicities, is_squarefree, angle_rank, has_jacobian, has_principal_polarization, jacobian_count,
       twist_labels, lmfdb, status, provenance_id)
SELECT r.q, r.p, r.r, r.g, r.l_polynomial, r.label, r.order_value, ec.bits(r.order_value), r.point_counts,
       r.curve_counts, r.p_rank, r.newton_slopes, r.is_ordinary, r.is_supersingular, r.is_simple,
       r.is_geometrically_simple, r.simple_factor_labels, r.simple_multiplicities, r.is_squarefree, r.angle_rank,
       r.has_jacobian, r.has_principal_polarization, r.jacobian_count, r.twist_labels, r.lmfdb, 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(q numeric, p numeric, r int, g int, l_polynomial numeric[], label text,
     order_value numeric, point_counts numeric[], curve_counts numeric[], p_rank int, newton_slopes text[],
     is_ordinary boolean, is_supersingular boolean, is_simple boolean, is_geometrically_simple boolean,
     simple_factor_labels text[], simple_multiplicities int[], is_squarefree boolean, angle_rank int,
     has_jacobian boolean, has_principal_polarization boolean, jacobian_count int, twist_labels text[], lmfdb jsonb)
ON CONFLICT (label) DO UPDATE SET lmfdb = EXCLUDED.lmfdb, p_rank = EXCLUDED.p_rank, newton_slopes = EXCLUDED.newton_slopes,
       is_simple = EXCLUDED.is_simple, is_geometrically_simple = EXCLUDED.is_geometrically_simple,
       simple_factor_labels = EXCLUDED.simple_factor_labels, simple_multiplicities = EXCLUDED.simple_multiplicities,
       angle_rank = EXCLUDED.angle_rank, has_jacobian = EXCLUDED.has_jacobian,
       has_principal_polarization = EXCLUDED.has_principal_polarization, jacobian_count = EXCLUDED.jacobian_count,
       twist_labels = EXCLUDED.twist_labels";

const LABEL_SQL: &str = "
INSERT INTO ec.external_label (object_table, object_id, catalog, label, url, match_status, provenance_id)
SELECT 'isogeny_class_fq', ic.id, 'lmfdb', r.label, 'https://www.lmfdb.org/Variety/Abelian/Fq/' || replace(r.label, '.', '/'),
       'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(label text)
JOIN ec.isogeny_class_fq ic ON ic.label = r.label
ON CONFLICT DO NOTHING";

const MAPPED: &[&str] = &[
    "label",
    "g",
    "q",
    "p",
    "poly",
    "abvar_counts",
    "curve_counts",
    "p_rank",
    "slopes",
    "is_supersingular",
    "is_simple",
    "is_geometrically_simple",
    "simple_distinct",
    "simple_multiplicities",
    "is_squarefree",
    "angle_rank",
    "has_jacobian",
    "has_principal_polarization",
    "jacobian_count",
    "twists",
];

fn tri(v: &Value) -> Value {
    // LMFDB writes 1 / 0 / -1 (unknown) for some booleans.
    match v {
        Value::Bool(b) => json!(b),
        Value::Number(n) => match n.as_i64() {
            Some(1) => json!(true),
            Some(0) => json!(false),
            _ => Value::Null,
        },
        _ => Value::Null,
    }
}

fn row(r: &Map<String, Value>) -> Result<Value> {
    let label = r.get("label").and_then(Value::as_str).ok_or_else(|| anyhow!("row without label"))?;
    let g = r.get("g").and_then(Value::as_u64).ok_or_else(|| anyhow!("{label}: no g"))? as usize;
    let q: BigInt =
        r.get("q").map(|v| v.to_string()).and_then(|s| s.parse().ok()).ok_or_else(|| anyhow!("{label}: no q"))?;
    let p: BigInt =
        r.get("p").map(|v| v.to_string()).and_then(|s| s.parse().ok()).ok_or_else(|| anyhow!("{label}: no p"))?;
    let poly: Vec<BigInt> = match (r.get("poly"), r.get("poly_str")) {
        (Some(Value::Array(a)), _) => a.iter().map(|c| c.to_string().parse()).collect::<Result<_, _>>()?,
        (_, Some(Value::String(s))) => s.split_whitespace().map(|c| c.parse()).collect::<Result<_, _>>()?,
        _ => bail!("{label}: no Weil polynomial"),
    };
    let mine = weil_label(g, &q, &poly)?;
    if mine != label {
        bail!("label {label} does not match its polynomial (ecdb computes {mine})");
    }
    let mut rr = 0u32;
    let mut acc = BigInt::from(1);
    while acc < q {
        acc *= &p;
        rr += 1;
    }
    if acc != q {
        bail!("{label}: q is not a power of p");
    }
    let order: BigInt = poly.iter().sum();
    let extra: Map<String, Value> =
        r.iter().filter(|(k, _)| !MAPPED.contains(&k.as_str())).map(|(k, v)| (k.clone(), v.clone())).collect();
    let slopes: Option<Vec<String>> = r
        .get("slopes")
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect());
    let p_rank = r.get("p_rank").and_then(Value::as_i64);
    Ok(json!({
        "q": q.to_string(), "p": p.to_string(), "r": rr, "g": g,
        "l_polynomial": poly.iter().map(|c| c.to_string()).collect::<Vec<_>>(), "label": label,
        "order_value": order.to_string(),
        "point_counts": r.get("abvar_counts").map(|a| a.as_array().map(|v| v.iter().map(|x| x.to_string()).collect::<Vec<_>>())),
        "curve_counts": r.get("curve_counts").map(|a| a.as_array().map(|v| v.iter().map(|x| x.to_string()).collect::<Vec<_>>())),
        "p_rank": p_rank, "newton_slopes": slopes,
        "is_ordinary": p_rank.map(|x| x as usize == g), "is_supersingular": r.get("is_supersingular"),
        "is_simple": r.get("is_simple"), "is_geometrically_simple": r.get("is_geometrically_simple"),
        "simple_factor_labels": r.get("simple_distinct"), "simple_multiplicities": r.get("simple_multiplicities"),
        "is_squarefree": r.get("is_squarefree"), "angle_rank": r.get("angle_rank"),
        "has_jacobian": tri(r.get("has_jacobian").unwrap_or(&Value::Null)),
        "has_principal_polarization": tri(r.get("has_principal_polarization").unwrap_or(&Value::Null)),
        "jacobian_count": r.get("jacobian_count"),
        "twist_labels": r.get("twists").and_then(Value::as_array).map(|t| t.iter().filter_map(|x| x.get(0).cloned()).collect::<Vec<_>>()),
        "lmfdb": extra,
    }))
}

pub async fn import(client: &Client, path: &Path, provenance_title: &str) -> Result<Report> {
    let display = path.display().to_string();
    let input = super::read_json(path, &display)?;
    let mut report = Report::new(&input);
    let rows_in: Vec<Map<String, Value>> = match &input.doc {
        Value::Array(a) => a.iter().filter_map(|v| v.as_object().cloned()).collect(),
        Value::Object(o) => o
            .get("rows")
            .or_else(|| o.get("data"))
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(|v| v.as_object().cloned()).collect())
            .unwrap_or_default(),
        _ => bail!("unrecognized LMFDB export shape"),
    };
    let src: i64 = client
        .query_one(
            "WITH ins AS (INSERT INTO ec.source (kind, title, uri, sha256, provenance_class, retrieved_at)
                          VALUES ('database', $1, 'https://www.lmfdb.org/Variety/Abelian/Fq/', $2::text, 'retrieved', now())
                          ON CONFLICT DO NOTHING RETURNING id)
             SELECT id FROM ins UNION ALL
             SELECT id FROM ec.source WHERE kind = 'database' AND sha256::text = $2 AND repository IS NULL AND path IS NULL
             LIMIT 1",
            &[&provenance_title, &input.sha256],
        )
        .await?
        .get(0);
    let prov: i64 = client
        .query_one(
            "WITH ins AS (INSERT INTO ec.provenance (source_id, method, tool, tool_version, computed_at)
                          VALUES ($1, 'LMFDB av_fq_isog row; ecdb recomputed the label from the Weil polynomial', $2, $3, now())
                          ON CONFLICT DO NOTHING RETURNING id)
             SELECT id FROM ins UNION ALL
             SELECT id FROM ec.provenance WHERE source_id = $1 AND tool = $2 AND tool_version = $3 LIMIT 1",
            &[&src, &super::TOOL, &super::TOOL_VERSION],
        )
        .await?
        .get(0);
    let mut rows = vec![];
    let mut labels = vec![];
    for r in &rows_in {
        match row(r) {
            Ok(v) => {
                labels.push(json!({ "label": v["label"] }));
                rows.push(v);
            }
            Err(e) => report.problem(e.to_string()),
        }
    }
    report.count("isogeny_classes", bulk(client, CLASS_SQL, &rows, prov).await?);
    report.count("external_labels", bulk(client, LABEL_SQL, &labels, prov).await?);
    Ok(report)
}
