use crate::ml_dataset::execution::LaunchContext;
use serde_json::json;
use sha2::{Digest,Sha256};
struct Ledger{work:u64,bytes:u64,stopped:bool,checks:u32,refuse_final:bool}
impl Ledger{fn new()->Self{Self{work:0,bytes:0,stopped:false,checks:0,refuse_final:false}}}
impl DecodeBudget for Ledger{
 fn reserve_checking(&mut self,w:u64,b:u64,_:Instant)->Result<(),Stop>{self.check(Instant::now())?;self.work+=w;self.bytes+=b;Ok(())}
 fn check(&mut self,_:Instant)->Result<(),Stop>{self.checks+=1;if self.stopped||(self.refuse_final&&self.checks>1){self.stopped=true;Err(Stop::Deadline)}else{Ok(())}}
 fn fail<T>(&mut self,s:Stop)->Result<T,Stop>{self.stopped=true;Err(s)}
}
struct Fixture{run:Vec<u8>,execution:Vec<u8>,manifest:Vec<u8>,binding:AttemptBinding,launch:LaunchContext,value:Value}
impl Fixture{
 fn new()->Self{
  let run=serde_json::to_vec(&json!({"schema":"crypto.ml-dataset.run/v1","seconds_per_case":60,"memory_mib":128,"trace_mib":8})).unwrap();
  let hash=|b:&[u8]|hex::encode(Sha256::digest(b));
  let binding=AttemptBinding{schema:"crypto.ml-dataset.attempt-binding/v1".into(),run_sha256:hash(&run),plan_sha256:"2".repeat(64),protocol_sha256:"3".repeat(64),config_sha256:"4".repeat(64),source_commit:"a".repeat(40),binary_sha256:"5".repeat(64),cell_index:7,attempt_sha256:"6".repeat(64)};
  let launch=LaunchContext{schema:"crypto.ml-dataset.execution-context/v1".into(),binding:binding.clone(),boot_id:"00000000-0000-0000-0000-000000000000".into(),start_monotonic_ns:10,deadline_monotonic_ns:60_000_000_010,seconds_per_case:60,memory_bytes:128<<20,trace_bytes:8<<20};
  let execution=serde_json::to_vec_pretty(&launch).unwrap();
  let mut artifacts=Vec::new();
  for (path,sha,n) in [("run.json",binding.run_sha256.clone(),run.len()),("plan.json",binding.plan_sha256.clone(),10),("protocol.json",binding.protocol_sha256.clone(),10),("config.json",binding.config_sha256.clone(),10),("attempt.json",binding.attempt_sha256.clone(),10),("public.json",hex::encode([0x12u8;32]),111),("factor-base.json",hex::encode([0x34u8;32]),222),("execution-context.json",hash(&execution),execution.len())]{artifacts.push(json!({"path":path,"sha256":sha,"bytes":n}));}
  let manifest=serde_json::to_vec(&json!({"schema":"crypto.ml-dataset.cell/v1","cell_index":7,"artifacts":artifacts})).unwrap();
  Self{run,execution,manifest,binding,launch,value:json!({})}
 }
 fn original(&self)->VerifiedWorkerOriginal<'_>{VerifiedWorkerOriginal{config:&self.value,public:&self.value,factor_base:&self.value,root:Path::new("fixture-only"),binding:Some(&self.binding),public_sha256:[0x12;32],factor_base_sha256:[0x34;32],public_bytes:111,factor_base_bytes:222}}
 fn derive(&self,b:&mut Ledger)->Result<capsule::Capsule,String>{capsule::derive(&self.original(),&self.launch,&self.run,&self.execution,&self.manifest,b)}
}
#[test]fn canonical_matches_actual_binding_digest(){let f=Fixture::new();let c=f.derive(&mut Ledger::new()).unwrap();assert_eq!(c.capture_binding_sha256,f.binding.digest().unwrap());assert!(!c.original_math_replayed_here);assert!(!c.cell_files_revalidated_here);}
#[test]fn pretty_raw_launch_is_not_typed_launch(){let f=Fixture::new();let c=f.derive(&mut Ledger::new()).unwrap();assert_ne!(c.launch_sha256,c.raw_execution_context_sha256);assert_eq!(c.launch_sha256,hex::encode(Sha256::digest(serde_json::to_vec(&f.launch).unwrap())));}
#[test]fn owner_domain_matches_original_recipe(){let f=Fixture::new();let c=f.derive(&mut Ledger::new()).unwrap();let mut h=Sha256::new();h.update(b"ml-dataset.sat-original-owner/v1\0");h.update(f.binding.digest().unwrap().as_bytes());h.update([0x12;32]);h.update([0x34;32]);assert_eq!(c.binding_sha256,hex::encode(h.finalize()));assert_ne!(c.binding_sha256,c.capture_binding_sha256);}
#[test]fn mismatched_typed_binding_refused(){let mut f=Fixture::new();f.launch.binding.cell_index+=1;assert!(f.derive(&mut Ledger::new()).unwrap_err().contains("execution context differs"));}
#[test]fn changed_launch_limits_refused(){let mut f=Fixture::new();f.launch.memory_bytes+=1;assert!(f.derive(&mut Ledger::new()).is_err());}
#[test]fn raw_run_mutation_refused(){let mut f=Fixture::new();f.run.push(b' ');assert!(f.derive(&mut Ledger::new()).unwrap_err().contains("raw run"));}
#[test]fn raw_execution_mutation_refused(){let mut f=Fixture::new();let mut v:Value=serde_json::from_slice(&f.execution).unwrap();v["start_monotonic_ns"]=json!(11);f.execution=serde_json::to_vec(&v).unwrap();assert!(f.derive(&mut Ledger::new()).unwrap_err().contains("raw/typed"));}
#[test]fn token_hash_substitution_refused(){let f=Fixture::new();let mut o=f.original();o.public_sha256=[0x55;32];assert!(capsule::derive(&o,&f.launch,&f.run,&f.execution,&f.manifest,&mut Ledger::new()).unwrap_err().contains("artifact mismatch"));}
#[test]fn duplicate_identity_artifact_refused(){let mut f=Fixture::new();let mut v:Value=serde_json::from_slice(&f.manifest).unwrap();let a=v["artifacts"][0].clone();v["artifacts"].as_array_mut().unwrap().push(a);f.manifest=serde_json::to_vec(&v).unwrap();assert!(f.derive(&mut Ledger::new()).unwrap_err().contains("duplicate"));}
#[test]fn manifest_mutation_is_fingerprinted(){let mut f=Fixture::new();let a=f.derive(&mut Ledger::new()).unwrap();f.manifest.push(b' ');let b=f.derive(&mut Ledger::new()).unwrap();assert_ne!(a.manifest_sha256,b.manifest_sha256);assert_eq!(a.binding_sha256,b.binding_sha256);}
#[test]fn stopped_budget_refuses_before_parse(){let f=Fixture::new();let mut b=Ledger::new();b.stopped=true;assert!(f.derive(&mut b).is_err());assert_eq!(b.work,0);}
#[test]fn final_budget_refusal_preserves_charge(){let f=Fixture::new();let mut b=Ledger::new();b.refuse_final=true;assert!(f.derive(&mut b).unwrap_err().contains("final check"));assert!(b.work>0&&b.bytes>0);}
#[test]fn raw_bound_precedes_admission(){let mut f=Fixture::new();f.run=vec![0;1024*1024+1];let mut b=Ledger::new();assert!(f.derive(&mut b).unwrap_err().contains("cap"));assert_eq!(b.work,0);}
#[test]fn original_token_must_be_bound(){let f=Fixture::new();let mut o=f.original();o.binding=None;assert!(capsule::derive(&o,&f.launch,&f.run,&f.execution,&f.manifest,&mut Ledger::new()).unwrap_err().contains("unbound"));}

#[test]fn independent_python_reference_matches(){let f=Fixture::new();let c=f.derive(&mut Ledger::new()).unwrap();assert_eq!(c.capture_binding_sha256,"e3b4f34863b881a771dce9ce44dc8c45b38d09a232e45c291dc6fc2af4df4ea2");assert_eq!(c.launch_sha256,"22271b6fea603ebbb998e76dfc25e8f7b8f2c715e53186e4e62e33e0cadc7a27");assert_eq!(c.binding_sha256,"68d99440e7faf74a0e8ccdac4136b69743d35944acf045e0fa677f9e0881040a");}
