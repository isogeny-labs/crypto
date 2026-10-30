//! docs/curves/registry.json: every ICV1 curve the repository names, with
//! aliases, standards publications and EC1 representations.
//!
//! Checks made on the way in (each failure is reported, never papered over):
//! the ICV1 field hash against the modulus, SHA-256 of the model JSON
//! against the slug, #E = q + 1 - t, the Hasse bound, the EC1 field and
//! curve digests against their recomputed preimages, and j computed from the
//! model against the registry's j.

use super::{bulk, s, Ctx, Report};
use crate::field::{a_invariants, invariants_from_a, Fq};
use crate::math::{
    canonical_json, ec1_preimage, genus1_l_poly, hasse_ok, icv1_binary_field, icv1_extension_field, is_retired_name,
    modulus_exponents, polynomial_shape, sha256_hex, weil_label,
};
use anyhow::{anyhow, bail, Result};
use num_bigint::{BigInt, BigUint};
use num_traits::One;
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

/// A field as the importers need it: arithmetic plus its `ec.field` row.
pub struct FieldSpec {
    pub fq: Fq,
    pub q: BigInt,
    pub p: BigInt,
    pub degree: u32,
    pub row: Value,
}

pub fn binary_field(icv1_field: &str, modulus_hex: &str) -> Result<FieldSpec> {
    let modulus_hex = modulus_hex.to_lowercase();
    let modulus = BigUint::parse_bytes(modulus_hex.trim_start_matches("0x").as_bytes(), 16)
        .ok_or_else(|| anyhow!("bad binary modulus {modulus_hex}"))?;
    let m = modulus.bits() - 1;
    let expect = icv1_binary_field(m, &modulus_hex);
    if expect != icv1_field {
        bail!("ICV1 field {icv1_field} does not match its modulus {modulus_hex} (expected {expect})");
    }
    let fq = Fq::binary(&modulus);
    let exps = modulus_exponents(&modulus);
    let q = BigInt::from(2u8).pow(m as u32);
    let row = json!({
        "kind": "binary", "characteristic": "2", "degree": m, "order_q": q.to_string(), "order_bits": m,
        "representation": "polynomial",
        "defining_polynomial": fq.f.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
        "modulus_exponents": exps, "modulus_hex": modulus_hex, "polynomial_shape": polynomial_shape(exps.len()),
        "element_encoding": "hex_bitset", "icv1_field": icv1_field, "label": format!("GF(2^{m})"),
    });
    Ok(FieldSpec { fq, q, p: BigInt::from(2u8), degree: m as u32, row })
}

pub fn prime_field(icv1_field: &str, p: &BigInt) -> Result<FieldSpec> {
    if icv1_field != format!("fp-{p}") {
        bail!("ICV1 field {icv1_field} does not name GF({p})");
    }
    let row = json!({
        "kind": "prime", "characteristic": p.to_string(), "degree": 1, "order_q": p.to_string(),
        "order_bits": (p - 1u8).bits(), "representation": "prime", "defining_polynomial": null,
        "modulus_exponents": null, "modulus_hex": null, "polynomial_shape": null,
        "element_encoding": "decimal", "icv1_field": icv1_field, "label": format!("GF({p})"),
    });
    Ok(FieldSpec { fq: Fq::prime(p.clone()), q: p.clone(), p: p.clone(), degree: 1, row })
}

pub fn extension_field(icv1_field: &str, p: &BigInt, low: &[BigInt]) -> Result<FieldSpec> {
    let expect = icv1_extension_field(p, low);
    if expect != icv1_field {
        bail!("ICV1 field {icv1_field} does not match its modulus (expected {expect})");
    }
    let fq = Fq::extension(p.clone(), low);
    let k = low.len() as u32;
    let q = p.pow(k);
    let row = json!({
        "kind": "prime_power", "characteristic": p.to_string(), "degree": k, "order_q": q.to_string(),
        "order_bits": (&q - 1u8).bits(), "representation": "polynomial",
        "defining_polynomial": fq.f.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
        "modulus_exponents": null, "modulus_hex": null, "polynomial_shape": null,
        "element_encoding": "decimal_vector", "icv1_field": icv1_field, "label": format!("GF({p}^{k})"),
    });
    Ok(FieldSpec { fq, q, p: p.clone(), degree: k, row })
}

/// The field named by an ICV1 model JSON.
pub fn field_of_model(model: &Map<String, Value>) -> Result<FieldSpec> {
    let fid = model.get("field").and_then(Value::as_str).ok_or_else(|| anyhow!("model without field"))?;
    let int = |k: &str| -> Result<BigInt> {
        model.get(k).and_then(s).and_then(|x| x.parse().ok()).ok_or_else(|| anyhow!("model without {k}"))
    };
    if fid.starts_with("f2m-") {
        let modulus =
            model.get("modulus").and_then(Value::as_str).ok_or_else(|| anyhow!("binary model without modulus"))?;
        binary_field(fid, modulus)
    } else if fid.starts_with("fpk-") {
        let p = int("p")?;
        let low: Vec<BigInt> = model
            .get("modulus")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("extension model without modulus"))?
            .iter()
            .map(|c| s(c).and_then(|x| x.parse().ok()).ok_or_else(|| anyhow!("bad modulus coefficient")))
            .collect::<Result<_>>()?;
        extension_field(fid, &p, &low)
    } else if fid.starts_with("fp-") {
        prime_field(fid, &int("p")?)
    } else {
        bail!("unsupported ICV1 field {fid}")
    }
}

/// (ecdb model, coefficient names, equation template) for an ICV1 form.
pub fn model_of_form(form: &str) -> Result<(&'static str, &'static [&'static str])> {
    Ok(match form {
        "y^2=x^3+a*x+b" => ("short_weierstrass", &["a", "b"]),
        "y^2+xy=x^3+a*x^2+b" => ("binary_weierstrass", &["a", "b"]),
        "B*y^2=x^3+A*x^2+x" => ("montgomery", &["A", "B"]),
        "x^2+y^2=c^2*(1+d*x^2*y^2)" => ("edwards", &["c", "d"]),
        "a*x^2+y^2=1+d*x^2*y^2" => ("twisted_edwards", &["a", "d"]),
        other => bail!("unsupported ICV1 form {other}"),
    })
}

/// Canonical element string of a model coefficient (lists become e_0,...,e_{k-1}).
pub fn canon_element(k: &Fq, v: &Value) -> Result<String> {
    let raw = match v {
        Value::Array(a) => {
            a.iter().map(|c| s(c).ok_or_else(|| anyhow!("bad coefficient"))).collect::<Result<Vec<_>>>()?.join(",")
        }
        other => s(other).ok_or_else(|| anyhow!("bad coefficient {other}"))?,
    };
    canon_str(k, &raw)
}

pub fn canon_str(k: &Fq, raw: &str) -> Result<String> {
    use crate::field::Encoding;
    if k.encoding == Encoding::Decimal && raw.starts_with("0x") {
        let v = BigUint::parse_bytes(&raw.as_bytes()[2..], 16).ok_or_else(|| anyhow!("bad hex {raw}"))?;
        return Ok(k.format(&k.parse(&v.to_string())?));
    }
    Ok(k.format(&k.parse(&raw.to_lowercase())?))
}

fn equation(model: &str, c: &Map<String, Value>) -> String {
    let g = |k: &str| c.get(k).and_then(Value::as_str).unwrap_or("?").to_string();
    match model {
        "short_weierstrass" => format!("y^2 = x^3 + ({})*x + ({})", g("a"), g("b")),
        "binary_weierstrass" => format!("y^2 + x*y = x^3 + ({})*x^2 + ({})", g("a"), g("b")),
        "montgomery" => format!("({})*y^2 = x^3 + ({})*x^2 + x", g("B"), g("A")),
        "edwards" => format!("x^2 + y^2 = ({c})^2*(1 + ({d})*x^2*y^2)", c = g("c"), d = g("d")),
        "twisted_edwards" => format!("({})*x^2 + y^2 = 1 + ({})*x^2*y^2", g("a"), g("d")),
        _ => String::new(),
    }
}

/// Genus-1 invariant row, j checked against `expected_j` when given.
pub fn invariant_row(
    key: &str,
    k: &Fq,
    model: &str,
    coeff: &Map<String, Value>,
    expected_j: Option<&str>,
) -> Result<Value> {
    let (a, map) = a_invariants(k, model, coeff)?;
    let inv = invariants_from_a(k, a)?;
    let j = k.format(&inv.j);
    if let Some(e) = expected_j {
        let e = canon_str(k, e)?;
        if e != j {
            bail!("j computed from the model is {j}, the record says {e}");
        }
    }
    let f = |e: &Vec<BigInt>| k.format(e);
    let is_char23 = k.p == BigInt::from(2u8) || k.p == BigInt::from(3u8);
    let j0 = k.is_zero(&inv.j);
    let j1728 = if is_char23 { j0 } else { k.is_zero(&k.sub(&inv.j, &k.from_int(1728))) };
    Ok(json!({
        "key": key,
        "a1": f(&inv.a[0]), "a2": f(&inv.a[1]), "a3": f(&inv.a[2]), "a4": f(&inv.a[3]), "a6": f(&inv.a[4]),
        "b2": f(&inv.b2), "b4": f(&inv.b4), "b6": f(&inv.b6), "b8": f(&inv.b8),
        "c4": f(&inv.c4), "c6": f(&inv.c6), "discriminant": f(&inv.disc), "j_invariant": j,
        "j_is_0": j0, "j_is_1728": j1728,
        "to_weierstrass": { "map": map },
    }))
}

pub const FIELD_SQL: &str = "
INSERT INTO ec.field (kind, characteristic, degree, order_q, order_bits, representation, defining_polynomial,
                      modulus_exponents, modulus_hex, polynomial_shape, element_encoding, icv1_field, label, provenance_id)
SELECT r.kind::ec.field_kind, r.characteristic, r.degree, r.order_q, r.order_bits,
       r.representation::ec.field_representation, r.defining_polynomial, r.modulus_exponents, r.modulus_hex,
       r.polynomial_shape, r.element_encoding::ec.fe_encoding, r.icv1_field, r.label, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(kind text, characteristic numeric, degree int, order_q numeric, order_bits int,
     representation text, defining_polynomial numeric[], modulus_exponents int[], modulus_hex text,
     polynomial_shape text, element_encoding text, icv1_field text, label text)
ON CONFLICT DO NOTHING";

pub const CURVE_SQL: &str = "
INSERT INTO ec.curve (field_id, genus, model, form, coefficients, equation, family, model_json, model_sha256,
                      icv1, icv1_slug, is_smooth, smoothness_status, provenance_id)
SELECT f.id, 1, r.model::ec.curve_model, r.form, r.coefficients, r.equation, r.family, r.model_json, r.model_sha256,
       r.icv1, r.slug, r.is_smooth, r.smoothness_status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(field text, model text, form text, coefficients jsonb, equation text,
     family text, model_json text, model_sha256 text, icv1 text, slug text, is_smooth boolean, smoothness_status text)
JOIN ec.field f ON f.icv1_field = r.field
ON CONFLICT DO NOTHING";

pub const INVARIANT_SQL: &str = "
INSERT INTO ec.genus1_invariants (curve_id, a1, a2, a3, a4, a6, b2, b4, b6, b8, c4, c6, discriminant, j_invariant,
                                  j_is_0, j_is_1728, to_weierstrass, status, provenance_id)
SELECT c.id, r.a1, r.a2, r.a3, r.a4, r.a6, r.b2, r.b4, r.b6, r.b8, r.c4, r.c6, r.discriminant, r.j_invariant,
       r.j_is_0, r.j_is_1728, r.to_weierstrass, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, a1 text, a2 text, a3 text, a4 text, a6 text, b2 text, b4 text,
     b6 text, b8 text, c4 text, c6 text, discriminant text, j_invariant text, j_is_0 boolean, j_is_1728 boolean,
     to_weierstrass jsonb, status text)
JOIN ec.curve c ON c.icv1_slug = r.key
ON CONFLICT (curve_id) DO NOTHING";

pub const CLASS_SQL: &str = "
INSERT INTO ec.isogeny_class_fq (q, p, r, g, l_polynomial, label, order_value, order_bits, point_counts, curve_counts,
                                 is_ordinary, is_supersingular, status, provenance_id)
SELECT r.q, r.p, r.r, r.g, r.l_polynomial, r.label, r.order_value, r.order_bits, r.point_counts, r.point_counts,
       r.is_ordinary, r.is_supersingular, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(q numeric, p numeric, r int, g int, l_polynomial numeric[], label text,
     order_value numeric, order_bits int, point_counts numeric[], is_ordinary boolean, is_supersingular boolean, status text)
ON CONFLICT DO NOTHING";

pub const LINK_CLASS_SQL: &str = "
UPDATE ec.abelian_variety av SET isogeny_class_id = ic.id
FROM jsonb_to_recordset($1::jsonb) AS r(key text, label text)
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.isogeny_class_fq ic ON ic.label = r.label
WHERE av.curve_id = c.id AND av.isogeny_class_id IS DISTINCT FROM ic.id";

pub const ZETA_SQL: &str = "
INSERT INTO ec.curve_zeta (curve_id, isogeny_class_id, point_counts, order_check, provenance_id)
SELECT c.id, av.isogeny_class_id, r.point_counts, 'unknown', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, point_counts numeric[])
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.abelian_variety av ON av.curve_id = c.id
ON CONFLICT (curve_id) DO NOTHING";

pub const ALIAS_SQL: &str = "
INSERT INTO ec.curve_alias (curve_id, namespace, alias, is_retired)
SELECT c.id, r.namespace::ec.alias_namespace, r.alias, r.retired
FROM jsonb_to_recordset($1::jsonb) AS r(key text, namespace text, alias text, retired boolean)
JOIN ec.curve c ON c.icv1_slug = r.key
ON CONFLICT DO NOTHING";

const STANDARD_SQL: &str = "
INSERT INTO ec.curve_standard (curve_id, display_name, category, oid, source_ref, adoption_status,
                               parameter_validation, sources, provenance_id)
SELECT c.id, r.display_name, r.category, r.oid, r.source_ref, r.adoption_status, r.parameter_validation, r.sources, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, display_name text, category text, oid text, source_ref text,
     adoption_status text, parameter_validation text, sources jsonb)
JOIN ec.curve c ON c.icv1_slug = r.key
ON CONFLICT DO NOTHING";

const FIELD_EC1_SQL: &str = "
INSERT INTO ec.field_ec1_record (field_id, field_sha256, canonical_json, record)
SELECT f.id, r.field_sha256, r.canonical_json, r.record
FROM jsonb_to_recordset($1::jsonb) AS r(field text, field_sha256 text, canonical_json text, record jsonb)
JOIN ec.field f ON f.icv1_field = r.field
ON CONFLICT DO NOTHING";

pub const REPRESENTATION_SQL: &str = "
INSERT INTO ec.curve_representation (curve_id, ec1_alias, curve_uid, curve_sha256, field_record_id, canonical_preimage,
       curve_record, model_label, subgroup_order, cofactor, generator, target_group, sources, provenance_id)
SELECT c.id, r.ec1, r.uid, r.sha, fr.id, r.preimage, r.curve_record, r.model_label, r.subgroup_order, r.cofactor,
       r.generator, r.target_group, r.sources, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, ec1 text, uid text, sha text, field_sha256 text, preimage text,
     curve_record jsonb, model_label text, subgroup_order numeric, cofactor numeric, generator jsonb,
     target_group text, sources text[])
JOIN ec.curve c ON c.icv1_slug = r.key
LEFT JOIN ec.field_ec1_record fr ON fr.field_sha256::text = r.field_sha256
ON CONFLICT DO NOTHING";

pub const POINT_SQL: &str = "
INSERT INTO ec.point (curve_id, av_id, field_id, representation, coordinates, order_value, order_status, role, provenance_id)
SELECT c.id, av.id, c.field_id, 'affine', r.coordinates, r.order_value, 'unknown', 'generator', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, coordinates jsonb, order_value numeric)
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.abelian_variety av ON av.curve_id = c.id
ON CONFLICT DO NOTHING";

pub const SUBGROUP_SQL: &str = "
INSERT INTO ec.subgroup (av_id, field_id, order_value, order_bits, is_prime_order, primality_status, cofactor,
                         generator_point_id, representation_id, status, provenance_id)
SELECT av.id, c.field_id, r.order_value, ec.bits(r.order_value), r.is_prime, r.primality_status::ec.fact_status,
       r.cofactor, p.id, rep.id, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, order_value numeric, cofactor numeric, coordinates jsonb, uid text,
     is_prime boolean, primality_status text, status text)
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.abelian_variety av ON av.curve_id = c.id
LEFT JOIN ec.point p ON p.curve_id = c.id AND p.role = 'generator' AND p.coordinates = r.coordinates
LEFT JOIN ec.curve_representation rep ON rep.curve_uid = r.uid
ON CONFLICT DO NOTHING";

const GENERATION_SQL: &str = "
INSERT INTO ec.generation_provenance (curve_id, method, generator_call, status, provenance_id)
SELECT c.id, r.method, r.generator_call, 'unknown', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, method text, generator_call text)
JOIN ec.curve c ON c.icv1_slug = r.key
ON CONFLICT (curve_id) DO NOTHING";

pub const EVIDENCE_SQL: &str = "
INSERT INTO ec.evidence_link (curve_id, kind, repository, path, provenance_id)
SELECT c.id, r.kind, r.repository, r.path, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, kind text, repository text, path text)
JOIN ec.curve c ON c.icv1_slug = r.key
ON CONFLICT DO NOTHING";

/// Isogeny-class row for a genus-1 curve over F_q with trace t.
pub fn class_row(spec: &FieldSpec, t: &BigInt) -> Result<(String, Value)> {
    if !hasse_ok(t, &spec.q) {
        bail!("trace {t} violates the Hasse bound over a field of size {}", spec.q);
    }
    let l = genus1_l_poly(t, &spec.q);
    let label = weil_label(1, &spec.q, &l)?;
    let order = &spec.q + BigInt::one() - t;
    let supersingular = (t % &spec.p) == BigInt::from(0);
    Ok((
        label.clone(),
        json!({
            "q": spec.q.to_string(), "p": spec.p.to_string(), "r": spec.degree, "g": 1,
            "l_polynomial": l.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
            "label": label, "order_value": order.to_string(), "order_bits": order.bits(),
            "point_counts": [order.to_string()],
            "is_ordinary": !supersingular, "is_supersingular": supersingular, "status": "unknown",
        }),
    ))
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let input = ctx.read("docs/curves/registry.json")?;
    let mut report = Report::new(&input);
    let src = ctx.source(&input.rel_path, &input.sha256, "ICV1 curve registry").await?;
    let prov = ctx.provenance(src, "registry record (scripts/build_curve_registry.py)", None).await?;
    let prov_inv = ctx
        .provenance(
            src,
            "a-, b-, c-invariants, discriminant and j computed by ecdb from the model; j equal to the registry's",
            None,
        )
        .await?;
    let prov_class = ctx
        .provenance(src, "L(T) = 1 - tT + qT^2 from the registry trace; #E = q + 1 - t and the Hasse bound checked by ecdb; status set from the trait catalogue order_check", None)
        .await?;
    let prov_rep = ctx
        .provenance(
            src,
            "EC1 representation; field and curve digests recomputed from the canonical preimage by ecdb",
            None,
        )
        .await?;

    let curves = input.doc["curves"].as_array().ok_or_else(|| anyhow!("registry without curves"))?;
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    let mut classes: BTreeMap<String, Value> = BTreeMap::new();
    let (mut curve_rows, mut inv_rows, mut alias_rows, mut std_rows, mut fec1_rows) =
        (vec![], vec![], vec![], vec![], vec![]);
    let (mut rep_rows, mut point_rows, mut sub_rows, mut link_rows, mut zeta_rows) =
        (vec![], vec![], vec![], vec![], vec![]);
    let (mut gen_rows, mut evid_rows) = (vec![], vec![]);

    for c in curves {
        let slug = c["slug"].as_str().unwrap_or_default().to_string();
        let res: Result<()> = (|| {
            let model_json = c["model_json"].as_str().ok_or_else(|| anyhow!("no model_json"))?;
            let model: Map<String, Value> = serde_json::from_str(model_json)?;
            let model_sha = sha256_hex(model_json);
            if !slug.ends_with(&model_sha[..8]) {
                bail!("model JSON hash {} does not match the slug", &model_sha[..8]);
            }
            let spec = field_of_model(&model)?;
            let form = model.get("form").and_then(Value::as_str).unwrap_or_default();
            let (ecdb_model, names) = model_of_form(form)?;
            let mut coeff = Map::new();
            for n in names {
                let v = model.get(*n).ok_or_else(|| anyhow!("model lacks coefficient {n}"))?;
                coeff.insert((*n).to_string(), Value::String(canon_element(&spec.fq, v)?));
            }
            let field_id = spec.row["icv1_field"].as_str().unwrap_or_default().to_string();
            fields.entry(field_id.clone()).or_insert_with(|| spec.row.clone());

            let t: BigInt = s(&c["trace"]).and_then(|x| x.parse().ok()).ok_or_else(|| anyhow!("no trace"))?;
            let order: BigInt = s(&c["order"]).and_then(|x| x.parse().ok()).ok_or_else(|| anyhow!("no order"))?;
            if &spec.q + BigInt::one() - &t != order {
                bail!("order {order} is not q + 1 - t for t = {t}");
            }
            let (label, class) = class_row(&spec, &t)?;
            classes.entry(label.clone()).or_insert(class);

            curve_rows.push(json!({
                "field": field_id, "model": ecdb_model, "form": form, "coefficients": coeff,
                "equation": equation(ecdb_model, &coeff), "family": c["family"], "model_json": model_json,
                "model_sha256": model_sha, "icv1": c["icv1"], "slug": slug,
                "is_smooth": true, "smoothness_status": "proved",
            }));
            link_rows.push(json!({ "key": slug, "label": label }));
            zeta_rows.push(json!({ "key": slug, "point_counts": [order.to_string()] }));

            match invariant_row(&slug, &spec.fq, ecdb_model, &coeff, c["j"].as_str()) {
                Ok(mut row) => {
                    row["status"] = json!("proved");
                    inv_rows.push(row);
                }
                Err(e) => report.problem(format!("{slug}: invariants not recorded: {e}")),
            }

            alias_rows.push(json!({ "key": slug, "namespace": "icv1_slug", "alias": slug, "retired": false }));
            if let Some(icv1) = c["icv1"].as_str() {
                alias_rows.push(json!({ "key": slug, "namespace": "icv1", "alias": icv1, "retired": false }));
            }
            for a in c["aliases"].as_array().into_iter().flatten().filter_map(Value::as_str) {
                if a == slug {
                    continue;
                }
                let retired = is_retired_name(a);
                let ns = if retired { "retired" } else { "registry_alias" };
                alias_rows.push(json!({ "key": slug, "namespace": ns, "alias": a, "retired": retired }));
            }
            for a in c["standard_names"].as_array().into_iter().flatten().filter_map(Value::as_str) {
                alias_rows.push(json!({ "key": slug, "namespace": "standard", "alias": a, "retired": false }));
            }
            for sp in c["standards_provenance"].as_array().into_iter().flatten() {
                std_rows.push(json!({
                    "key": slug, "display_name": sp["display_name"], "category": sp["category"], "oid": sp["oid"],
                    "source_ref": sp["source_id"], "adoption_status": sp["adoption_status"],
                    "parameter_validation": sp["parameter_validation"], "sources": sp["sources"],
                }));
                if let Some(oid) = sp["oid"].as_str() {
                    alias_rows.push(json!({ "key": slug, "namespace": "oid", "alias": oid, "retired": false }));
                }
            }

            for rep in c["representations"].as_array().into_iter().flatten() {
                let field_rec = &rep["field"];
                let fcanon = canonical_json(field_rec)?;
                let fsha = sha256_hex(&fcanon);
                if Some(fsha.as_str()) != rep["field_sha256"].as_str() {
                    report.problem(format!("{slug}: EC1 field digest {fsha} != recorded {}", rep["field_sha256"]));
                    continue;
                }
                fec1_rows.push(
                    json!({ "field": field_id, "field_sha256": fsha, "canonical_json": fcanon, "record": field_rec }),
                );
                let uid = rep["curve_uid"].as_str().unwrap_or_default();
                let sha = uid.strip_prefix("urn:ec-record:1:sha256:").unwrap_or_default();
                let pre = ec1_preimage(field_rec, &rep["curve"])?;
                let preimage = if sha256_hex(&pre) == sha {
                    Some(pre)
                } else {
                    report.problem(format!("{slug}: EC1 preimage does not reproduce {uid}; recorded without preimage"));
                    None
                };
                let cur = &rep["curve"];
                let coords: Option<Vec<String>> = cur["generator"]
                    .as_array()
                    .map(|g| g.iter().filter_map(|x| s(x).and_then(|v| canon_str(&spec.fq, &v).ok())).collect());
                let coords = coords.filter(|v| v.len() == 2);
                rep_rows.push(json!({
                    "key": slug, "ec1": rep["ec1"], "uid": uid, "sha": sha, "field_sha256": fsha, "preimage": preimage,
                    "curve_record": cur, "model_label": cur["model"], "subgroup_order": s(&cur["subgroup_order"]),
                    "cofactor": s(&cur["cofactor"]), "generator": cur["generator"], "target_group": cur["target_group"],
                    "sources": rep["sources"],
                }));
                alias_rows.push(json!({ "key": slug, "namespace": "ec1", "alias": rep["ec1"], "retired": false }));
                if let Some(xy) = &coords {
                    point_rows
                        .push(json!({ "key": slug, "coordinates": xy, "order_value": s(&cur["subgroup_order"]) }));
                }
                sub_rows.push(json!({
                    "key": slug, "order_value": s(&cur["subgroup_order"]), "cofactor": s(&cur["cofactor"]),
                    "coordinates": coords, "uid": uid,
                    "is_prime": cur["target_group"].as_str() == Some("prime-order subgroup"),
                    "primality_status": "unknown", "status": "unknown",
                }));
            }

            let family = c["family"].as_str().unwrap_or_default();
            let call = c["params"]["generator_call"].as_str();
            let method = if c["standards_provenance"].is_array() {
                "standard_published"
            } else {
                match family {
                    "koblitz" => "koblitz",
                    "subfield" => "subfield",
                    _ if call.is_some() => "seeded_generator",
                    _ => "unknown",
                }
            };
            gen_rows.push(json!({ "key": slug, "method": method, "generator_call": call }));
            for src_path in c["sources"].as_array().into_iter().flatten().filter_map(Value::as_str) {
                evid_rows.push(
                    json!({ "key": slug, "kind": "recorded_in", "repository": ctx.repository, "path": src_path }),
                );
            }
            Ok(())
        })();
        if let Err(e) = res {
            report.problem(format!("{slug}: skipped: {e}"));
            report.count("curves_skipped", 1);
        }
    }

    let client = ctx.client;
    let field_rows: Vec<Value> = fields.into_values().collect();
    report.count("fields", bulk(client, FIELD_SQL, &field_rows, prov).await?);
    report.count("field_ec1_records", super::bulk1(client, FIELD_EC1_SQL, &fec1_rows).await?);
    report.count("curves", bulk(client, CURVE_SQL, &curve_rows, prov).await?);
    report.count("genus1_invariants", bulk(client, INVARIANT_SQL, &inv_rows, prov_inv).await?);
    report.count("aliases", super::bulk1(client, ALIAS_SQL, &alias_rows).await?);
    report.count("standards", bulk(client, STANDARD_SQL, &std_rows, prov).await?);
    let class_rows: Vec<Value> = classes.into_values().collect();
    report.count("isogeny_classes", bulk(client, CLASS_SQL, &class_rows, prov_class).await?);
    report.count("jacobians_linked", super::bulk1(client, LINK_CLASS_SQL, &link_rows).await?);
    report.count("zeta", bulk(client, ZETA_SQL, &zeta_rows, prov_class).await?);
    report.count("representations", bulk(client, REPRESENTATION_SQL, &rep_rows, prov_rep).await?);
    report.count("points", bulk(client, POINT_SQL, &point_rows, prov_rep).await?);
    report.count("subgroups", bulk(client, SUBGROUP_SQL, &sub_rows, prov_rep).await?);
    report.count("generation", bulk(client, GENERATION_SQL, &gen_rows, prov).await?);
    report.count("evidence_links", bulk(client, EVIDENCE_SQL, &evid_rows, prov).await?);
    Ok(report)
}
