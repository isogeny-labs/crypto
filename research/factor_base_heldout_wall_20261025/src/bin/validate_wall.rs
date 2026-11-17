//! Native, fail-closed comparison of all frozen N12 timed observations.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const BITS: [u64; 7] = [16, 20, 24, 28, 32, 36, 40];
const SLOTS: [(&str, u64, &str); 12] = [
    ("aa1",0,"rho"),("aa2",0,"rho"),
    ("ab1a",1,"candidate"),("ab1b",1,"rho"),
    ("ab2a",2,"rho"),("ab2b",2,"candidate"),
    ("ab3a",3,"candidate"),("ab3b",3,"rho"),
    ("ab4a",4,"rho"),("ab4b",4,"candidate"),
    ("ab5a",5,"candidate"),("ab5b",5,"rho")
];
fn read(path: &Path) -> Result<Vec<Value>, String> {
    let f = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    BufReader::new(f).lines().map(|line| {
        let line = line.map_err(|e| e.to_string())?;
        serde_json::from_str(&line).map_err(|e| format!("{}: {e}", path.display()))
    }).collect()
}
fn csv(s: &str) -> String { format!("\"{}\"", s.replace('"', "\"\"")) }
fn n(v: &Value) -> Option<u128> { v.as_str().and_then(|s| s.parse().ok()) }
fn eq_fields(a: &Value, b: &Value) -> bool {
    ["counts","source_add","native","phases","detail","recovered","verified","exhausted"]
        .iter().all(|k| a[*k] == b[*k])
}
fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(),4,"usage: validate-wall WALL-RESULTS ISOLATION-JSONL OUTPUT-DIR");
    let root=Path::new(&args[1]); let iso=Path::new(&args[2]); let output=Path::new(&args[3]);
    fs::create_dir_all(output).unwrap();
    let mut expected=BTreeMap::<(u64,u64),Value>::new();
    for bits in BITS {
        let p=format!("research/factor_base_heldout_20261025/results/raw/evaluation-bits{bits}.jsonl");
        let rows=read(Path::new(&p)).unwrap();
        assert_eq!(rows.len(),60,"original frozen count {bits}");
        for row in rows { let index=row["target_index"].as_u64().unwrap();
            assert!(expected.insert((bits,index),row).is_none()); }
    }
    assert_eq!(expected.len(),420);
    let target_list=read(Path::new("research/factor_base_heldout_20261025/results/evaluation-targets.jsonl")).unwrap();
    assert_eq!(target_list.len(),420,"frozen target list");
    let mut target_keys=BTreeSet::new();
    for row in target_list {
        let key=(row["bits"].as_u64().unwrap(),row["target_index"].as_u64().unwrap());
        let original=expected.get(&key).expect("target absent from frozen original");
        assert_eq!(row["workload_id"],original["workload_id"]);
        assert_eq!(row["planted"],original["planted"]);
        assert!(target_keys.insert(key),"duplicate frozen target");
    }
    let processes=read(&root.join("processes.jsonl")).unwrap_or_default();
    let isolation=read(iso).unwrap_or_default();
    let io_pressure=fs::read_to_string(root.parent().unwrap().join("io-pressure.json"))
        .ok().and_then(|s|serde_json::from_str::<Value>(&s).ok()).unwrap_or(Value::Null);
    let io_ok=io_pressure["status"]=="attempted" &&
        io_pressure["before_some_avg10"].as_f64().is_some_and(|x|x<=5.0) &&
        io_pressure["after_some_avg10"].as_f64().is_some_and(|x|x<=5.0);
    let iso_ok=isolation.len()==1 && isolation[0]["run"]["exit_status"]==0
        && isolation[0]["run"]["contended"]==false
        && isolation[0]["reserved_cpus"]==json!([0,16]) && io_ok;
    let mut t=File::create(output.join("wall-targets.csv")).unwrap();
    writeln!(t,"slot,round,arm,bits,target_index,workload_id,planted,recovered,verified,wall_ns,status").unwrap();
    let mut p=File::create(output.join("wall-processes.csv")).unwrap();
    writeln!(p,"slot,round,arm,start_unix_ns,wall_ns,exit_code,rows,verified,io_some_avg10_before,io_some_avg10_after,passed,status").unwrap();
    let mut missing_count=0u64; let mut invalid_count=0u64; let mut valid_count=0u64;
    let mut durations=BTreeMap::<String,u128>::new();
    for (slot,round,arm) in SLOTS {
        let rec=processes.iter().find(|r|r["slot"]==slot);
        let raw=root.join("raw").join(format!("{slot}-{arm}.jsonl"));
        let observed=read(&raw).unwrap_or_default();
        let mut got=BTreeMap::<(u64,u64),Value>::new();
        let mut duplicates=BTreeSet::new();
        for row in observed {
            if let (Some(bits),Some(index))=(row["bits"].as_u64(),row["target_index"].as_u64()) {
                if got.insert((bits,index),row).is_some() {duplicates.insert((bits,index));}
            }
        }
        let process_ok=rec.is_some_and(|r| r["round"]==round && r["arm"]==arm &&
            r["passed"]==true && r["exit_code"]==0 && r["rows"]==420 && r["verified"]==420 &&
            r["io_some_avg10_before"].as_f64().is_some_and(|x|x<=5.0) &&
            r["io_some_avg10_after"].as_f64().is_some_and(|x|x<=5.0))
            && got.len()==420 && duplicates.is_empty();
        let process_wall=rec.and_then(|r|n(&r["wall_ns"]));
        if process_ok { if let Some(ns)=process_wall {durations.insert(slot.into(),ns);} }
        writeln!(p,"{slot},{round},{arm},{},{},{},{},{},{},{},{},{}",
            rec.map(|r|r["start_unix_ns"].as_str().unwrap_or("")).unwrap_or(""),
            process_wall.map(|x|x.to_string()).unwrap_or_default(),
            rec.and_then(|r|r["exit_code"].as_i64()).map(|x|x.to_string()).unwrap_or_default(),
            rec.and_then(|r|r["rows"].as_u64()).map(|x|x.to_string()).unwrap_or_default(),
            rec.and_then(|r|r["verified"].as_u64()).map(|x|x.to_string()).unwrap_or_default(),
            rec.and_then(|r|r["io_some_avg10_before"].as_f64()).map(|x|x.to_string()).unwrap_or_default(),
            rec.and_then(|r|r["io_some_avg10_after"].as_f64()).map(|x|x.to_string()).unwrap_or_default(),
            process_ok,if process_ok {"valid"} else if rec.is_none() {"not_run"} else {"invalid"}).unwrap();
        for ((bits,index),orig) in &expected {
            let row=got.get(&(*bits,*index));
            let reference=if arm=="candidate" {orig} else {&orig["rho"]};
            let ok=process_ok && row.is_some_and(|r| {
                r["wall_schema"]=="factor-base-heldout-wall-target/v1" && r["arm"]==arm &&
                r["workload_id"]==orig["workload_id"] && r["workload_sha256"]==orig["workload_sha256"] &&
                r["source_workload_id"]==orig["source_workload_id"] &&
                r["target"]==orig["target"] && r["algorithm_seed"]==orig["algorithm_seed"] &&
                r["cap"]==orig["cap"] && r["planted"]==orig["planted"] &&
                r["recovered"]==r["planted"] && r["verified"]==true &&
                n(&r["wall_ns"]).is_some_and(|x|x>0) && eq_fields(r,reference)
            });
            if ok {valid_count+=1;} else if row.is_none() {missing_count+=1;} else {invalid_count+=1;}
            let scalar=row.and_then(|r|r["recovered"].as_u64()).map(|x|x.to_string()).unwrap_or_default();
            let elapsed=row.and_then(|r|n(&r["wall_ns"])).map(|x|x.to_string()).unwrap_or_default();
            writeln!(t,"{slot},{round},{arm},{bits},{index},{},{},{},{},{},{}",
                csv(orig["workload_id"].as_str().unwrap()),orig["planted"],scalar,
                row.is_some_and(|r|r["verified"]==true),elapsed,
                if ok {"valid"} else if row.is_none() {"not_run"} else {"invalid"}).unwrap();
        }
    }
    let full=iso_ok && processes.len()==12 && valid_count==5040 &&
        missing_count==0 && invalid_count==0 && durations.len()==12;
    let aa_ratio=if full {Some(durations["aa1"] as f64/durations["aa2"] as f64)} else {None};
    let mut ratios=Vec::new();
    if full {for round in 1..=5 {
        let c=if round%2==1 {format!("ab{round}a")} else {format!("ab{round}b")};
        let r=if round%2==1 {format!("ab{round}b")} else {format!("ab{round}a")};
        ratios.push(durations[&c] as f64/durations[&r] as f64);
    }}
    let (median,minimum)=if full {let mut sorted=ratios.clone();sorted.sort_by(|a,b|a.total_cmp(b));
        (Some(sorted[2]),Some(sorted[0]))} else {(None,None)};
    let report=json!({"schema":"factor-base-heldout-wall-validation/v1","expected_observations":5040,
        "valid_observations":valid_count,"missing_observations":missing_count,
        "invalid_observations":invalid_count,"process_count":processes.len(),
        "isolation_valid":iso_ok,"io_pressure_valid":io_ok,"full_schedule_valid":full,"aa_ratio":aa_ratio,
        "round_candidate_over_rho":ratios,"median_candidate_over_rho":median,
        "minimum_candidate_over_rho":minimum,"operation_count_ratio_is_separate":true});
    fs::write(output.join("validation.json"),serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    println!("{report}");
    if !full {std::process::exit(1)}
}
