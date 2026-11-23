//! Fixed-registry saved-output reader and its frozen mutation panel.
//! The extractor is never invoked from this module.
use crate::{
    digest,
    extractor::{Extraction, Input},
    independent,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::{fs, path::Path};

type Files = BTreeMap<String, Vec<u8>>;
const SAMPLE: &str = "order4-d1-minus-all";
const SUBGROUP: &str = "sub5-d96-plus-liftable";
const TORSION: &str = "rank97-d2-x1-all";
const PROTOCOL: &[u8] = include_bytes!("../PROTOCOL.md");

fn filename(id: &str, side: &str) -> String {
    format!("{id}.{side}.json")
}

fn load(path: &Path) -> Result<Files, String> {
    let mut files = Files::new();
    for entry in fs::read_dir(path).map_err(|e| format!("read directory: {e}"))? {
        let entry = entry.map_err(|e| format!("directory entry: {e}"))?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| "non-UTF8 artifact name")?;
        let metadata = fs::symlink_metadata(entry.path()).map_err(|e| format!("metadata: {e}"))?;
        if !metadata.file_type().is_file() || metadata.len() > 65536 {
            return Err("artifacts must be ordinary files within the fixed byte cap".into());
        }
        let bytes = fs::read(entry.path()).map_err(|e| format!("read artifact: {e}"))?;
        if files.insert(name, bytes).is_some() {
            return Err("duplicate artifact filename".into());
        }
    }
    Ok(files)
}

/// Direct typed parsing rejects duplicate declared fields; roundtrip equality
/// also rejects unknown nested fields in the unchanged predecessor certificate.
fn parse_exact<T: DeserializeOwned + Serialize>(bytes: &[u8]) -> Result<T, String> {
    let parsed: T = serde_json::from_slice(bytes).map_err(|e| format!("typed JSON: {e}"))?;
    let value: Value = serde_json::from_slice(bytes).map_err(|e| format!("JSON value: {e}"))?;
    if serde_json::to_value(&parsed).map_err(|e| e.to_string())? != value {
        return Err("JSON carries unknown fields or omits explicit contract fields".into());
    }
    Ok(parsed)
}
fn string<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {key}"))
}
fn integers(value: &Value, key: &str, len: usize) -> Result<Vec<u64>, String> {
    let values = value
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing array {key}"))?;
    if values.len() != len {
        return Err(format!("wrong {key} array length"));
    }
    values
        .iter()
        .map(|v| v.as_u64().ok_or_else(|| format!("noninteger in {key}")))
        .collect()
}
fn hashes(value: &Value) -> Result<Vec<&str>, String> {
    let values = value
        .get("h")
        .and_then(Value::as_array)
        .ok_or("missing hash array")?;
    if values.len() != 2 {
        return Err("wrong hash array length".into());
    }
    values
        .iter()
        .map(|v| {
            let hash = v.as_str().ok_or("nonstring hash")?;
            if hash.len() != 64
                || !hash
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            {
                return Err("malformed SHA-256 digest".into());
            }
            Ok(hash)
        })
        .collect()
}

fn parse_raw(bytes: &[u8]) -> Result<BTreeMap<String, Value>, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| format!("raw UTF8: {e}"))?;
    let fixtures = independent::registry()?;
    let mut rows = BTreeMap::new();
    let mut scopes = 0;
    let mut summaries = 0;
    let mut refusal_ids = BTreeSet::new();
    let expected_refusals: BTreeSet<_> = [
        "unsupported_prime",
        "invalid_prime",
        "noncanonical_a",
        "noncanonical_b",
        "singular_curve",
        "zero_degree",
        "nondivisor_degree",
        "overdegree",
        "noncanonical_target_x",
        "noncanonical_target_y",
        "off_curve_target",
        "subgroup_nonminimal_order",
        "subgroup_wrong_order",
        "subgroup_unit_order",
        "subgroup_cap",
        "subgroup_off_curve_generator",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    for (line_number, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let value: Value =
            serde_json::from_str(line).map_err(|e| format!("raw line {}: {e}", line_number + 1))?;
        match string(&value, "kind")? {
            "scope" => {
                scopes += 1;
                if value.get("cases").and_then(Value::as_u64) != Some(132)
                    || value.get("persisted").and_then(Value::as_u64) != Some(42)
                    || string(&value, "protocol_sha256")? != digest(PROTOCOL)
                    || string(&value, "mode")? != "finite_native_correctness"
                    || value.get("timing_claim").and_then(Value::as_bool) != Some(false)
                {
                    return Err("raw scope differs from compiled fixed protocol".into());
                }
            }
            "case" => {
                let id = string(&value, "id")?.to_owned();
                let fixture = fixtures.get(&id).ok_or("unknown raw case")?;
                if value.get("persist").and_then(Value::as_bool)
                    != Some(independent::persisted(fixture))
                {
                    return Err("raw persistence policy differs from independent registry".into());
                }
                hashes(&value)?;
                let sizes = integers(&value, "b", 2)?;
                if sizes[0] == 0 || sizes[0] > 32 * 1024 * 1024 || sizes[1] == 0 || sizes[1] > 65536
                {
                    return Err("raw artifact sizes outside fixed caps".into());
                }
                if independent::persisted(fixture) && sizes[0] > 65536 {
                    return Err("persisted proof exceeds fixed cap".into());
                }
                integers(&value, "n", 11)?;
                integers(&value, "c", 15)?;
                integers(&value, "k", 6)?;
                if rows.insert(id, value).is_some() {
                    return Err("duplicate raw case".into());
                }
            }
            "refusal" => {
                let id = string(&value, "id")?.to_owned();
                if !expected_refusals.contains(&id)
                    || !refusal_ids.insert(id)
                    || value.get("error").is_none_or(Value::is_null)
                {
                    return Err("raw refusal inventory differs".into());
                }
            }
            "summary" => {
                summaries += 1;
                if value.get("passed").and_then(Value::as_bool) != Some(true)
                    || value.get("refused_inputs").and_then(Value::as_u64) != Some(16)
                    || value.pointer("/totals/cases").and_then(Value::as_u64) != Some(132)
                    || value.pointer("/totals/persisted").and_then(Value::as_u64) != Some(42)
                {
                    return Err("raw completion summary differs".into());
                }
            }
            _ => return Err("unknown raw record kind".into()),
        }
    }
    if scopes != 1
        || summaries != 1
        || refusal_ids != expected_refusals
        || rows.keys().ne(fixtures.keys())
    {
        return Err("raw stream is incomplete or has a different fixed case set".into());
    }
    Ok(rows)
}

fn checked_counts(output: &Extraction) -> Vec<u64> {
    let s = &output.stats;
    [
        output.base.len(),
        output.certificate.f.len() - 1,
        output.certificate.g.len() - 1,
        output.certificate.support_count,
        output.triples.len(),
        s.affine_triples,
        s.identity_prefix_triples,
        output.permutations.len(),
        output.rows.len(),
        s.inverse_pair_triples,
        s.masked_noncancelling_triples,
    ]
    .into_iter()
    .map(|x| x as u64)
    .collect()
}
fn diagnostic_counts(output: &Extraction) -> Vec<u64> {
    let s = &output.stats;
    [
        s.selector_pow_calls,
        s.square_root_calls,
        s.subgroup_additions,
        s.right_polynomial_products,
        s.tensor_ring_multiplications,
        s.scalar_norm_factors,
        s.positive_block_root_tests,
        s.positive_fibre_root_tests,
        s.left_gcd_queries,
        s.right_gcd_queries,
        s.left_lift_pair_attempts,
        s.prefix_additions,
        s.residual_subtractions,
        s.base_membership_queries,
        s.replay_additions,
    ]
    .into_iter()
    .map(|x| x as u64)
    .collect()
}

/// Shared core for disk reading and all in-memory mutations.
fn check_loaded(files: &Files, raw: Option<&[u8]>) -> Result<Value, String> {
    let all_fixtures = independent::registry()?;
    let fixtures: BTreeMap<_, _> = all_fixtures
        .into_iter()
        .filter(|(_, s)| independent::persisted(s))
        .collect();
    if fixtures.len() != 42 {
        return Err("independent persisted registry cardinality".into());
    }
    let expected_files: BTreeSet<_> = fixtures
        .keys()
        .flat_map(|id| [filename(id, "certificate"), filename(id, "input")])
        .collect();
    if files.keys().cloned().collect::<BTreeSet<_>>() != expected_files {
        return Err("artifact filenames differ from the exact 42-pair registry".into());
    }
    let raw_rows = raw.map(parse_raw).transpose()?;
    let mut checked = Vec::new();
    let mut totals = [0u64; 6];
    for (id, fixture) in fixtures {
        let output_bytes = &files[&filename(&id, "certificate")];
        let input_bytes = &files[&filename(&id, "input")];
        if output_bytes.len() > 65536 || input_bytes.len() > 65536 {
            return Err("artifact byte cap exceeded".into());
        }
        let output_hash = digest(output_bytes);
        let input_hash = digest(input_bytes);
        if let Some(rows) = &raw_rows {
            let row = &rows[&id];
            let expected_hashes = hashes(row)?;
            let expected_sizes = integers(row, "b", 2)?;
            if expected_hashes != vec![output_hash.as_str(), input_hash.as_str()]
                || expected_sizes != vec![output_bytes.len() as u64, input_bytes.len() as u64]
            {
                return Err(format!("exact artifact byte binding fails for {id}"));
            }
        }
        let input: Input = parse_exact(input_bytes)?;
        if independent::from_input(&input) != fixture {
            return Err(format!("input registry binding fails for {id}"));
        }
        let output: Extraction = parse_exact(output_bytes)?;
        let comparison = independent::compare(&input, &output)?;
        let keys = [
            "pairs",
            "grid",
            "identities",
            "triples",
            "prefixO",
            "masked_noncc",
        ];
        let values: Vec<_> = keys
            .into_iter()
            .map(|key| {
                comparison[key]
                    .as_u64()
                    .ok_or_else(|| format!("missing independent count {key}"))
            })
            .collect::<Result<_, _>>()?;
        if let Some(rows) = &raw_rows {
            let row = &rows[&id];
            if integers(row, "n", 11)? != checked_counts(&output)
                || integers(row, "c", 15)? != diagnostic_counts(&output)
                || integers(row, "k", 6)? != values
            {
                return Err(format!(
                    "raw counts differ from bound retained object for {id}"
                ));
            }
        }
        for i in 0..6 {
            totals[i] += values[i];
        }
        checked.push(json!({"id":id,"h":[output_hash,input_hash],"k":values}));
    }
    Ok(
        json!({"kind":"signed_reader","cases":42,"raw_case_count":raw_rows.as_ref().map_or(0,BTreeMap::len),
        "registry_binding":true,"exact_byte_binding":raw.is_some(),"raw_sha256":raw.map(digest),
        "protocol_sha256":digest(PROTOCOL),"pairs":totals[0],"grid_cells":totals[1],
        "certificate_identity_checks":totals[2],"ordered_triples":totals[3],"identity_prefix_triples":totals[4],
        "masked_noncancelling_triples":totals[5],"extractor_invoked":false,"norm_constructor_invoked":false,
        "quotient_solver_invoked":false,"arithmetic_cost_counts_independently_priced":false,"checks":checked}),
    )
}

pub fn check_directory(path: &Path, raw: Option<&Path>) -> Result<Value, String> {
    let files = load(path)?;
    let raw = raw
        .map(fs::read)
        .transpose()
        .map_err(|e| format!("read raw: {e}"))?;
    check_loaded(&files, raw.as_deref())
}

fn edit(
    files: &mut Files,
    id: &str,
    side: &str,
    mutate: impl FnOnce(&mut Value) -> Result<(), String>,
) -> Result<(), String> {
    let key = filename(id, side);
    let mut value: Value =
        serde_json::from_slice(files.get(&key).ok_or("missing control artifact")?)
            .map_err(|e| e.to_string())?;
    mutate(&mut value)?;
    files.insert(key, serde_json::to_vec(&value).map_err(|e| e.to_string())?);
    Ok(())
}
fn array<'a>(value: &'a mut Value, pointer: &str) -> Result<&'a mut Vec<Value>, String> {
    value
        .pointer_mut(pointer)
        .and_then(Value::as_array_mut)
        .ok_or_else(|| format!("missing control array {pointer}"))
}
fn remove_first(value: &mut Value, pointer: &str) -> Result<(), String> {
    let values = array(value, pointer)?;
    if values.is_empty() {
        return Err(format!("control array empty {pointer}"));
    }
    values.remove(0);
    Ok(())
}
fn set(value: &mut Value, pointer: &str, new: Value) -> Result<(), String> {
    *value
        .pointer_mut(pointer)
        .ok_or_else(|| format!("missing control field {pointer}"))? = new;
    Ok(())
}
fn bump(value: &mut Value, pointer: &str) -> Result<(), String> {
    let old = value
        .pointer(pointer)
        .and_then(Value::as_u64)
        .ok_or("missing numeric control field")?;
    set(value, pointer, json!(old + 1))
}
fn omit_triples(
    value: &mut Value,
    keep: impl Fn(usize, &Value, &Value) -> bool,
) -> Result<(), String> {
    let triples = value
        .get("triples")
        .and_then(Value::as_array)
        .ok_or("missing triples")?
        .clone();
    let routes = value
        .get("routes")
        .and_then(Value::as_array)
        .ok_or("missing routes")?
        .clone();
    if triples.len() != routes.len() {
        return Err("control route shape".into());
    }
    let mut kept_triples = Vec::new();
    let mut kept_routes = Vec::new();
    for (i, (triple, route)) in triples.iter().zip(&routes).enumerate() {
        if keep(i, triple, route) {
            kept_triples.push(triple.clone());
            kept_routes.push(route.clone());
        }
    }
    if kept_triples.len() == triples.len() {
        return Err("control omitted no required triple".into());
    }
    value["triples"] = Value::Array(kept_triples);
    value["routes"] = Value::Array(kept_routes);
    Ok(())
}

type Mutation = (&'static str, fn(&mut Files) -> Result<(), String>);
fn semantic_mutations() -> Vec<Mutation> {
    vec![
        ("input_case_id", |f| {
            edit(f, SAMPLE, "input", |v| {
                set(v, "/case_id", json!("unregistered"))
            })
        }),
        ("input_prime", |f| {
            edit(f, SAMPLE, "input", |v| set(v, "/p", json!(101)))
        }),
        ("input_A", |f| {
            edit(f, SAMPLE, "input", |v| set(v, "/a", json!(2)))
        }),
        ("input_B", |f| {
            edit(f, SAMPLE, "input", |v| set(v, "/b", json!(1)))
        }),
        ("input_degree", |f| {
            edit(f, SAMPLE, "input", |v| set(v, "/d", json!(2)))
        }),
        ("valid_opposite_target", |f| {
            edit(f, SAMPLE, "input", |v| set(v, "/target", json!([1, 14])))
        }),
        ("input_F_policy", |f| {
            edit(f, SAMPLE, "input", |v| {
                set(v, "/restrict_liftable", json!(true))
            })
        }),
        ("subgroup_removed", |f| {
            edit(f, SUBGROUP, "input", |v| set(v, "/subgroup", Value::Null))
        }),
        ("equivalent_inverse_generator", |f| {
            edit(f, SUBGROUP, "input", |v| {
                set(v, "/subgroup/generator", json!([3, 91]))
            })
        }),
        ("nonminimal_subgroup_order", |f| {
            edit(f, SUBGROUP, "input", |v| {
                set(v, "/subgroup/order", json!(10))
            })
        }),
        ("missing_case_pair", |f| {
            f.remove(&filename(SAMPLE, "certificate"));
            f.remove(&filename(SAMPLE, "input"));
            Ok(())
        }),
        ("extra_case_pair", |f| {
            for side in ["certificate", "input"] {
                let bytes = f[&filename(SAMPLE, side)].clone();
                f.insert(filename("extra", side), bytes);
            }
            Ok(())
        }),
        ("missing_input_sidecar", |f| {
            f.remove(&filename(SAMPLE, "input"));
            Ok(())
        }),
        ("unknown_case_pair", |f| {
            for side in ["certificate", "input"] {
                let bytes = f
                    .remove(&filename(SAMPLE, side))
                    .ok_or("missing source control")?;
                f.insert(filename("unknown", side), bytes);
            }
            Ok(())
        }),
        ("output_case_id", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                set(v, "/case_id", json!("wrong"))
            })
        }),
        ("selector_root_omitted", |f| {
            edit(f, SAMPLE, "certificate", |v| remove_first(v, "/selector"))
        }),
        ("F_root_omitted", |f| {
            edit(f, SAMPLE, "certificate", |v| remove_first(v, "/x_roots"))
        }),
        ("G_root_omitted", |f| {
            edit(f, SAMPLE, "certificate", |v| remove_first(v, "/t_roots"))
        }),
        ("base_ordinate_changed", |f| {
            edit(f, SAMPLE, "certificate", |v| set(v, "/base/0/1", json!(15)))
        }),
        ("two_torsion_duplicated", |f| {
            edit(f, TORSION, "certificate", |v| {
                let first = v["base"][0].clone();
                array(v, "/base")?.push(first);
                Ok(())
            })
        }),
        ("subgroup_member_omitted", |f| {
            edit(f, SUBGROUP, "certificate", |v| {
                remove_first(v, "/subgroup_points")
            })
        }),
        ("quotient_H_changed", |f| {
            edit(f, SAMPLE, "certificate", |v| bump(v, "/certificate/h/0/0"))
        }),
        ("quotient_support_count_changed", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                bump(v, "/certificate/support_count")
            })
        }),
        ("left_bucket_omitted", |f| {
            edit(f, SAMPLE, "certificate", |v| remove_first(v, "/left"))
        }),
        ("left_witness_omitted", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                remove_first(v, "/left/0/roots")
            })
        }),
        ("left_gcd_changed", |f| {
            edit(f, SAMPLE, "certificate", |v| bump(v, "/left/0/gcd/0"))
        }),
        ("right_bucket_omitted", |f| {
            edit(f, SAMPLE, "certificate", |v| remove_first(v, "/right"))
        }),
        ("right_witness_omitted", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                remove_first(v, "/right/0/roots")
            })
        }),
        ("right_gcd_changed", |f| {
            edit(f, SAMPLE, "certificate", |v| bump(v, "/right/0/gcd/0"))
        }),
        ("noncancelling_PPP_omitted", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                omit_triples(v, |_, t, _| *t != json!([0, 0, 0]))
            })
        }),
        ("invalid_ordered_triple_added", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                array(v, "/triples")?.push(json!([0, 0, 1]));
                Ok(())
            })
        }),
        ("ordered_triple_duplicated", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                let first = v["triples"][0].clone();
                array(v, "/triples")?.push(first);
                Ok(())
            })
        }),
        ("wrong_prefix_chart", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                set(v, "/routes/0/chart", json!(1))
            })
        }),
        ("wrong_prefix_abscissa", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                set(v, "/routes/0/t", json!(1))
            })
        }),
        ("wrong_left_backpointer", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                set(v, "/routes/0/left", json!(1))
            })
        }),
        ("wrong_right_backpointer", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                set(v, "/routes/0/right", json!(1))
            })
        }),
        ("permutation_backpointer_omitted", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                remove_first(v, "/permutations/0/witnesses")
            })
        }),
        ("integer_row_coefficient_changed", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                let old = v["rows"][0]["coefficients"][0]
                    .as_i64()
                    .ok_or("missing row coefficient")?;
                set(v, "/rows/0/coefficients/0", json!(old + 1))
            })
        }),
        ("integer_row_target_changed", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                set(v, "/rows/0/target", json!(-1))
            })
        }),
        ("integer_row_backpointer_omitted", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                remove_first(v, "/rows/0/witnesses")
            })
        }),
        ("identity_prefix_branch_omitted", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                omit_triples(v, |_, _, r| r["chart"] != json!(1))
            })
        }),
        ("cancellation_coordinate_mask_discarded", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                omit_triples(v, |_, _, r| r["chart"] != json!(0))
            })
        }),
        ("coordinate_collision_bucket_collapsed", |f| {
            edit(f, SAMPLE, "certificate", |v| {
                let routes = v["routes"].as_array().ok_or("missing routes")?;
                let mut seen = BTreeSet::new();
                let mut keep = BTreeSet::new();
                for (i, r) in routes.iter().enumerate() {
                    if r["chart"] != json!(0)
                        || seen.insert((r["left"].as_u64(), r["right"].as_u64()))
                    {
                        keep.insert(i);
                    }
                }
                omit_triples(v, |i, _, _| keep.contains(&i))
            })
        }),
    ]
}

fn edit_raw(
    raw: &[u8],
    mutate: impl FnOnce(&mut Value) -> Result<(), String>,
) -> Result<Vec<u8>, String> {
    let text = std::str::from_utf8(raw).map_err(|e| e.to_string())?;
    let mut values: Vec<Value> = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;
    let row = values
        .iter_mut()
        .find(|v| v["kind"] == json!("case") && v["id"] == json!(SAMPLE))
        .ok_or("control raw case missing")?;
    mutate(row)?;
    let mut output = Vec::new();
    for value in values {
        output.extend(serde_json::to_vec(&value).map_err(|e| e.to_string())?);
        output.push(b'\n');
    }
    Ok(output)
}

pub fn controls(path: &Path, raw_path: &Path) -> Result<Value, String> {
    let original = load(path)?;
    let raw = fs::read(raw_path).map_err(|e| format!("read raw: {e}"))?;
    check_loaded(&original, Some(&raw))?;
    check_loaded(&original, None)?;
    let mut whitespace = original.clone();
    whitespace
        .get_mut(&filename(SAMPLE, "input"))
        .ok_or("control sidecar")?
        .push(b'\n');
    check_loaded(&whitespace, None)?;
    let mut outcomes = Vec::new();
    let mutations = semantic_mutations();
    if mutations.len() != 43 {
        return Err("semantic mutation count changed".into());
    }
    for (name, mutate) in mutations {
        let mut modified = original.clone();
        mutate(&mut modified)?;
        for with_raw in [true, false] {
            let error = check_loaded(&modified, if with_raw { Some(&raw) } else { None })
                .err()
                .ok_or_else(|| format!("mutation accepted: {name} with_raw={with_raw}"))?;
            outcomes.push(json!({"control":name,"raw":with_raw,"rejected":true,"reason":error}));
        }
    }
    let error = check_loaded(&whitespace, Some(&raw))
        .err()
        .ok_or("byte-changing whitespace accepted with raw")?;
    outcomes.push(
        json!({"control":"sidecar_whitespace_bytes","raw":true,"rejected":true,"reason":error}),
    );
    type RawMutation = (&'static str, fn(&mut Value) -> Result<(), String>);
    let raw_mutations: Vec<RawMutation> = vec![
        ("sidecar_hash_changed", |v| {
            set(v, "/h/1", json!("0".repeat(64)))
        }),
        ("sidecar_byte_count_changed", |v| bump(v, "/b/1")),
        ("sidecar_hash_missing", |v| {
            array(v, "/h")?.remove(1);
            Ok(())
        }),
        ("sidecar_byte_count_missing", |v| {
            array(v, "/b")?.remove(1);
            Ok(())
        }),
    ];
    for (name, mutate) in raw_mutations {
        let modified = edit_raw(&raw, mutate)?;
        let error = check_loaded(&original, Some(&modified))
            .err()
            .ok_or_else(|| format!("raw mutation accepted: {name}"))?;
        outcomes.push(json!({"control":name,"raw":true,"rejected":true,"reason":error}));
    }
    if outcomes.len() != 91 {
        return Err("reader rejection count changed".into());
    }
    Ok(
        json!({"kind":"signed_reader_controls","semantic_mutations":43,"semantic_mode_rejections":86,
        "raw_only_rejections":5,"rejections":91,"positive_checks":3,"all_passed":true,
        "extractor_invoked":false,"norm_constructor_invoked":false,"quotient_solver_invoked":false,
        "mutations_in_memory_only":true,"raw_sha256":digest(&raw),"protocol_sha256":digest(PROTOCOL),
        "outcomes":outcomes}),
    )
}
