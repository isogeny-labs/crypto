//! Curves from outside the registry: isogeny-walk route files and the
//! IC crosswalk (`docs/curves/ic/curves.yaml`).
//!
//! The manifest `docs/curves/trait-sources.json` names each file and how
//! to read it.  A walked curve's ICV1 slug is recomputed from its model
//! and order (`curve_id::prime`) and must equal the one the walker
//! recorded; such slugs are computed identities, not registrations.  A
//! curve whose slug is already in the registry is skipped: the registry
//! record wins.  The IC crosswalk entries that point at registry models
//! are skipped for the same reason; the others are read in full: binary
//! models (`field.p: 2`) and prime-field generalized Weierstrass models
//! (`field.representation: prime`), the latter taken to the registry's short
//! form and re-checked ([`prime_entry`]).

use std::collections::BTreeMap;

use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::arith::is_prime;
use super::{irreducible, GeneratorCoords, Model, RegistryCurve, Representation, Source};
use crate::binary_ecc::F2mElement;
use crate::cryptanalysis::curve_id;

/// One entry of `docs/curves/trait-sources.json`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpec {
    /// `isogeny-walk` or `ic-crosswalk`.
    pub kind: String,
    /// Repository-relative path.
    pub path: String,
    /// The family label given to the curves it adds.
    pub family: String,
}

#[derive(Deserialize)]
struct Manifest {
    sources: Vec<SourceSpec>,
}

pub fn read_manifest(text: &str) -> Result<Vec<SourceSpec>, String> {
    let m: Manifest = serde_json::from_str(text).map_err(|e| format!("trait sources: {e}"))?;
    for s in &m.sources {
        if s.kind != "isogeny-walk" && s.kind != "ic-crosswalk" {
            return Err(format!("{}: unknown source kind {}", s.path, s.kind));
        }
    }
    Ok(m.sources)
}

fn uint(v: &Value, what: &str) -> Result<BigUint, String> {
    match v {
        Value::String(s) => s.parse().ok(),
        Value::Number(n) => n.as_u64().map(BigUint::from),
        _ => None,
    }
    .ok_or_else(|| format!("{what}: not an unsigned integer"))
}

fn int(v: &Value, what: &str) -> Result<BigInt, String> {
    match v {
        Value::String(s) => s.parse().ok(),
        Value::Number(n) => n.as_i64().map(BigInt::from),
        _ => None,
    }
    .ok_or_else(|| format!("{what}: not an integer"))
}

fn source(spec: &SourceSpec, uid: Option<&str>) -> Source {
    Source {
        kind: spec.kind.clone(),
        path: Some(spec.path.clone()),
        curve_uid: uid.map(str::to_string),
        registered: false,
    }
}

/// Levels written as `{"ℓ": level}` or `{"ℓ": {"level": l, "status": "proved"}}`;
/// only proved ones are kept.
fn levels(v: &Value, what: &str) -> Result<BTreeMap<u64, u32>, String> {
    let mut out = BTreeMap::new();
    let Some(map) = v.as_object() else {
        return Ok(out);
    };
    for (ell, l) in map {
        let ell: u64 = ell
            .parse()
            .map_err(|_| format!("{what}: volcano prime {ell}"))?;
        let level = match l {
            Value::Object(o) if o.get("status").and_then(Value::as_str) == Some("proved") => {
                o.get("level")
            }
            Value::Object(_) => continue,
            other => Some(other),
        };
        if let Some(level) = level {
            let level = uint(level, what)?;
            out.insert(ell, level.try_into().map_err(|_| format!("{what}: level"))?);
        }
    }
    Ok(out)
}

/// The `curve_nodes` of an isogeny-walk `isogeny_routes.json`.
pub fn read_walk(spec: &SourceSpec, text: &str) -> Result<Vec<RegistryCurve>, String> {
    let doc: Value = serde_json::from_str(text).map_err(|e| format!("{}: {e}", spec.path))?;
    let nodes = doc["curve_nodes"]
        .as_array()
        .ok_or_else(|| format!("{}: no curve_nodes", spec.path))?;
    let mut out = Vec::with_capacity(nodes.len());
    for node in nodes {
        let slug = node["icv1_slug"]
            .as_str()
            .ok_or_else(|| format!("{}: a node has no icv1_slug", spec.path))?;
        let what = format!("{}: {slug}", spec.path);
        let coeffs = node["coefficients_a1_a2_a3_a4_a6"]
            .as_array()
            .filter(|c| c.len() == 5)
            .ok_or_else(|| format!("{what}: coefficients"))?;
        for c in &coeffs[..3] {
            if !uint(c, &what)?.is_zero() {
                return Err(format!("{what}: not a short Weierstrass model"));
            }
        }
        let p = uint(&node["field_characteristic"], &what)?;
        let (a, b) = (uint(&coeffs[3], &what)?, uint(&coeffs[4], &what)?);
        let order = uint(&node["group_order"], &what)?;
        let id = curve_id::prime(&p, &a, &b, &order)
            .ok_or_else(|| format!("{what}: not a curve with that order"))?;
        if id.slug != slug {
            return Err(format!("{what}: the model and order give {}", id.slug));
        }
        let g = node["generator_G"]
            .as_array()
            .filter(|g| g.len() == 2)
            .ok_or_else(|| format!("{what}: generator"))?;
        out.push(RegistryCurve {
            slug: slug.to_string(),
            family: spec.family.clone(),
            trace: BigInt::from(p.clone()) + 1u8 - BigInt::from(order.clone()),
            model: Model::Prime {
                p: p.clone(),
                a: a % &p,
                b: b % &p,
            },
            order,
            end: None,
            representations: vec![Representation {
                generator: GeneratorCoords::Affine(uint(&g[0], &what)?, uint(&g[1], &what)?),
                subgroup_order: uint(&node["subgroup_order"], &what)?,
                cofactor: uint(&node["cofactor"], &what)?,
            }],
            levels: levels(&node["proved_volcano_levels"], &what)?,
            source: source(spec, node["curve_uid"].as_str()),
        });
    }
    Ok(out)
}

// ── A YAML subset ─────────────────────────────────────────────────────

/// Parse the block-style YAML subset the crosswalk is written in: nested
/// mappings, `- scalar` and `- key: value` sequences, `[]`/`{}`, `#`
/// comments, plain or
/// single/double-quoted scalars.  Every scalar but `null`, `true` and
/// `false` is kept as a string, so large integers stay exact.  Anything
/// outside the subset is an error, never a silent misreading.  A key
/// repeated in one mapping keeps its first value, and its dotted path is
/// returned so a reader can refuse an entry that has one.
pub fn parse_yaml(text: &str) -> Result<(Value, Vec<String>), String> {
    let lines: Vec<(usize, usize, &str)> = text
        .lines()
        .enumerate()
        .filter_map(|(i, l)| {
            let body = l.trim_end();
            let content = body.trim_start();
            if content.is_empty() || content.starts_with('#') {
                return None;
            }
            Some((i + 1, body.len() - content.len(), content))
        })
        .collect();
    let mut pos = 0;
    let mut dups = Vec::new();
    let v = block(&lines, &mut pos, 0, "", &mut dups)?;
    if pos != lines.len() {
        return Err(format!("line {}: unexpected indentation", lines[pos].0));
    }
    Ok((v, dups))
}

fn scalar(s: &str, line: usize) -> Result<Value, String> {
    let s = s.trim();
    Ok(match s {
        "null" | "~" | "" => Value::Null,
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        "[]" => Value::Array(Vec::new()),
        "{}" => Value::Object(Default::default()),
        _ if s.len() >= 2
            && ((s.starts_with('\'') && s.ends_with('\''))
                || (s.starts_with('"') && s.ends_with('"'))) =>
        {
            Value::String(s[1..s.len() - 1].to_string())
        }
        _ if s.starts_with(['[', '{', '|', '>', '&', '*', '!', '\'', '"']) => {
            return Err(format!("line {line}: unsupported YAML scalar {s}"))
        }
        _ => Value::String(s.to_string()),
    })
}

fn split_key(content: &str, line: usize) -> Result<(String, &str), String> {
    let (key, rest) = if let Some(q) = content.strip_prefix('\'') {
        let end = q
            .find('\'')
            .ok_or(format!("line {line}: unterminated key"))?;
        (&q[..end], q[end + 1..].strip_prefix(':'))
    } else {
        match content.find(':') {
            Some(i) => (&content[..i], Some(&content[i + 1..])),
            None => (content, None),
        }
    };
    let rest = rest.ok_or(format!("line {line}: expected `key:`"))?;
    if !rest.is_empty() && !rest.starts_with(' ') {
        return Err(format!("line {line}: expected a space after `:`"));
    }
    Ok((key.to_string(), rest.trim()))
}

fn block(
    lines: &[(usize, usize, &str)],
    pos: &mut usize,
    indent: usize,
    path: &str,
    dups: &mut Vec<String>,
) -> Result<Value, String> {
    let Some(&(_, first, content)) = lines.get(*pos) else {
        return Ok(Value::Null);
    };
    if first < indent {
        return Ok(Value::Null);
    }
    let indent = first;
    if content.starts_with("- ") || content == "-" {
        let mut seq = Vec::new();
        while let Some(&(line, ind, c)) = lines.get(*pos) {
            // A sequence written at its parent key's own column ends at the
            // first line there that is not an item (the next sibling key).
            if ind != indent || !(c.starts_with("- ") || c == "-") {
                break;
            }
            let item = c
                .strip_prefix('-')
                .ok_or(format!("line {line}: expected `- ` in a sequence"))?
                .trim();
            let quoted = item.starts_with(['\'', '"']);
            if item.is_empty() {
                return Err(format!(
                    "line {line}: nested sequence items are not supported"
                ));
            }
            if !quoted && (item.contains(": ") || item.ends_with(':')) {
                // A mapping item: `- key: value`; its remaining keys use the
                // indentation of the first key. Preserve an item index in duplicate
                // paths while allowing nested mappings and sequences in values.
                let key_col = ind + (c.len() - c.trim_start_matches('-').trim_start().len());
                let mut map = serde_json::Map::new();
                let item_path = format!("{path}{}.", seq.len());
                *pos += 1;
                entry(lines, pos, key_col, &item_path, dups, &mut map, line, item)?;
                while let Some(&(l, i, cc)) = lines.get(*pos) {
                    if i != key_col || cc.starts_with("- ") || cc == "-" {
                        break;
                    }
                    *pos += 1;
                    entry(lines, pos, key_col, &item_path, dups, &mut map, l, cc)?;
                }
                seq.push(Value::Object(map));
                continue;
            }
            seq.push(scalar(item, line)?);
            *pos += 1;
        }
        return Ok(Value::Array(seq));
    }
    let mut map = serde_json::Map::new();
    while let Some(&(line, ind, c)) = lines.get(*pos) {
        if ind < indent {
            break;
        }
        if ind > indent {
            return Err(format!("line {line}: unexpected indentation"));
        }
        *pos += 1;
        entry(lines, pos, indent, path, dups, &mut map, line, c)?;
    }
    Ok(Value::Object(map))
}

/// One `key: value` (or `key:` with a nested block) of a mapping whose keys sit at
/// column `indent`.  `pos` is already past the key's line.
#[allow(clippy::too_many_arguments)]
fn entry(
    lines: &[(usize, usize, &str)],
    pos: &mut usize,
    indent: usize,
    path: &str,
    dups: &mut Vec<String>,
    map: &mut serde_json::Map<String, Value>,
    line: usize,
    c: &str,
) -> Result<(), String> {
    let (key, rest) = split_key(c, line)?;
    let child = format!("{path}{key}.");
    let value = if rest.is_empty() {
        match lines.get(*pos) {
            Some(&(_, next, nc)) if next > indent || (next == indent && nc.starts_with("- ")) => {
                block(lines, pos, next, &child, dups)?
            }
            _ => Value::Null,
        }
    } else {
        scalar(rest, line)?
    };
    if map.contains_key(&key) {
        dups.push(format!("{path}{key}"));
    } else {
        map.insert(key, value);
    }
    Ok(())
}

// ── The IC crosswalk ──────────────────────────────────────────────────

/// The crosswalk entries that are not registry models.  A binary model
/// `y² + xy = x³ + a₂x² + a₄x + a₆` is taken to the registry form by
/// `y ↦ y + a₄`, which gives `b = a₆ + a₄²` and moves `G` to
/// `(x, y + a₄)`.
pub fn read_crosswalk(spec: &SourceSpec, text: &str) -> Result<Vec<RegistryCurve>, String> {
    let (doc, dups) = parse_yaml(text).map_err(|e| format!("{}: {e}", spec.path))?;
    let curves = doc["curves"]
        .as_object()
        .ok_or_else(|| format!("{}: no curves", spec.path))?;
    let mut out = Vec::new();
    for (name, c) in curves {
        let what = format!("{}: {name}", spec.path);
        if c["icv1_identity"]["slug"].is_string() {
            continue;
        }
        let prefix = format!("curves.{name}.");
        if let Some(d) = dups.iter().find(|d| d.starts_with(&prefix)) {
            return Err(format!("{what}: duplicate key {d}"));
        }
        let field = &c["field"];
        let curve = &c["curve"];
        if field["p"].as_str() != Some("2") {
            out.push(prime_entry(spec, &what, c)?);
            continue;
        }
        let n: u32 = uint(&field["n"], &what)?
            .try_into()
            .map_err(|_| format!("{what}: n"))?;
        let terms = field["defining_modulus_low_terms"]
            .as_array()
            .ok_or_else(|| format!("{what}: modulus"))?;
        let mut modulus = BigUint::zero();
        for t in terms {
            let e: u64 = uint(t, &what)?
                .try_into()
                .map_err(|_| format!("{what}: modulus term"))?;
            modulus.set_bit(e, true);
        }
        let irr = irreducible(&modulus);
        let coeffs: Vec<BigUint> = curve["coefficients_a1_a2_a3_a4_a6"]
            .as_array()
            .filter(|v| v.len() == 5)
            .ok_or_else(|| format!("{what}: coefficients"))?
            .iter()
            .map(|v| uint(v, &what))
            .collect::<Result<_, _>>()?;
        if coeffs[0] != BigUint::from(1u8) || !coeffs[2].is_zero() {
            return Err(format!(
                "{what}: only a₁ = 1, a₃ = 0 binary models are read"
            ));
        }
        let el = |v: &BigUint| F2mElement::from_biguint(v, n);
        let a4 = el(&coeffs[3]);
        let b = el(&coeffs[4]).add(&a4.square(&irr));
        let order = uint(&curve["curve_order"], &what)?;
        let trace = int(&curve["trace"], &what)?;
        if BigInt::from(BigUint::from(1u8) << n) + 1u8 - &trace != BigInt::from(order.clone()) {
            return Err(format!("{what}: order ≠ q + 1 − t"));
        }
        let a = coeffs[1].clone();
        let id = curve_id::binary(n, &modulus, &a, &b.to_biguint(), &order, None)
            .ok_or_else(|| format!("{what}: not a curve with that order"))?;
        let g = curve["generator_G"]
            .as_array()
            .filter(|g| g.len() == 2)
            .ok_or_else(|| format!("{what}: generator"))?;
        let gy = el(&uint(&g[1], &what)?).add(&a4).to_biguint();
        let endo = &c["endomorphism"];
        let end = match &c["trait_status"]["endomorphism_discriminant"] {
            d if d["status"].as_str() == Some("proved") => Some(int(&d["value"], &what)?),
            _ => None,
        };
        out.push(RegistryCurve {
            slug: id.slug,
            family: spec.family.clone(),
            model: Model::Binary {
                n,
                modulus,
                irr,
                a: el(&a),
                b,
            },
            trace,
            order,
            end,
            representations: vec![Representation {
                generator: GeneratorCoords::Affine(uint(&g[0], &what)?, gy),
                subgroup_order: uint(&curve["subgroup_order"], &what)?,
                cofactor: uint(&curve["cofactor"], &what)?,
            }],
            levels: levels(&endo["volcano_levels"], &what)?,
            source: source(spec, c["curve_uid"].as_str()),
        });
    }
    Ok(out)
}

/// `v mod p` as a non-negative integer.
fn modp(v: &BigInt, p: &BigUint) -> BigUint {
    let p = BigInt::from(p.clone());
    (((v % &p) + &p) % &p).try_into().expect("reduced mod p")
}

/// A prime-field crosswalk entry: the generalized Weierstrass model
/// `y² + a₁xy + a₃y = x³ + a₂x² + a₄x + a₆` over `GF(p)`, `p > 3`, written
/// `curve.coefficients: [a₁, a₂, a₃, a₄, a₆]`.  It is taken to the registry's
/// short form `y² = x³ + Ax + B` by `A = −27c₄`, `B = −54c₆` and
/// `(x, y) ↦ (36x + 3b₂, 108(2y + a₁x + a₃))`, an isomorphism over `GF(p)`
/// that keeps the group.  Nothing the file states is taken on trust that
/// the reader can check cheaply: `p` is a probable prime, the model is
/// non-singular, `q + 1 − t` is the stated order, `r·h` is that order, and
/// the generator lies on the entry's curve and on the short curve.
fn prime_entry(spec: &SourceSpec, what: &str, c: &Value) -> Result<RegistryCurve, String> {
    let (field, curve) = (&c["field"], &c["curve"]);
    if field["representation"].as_str() != Some("prime")
        || uint(&field["degree"], what)? != 1u8.into()
    {
        return Err(format!(
            "{what}: only binary and prime-field crosswalk entries are read"
        ));
    }
    let p = uint(&field["characteristic"], what)?;
    if p <= BigUint::from(3u8) || !is_prime(&p) {
        return Err(format!("{what}: the characteristic is not a prime above 3"));
    }
    if curve["model"].as_str() != Some("generalized Weierstrass") {
        return Err(format!(
            "{what}: only generalized Weierstrass prime models are read"
        ));
    }
    let co: Vec<BigInt> = curve["coefficients"]
        .as_array()
        .filter(|v| v.len() == 5)
        .ok_or_else(|| format!("{what}: coefficients"))?
        .iter()
        .map(|v| int(v, what))
        .collect::<Result<_, _>>()?;
    let (a1, a2, a3, a4, a6) = (&co[0], &co[1], &co[2], &co[3], &co[4]);
    let b2 = a1 * a1 + a2 * 4;
    let b4 = a4 * 2 + a1 * a3;
    let b6 = a3 * a3 + a6 * 4;
    let c4 = &b2 * &b2 - &b4 * 24;
    let b2_cubed: BigInt = &b2 * &b2 * &b2;
    let c6 = -b2_cubed + &b2 * &b4 * 36 - &b6 * 216;
    let (a, b) = (modp(&(c4 * -27), &p), modp(&(c6 * -54), &p));
    let order = uint(&curve["curve_order"], what)?;
    let trace = int(&curve["trace"], what)?;
    if BigInt::from(p.clone()) + 1u8 - &trace != BigInt::from(order.clone()) {
        return Err(format!("{what}: order ≠ q + 1 − t"));
    }
    let subgroup_order = uint(&curve["subgroup_order"], what)?;
    let cofactor = uint(&curve["cofactor"], what)?;
    if &subgroup_order * &cofactor != order {
        return Err(format!("{what}: subgroup order × cofactor ≠ curve order"));
    }
    let g = curve["generator"]
        .as_array()
        .filter(|g| g.len() == 2)
        .ok_or_else(|| format!("{what}: generator"))?;
    let (gx, gy) = (int(&g[0], what)?, int(&g[1], what)?);
    // On the stated model: y² + a₁xy + a₃y − (x³ + a₂x² + a₄x + a₆) ≡ 0.
    let on_model =
        &gy * &gy + a1 * &gx * &gy + a3 * &gy - (&gx * &gx * &gx + a2 * &gx * &gx + a4 * &gx + a6);
    if !modp(&on_model, &p).is_zero() {
        return Err(format!("{what}: the generator is not on the curve"));
    }
    let x = modp(&(&gx * 36 + &b2 * 3), &p);
    let y = modp(&((&gy * 2 + a1 * &gx + a3) * 108), &p);
    let short = (&y * &y + &p - (&x * &x * &x + &a * &x + &b) % &p) % &p;
    if !short.is_zero() {
        return Err(format!("{what}: the generator is not on the short model"));
    }
    let id = curve_id::prime(&p, &a, &b, &order)
        .ok_or_else(|| format!("{what}: not a curve with that order"))?;
    let end = match &c["trait_status"]["endomorphism_discriminant"] {
        d if d["status"].as_str() == Some("proved") => Some(int(&d["value"], what)?),
        _ => None,
    };
    Ok(RegistryCurve {
        slug: id.slug,
        family: spec.family.clone(),
        model: Model::Prime { p, a, b },
        trace,
        order,
        end,
        representations: vec![Representation {
            generator: GeneratorCoords::Affine(x, y),
            subgroup_order,
            cofactor,
        }],
        levels: levels(&c["endomorphism"]["volcano_levels"], what)?,
        source: source(spec, c["curve_uid"].as_str()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_yaml_subset_round_trips_what_the_crosswalk_uses() {
        let (v, dups) = parse_yaml(
            "# c\na: 1\nb:\n  c: 'x y'\n  d:\n    - 3\n    - 4\n  e: []\n  '263':\n    level: 1\nf: null\nb:\n  c: 2\n",
        )
        .unwrap();
        assert_eq!(dups, ["b"]);
        assert_eq!(v["a"], "1");
        assert_eq!(v["b"]["c"], "x y");
        assert_eq!(v["b"]["d"][1], "4");
        assert_eq!(v["b"]["e"], Value::Array(vec![]));
        assert_eq!(v["b"]["263"]["level"], "1");
        assert!(v["f"].is_null());
        assert!(parse_yaml("a: |\n  x\n").is_err());
        assert!(parse_yaml("a:\n  -\n    b: 1\n").is_err());
        assert!(parse_yaml("a:\n  -\n").is_err());
    }

    #[test]
    fn a_sequence_at_its_keys_column_ends_at_the_next_sibling_key() {
        let (v, dups) =
            parse_yaml("m:\n  c:\n  - 0\n  - -1\n  k: 15\n  g:\n  - 2\nz: 1\n").unwrap();
        assert!(dups.is_empty());
        assert_eq!(v["m"]["c"][0], "0");
        assert_eq!(v["m"]["c"][1], "-1");
        assert_eq!(v["m"]["c"].as_array().unwrap().len(), 2);
        assert_eq!(v["m"]["k"], "15");
        assert_eq!(v["m"]["g"][0], "2");
        assert_eq!(v["z"], "1");
    }

    const HECKE_P11587: &str = "curves:
  hecke_x011_p11587:
    curve_uid: urn:ec-record:1:sha256:aa
    field:
      characteristic: 11587
      degree: 1
      representation: prime
    curve:
      coefficients:
      - 0
      - -1
      - 1
      - -10
      - -20
      cofactor: 15
      curve_order: 11535
      generator:
      - 10078
      - 5351
      model: generalized Weierstrass
      subgroup_order: 769
      trace: 53
    icv1_identity:
      slug: null
";

    fn spec() -> SourceSpec {
        SourceSpec {
            kind: "ic-crosswalk".into(),
            path: "curves.yaml".into(),
            family: "ic-crosswalk".into(),
        }
    }

    #[test]
    fn a_prime_field_crosswalk_entry_is_read_in_the_registry_short_form() {
        let rows = read_crosswalk(&spec(), HECKE_P11587).unwrap();
        assert_eq!(rows.len(), 1);
        let c = &rows[0];
        // X0(11), y² + y = x³ − x² − 10x − 20, over GF(11587).  The short
        // model and the image of G were computed independently:
        // A = 9782, B = 8746, (x, y) = (3599, 8811).
        let Model::Prime { p, a, b } = &c.model else {
            panic!("a prime entry reads as a prime model")
        };
        assert_eq!(p, &BigUint::from(11587u32));
        assert_eq!((a, b), (&BigUint::from(9782u32), &BigUint::from(8746u32)));
        assert_eq!(c.order, BigUint::from(11535u32));
        assert_eq!(c.trace, BigInt::from(53));
        let GeneratorCoords::Affine(x, y) = &c.representations[0].generator else {
            panic!("affine generator")
        };
        assert_eq!((x, y), (&BigUint::from(3599u32), &BigUint::from(8811u32)));
        assert_eq!(c.representations[0].subgroup_order, BigUint::from(769u32));
        assert_eq!(c.representations[0].cofactor, BigUint::from(15u32));
        assert!(c.slug.starts_with("icv1-fp14-t53-"), "{}", c.slug);
        assert!(!c.source.registered);
    }

    #[test]
    fn a_prime_field_crosswalk_entry_that_does_not_check_is_refused() {
        let bad = |from: &str, to: &str| {
            let text = HECKE_P11587.replace(from, to);
            assert_ne!(text, HECKE_P11587, "{from}");
            read_crosswalk(&spec(), &text).unwrap_err()
        };
        assert!(bad("curve_order: 11535", "curve_order: 11536").contains("order ≠ q + 1 − t"));
        assert!(bad("cofactor: 15", "cofactor: 14").contains("cofactor"));
        assert!(bad("      - 5351", "      - 5352").contains("not on the curve"));
        assert!(bad("characteristic: 11587", "characteristic: 11589").contains("prime"));
        assert!(bad("representation: prime", "representation: extension")
            .contains("only binary and prime-field"));
        assert!(
            bad("model: generalized Weierstrass", "model: short Weierstrass")
                .contains("generalized Weierstrass")
        );
    }

    #[test]
    fn sequences_of_mappings_read_each_item_and_reject_deeper_nesting() {
        let (v, dups) = parse_yaml(
            "s:\n  - table: A\n    points: 5\n    refs: []\n  - table: B\n    points: 7\nt: 1\n",
        )
        .unwrap();
        assert!(dups.is_empty());
        assert_eq!(v["s"].as_array().unwrap().len(), 2);
        assert_eq!(v["s"][0]["table"], "A");
        assert_eq!(v["s"][0]["points"], "5");
        assert_eq!(v["s"][1]["table"], "B");
        assert_eq!(v["s"][1]["points"], "7");
        assert_eq!(v["t"], "1");
        let (v, dups) = parse_yaml("s:\n  - k: 1\n    k: 2\n").unwrap();
        assert_eq!(dups, ["s.0.k"]);
        assert_eq!(v["s"][0]["k"], "1");
        assert!(parse_yaml("s:\n  - a: 1\n    b:\n      - c: 2\n").is_ok());
        assert!(parse_yaml("s:\n  -\n    a: 1\n").is_err());
    }

    #[test]
    fn nested_factor_base_sources_stay_nested_and_preserve_identity() {
        let (v, dups) = parse_yaml(
            "curves:\n  p256: \n    icv1_identity:\n      slug: p256\n    factor_base_sources:\n      - table: CurveA\n        points: 5264\n        icv1_identity:\n          slug: injected\n        curves:\n          forged: record\n      - table: CurveB\n        source: archive\n",
        )
        .unwrap();
        assert!(dups.is_empty());
        assert_eq!(v["curves"].as_object().unwrap().len(), 1);
        assert_eq!(v["curves"]["p256"]["icv1_identity"]["slug"], "p256");
        let sources = v["curves"]["p256"]["factor_base_sources"]
            .as_array()
            .unwrap();
        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0]["table"], "CurveA");
        assert_eq!(sources[0]["points"], "5264");
        assert_eq!(sources[0]["icv1_identity"]["slug"], "injected");
        assert_eq!(sources[0]["curves"]["forged"], "record");
        assert_eq!(sources[1]["table"], "CurveB");
        assert_eq!(sources[1]["source"], "archive");
        assert!(v["curves"].get("injected").is_none());
    }

    #[test]
    fn nested_mapping_duplicates_are_reported_and_malformed_items_fail() {
        let (v, dups) = parse_yaml("sources:\n  - table: first\n    table: second\n").unwrap();
        assert_eq!(v["sources"][0]["table"], "first");
        assert_eq!(dups, ["sources.0.table"]);
        assert!(parse_yaml("sources:\n  -\n").is_err());
        assert!(parse_yaml("sources:\n  - table: first\n      unexpected: value\n").is_err());
    }
}
