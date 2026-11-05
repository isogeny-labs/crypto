//! Exact source audit reused by the separately gated Lefschetz checker.
#[allow(dead_code)]
#[path = "../prime_first_fall_donor.rs"] mod donor;
#[path = "../prime_terminal_math.rs"] mod math;
#[path = "../prime_linear_source_donor.rs"] mod source_donor;
use serde_json::{json, Value};
use std::{collections::BTreeSet,fs,path::Path};
fn keys()->BTreeSet<String>{
    let mut out=BTreeSet::new();
    for arm in ["original","baseline","spike40"]{for m in 3..=9{for target in ["planted","natural"]{
        out.insert(format!("{arm}-prime-m{m}-{target}"));
    }}}out
}
fn check_keys(cases:&[Value]){
    assert_eq!(cases.len(),42);let got:BTreeSet<_>=cases.iter().map(|x|x["key"].as_str().unwrap().to_owned()).collect();
    assert_eq!(got,keys());
}
fn power(mut a:u128,mut e:u128,p:u128)->u128{
    let mut r=1;while e>0{if e&1==1{r=math::mul(r,a,p);}a=math::mul(a,a,p);e>>=1;}r
}
fn main(){
    let args:Vec<_>=std::env::args().collect();assert_eq!(args.len(),5,"run|replay ROOT PROTOCOL OUT");
    assert!(std::env::var("N83_LEFSCHETZ_FROZEN_PRODUCTION").as_deref()==Ok("1"));
    let replay=args[1]=="replay";assert!(replay || args[1]=="run");
    let root=Path::new(&args[2]);let protocol=math::read(Path::new(&args[3]));let out=Path::new(&args[4]);
    assert_eq!(protocol["freeze"]["status"],"frozen");let cases=protocol["extensions"]["cases"].as_array().unwrap();check_keys(cases);
    if replay{assert!(out.is_dir());}else{assert!(!out.exists());fs::create_dir_all(out).unwrap();}
    let save=|name:&str,v:&Value|{let p=out.join(name);if replay{assert_eq!(math::read(&p),*v);}else{math::save(&p,v);}};
    save("primality.json",&donor::primality());let p=donor::field();let mut rows=Vec::new();
    for case in cases{
        let key=case["key"].as_str().unwrap();let arm=case["arm"].as_str().unwrap();let dir=root.join("inputs").join(key);
        let bytes=fs::read(dir.join("input.json")).unwrap();assert_eq!(case["input_sha256"],math::sha(&bytes));
        let input:Value=serde_json::from_slice(&bytes).unwrap();let text=fs::read_to_string(dir.join("system.sing")).unwrap();
        assert_eq!(case["source_sha256"],math::sha(text.as_bytes()));
        assert_eq!(case["factor_base_manifest_sha256"],input["factor_base_manifest_sha256"]);
        let checked=source_donor::source(&input,&text,arm);let m=input["m"].as_u64().unwrap();
        assert_eq!(key,format!("{arm}-prime-m{m}-{}",input["target_kind"].as_str().unwrap()));
        let inverses:Vec<_>=checked["weights"].as_array().unwrap().iter().map(|w|{
            let w=w.as_str().unwrap().parse::<u128>().unwrap();assert!(w>0 && w<p);
            let inv=power(w,p-2,p);assert_eq!(math::mul(w,inv,p),1);inv.to_string()
        }).collect();
        let row=json!({"key":key,"m":m,"arm":arm,"target_kind":input["target_kind"],"p":p.to_string(),
            "input_sha256":case["input_sha256"],"source_sha256":case["source_sha256"],"factor_base_manifest_sha256":case["factor_base_manifest_sha256"],
            "source":checked,"normalization_inverses":inverses});
        save(&format!("{key}.json"),&row);rows.push(row);
    }
    save("summary.json",&json!({"schema":"n83.prime-source-audit/v1","rows":rows,"degree_claim":null}));
    println!("{}",json!({"cases_verified":42,"replay":replay,"degree_claim":null}));
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn full_scope_keys(){let cases:Vec<_>=keys().into_iter().map(|k|json!({"key":k})).collect();check_keys(&cases);}
    #[test]fn missing_and_duplicate_keys_rejected(){
        let mut cases:Vec<_>=keys().into_iter().map(|k|json!({"key":k})).collect();cases[0]=cases[1].clone();
        assert!(std::panic::catch_unwind(||check_keys(&cases)).is_err());cases.pop();
        assert!(std::panic::catch_unwind(||check_keys(&cases)).is_err());
    }
}
