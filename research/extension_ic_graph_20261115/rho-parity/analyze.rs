//! Recompute the development comparison from the retained native JSONL.
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs::OpenOptions, io::Write};
type Arms<'a> = BTreeMap<(String, u64), &'a Value>;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: ANALYZER RAW_JSONL NEW_SUMMARY_JSON".into());
    }
    let lines: Vec<Value> = std::fs::read_to_string(&args[0])?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    let header = lines.first().ok_or("missing header")?;
    let footer = lines.last().ok_or("missing footer")?;
    if header["schema"] != "nagao.unit-elimination.development/v1"
        || footer["completed"] != 24
        || footer["complete_cost"] != false
        || footer["ratios"] != Value::Null
        || footer["wall_ranking"] != Value::Null
    {
        return Err("incomplete or changed boundary".into());
    }
    let mut groups: BTreeMap<(u64, String), Arms<'_>> = BTreeMap::new();
    for row in &lines[1..lines.len() - 1] {
        if row["type"] != "case" || row["foreign_thread_hooks"] != 0 {
            return Err("case accounting".into());
        }
        let p = row["p"].as_u64().ok_or("field")?;
        let label = row["residual"].as_str().ok_or("residual")?.to_owned();
        if ![53, 101, 1009].contains(&p) || !["generator", "planted-six"].contains(&label.as_str())
        {
            return Err("unregistered case".into());
        }
        let key = (
            row["arm"].as_str().ok_or("arm")?.to_owned(),
            row["repeat"].as_u64().ok_or("repeat")?,
        );
        if groups
            .entry((p, label))
            .or_default()
            .insert(key, row)
            .is_some()
        {
            return Err("duplicate arm".into());
        }
    }
    if groups.len() != 6 {
        return Err("missing fixture".into());
    }
    let mut records = Vec::new();
    let mut total_six = 0u128;
    let mut total_unit = 0u128;
    for ((p, label), group) in groups {
        if group.len() != 4 {
            return Err("missing arms".into());
        }
        let get = |a: &str, b: u64| group.get(&(a.to_owned(), b)).copied().ok_or("missing arm");
        let a = get("six", 0)?;
        let b = get("unit", 0)?;
        for arm in ["six", "unit"] {
            let x = get(arm, 0)?;
            let y = get(arm, 1)?;
            if x["input"] != y["input"]
                || x["roots"] != y["roots"]
                || x["rows"] != y["rows"]
                || x["work"] != y["work"]
                || x["f4"] != y["f4"]
            {
                return Err("repeat mismatch".into());
            }
        }
        if a["input"] != b["input"] || a["roots"] != b["roots"] || a["rows"] != b["rows"] {
            return Err("cross-solver geometry mismatch".into());
        }
        let number = |x: &Value, k: &str| -> Result<u128, Box<dyn std::error::Error>> {
            Ok(x["work"][k].as_str().ok_or("subtotal")?.parse()?)
        };
        let six = number(a, "materializer")?;
        let unit = number(b, "materializer")?;
        total_six += six;
        total_unit += unit;
        println!(
            "{p} {label}: six={six} unit={unit} ratio={:.6} roots={} full={} partial={}",
            unit as f64 / six as f64,
            a["roots"].as_array().ok_or("roots")?.len(),
            a["rows"]["full"].as_array().ok_or("full")?.len(),
            a["rows"]["cofactors"]
                .as_array()
                .ok_or("cofactors")?
                .iter()
                .filter(|r| r["partial"] != Value::Null)
                .count()
        );
        records.push(json!({"p":p,"residual":label,"six":six.to_string(),"unit":unit.to_string(),"unit_over_six_observed_subtotal":unit as f64/six as f64,"coordinate_six":number(a,"coordinate")?.to_string(),"coordinate_unit":number(b,"coordinate")?.to_string(),"roots":a["roots"],"full_rows":a["rows"]["full"].as_array().unwrap().len(),"f4_six":a["f4"],"f4_unit":b["f4"]}));
    }
    let summary = json!({"schema":"nagao.unit-elimination.development-summary/v1","source_revision":header["source_revision"],"records":records,"all_geometry_and_repeats_equal":true,"observed_subtotal_six":total_six.to_string(),"observed_subtotal_unit":total_unit.to_string(),"observed_subtotal_unit_over_six":total_unit as f64/total_six as f64,"complete_total":null,"rho_parity":null,"wall_ranking":null});
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&args[1])?;
    serde_json::to_writer_pretty(&mut file, &summary)?;
    file.write_all(b"\n")?;
    println!(
        "aggregate observed subtotal ratio: {:.6}",
        total_unit as f64 / total_six as f64
    );
    Ok(())
}
