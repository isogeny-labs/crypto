//! Private identity derivation after original cell admission, not a cell loader.
use super::{VerifiedWorkerOriginal, DecodeBudget, Stop};
use crate::ml_dataset::{capture::AttemptBinding,execution::LaunchContext};
use serde::Serialize;
use sha2::{Digest,Sha256};
use std::{io::{self,Write},time::Instant};
const RAW_CAP:usize=1024*1024;
const TYPED_CAP:usize=65536;
#[derive(Debug,Serialize)]
pub(super) struct Capsule {
 pub schema:&'static str,
 pub capture_binding_sha256:String,
 pub public_sha256:String,
 pub factor_sha256:String,
 pub launch_sha256:String,
 pub binding_sha256:String,
 pub manifest_sha256:String,
 pub raw_execution_context_sha256:String,
 pub run_sha256:String,
 pub original_math_replayed_here:bool,
 pub cell_files_revalidated_here:bool,
}
struct HashWriter{hash:Sha256,bytes:usize}
impl Write for HashWriter{
 fn write(&mut self,b:&[u8])->io::Result<usize>{
  let next=self.bytes.checked_add(b.len()).filter(|n|*n<=TYPED_CAP).ok_or_else(||io::Error::other("typed identity bound"))?;
  self.hash.update(b);self.bytes=next;Ok(b.len())
 }
 fn flush(&mut self)->io::Result<()>{Ok(())}
}
fn typed_hash<T:Serialize>(v:&T)->Result<String,String>{
 let mut w=HashWriter{hash:Sha256::new(),bytes:0};
 serde_json::to_writer(&mut w,v).map_err(|e|e.to_string())?;
 Ok(hex::encode(w.hash.finalize()))
}
fn raw_hash(b:&[u8])->String{hex::encode(Sha256::digest(b))}
fn require(ok:bool,why:&str)->Result<(),String>{if ok{Ok(())}else{Err(why.into())}}
/// Only the already-admitted central parent may invoke this. `original` must be
/// minted after public/factor mathematical verification, and all raw inputs must
/// be retained by that same central frame following manifest/context admission.
/// No public constructor takes caller-selected public/factor hash strings.
/// File loading, original mathematics and revalidation are deliberately separate.
pub(super) fn derive<B:DecodeBudget>(original:&VerifiedWorkerOriginal<'_>,launch:&LaunchContext,
 run_raw:&[u8],execution_raw:&[u8],manifest_raw:&[u8],budget:&mut B)->Result<Capsule,String>{
 require([run_raw.len(),execution_raw.len(),manifest_raw.len()].iter().all(|n|*n<=RAW_CAP),"raw context cap")?;
 let total=run_raw.len().checked_add(execution_raw.len()).and_then(|n|n.checked_add(manifest_raw.len())).ok_or("context size overflow")? as u64;
 // Prepay bounded parsing, hashing, fixed typed serialization and output. The
 // caller supplies the existing replay ledger; no allowance or clock is minted.
 budget.reserve_checking(total.checked_mul(64).and_then(|x|x.checked_add(1_048_576)).ok_or("work quote overflow")?,total.checked_mul(32).and_then(|x|x.checked_add(1_048_576)).ok_or("memory quote overflow")?,Instant::now()).map_err(|_|"identity admission refused")?;
 let binding:&AttemptBinding=original.binding.ok_or("unbound original token")?;
 binding.validate()?;
 require(raw_hash(run_raw)==binding.run_sha256,"raw run identity mismatch")?;
 let run:serde_json::Value=serde_json::from_slice(run_raw).map_err(|e|e.to_string())?;
 launch.validate(&run,binding)?;
 let parsed:LaunchContext=serde_json::from_slice(execution_raw).map_err(|e|e.to_string())?;
 require(&parsed==launch,"raw/typed launch mismatch")?;
 let manifest:serde_json::Value=serde_json::from_slice(manifest_raw).map_err(|e|e.to_string())?;
 require(manifest["schema"]=="crypto.ml-dataset.cell/v1","manifest schema")?;
 require(manifest["cell_index"].as_u64()==Some(binding.cell_index),"manifest cell index")?;
 let artifacts=manifest["artifacts"].as_array().ok_or("manifest artifacts")?;
 require(artifacts.len()<=4096,"artifact count cap")?;
 let public=hex::encode(original.public_sha256);let factor=hex::encode(original.factor_base_sha256);
 let execution_hash=raw_hash(execution_raw);
 let expected=[("run.json",binding.run_sha256.as_str(),Some(run_raw.len() as u64)),
 ("plan.json",binding.plan_sha256.as_str(),None),("protocol.json",binding.protocol_sha256.as_str(),None),
 ("config.json",binding.config_sha256.as_str(),None),("attempt.json",binding.attempt_sha256.as_str(),None),
 ("public.json",public.as_str(),Some(original.public_bytes)),("factor-base.json",factor.as_str(),Some(original.factor_base_bytes)),
 ("execution-context.json",execution_hash.as_str(),Some(execution_raw.len() as u64))];
 for (path,hash,size) in expected{
  let mut matches=artifacts.iter().filter(|a|a["path"].as_str()==Some(path));
  let a=matches.next().ok_or_else(||format!("missing identity artifact {path}"))?;
  require(matches.next().is_none(),"duplicate identity artifact")?;
  require(a["sha256"].as_str()==Some(hash)&&a["bytes"].as_u64().is_some()&&size.is_none_or(|n|a["bytes"].as_u64()==Some(n)),"identity artifact mismatch")?;
 }
 // AttemptBinding::digest uses this same validated typed serde representation.
 let capture_binding=typed_hash(binding)?;
 let launch_hash=typed_hash(launch)?;
 let mut h=Sha256::new();h.update(b"ml-dataset.sat-original-owner/v1\0");h.update(capture_binding.as_bytes());h.update(original.public_sha256);h.update(original.factor_base_sha256);
 let capsule=Capsule{schema:"crypto.ml-dataset.owned-search-identity-capsule/v1",capture_binding_sha256:capture_binding,
 public_sha256:public,factor_sha256:factor,launch_sha256:launch_hash,binding_sha256:hex::encode(h.finalize()),
 manifest_sha256:raw_hash(manifest_raw),raw_execution_context_sha256:execution_hash,run_sha256:binding.run_sha256.clone(),
 original_math_replayed_here:false,cell_files_revalidated_here:false};
 budget.check(Instant::now()).map_err(|_|"identity final check refused")?;
 Ok(capsule)
}
