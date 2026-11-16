//! Native entry point. Performance eligibility is deliberately separate from a
//! successful solve: repository checkpoint/isolation/PCC gates remain required.
use courtois_native_study::*;
use serde_json::json;
use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let output = args
        .get(2)
        .ok_or("usage: courtois-native-study check|field-vectors OUTPUT.json")?;
    if fs::exists(output)? {
        return Err("refusing to replace prior evidence".into());
    }
    let value = match args.get(1).map(String::as_str) {
        Some("field-vectors") => serde_json::to_value(field_vectors())?,
        Some("check") => {
            let mut rows = Vec::new();
            for fixture in fixtures() {
                let c = NativeCurve::new(fixture)?;
                let secret = 137 % c.spec.q;
                let target = c.mul(c.g(), secret);
                for t in 3..=if c.spec.n <= 13 { 5 } else { 3 } {
                    for engine in ["plain", "cached", "symmetry", "pairtable"] {
                        let result = index_calculus(&c, target, t, engine, 701);
                        match result {
                            Ok(record)=>{assert_eq!(record.recovered,secret);rows.push(json!({"status":"producer_check_passed","record":record}));},
                            Err(error)=>rows.push(json!({"status":"failed","n":c.spec.n,"arity":t,"engine":engine,"error":error})),
                        }
                    }
                }
                let rho = native_rho(&c, target, 701);
                rows.push(json!({"method":"existing-native-generic-rho","n":c.spec.n,"recovered":rho.ok(),"target":target}));
            }
            json!({"schema":"courtois.native.correctness/v1","performance_eligible":false,
                   "claim_standing":"pcc:unverified","results":rows})
        }
        _ => return Err("unknown command".into()),
    };
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(output)?;
    file.write_all(serde_json::to_string_pretty(&value)?.as_bytes())?;
    Ok(())
}
