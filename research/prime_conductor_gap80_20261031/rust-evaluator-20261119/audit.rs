//! Serialize native Rust receipts for independent PARI; no curve arithmetic.
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
fn read(p:impl AsRef<Path>)->Value {serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn number(v:&Value)->Value {if let Some(s)=v.as_str(){serde_json::from_str(s).unwrap()}else{v.clone()}}
fn point(v:&Value)->Value {if v.is_null(){json!([0])}else{Value::Array(v.as_array().unwrap().iter().map(number).collect())}}
fn receipt(p:&Path)->Value {
    let r=read(p);assert_eq!(r["implementation"],"rust_complete_twist_v1");assert_eq!(r["status"],"complete_evaluation");
    let rows=r["images"].as_array().unwrap();
    json!([r["codomain"].as_array().unwrap().iter().map(number).collect::<Vec<_>>(),rows.iter().map(|r|point(&r["input"])).collect::<Vec<_>>(),rows.iter().map(|r|point(&r["image"])).collect::<Vec<_>>(),number(&r["kernel_pairs_covered"]),number(&r["ell"]),number(&r["p"])])
}
fn main(){
    let args:Vec<_>=std::env::args().collect();assert_eq!(args.len(),3,"study main|dual");let study=Path::new(&args[1]);
    let baseline=Path::new("research/prime_conductor_gap80_20261031/twist-transport-20261119");
    let cv=read(baseline.join("case0.gpvec"));
    let mut data=vec![cv,receipt(&study.join("results/main/result.json")),receipt(&study.join("results/probe/result.json"))];
    for (rust,old) in [("main","case0-main-b"),("probe","case0-probe-b")]{
        let new=read(study.join(format!("results/{rust}/result.json")));let reference=read(baseline.join(format!("results/{old}/stdout")));
        for field in ["status","p","source","ell","codomain","baby_count","giant_count","kernel_pairs_covered","images"]{assert_eq!(new[field],reference[field],"{rust}:{field}");}
    }
    if args[2]=="dual"{data.push(receipt(&study.join("results/dual/result.json")));}
    else{assert_eq!(args[2],"main");}
    let name=if args[2]=="dual"{"dual-audit.gpvec"}else{"main-audit.gpvec"};
    fs::write(study.join(name),serde_json::to_vec(&data).unwrap()).unwrap();
    println!("native Rust receipts serialized; main/probe match retained reference; independent PARI follows");
}
