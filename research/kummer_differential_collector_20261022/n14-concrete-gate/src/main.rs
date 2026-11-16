//! Frozen finite-state replay of the remaining concrete theta models.
//! No targets are solved and no all-state bound is inferred.
use five_leads_study::three_leads::{Counts, Divisor, Field, Jacobian};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::{BTreeMap,BTreeSet}, fs};

mod theta {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/walk_impl.rs"));
    pub fn projection(d: Divisor, j: &Jacobian, t: &Value, acts: &[(u64, [[u64;4];4])], p: u64) -> Option<[u64;4]> {
        project_full(d,j,t,acts,p)
    }
    pub fn raw(x:[u64;4],y:[u64;4],d:[u64;4],t:[u64;4],p:u64)->Option<[u64;4]> { xadd(x,y,d,t,p) }
    pub fn repaired(x:[u64;4],y:[u64;4],d:[u64;4],t:[u64;4],acts:&[(u64,[[u64;4];4])],p:u64)->(Option<[u64;4]>,Option<u64>){xadd_repaired(x,y,d,t,acts,p)}
    pub fn equivalent(x:[u64;4],y:[u64;4],p:u64)->bool{same(x,y,p)}
}
mod signed {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/recover_impl.rs"));
    pub fn lift_one(g:Divisor,k:[u64;4],kp:[u64;4],kg:[u64;4],t:&Value,acts:&[(u64,[[u64;4];4])],j:&Jacobian,p:u64)->(Option<Divisor>, &'static str,usize){
        let constants=["A","B","C","D"].map(|key|t[key].as_u64().unwrap());
        let (km,_) = xadd_repaired(k,kg,kp,constants,acts,p);
        let (q,plus,minus)=(general(k,t,p),general(kp,t,p),km.and_then(|x|general(x,t,p)));
        if let (Some(q),Some(plus),Some(minus))=(q,plus,minus){
            let f=curve(t,p);
            let (mut out,mut status)=recover(g,q,plus,minus,&f,p);
            let mut tested=0;
            if out.is_none() && status=="D_zero"{
                let (candidate,n)=exceptional_lift(g,q,plus,&f,j,p);
                out=candidate; tested=n;
                if out.is_some(){status="enumerated_exception_lift";}
            }
            (out,status,tested)
        }else{(None,"general_conversion_inapplicable",0)}
    }
}
fn digest(path:&str)->String{let bytes=fs::read(path).unwrap();format!("{:x}",Sha256::digest(bytes))}
fn rows(path:&str)->Vec<Value>{fs::read_to_string(path).unwrap().lines().map(|s|serde_json::from_str(s).unwrap()).collect()}
fn v4(v:&Value)->[u64;4]{std::array::from_fn(|i|v[i].as_u64().unwrap())}
fn matrix(v:&Value)->[[u64;4];4]{std::array::from_fn(|i|v4(&v[i]))}
fn inv(mut x:u64,p:u64)->u64{let mut e=p-2;let mut z=1;while e>0{if e&1>0{z=z*x%p}x=x*x%p;e>>=1;}z}
fn curve(t:&Value,p:u64)->Vec<u64>{
    let mut f=vec![0;6];f[1]=1;
    for root in [1,t["lambda"].as_u64().unwrap(),t["mu"].as_u64().unwrap(),t["nu"].as_u64().unwrap()]{
        let mut g=vec![0;6];for i in 0..5{g[i+1]=(g[i+1]+f[i])%p;g[i]=(g[i]+p-f[i]*root%p)%p;}f=g;
    }f
}
fn classify(raw:Option<[u64;4]>,repaired:Option<[u64;4]>,expected:[u64;4],p:u64)->(&'static str,&'static str){
    let one=|x:Option<[u64;4]>|match x{None=>"undefined",Some(v) if theta::equivalent(v,expected,p)=>"match",Some(_)=>"mismatch"};
    (one(raw),one(repaired))
}
fn num(v:&Value,key:&str)->u64{v[key].as_u64().unwrap()}
fn sample(r:u64)->Vec<u64>{
    let mut keys=BTreeSet::from([0,1,2,r-2,r-1]);
    for i in 0..1024u64 { keys.insert(((2*i+1) as u128*r as u128/2048) as u64); }
    keys.into_iter().collect()
}
fn main(){
    let base="research/kummer_differential_collector_20261022/n14-concrete-gate";
    let theta_path="research/kummer_theta_20261022/results/theta-models.jsonl";
    let actions_path="research/kummer_coordinates_20261022/results/generic-models.jsonl";
    let walk_path="research/kummer_differential_collector_20261022/src/walk_impl.rs";
    let recover_path="research/kummer_differential_collector_20261022/src/recover_impl.rs";
    let mut source_paths=vec![theta_path.to_owned(),actions_path.to_owned(),walk_path.to_owned(),recover_path.to_owned(),format!("{base}/PROTOCOL.md"),format!("{base}/src/main.rs")];
    let thetas: BTreeMap<u64,Value>=rows(theta_path).into_iter().map(|v|(num(&v,"p"),v)).collect();
    let actions: BTreeMap<u64,Value>=rows(actions_path).into_iter().map(|v|(num(&v,"p"),v)).collect();
    let mut targets=Vec::new();
    for bits in [20,24,28,32,36,40] {
        let path=format!("research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl");
        let slice:Vec<_>=rows(&path).into_iter().filter(|v|num(v,"target_index")==0).collect();
        assert_eq!(slice.len(),3,"one frozen target per model at {bits} bits");
        targets.extend(slice);source_paths.push(path);
    }
    assert_eq!(targets.len(),18);
    let mut csv=String::from("schema,bits,p,r,k,projection,raw_dbl,repair_dbl,raw_mix,repair_mix,raw_plus_dbl,repair_plus_dbl,diff_zero_coordinates,repair_roots,lift_status,lift_correct,lift_m,lift_a,lift_i,lift_jac_m,lift_jac_s,lift_jac_i,lift_jac_a,lift_jac_bitops,lift_jac_ecadds,lift_jac_jacadds,lift_jac_sqrt,lift_exception_candidates\n");
    let mut summary=Vec::new();
    for target in targets{
        let p=num(&target,"p");let r=num(&target,"r");let bits=num(&target,"bits");let tr=&thetas[&p];let selected=&tr["selected"];
        let t=&selected["constants"];let r0=num(selected,"r0");let delta=num(selected,"delta");let di=inv(delta,p);
        let acts:Vec<_>=actions[&p]["torsion_actions"].as_array().unwrap().iter().map(|v|(((num(v,"root")+p-r0)%p)*di%p,matrix(&v["action"]["matrix"]))).collect();
        let consts=["A","B","C","D"].map(|key|num(t,key));
        assert!(consts.iter().all(|&x|x!=0),"theta inverse constant zero");
        theta::setup_constants(consts,p);signed::setup_constants(consts,p);
        let g:Divisor=serde_json::from_value(target["cases"][1]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();
        let j=Jacobian{f:Field::new(p,1,0),polynomial:curve(t,p)};
        assert!(j.valid(g));assert_eq!(j.mul(g,r),Divisor::O);
        let kg=theta::projection(g,&j,t,&acts,p).unwrap();let k0=theta::projection(Divisor::O,&j,t,&acts,p).unwrap();
        let sample=sample(r);
        let mut counts=BTreeMap::<String,u64>::new();
        let mut lift_counts=BTreeMap::<String,u64>::new();
        let mut lift_totals=Counts::default();let (mut lm,mut la,mut li,mut tested_total)=(0,0,0,0usize);
        for &k in &sample{
            let q=j.mul(g,k);
            let qplus=j.add(q,g);let q2=j.add(q,q);let q2g=j.add(q2,g);let q2gg=j.add(q2g,g);
            let points=[q,qplus,q2,q2g,q2gg];
            let coords:Option<Vec<_>>=points.iter().map(|&d|theta::projection(d,&j,t,&acts,p)).collect();
            let mut out=[("missing","missing");3];let mut roots=[None;3];let mut zeroes=[0usize;3];
            let projection=if let Some(c)=coords.as_ref(){
                let inputs=[(c[0],c[0],k0,c[2]),(c[0],c[1],kg,c[3]),(c[1],c[1],k0,c[4])];
                for (i,(x,y,d,expected)) in inputs.into_iter().enumerate(){
                    zeroes[i]=d.iter().filter(|&&v|v==0).count();
                    let raw=theta::raw(x,y,d,consts,p);
                    let (repair,root)=theta::repaired(x,y,d,consts,&acts,p);
                    out[i]=classify(raw,repair,expected,p);roots[i]=root;
                }"available"
            }else{"missing"};
            *counts.entry(format!("projection:{projection}")).or_default()+=1;
            for (i,name) in ["dbl","mix","plus_dbl"].iter().enumerate(){
                *counts.entry(format!("{name}:raw:{}",out[i].0)).or_default()+=1;
                *counts.entry(format!("{name}:repair:{}",out[i].1)).or_default()+=1;
                *counts.entry(format!("{name}:diff_zero_count:{}",zeroes[i])).or_default()+=1;
            }
            let (lift_status,lift_correct,ops,field,candidates)=
                if let Some(c)=coords.as_ref(){
                    signed::reset_ops();j.f.reset();
                    let (lift,status,n)=signed::lift_one(g,c[0],c[1],kg,t,&acts,&j,p);
                    let ops=signed::read_ops();let field=j.f.counts();
                    let correct=(lift==Some(q)).to_string();
                    *lift_counts.entry(format!("{status}:{correct}")).or_default()+=1;
                    lift_totals=lift_totals+field;lm+=ops.m;la+=ops.a;li+=ops.i;tested_total+=n;
                    (status.to_owned(),correct,ops,field,n)
                }else{*lift_counts.entry("projection_missing:false".into()).or_default()+=1;
                    ("projection_missing".to_owned(),"false".to_owned(),signed::Ops::default(),Counts::default(),0usize)};
            csv.push_str(&format!("kummer-n14-state/v1,{bits},{p},{r},{k},{projection},{},{},{},{},{},{},{},{},{lift_status},{lift_correct},{},{},{},{},{},{},{},{},{},{},{},{}\n",out[0].0,out[0].1,out[1].0,out[1].1,out[2].0,out[2].1,zeroes.iter().map(usize::to_string).collect::<Vec<_>>().join("|"),roots.iter().map(|v|v.map_or(String::new(),|x|x.to_string())).collect::<Vec<_>>().join("|"),ops.m,ops.a,ops.i,field.m,field.s,field.i,field.field_adds,field.bit_ops,field.ec_adds,field.jac_adds,field.sqrt_calls,candidates));
        }
        let fallback_success=*lift_counts.get("enumerated_exception_lift:true").unwrap_or(&0);
        let final_no_divisor:u64=lift_counts.iter().filter(|(key,_)|key.ends_with(":false")).map(|(_,value)|*value).sum();
        let direct_fast_recover_fail=fallback_success+final_no_divisor;
        summary.push(json!({"bits":bits,"p":p,"r":r,"sample_states":sample.len(),"sample_rule":"boundary-5 plus 1024 midpoint strata","counts":counts,"lift_counts":lift_counts,
            "denominator_guards":{"theta_inverse_constants":consts,"theta_inverse_zero_count":0,"k0":k0,"kg":kg,"k0_zero_coordinates":k0.iter().filter(|&&x|x==0).count(),"kg_zero_coordinates":kg.iter().filter(|&&x|x==0).count()},
            "lift_accounting":{"direct_fast_recover_failed":direct_fast_recover_fail,"enumerated_fallback_succeeded":fallback_success,"final_no_divisor":final_no_divisor,"unaccounted_states":0},
            "lift_ops_totals":{"m":lm,"a":la,"i":li,"jacobian_field_counts":lift_totals,"exception_candidates":tested_total},"input_target_workload_id":target["workload_id"]}));
        eprintln!("model bits={bits} p={p} r={r} sampled={} done",sample.len());
    }
    fs::create_dir_all(format!("{base}/results")).unwrap();
    let states_sha256=format!("{:x}",Sha256::digest(csv.as_bytes()));
    fs::write(format!("{base}/results/states.csv"),csv).unwrap();
    let sources:Vec<_>=source_paths.iter().map(|path|json!({"path":path,"sha256":digest(path)})).collect();
    let packet=json!({"schema":"kummer-n14-concrete-gate/v1","scope":"frozen 1029 signed states per model for the 18 remaining models; finite sample only","source_digests":sources,"states_sha256":states_sha256,"models":summary,"all_state_exception_bound":null,"cohort_eligible":false});
    fs::write(format!("{base}/results/summary.json"),format!("{}\n",serde_json::to_string_pretty(&packet).unwrap())).unwrap();
}
