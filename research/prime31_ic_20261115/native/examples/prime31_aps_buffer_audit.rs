//! Replays four COMPLETE prime31 APS producer records in fixed ABBA order.
//! Uses no secret scalar to solve; checks the caller's independent known-answer
//! labels and the existing Rust APS signed/oracle transcripts, then reports a
//! predeclared engineering gate without turning a null result into CI failure.
//! Native: cargo +1.100.0 run --release --example prime31_aps_buffer_audit
//!    old1.json new1.json new2.json old2.json
use serde_json::{json, Value};
use std::{env, fs};

const ARMS: [&str; 6] = [
    "APS_joint_binary", "APS_window4_sqrt", "APS_window4_public_y",
    "APS_window4_public_y_AA", "unfolded_rho32", "unfolded_rho64",
];
const APS_KEYS: [&str; 8] = [
    "round", "seed", "s3_pair_quadratics", "s3_batch_inversions",
    "signed_candidates", "solution", "rejected_duplicate_x",
    "rejected_identity",
];

fn load(path: &str) -> Result<Value, String> {
    let s = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    serde_json::from_str(&s).map_err(|e| format!("{path}: {e}"))
}
fn number(v: &Value, path: &str) -> Result<u64, String> {
    v.as_u64().ok_or_else(|| format!("missing/non-u64 {path}"))
}
fn validate_panel(v: &Value, label: &str) -> Result<(), String> {
    for (key, expected) in [("p", 2147483647),("n",2147446831),("cofactor",1),
                            ("cases_count",24)] {
        if number(&v[key], key)? != expected {
            return Err(format!("{label}: bad {key}"));
        }
    }
    if v["profile"] != json!([384,384,16384]) ||
       v["all_144_complete"] != true ||
       v["completed_calls"] != json!([24,24,24,24,24,24]) {
        return Err(format!("{label}: incomplete calls / changed profile"));
    }
    let cs = v["cases"].as_array().ok_or("missing cases")?;
    if cs.len()!=24 { return Err(format!("{label}: expected 24 cases")); }
    for (i,c) in cs.iter().enumerate() {
        if number(&c["case"], "case")? != i as u64 {
            return Err(format!("{label}: bad case index {i}"));
        }
        let k = number(&c["caller_known_scalar_verifier_only"], "caller k")?;
        if k == 0 || k >= 2147446831 { return Err(format!("{label}: invalid caller k")); }
        if !c["public_Q"].is_array() { return Err(format!("{label}: no public Q")); }
        for key in ARMS {
            let a=&c[key];
            if a["complete_target_scalar_recovered"] != true {
                return Err(format!("{label} case {i} arm {key} incomplete"));
            }
            let log=if key.starts_with("APS_") {
                a["attempts"].as_array()
                    .and_then(|x|x.last()).and_then(|x|x["solution"]["log"].as_u64())
            }else { a["log"].as_u64() };
            if log!=Some(k) { return Err(format!("{label} case {i} arm {key} wrong log")); }
            number(&a["cold_wall_ns"], "cold_wall_ns")?;
        }
    }
    Ok(())
}
fn equivalent(a: &Value, b: &Value, label: &str) -> Result<(), String> {
    for key in ["p","n","cofactor","profile","cases_count","target_seed","rho_seed_base"] {
        if a[key]!=b[key] { return Err(format!("{label}: mismatch header {key}")); }
    }
    let ac=a["cases"].as_array().ok_or("a cases")?;
    let bc=b["cases"].as_array().ok_or("b cases")?;
    for (i,(x,y)) in ac.iter().zip(bc).enumerate() {
        for key in ["case","public_Q","caller_known_scalar_verifier_only"] {
            if x[key]!=y[key] { return Err(format!("{label}: target {i} changed {key}")); }
        }
        for arm in ARMS {
            if x[arm]["complete_target_scalar_recovered"]!=y[arm]["complete_target_scalar_recovered"] {
                return Err(format!("{label}: status changed {i} {arm}"));
            }
            if arm.starts_with("APS_") {
                let ar=x[arm]["attempts"].as_array().ok_or("missing old attempts")?;
                let br=y[arm]["attempts"].as_array().ok_or("missing new attempts")?;
                if ar.len()!=br.len() {return Err(format!("{label}: round count {i} {arm}"));}
                for (j,(old,new)) in ar.iter().zip(br).enumerate() {
                    for key in APS_KEYS {
                        if old[key]!=new[key] {
                            return Err(format!("{label} case {i} arm {arm} round {j}: {key} changed"));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
fn price(panel:&Value)->Result<Vec<u64>,String> {
    panel["cases"].as_array().ok_or("missing cases")?.iter()
        .map(|v| number(&v["APS_window4_public_y"]["cold_wall_ns"],"composed cold"))
        .collect()
}
fn evaluate(old:&Value,new:&Value)->Result<Value,String> {
    let o=price(old)?;let n=price(new)?;
    let old_total:u128=o.iter().map(|&x|x as u128).sum();
    let new_total:u128=n.iter().map(|&x|x as u128).sum();
    if old_total==0 {return Err("zero baseline cost".into());}
    let wins=o.iter().zip(&n).filter(|(a,b)|b<a).count();
    let ratio=new_total as f64/old_total as f64;
    let drift=new["candidate_AA_drift"].as_f64()
        .ok_or("missing APS composed/AA drift")?;
    Ok(json!({
        "old_ns":old_total.to_string(),"candidate_ns":new_total.to_string(),
        "ratio":ratio,"candidate_paired_wins":wins,
        "candidate_internal_AA_drift":drift,
        "predeclared_gate_pass":ratio<=0.95 && wins>=18 && drift<=0.05
    }))
}
fn run(args:&[String])->Result<Value,String>{
    if args.len()!=4 { return Err("usage: old1 new1 new2 old2".into()); }
    let panels:Vec<Value>=args.iter().map(|p|load(p)).collect::<Result<_,_>>()?;
    for (i,p) in panels.iter().enumerate() {validate_panel(p,&format!("panel{}",i))?;}
    // Every cross-arm transcript and public target must match: failed
    // equality is a correctness/measurement failure, not an optional gate.
    for i in 1..4 {equivalent(&panels[0],&panels[i],&format!("panel{i}"))?;}
    let first=evaluate(&panels[0],&panels[1])?;
    let second=evaluate(&panels[3],&panels[2])?;
    let old1:u128=price(&panels[0])?.iter().map(|&v|v as u128).sum();
    let old2:u128=price(&panels[3])?.iter().map(|&v|v as u128).sum();
    let new1:u128=price(&panels[1])?.iter().map(|&v|v as u128).sum();
    let new2:u128=price(&panels[2])?.iter().map(|&v|v as u128).sum();
    let old_drift=((old1 as f64/old2 as f64)-1.0).abs();
    let new_drift=((new1 as f64/new2 as f64)-1.0).abs();
    let accepted=first["predeclared_gate_pass"]==true &&
        second["predeclared_gate_pass"]==true &&
        old_drift<=0.05 && new_drift<=0.05;
    Ok(json!({
        "schema":"prime31.aps.buffer_reuse.audit.v1",
        "standing":"[pcc:unverified]",
        "method":"published APS S3 one-relation, explicit generic pair oracle",
        "curve_p":2147483647_u64,"group_order":2147446831_u64,
        "fixture":"24 new fixed-seed public targets per panel, 4 complete ABBA panels",
        "complete_cold_calls_checked":576,
        "first_old_then_new":first,"second_new_then_old":second,
        "old_repeat_aggregate_drift":old_drift,
        "new_repeat_aggregate_drift":new_drift,
        "protocol_gate_pass":accepted,
        "qualification":"producer CI, no isolated host, no WZ or external fastest-frontier result"
    }))
}
fn main()->Result<(),String>{
    let args:Vec<String>=env::args().skip(1).collect();
    let result=run(&args)?;
    println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gate_rejects_bad_panels() {
        assert!(validate_panel(&json!({}),"bad").is_err());
        assert!(run(&[]).is_err());
        assert!(number(&json!(null),"missing").is_err());
    }
    #[test]
    fn price_and_result_guard() {
        assert!(price(&json!({})).is_err());
        assert!(evaluate(&json!({}),&json!({})).is_err());
    }
}
