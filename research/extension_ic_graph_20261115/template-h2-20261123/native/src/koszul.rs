use certified_relations as algebra;
use nagao_template::{same_ideal, Limits, Poly, Template};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{env, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const INPUT_SHA: &str = "e63a6126b3309bc7e387347f4f8629d78f12897a7c55c928b7152e75c21e76f1";
fn emit(f: &mut fs::File, v: Value) -> Result<()> {
    writeln!(f, "{v}")?;
    f.flush()?;
    Ok(())
}
fn save(p: &Path, v: Value) -> Result<()> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)?;
    writeln!(f, "{}", serde_json::to_string_pretty(&v)?)?;
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
        return Err("two saved cases".into());
    }
    for (i, c) in saved.iter().enumerate() {
        if c["index"] != i
            || c["p"] != 1009
            || c["variables"] != 6
            || c["f4_completion_flags"]["timed_out"] != false
            || c["f4_completion_flags"]["pairs_above_bound"] != 0
            || !c["f4_completion_flags"]["staircase_at_stop"].is_null()
        {
            return Err("original identity/completion flags".into());
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
    let limits = Limits {
        rows: 1536,
        columns: 1024,
        cells: 4_000_000,
        degree: 16,
    };
    let input: Vec<Poly> = serde_json::from_value(saved[0]["input"].clone())?;
    let fingerprint: Vec<Poly> = serde_json::from_value(saved[0]["basis"].clone())?;
    let mut degrees = Vec::new();
    for degree in [5, 6] {
        emit(
            &mut events,
            json!({"phase":"construction_started","degree":degree,"limits":limits}),
        )?;
        algebra::reset_counters();
        let result = Template::principal_syzygy(&input, &fingerprint, 6, 1009, degree, limits);
        let construction_validation = algebra::counters();
        let template = match result {
            Ok(t) => t,
            Err(reason) => {
                let v = json!({"degree":degree,"construction_refused":reason,"construction_validation":construction_validation});
                emit(&mut events, v.clone())?;
                degrees.push(v);
                continue;
            }
        };
        let shape = json!({"enumerated":template.enumerated_rows(),"kept":template.rows(),"columns":template.columns()});
        emit(
            &mut events,
            json!({"phase":"construction_complete","degree":degree,"shape":shape,"construction_validation":construction_validation}),
        )?;
        save(
            &root.join(format!("template-degree-{degree}.json")),
            serde_json::to_value(&template)?,
        )?;
        let mut cases = Vec::new();
        for (index, case) in saved.iter().enumerate() {
            emit(
                &mut events,
                json!({"phase":"application_started","degree":degree,"case":index}),
            )?;
            let input: Vec<Poly> = serde_json::from_value(case["input"].clone())?;
            let reference: Vec<Poly> = serde_json::from_value(case["basis"].clone())?;
            algebra::reset_counters();
            let applied = template.apply(&input);
            let checker = algebra::counters();
            let result = match applied {
                Err(refusal) => {
                    json!({"case":index,"degree":degree,"accepted":false,"refusal":refusal,"checker":checker})
                }
                Ok(value) => {
                    emit(
                        &mut certificates,
                        json!({"degree":degree,"case":index,"basis":value.basis,
                        "witness":value.certificates.iter().map(|r|json!({
                            "poly":r.poly.iter().map(|(m,c)|json!([m,c])).collect::<Vec<_>>(),
                            "proof":r.proof.iter().map(|((i,m),c)|json!({"input":i,"multiplier":m,"coefficient":c})).collect::<Vec<_>>()
                        })).collect::<Vec<_>>()}),
                    )?;
                    algebra::reset_counters();
                    let compared = same_ideal(&value.basis, &reference, 6, 1009, limits);
                    let comparison_validation = algebra::counters();
                    match compared {
                        Ok((equal, comparison)) => {
                            json!({"case":index,"degree":degree,"accepted":true,
                            "ideal_equal_saved_f4":equal,"elimination":value.elimination,"checker":checker,
                            "comparison":comparison,"comparison_validation":comparison_validation,"rank":value.rank,"basis_size":value.basis.len()})
                        }
                        Err(error) => {
                            json!({"case":index,"degree":degree,"accepted":true,"comparison_refused":error,
                            "elimination":value.elimination,"checker":checker})
                        }
                    }
                }
            };
            emit(&mut events, result.clone())?;
            cases.push(result);
        }
        degrees.push(json!({"degree":degree,"shape":shape,"construction_validation":construction_validation,"cases":cases}));
    }
    let summary = json!({"schema":"nagao-koszul-probe/v1","degrees":degrees,"input_sha256":INPUT_SHA,
        "scope":"offline oracle-assisted construction; principal syzygies only; no fresh F4 cost, runtime or parity ratio",
        "signed_validation":false});
    save(&root.join("summary.json"), summary.clone())?;
    println!("{summary}");
    Ok(())
}
