//! Native metadata freezer for the canonical experiment.protocol/v1 hash.
//! JSON Schema validation remains a separate maintained-tool check.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn canonical_hash(value: &Value) -> Result<String, String> {
    let mut body = value
        .as_object()
        .ok_or("protocol must be an object")?
        .clone();
    body.remove("freeze");
    let mut encoded = String::new();
    canonical_value(&Value::Object(body), &mut encoded)?;
    Ok(hash(encoded.as_bytes()))
}

// Match the canonical checker's float exponent thresholds and zero padding.
fn canonical_float(raw: &str) -> Result<String, String> {
    let (sign, unsigned) = raw.strip_prefix('-').map_or(("", raw), |s| ("-", s));
    let (mantissa, exponent) = unsigned
        .split_once('e')
        .map_or(Ok((unsigned, 0i32)), |(m, e)| {
            e.parse::<i32>().map(|e| (m, e)).map_err(|e| e.to_string())
        })?;
    let dot = mantissa.find('.').unwrap_or(mantissa.len());
    let all_digits: String = mantissa.chars().filter(|&c| c != '.').collect();
    let Some(first) = all_digits.bytes().position(|c| c != b'0') else {
        return Ok(format!("{sign}0.0"));
    };
    let digits = all_digits[first..].trim_end_matches('0');
    let power = exponent + dot as i32 - first as i32 - 1;
    if !(-4..16).contains(&power) {
        let tail = if digits.len() > 1 {
            format!(".{}", &digits[1..])
        } else {
            String::new()
        };
        let exp_sign = if power < 0 { '-' } else { '+' };
        return Ok(format!(
            "{sign}{}{tail}e{exp_sign}{:02}",
            &digits[..1],
            power.unsigned_abs()
        ));
    }
    let position = power + 1;
    if position <= 0 {
        Ok(format!(
            "{sign}0.{}{digits}",
            "0".repeat((-position) as usize)
        ))
    } else if position as usize >= digits.len() {
        Ok(format!(
            "{sign}{digits}{}.0",
            "0".repeat(position as usize - digits.len())
        ))
    } else {
        Ok(format!(
            "{sign}{}.{}",
            &digits[..position as usize],
            &digits[position as usize..]
        ))
    }
}

fn canonical_value(value: &Value, output: &mut String) -> Result<(), String> {
    match value {
        Value::Object(map) => {
            output.push('{');
            for (index, (key, value)) in map.iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                output.push_str(&serde_json::to_string(key).map_err(|e| e.to_string())?);
                output.push(':');
                canonical_value(value, output)?;
            }
            output.push('}');
        }
        Value::Array(values) => {
            output.push('[');
            for (index, value) in values.iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                canonical_value(value, output)?;
            }
            output.push(']');
        }
        Value::Number(number) if number.is_f64() => {
            output.push_str(&canonical_float(&number.to_string())?)
        }
        _ => output.push_str(&serde_json::to_string(value).map_err(|e| e.to_string())?),
    }
    Ok(())
}

fn valid_sha(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn validate_value(value: &Value, path: &Path, require_frozen: bool) -> Result<(), String> {
    if value["schema"] != "experiment.protocol/v1" {
        return Err("unexpected protocol schema".into());
    }
    let arms = value["arms"].as_array().ok_or("missing arms")?;
    let mut ids = std::collections::BTreeSet::new();
    for arm in arms {
        let id = arm["id"].as_str().ok_or("missing arm id")?;
        if !ids.insert(id) {
            return Err("duplicate protocol arm".into());
        }
    }
    let expected: std::collections::BTreeSet<_> = [
        "random",
        "progression",
        "collision_avoid",
        "twist_l1",
        "twist_l2",
        "twist_l3",
        "twist_l4",
        "planted_rank",
        "b2_negative",
    ]
    .into_iter()
    .collect();
    if ids != expected
        || value["sample_size"]["instances"] != 3
        || value["sample_size"]["replicates"] != 1
    {
        return Err("protocol does not declare the frozen 9 by 3 finite audit".into());
    }
    let scope = &value["extensions"]["finite_scope"];
    if scope["primes"] != json!([7, 11, 19])
        || scope["base_sizes"] != json!([2, 3])
        || scope["thresholds"] != json!([1, 2, 3, 4])
        || scope["selector_budget"] != 8
        || scope["walk_length"] != 8
        || scope["coefficient_model_dispositions"] != 531
    {
        return Err("protocol finite scope differs from the compiled experiment".into());
    }
    for arm in arms {
        let id = arm["id"].as_str().unwrap();
        if ["planted_rank", "b2_negative"].contains(&id) {
            continue;
        }
        if arm["parameters"]["base_sizes"] != json!([2, 3])
            || arm["parameters"]["selector_budget"] != 8
            || arm["parameters"]["walk_length"] != 8
        {
            return Err(format!("arm {id} differs from the compiled finite scope"));
        }
        if let Some(suffix) = id.strip_prefix("twist_l") {
            if arm["parameters"]["threshold"].as_u64() != suffix.parse::<u64>().ok() {
                return Err(format!("arm {id} has a mismatched threshold"));
            }
        }
    }
    let script = &value["analysis_plan"]["script"];
    if require_frozen && script.is_null() {
        return Err("TR-04 requires a frozen analysis source".into());
    }
    if !script.is_null() {
        let name = script["path"].as_str().ok_or("missing analysis path")?;
        let full = path.parent().ok_or("protocol has no parent")?.join(name);
        let actual = hash(&fs::read(&full).map_err(|e| format!("{}: {e}", full.display()))?);
        if let Some(want) = script["sha256"].as_str() {
            if want != actual {
                return Err("analysis source hash differs from protocol".into());
            }
        } else if require_frozen {
            return Err("frozen protocol requires analysis source hash".into());
        }
    }
    if require_frozen
        && (value["freeze"]["status"] != "frozen"
            || value["freeze"]["filed_before_affected_data"] != true
            || value["analysis_plan"]["frozen_before_data"] != true
            || value["sample_size"]["preflight_runtime"]["estimated_total_seconds"].is_null()
            || value["freeze"]["frozen_at"]
                .as_str()
                .is_none_or(str::is_empty)
            || value["freeze"]["commit"]
                .as_str()
                .is_none_or(|s| !valid_sha(s, 40))
            || value["freeze"]["protocol_sha256"].as_str() != Some(canonical_hash(value)?.as_str()))
    {
        return Err("missing or inconsistent frozen protocol metadata".into());
    }
    Ok(())
}

pub fn validate(path: &Path, require_frozen: bool) -> Result<(), String> {
    validate_with_analysis_protocol(path, path, require_frozen)
}

/// Validate a retained protocol copy while resolving its source attachment
/// against the original repository protocol location. The caller separately
/// binds the copied bytes to its manifest and, for primary data, current source.
pub fn validate_with_analysis_protocol(
    path: &Path,
    analysis_protocol: &Path,
    require_frozen: bool,
) -> Result<(), String> {
    let value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    validate_value(&value, analysis_protocol, require_frozen)
}

pub fn cli(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("tr04-protocol validate|hash|freeze PROTOCOL [COMMIT]".into());
    }
    let path = Path::new(&args[1]);
    let mut value: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    match args[0].as_str() {
        "hash" if args.len() == 2 => println!("{}", canonical_hash(&value)?),
        "validate" if args.len() == 2 => {
            validate_value(&value, path, true)?;
            println!(
                "frozen protocol metadata and canonical hash verified; schema is a separate check"
            );
        }
        "freeze" if args.len() == 3 => {
            if value["freeze"]["status"] == "frozen" {
                return Err("refusing to overwrite a frozen protocol; amend a new version".into());
            }
            if !valid_sha(&args[2], 40) {
                return Err("freeze requires explicit 40-character commit".into());
            }
            validate_value(&value, path, false)?;
            let output = Command::new("date")
                .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
                .output()
                .map_err(|e| e.to_string())?;
            if !output.status.success() {
                return Err("cannot obtain UTC freeze time".into());
            }
            let now = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
            value["freeze"] = json!({"status":"frozen", "protocol_sha256":canonical_hash(&value)?, "commit":args[2], "frozen_at":now.trim(), "filed_before_affected_data":true});
            validate_value(&value, path, true)?;
            crate::evidence::write_json_atomic(path, &value)?;
            println!("{}", value["freeze"]["protocol_sha256"].as_str().unwrap());
        }
        _ => return Err("unsupported protocol metadata command/arguments".into()),
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retained_protocol_resolves_analysis_against_repository_location() {
        let root = std::env::temp_dir().join(format!(
            "tr04-protocol-relocation-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let original = root.join("study/protocol.json");
        let copy = root.join("artifact/results/protocol.json");
        fs::create_dir_all(original.parent().unwrap()).unwrap();
        fs::create_dir_all(copy.parent().unwrap()).unwrap();
        fs::write(root.join("study/analysis.rs"), b"fixed analysis").unwrap();
        let arms = [
            "random",
            "progression",
            "collision_avoid",
            "twist_l1",
            "twist_l2",
            "twist_l3",
            "twist_l4",
            "planted_rank",
            "b2_negative",
        ]
        .into_iter()
        .map(|id| {
            let mut parameters = json!({"base_sizes":[2,3],"selector_budget":8,"walk_length":8});
            if let Some(threshold) = id.strip_prefix("twist_l") {
                parameters["threshold"] = json!(threshold.parse::<u64>().unwrap());
            }
            json!({"id":id,"parameters":parameters})
        })
        .collect::<Vec<_>>();
        let value = json!({"schema":"experiment.protocol/v1","arms":arms,
            "sample_size":{"instances":3,"replicates":1},
            "extensions":{"finite_scope":{"primes":[7,11,19],"base_sizes":[2,3],"thresholds":[1,2,3,4],"selector_budget":8,"walk_length":8,"coefficient_model_dispositions":531}},
            "analysis_plan":{"script":{"path":"analysis.rs","sha256":hash(b"fixed analysis")}}});
        fs::write(&copy, serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(validate(&copy, false).is_err());
        validate_with_analysis_protocol(&copy, &original, false).unwrap();
        fs::write(root.join("study/analysis.rs"), b"changed analysis").unwrap();
        assert!(validate_with_analysis_protocol(&copy, &original, false).is_err());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn canonical_float_matches_reference_exponent_conventions() {
        for (input, expected) in [
            ("1e-7", "1e-07"),
            ("0.00001", "1e-05"),
            ("0.0001", "0.0001"),
            ("1.0", "1.0"),
            ("-0.0", "-0.0"),
            ("1e15", "1000000000000000.0"),
            ("1e16", "1e+16"),
            ("1.234e-5", "1.234e-05"),
            ("1.23e5", "123000.0"),
        ] {
            assert_eq!(canonical_float(input).unwrap(), expected);
        }
    }
    #[test]
    fn canonical_hash_ignores_only_freeze_and_is_order_independent() {
        let a = json!({"z":"λ", "a":{"n":2}, "freeze":{"status":"draft"}});
        let b: Value = serde_json::from_str(
            "{\"a\":{\"n\":2},\"z\":\"λ\",\"freeze\":{\"status\":\"frozen\"}}",
        )
        .unwrap();
        assert_eq!(canonical_hash(&a).unwrap(), canonical_hash(&b).unwrap());
        let mut changed = b;
        changed["a"]["n"] = json!(3);
        assert_ne!(
            canonical_hash(&a).unwrap(),
            canonical_hash(&changed).unwrap()
        );
    }
}
