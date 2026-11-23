//! Isogeny-walk outputs (`isogeny_routes.json`, written by
//! src/bin/isogeny_walk.rs): curve nodes, kernel-certified Vélu edges and
//! IW1 routes. A node's model JSON is rebuilt by the ICV1 prime-field rule and
//! must reproduce the slug's model hash; an edge needs both endpoints.

use super::registry::{
    class_row, invariant_row, prime_field, ALIAS_SQL, CLASS_SQL, CURVE_SQL, FIELD_SQL, INVARIANT_SQL, LINK_CLASS_SQL,
    POINT_SQL, REPRESENTATION_SQL, SUBGROUP_SQL, ZETA_SQL,
};
use super::{bulk, bulk1, s, Ctx, Report};
use crate::math::sha256_hex;
use anyhow::{anyhow, bail, Result};
use num_bigint::BigInt;
use num_traits::One;
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

pub const DEFAULT_FILES: &[&str] = &[
    "research/isogeny_walk_20261004/runs/p256-ell31-radius2/isogeny_routes.json",
    "research/isogeny_walk_20261004/runs/p224-ell61-radius2/isogeny_routes.json",
    "research/isogeny_walk_p192_20261004/runs/p192-ell61-radius2/isogeny_routes.json",
    "research/prime61_isogeny_20261016/one-hop-l13/isogeny_routes.json",
    "research/prime61_isogeny_20261016/one-hop-l61/isogeny_routes.json",
    "docs/ic/fixtures/current_head_prime_route_20261024/isogeny_routes.json",
];

const LEVEL_SQL: &str = "
INSERT INTO ec.endomorphism_level (curve_id, ell, level, method, status, provenance_id)
SELECT c.id, r.ell, r.level, 'proved_volcano_levels of the isogeny walk', 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, ell numeric, level int)
JOIN ec.curve c ON c.icv1_slug = r.key
ON CONFLICT (curve_id, ell) DO NOTHING";

const EDGE_SQL: &str = "
INSERT INTO ec.isogeny (external_id, source_av_id, target_av_id, source_curve_id, target_curve_id, dimension, degree, ell,
       kernel_structure, is_separable, field_of_definition_id, formula, kernel_polynomial, codomain_raw, post_isomorphism,
       direction, certificate, status, provenance_id)
SELECT r.id, sa.id, ta.id, sr.curve_id, tr.curve_id, 1, r.degree, r.degree, ARRAY[r.degree]::numeric[], r.separable,
       sc.field_id, 'velu', r.kernel_polynomial, r.codomain, r.post_isomorphism, r.direction::ec.volcano_direction,
       r.certificate, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(id text, source_uid text, target_uid text, degree numeric, separable boolean,
     kernel_polynomial jsonb, codomain jsonb, post_isomorphism jsonb, direction text, certificate text, status text)
JOIN ec.curve_representation sr ON sr.curve_uid = r.source_uid
JOIN ec.curve_representation tr ON tr.curve_uid = r.target_uid
JOIN ec.curve sc ON sc.id = sr.curve_id
JOIN ec.abelian_variety sa ON sa.curve_id = sr.curve_id
JOIN ec.abelian_variety ta ON ta.curve_id = tr.curve_id
ON CONFLICT DO NOTHING";

const ROUTE_SQL: &str = "
INSERT INTO ec.isogeny_route (external_id, source_curve_id, target_curve_id, source_av_id, target_av_id, total_degree,
       length, search_prime, status, provenance_id)
SELECT r.id, sc.id, tc.id, sa.id, ta.id, r.total_degree,
       cardinality(r.edge_ids), r.search_prime, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(id text, source_key text, target_key text, edge_ids text[], total_degree numeric,
     search_prime numeric, status text)
JOIN ec.curve sc ON sc.icv1_slug = r.source_key
JOIN ec.curve tc ON tc.icv1_slug = r.target_key
JOIN ec.abelian_variety sa ON sa.curve_id = sc.id
JOIN ec.abelian_variety ta ON ta.curve_id = tc.id
WHERE (SELECT count(*) FROM ec.isogeny i WHERE i.external_id = ANY (r.edge_ids)) = cardinality(r.edge_ids)
ON CONFLICT DO NOTHING";

const STEP_SQL: &str = "
INSERT INTO ec.isogeny_route_step (route_id, step_index, isogeny_id)
SELECT rt.id, s.idx - 1, i.id
FROM jsonb_to_recordset($1::jsonb) AS r(id text, edge_ids text[])
JOIN ec.isogeny_route rt ON rt.external_id = r.id
CROSS JOIN LATERAL unnest(r.edge_ids) WITH ORDINALITY AS s(edge, idx)
JOIN ec.isogeny i ON i.external_id = s.edge
ON CONFLICT DO NOTHING";

fn direction(d: &str) -> &'static str {
    match d {
        "h" | "horizontal" => "horizontal",
        "u" | "up" | "ascending" => "ascending",
        "d" | "down" | "descending" => "descending",
        _ => "unknown",
    }
}

fn status(s: &str) -> &'static str {
    match s {
        "verified" => "proved",
        "probable" => "probable",
        _ => "unknown",
    }
}

pub async fn import(ctx: &Ctx<'_>, files: &[String]) -> Result<Vec<Report>> {
    let mut reports = Vec::new();
    for rel in files {
        let input = ctx.read(rel)?;
        let mut report = Report::new(&input);
        let src = ctx.source(&input.rel_path, &input.sha256, "Isogeny walk routes (src/bin/isogeny_walk.rs)").await?;
        let prov = ctx.provenance(src, "isogeny_walk node / kernel-certified edge / IW1 route", None).await?;

        let mut ref_to_slug: BTreeMap<String, String> = BTreeMap::new();
        let mut fields: BTreeMap<String, Value> = BTreeMap::new();
        let mut classes: BTreeMap<String, Value> = BTreeMap::new();
        let (mut curve_rows, mut inv_rows, mut alias_rows, mut rep_rows, mut point_rows) =
            (vec![], vec![], vec![], vec![], vec![]);
        let (mut sub_rows, mut link_rows, mut zeta_rows, mut level_rows) = (vec![], vec![], vec![], vec![]);

        for n in input.doc["curve_nodes"].as_array().into_iter().flatten() {
            let slug = n["icv1_slug"].as_str().unwrap_or_default().to_string();
            let res: Result<()> = (|| {
                let p: BigInt =
                    s(&n["field_characteristic"]).and_then(|x| x.parse().ok()).ok_or_else(|| anyhow!("no p"))?;
                let a = n["coefficients_a1_a2_a3_a4_a6"].as_array().ok_or_else(|| anyhow!("no coefficients"))?;
                if a.len() != 5 || a[..3].iter().any(|x| s(x).as_deref() != Some("0")) {
                    bail!("node is not a short Weierstrass model");
                }
                let spec = prime_field(&format!("fp-{p}"), &p)?;
                let a4 = super::registry::canon_str(&spec.fq, &s(&a[3]).unwrap_or_default())?;
                let a6 = super::registry::canon_str(&spec.fq, &s(&a[4]).unwrap_or_default())?;
                // ICV1 prime model JSON: sorted keys, no whitespace (docs/curves/ICV1.md).
                let model_json = format!(
                    "{{\"a\":\"{a4}\",\"b\":\"{a6}\",\"field\":\"fp-{p}\",\"form\":\"y^2=x^3+a*x+b\",\"p\":\"{p}\",\"v\":\"1\"}}"
                );
                let model_sha = sha256_hex(&model_json);
                if !slug.ends_with(&model_sha[..8]) {
                    bail!("rebuilt model JSON hash {} does not match the slug", &model_sha[..8]);
                }
                let order: BigInt =
                    s(&n["group_order"]).and_then(|x| x.parse().ok()).ok_or_else(|| anyhow!("no order"))?;
                let t = &p + BigInt::one() - &order;
                let (label, class) = class_row(&spec, &t)?;
                classes.entry(label.clone()).or_insert(class);
                fields.entry(format!("fp-{p}")).or_insert_with(|| spec.row.clone());
                let mut coeff = Map::new();
                coeff.insert("a".into(), Value::String(a4.clone()));
                coeff.insert("b".into(), Value::String(a6.clone()));
                curve_rows.push(json!({
                    "field": format!("fp-{p}"), "model": "short_weierstrass", "form": "y^2=x^3+a*x+b",
                    "coefficients": coeff, "equation": n["equation"], "family": "isogeny-walk",
                    "model_json": model_json, "model_sha256": model_sha, "icv1": null, "slug": slug,
                    "is_smooth": true, "smoothness_status": "proved",
                }));
                match invariant_row(&slug, &spec.fq, "short_weierstrass", &coeff, n["j_invariant"].as_str()) {
                    Ok(mut row) => {
                        row["status"] = json!("proved");
                        inv_rows.push(row);
                    }
                    Err(e) => report.problem(format!("{slug}: invariants not recorded: {e}")),
                }
                link_rows.push(json!({ "key": slug, "label": label }));
                zeta_rows.push(json!({ "key": slug, "point_counts": [order.to_string()] }));
                alias_rows.push(json!({ "key": slug, "namespace": "icv1_slug", "alias": slug, "retired": false }));
                if let Some(pa) = n["position_alias"].as_str() {
                    alias_rows.push(json!({ "key": slug, "namespace": "other", "alias": pa, "retired": false }));
                }
                let uid = n["curve_uid"].as_str().unwrap_or_default();
                let sha = uid.strip_prefix("urn:ec-record:1:sha256:").unwrap_or_default();
                let gen: Option<Vec<String>> = n["generator_G"].as_array().map(|g| g.iter().filter_map(s).collect());
                rep_rows.push(json!({
                    "key": slug, "ec1": n["curve_id"], "uid": uid, "sha": sha, "field_sha256": null, "preimage": null,
                    "curve_record": null, "model_label": "short Weierstrass", "subgroup_order": s(&n["subgroup_order"]),
                    "cofactor": s(&n["cofactor"]), "generator": n["generator_G"], "target_group": "prime-order subgroup",
                    "sources": [rel],
                }));
                if let Some(xy) = &gen {
                    point_rows.push(json!({ "key": slug, "coordinates": xy, "order_value": s(&n["subgroup_order"]) }));
                }
                sub_rows.push(json!({
                    "key": slug, "order_value": s(&n["subgroup_order"]), "cofactor": s(&n["cofactor"]), "coordinates": gen,
                    "uid": uid, "is_prime": true, "primality_status": "unknown", "status": "unknown",
                }));
                for (ell, level) in n["proved_volcano_levels"].as_object().into_iter().flatten() {
                    level_rows.push(json!({ "key": slug, "ell": ell, "level": level }));
                }
                Ok(())
            })();
            match res {
                Ok(()) => {
                    ref_to_slug.insert(n["ref"].as_str().unwrap_or_default().to_string(), slug);
                }
                Err(e) => {
                    report.problem(format!("{slug}: node skipped: {e}"));
                    report.count("nodes_skipped", 1);
                }
            }
        }

        let edge_rows: Vec<Value> = input.doc["edges"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|e| {
                json!({
                    "id": e["id"], "source_uid": e["source_curve_uid"], "target_uid": e["target_curve_uid"],
                    "degree": s(&e["degree"]), "separable": e["separable"], "kernel_polynomial": e["kernel_polynomial"],
                    "codomain": e["velu_codomain_a4_a6"].as_array().map(|v| json!({"a4": v.first(), "a6": v.get(1)})),
                    "post_isomorphism": e["isomorphism_u2"].as_str().map(|u| json!({ "u2": u })),
                    "direction": direction(e["direction"].as_str().unwrap_or_default()),
                    "certificate": e["kernel_certificate"], "status": status(e["status"].as_str().unwrap_or_default()),
                })
            })
            .collect();
        let degrees: BTreeMap<String, BigInt> = input.doc["edges"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|e| Some((e["id"].as_str()?.to_string(), s(&e["degree"])?.parse().ok()?)))
            .collect();
        let route_rows: Vec<Value> = input.doc["routes"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|r| {
                let sk = ref_to_slug.get(r["source_curve_ref"].as_str()?)?;
                let tk = ref_to_slug.get(r["target_curve_ref"].as_str()?)?;
                let total = r["edge_ids"]
                    .as_array()?
                    .iter()
                    .map(|id| id.as_str().and_then(|id| degrees.get(id)).cloned())
                    .try_fold(BigInt::one(), |acc, d| d.map(|d| acc * d));
                Some(json!({
                    "id": r["id"], "source_key": sk, "target_key": tk, "edge_ids": r["edge_ids"],
                    "total_degree": total.map(|t| t.to_string()),
                    "search_prime": s(&r["search_prime"]), "status": status(r["status"].as_str().unwrap_or_default()),
                }))
            })
            .collect();

        let client = ctx.client;
        let field_rows: Vec<Value> = fields.into_values().collect();
        report.count("fields", bulk(client, FIELD_SQL, &field_rows, prov).await?);
        report.count("curves", bulk(client, CURVE_SQL, &curve_rows, prov).await?);
        report.count("genus1_invariants", bulk(client, INVARIANT_SQL, &inv_rows, prov).await?);
        report.count("aliases", bulk1(client, ALIAS_SQL, &alias_rows).await?);
        let class_rows: Vec<Value> = classes.into_values().collect();
        report.count("isogeny_classes", bulk(client, CLASS_SQL, &class_rows, prov).await?);
        report.count("jacobians_linked", bulk1(client, LINK_CLASS_SQL, &link_rows).await?);
        report.count("zeta", bulk(client, ZETA_SQL, &zeta_rows, prov).await?);
        report.count("representations", bulk(client, REPRESENTATION_SQL, &rep_rows, prov).await?);
        report.count("points", bulk(client, POINT_SQL, &point_rows, prov).await?);
        report.count("subgroups", bulk(client, SUBGROUP_SQL, &sub_rows, prov).await?);
        report.count("volcano_levels", bulk(client, LEVEL_SQL, &level_rows, prov).await?);
        report.count("isogenies", bulk(client, EDGE_SQL, &edge_rows, prov).await?);
        report.count("routes", bulk(client, ROUTE_SQL, &route_rows, prov).await?);
        report.count("route_steps", bulk1(client, STEP_SQL, &route_rows).await?);
        reports.push(report);
    }
    Ok(reports)
}
