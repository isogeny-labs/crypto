//! Independent general-arithmetic replay of every explicit-base relation in
//! an ecbench records.jsonl. No planted scalar enters the verification.
use crypto_lib::cryptanalysis::{
    ecbench::workload::{CurveSpec, Instance},
    ecbench_large_prime::{
        independent_verify,
        trace_line::{construct, verify_relations, Evidence},
        PartialRelation,
    },
    koblitz_fast::FastPoint,
};
use serde_json::{json, Value};
use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: trace_line_replay RECORDS.jsonl")?;
    let bytes = fs::read(&path)?;
    let mut receipt = replay(&bytes)?;
    receipt["input_file"] = json!(Path::new(&path).file_name().and_then(|s| s.to_str()));
    println!("{receipt}");
    Ok(())
}

fn replay(bytes: &[u8]) -> Result<Value, Box<dyn std::error::Error>> {
    if bytes.is_empty() {
        return Err("empty transcript".into());
    }
    let mut records = 0u64;
    let mut relations = 0u64;
    let mut recovered = 0u64;
    for text in BufReader::new(bytes).lines() {
        let row: Value = serde_json::from_str(&text?)?;
        if row["method"]["id"] != "ic.trace_line_2lp" {
            continue;
        }
        let spec: CurveSpec = serde_json::from_value(row["workload"]["curve_spec"].clone())?;
        let Instance::Binary(i) = spec.build()? else {
            return Err("nonbinary workload".into());
        };
        let coord = |v: &Value| -> Result<u64, Box<dyn std::error::Error>> {
            Ok(u64::from_str_radix(
                v.as_str()
                    .ok_or("missing hex coordinate")?
                    .trim_start_matches("0x"),
                16,
            )?)
        };
        let target = FastPoint::affine(
            coord(&row["workload"]["target"][0])?,
            coord(&row["workload"]["target"][1])?,
        );
        let detail = &row["detail"];
        let evidence: Evidence = serde_json::from_value(detail["trace_evidence"].clone())?;
        let params = &row["method"]["params"];
        let d: u32 = params["d"].as_str().ok_or("missing d")?.parse()?;
        let small_dimension: u32 = params["small_dimension"]
            .as_str()
            .ok_or("missing small_dimension")?
            .parse()?;
        let max_u: u64 = params["max_u"].as_str().ok_or("missing max_u")?.parse()?;
        let line = construct(
            &i,
            d,
            params["line"].as_str().ok_or("missing line policy")?,
            max_u,
        )?;
        if evidence.basis != line.basis {
            return Err("recorded basis differs from declared line policy".into());
        }
        let rows: Vec<PartialRelation> =
            serde_json::from_value(detail["relation_samples"].clone())?;
        let count = detail["counters"]["accepted_relations"]
            .as_u64()
            .ok_or("missing accepted count")?;
        verify_relations(&i, target, small_dimension, &evidence, &rows, count)?;
        let outcome = &row["outcome"];
        let status = outcome["status"].as_str().ok_or("missing outcome status")?;
        match status {
            "verified" => {
                if outcome["matches_target"].as_bool() != Some(true) {
                    return Err("verified outcome does not record matches_target=true".into());
                }
                let outcome_scalar: u64 = outcome["recovered"]
                    .as_str()
                    .ok_or("verified outcome is missing its recovered scalar")?
                    .parse()
                    .map_err(|_| "verified outcome has a malformed recovered scalar")?;
                let detail_scalar = detail["recovered"]
                    .as_u64()
                    .ok_or("verified outcome is missing detail.recovered")?;
                if outcome_scalar != detail_scalar {
                    return Err("outcome.recovered disagrees with detail.recovered".into());
                }
                independent_verify(&i, target, outcome_scalar)?;
                recovered += 1;
            }
            "exhausted" if outcome["recovered"].is_null() && detail["recovered"].is_null() => {}
            "exhausted" => return Err("exhausted outcome contains a recovered scalar".into()),
            _ => {
                return Err(
                    format!("trace-line record has unverified outcome status {status}").into(),
                )
            }
        }
        records += 1;
        relations += count;
    }
    if records == 0 {
        return Err("no trace-line records; empty replay is not verification".into());
    }
    let digest = |data: &[u8]| hex::encode(crypto_lib::hash::sha256::sha256(data));
    let executable = fs::read(std::env::current_exe()?)?;
    Ok(json!({"schema":"trace-line.replay/v1", "records":records,
        "relations_verified":relations,"scalars_verified":recovered,
        "input_sha256":digest(bytes),"verifier_binary_sha256":digest(&executable),
        "arithmetic":"general BinaryCurve point_add/scalar_mul",
        "complete_pipeline_cost":null,"c1_algebraic_trial_cost":null}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::ecbench::{
        methods::{resolve, solve, MethodSpec},
        workload::{TargetKind, Workload},
    };
    #[test]
    fn serialised_records_replay_and_corruption_or_empty_input_is_rejected() {
        let spec = CurveSpec::BinaryRandom {
            n: 12,
            seed: 20261027,
            max_cofactor: 16,
        };
        let (workload, instance) =
            Workload::build(&spec, 20261027, 0, TargetKind::Planted).unwrap();
        let params = std::collections::BTreeMap::from([
            ("line".into(), "trace".into()),
            ("d".into(), "3".into()),
            ("small_dimension".into(), "2".into()),
            ("max_u".into(), "4095".into()),
            ("max_trials".into(), "5000".into()),
            ("max_states".into(), "100000".into()),
        ]);
        let method = resolve(&MethodSpec {
            id: "ic.trace_line_2lp".into(),
            params: params.clone(),
        })
        .unwrap();
        let result = solve(&method, &instance, &workload.curve, &workload.target, 23).unwrap();
        let scalar = result.recovered.unwrap();
        let mut row = json!({"method":{"id":"ic.trace_line_2lp","params":params},"workload":workload,
            "outcome":{"status":"verified","recovered":scalar.to_string(),"matches_target":true},
            "detail":result.detail});
        let receipt = replay(&serde_json::to_vec(&row).unwrap()).unwrap();
        assert_eq!(receipt["records"], 1);
        assert_eq!(receipt["scalars_verified"], 1);
        let original = row.clone();
        row["outcome"]["recovered"] = json!((scalar + 1).to_string());
        assert!(replay(&serde_json::to_vec(&row).unwrap()).is_err());
        row = original.clone();
        row["outcome"]["recovered"] = json!("not-a-scalar");
        assert!(replay(&serde_json::to_vec(&row).unwrap()).is_err());
        row = original.clone();
        row["outcome"]["status"] = json!("exhausted");
        assert!(replay(&serde_json::to_vec(&row).unwrap()).is_err());
        row = original.clone();
        row["detail"]["recovered"] = Value::Null;
        assert!(replay(&serde_json::to_vec(&row).unwrap()).is_err());
        row = original.clone();
        row["method"]["params"]["small_dimension"] = json!("1");
        assert!(replay(&serde_json::to_vec(&row).unwrap()).is_err());
        row = original;
        row["detail"]["relation_samples"][0]["a"] = json!(1000000);
        assert!(replay(&serde_json::to_vec(&row).unwrap()).is_err());
        assert!(replay(b"").is_err());
        assert!(replay(b"{\"method\":{\"id\":\"rho.negation\"}}").is_err());
    }
}
