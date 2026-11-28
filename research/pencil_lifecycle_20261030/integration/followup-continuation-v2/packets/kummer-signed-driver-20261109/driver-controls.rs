//! Prospective tests only. Run serially with the exact retained p739 fixture staged.
use super::*;
use pencil_symbolic_graph::relation_stream::{Identity, NativeEvents, NativeProtocol, Trace};
use std::cell::Cell;
use std::io::{self, Write};

struct FaultWriter<'a> { bytes: Vec<u8>, fail: &'a Cell<bool> }
impl Write for FaultWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.fail.get() { return Err(io::Error::other("fixed driver callback fault")); }
        self.bytes.extend_from_slice(bytes); Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}
fn identity(fixture: &Value, name: &str) -> Identity {
    Identity { source_revision:env!("KUMMER_DRIVER_REVISION").into(),
        source_sha256:env!("KUMMER_DRIVER_SOURCE_SHA256").into(),
        workload_id:fixture["target"]["workload_id"].as_str().unwrap().into(),run_id:name.into() }
}
fn expected_header(fixture: &Value) -> Value {
    let v=&fixture["target"];let cases=v["cases"].as_array().unwrap();
    let p=v["p"].as_u64().unwrap();
    json!({"kind":"header","schema":"kummer-driver-events/v1", "workload_id":v["workload_id"],
        "target_index":v["target_index"],"bits":16,"p":p,"r":v["r"],
        "g":cases[1]["coordinate_witness"]["jacobian_expected"],
        "q":cases[2]["coordinate_witness"]["jacobian_expected"],
        "initial_pair":[cases[2]["coordinate_witness"]["k_expected"],cases[3]["coordinate_witness"]["k_expected"]],
        "initial_labels":[0,1],"algorithm_seed":v["algorithm_seed"],"candidate_cap":100*p+10000,
        "transition_validation_enabled":false})
}
fn target_row(directory: &std::path::Path) -> Value {
    let text=fs::read_to_string(directory.join("collector-16.jsonl")).unwrap();
    assert_eq!(text.lines().count(),1);
    serde_json::from_str(text.lines().next().unwrap()).unwrap()
}
#[test]
fn actual_driver_original_disabled_observed_and_fixed_faults() {
    // Envelope and exact fixture staging are external protocol inputs; no environment mutations here.
    assert_eq!(std::env::var("KUMMER_TARGET_CAP").as_deref(),Ok("1"));
    assert_eq!(std::env::var("KUMMER_START_INDEX").as_deref(),Ok("0"));
    assert_ne!(std::env::var("KUMMER_VALIDATE_TRANSITIONS").as_deref(),Ok("1"));
    let fixture:Value=serde_json::from_str(include_str!("../fixture.json")).unwrap();
    assert_eq!(fixture["target"]["p"],739);assert_eq!(fixture["target"]["r"],34147);
    let out=std::path::PathBuf::from(std::env::var("KUMMER_DRIVER_CONTROL_OUTPUT").unwrap());
    fs::create_dir(&out).unwrap();
    crate::original::run(out.join("original").to_str().unwrap());
    run(out.join("disabled").to_str().unwrap());
    let expected=target_row(&out.join("original"));
    assert_eq!(expected["status"],"verified");
    assert_eq!(expected["source_verified"],true);
    assert_eq!(expected["recovered"],fixture["target"]["planted"]);
    assert_eq!(expected,target_row(&out.join("disabled")));
    let fail=Cell::new(false);let mut writer=FaultWriter{bytes:vec![],fail:&fail};let mut events=vec![];
    {
        let mut trace=Trace::new(&mut writer,identity(&fixture,"observed"),1_000_000,64*1024*1024).unwrap();
        {
            let mut adapter=NativeEvents::new(&mut trace,NativeProtocol::KummerDriver,expected_header(&fixture)).unwrap();
            run_observed(out.join("observed").to_str().unwrap(),&mut |event| {
                adapter.observe(event.clone()).map_err(|e|e.to_string())?;events.push(event);Ok(())
            }).unwrap();
        }
        assert!(trace.complete());
    }
    assert_eq!(expected,target_row(&out.join("observed")));
    fs::write(out.join("observed.jsonl"),&writer.bytes).unwrap();
    // Eight predeclared callback joints. Missing native branch coverage fails; no seed search.
    let joints=[("header",""),("driver_begin","advance"),("driver_end","advance"),
        ("driver_begin","lift"),("driver_end","lift"),("driver_relation",""),
        ("driver_duplicate",""),("terminal","")];
    let mut fault_rows=vec![];
    for (index,(kind,op)) in joints.into_iter().enumerate() {
        let position=events.iter().position(|v|v["kind"]==kind&&(op.is_empty()||v["operation"]==op))
            .expect("fixed native fixture did not cover required callback joint");
        let fail=Cell::new(false);let mut writer=FaultWriter{bytes:vec![],fail:&fail};let mut seen=0;
        let mut trace=Trace::new(&mut writer,identity(&fixture,&format!("fault-{index}")),1_000_000,64*1024*1024).unwrap();
        let result={
            let mut adapter=NativeEvents::new(&mut trace,NativeProtocol::KummerDriver,expected_header(&fixture)).unwrap();
            run_observed(out.join(format!("fault-{index}")).to_str().unwrap(),&mut |event| {
                if seen==position { fail.set(true); } seen+=1;
                adapter.observe(event).map_err(|e|e.to_string())
            })
        };
        assert!(result.is_err());assert!(!trace.complete());assert!(trace.failure().is_some());
        assert_eq!(seen,position+1);drop(trace);
        fs::write(out.join(format!("fault-{index}.jsonl")),&writer.bytes).unwrap();
        fault_rows.push(json!({"kind":kind,"operation":op,"position":position,"passed":true}));
    }
    fs::write(out.join("result.json"),serde_json::to_vec_pretty(&json!({
        "schema":"kummer-driver-software-qualification/v1","original_disabled_observed_equal":true,
        "complete_original_row":expected,"faults":fault_rows,"native_runs":11,
        "timing_comparison":null,"exceptional_branch_coverage":"separate native probe required"})).unwrap()).unwrap();
}

#[test]
fn actual_forced_fallback_and_zero_vector_lift_are_recorded() {
    let fixture:Value=serde_json::from_str(include_str!("../fixture.json")).unwrap();
    let v=&fixture["target"];let t=&fixture["theta"]["selected"]["constants"];
    let p=v["p"].as_u64().unwrap();let r=v["r"].as_u64().unwrap();
    let f=signed::curve_coeff(t,p);let j=Jacobian{f:Field::new(p,1,0),polynomial:f[..6].to_vec()};
    let cases=v["cases"].as_array().unwrap();
    let g:Divisor=serde_json::from_value(cases[1]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();
    let q:Divisor=serde_json::from_value(cases[2]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();
    let kg=parse_vec4(&cases[1]["coordinate_witness"]["k_expected"]);
    let k=parse_vec4(&cases[2]["coordinate_witness"]["k_expected"]);
    let kp=parse_vec4(&cases[3]["coordinate_witness"]["k_expected"]);
    let constants=["A","B","C","D"].map(|key|t[key].as_u64().unwrap());
    let mut out=vec![];let mut trace=Trace::new(&mut out,identity(&fixture,"forced-native"),32,1024*1024).unwrap();
    {
        let mut adapter=NativeEvents::new(&mut trace,NativeProtocol::KummerDriver,expected_header(&fixture)).unwrap();
        adapter.observe(expected_header(&fixture)).unwrap();
        let mut sink=|event|adapter.observe(event).map_err(|e|e.to_string());
        let mut observer:Option<&mut dyn FnMut(Value)->Result<(),String>>=Some(&mut sink);
        let mut calls=0;
        begin_driver(&mut observer,&mut calls,"advance",||json!({"labels":[0,1],"pair":[k,kp],"forced_zero_difference":true})).unwrap();
        let step=advance(k,kp,kg,[0;4],constants,&[],&j,t,p,r,g,q,0,1,true);
        end_advance(&mut observer,calls,&step).unwrap();
        assert!(step.used_fallback);assert!(!step.validation_failed);
        let (next,plus)=step.next.expect("actual fallback must reconstruct the fixture");
        let d=j.add(j.mul(g,step.choice%r),j.mul(q,2%r));
        assert!(walk::same_k(next,walk::project_any(d,&j,t,&[],p).unwrap(),p));
        assert!(walk::same_k(plus,walk::project_any(j.add(d,g),&j,t,&[],p).unwrap(),p));
        begin_driver(&mut observer,&mut calls,"lift",||json!({"labels":[0,1],"pair":[[0;4],[0;4]]})).unwrap();
        let (divisor,status,tested)=signed::lift(g,[0;4],[0;4],kg,t,&[],&j,p);
        assert!(divisor.is_none());assert_eq!(status,"zero_projective_vector");assert_eq!(tested,0);
        observe_driver(&mut observer,||json!({"kind":"driver_end","operation":"lift","call":calls,
            "divisor":divisor,"status":status,"tested":tested,"labels":[0,1]})).unwrap();
        observe_driver(&mut observer,||json!({"kind":"terminal","attempts":1,"driver_calls":calls,
            "status":"fixture_complete","recovered":null})).unwrap();
    }
    assert!(trace.complete());drop(trace);
    let output=std::path::PathBuf::from(std::env::var("KUMMER_DRIVER_CONTROL_OUTPUT").unwrap())
        .with_extension("native-probe");
    fs::create_dir(&output).unwrap();
    fs::write(output.join("events.jsonl"),&out).unwrap();
    fs::write(output.join("result.json"),serde_json::to_vec_pretty(&json!({
        "schema":"kummer-driver-native-probe/v1","p":p,"r":r,"passed":true,
        "forced_fallback_reconstructed_signed_successor":true,
        "zero_vector_lift_status":"zero_projective_vector","zero_vector_lift_candidates_tested":0,
        "trace_complete":true,"scope":"two deliberately forced native branch controls; no natural frequency estimate"})).unwrap()).unwrap();
}
