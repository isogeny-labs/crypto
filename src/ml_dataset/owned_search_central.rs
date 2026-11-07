//! Dedicated original-owned Boolean capture admission. Never a SAT transcript seal.
use super::*;
mod sparse {
 pub use super::provenance::boolean_equations::{Charge,Monomial,Polynomial};
 pub trait Owner{fn reserve(&mut self,q:Charge)->Result<(),()>;fn check(&mut self)->Result<(),()>;}
}
#[path="dynamic_boolean_search_scheduler.rs"] mod scheduler;
#[path="dynamic_boolean_search_bqi.rs"] mod bridge;
#[path="../cryptanalysis/boolean_identity_bqi.rs"] mod bqi;
#[path="owned_search_wire_decode.rs"] pub(super) mod wire;
#[path="owned_search_semantic_check.rs"] mod semantic;
#[path="owned_search_original_descent.rs"] mod descent;
#[path="owned_search_checkpoint_selection.rs"] mod checkpoint_selection;
#[path="owned_search_interrupted_outcome.rs"] mod interrupted_outcome;
const ROLE:&str="boolean-original-owned-search-snapshot-chunk-v1";
const MAX_SNAPSHOTS:usize=4096;
const MAX_CHUNKS:usize=65536;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(in crate::ml_dataset::solvers) enum Verdict{CheckedScope,Incomplete,Unknown,Rejected}
pub(in crate::ml_dataset::solvers) struct Custody{
 frame:Frame,context:Option<OriginalContext>,verdict:Verdict,
 run_raw:Vec<u8>,plan_raw:Vec<u8>,protocol_raw:Vec<u8>,plan:Option<crate::ml_dataset::DatasetPlan>,run_value:Option<Value>,execution_raw:Vec<u8>,manifest_raw:Vec<u8>,manifest:Option<crate::ml_dataset::CellManifest>,
 capsule:Option<owned_identity::Capsule>,map_ack:Option<capture::CommittedEventPosition>,
 descent:Option<descent::Audit>,snapshots:Vec<Snapshot>,recheck_raw:Vec<u8>,projected:bool,incomplete:bool,
}
struct Chunk{artifact:Artifact,raw:Vec<u8>,checked_raw:Vec<u8>,checked:bool,ack:capture::CommittedEventPosition}
struct Snapshot{index:u64,chunks:Vec<Chunk>,joined:Vec<u8>,decoded:Option<wire::Attempt>,audit:Option<semantic::Audit>}
impl wire::Admission for ReplayBudget{
 fn reserve(&mut self,w:u64,b:u64)->Result<(),()>{self.reserve_checking(w,b,Instant::now()).map_err(|_|())}
 fn check(&mut self)->Result<(),()>{DecodeBudget::check(self,Instant::now()).map_err(|_|())}
}
pub(in crate::ml_dataset::solvers) fn verify(dir:&Path,config:&CaseConfig,outcome:&Value,artifacts:&[Artifact],policy:Policy)->Custody{
 let mut c=Custody{frame:new_frame(policy,None,None),context:None,verdict:Verdict::Rejected,run_raw:Vec::new(),plan_raw:Vec::new(),protocol_raw:Vec::new(),plan:None,run_value:None,execution_raw:Vec::new(),manifest_raw:Vec::new(),manifest:None,capsule:None,map_ack:None,descent:None,snapshots:Vec::new(),recheck_raw:Vec::new(),projected:false,incomplete:false};
 // Failure before admission retains the empty frame without diagnostic allocation.
 if c.frame.budget.reserve_checking(65536,65536,Instant::now()).is_err(){return c}
 let result=c.run(dir,config,outcome,artifacts);
 match result{Ok(v)=>c.verdict=v,Err(e)=>{
  let unknown=matches!(c.frame.math_refusal,Some(original_math::Error::UnsupportedPolicy|original_math::Error::ActiveTally|original_math::Error::UnsupportedMembership|original_math::Error::UnsupportedCertificate));
  c.frame.diagnostic=Some(e);
  let resource=matches!(c.frame.budget.first_stop,Some(Stop::Work|Stop::Bytes|Stop::Deadline|Stop::Allocation));
  if unknown{let _:Result<(),Stop>=c.frame.budget.fail(Stop::InvalidLimits);c.verdict=Verdict::Unknown}
  else if c.incomplete||resource{c.verdict=Verdict::Incomplete}
  else{let _:Result<(),Stop>=c.frame.budget.fail(Stop::NativeFailure);c.verdict=Verdict::Rejected}
 }}c
}
impl Custody{
 fn authenticated_map(&self)->Option<&provenance::CheckedMap>{self.descent.as_ref().map(|a|a.source()).or(self.frame.map.as_ref())}

 pub(in crate::ml_dataset::solvers) fn usage(&self)->(u64,u64){self.frame.usage()}
 pub(in crate::ml_dataset::solvers) fn charged(&self)->(u64,u64){self.usage()}
 pub(in crate::ml_dataset::solvers) fn retained_slot_bytes(&self)->u64{self.frame.budget.bytes}
 pub(in crate::ml_dataset::solvers) fn verdict(&self)->Verdict{self.verdict}
 fn pay(&mut self,w:u64,b:u64)->Result<(),String>{self.frame.budget.reserve_checking(w,b,Instant::now()).map_err(|_|"owned replay admission refused".into())}
 fn run(&mut self,dir:&Path,config:&CaseConfig,outcome:&Value,artifacts:&[Artifact])->Result<Verdict,String>{
  if !super::super::owned_search_worker::supported(config){return Err("unsupported owned F4 profile".into())}
  self.context=Some(admit_original_inputs(dir,config,artifacts,&mut self.frame)?);
  self.read_identity(dir,config,outcome,artifacts)?;
  self.replay_capture(dir,config)?;
  if self.snapshots.is_empty(){return Ok(Verdict::Incomplete)}
  self.decode_snapshots(config)?;
  let declared=outcome["adapter_id"]=="original-owned-boolean-search-v1"||outcome["schema"]=="crypto.ml-dataset.owned-search-report/v1";
  if !declared&&matches!(outcome["status"].as_str(),Some("timeout"|"worker_failed"|"trace_budget_exhausted"|"error"|"budget_exhausted"))&&outcome["solver_completed"]==false&&outcome["scalar_recovered_without_witness"]!=true&&outcome["ecdlp_pipeline_completed"]!=true{
   self.revalidate_inner(dir,config,outcome,artifacts)?;return Ok(Verdict::Incomplete)
  }
  self.join_report(config,outcome)?;
  // Identity and all inputs are checked again before any successful scope exists.
  self.revalidate_inner(dir,config,outcome,artifacts)?;
  let Some(a)=self.snapshots.last().and_then(|s|s.audit.as_ref())else{return Ok(Verdict::Incomplete)};
  if self.descent.as_ref().is_some_and(|d|d.original_descent_verified())&&a.result.is_ok()&&!a.rows.is_empty()&&self.frame.head.as_ref().is_some_and(|h|h.stop.is_none()){Ok(Verdict::CheckedScope)}else{Ok(Verdict::Incomplete)}
 }
 fn read_identity(&mut self,dir:&Path,config:&CaseConfig,outcome:&Value,artifacts:&[Artifact])->Result<(),String>{
  let f=&mut self.frame;
  self.run_raw=read(dir,"run.json",1024*1024,&mut f.budget,&mut f.pending_read)?;
  self.execution_raw=read(dir,"execution-context.json",1024*1024,&mut f.budget,&mut f.pending_read)?;
  self.manifest_raw=read(dir,"manifest.json",1024*1024,&mut f.budget,&mut f.pending_read)?;
  let bytes=self.manifest_raw.len() as u64;
  f.budget.reserve_checking(bytes.checked_mul(64).ok_or("manifest parse overflow")?,bytes.checked_mul(128).ok_or("manifest owner overflow")?,Instant::now()).map_err(|_|"owned manifest parse admission")?;
  self.manifest=Some(serde_json::from_slice(&self.manifest_raw).map_err(|e|e.to_string())?);
  let saved=self.manifest.as_ref().unwrap();
  if saved.config!=*config||saved.outcome!=*outcome||serde_json::to_value(&saved.artifacts).map_err(|e|e.to_string())?!=serde_json::to_value(artifacts).map_err(|e|e.to_string())?{return Err("owned manifest argument mismatch".into())}
  self.plan_raw=read(dir,"plan.json",8192,&mut f.budget,&mut f.pending_read)?;
  self.protocol_raw=read(dir,"protocol.json",65536,&mut f.budget,&mut f.pending_read)?;
  let plan_bytes=(self.plan_raw.len()+self.protocol_raw.len()+self.run_raw.len()) as u64;
  f.budget.reserve_checking(plan_bytes.checked_mul(512).and_then(|n|n.checked_add(65536)).ok_or("plan work overflow")?,plan_bytes.checked_mul(128).and_then(|n|n.checked_add(65536)).ok_or("plan owner overflow")?,Instant::now()).map_err(|_|"bounded plan/protocol admission")?;
  self.plan=Some(serde_json::from_slice(&self.plan_raw).map_err(|e|e.to_string())?);
  self.run_value=Some(serde_json::from_slice(&self.run_raw).map_err(|e|e.to_string())?);
  let plan=self.plan.as_ref().unwrap();plan.validate()?;
  crate::ml_dataset::verify_protocol(plan,&self.protocol_raw)?;
  let plan_hash=plan.binding_sha256()?;let run=self.run_value.as_ref().unwrap();
  if saved.plan_sha256!=plan_hash||plan.cell(saved.cell_index)?!=*config||run["schema"]!="crypto.ml-dataset.run/v1"||run["plan_sha256"]!=plan_hash{return Err("owned original plan/config mismatch".into())}
  crate::store::index::validate_sha256(run["binary_sha256"].as_str().ok_or("missing binary hash")?)?;
  if run["source_commit"].as_str().is_none_or(|s|s.len()!=40||!s.bytes().all(|b|b.is_ascii_hexdigit())){return Err("missing source commit".into())}
  let ctx=self.context.as_ref().unwrap();let case=f.case.as_ref().ok_or("original case absent")?;
  let token=VerifiedWorkerOriginal{config,public:&case.public_instance,factor_base:&case.factor_base,root:dir,binding:Some(&ctx.binding),public_sha256:Sha256::digest(&f.public).into(),factor_base_sha256:Sha256::digest(&f.factor).into(),public_bytes:f.public.len() as u64,factor_base_bytes:f.factor.len() as u64};
  self.capsule=Some(owned_identity::derive(&token,&ctx.launch,&self.run_raw,&self.execution_raw,&self.manifest_raw,&mut f.budget)?);
  Ok(())
 }
 fn replay_capture(&mut self,dir:&Path,config:&CaseConfig)->Result<(),String>{
  self.pay(65536,65536)?;
  let ctx=self.context.as_ref().unwrap();let binding=ctx.binding.clone();let caps=ctx.caps.clone();
  let f=&mut self.frame;
  f.head_raw=read(dir,"live/committed.json",caps.metadata_reserve_bytes,&mut f.budget,&mut f.pending_read)?;
  let head=admitted_head(&f.head_raw,&mut f.budget)?;
  if head.byte_end>f.budget.policy.stream_bytes||head.payload_bytes>f.budget.policy.payload_bytes||head.payload_count>MAX_CHUNKS as u64+1{return Err("owned stream payload cap".into())}
  let manifest=self.manifest.as_ref().ok_or("manifest absent")?;
  require_manifest_entry(&manifest.artifacts,"live/committed.json",f.head_raw.len() as u64,&hex::encode(Sha256::digest(&f.head_raw)))?;
  let mut streams=manifest.artifacts.iter().filter(|a|a.path=="live/events.jsonl");let stream=streams.next().ok_or("manifest omits live event stream")?;
  if streams.next().is_some()||stream.bytes<head.byte_end||stream.bytes>f.budget.policy.stream_bytes{return Err("manifest event stream count/cap".into())}
  f.head=Some(head.clone());
  f.budget.reserve_checking(head.byte_end.checked_add(head.payload_bytes).and_then(|n|n.checked_mul(128)).ok_or("capture work overflow")?,capture_buffer_owners(&head,dir)?.max(head.byte_end.checked_mul(128).and_then(|n|n.checked_add(2*1024*1024)).ok_or("capture owner overflow")?),Instant::now()).map_err(|_|"owned capture admission")?;
  let authenticated=capture::verify_live_committed_prefix_owned(dir,&binding,&caps,&head,&mut |request|{
   match request{
    capture::OwnedReplayRequest::Head(_)=>{
     let f=&mut self.frame;if f.checked_head.is_some(){return Err("duplicate head request".into())}
     f.checked_head_raw=read(dir,"live/committed.json",caps.metadata_reserve_bytes,&mut f.budget,&mut f.pending_read)?;
     f.checked_head=Some(admitted_head(&f.checked_head_raw,&mut f.budget)?);
     Ok(capture::OwnedReplayReply::Head(f.checked_head.as_ref().unwrap().clone()))
    },
    capture::OwnedReplayRequest::Payload(_,a)=>self.verify_payload(dir,a),
    capture::OwnedReplayRequest::Record(phase,line)=>{
     if phase==capture::RecordPhase::BeforeRecordDecode{
      let f=&mut self.frame;f.visited.admit_line(line.len(),&head).map_err(|_|"owned prefix exceeds head")?;
      f.budget.reserve_checking((line.len() as u64).checked_mul(16).and_then(|n|n.checked_add(1024)).ok_or("record quote overflow")?,(line.len() as u64).checked_mul(128).and_then(|n|n.checked_add(65536)).ok_or("record owner overflow")?,Instant::now()).map_err(|_|"owned record admission")?;
      f.pending_line.clear();f.pending_line.extend_from_slice(line);
      let shape=provenance::gate_wire::json_shape::inspect(line,||DecodeBudget::check(&mut f.budget,Instant::now()).map_err(|_|provenance::gate_wire::json_shape::Error::Admission)).map_err(|_|"record shape refused")?;
      f.budget.reserve_checking(0,shape.owner_bytes().map_err(|_|"record shape overflow")?.checked_mul(4).ok_or("record shape owner overflow")?,Instant::now()).map_err(|_|"record materialization refused")?;
      return Ok(capture::OwnedReplayReply::Done)
     }
     self.record(dir,config,line,&head)?;Ok(capture::OwnedReplayReply::Done)
    }
   }
  })?.ok_or("missing owned capture")?;
  self.frame.visited.exact(&head).map_err(|_|"owned visited endpoint mismatch")?;
  let expected=1usize.checked_add(self.snapshots.iter().map(|s|s.chunks.len()).sum::<usize>()).ok_or("payload count overflow")?;
  if self.frame.map.is_none()||authenticated.payloads.len()!=expected{return Err("owned capture map/count mismatch".into())}
  if !self.frame.payload_check_complete||self.snapshots.iter().any(|s|s.chunks.iter().any(|c|!c.checked)){return Err("owned payload verification missing".into())}
  Ok(())
 }
 fn record(&mut self,dir:&Path,config:&CaseConfig,line:&[u8],head:&capture::CommittedHead)->Result<(),String>{
  let row:capture::EventRecord=serde_json::from_slice(line).map_err(|e|e.to_string())?;
  let digest=hex::encode(Sha256::digest(line));self.frame.visited.last_hash=Some(digest.clone());
  if row.event["kind"]!="capture_payload_committed"{return Err("nonpayload event in owned capture".into())}
  let a:Artifact=serde_json::from_value(row.event["artifact"].clone()).map_err(|e|e.to_string())?;
  self.frame.visited.payload(a.bytes,head).map_err(|_|"owned payload exceeds head")?;
  self.pay((self.manifest.as_ref().ok_or("manifest absent")?.artifacts.len() as u64).checked_mul(1024).ok_or("payload manifest scan overflow")?,4096)?;
  require_manifest_entry(&self.manifest.as_ref().unwrap().artifacts,&format!("live/{}",a.path),a.bytes,&a.sha256)?;
  let ack=capture::CommittedEventPosition{binding_sha256:head.binding_sha256.clone(),event_count:row.sequence,byte_end:self.frame.visited.bytes,last_event_sha256:digest};
  if self.frame.map.is_none(){
   if self.map_ack.is_some()||a.role!="sat-original-gate-map"||a.encoding!="json"{return Err("owned original map order/schema".into())}
   let path=format!("live/{}",a.path);let f=&mut self.frame;f.map_bytes=read(dir,&path,a.bytes,&mut f.budget,&mut f.pending_read)?;
   if f.map_bytes.len() as u64!=a.bytes||hex::encode(Sha256::digest(&f.map_bytes))!=a.sha256{return Err("owned map digest".into())}
   let pattern=check_original_map(f,config,self.context.as_ref().unwrap().original_binding,&a)?;
   if Some(pattern)!=super::super::owned_search_worker::selection(config).map(|x|x.0){return Err("owned requested/map pattern mismatch".into())}
   self.map_ack=Some(ack);return Ok(())
  }
  let (pattern,index,chunk)=chunk_path(&a.path)?;
  if a.role!=ROLE||a.encoding!="owned-search-wire-v1"||a.bytes==0||Some(pattern)!=self.frame.map.as_ref().map(|m|m.pattern()){return Err("owned chunk schema/pattern".into())}
  if index==self.snapshots.len() as u64{
   if chunk!=0||self.snapshots.len()>=MAX_SNAPSHOTS{return Err("owned snapshot order/cap".into())}
   let bytes=(self.snapshots.len()+1).checked_mul(std::mem::size_of::<Snapshot>()).ok_or("snapshot capacity overflow")? as u64;
   self.pay(1,bytes)?;self.snapshots.try_reserve_exact(1).map_err(|_|"snapshot allocation")?;
   self.snapshots.push(Snapshot{index,chunks:Vec::new(),joined:Vec::new(),decoded:None,audit:None});
  }
  if !is_latest_snapshot(index,self.snapshots.len()){return Err("owned snapshot interleaving".into())}
  let previous=self.snapshots.last().and_then(|s|s.chunks.last()).map(|c|c.ack.event_count).or_else(||self.snapshots.iter().rev().nth(1).and_then(|s|s.chunks.last()).map(|c|c.ack.event_count)).or_else(||self.map_ack.as_ref().map(|a|a.event_count)).ok_or("missing preceding owned receipt")?;
  let n=self.snapshots.last().unwrap().chunks.len();
  if chunk!=n as u64||n>=MAX_CHUNKS||row.sequence!=previous.checked_add(1).ok_or("receipt overflow")?{return Err("owned chunk order/cap".into())}
  self.pay(1024,(n+1).checked_mul(std::mem::size_of::<Chunk>()).and_then(|x|x.checked_add(65536)).ok_or("chunk owner overflow")? as u64)?;
  let s=self.snapshots.last_mut().unwrap();s.chunks.try_reserve_exact(1).map_err(|_|"chunk allocation")?;
  s.chunks.push(Chunk{artifact:a,raw:Vec::new(),checked_raw:Vec::new(),checked:false,ack});
  let c=s.chunks.last_mut().unwrap();let f=&mut self.frame;
  c.raw=read(dir,&format!("live/{}",c.artifact.path),c.artifact.bytes,&mut f.budget,&mut f.pending_read)?;
  if c.raw.len() as u64!=c.artifact.bytes||hex::encode(Sha256::digest(&c.raw))!=c.artifact.sha256{return Err("owned chunk hash".into())}
  Ok(())
 }
 fn verify_payload(&mut self,dir:&Path,a:&Artifact)->Result<capture::OwnedReplayReply,String>{
  let f=&mut self.frame;
  if a.role=="sat-original-gate-map"{
   if f.payload_check_complete{return Err("duplicate map payload check".into())}
   f.payload_check_raw=read(dir,&format!("live/{}",a.path),a.bytes,&mut f.budget,&mut f.pending_read)?;
   if f.payload_check_raw!=f.map_bytes{return Err("map changed during capture".into())}
   let v=capture::verify_replay_payload_bytes(a,&f.payload_check_raw)?;f.payload_check_complete=true;return Ok(capture::OwnedReplayReply::Payload(v))
  }
  let (_,index,chunk)=chunk_path(&a.path)?;
  let c=self.snapshots.get_mut(index as usize).and_then(|s|s.chunks.get_mut(chunk as usize)).ok_or("payload check missing chunk")?;
  if c.checked{return Err("duplicate chunk payload check".into())}
  c.checked_raw=read(dir,&format!("live/{}",a.path),a.bytes,&mut f.budget,&mut f.pending_read)?;
  if c.checked_raw!=c.raw{return Err("chunk changed during capture".into())}
  let v=capture::verify_replay_payload_bytes(a,&c.checked_raw)?;c.checked=true;Ok(capture::OwnedReplayReply::Payload(v))
 }
 fn decode_snapshots(&mut self,config:&CaseConfig)->Result<(),String>{
  for i in 0..self.snapshots.len(){
   let total=self.snapshots[i].chunks.iter().try_fold(0usize,|n,c|n.checked_add(c.raw.len()).ok_or("snapshot bytes overflow"))?;
   self.pay((total as u64).checked_mul(2).ok_or("snapshot copy work overflow")?,total as u64+64)?;
   let s=&mut self.snapshots[i];s.joined.try_reserve_exact(total).map_err(|_|"snapshot concatenate allocation")?;
   for c in &s.chunks{s.joined.extend_from_slice(&c.raw)}
   let f=&mut self.frame;let remaining_work=f.budget.policy.work.checked_sub(f.budget.work).ok_or("work exhausted")?;
   let remaining_bytes=f.budget.policy.bytes.checked_sub(f.budget.bytes).ok_or("bytes exhausted")?;
   let limits=wire::Limits{raw_bytes:usize::try_from(f.budget.policy.payload_bytes).map_err(|_|"payload usize")?.min(128*1024*1024),work:remaining_work.min(32_000_000),allocation_bytes:remaining_bytes.min(256*1024*1024),..wire::Limits::default()};
   let decoded=wire::decode(std::mem::take(&mut s.joined),limits,f.budget.policy.deadline,&mut f.budget);
   s.decoded=Some(decoded);
   if let Err(e)=s.decoded.as_ref().unwrap().result{
    self.incomplete=matches!(e,wire::Failure::Limits|wire::Failure::RawCap|wire::Failure::Nodes|wire::Failure::Depth|wire::Failure::Container|wire::Failure::StringCap|wire::Failure::BytesCap|wire::Failure::Work|wire::Failure::Memory|wire::Failure::Overflow|wire::Failure::OwnerReserve|wire::Failure::OwnerCheck|wire::Failure::Deadline|wire::Failure::Allocation|wire::Failure::Truncated);
    return Err("owned snapshot wire decode refused".into())
   }
   f.budget.reserve_checking((f.map_bytes.len() as u64).checked_mul(16).and_then(|n|n.checked_add(65536)).ok_or("wire identity quote")?,65536,Instant::now()).map_err(|_|"wire identity admission")?;
   // Every snapshot must authenticate its source and original-map receipt. Only
   // the latest snapshot supplies semantic rows, so earlier positives cannot win.
   check_snapshot_binding(s,self.capsule.as_ref().ok_or("missing identity capsule")?,self.map_ack.as_ref().ok_or("missing map receipt")?,&f.map_bytes,f.map.as_ref().ok_or("missing map")?.pattern())?;
  }
  let f=&mut self.frame;let work=f.budget.policy.work.checked_sub(f.budget.work).ok_or("semantic work exhausted")?;
  let bytes=f.budget.policy.bytes.checked_sub(f.budget.bytes).ok_or("semantic bytes exhausted")?;
  let s=self.snapshots.last_mut().ok_or("snapshot absent")?;let root=s.decoded.as_ref().and_then(|d|d.value.as_ref()).ok_or("decoded snapshot absent")?;
  let Some(n)=root.get("expansion").and_then(|v|v.get("variables")).and_then(wire::Value::as_u64).and_then(|n|u32::try_from(n).ok())else{return Ok(())};
  if n!=f.map.as_ref().ok_or("map absent")?.variables(){return Err("captured variable count differs from checked map".into())}
  let map=f.map.take().ok_or("original descent map already consumed")?;
  let descent_work=work.min(4_000_000);let descent_bytes=bytes.min(8_000_000);
  let limits=descent::Limits{expansion:provenance::boolean_equations::Limits{variables:4096,generators:4096,terms:4096,degree:(config.degree_bound as usize).min(64),work:descent_work,bytes:descent_bytes},work:descent_work,bytes:descent_bytes};
  // The authentic map and all failed expansion prefixes move into retained custody.
  self.descent=Some(descent::verify(map,root,limits,f.budget.policy.deadline,&mut f.budget));
  if let Err(e)=self.descent.as_ref().unwrap().result{
   self.incomplete=matches!(e,descent::Failure::Owner|descent::Failure::Deadline|descent::Failure::Cap|descent::Failure::Overflow|descent::Failure::Allocation|descent::Failure::Missing|descent::Failure::Incomplete|descent::Failure::Expansion(provenance::boolean_equations::Failure::Unknown(_)));
   return Err("owned original equation descent did not complete".into())
  }
  let work=f.budget.policy.work.checked_sub(f.budget.work).ok_or("semantic work exhausted after descent")?;
  let bytes=f.budget.policy.bytes.checked_sub(f.budget.bytes).ok_or("semantic bytes exhausted after descent")?;
  s.audit=Some(semantic::verify(root,n,semantic::Limits::conservative(work,bytes),f.budget.policy.deadline,&mut f.budget));
  if let Err(e)=s.audit.as_ref().unwrap().result{
   self.incomplete=matches!(e,semantic::Failure::Cap|semantic::Failure::Owner|semantic::Failure::Deadline|semantic::Failure::Overflow|semantic::Failure::Allocation|semantic::Failure::Unavailable|semantic::Failure::Certificate(bridge::Failure::Cap|bridge::Failure::Owner|bridge::Failure::Deadline|bridge::Failure::Overflow|bridge::Failure::Allocation));
   if e==semantic::Failure::Certificate(bridge::Failure::Checker){
    self.incomplete=s.audit.as_ref().and_then(|a|a.rows.last()).is_some_and(|r|checker_exhausted(&r.certificate));
   }
   return Err("owned semantic replay did not complete".into())
  }
  Ok(())
 }
 fn join_report(&mut self,config:&CaseConfig,outcome:&Value)->Result<(),String>{
  self.pay(1_048_576,1_048_576)?;
  let f=&self.frame;let capsule=self.capsule.as_ref().ok_or("identity capsule absent")?;
  let snapshot=self.snapshots.last().ok_or("snapshot absent")?;let ack=&snapshot.chunks.last().ok_or("chunk absent")?.ack;
  let head=f.head.as_ref().ok_or("head absent")?;
  let pattern=super::super::owned_search_worker::selection(config).ok_or("request missing")?.0;
  validate_report(outcome,pattern,self.snapshots.len(),snapshot.index,snapshot.chunks.len(),ack,head,capsule)
 }

 fn revalidate_inner(&mut self,dir:&Path,config:&CaseConfig,outcome:&Value,artifacts:&[Artifact])->Result<(),String>{
  let artifact_bytes=self.context.as_ref().ok_or("missing context")?.artifact_bytes;
  self.pay(artifact_bytes.checked_mul(64).and_then(|v|v.checked_add(65536)).ok_or("revalidation work overflow")?,2*1024*1024)?;
  if Some(hash_artifact_descriptors(artifacts,&mut self.frame.budget)?)!=self.frame.artifact_fingerprint{return Err("owned artifact descriptors changed".into())}
  storage::verify_manifest(dir,artifacts)?;execution::verify(dir,artifacts)?;crate::ml_dataset::verify_capture_context(dir,artifacts)?;
  let ctx=self.context.as_ref().unwrap();
  if capture::AttemptBinding::from_cell_files(dir)?!=ctx.binding||execution::load(dir)?.as_ref()!=Some(&ctx.launch)||crate::ml_dataset::admit_capture_context(dir)?.as_ref()!=Some(&(ctx.binding.clone(),ctx.caps.clone())){return Err("owned original context changed".into())}
  let saved=self.manifest.as_ref().ok_or("manifest absent")?;
  if &saved.config!=config||&saved.outcome!=outcome{return Err("owned report/config changed".into())}
  // Raw originals and report/context remain retained; exact bytes defeat a
  // differently serialized replacement even if some typed fields compare equal.
  for path in ["config.json","public.json","factor-base.json","run.json","plan.json","protocol.json","execution-context.json","manifest.json","measurement-policy.json","live/committed.json"]{
   let cap=if path=="factor-base.json"{131072}else if matches!(path,"run.json"|"execution-context.json"|"manifest.json"){1024*1024}else{65536};
   self.recheck_raw=read(dir,path,cap,&mut self.frame.budget,&mut self.frame.pending_read)?;
   let expected=match path{"config.json"=>&self.frame.config,"public.json"=>&self.frame.public,"factor-base.json"=>&self.frame.factor,"run.json"=>&self.run_raw,"plan.json"=>&self.plan_raw,"protocol.json"=>&self.protocol_raw,"execution-context.json"=>&self.execution_raw,"manifest.json"=>&self.manifest_raw,"live/committed.json"=>&self.frame.head_raw,_=>&self.frame.revalidation_raw};
   if &self.recheck_raw!=expected{return Err("owned raw source/context changed".into())}
  }
  // Verify all admitted payloads afresh, retaining the latest read on refusal.
  let pattern=self.authenticated_map().ok_or("checked map absent")?.pattern();
  self.recheck_raw=read(dir,&format!("live/payloads/sat-original-pattern-{pattern}.gate.json"),self.frame.map_bytes.len() as u64,&mut self.frame.budget,&mut self.frame.pending_read)?;
  if self.recheck_raw!=self.frame.map_bytes{return Err("owned raw map changed".into())}
  for s in &self.snapshots{for c in &s.chunks{
   self.recheck_raw=read(dir,&format!("live/{}",c.artifact.path),c.artifact.bytes,&mut self.frame.budget,&mut self.frame.pending_read)?;
   if self.recheck_raw!=c.raw{return Err("owned raw chunk changed".into())}
  }}
  self.frame.budget.reserve_checking(artifact_bytes.checked_mul(64).ok_or("final source quote overflow")?,2*1024*1024,Instant::now()).map_err(|_|"final source admission")?;
  storage::verify_manifest(dir,artifacts)?;
  self.recheck_raw=read(dir,"manifest.json",1024*1024,&mut self.frame.budget,&mut self.frame.pending_read)?;
  if self.recheck_raw!=self.manifest_raw{return Err("owned manifest changed during revalidation".into())}
  // Final head read closes changes observed during the source/payload pass.
  self.frame.final_head_raw=read(dir,"live/committed.json",65536,&mut self.frame.budget,&mut self.frame.pending_read)?;
  if self.frame.final_head_raw!=self.frame.head_raw{return Err("owned capture head changed".into())}
  DecodeBudget::check(&mut self.frame.budget,Instant::now()).map_err(|_|"owned revalidation deadline".into())
 }
 pub(in crate::ml_dataset::solvers) fn revalidate(&mut self,dir:&Path,config:&CaseConfig,outcome:&Value,artifacts:&[Artifact])->Result<(),Stop>{
  if self.verdict!=Verdict::CheckedScope{return Err(self.frame.budget.first_stop.unwrap_or(Stop::Phase))}
  if let Err(e)=self.revalidate_inner(dir,config,outcome,artifacts){self.frame.diagnostic=Some(e);self.verdict=Verdict::Rejected;return self.frame.budget.fail(Stop::NativeFailure)}
  Ok(())
 }
 pub(in crate::ml_dataset::solvers) fn project_restricted(&mut self)->Result<Value,String>{
  let result=self.project_inner();
  if result.is_err(){self.verdict=Verdict::Rejected;let _:Result<(),Stop>=self.frame.budget.fail(Stop::NativeFailure);}
  result
 }
 fn project_inner(&mut self)->Result<Value,String>{
  if self.verdict!=Verdict::CheckedScope||self.projected||!self.descent.as_ref().is_some_and(|d|d.original_descent_verified()){return Err("owned projection not admitted or consumed".into())}
  self.projected=true;
  let rows=self.snapshots.last().and_then(|s|s.audit.as_ref()).ok_or("semantic audit absent")?.rows.len();
  self.pay((rows as u64).checked_mul(8192).and_then(|x|x.checked_add(65536)).ok_or("projection work overflow")?,(rows as u64).checked_mul(16384).and_then(|x|x.checked_add(262144)).ok_or("projection memory overflow")?)?;
  let s=self.snapshots.last().unwrap();let audit=s.audit.as_ref().unwrap();let ack=&s.chunks.last().unwrap().ack;
  let mut labels=Vec::new();labels.try_reserve_exact(rows).map_err(|_|"projection allocation")?;
  for r in &audit.rows{labels.push(serde_json::json!({"partition":r.partition,"node":r.node,"basis_row":r.row,"identity_verified_relative_captured_equations":true}));}
  let value=serde_json::json!({"schema":"crypto.ml-dataset.owned-search-restricted-rows/v1","visibility":"restricted","status":"verified_selected_rows","snapshot":s.index,"pattern":self.authenticated_map().ok_or("owned source absent")?.pattern(),"capture_head_event_count":ack.event_count,"capture_head_sha256":ack.last_event_sha256,"identity_capsule":self.capsule,"rows":labels,"verified_rows":rows,"original_public_factor_and_map_verified":true,"report_capture_join_verified":true,"original_descent_verified":self.descent.as_ref().is_some_and(|a|a.original_descent_verified()),"descent_scope":"authenticated-pattern-auxiliary-boolean-gate-equations","solver_completed":false,"scalar_recovered":false,"all_patterns_negative":false,"first_fall_degree":null,"last_fall_degree":null,"degree_of_regularity":null,"producer_counters_verified":false,"producer_lift_classifications_verified":false,"public_features":null});
  if DecodeBudget::check(&mut self.frame.budget,Instant::now()).is_err(){self.verdict=Verdict::Rejected;return Err("owned projection deadline".into())}Ok(value)
 }
}
fn chunk_path(path:&str)->Result<(u32,u64,u64),String>{
 let tail=path.strip_prefix("payloads/boolean-owned-pattern-").and_then(|s|s.strip_suffix(".bin.part")).ok_or("owned chunk path")?;
 let (p,tail)=tail.split_once("-snapshot-").ok_or("owned chunk snapshot path")?;let(s,c)=tail.split_once("-chunk-").ok_or("owned chunk index path")?;
 fn number(s:&str)->Result<u64,String>{let n=s.parse::<u64>().map_err(|_|"chunk number")?;if n.to_string()!=s{return Err("noncanonical chunk number".into())}Ok(n)}
 Ok((u32::try_from(number(p)?).map_err(|_|"pattern overflow")?,number(s)?,number(c)?))
}
fn check_snapshot_binding(s:&Snapshot,c:&owned_identity::Capsule,map_ack:&capture::CommittedEventPosition,map:&[u8],pattern:u32)->Result<(),String>{
 let root=s.decoded.as_ref().and_then(|d|d.value.as_ref()).ok_or("snapshot decode absent")?;
 check_identity_binding(root,s.index,c,map_ack,map,pattern)
}
/// Caller must reserve the same identity-check work/bytes as the complete path
/// and check that same absolute deadline after these comparisons before use.
/// This checks original identity only; it never grants row export admission.
fn check_prefix_binding(a:&wire::strict_prefix::Audit,index:u64,c:&owned_identity::Capsule,map_ack:&capture::CommittedEventPosition,map:&[u8],pattern:u32)->Result<(),String>{
 if a.result()!=Ok(wire::strict_prefix::State::IncompleteIdentityAvailable){return Err("wire prefix identity unavailable".into())}
 let root=a.binding_prefix().ok_or("wire prefix identity absent")?;
 check_identity_binding(root,index,c,map_ack,map,pattern)
}
/// Common complete-root/prefix binding checks under the caller's prior admission.
fn check_identity_binding(root:&wire::Value,index:u64,c:&owned_identity::Capsule,map_ack:&capture::CommittedEventPosition,map:&[u8],pattern:u32)->Result<(),String>{
 if root.get("snapshot_index").and_then(wire::Value::as_u64)!=Some(index){return Err("wire snapshot index".into())}
 let id=root.get("identity").ok_or("wire identity absent")?;
 for(k,h)in [("public_sha256",&c.public_sha256),("factor_sha256",&c.factor_sha256),("launch_sha256",&c.launch_sha256),("binding_sha256",&c.binding_sha256)]{
  let raw=id.get(k).and_then(wire::Value::as_bytes).ok_or("wire identity bytes")?;if raw.len()!=32||hex::encode(raw)!=*h{return Err("wire original identity mismatch".into())}
 }
 if id.get("pattern").and_then(wire::Value::as_u64)!=Some(pattern as u64)||id.get("map_bytes").and_then(wire::Value::as_bytes)!=Some(map)||id.get("map_published").and_then(wire::Value::as_bool)!=Some(true)||id.get("publication_attempted").and_then(wire::Value::as_bool)!=Some(true){return Err("wire original map mismatch".into())}
 let a=id.get("map_ack").ok_or("wire map receipt")?;
 for(k,v)in [("event_count",map_ack.event_count),("byte_end",map_ack.byte_end)]{if a.get(k).and_then(wire::Value::as_u64)!=Some(v){return Err("wire map receipt count".into())}}
 for(k,v)in [("binding_sha256",&map_ack.binding_sha256),("last_event_sha256",&map_ack.last_event_sha256)]{if a.get(k).and_then(wire::Value::as_str)!=Some(v.as_str()){return Err("wire map receipt hash".into())}}
 Ok(())
}

/// Fixed-size report join after original source/capture admission.
fn validate_report(outcome:&Value,pattern:u32,snapshot_count:usize,snapshot_index:u64,chunk_count:usize,ack:&capture::CommittedEventPosition,head:&capture::CommittedHead,capsule:&owned_identity::Capsule)->Result<(),String>{
  if ack.event_count!=head.event_count||ack.byte_end!=head.byte_end||ack.last_event_sha256!=head.last_event_sha256{return Err("owned report snapshot not terminal head".into())}
  for(k,v)in [("adapter_id","original-owned-boolean-search-v1"),("strategy","original-owned-boolean-search-v1"),("schema","crypto.ml-dataset.owned-search-report/v1"),("status","partial"),("scope","single-pattern-retained-search"),("accounting_kind","original-ledger-reservations-not-retired-instructions")]{if outcome[k]!=v{return Err("owned report identity/scope".into())}}
  for k in ["all_patterns_covered","solver_completed","ecdlp_pipeline_completed","scalar_recovered_without_witness","full_span_search","independent_original_source_replay","s3_custody_verified"]{if outcome[k]!=false{return Err("owned report overclaim".into())}}
  for k in ["first_fall_degree","last_fall_degree","degree_of_regularity","first_stop"]{if outcome.get(k)!=Some(&Value::Null){return Err("owned report unsupported invariant".into())}}
  for(k,v)in [("pattern",pattern as u64),("capture_snapshot_index",snapshot_index),("capture_chunk_count",chunk_count as u64),("capture_head_event_count",ack.event_count),("retained_snapshot_count",snapshot_count as u64)]{if outcome[k].as_u64()!=Some(v){return Err("owned report position".into())}}
  if outcome["capture_snapshot_complete"]!=true||outcome["capture_head_sha256"]!=ack.last_event_sha256{return Err("owned report head".into())}
  for(k,expected)in [("public_sha256_bytes",&capsule.public_sha256),("factor_base_sha256_bytes",&capsule.factor_sha256),("launch_sha256_bytes",&capsule.launch_sha256),("binding_sha256_bytes",&capsule.binding_sha256)]{
   let values=outcome[k].as_array().ok_or("report hash bytes")?;
   if values.len()!=32{return Err("report hash length".into())}
   let mut hash=[0u8;32];for(i,v)in values.iter().enumerate(){hash[i]=v.as_u64().and_then(|n|u8::try_from(n).ok()).ok_or("report hash byte")?;}
   if hex::encode(hash)!=*expected{return Err("report original hash mismatch".into())}
  }
  Ok(())
}

fn is_latest_snapshot(index:u64,count:usize)->bool{index.checked_add(1)==u64::try_from(count).ok()}
fn require_manifest_entry(artifacts:&[Artifact],path:&str,bytes:u64,sha:&str)->Result<(),String>{
 let mut matches=artifacts.iter().filter(|a|a.path==path);let a=matches.next().ok_or("manifest omits owned capture evidence")?;
 if matches.next().is_some()||a.bytes!=bytes||a.sha256!=sha{return Err("owned capture manifest descriptor mismatch".into())}Ok(())
}

fn checker_exhausted(c:&bridge::Certificate)->bool{
 matches!(c.result,Err(bridge::Failure::Checker))&&c.audit.as_ref().is_some_and(|a|matches!(a.result,Err(bqi::Failure::Unknown(_))))
}
