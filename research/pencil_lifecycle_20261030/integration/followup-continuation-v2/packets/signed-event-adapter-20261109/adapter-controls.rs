#[cfg(test)]
mod native_adapter_controls {
 use super::*;
 use serde_json::json;
 fn identity()->Identity { Identity { source_revision:"a".repeat(40),source_sha256:"b".repeat(64),workload_id:"target".into(),run_id:"attempt".into() } }
 fn header()->serde_json::Value { json!({"kind":"header","schema":"signed-collector-events/v1","r":7,"target":"bound-target"}) }
 fn candidate(a:u64)->serde_json::Value { json!({"kind":"candidate","attempt":a}) }
 #[test]
 fn identity_before_terminal_is_retained() {
  let mut out=Vec::new();
  { let mut t=Trace::new(&mut out,identity(),20,10000).unwrap();
    { let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
      a.observe(header()).unwrap();a.observe(candidate(1)).unwrap();
      a.observe(json!({"kind":"identity","attempt":1,"recovered":3})).unwrap();
      a.observe(json!({"kind":"terminal","attempts":1,"recovered":3})).unwrap(); }
    assert!(t.complete());assert_eq!(t.records_written(),4); }
  let rows:Vec<Record>=out.split(|b|*b==b'\n').filter(|b|!b.is_empty()).map(|b|serde_json::from_slice(b).unwrap()).collect();
  assert!(matches!(&rows[2].event,Event::Observed(v) if v["kind"]=="identity"));
 }
 #[test]
 fn duplicate_disposition_and_first_reference_are_retained() {
  let mut out=Vec::new();let mut t=Trace::new(&mut out,identity(),20,10000).unwrap();
  { let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
    a.observe(header()).unwrap();a.observe(candidate(1)).unwrap();
    a.observe(json!({"kind":"edge","attempt":1,"node_count":1,"nodes":[0,0],"signs":[1,-1],"canonical":[1,2],"recovered":null})).unwrap();
    a.observe(candidate(2)).unwrap();
    a.observe(json!({"kind":"duplicate","attempt":2,"canonical":[1,2],"first_attempt":1})).unwrap();
    a.observe(json!({"kind":"terminal","attempts":2,"recovered":null})).unwrap(); }
  assert!(t.complete());assert_eq!(t.records_written(),6);
 }
 #[test]
 fn missing_disposition_poisoned() {
  let mut out=Vec::new();let mut t=Trace::new(&mut out,identity(),20,10000).unwrap();
  { let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
    a.observe(header()).unwrap();a.observe(candidate(1)).unwrap();
    assert_eq!(a.observe(json!({"kind":"terminal","attempts":1,"recovered":null})),Err(Error::State)); }
  assert!(!t.complete());assert_eq!(t.failure(),Some(&Error::State));
 }
 #[test]
 fn post_identity_candidate_rejected() {
  let mut out=Vec::new();let mut t=Trace::new(&mut out,identity(),20,10000).unwrap();
  let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
  a.observe(header()).unwrap();a.observe(candidate(1)).unwrap();
  a.observe(json!({"kind":"identity","attempt":1,"recovered":3})).unwrap();
  assert_eq!(a.observe(candidate(2)),Err(Error::State));
 }
 #[test]
 fn changed_target_header_rejected() {
  let mut out=Vec::new();let mut t=Trace::new(&mut out,identity(),20,10000).unwrap();
  let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
  let mut wrong=header();wrong["target"]=json!("other");assert_eq!(a.observe(wrong),Err(Error::State));
 }
 #[test]
 fn early_kummer_rejection_has_result_and_terminal() {
  let h=json!({"kind":"header","schema":"kummer-label-events/v1","provenance":"fixed_control","r":7});
  let mut out=Vec::new();let mut t=Trace::new(&mut out,identity(),20,10000).unwrap();
  { let mut a=NativeEvents::new(&mut t,NativeProtocol::KummerLabels,h.clone()).unwrap();
    a.observe(h).unwrap();a.observe(json!({"kind":"label_candidate","attempt":1,"a":7,"b":0})).unwrap();
    a.observe(json!({"kind":"label_result","attempt":1,"result":{"status":"noncanonical_label","emit_graph_edge":false,"answer":null}})).unwrap();
    a.observe(json!({"kind":"terminal","attempts":1,"recovered":null})).unwrap(); }
  assert!(t.complete());assert_eq!(t.records_written(),4);
 }
 struct FailsSecondWrite { calls:usize }
 impl std::io::Write for FailsSecondWrite {
  fn write(&mut self, bytes:&[u8])->std::io::Result<usize> {
   self.calls+=1;
   if self.calls==2 { Err(std::io::Error::other("injected native callback output failure")) }
   else { Ok(bytes.len()) }
  }
  fn flush(&mut self)->std::io::Result<()> { Ok(()) }
 }
 #[test]
 fn callback_preserves_first_output_error_after_later_event() {
  let mut sink=FailsSecondWrite {calls:0};
  let mut t=Trace::new(&mut sink,identity(),20,10000).unwrap();
  let first;
  { let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
    // Exercise the same fallible closure shape passed to the native producer.
    a.observe(header()).unwrap();
    let mut callback=|event| a.observe(event);
    first=callback(candidate(1)).unwrap_err();
    assert!(matches!(&first,Error::Output(message) if message.contains("injected native callback")));
    assert_eq!(callback(json!({"kind":"identity","attempt":1,"recovered":3})),Err(first.clone()));
  }
  assert_eq!(t.failure(),Some(&first));assert!(!t.complete());assert_eq!(t.records_written(),1);
 }
 #[test]
 fn callback_preserves_first_record_limit_after_later_event() {
  let mut sink=Vec::new();let mut t=Trace::new(&mut sink,identity(),2,10000).unwrap();
  { let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
    a.observe(header()).unwrap();a.observe(candidate(1)).unwrap();
    let error=a.observe(json!({"kind":"identity","attempt":1,"recovered":3})).unwrap_err();
    assert_eq!(error,Error::RecordLimit);
    assert_eq!(a.observe(json!({"kind":"terminal","attempts":1,"recovered":3})),Err(error));
  }
  assert_eq!(t.failure(),Some(&Error::RecordLimit));assert!(!t.complete());
 }

}
