//! Fixed software controls. Never a cohort timing or cost observation.
use super::*;
use kernel::relation_stream::{Event, Identity, Record, Trace};
use std::io::{self, Write};
const SEED: u64 = 17823097723983109480;
const KNOWN: u64 = 31124;
const MAX_EVENTS: usize = 100_000;
const MAX_BYTES: usize = 16 * 1024 * 1024;
thread_local! { static ORDER: RefCell<Option<Vec<Value>>> = const { RefCell::new(None) }; }
pub(super) fn observe(event: Value) {
    ORDER.with(|slot| { if let Some(rows) = slot.borrow_mut().as_mut() {
        assert!(rows.len() < MAX_EVENTS, "fixed observer event cap");
        rows.push(event);
    }});
}
struct OrderGuard;
impl OrderGuard {
    fn begin() -> Self { ORDER.with(|s| { assert!(s.borrow().is_none()); *s.borrow_mut()=Some(Vec::new()); }); Self }
    fn finish(self) -> Vec<Value> { let rows=ORDER.with(|s|s.borrow_mut().take().unwrap()); assert!(serde_json::to_vec(&rows).unwrap().len()<=MAX_BYTES); rows }
}
impl Drop for OrderGuard { fn drop(&mut self) { ORDER.with(|s| { s.borrow_mut().take(); }); } }
fn fixture() -> (StudyInstance, Point) {
    let inst=StudyInstance { curve:engine::Elliptic { f:engine::Field::new(739,2,1), c:[193650,493102,0,1], binary:false }, order:546352,r:34147,h:16,generator:Point::affine(433246,59741),family:"scholten".into(),parameter:0,evidence:Value::Null };
    inst.validate().unwrap(); let target=Point::affine(0x15ed,0x5b1d4);
    let check=inst.fresh(); assert_eq!(check.curve.mul(check.generator,KNOWN),target);
    assert_ne!(check.curve.mul(check.generator,KNOWN+1),target); (inst,target)
}
fn config() -> progress_native::Config { progress_native::Config { denominator:8,matrix_coefficient_cap:25_000_000 } }
// Synthetic fixture-parent identity; actual compiled source identity belongs to the qualification receipt.
fn identity() -> Identity { Identity { source_revision:"f3c1b8cc7663ca7d15374ae5e9a16c86839c9d18".into(),source_sha256:"4210f7425d1042296f17a635a523f3bd3cd11ade343f6af46700ab813b4ceb0a".into(),workload_id:"fixed-p739-target0-software-control".into(),run_id:"managed-parity-control".into() } }
#[derive(Default)]
struct Capture(Vec<(String,String,Option<u64>,u64,Value)>);
impl progress::Sink for Capture {
    fn emit(&mut self,b:&str,e:&str,w:Option<u64>,n:u64,v:Value) { assert!(self.0.len()<progress::MAX_SNAPSHOTS as usize); self.0.push((b.into(),e.into(),w,n,v)); }
}
fn direct(progress_on:bool,trace_on:bool) -> (Value,Vec<Value>,Vec<u8>,usize) {
    let (inst,target)=fixture(); let mut noop=progress::Noop;let mut capture=Capture::default();let mut bytes=Vec::new();
    let mut trace=Trace::new(&mut bytes,identity(),100_000,MAX_BYTES).unwrap();
    let order=OrderGuard::begin();let session=measurement::Session::begin().unwrap();
    let result=progress_native::solve_one_with_progress_and_trace(&inst.fresh(),target,SEED,config(),None,if progress_on { &mut capture } else { &mut noop },if trace_on {Some(&mut trace)} else {None}).unwrap();
    assert_eq!(result.recovered,Some(KNOWN));assert!(result.source_verified);
    assert!(session.finish().unwrap().online_wall_ns.is_some());
    assert_eq!(trace.complete(),trace_on);drop(trace);
    let rows=order.finish();assert!(rows.iter().any(|r|r["kind"]=="walk"));assert!(rows.iter().any(|r|r["kind"]=="lambda"));assert!(rows.iter().any(|r|r["kind"]=="collector"));
    if trace_on { let records:Vec<Record>=bytes.split(|b|*b==b'\n').filter(|b|!b.is_empty()).map(|b|serde_json::from_slice(b).unwrap()).collect();assert!(matches!(&records.first().unwrap().event,Event::Header(_)));assert!(matches!(&records.last().unwrap().event,Event::Terminal(_)));for (i,r) in records.iter().enumerate(){assert_eq!(r.sequence,i as u64);} }
    (replay_ledger(&json!({"setup":result.setup,"detail":result.detail})),rows,bytes,capture.0.len())
}
#[test]
fn direct_managed_four_modes_preserve_order_and_counters() {
    let base=direct(false,false);let progress=direct(true,false);let trace=direct(false,true);let both=direct(true,true);
    for candidate in [&progress,&trace,&both] { assert_eq!(base.0,candidate.0);assert_eq!(base.1,candidate.1); }
    assert_eq!(trace.2,both.2);assert!(base.2.is_empty()&&progress.2.is_empty());assert_eq!(base.3,0);assert!(progress.3>0&&both.3>0);
    println!("{}",json!({"control":"managed_four_modes","modes":4,"ordered_events":base.1.len(),"known_scalar":KNOWN,"scientific_evidence":false}));
}
#[test]
fn actual_adapter_native_and_managed_routes_preserve_order_and_counters() {
    use crate::cryptanalysis::ecbench::methods::{resolve,MethodSpec};
    let method=resolve(&MethodSpec{id:"ic.three_leads_genus2_pencil_symbolic_v2".into(),params:BTreeMap::from([("small_denominator".into(),"8".into()),("matrix_coefficient_cap".into(),"25000000".into())])}).unwrap();
    let (inst,target)=fixture();let mut capture=Capture::default();
    let a=OrderGuard::begin();let session=measurement::Session::begin().unwrap();let native=solve_with_progress(&method,&inst.fresh(),target,SEED,None).unwrap();assert!(session.finish().unwrap().online_wall_ns.is_some());let native_order=a.finish();
    let b=OrderGuard::begin();let session=measurement::Session::begin().unwrap();let managed=solve_with_progress(&method,&inst.fresh(),target,SEED,Some(&mut capture)).unwrap();assert!(session.finish().unwrap().online_wall_ns.is_some());let managed_order=b.finish();
    for kind in ["walk","lambda","collector"] {assert!(native_order.iter().any(|r|r["kind"]==kind));assert!(managed_order.iter().any(|r|r["kind"]==kind));}
    let verifier=inst.fresh();assert_eq!(verifier.curve.mul(verifier.generator,native.recovered.unwrap() as u64),target);assert_eq!(verifier.curve.mul(verifier.generator,managed.recovered.unwrap() as u64),target);
    assert_eq!(native.recovered,Some(KNOWN as u128));assert_eq!(native.recovered,managed.recovered);assert_eq!(native.counters,managed.counters);assert_eq!(native_order,managed_order);
    for key in ["candidate","cold_setup"] {assert_eq!(replay_ledger(&native.detail[key]),replay_ledger(&managed.detail[key]));}
    let mut bytes=Vec::new();let mut trace=Trace::new(&mut bytes,identity(),100_000,MAX_BYTES).unwrap();
    let rejected=solve_with_progress_and_trace(&method,&inst.fresh(),target,SEED,None,Some(&mut trace));assert!(matches!(rejected,Err(ref e) if e=="relation trace requires managed progress dispatch"));assert_eq!(trace.records_written(),0);
    println!("{}",json!({"control":"actual_adapter_dispatch","routes":2,"ordered_events":native_order.len(),"trace_only_rejected":true,"scientific_evidence":false}));
}
#[test]
fn actual_progress_io_failure_is_nonfatal_and_online_closes() {
    let (inst,target)=fixture();let path=std::env::temp_dir().join(format!("managed-parity-io-{}",std::process::id()));
    let mut sink=progress::AtomicSink::create(&path,progress::Identity{session:"software-control".into(),execution:0,workload_sha256:"a".repeat(64),binary_sha256:"b".repeat(64),protocol_sha256:"c".repeat(64),seed:SEED,attempt:0,source_sha256:"d".repeat(64),method_config_sha256:"e".repeat(64)}).unwrap();
    std::fs::remove_dir(&path).unwrap(); // Actual create/write path failure, no permission assumption.
    let order=OrderGuard::begin();let session=measurement::Session::begin().unwrap();let result=progress_native::solve_one_with_progress(&inst.fresh(),target,SEED,config(),None,&mut sink).unwrap();assert!(session.finish().unwrap().online_wall_ns.is_some());let rows=order.finish();
    assert!(sink.totals.failure.is_some());assert_eq!(sink.totals.committed,0);assert_eq!(result.recovered,Some(KNOWN));assert!(result.source_verified);
    let baseline=direct(false,false);assert_eq!(rows,baseline.1);assert_eq!(replay_ledger(&json!({"setup":result.setup,"detail":result.detail})),baseline.0);
}
struct FailAfterHeader { writes:usize }
impl Write for FailAfterHeader {
    fn write(&mut self,b:&[u8])->io::Result<usize>{if self.writes>=1{return Err(io::Error::other("fixed trace I/O failure"));}self.writes+=1;Ok(b.len())}
    fn flush(&mut self)->io::Result<()>{Ok(())}
}
#[test]
fn trace_io_record_byte_limits_fail_and_online_guard_closes() {
    for case in 0..3 {
        let (inst,target)=fixture();let mut bytes=Vec::new();let mut failing=FailAfterHeader{writes:0};let writer:&mut dyn Write=if case==0{&mut failing}else{&mut bytes};
        let mut trace=Trace::new(writer,identity(),if case==1{2}else{100_000},if case==2{1}else{MAX_BYTES}).unwrap();
        let session=measurement::Session::begin().unwrap();let result=progress_native::solve_one_with_progress_and_trace(&inst.fresh(),target,SEED,config(),None,&mut progress::Noop,Some(&mut trace));
        assert!(matches!(result,Err(ref e) if e.reason=="relation_trace_failure"));assert!(trace.failure().is_some());assert!(!trace.complete());
        let snap=session.finish().unwrap();assert_eq!(snap.online_wall_ns.is_some(),case!=2);
        if case==0 {assert_eq!(trace.records_written(),1);}if case==1{assert_eq!(trace.records_written(),2);}if case==2{assert_eq!(trace.records_written(),0);}
    }
}
