//! Facts determined by data already in the database or the repository,
//! computed exactly and checked where a check exists:
//!
//! * classical modular polynomials Phi_l, l <= 19, from
//!   src/cryptanalysis/curve_traits/modular_polynomials.txt (PARI/GP polmodular);
//! * Hilbert class polynomials H_D = x - j(D) for the 13 class-number-one
//!   discriminants, each checked against every stored curve whose End(E)
//!   has that discriminant (j(E) must be j(D) reduced into its field);
//! * Semaev summation polynomials S_2, S_3 for short Weierstrass models;
//! * division polynomials psi_2^2, psi_3, psi_4/psi_2, psi_5 of every
//!   genus-1 curve, from its stored b-invariants;
//! * GLV endomorphisms of j = 0 / j = 1728 curves over prime fields, the
//!   eigenvalue verified on the stored generator, with the GLV lattice;
//! * typed relations from the twist theorem: over F_q, curves with equal
//!   j != 0, 1728 are isomorphic when their traces agree and quadratic twists
//!   when the traces are opposite (t != 0);
//! * cyclic group structure where gcd(#E, q - 1) shows no l with l^2 | #E
//!   and l | q - 1, and the p-rank / a-number of genus-1 Jacobians.

use super::{bulk, bulk1, Ctx, Report};
use crate::ec::{glv_basis, modpow, root_of_unity, sqrt_mod, Curve, Point};
use crate::field::{Elt, Fq};
use crate::math::{canonical_json, sha256_hex};
use anyhow::{anyhow, bail, Result};
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub const MODULAR_FILE: &str = "src/cryptanalysis/curve_traits/modular_polynomials.txt";

/// Class-number-one discriminants and their j-invariants (recalled; checked
/// below against the stored curves with these endomorphism rings).
const CLASS_NUMBER_ONE: &[(i64, &str)] = &[
    (-3, "0"),
    (-4, "1728"),
    (-7, "-3375"),
    (-8, "8000"),
    (-11, "-32768"),
    (-12, "54000"),
    (-16, "287496"),
    (-19, "-884736"),
    (-27, "-12288000"),
    (-28, "16581375"),
    (-43, "-884736000"),
    (-67, "-147197952000"),
    (-163, "-262537412640768000"),
];

const POLY_SQL: &str = "
INSERT INTO ec.polynomial (sha256, coefficient_ring, coefficient_field_id, variables, representation, degrees, total_degree,
       term_count, max_coefficient_bits, coefficients, status, provenance_id)
SELECT r.sha, r.ring, r.field_id, r.variables, r.representation, r.degrees, r.total_degree, r.term_count, r.max_bits,
       r.coefficients, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(sha text, ring text, field_id bigint, variables text[], representation text,
     degrees int[], total_degree int, term_count bigint, max_bits int, coefficients jsonb, status text)
ON CONFLICT (sha256) DO NOTHING";

const MODULAR_SQL: &str = "
INSERT INTO ec.modular_polynomial (level, kind, field_id, polynomial_id)
SELECT r.level, 'classical', NULL, p.id
FROM jsonb_to_recordset($1::jsonb) AS r(level int, sha text)
JOIN ec.polynomial p ON p.sha256 = r.sha::bpchar
ON CONFLICT DO NOTHING";

const CLASS_SQL: &str = "
INSERT INTO ec.class_polynomial (discriminant, kind, field_id, class_number, polynomial_id)
SELECT r.d, 'hilbert', NULL, 1, p.id
FROM jsonb_to_recordset($1::jsonb) AS r(d numeric, sha text)
JOIN ec.polynomial p ON p.sha256 = r.sha::bpchar
ON CONFLICT DO NOTHING";

const SUMMATION_SQL: &str = "
INSERT INTO ec.summation_polynomial (model, m, curve_id, symmetrized, polynomial_id)
SELECT 'short_weierstrass', r.m, NULL, false, p.id
FROM jsonb_to_recordset($1::jsonb) AS r(m int, sha text)
JOIN ec.polynomial p ON p.sha256 = r.sha::bpchar
ON CONFLICT DO NOTHING";

const DIVISION_SQL: &str = "
INSERT INTO ec.division_polynomial (curve_id, n, variant, polynomial_id)
SELECT r.curve_id, r.n, r.variant, p.id
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, n int, variant text, sha text)
JOIN ec.polynomial p ON p.sha256 = r.sha::bpchar
ON CONFLICT DO NOTHING";

const GLV_ENDO_SQL: &str = "
INSERT INTO ec.endomorphism (curve_id, kind, name, degree, characteristic_polynomial, trace, norm, field_of_definition_id,
       is_separable, rational_map, formula, eigenvalue, subgroup_id, evaluation_cost, status, provenance_id)
SELECT c.id, 'glv', r.name, 1, r.char_poly, r.trace, 1, c.field_id, true, r.rational_map, r.formula, r.eigenvalue,
       r.subgroup_id, r.cost, 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, name text, char_poly numeric[], trace numeric,
     rational_map jsonb, formula text, eigenvalue numeric, subgroup_id bigint, cost jsonb)
JOIN ec.curve c ON c.id = r.curve_id
WHERE NOT EXISTS (SELECT 1 FROM ec.endomorphism e WHERE e.curve_id = c.id AND e.kind = 'glv')";

const GLV_LATTICE_SQL: &str = "
INSERT INTO ec.scalar_decomposition (curve_id, subgroup_id, endomorphism_ids, dimension, lattice_basis,
       max_component_bits, method, status, provenance_id)
SELECT r.curve_id, r.subgroup_id, ARRAY[e.id], 2, r.basis, r.max_bits,
       'extended Euclid on (r, lambda) stopped at sqrt(r) (Gallant-Lambert-Vanstone 2001)', 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, subgroup_id bigint, basis jsonb, max_bits int)
JOIN ec.endomorphism e ON e.curve_id = r.curve_id AND e.kind = 'glv'
WHERE r.basis IS NOT NULL
  AND NOT EXISTS (SELECT 1 FROM ec.scalar_decomposition d WHERE d.curve_id = r.curve_id)";

// Silverman AEC X.5.4 and X.2: for j != 0, 1728 the twists of E over F_q are
// E and its quadratic twist, of traces t and -t.
const RELATION_SQL: &str = "
WITH g AS (
  SELECT c.id, c.field_id, c.model, gi.j_invariant AS j, ic.trace AS t, ic.status
  FROM ec.curve c
  JOIN ec.genus1_invariants gi ON gi.curve_id = c.id
  JOIN ec.abelian_variety av ON av.curve_id = c.id
  JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
  WHERE c.genus = 1 AND NOT gi.j_is_0 AND NOT gi.j_is_1728 AND ic.status IN ('proved', 'probable')
)
INSERT INTO ec.curve_relation (source_curve_id, target_curve_id, kind, field_of_definition_id, map, status, provenance_id)
SELECT a.id, b.id,
       (CASE WHEN a.t = -b.t THEN 'quadratic_twist'
             WHEN a.model = b.model THEN 'isomorphism'
             ELSE 'birational_model_change' END)::ec.relation_kind,
       a.field_id,
       jsonb_build_object('explicit', false,
         'reason', CASE WHEN a.t = -b.t
                        THEN 'equal j (not 0 or 1728), traces t and -t: quadratic twists, isomorphic over the quadratic extension'
                        ELSE 'equal j (not 0 or 1728) and equal trace t != 0: isomorphic over the base field' END,
         'theorem', 'Silverman, AEC, X.5.4 (twists) and the trace of the quadratic twist'),
       (CASE WHEN a.status = 'proved' AND b.status = 'proved' THEN 'proved' ELSE 'probable' END)::ec.fact_status,
       $1::bigint
FROM g a JOIN g b ON a.field_id = b.field_id AND a.j = b.j AND a.id < b.id
WHERE a.t <> 0 AND (a.t = b.t OR a.t = -b.t)
ON CONFLICT DO NOTHING";

const GROUP_SQL: &str = "
INSERT INTO ec.group_structure (av_id, extension_degree, invariant_factors, order_value, is_cyclic, status, provenance_id)
SELECT r.av_id, 1, ARRAY[r.n]::numeric[], r.n, true, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(av_id bigint, n numeric, status text)
ON CONFLICT (av_id, extension_degree) DO NOTHING";

const PADIC_SQL: &str = "
INSERT INTO ec.p_adic_invariants (av_id, p_rank, a_number, is_superspecial, newton_polygon, status, provenance_id)
SELECT av.id, CASE WHEN ic.is_supersingular THEN 0 ELSE 1 END, CASE WHEN ic.is_supersingular THEN 1 ELSE 0 END,
       ic.is_supersingular,
       CASE WHEN ic.is_supersingular THEN '[\"1/2\", \"1/2\"]'::jsonb ELSE '[\"0\", \"1\"]'::jsonb END,
       ic.status, $1::bigint
FROM ec.abelian_variety av JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
WHERE av.dimension = 1 AND ic.is_supersingular IS NOT NULL
ON CONFLICT (av_id) DO NOTHING";

const CLASS_PRANK_SQL: &str = "
UPDATE ec.isogeny_class_fq SET p_rank = CASE WHEN is_supersingular THEN 0 ELSE 1 END,
       newton_slopes = CASE WHEN is_supersingular THEN ARRAY['1/2', '1/2'] ELSE ARRAY['0', '1'] END
WHERE g = 1 AND p_rank IS NULL AND is_supersingular IS NOT NULL";

/// One `ec.polynomial` row; the digest covers ring, field, variables and coefficients.
#[allow(clippy::too_many_arguments)]
fn poly_row(
    ring: &str,
    field: Option<(i64, &str)>,
    variables: &[&str],
    representation: &str,
    degrees: Vec<usize>,
    coefficients: Value,
    max_bits: Option<u64>,
    term_count: usize,
) -> Result<(String, Value)> {
    let sha = sha256_hex(&canonical_json(&json!({
        "ring": ring, "field": field.map(|f| f.1), "variables": variables,
        "representation": representation, "coefficients": coefficients,
    }))?);
    let total = degrees.iter().copied().max().unwrap_or(0);
    Ok((
        sha.clone(),
        json!({
            "sha": sha, "ring": ring, "field_id": field.map(|f| f.0), "variables": variables,
            "representation": representation, "degrees": degrees, "total_degree": total,
            "term_count": term_count, "max_bits": max_bits, "coefficients": coefficients, "status": "proved",
        }),
    ))
}

/// Sparse bivariate terms (i, j, c): c X^i Y^j.
pub type Bivariate = Vec<(usize, usize, BigInt)>;

/// Phi_l from the repository table: sparse [[i, j], c] over Z, both (i, j) and (j, i).
pub fn modular_polynomials(text: &str) -> Result<BTreeMap<u32, Bivariate>> {
    let mut out: BTreeMap<u32, Bivariate> = BTreeMap::new();
    for line in text.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() != 4 {
            bail!("bad modular-polynomial line {line:?}");
        }
        let (l, i, j): (u32, usize, usize) = (f[0].parse()?, f[1].parse()?, f[2].parse()?);
        let c: BigInt = f[3].parse()?;
        let e = out.entry(l).or_default();
        e.push((i, j, c.clone()));
        if i != j {
            e.push((j, i, c));
        }
    }
    for terms in out.values_mut() {
        terms.sort_by_key(|t| std::cmp::Reverse((t.0, t.1)));
    }
    Ok(out)
}

/// S_3 for y^2 = x^3 + a x + b (Semaev 2004), as sparse terms in (x1, x2, x3, a, b).
pub fn s3_terms() -> Vec<([u32; 5], i64)> {
    vec![
        ([2, 0, 2, 0, 0], 1),
        ([1, 1, 2, 0, 0], -2),
        ([0, 2, 2, 0, 0], 1),
        ([2, 1, 1, 0, 0], -2),
        ([1, 2, 1, 0, 0], -2),
        ([1, 0, 1, 1, 0], -2),
        ([0, 1, 1, 1, 0], -2),
        ([0, 0, 1, 0, 1], -4),
        ([2, 2, 0, 0, 0], 1),
        ([1, 1, 0, 1, 0], -2),
        ([0, 0, 0, 2, 0], 1),
        ([1, 0, 0, 0, 1], -4),
        ([0, 1, 0, 0, 1], -4),
    ]
}

/// S_3 evaluated over a field (tests and spot checks).
pub fn eval_s3(k: &Fq, x1: &Elt, x2: &Elt, x3: &Elt, a: &Elt, b: &Elt) -> Elt {
    let vars = [x1, x2, x3, a, b];
    s3_terms().into_iter().fold(k.zero(), |acc, (e, c)| {
        let mut t = k.from_int(c);
        for (v, &n) in vars.iter().zip(e.iter()) {
            for _ in 0..n {
                t = k.mul(&t, v);
            }
        }
        k.add(&acc, &t)
    })
}

/// (beta, lambda) with [lambda] P = (beta x, y) on the subgroup of `g` (j = 0).
pub fn glv_j0(e: &Curve, p: &BigInt, r: &BigInt, g: &Point) -> Result<(BigInt, BigInt)> {
    let Some((x, y)) = g else { bail!("generator is the point at infinity") };
    let beta = root_of_unity(3, p)?;
    let s = sqrt_mod(&BigInt::from(-3), r).ok_or_else(|| anyhow!("-3 is not a square mod r"))?;
    let half = modpow(&BigInt::from(2), &(r - 2u8), r);
    let lambdas = [((&s - 1u8) * &half).mod_floor(r), ((-&s - 1u8) * &half).mod_floor(r)];
    let k = e.k;
    for b in [beta.clone(), (&beta * &beta).mod_floor(p)] {
        let image: Point = Some((k.mul(x, &k.parse(&b.to_string())?), y.clone()));
        for l in &lambdas {
            if e.mul(l, g)? == image {
                return Ok((b, l.clone()));
            }
        }
    }
    bail!("no (beta, lambda) pair matches on the generator")
}

/// (i, lambda) with [lambda] P = (-x, i y) on the subgroup of `g` (j = 1728).
pub fn glv_j1728(e: &Curve, p: &BigInt, r: &BigInt, g: &Point) -> Result<(BigInt, BigInt)> {
    let Some((x, y)) = g else { bail!("generator is the point at infinity") };
    let i = root_of_unity(4, p)?;
    let s = sqrt_mod(&BigInt::from(-1), r).ok_or_else(|| anyhow!("-1 is not a square mod r"))?;
    let k = e.k;
    for ii in [i.clone(), (p - &i).mod_floor(p)] {
        let image: Point = Some((k.neg(x), k.mul(y, &k.parse(&ii.to_string())?)));
        for l in [s.clone(), (r - &s).mod_floor(r)] {
            if e.mul(&l, g)? == image {
                return Ok((ii, l));
            }
        }
    }
    bail!("no (i, lambda) pair matches on the generator")
}

/// Field arithmetic for every finite-field row, by id.
pub async fn fields(client: &tokio_postgres::Client) -> Result<BTreeMap<i64, (Fq, Option<String>)>> {
    let rows = client
        .query(
            "SELECT id, kind::text, characteristic::text, defining_polynomial::text[], modulus_hex, icv1_field
               FROM ec.field WHERE kind IN ('prime', 'binary', 'prime_power')",
            &[],
        )
        .await?;
    let mut out = BTreeMap::new();
    for r in rows {
        let (id, kind, p): (i64, String, String) = (r.get(0), r.get(1), r.get(2));
        let poly: Option<Vec<String>> = r.get(3);
        let modulus: Option<String> = r.get(4);
        let pb: BigInt = p.parse()?;
        let k = match kind.as_str() {
            "prime" => Fq::prime(pb),
            "binary" => {
                let m = modulus.ok_or_else(|| anyhow!("binary field {id} without modulus"))?;
                Fq::binary(
                    &BigUint::parse_bytes(m.trim_start_matches("0x").as_bytes(), 16)
                        .ok_or_else(|| anyhow!("bad modulus"))?,
                )
            }
            _ => {
                let poly = poly.ok_or_else(|| anyhow!("extension field {id} without defining polynomial"))?;
                let low: Vec<BigInt> = poly[..poly.len() - 1].iter().map(|c| c.parse()).collect::<Result<_, _>>()?;
                Fq::extension(pb, &low)
            }
        };
        out.insert(id, (k, r.get(5)));
    }
    Ok(out)
}

/// j(D) reduced into the field, in its canonical encoding.
fn reduce_integer(k: &Fq, n: &BigInt) -> String {
    let mut e = k.zero();
    e[0] = n.mod_floor(&k.p);
    k.format(&e)
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let text = super::read_text(&ctx.repo_root.join(MODULAR_FILE))?;
    let input = super::Input { rel_path: MODULAR_FILE.into(), sha256: sha256_hex(&text), doc: Value::Null };
    let mut report = Report::new(&input);
    report.sha256 = String::new();
    let client = ctx.client;
    let src_mod =
        ctx.source(MODULAR_FILE, &sha256_hex(&text), "Classical modular polynomials (PARI/GP polmodular)").await?;
    let prov_mod =
        ctx.provenance(src_mod, "PARI/GP polmodular(l), l <= 19, as stored for curve_traits walks", None).await?;
    let src = client
        .query_one(
            // source rows are keyed by (kind, repository, path, sha256, uri): the uri tells this one apart
            "WITH ins AS (INSERT INTO ec.source (kind, title, uri, provenance_class)
                          VALUES ('computation', 'ecdb enrichment', 'ecdb:enrich', 'internal') ON CONFLICT DO NOTHING RETURNING id)
             SELECT id FROM ins UNION ALL
             SELECT id FROM ec.source WHERE kind = 'computation' AND uri = 'ecdb:enrich' AND repository IS NULL LIMIT 1",
            &[],
        )
        .await?
        .get::<_, i64>(0);
    let prov_class = ctx
        .provenance(
            src,
            "class-number-one j-invariants (recalled); each checked against stored curves with that End(E)",
            None,
        )
        .await?;
    let prov_sum = ctx
        .provenance(src, "Semaev summation polynomials S_2, S_3 (2004), checked numerically in ecdb tests", None)
        .await?;
    let prov_div = ctx
        .provenance(src, "division polynomials from b-invariants (Silverman AEC Ex. 3.7); n = 2: psi_2^2, n = 3, 5: psi_n, n = 4: psi_4/psi_2", None)
        .await?;
    let prov_glv = ctx
        .provenance(src, "GLV endomorphism; eigenvalue verified by scalar multiplication on the stored generator", None)
        .await?;
    let prov_rel = ctx.provenance(src, "twist theorem over F_q from stored j-invariants and traces", None).await?;
    let prov_grp = ctx.provenance(src, "E(F_q) cyclic: no prime l with l | gcd(#E, q - 1) and l^2 | #E", None).await?;
    let prov_padic =
        ctx.provenance(src, "genus 1: p-rank 1 (ordinary) or 0 (supersingular); a-number 1 - p-rank", None).await?;

    // ---- modular polynomials
    let mut polys = vec![];
    let mut links = vec![];
    for (l, terms) in modular_polynomials(&text)? {
        let max_bits = terms.iter().map(|t| t.2.bits()).max();
        let coeffs: Vec<Value> = terms.iter().map(|(i, j, c)| json!([[i, j], c.to_string()])).collect();
        let n = coeffs.len();
        let (sha, row) = poly_row(
            "Z",
            None,
            &["X", "Y"],
            "sparse",
            vec![l as usize + 1, l as usize + 1],
            json!(coeffs),
            max_bits,
            n,
        )?;
        polys.push(row);
        links.push(json!({ "level": l, "sha": sha }));
    }
    report.count("polynomials_modular", bulk(client, POLY_SQL, &polys, prov_mod).await?);
    report.count("modular_polynomials", bulk1(client, MODULAR_SQL, &links).await?);

    // ---- class polynomials, checked against curves with those endomorphism rings
    let (mut polys, mut links) = (vec![], vec![]);
    for (d, j) in CLASS_NUMBER_ONE {
        let jb: BigInt = j.parse()?;
        let coeffs = json!([(-&jb).to_string(), "1"]);
        let (sha, row) = poly_row("Z", None, &["x"], "dense_univariate", vec![1], coeffs, Some(jb.bits()), 2)?;
        polys.push(row);
        links.push(json!({ "d": d.to_string(), "sha": sha }));
    }
    report.count("polynomials_class", bulk(client, POLY_SQL, &polys, prov_class).await?);
    report.count("class_polynomials", bulk1(client, CLASS_SQL, &links).await?);
    let fq = fields(client).await?;
    let ds: Vec<String> = CLASS_NUMBER_ONE.iter().map(|(d, _)| d.to_string()).collect();
    let rows = client
        .query(
            "SELECT c.field_id, er.discriminant::text, g.j_invariant FROM ec.endomorphism_ring er
               JOIN ec.curve c ON c.id = er.curve_id JOIN ec.genus1_invariants g ON g.curve_id = c.id
              WHERE er.discriminant::text = ANY ($1) AND er.status IN ('proved', 'probable')",
            &[&ds],
        )
        .await?;
    let (mut ok, mut bad) = (0u64, 0u64);
    for r in rows {
        let (fid, d, j): (i64, String, String) = (r.get(0), r.get(1), r.get(2));
        let Some((k, _)) = fq.get(&fid) else { continue };
        let jd: BigInt =
            CLASS_NUMBER_ONE.iter().find(|(x, _)| x.to_string() == d).map(|x| x.1).unwrap_or("0").parse()?;
        if reduce_integer(k, &jd) == j {
            ok += 1;
        } else {
            bad += 1;
            report.problem(format!("curve over field {fid} with End discriminant {d} has j {j}, not j({d}) reduced"));
        }
    }
    report.count("class_polynomial_curve_checks_passed", ok);
    report.count("class_polynomial_curve_checks_failed", bad);

    // ---- summation polynomials
    let s2 =
        poly_row("Z", None, &["x1", "x2"], "sparse", vec![1, 1], json!([[[1, 0], "1"], [[0, 1], "-1"]]), Some(1), 2)?;
    let s3_coeffs: Vec<Value> = s3_terms().iter().map(|(e, c)| json!([e, c.to_string()])).collect();
    let s3 = poly_row(
        "Z",
        None,
        &["x1", "x2", "x3", "a", "b"],
        "sparse",
        vec![2, 2, 2, 2, 1],
        json!(s3_coeffs),
        Some(3),
        13,
    )?;
    report.count("polynomials_summation", bulk(client, POLY_SQL, &[s2.1.clone(), s3.1.clone()], prov_sum).await?);
    report.count(
        "summation_polynomials",
        bulk1(client, SUMMATION_SQL, &[json!({ "m": 2, "sha": s2.0 }), json!({ "m": 3, "sha": s3.0 })]).await?,
    );

    // ---- division polynomials
    let rows = client
        .query(
            "SELECT c.id, c.field_id, g.b2, g.b4, g.b6, g.b8 FROM ec.curve c
               JOIN ec.genus1_invariants g ON g.curve_id = c.id
              WHERE c.genus = 1 AND g.b8 IS NOT NULL
                AND NOT EXISTS (SELECT 1 FROM ec.division_polynomial d WHERE d.curve_id = c.id)",
            &[],
        )
        .await?;
    let (mut polys, mut links) = (vec![], vec![]);
    for r in rows {
        let (cid, fid): (i64, i64) = (r.get(0), r.get(1));
        let Some((k, fkey)) = fq.get(&fid) else { continue };
        let fkey = fkey.clone().unwrap_or_else(|| format!("field:{fid}"));
        let b: Vec<Elt> = (2..6).map(|i| k.parse(&r.get::<_, String>(i))).collect::<Result<_>>()?;
        let d = crate::ec::division_polynomials(k, &b[0], &b[1], &b[2], &b[3]);
        for (n, variant, p) in [
            (2, "psi_squared", &d.psi2_squared),
            (3, "psi", &d.psi3),
            (4, "f_x_only", &d.psi4_over_psi2),
            (5, "psi", &d.psi5),
        ] {
            let coeffs: Vec<String> = p.iter().map(|c| k.format(c)).collect();
            let (sha, row) = poly_row(
                "field",
                Some((fid, &fkey)),
                &["x"],
                "dense_univariate",
                vec![coeffs.len() - 1],
                json!(coeffs),
                None,
                p.len(),
            )?;
            polys.push(row);
            links.push(json!({ "curve_id": cid, "n": n, "variant": variant, "sha": sha }));
        }
    }
    report.count("polynomials_division", bulk(client, POLY_SQL, &polys, prov_div).await?);
    report.count("division_polynomials", bulk1(client, DIVISION_SQL, &links).await?);

    // ---- GLV endomorphisms (prime fields, j = 0 or 1728 with the automorphism defined over F_p)
    let rows = client
        .query(
            "SELECT c.id, c.field_id, f.characteristic::text, c.coefficients->>'a', c.coefficients->>'b', gi.j_is_0,
                    s.id, s.order_value::text, p.coordinates
               FROM ec.curve c
               JOIN ec.field f ON f.id = c.field_id AND f.kind = 'prime'
               JOIN ec.genus1_invariants gi ON gi.curve_id = c.id
               JOIN ec.automorphism_group ag ON ag.curve_id = c.id AND ag.group_order IN (4, 6)
               JOIN ec.abelian_variety av ON av.curve_id = c.id
               LEFT JOIN LATERAL (SELECT s.* FROM ec.subgroup s WHERE s.av_id = av.id AND s.field_id = c.field_id
                                   AND s.is_prime_order AND s.generator_point_id IS NOT NULL
                                  ORDER BY s.order_value DESC LIMIT 1) s ON true
               LEFT JOIN ec.point p ON p.id = s.generator_point_id
              WHERE c.model = 'short_weierstrass'",
            &[],
        )
        .await?;
    let (mut endos, mut lattices) = (vec![], vec![]);
    for r in rows {
        let (cid, fid): (i64, i64) = (r.get(0), r.get(1));
        let p: BigInt = r.get::<_, String>(2).parse()?;
        let j0: bool = r.get(5);
        let Some((k, _)) = fq.get(&fid) else { continue };
        let a = [k.zero(), k.zero(), k.zero(), k.parse(&r.get::<_, String>(3))?, k.parse(&r.get::<_, String>(4))?];
        let e = Curve { k, a };
        let sub: Option<i64> = r.get(6);
        let order: Option<String> = r.get(7);
        let coords: Option<Value> = r.get(8);
        let g: Option<Point> = coords.as_ref().and_then(|c| {
            let x = k.parse(c.get(0)?.as_str()?).ok()?;
            let y = k.parse(c.get(1)?.as_str()?).ok()?;
            Some(Some((x, y)))
        });
        let mut eigen: Option<(BigInt, BigInt, BigInt)> = None; // (root, lambda, r)
        if let (Some(g), Some(order)) = (&g, &order) {
            let rr: BigInt = order.parse()?;
            if e.on_curve(g) && rr > BigInt::from(3) {
                let res = if j0 { glv_j0(&e, &p, &rr, g) } else { glv_j1728(&e, &p, &rr, g) };
                match res {
                    Ok((root, l)) => eigen = Some((root, l, rr)),
                    Err(err) => report.problem(format!("curve {cid}: GLV eigenvalue not found: {err}")),
                }
            }
        }
        let root = match &eigen {
            Some((root, _, _)) => root.clone(),
            None => root_of_unity(if j0 { 3 } else { 4 }, &p)?,
        };
        let (name, formula, char_poly, trace, map) = if j0 {
            (
                "zeta_3 automorphism",
                format!("(x, y) -> (beta x, y), beta = {root}"),
                json!(["1", "1", "1"]),
                "-1",
                json!({ "x": "beta*x", "y": "y", "beta": root.to_string() }),
            )
        } else {
            (
                "zeta_4 automorphism",
                format!("(x, y) -> (-x, i y), i = {root}"),
                json!(["1", "0", "1"]),
                "0",
                json!({ "x": "-x", "y": "i*y", "i": root.to_string() }),
            )
        };
        endos.push(json!({
            "curve_id": cid, "name": name, "char_poly": char_poly, "trace": trace, "rational_map": map,
            "formula": formula, "eigenvalue": eigen.as_ref().map(|t| t.1.to_string()),
            "subgroup_id": if eigen.is_some() { sub } else { None },
            "cost": { "field_multiplications": 1 },
        }));
        if let Some((_, l, rr)) = &eigen {
            let [v1, v2] = glv_basis(rr, l);
            let max_bits = [&v1.0, &v1.1, &v2.0, &v2.1].iter().map(|n| n.abs().bits()).max();
            lattices.push(json!({
                "curve_id": cid, "subgroup_id": sub, "max_bits": max_bits,
                "basis": [[v1.0.to_string(), v1.1.to_string()], [v2.0.to_string(), v2.1.to_string()]],
            }));
        }
    }
    report.count("glv_endomorphisms", bulk(client, GLV_ENDO_SQL, &endos, prov_glv).await?);
    report.count("glv_lattices", bulk(client, GLV_LATTICE_SQL, &lattices, prov_glv).await?);

    // ---- typed relations
    report.count("curve_relations", client.execute(RELATION_SQL, &[&prov_rel]).await?);

    // ---- cyclic group structure
    let rows = client
        .query(
            "SELECT av.id, f.order_q::text, ic.order_value::text, ic.status::text, s.primality_status::text
               FROM ec.abelian_variety av
               JOIN ec.curve c ON c.id = av.curve_id
               JOIN ec.field f ON f.id = c.field_id
               JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
               LEFT JOIN LATERAL (SELECT s.primality_status FROM ec.subgroup s WHERE s.av_id = av.id
                                   AND s.is_prime_order ORDER BY s.order_value DESC LIMIT 1) s ON true
              WHERE av.dimension = 1 AND ic.status IN ('proved', 'probable')
                AND NOT EXISTS (SELECT 1 FROM ec.group_structure gs WHERE gs.av_id = av.id AND gs.extension_degree = 1)",
            &[],
        )
        .await?;
    let mut groups = vec![];
    let mut undecided = 0u64;
    for r in rows {
        let q: BigInt = r.get::<_, String>(1).parse()?;
        let n: BigInt = r.get::<_, String>(2).parse()?;
        let g = n.gcd(&(&q - 1u8));
        match cyclic_by_gcd(&n, &g) {
            Some(true) => {
                let st: String = r.get(3);
                let status = if st == "proved" { "proved" } else { "probable" };
                groups.push(json!({ "av_id": r.get::<_, i64>(0), "n": n.to_string(), "status": status }));
            }
            _ => undecided += 1,
        }
    }
    report.count("group_structures_cyclic", bulk(client, GROUP_SQL, &groups, prov_grp).await?);
    report.count("group_structures_undecided", undecided);

    // ---- p-adic invariants
    report.count("p_adic_invariants", client.execute(PADIC_SQL, &[&prov_padic]).await?);
    report.count("class_p_rank_updates", client.execute(CLASS_PRANK_SQL, &[]).await?);
    Ok(report)
}

/// E(F_q) = Z/n1 x Z/n2 with n1 | n2 and n1 | q - 1, so it is cyclic when no
/// prime l divides g = gcd(n, q - 1) with l^2 | n. Some(true): cyclic;
/// Some(false): such an l exists (structure undecided here); None: g did not
/// factor by trial division.
pub fn cyclic_by_gcd(n: &BigInt, g: &BigInt) -> Option<bool> {
    let mut m = g.clone();
    let mut d = BigInt::from(2);
    let limit = BigInt::from(10_000_000u64);
    while &d * &d <= m {
        if d > limit {
            return None; // an unfactored part with no factor below the limit
        }
        if (&m % &d).is_zero() {
            if (n % (&d * &d)).is_zero() {
                return Some(false);
            }
            while (&m % &d).is_zero() {
                m /= &d;
            }
        }
        d += 1;
    }
    // what is left of m is 1 or a prime
    Some(m.is_one() || !(n % (&m * &m)).is_zero())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modular_polynomial_phi2_known_coefficients() {
        let text = include_str!("../../../src/cryptanalysis/curve_traits/modular_polynomials.txt");
        let phi = modular_polynomials(text).unwrap();
        assert_eq!(phi.keys().copied().collect::<Vec<_>>(), vec![2, 3, 5, 7, 11, 13, 17, 19]);
        let p2 = &phi[&2];
        // Phi_2 = X^3 + Y^3 - X^2 Y^2 + 1488 X^2 Y + 1488 X Y^2 - 162000 X^2 - 162000 Y^2 + ...
        assert!(p2.contains(&(3, 0, BigInt::from(1))) && p2.contains(&(0, 3, BigInt::from(1))));
        assert!(p2.contains(&(2, 2, BigInt::from(-1))));
        assert!(p2.contains(&(1, 2, BigInt::from(1488))) && p2.contains(&(2, 1, BigInt::from(1488))));
    }

    #[test]
    fn modular_polynomial_relates_isogenous_j() {
        // Phi_2(j(E), j(E')) = 0 for 2-isogenous E, E' : check on j = 1728 (CM by Z[i]),
        // which is 2-isogenous to j = 287496 (CM by Z[2i]): Phi_2(1728, 287496) = 0 over Z.
        let text = include_str!("../../../src/cryptanalysis/curve_traits/modular_polynomials.txt");
        let phi = modular_polynomials(text).unwrap();
        let (x, y) = (BigInt::from(1728), BigInt::from(287496));
        let v: BigInt =
            phi[&2].iter().map(|(i, j, c)| c * num_traits::pow(x.clone(), *i) * num_traits::pow(y.clone(), *j)).sum();
        assert!(v.is_zero());
    }

    #[test]
    fn cyclic_test() {
        let n = BigInt::from(4 * 29);
        assert_eq!(cyclic_by_gcd(&n, &BigInt::from(1)), Some(true));
        // n = 4 * 29 with 2 | q - 1 -> 2^2 | n: undecided
        assert_eq!(cyclic_by_gcd(&n, &BigInt::from(2)), Some(false));
        assert_eq!(cyclic_by_gcd(&BigInt::from(2 * 29), &BigInt::from(2)), Some(true));
    }
}
