use serde_json::{Value,json};
use std::{fs,collections::{BTreeMap,BTreeSet}};
fn f(v:&Value,k:&str)->f64 { let n=v[k].as_f64().unwrap(); assert!(n.is_finite()); n }
fn q(v:&[f64],p:f64)->f64 { let mut a=v.to_vec(); a.sort_by(f64::total_cmp); a[((a.len()-1) as f64*p).round() as usize] }
fn eq(a:f64,b:f64){ assert!((a-b).abs()<=1e-12*(1.0+a.abs()+b.abs()),"{a} != {b}") }
fn main(){
let dir=std::env::args().nth(1).unwrap();
let raw=fs::read_to_string(format!("{dir}/rows_v3.jsonl")).unwrap();
let rows:Vec<Value>=raw.lines().map(|s|serde_json::from_str(s).unwrap()).collect(); assert_eq!(rows.len(),9015);
let s:Value=serde_json::from_str(&fs::read_to_string(format!("{dir}/results_v3.json")).unwrap()).unwrap();
assert_eq!(s["mode"],"v3"); assert_eq!(s["N"],json!({"E1b":800,"E1b_pos":50,"E4r":500}));
let mut counts=BTreeMap::<String,usize>::new(); for r in &rows {*counts.entry(format!("{}:{}",r["exp"].as_str().unwrap(),r["arm"].as_str().unwrap())).or_default()+=1;}
assert_eq!(counts,BTreeMap::from([("E1b:control".into(),7200),("E1b:target".into(),9),("E1b:positive".into(),150),("E4r:control".into(),1500),("E4r:target".into(),3),("E4r:positive".into(),3),("E4r:parallelogram".into(),150)]));
let mut notes=vec![]; let mut validated=0;
for curve in ["P-192","P-224","P-256"] {
 let mut unsplit=0; let mut changed=0; let mut e1pos=0;
 for val in ["n-1","n+1","twist"] {
  let rs:Vec<_>=rows.iter().filter(|r|r["curve"]==curve&&r["exp"]=="E1b"&&r["arm"]=="control"&&r["value"]==val).collect(); assert_eq!(rs.len(),800);
  let ids:BTreeSet<_>=rs.iter().map(|r|r["i"].as_u64().unwrap()).collect(); assert_eq!(ids,(0..800).collect());
  for r in rs { unsplit+=r["unsplit"].as_u64().unwrap(); changed+=usize::from((f(r,"M1")-f(r,"M1_rho")).abs()>1e-9); eq(f(r,"M2"),f(r,"M2_rho")); }
 }
 for r in rows.iter().filter(|r|r["curve"]==curve&&r["exp"]=="E1b"&&r["arm"]=="positive") { let hit=(f(r,"M1")-f(r,"planted_log2")).abs()<1e-6; assert_eq!(r["found"],hit); e1pos+=usize::from(hit); }
 assert_eq!(e1pos,50); assert_eq!(changed,0);
 let ctrl:Vec<_>=rows.iter().filter(|r|r["curve"]==curve&&r["exp"]=="E4r"&&r["arm"]=="control").collect(); assert_eq!(ctrl.len(),500);
 let zeros=ctrl.iter().filter(|r|f(r,"abs_cos")==0.0).count(); let near=ctrl.iter().filter(|r|f(r,"abs_cos")<=1e-12).count();
 let pars:Vec<_>=rows.iter().filter(|r|r["curve"]==curve&&r["exp"]=="E4r"&&r["arm"]=="parallelogram").collect(); assert_eq!(pars.len(),50);
 assert_eq!(pars.iter().map(|r|r["i"].as_u64().unwrap()).collect::<BTreeSet<_>>(),(0..50).collect());
 let parmax=pars.iter().map(|r|f(r,"defect").abs()).fold(0f64,f64::max); assert!(parmax<=1e-2);
 let positives:Vec<_>=rows.iter().filter(|r|r["curve"]==curve&&r["exp"]=="E4r"&&r["arm"]=="positive").collect(); assert_eq!(positives.len(),1);
 for name in ["g2","g3"] { let a=positives[0][name].as_array().unwrap(); assert_eq!(a.len(),4);assert!((a[0].as_f64().unwrap()-1.0).abs()<=1e-4 && a[1].as_f64().unwrap().abs()<=1e-4); }
 notes.push(json!({"curve":curve,"control_values":2400,"unsplit_composites":unsplit,"M1_changed":changed,"E1_positive_passed":e1pos,"pair_controls":500,"float_exact_zero":zeros,"abs_cos_le_1e_minus_12":near,"parallelogram_max_abs":parmax}));
}
for rec in s["summary"].as_array().unwrap() {
 if rec.get("metric").is_none() {continue}
 let e=rec["exp"].as_str().unwrap(); let curve=rec["curve"].as_str().unwrap(); let metric=rec["metric"].as_str().unwrap();
 let key=match metric {"|cos|"=>"abs_cos","det/(ĥĥ)"=>"norm_reg",x=>x};
 let matching=|r:&&Value|r["exp"]==e&&r["curve"]==curve&&(e!="E1b"||r["value"]==rec["value"]);
 let cv:Vec<f64>=rows.iter().filter(matching).filter(|r|r["arm"]=="control").map(|r|f(r,key)).collect();
 let targets:Vec<_>=rows.iter().filter(matching).filter(|r|r["arm"]=="target").collect();assert_eq!(targets.len(),1);
 let t=f(targets[0],key); let n=cv.len();let le=cv.iter().filter(|&&v|v<=t).count();let ge=cv.iter().filter(|&&v|v>=t).count();let rank=cv.iter().filter(|&&v|v<t).count();
 let p=(2.0*(1+le.min(ge)) as f64/(n+1) as f64).min(1.0); let alpha=if e=="E1b"{0.05/9.0}else{0.05/6.0};
 eq(f(rec,"target"),t); eq(f(rec,"median"),q(&cv,0.5)); eq(f(rec,"p"),p); eq(f(rec,"alpha"),alpha);assert_eq!(rec["rank"],rank);assert_eq!(rec["N"],n);assert_eq!(rec["unusual"],p<alpha);
 if e=="E1b" { eq(f(rec,"target_rho"),f(targets[0],"M1_rho"));let us:u64=rows.iter().filter(matching).filter(|r|r["arm"]=="control").map(|r|r["unsplit"].as_u64().unwrap()).sum(); assert_eq!(rec["unsplit"],us);assert_eq!(rec["controls_changed"],0); }
 else {eq(f(rec,"q025"),q(&cv,0.025));eq(f(rec,"q975"),q(&cv,0.975));let k4=format!("{key}_k4");let ds:Vec<_>=rows.iter().filter(matching).filter(|r|r["arm"]=="control").map(|r|(f(r,key)-f(r,&k4)).abs()).collect();eq(f(rec,"median_abs_k5_minus_k4"),q(&ds,0.5));let pos=rows.iter().find(|r|r["exp"]==e&&r["curve"]==curve&&r["arm"]=="positive").unwrap();let idx=usize::from(key=="norm_reg");for(i,name)in ["g2","g3"].iter().enumerate(){eq(rec["dependent"][i].as_f64().unwrap(),pos[name][idx].as_f64().unwrap());}}
 validated+=1;
}
assert_eq!(validated,15);let gate=s["summary"].as_array().unwrap().last().unwrap();assert_eq!(gate["accuracy_gate_passed"],true);assert_eq!(gate["parallelogram_gate_passed"],true);
println!("{}",serde_json::to_string_pretty(&json!({"status":"PASS_RETAINED_STATISTICAL_REPLAY","rows":rows.len(),"counts":counts,"summary_metrics_verified":validated,"curves":notes,"scope":"Stored row summaries only; no independent recomputation of factoring or heights; no new experiment run"})).unwrap());
}
