//! docs/curves/traits.json (schema curve-traits/2): Frobenius, endomorphism
//! ring and volcano levels, small-prime data, subfield descent, subgroup,
//! embedding degree, twist, DiSSECT families. Every value keeps its status;
//! the verbatim record also lands in ec.trait_record.

use super::{bulk, bulk1, s, Ctx, Report};
use crate::math::imaginary_quadratic_field;
use anyhow::{anyhow, Result};
use num_bigint::BigInt;
use serde_json::{json, Value};
use std::collections::BTreeMap;

const ORDER_CHECK_SQL: &str = "
UPDATE ec.curve_zeta z SET order_check = r.status::ec.fact_status, order_method = r.method
FROM jsonb_to_recordset($1::jsonb) AS r(key text, status text, method text)
JOIN ec.curve c ON c.icv1_slug = r.key
WHERE z.curve_id = c.id";

const CLASS_STATUS_SQL: &str = "
UPDATE ec.isogeny_class_fq ic SET status = r.status::ec.fact_status, is_ordinary = r.ordinary,
       is_supersingular = NOT r.ordinary
FROM jsonb_to_recordset($1::jsonb) AS r(key text, status text, ordinary boolean)
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.abelian_variety av ON av.curve_id = c.id
WHERE ic.id = av.isogeny_class_id";

const NUMBER_FIELD_SQL: &str = "
INSERT INTO ec.number_field (lmfdb_label, degree, signature_r1, signature_r2, defining_polynomial, discriminant,
                             fundamental_discriminant, class_number, is_galois, galois_group_label, is_cm, status, provenance_id)
SELECT r.label, 2, 0, 1, r.poly, r.d, r.d, r.class_number, true, '2T1', true, 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(label text, poly numeric[], d numeric, class_number numeric)
ON CONFLICT DO NOTHING";

const FROBENIUS_SQL: &str = "
INSERT INTO ec.frobenius_genus1 (isogeny_class_id, trace, trace_ratio, trace_mod_p, frobenius_discriminant,
       disc_factorization, disc_unfactored, disc_status, cm_discriminant, cm_field_id, conductor_v, conductor_bits,
       conductor_factorization, conductor_unfactored, conductor_fraction, class_number, class_number_status,
       is_anomalous, is_supersingular, status, provenance_id)
SELECT DISTINCT ON (av.isogeny_class_id) av.isogeny_class_id, r.trace, r.trace_ratio, mod(r.trace, ic.p), r.disc,
       r.disc_primes, r.disc_unfactored, r.disc_status::ec.fact_status, r.cm_disc, nf.id, r.conductor, r.conductor_bits,
       r.conductor_primes, r.conductor_unfactored, r.conductor_fraction, r.class_number,
       r.class_number_status::ec.fact_status, r.anomalous, NOT r.ordinary, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, trace numeric, trace_ratio numeric, disc numeric, disc_primes jsonb,
     disc_unfactored numeric[], disc_status text, cm_disc numeric, conductor numeric, conductor_bits int,
     conductor_primes jsonb, conductor_unfactored numeric[], conductor_fraction numeric, class_number numeric,
     class_number_status text, anomalous boolean, ordinary boolean, status text)
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
LEFT JOIN ec.number_field nf ON nf.fundamental_discriminant = r.cm_disc AND nf.degree = 2
ORDER BY av.isogeny_class_id
ON CONFLICT (isogeny_class_id) DO NOTHING";

// Only class invariants here (small_primes: depth, splitting, and the isogeny
// count it gives only at depth 0), so any member's record represents the class.
const FROBENIUS_ELL_SQL: &str = "
INSERT INTO ec.frobenius_ell (isogeny_class_id, ell, volcano_depth, splitting, rational_isogenies, status, provenance_id)
SELECT DISTINCT ON (av.isogeny_class_id, r.ell) av.isogeny_class_id, r.ell, r.depth, r.splitting,
       r.rational_isogenies, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, ell numeric, depth int, splitting text, rational_isogenies int,
     status text)
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.abelian_variety av ON av.curve_id = c.id
WHERE av.isogeny_class_id IS NOT NULL
ORDER BY av.isogeny_class_id, r.ell
ON CONFLICT (isogeny_class_id, ell) DO NOTHING";

// DiSSECT torsion records depend on the curve (its volcano level), not only the class.
const TORSION_SQL: &str = "
INSERT INTO ec.curve_ell_torsion (curve_id, ell, frobenius_action, rational_isogenies, torsion_point_degree,
       full_torsion_degree, isogeny_field_degree, all_isogenies_degree, division_poly_factor_degrees, status,
       provenance_id)
SELECT c.id, r.ell, r.action, r.rational_isogenies, r.point_degree, r.full_degree, r.isogeny_degree,
       r.all_isogenies_degree, r.psi_degrees, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, ell numeric, action text, rational_isogenies int, point_degree int,
     full_degree int, isogeny_degree int, all_isogenies_degree int, psi_degrees text, status text)
JOIN ec.curve c ON c.icv1_slug = r.key
ON CONFLICT (curve_id, ell) DO NOTHING";

const ENDO_SQL: &str = "
INSERT INTO ec.endomorphism_ring (curve_id, kind, discriminant, conductor_f, conductor_divides, is_maximal, contains,
       cm_field_id, status, method, provenance_id)
SELECT c.id, r.kind, r.discriminant, r.conductor, r.conductor_divides, r.maximal, r.contains, nf.id,
       r.status::ec.fact_status, 'curve_traits (docs/curves/TRAITS.md, endomorphism)', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, kind text, discriminant numeric, conductor numeric,
     conductor_divides numeric, maximal boolean, contains text[], cm_disc numeric, status text)
JOIN ec.curve c ON c.icv1_slug = r.key
LEFT JOIN ec.number_field nf ON nf.fundamental_discriminant = r.cm_disc AND nf.degree = 2
ON CONFLICT (curve_id) DO UPDATE SET kind = EXCLUDED.kind, discriminant = EXCLUDED.discriminant,
       conductor_f = EXCLUDED.conductor_f, conductor_divides = EXCLUDED.conductor_divides,
       is_maximal = EXCLUDED.is_maximal, contains = EXCLUDED.contains, cm_field_id = EXCLUDED.cm_field_id,
       status = EXCLUDED.status, method = EXCLUDED.method, provenance_id = EXCLUDED.provenance_id";

const LEVEL_SQL: &str = "
INSERT INTO ec.endomorphism_level (curve_id, ell, depth, level, level_at_most, method, status, provenance_id)
SELECT c.id, r.ell, r.depth, r.level, r.level_at_most, r.method, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, ell numeric, depth int, level int, level_at_most int, method text, status text)
JOIN ec.curve c ON c.icv1_slug = r.key
ON CONFLICT (curve_id, ell) DO UPDATE SET depth = EXCLUDED.depth, level = EXCLUDED.level,
       level_at_most = EXCLUDED.level_at_most, method = EXCLUDED.method, status = EXCLUDED.status,
       provenance_id = EXCLUDED.provenance_id";

const DESCENT_SQL: &str = "
INSERT INTO ec.descent_data (curve_id, j_field_degree, definition_degree, base_trace, base_trace_sign_free, status,
       method, provenance_id)
SELECT c.id, r.j_field_degree, r.definition_degree, r.base_trace, r.base_trace_sign_free, r.status::ec.fact_status,
       r.method, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, j_field_degree int, definition_degree int, base_trace numeric,
     base_trace_sign_free boolean, status text, method text)
JOIN ec.curve c ON c.icv1_slug = r.key
ON CONFLICT (curve_id) DO NOTHING";

// A trait subgroup is the registry representation's when one exists, so a
// second generator-less row is created only when no subgroup of that order exists.
const SUBGROUP_SQL: &str = "
INSERT INTO ec.subgroup (av_id, field_id, order_value, order_bits, is_prime_order, primality_status, cofactor, status,
       provenance_id)
SELECT av.id, c.field_id, r.order_value, ec.bits(r.order_value), r.is_prime, r.status::ec.fact_status, r.cofactor,
       r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, order_value numeric, cofactor numeric, is_prime boolean, status text)
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.abelian_variety av ON av.curve_id = c.id
WHERE NOT EXISTS (SELECT 1 FROM ec.subgroup s WHERE s.av_id = av.id AND s.field_id = c.field_id AND s.order_value = r.order_value)
ON CONFLICT DO NOTHING";

const SUBGROUP_STATUS_SQL: &str = "
UPDATE ec.subgroup s SET primality_status = r.status::ec.fact_status, status = r.status::ec.fact_status,
       is_prime_order = r.is_prime
FROM jsonb_to_recordset($1::jsonb) AS r(key text, order_value numeric, cofactor numeric, is_prime boolean, status text)
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.abelian_variety av ON av.curve_id = c.id
WHERE s.av_id = av.id AND s.field_id = c.field_id AND s.order_value = r.order_value";

const PAIRING_SQL: &str = "
INSERT INTO ec.pairing_data (subgroup_id, embedding_degree, embedding_lower_bound, complement_bits, target_field_bits,
       rho_value, status, provenance_id)
SELECT DISTINCT ON (s.id) s.id, r.degree, r.lower_bound, r.complement_bits,
       CASE WHEN r.degree IS NOT NULL AND r.degree < 1000000 THEN (r.degree * f.order_bits)::int END,
       round(f.order_bits::numeric / nullif(s.order_bits, 0), 6), r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, order_value numeric, degree numeric, lower_bound numeric,
     complement_bits int, status text)
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.field f ON f.id = c.field_id
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.subgroup s ON s.av_id = av.id AND s.field_id = c.field_id AND s.order_value = r.order_value
ORDER BY s.id
ON CONFLICT (subgroup_id) DO NOTHING";

const TWIST_SQL: &str = "
INSERT INTO ec.twist_data (curve_id, twist_degree, twist_index, twist_order, largest_prime_bits, cofactor, status,
       provenance_id)
SELECT c.id, 2, 1, r.twist_order, r.largest_prime_bits, r.cofactor, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, twist_order numeric, largest_prime_bits int, cofactor numeric, status text)
JOIN ec.curve c ON c.icv1_slug = r.key
ON CONFLICT (curve_id, twist_degree, twist_index) DO NOTHING";

const RECORD_SQL: &str = "
INSERT INTO ec.trait_record (curve_id, schema, source_id, record)
SELECT c.id, r.schema, r.source_id, r.record
FROM jsonb_to_recordset($1::jsonb) AS r(key text, schema text, source_id bigint, record jsonb)
JOIN ec.curve c ON c.icv1_slug = r.key
ON CONFLICT (curve_id, schema, source_id) DO NOTHING";

fn status(v: &Value) -> String {
    v.as_str().unwrap_or("unknown").to_string()
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let input = ctx.read("docs/curves/traits.json")?;
    let mut report = Report::new(&input);
    let schema = input.doc["schema"].as_str().unwrap_or("curve-traits").to_string();
    let src = ctx.source(&input.rel_path, &input.sha256, "Curve trait catalogue").await?;
    let prov = ctx.provenance(src, "curve_traits build (docs/curves/TRAITS.md)", None).await?;
    let curves = input.doc["curves"].as_array().ok_or_else(|| anyhow!("traits without curves"))?;

    let (mut order_rows, mut class_rows, mut frob_rows, mut ell_rows, mut endo_rows) =
        (vec![], vec![], vec![], vec![], vec![]);
    let (mut level_rows, mut descent_rows, mut sub_rows, mut pair_rows, mut twist_rows) =
        (vec![], vec![], vec![], vec![], vec![]);
    let (mut record_rows, mut torsion_rows) = (vec![], vec![]);
    let mut cm_fields: BTreeMap<String, Value> = BTreeMap::new();

    for c in curves {
        let key = c["slug"].as_str().unwrap_or_default();
        let fr = &c["frobenius"];
        let en = &c["endomorphism"];
        let ordinary = c["ordinary"].as_bool().unwrap_or(true);
        order_rows.push(
            json!({ "key": key, "status": status(&c["order_check"]["status"]), "method": c["order_check"]["method"] }),
        );
        class_rows.push(json!({ "key": key, "status": status(&c["order_check"]["status"]), "ordinary": ordinary }));

        let cm_disc = s(&fr["cm_disc"]);
        if let Some(d) = cm_disc.as_deref().and_then(|d| d.parse::<BigInt>().ok()) {
            if let Ok((poly, label)) = imaginary_quadratic_field(&d) {
                let class_number =
                    if fr["class_number_status"].as_str() == Some("proved") { s(&fr["class_number"]) } else { None };
                let entry = cm_fields.entry(d.to_string()).or_insert_with(|| {
                    json!({ "label": label, "poly": poly.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
                            "d": d.to_string(), "class_number": null })
                });
                if entry["class_number"].is_null() && class_number.is_some() {
                    entry["class_number"] = json!(class_number);
                }
            }
        }
        frob_rows.push(json!({
            "key": key, "trace": s(&c["trace"]), "trace_ratio": c["trace_ratio"], "disc": s(&fr["disc"]),
            "disc_primes": fr["disc_primes"], "disc_unfactored": fr["disc_unfactored"], "disc_status": status(&fr["disc_status"]),
            "cm_disc": cm_disc, "conductor": s(&fr["conductor"]), "conductor_bits": fr["conductor_bits"],
            "conductor_primes": fr["conductor_primes"], "conductor_unfactored": fr["conductor_unfactored"],
            "conductor_fraction": fr["conductor_fraction"], "class_number": s(&fr["class_number"]),
            "class_number_status": status(&fr["class_number_status"]), "anomalous": c["anomalous"],
            "ordinary": ordinary, "status": status(&fr["status"]),
        }));

        for sp in c["small_primes"].as_array().into_iter().flatten() {
            ell_rows.push(json!({
                "key": key, "ell": s(&sp["ell"]), "depth": sp["depth"], "splitting": sp["splitting"],
                "rational_isogenies": sp["rational_isogenies"], "status": status(&fr["status"]),
            }));
        }
        for to in c["dissect"]["torsion"].as_array().into_iter().flatten() {
            torsion_rows.push(json!({
                "key": key, "ell": s(&to["ell"]), "action": to["frobenius"], "rational_isogenies": to["rational_isogenies"],
                "point_degree": to["point_degree"], "full_degree": to["full_degree"],
                "isogeny_degree": to["isogeny_degree"], "all_isogenies_degree": to["all_isogenies_degree"],
                "psi_degrees": s(&to["psi_degrees"]), "status": status(&c["dissect"]["status"]),
            }));
        }

        let conductor = s(&en["conductor"]);
        let discriminant = match (&conductor, &cm_disc) {
            (Some(f), Some(d)) => match (f.parse::<BigInt>(), d.parse::<BigInt>()) {
                (Ok(f), Ok(d)) => Some((&f * &f * d).to_string()),
                _ => None,
            },
            _ => None,
        };
        if en["status"].as_str() != Some("not_applicable") {
            endo_rows.push(json!({
                "key": key, "kind": if ordinary { "imaginary_quadratic_order" } else { "unknown" },
                "discriminant": discriminant, "conductor": conductor, "conductor_divides": s(&en["conductor_divides"]),
                "maximal": en["maximal"], "contains": en["contains"], "cm_disc": cm_disc, "status": status(&en["status"]),
            }));
        }
        for l in en["levels"].as_array().into_iter().flatten() {
            level_rows.push(json!({
                "key": key, "ell": s(&l["ell"]), "depth": l["depth"], "level": l["level"],
                "level_at_most": l["level_at_most"], "method": l["method"],
                "status": if l["level"].is_null() { "bounded".to_string() } else { status(&en["status"]) },
            }));
        }
        let sf = &c["subfield"];
        if sf["status"].as_str() != Some("not_applicable") {
            descent_rows.push(json!({
                "key": key, "j_field_degree": sf["j_field_degree"], "definition_degree": sf["definition_degree"],
                "base_trace": s(&sf["base_trace"]), "base_trace_sign_free": sf["base_trace_sign_free"],
                "status": status(&sf["status"]), "method": sf["method"],
            }));
        }
        let sg = &c["subgroup"];
        if let Some(order) = s(&sg["order"]) {
            let st = status(&sg["status"]);
            let is_prime = match st.as_str() {
                "proved" | "probable" => Some(true),
                _ => None,
            };
            sub_rows.push(json!({ "key": key, "order_value": order, "cofactor": s(&sg["cofactor"]), "is_prime": is_prime, "status": st }));
            let em = &c["embedding"];
            if em["status"].as_str() != Some("not_applicable") {
                pair_rows.push(json!({
                    "key": key, "order_value": order, "degree": s(&em["degree"]), "lower_bound": s(&em["lower_bound"]),
                    "complement_bits": em["complement_bits"], "status": status(&em["status"]),
                }));
            }
        }
        let tw = &c["twist"];
        twist_rows.push(json!({
            "key": key, "twist_order": s(&tw["order"]), "largest_prime_bits": tw["largest_prime_bits"],
            "cofactor": s(&tw["cofactor"]), "status": status(&tw["status"]),
        }));
        record_rows.push(json!({ "key": key, "schema": schema, "source_id": src, "record": c }));
    }

    let client = ctx.client;
    let cm_rows: Vec<Value> = cm_fields.into_values().collect();
    report.count("cm_number_fields", bulk(client, NUMBER_FIELD_SQL, &cm_rows, prov).await?);
    report.count("order_checks", bulk1(client, ORDER_CHECK_SQL, &order_rows).await?);
    report.count("class_status", bulk1(client, CLASS_STATUS_SQL, &class_rows).await?);
    report.count("frobenius", bulk(client, FROBENIUS_SQL, &frob_rows, prov).await?);
    report.count("frobenius_ell", bulk(client, FROBENIUS_ELL_SQL, &ell_rows, prov).await?);
    report.count("curve_ell_torsion", bulk(client, TORSION_SQL, &torsion_rows, prov).await?);
    report.count("endomorphism_rings", bulk(client, ENDO_SQL, &endo_rows, prov).await?);
    report.count("volcano_levels", bulk(client, LEVEL_SQL, &level_rows, prov).await?);
    report.count("descent", bulk(client, DESCENT_SQL, &descent_rows, prov).await?);
    report.count("subgroups_new", bulk(client, SUBGROUP_SQL, &sub_rows, prov).await?);
    report.count("subgroups_status", bulk1(client, SUBGROUP_STATUS_SQL, &sub_rows).await?);
    report.count("pairing", bulk(client, PAIRING_SQL, &pair_rows, prov).await?);
    report.count("twists", bulk(client, TWIST_SQL, &twist_rows, prov).await?);
    report.count("trait_records", bulk1(client, RECORD_SQL, &record_rows).await?);
    let slugs: Vec<Value> = curves.iter().map(|c| c["slug"].clone()).collect();
    let unmatched: i64 = client
        .query_one(
            "SELECT count(*) FROM jsonb_array_elements_text($1::jsonb) k
              WHERE NOT EXISTS (SELECT 1 FROM ec.curve c WHERE c.icv1_slug = k)",
            &[&Value::Array(slugs)],
        )
        .await?
        .get(0);
    if unmatched > 0 {
        report.problem(format!("{unmatched} trait records name a slug with no curve row (import walks first, or the curve is unregistered)"));
    }
    Ok(report)
}
