//! docs/curves/covers.json (curve-covers/v1): explicit same-field
//! hyperelliptic covers H -> E, H: v^2 + h(u) v = f(u), x = x(u),
//! y = y_v(u) v + y_0(u). Each certificate becomes a genus-2 or genus-3
//! curve row (with its Jacobian) and a curve_morphism to E. Existence over the
//! declared field only: no descent, subgroup-map or DLP-cost claim.

use super::{bulk, bulk1, Ctx, Report};
use crate::field::Fq;
use anyhow::{anyhow, Result};
use num_bigint::{BigInt, BigUint};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const H_CURVE_SQL: &str = "
INSERT INTO ec.curve (field_id, genus, model, form, coefficients, equation, family, is_smooth, smoothness_status, provenance_id)
SELECT e.field_id, r.genus, 'hyperelliptic', 'y^2+h(x)*y=f(x)', r.coefficients, r.equation, 'cover', true,
       r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(target text, genus int, coefficients jsonb, equation text, status text)
JOIN ec.curve e ON e.icv1_slug = r.target
ON CONFLICT DO NOTHING";

const H_MODEL_SQL: &str = "
INSERT INTO ec.hyperelliptic_model (curve_id, f, h, deg_f, deg_h, infinity_type, points_at_infinity, status, provenance_id)
SELECT h.id, r.coefficients->'f', r.coefficients->'h', r.deg_f, r.deg_h, r.infinity_type, r.points_at_infinity,
       r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(target text, coefficients jsonb, deg_f int, deg_h int, infinity_type text,
     points_at_infinity int, status text)
JOIN ec.curve e ON e.icv1_slug = r.target
JOIN ec.curve h ON h.field_id = e.field_id AND h.model = 'hyperelliptic' AND h.coefficients = r.coefficients
ON CONFLICT (curve_id) DO NOTHING";

const MORPHISM_SQL: &str = "
INSERT INTO ec.curve_morphism (source_curve_id, target_curve_id, kind, construction, degree, source_field_id,
       target_field_id, cover_genus, map, map_direction, field_relation, exists_over_base, minimal_degree, minimal_genus,
       subfield_descent, subgroup_transfer, status, method, provenance_id)
SELECT h.id, e.id, 'hyperelliptic_pullback', r.construction, r.degree, e.field_id, e.field_id, r.genus, r.map,
       r.map_direction, r.field_relation, r.exists, r.minimal_degree, r.minimal_genus, r.subfield_descent,
       r.subgroup_transfer, r.status::ec.fact_status, r.method, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(target text, coefficients jsonb, construction text, degree numeric, genus int,
     map jsonb, map_direction text, field_relation text, exists boolean, minimal_degree numeric, minimal_genus int,
     subfield_descent text, subgroup_transfer text, status text, method text)
JOIN ec.curve e ON e.icv1_slug = r.target
JOIN ec.curve h ON h.field_id = e.field_id AND h.model = 'hyperelliptic' AND h.coefficients = r.coefficients
ON CONFLICT DO NOTHING";

const FIELD_STATUS_SQL: &str = "
UPDATE ec.field f SET
  irreducibility_status = CASE WHEN r.irreducible AND f.degree > 1 THEN 'proved'::ec.fact_status
                               WHEN f.degree = 1 THEN 'not_applicable'::ec.fact_status
                               ELSE f.irreducibility_status END,
  primality_status = CASE WHEN f.characteristic = 2 THEN 'proved'::ec.fact_status
                          WHEN r.probable_prime AND f.primality_status = 'unknown' THEN 'probable'::ec.fact_status
                          ELSE f.primality_status END
FROM jsonb_to_recordset($1::jsonb) AS r(target text, irreducible boolean, probable_prime boolean)
JOIN ec.curve e ON e.icv1_slug = r.target
WHERE f.id = e.field_id";

/// Field arithmetic for every registered curve, keyed by slug.
pub async fn fields_by_slug(ctx: &Ctx<'_>) -> Result<BTreeMap<String, Fq>> {
    let rows = ctx
        .client
        .query(
            "SELECT c.icv1_slug, f.kind::text, f.characteristic::text, f.defining_polynomial::text[], f.modulus_hex
               FROM ec.curve c JOIN ec.field f ON f.id = c.field_id WHERE c.icv1_slug IS NOT NULL",
            &[],
        )
        .await?;
    let mut cache: BTreeMap<(String, String, Option<String>), Fq> = BTreeMap::new();
    let mut out = BTreeMap::new();
    for r in rows {
        let slug: String = r.get(0);
        let kind: String = r.get(1);
        let p: String = r.get(2);
        let poly: Option<Vec<String>> = r.get(3);
        let modulus: Option<String> = r.get(4);
        let key = (kind.clone(), p.clone(), modulus.clone().or_else(|| poly.as_ref().map(|v| v.join(","))));
        if let Some(k) = cache.get(&key) {
            out.insert(slug, k.clone());
            continue;
        }
        let pb: BigInt = p.parse()?;
        let k = match kind.as_str() {
            "prime" => Fq::prime(pb),
            "binary" => {
                let m = modulus.ok_or_else(|| anyhow!("binary field without modulus"))?;
                Fq::binary(
                    &BigUint::parse_bytes(m.trim_start_matches("0x").as_bytes(), 16)
                        .ok_or_else(|| anyhow!("bad modulus"))?,
                )
            }
            "prime_power" => {
                let poly = poly.ok_or_else(|| anyhow!("extension field without defining polynomial"))?;
                let low: Vec<BigInt> = poly[..poly.len() - 1].iter().map(|c| c.parse()).collect::<Result<_, _>>()?;
                Fq::extension(pb, &low)
            }
            _ => continue,
        };
        cache.insert(key, k.clone());
        out.insert(slug, k);
    }
    Ok(out)
}

fn convert(k: &Fq, v: &Value) -> Result<Vec<String>> {
    v.as_array()
        .ok_or_else(|| anyhow!("expected a coefficient list"))?
        .iter()
        .map(|c| k.from_packed_hex(c.as_str().ok_or_else(|| anyhow!("bad coefficient"))?))
        .collect()
}

/// The cover map as recorded. For a Montgomery or Edwards target the
/// polynomial map lands on an intermediate short Weierstrass model and the
/// certificate's `target_model_map` (docs/curves/COVERS.md, "Montgomery and
/// Edwards targets") carries it on to the target; both parts are kept, with
/// the composition stated. That map stays verbatim, packed-hex encoding included.
fn cover_map(k: &Fq, cert: &Value) -> Result<Value> {
    let mut map = json!({
        "x": convert(k, &cert["x"])?, "y_v": convert(k, &cert["y_v"])?, "y_0": convert(k, &cert["y_0"])?,
    });
    if cert["target_model_map"].is_null() {
        map["convention"] = json!("x = x(u), y = y_v(u)*v + y_0(u) on H: v^2 + h(u) v = f(u), onto the target model");
    } else {
        map["convention"] = json!(
            "x = x(u), y = y_v(u)*v + y_0(u) on H: v^2 + h(u) v = f(u) lands on target_model_map.source_model \
             (short Weierstrass); compose with target_model_map.forward to reach the target model. \
             target_model_map is verbatim from docs/curves/covers.json (packed-hex coefficient encoding)"
        );
        map["target_model_map"] = cert["target_model_map"].clone();
    }
    Ok(map)
}

fn poly_text(c: &[String], var: &str) -> String {
    let terms: Vec<String> = c
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, x)| !matches!(x.as_str(), "0" | "0x0") && !x.split(',').all(|d| d == "0"))
        .map(|(i, x)| match i {
            0 => format!("({x})"),
            1 => format!("({x})*{var}"),
            _ => format!("({x})*{var}^{i}"),
        })
        .collect();
    if terms.is_empty() {
        "0".into()
    } else {
        terms.join(" + ")
    }
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let input = ctx.read("docs/curves/covers.json")?;
    let mut report = Report::new(&input);
    let src = ctx.source(&input.rel_path, &input.sha256, "Explicit hyperelliptic covers (curve-covers/v1)").await?;
    let prov = ctx
        .provenance(
            src,
            "explicit pullback construction; substitution and genus/degree conditions checked by the producer",
            None,
        )
        .await?;
    let fields = fields_by_slug(ctx).await?;
    let mut rows = vec![];
    for c in input.doc["curves"].as_array().into_iter().flatten() {
        let slug = c["slug"].as_str().unwrap_or_default();
        if c["exists"].as_bool() != Some(true) || c["certificate"].is_null() {
            report.count("no_cover_recorded", 1);
            continue;
        }
        let Some(k) = fields.get(slug) else {
            report.problem(format!("{slug}: no curve row; cover skipped"));
            continue;
        };
        let res: Result<Value> = (|| {
            let cert = &c["certificate"];
            let f = convert(k, &cert["f"])?;
            let h = convert(k, &cert["h"])?;
            let genus = cert["genus"].as_u64().ok_or_else(|| anyhow!("no genus"))?;
            let deg_f = f.len().saturating_sub(1);
            let deg_h = if h.is_empty() { None } else { Some(h.len() - 1) };
            let field_check = c["field_check"].as_str().unwrap_or_default();
            let status = if field_check.contains("probable_prime") { "probable" } else { "proved" };
            let ramified = deg_f as u64 == 2 * genus + 1;
            Ok(json!({
                "target": slug, "genus": genus, "coefficients": { "f": f, "h": h },
                "equation": format!("y^2 + ({})*y = {}", poly_text(&h, "x"), poly_text(&f, "x")),
                "deg_f": deg_f, "deg_h": deg_h,
                "infinity_type": if ramified { Some("ramified") } else { None },
                "points_at_infinity": if ramified { Some(1) } else { None },
                "construction": cert["construction"], "degree": cert["degree"],
                "map": cover_map(k, cert)?,
                "map_direction": c["map_direction"], "field_relation": c["field_relation"], "exists": true,
                "minimal_degree": c["minimal_degree"], "minimal_genus": c["minimal_genus"],
                "subfield_descent": c["subfield_descent"], "subgroup_transfer": c["subgroup_transfer"],
                "status": status, "method": format!("{}; {}", c["reason"].as_str().unwrap_or_default(), field_check),
                "irreducible": field_check.contains("irreducibility") && field_check.contains("Rabin"),
                "probable_prime": field_check.contains("probable_prime"),
            }))
        })();
        match res {
            Ok(r) => rows.push(r),
            Err(e) => report.problem(format!("{slug}: cover skipped: {e}")),
        }
    }
    let client = ctx.client;
    report.count("cover_curves", bulk(client, H_CURVE_SQL, &rows, prov).await?);
    report.count("hyperelliptic_models", bulk(client, H_MODEL_SQL, &rows, prov).await?);
    report.count("morphisms", bulk(client, MORPHISM_SQL, &rows, prov).await?);
    report.count("field_status_updates", bulk1(client, FIELD_STATUS_SQL, &rows).await?);
    Ok(report)
}
