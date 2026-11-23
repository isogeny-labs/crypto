//! Exact paired-input identity checks and forwarding of native certificate fields.
use serde_json::{json, Value};
use std::{fs, path::Path};
fn read(p: &Path) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn main() {
 let args:Vec<_>=std::env::args().collect(); let root=Path::new(&args[1]);
 let base=read(&root.join("baseline/summary.json")); let candidate=read(&root.join("spike40/summary.json"));
 let mut rows=vec![];
 for b in base["rows"].as_array().unwrap() {
  let c=candidate["rows"].as_array().unwrap().iter().find(|r|r["id"]==b["id"]).unwrap();
  let bi=&b["first_fall"]["input"]; let ci=&c["first_fall"]["input"];
  for key in ["family","n","m","ell","n_vars","target_kind","target_x","target_y","points","witness"] { assert_eq!(bi[key],ci[key],"paired input mismatch: {key}"); }
  assert_eq!(bi["arm"],"baseline"); assert_eq!(ci["arm"],"spike40");
  let view=|r:&Value|json!({"first_fall_degree":r["first_fall"]["first_fall_degree"],"first_fall_lower_bound":r["first_fall"]["first_fall_lower_bound"],"first_fall_status":r["first_fall"]["status"],"last_fall_degree":r["last_fall_degree"],"last_fall_lower_bound":r["last_fall_lower_bound"],"closure_rank":r["closure_rank"],"linear_rank":r["linear_rank"],"closure_status":r["closure_status"],"contains_one":r["contains_one"],"exhaustive_affine_completion":r["exhaustive_affine_completion"],"source_sha256":r["first_fall"]["system_sha256"],"manifest_sha256":r["first_fall"]["input"]["factor_base_manifest_sha256"]});
  rows.push(json!({"id":b["id"],"family":b["family"],"m":b["m"],"target_kind":b["target_kind"],"targets_identical":true,"baseline":view(b),"spike40":view(c),"degree_of_regularity":null,"solving_degree":null}));
 }
 assert_eq!(rows.len(),48);
 let out=json!({"schema":"n83.paired-fall-panel/v1","paired_cases":48,"systems":96,"rows":rows,"scope":"Source-specific certificates and bounds only. Unknown degrees remain null. No regularity or solver-speed improvement established."});
 fs::write(root.join("paired-summary.json"),serde_json::to_string_pretty(&out).unwrap()+"\n").unwrap();
}
