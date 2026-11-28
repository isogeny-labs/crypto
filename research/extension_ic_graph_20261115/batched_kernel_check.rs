//! Extend unchanged independent44 replay by exact non-wall output equality.
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path};
type E = Box<dyn std::error::Error>;
fn normalize(v: &Value) -> Result<Value, E> {
    let mut out = v.clone();
    let records = out["corpora"].as_array_mut().ok_or("corpora")?;
    if records.len() != 2 {
        return Err("two corpora".into());
    }
    for row in records {
        let o = row.as_object_mut().ok_or("corpus object")?;
        let time = o.remove("inner_wall_ns").ok_or("wall time")?;
        if time.as_str().ok_or("wall string")?.parse::<u128>()? == 0 {
            return Err("zero elapsed time".into());
        }
    }
    Ok(out)
}
fn compare(a: &Value, b: &Value) -> Result<(), E> {
    if normalize(a)? != normalize(b)? {
        return Err("batched/scalar non-wall outputs differ".into());
    }
    Ok(())
}
fn main() -> Result<(), E> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: EQUALITY_CHECK RUN NEW_OUTPUT".into());
    }
    let dir = Path::new(&args[0]);
    let baseline: Value = serde_json::from_slice(&fs::read(dir.join("baseline-summary.json"))?)?;
    if baseline["schema"] != "cover.sparse-kernel.qualified/v1"
        || baseline["complete_cost"] != false
        || fs::read_to_string(dir.join("analysis-original.exit-code.txt"))?.trim() != "0"
        || fs::read_to_string(dir.join("batched-lanczos.exit-code.txt"))?.trim() != "0"
    {
        return Err("unchanged independent qualifier/worker status".into());
    }
    let scalar: Value =
        serde_json::from_slice(&fs::read(dir.join("randomized-lanczos.raw.json"))?)?;
    let batch: Value = serde_json::from_slice(&fs::read(dir.join("batched-lanczos.raw.json"))?)?;
    compare(&scalar, &batch)?;
    if baseline["source_revision"] != scalar["source_revision"]
        || baseline["input_sha256"] != scalar["input_sha256"]
        || scalar["method"] != "randomized-lanczos"
    {
        return Err("qualified source/input binding".into());
    }
    let mut rows = baseline["rows"].as_array().ok_or("qualified rows")?.clone();
    if rows.len() != 6 {
        return Err("six baseline cells".into());
    }
    for i in 0..2 {
        let record = &batch["corpora"][i];
        let mut row = rows
            .iter()
            .find(|r| {
                r["corpus"] == record["corpus"] && r["checked"]["method"] == "randomized-lanczos"
            })
            .ok_or("qualified scalar cell")?
            .clone();
        if row["checked"]["verified_kernel"] != true
            || row["checked"]["primitives"] != record["primitives"]
            || row["checked"]["observed_incomplete_subtotal"]
                != record["observed_incomplete_subtotal"]
        {
            return Err("qualified cell conservation".into());
        }
        row["checked"]["method"] = json!("batched-lanczos");
        row["checked"]["inner_wall_ns"] = record["inner_wall_ns"].clone();
        row["checked"]["exact_nonwall_outputs_equal"] = json!(true);
        rows.push(row);
    }
    let out = json!({"schema":"cover.batched-sparse-kernel.qualified/v1","source_revision":baseline["source_revision"],"input_sha256":baseline["input_sha256"],"complete_cost":false,"complete_total":null,"ratios":null,"baseline_qualifier_unchanged":true,"batched_scalar_nonwall_equality":true,"scope":"existing independent rank/kernel/trace replay, extended by exact equality of every non-wall worker field; producer software evidence, unsigned","rows":rows});
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[1])?;
    serde_json::to_writer_pretty(&mut f, &out)?;
    f.write_all(b"\n")?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Value {
        json!({"source_revision":"a","input_sha256":"b","corpora":[{"inner_wall_ns":"10","result":{"kernel":[2,1],"attempts":[{"diagonal":4}]},"primitives":{"modr_mul":8},"uniform":{"raw_draws":3}},{"inner_wall_ns":"20","value":7}]})
    }
    #[test]
    fn accepts_only_elapsed_differences() {
        let a = fixture();
        let mut b = a.clone();
        b["corpora"][0]["inner_wall_ns"] = json!("500");
        b["corpora"][1]["inner_wall_ns"] = json!("900");
        assert!(compare(&a, &b).is_ok());
    }
    #[test]
    fn refuses_every_scientific_counter_trace_and_identity_change() {
        let a = fixture();
        for path in [
            "/source_revision",
            "/input_sha256",
            "/corpora/0/result/kernel/0",
            "/corpora/0/result/attempts/0/diagonal",
            "/corpora/0/primitives/modr_mul",
            "/corpora/0/uniform/raw_draws",
            "/corpora/1/value",
        ] {
            let mut b = a.clone();
            *b.pointer_mut(path).unwrap() = json!(999);
            assert!(compare(&a, &b).is_err(), "{path}");
        }
    }
    #[test]
    fn missing_or_invalid_time_and_added_field_refused() {
        let a = fixture();
        for time in [json!("0"), json!("bad"), json!(5), Value::Null] {
            let mut b = a.clone();
            b["corpora"][0]["inner_wall_ns"] = time;
            assert!(compare(&a, &b).is_err());
        }
        let mut b = a.clone();
        b["extra"] = json!(true);
        assert!(compare(&a, &b).is_err());
    }
}
