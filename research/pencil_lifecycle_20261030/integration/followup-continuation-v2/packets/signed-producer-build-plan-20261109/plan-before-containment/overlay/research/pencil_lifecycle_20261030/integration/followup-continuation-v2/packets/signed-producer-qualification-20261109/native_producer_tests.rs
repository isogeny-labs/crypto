//! Prospective root-module tests; not compiled or executed.
use super::*;
use pencil_symbolic_graph::relation_stream::{Identity, Event, NativeEvents, NativeProtocol, Trace, Record};
use serde_json::{Value,json};
use std::io::{self,Write};
mod labels {
 include!(concat!(env!("CARGO_MANIFEST_DIR"),"/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/signed-producer-qualification-20261109/labels-root.rs"));
}
fn fixture_id(run:String)->Identity {
 Identity {source_revision:env!("QUAL_SOURCE_REVISION").into(),source_sha256:env!("QUAL_SOURCE_SHA256").into(),workload_id:"fixed-native-qualification".into(),run_id:run}
}
fn expected_header(inst:&StudyInstance,target:Point,seed:u64,cap:u64)->Value {
 let s=Scholten::new(inst.curve.f.p);
 let p=s.tau(s.from_short(inst.generator));let q=s.tau(s.from_short(target));
 json!({"kind":"header","schema":"signed-collector-events/v1","convention":"su_xu_plus_sv_xv_eq_a_plus_bk","zero_node":0,
 "p":inst.curve.f.p,"r":inst.r,"order":inst.order,"cofactor":inst.h,"source_field":inst.curve.f,"source_binary":inst.curve.binary,
 "source_coefficients":inst.curve.c,"generator":inst.generator,"target":target,"transferred_generator":p,"transferred_target":q,
 "polynomial":s.j.polynomial,"seed":seed,"candidate_cap":cap,"projective":true,"rank_policy":false,"geometric_relations_independently_verified":false})
}
#[derive(Clone,Copy,Debug)] enum FailureMode { Write, Flush }
struct NativeSink { bytes:Vec<u8>, fail:Option<(&'static str,FailureMode)>, last:String, fired:bool }
impl Write for NativeSink {
 fn write(&mut self,b:&[u8])->io::Result<usize> {
  let row:Record=serde_json::from_slice(b).map_err(io::Error::other)?;
  self.last=match row.event { Event::Observed(v)|Event::ObservedTerminal(v)=>v["kind"].as_str().unwrap_or("").into(),_=>"header".into() };
  if matches!(self.fail,Some((kind,FailureMode::Write)) if kind==self.last) {self.fired=true;return Err(io::Error::other("qualification write fault"));}
  self.bytes.extend_from_slice(b);Ok(b.len())
 }
 fn flush(&mut self)->io::Result<()> {
  if matches!(self.fail,Some((kind,FailureMode::Flush)) if kind==self.last) {self.fired=true;return Err(io::Error::other("qualification flush fault"));} Ok(())
 }
}
fn capture(inst:&StudyInstance,target:Point,seed:u64,cap:u64,fail:Option<(&'static str,FailureMode)>)->(Result<Outcome,String>,Vec<Record>,bool) {
 let h=expected_header(inst,target,seed,cap);
 let mut sink=NativeSink {bytes:vec![],fail,last:String::new(),fired:false};
 let result;
 {let mut trace=Trace::new(&mut sink,fixture_id(format!("p{}-seed{}-{:?}",inst.curve.f.p,seed,fail)),250_000,64*1024*1024).unwrap();
  {let mut adapter=NativeEvents::new(&mut trace,NativeProtocol::SignedCollector,h).unwrap();
   let mut callback=|v|adapter.observe(v).map_err(|e|e.to_string());
   result=solve_genus2_observed(inst,target,seed,cap,true,false,Some(&mut callback));
  }
  if result.is_ok(){assert!(trace.complete())}else{assert!(!trace.complete());assert!(trace.failure().is_some())}
 }
 let rows=sink.bytes.split(|x|*x==b'\n').filter(|x|!x.is_empty()).map(|b|serde_json::from_slice(b).unwrap()).collect();
 (result,rows,sink.fired)
}
#[test]
fn actual_signed_panel_identity_duplicate_terminal_and_faults() {
 let panel:Vec<Value>=serde_json::from_str(include_str!("signed-panel.json")).unwrap();
 assert_eq!(panel.len(),28);let mut identity_cases=0;let mut duplicate_cases=0;let mut completed=0;
 for row in panel {
  let p=row["p"].as_u64().unwrap();let k=row["k"].as_u64().unwrap();let seed=row["algorithm_seed"].as_u64().unwrap();let cap=row["candidate_cap"].as_u64().unwrap();
  let inst=scholten_instance(p,row["instance_seed"].as_u64().unwrap()).expect("fixed constructor must exist; no silent skips");
  let target=inst.curve.mul(inst.generator,k);assert!(!target.inf,"fixed target must be nonzero; no silent skips");
  let baseline=solve_genus2_observed(&inst,target,seed,cap,true,false,None).unwrap();
  let (captured,events,fired)=capture(&inst,target,seed,cap,None);assert!(!fired);let captured=captured.unwrap();
  assert_eq!(serde_json::to_value(&baseline).unwrap(),serde_json::to_value(&captured).unwrap(),"output and arithmetic/phase/native counters changed");
  assert_eq!(captured.recovered,Some(k%inst.r));assert_eq!(inst.curve.mul(inst.generator,captured.recovered.unwrap()),target);
  assert!(matches!(events.last().map(|r|&r.event),Some(Event::ObservedTerminal(_))));
  for (index,event) in events.iter().enumerate(){assert_eq!(event.sequence,index as u64)}
  let has=|kind:&str|events.iter().any(|r|matches!(&r.event,Event::Observed(v) if v["kind"]==kind));
  let identity=has("identity");let duplicate=has("duplicate");identity_cases+=usize::from(identity);duplicate_cases+=usize::from(duplicate);
  println!("{}",json!({"case":row,"identity":identity,"duplicate":duplicate,"records":events.len(),"arithmetic_counts":captured.counts,"native":captured.native,"pass":true}));
  for (kind,present) in [("identity",identity),("duplicate",duplicate)] {
   if present { for mode in [FailureMode::Write,FailureMode::Flush] {
    let (out,records,was_fired)=capture(&inst,target,seed,cap,Some((kind,mode)));
    assert!(was_fired);assert!(out.is_err());assert!(!records.iter().any(|r|matches!(r.event,Event::ObservedTerminal(_))));
   }}
  }
  completed+=1;
 }
 assert_eq!(completed,28);assert!(identity_cases>0,"fixed panel lacks native identity coverage; do not extend panel");
 assert!(duplicate_cases>0,"fixed panel lacks native duplicate coverage; do not extend panel");
 println!("{}",json!({"completed":completed,"identity_cases":identity_cases,"duplicate_cases":duplicate_cases,"pass":true}));
}
#[test]
fn actual_kummer_labels_results_counters_and_early_returns() {
 let f:Value=serde_json::from_str(include_str!("kummer-fixture.json")).unwrap();
 let j=Jacobian{f:Field::new(739,1,0),polynomial:serde_json::from_value(f["polynomial"].clone()).unwrap()};
 let g:Divisor=serde_json::from_value(f["generator"].clone()).unwrap();let q:Divisor=serde_json::from_value(f["target"].clone()).unwrap();
 let d:Divisor=serde_json::from_value(f["divisor"].clone()).unwrap();let r=34147;let k=31124;
 assert_eq!(j.mul(g,k),q);assert_eq!(j.add(j.mul(g,2),j.mul(q,8)),d);assert!(j.valid(d));
 let aa=(2+k)%r;let bb=7;
 let cases=[("informative_same",d,aa,bb,"candidate_verified"),("informative_opposite",j.neg(d),(r-aa)%r,(r-bb)%r,"candidate_verified"),
 ("redundant_same",d,2,8,"redundant"),("redundant_opposite",j.neg(d),r-2,r-8,"redundant"),
 ("invalid_zero_target",d,3,8,"invariant_error"),("invalid_wrong_candidate",d,(aa+1)%r,bb,"candidate_rejected"),
 ("noncanonical",d,r,8,"noncanonical_label")];
 for (name,second,a,b,status) in cases {
  let mut baseline=labels::Labels::new(&j,g,q,r);let first=baseline.ingest(d,2,8);let expected=baseline.ingest(second,a,b);
  assert_eq!(expected["status"],status);
  let mut actual=labels::Labels::new(&j,g,q,r);let mut bytes=Vec::new();
  let h=json!({"kind":"header","schema":"kummer-label-events/v1","provenance":"fixed_control","p":739,"r":r,"generator":g,"target":q,"case":name});
  {let mut trace=Trace::new(&mut bytes,fixture_id(name.into()),20,1024*1024).unwrap();
   {let mut adapter=NativeEvents::new(&mut trace,NativeProtocol::KummerLabels,h.clone()).unwrap();adapter.observe(h).unwrap();
    let mut callback=|v|adapter.observe(v).map_err(|e|e.to_string());
    assert_eq!(actual.ingest_observed(1,d,2,8,&mut callback).unwrap(),first);
    let observed=actual.ingest_observed(2,second,a,b,&mut callback).unwrap();assert_eq!(observed,expected,"including scalar/field operation deltas");
    callback(json!({"kind":"terminal","attempts":2,"recovered":observed["answer"]})).unwrap();
   } assert!(trace.complete());assert_eq!(trace.records_written(),6);
  }
  println!("{}",json!({"case":name,"result":expected,"bytes":bytes.len(),"pass":true}));
 }
}
