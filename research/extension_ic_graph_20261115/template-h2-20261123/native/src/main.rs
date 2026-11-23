use certified_relations as algebra;
use nagao_template::{same_ideal, Limits, Poly, Template};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{env, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const INPUT_SHA: &str = "e63a6126b3309bc7e387347f4f8629d78f12897a7c55c928b7152e75c21e76f1";
fn emit(file: &mut fs::File, value: Value) -> Result<()> {
    writeln!(file, "{value}")?;
    file.flush()?;
    Ok(())
}
fn exclusive(path: &Path, value: Value) -> Result<()> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    writeln!(f, "{}", serde_json::to_string_pretty(&value)?)?;
    f.sync_all()?;
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 || args[1] != "--probe" {
        return Err("usage: --probe SAVED_BASES NEW_OUTPUT".into());
    }
    let bytes = fs::read(&args[2])?;
    if format!("{:x}", Sha256::digest(&bytes)) != INPUT_SHA {
        return Err("input hash mismatch".into());
    }
    let saved: Vec<Value> = std::str::from_utf8(&bytes)?
        .lines()
        .take(2)
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    if saved.len() != 2 {
        return Err("two saved cases required".into());
    }
    for (i, case) in saved.iter().enumerate() {
        if case["index"] != i
            || case["p"] != 1009
            || case["variables"] != 6
            || case["f4_completion_flags"]["timed_out"] != false
            || case["f4_completion_flags"]["pairs_above_bound"] != 0
            || !case["f4_completion_flags"]["staircase_at_stop"].is_null()
        {
            return Err("saved original identity/completion flags".into());
        }
    }
    let root = Path::new(&args[3]);
    fs::create_dir(root)?;
    let mut events = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join("events.jsonl"))?;
    let mut certificates = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join("certificates.jsonl"))?;
    let input: Vec<Poly> = serde_json::from_value(saved[0]["input"].clone())?;
    emit(
        &mut events,
        json!({"phase":"learning_started","case":0,"input_sha256":INPUT_SHA,"limits":Limits::default()}),
    )?;
    algebra::reset_counters();
    let learned = Template::learn(&input, 6, 1009, Limits::default());
    let learning = algebra::counters();
    let template = match learned {
        Ok(t) => t,
        Err(reason) => {
            let result = json!({"schema":"nagao-template-probe/v1","outcome":"construction_refused","reason":reason,
                "learning":learning,"cases_applied":0,"input_sha256":INPUT_SHA,"signed_validation":false});
            emit(&mut events, result.clone())?;
            exclusive(&root.join("summary.json"), result)?;
            return Ok(());
        }
    };
    emit(
        &mut events,
        json!({"phase":"learning_complete","case":0,"rows":template.rows(),"columns":template.columns(),"learning":learning}),
    )?;
    exclusive(
        &root.join("template.json"),
        serde_json::to_value(&template)?,
    )?;
    let mut results = Vec::new();
    for (i, case) in saved.iter().enumerate() {
        let input: Vec<Poly> = serde_json::from_value(case["input"].clone())?;
        let reference: Vec<Poly> = serde_json::from_value(case["basis"].clone())?;
        emit(&mut events, json!({"phase":"application_started","case":i}))?;
        algebra::reset_counters();
        let applied = template.apply(&input);
        let application_checker = algebra::counters();
        let result = match applied {
            Ok(value) => {
                emit(
                    &mut certificates,
                    json!({"case":i,"basis":value.basis,
                    "witness":value.certificates.iter().map(|r|json!({
                        "poly":r.poly.iter().map(|(m,c)|json!([m,c])).collect::<Vec<_>>(),
                        "proof":r.proof.iter().map(|((index,m),c)|json!({"input":index,"multiplier":m,"coefficient":c})).collect::<Vec<_>>()
                    })).collect::<Vec<_>>()}),
                )?;
                algebra::reset_counters();
                let compared = same_ideal(&value.basis, &reference, 6, 1009, Limits::default());
                let comparison_validation = algebra::counters();
                match compared {
                    Ok((equal, comparison)) => {
                        json!({"case":i,"accepted":true,"ideal_equal_saved_f4":equal,
                        "elimination":value.elimination,"checker":application_checker,"comparison":comparison,
                        "comparison_validation":comparison_validation,"rank":value.rank,"basis_size":value.basis.len()})
                    }
                    Err(error) => json!({"case":i,"accepted":true,"comparison_refused":error,
                        "elimination":value.elimination,"checker":application_checker}),
                }
            }
            Err(error) => {
                json!({"case":i,"accepted":false,"refusal":error,"checker":application_checker})
            }
        };
        emit(&mut events, result.clone())?;
        results.push(result);
    }
    let result = json!({"schema":"nagao-template-probe/v1","outcome":"construction_completed",
        "learning":learning,"rows":template.rows(),"columns":template.columns(),"cases":results,
        "all_ideals_equal_saved_f4":results.iter().all(|r|r["ideal_equal_saved_f4"]==true),
        "input_sha256":INPUT_SHA,"scope":"unreduced provenance template; no F4 timing or parity comparison",
        "signed_validation":false});
    exclusive(&root.join("summary.json"), result.clone())?;
    println!("{result}");
    Ok(())
}
