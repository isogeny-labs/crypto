// Synthetic metadata tests of the actual wrapper; no production owner/custody claim.
use super::*;
use serde_json::{json,Value};
use super::super::{host, diagnostic_launch, isolation, runner};
fn bytes(v:&impl serde::Serialize)->Vec<u8>{serde_json::to_vec(v).unwrap()}
struct Data { routes: Vec<(String,Vec<u8>,Vec<u8>)>, artifacts: Vec<(String,Vec<u8>)> }
impl Data {
 fn load()->Self {let v:Value=serde_json::from_str(include_str!("inventory-inputs.json")).unwrap(); Self{
 routes:v["routes"].as_array().unwrap().iter().map(|r|(r["name"].as_str().unwrap().into(),std::fs::read(r["path"].as_str().unwrap()).unwrap(),std::fs::read(r["spec_path"].as_str().unwrap()).unwrap())).collect(),
 artifacts:v["artifacts"].as_object().unwrap().iter().map(|(k,v)|(k.clone(),std::fs::read(v["path"].as_str().unwrap()).unwrap())).collect()}}
 fn inputs(&self)->Vec<Input<'_>>{self.routes.iter().map(|(r,p,s)|Input{route:r,plan:p,spec:s}).collect()}
 fn artifacts(&self)->Vec<(&str,&[u8])>{self.artifacts.iter().map(|(k,v)|(k.as_str(),v.as_slice())).collect()}
}
fn host_fixture()->HostCapsule{
 let mut h:HostCapsule=serde_json::from_value(json!({"schema":"ecbench.host/v1","env_class_id":"ECBENV3hfixture","env_class_sha256":"synthetic-unused-full-digest","stable":{"os":"linux","arch":"aarch64","features":[],"logical_cpus":1,"numa_nodes":1,"topology":{"cpus":[],"nodes":[]},"perf_levels":[],"cpu_settings":{},"virtualization":{}},"build":{"binary_sha256":"d".repeat(64),"debug_assertions":true,"target_os":"linux","target_arch":"aarch64","build_env":{}},"captured_unix_ms":42})).unwrap();
 h.env_class_id=host::recompute_class(&h).unwrap();h
}
// Mutation occurs before ALL derived byte hashes/identities/contexts are built.
fn check(d:&Data,i:usize,mutation:&str)->Result<Report,String>{
 let (route,planb,specb)=&d.routes[i]; let plan:PlanDoc=serde_json::from_slice(planb).unwrap(); let spec=Spec::from_json(std::str::from_utf8(specb).unwrap()).unwrap(); let (spec_id,spec_sha)=short_id("ECS1",&spec.identity_view()).unwrap();
 let mut h=host_fixture(); let mut session_binary=Some("d".repeat(64));
 match mutation {"host_schema"=>h.schema="invalid".into(),"host_binary"=>h.build.binary_sha256=Some("a".repeat(64)),"session_binary"=>session_binary=Some("a".repeat(64)),"class_recompute"=>h.stable.logical_cpus=2,"launch_binary"=>{h.build.binary_sha256=Some("a".repeat(64));session_binary=Some("a".repeat(64));},_=>()}
 let hb=bytes(&h);let hh=sha256_hex(&hb);let ph=sha256_hex(planb);
 let sid=short_id("ECBS1",&json!({"spec_sha256":spec_sha,"host_sha256":hh,"plan_sha256":ph,"started_unix_ms":"42"})).unwrap().0;
 let profile=json!({"version":diagnostic_launch::PROFILE_VERSION,"sha256":"a".repeat(64)});
 let arms:Vec<_>=plan.arms.iter().map(|a|json!({"name":a.name,"role":a.role,"method":a.method.id,"method_config_sha256":diagnostic_launch::resolved_method_binding(&a.method).unwrap(),"profile":profile})).collect();
 let mut lv=json!({"schema":diagnostic_launch::LAUNCH_SCHEMA,"expected_spec_sha256":spec_sha,"binary_sha256":"d".repeat(64),"protocol_sha256":"e".repeat(64),"source_receipt":{"scope":"external_manifest_binding_only","receipt_sha256":"f".repeat(64),"source_manifest_sha256":"1".repeat(64)},"scientific_run_id":"synthetic-preflight-unit-test","attempt":0,"profile":profile,"arms":arms});
 if mutation=="spec_binding" {lv["expected_spec_sha256"]=json!("b".repeat(64));}
 if mutation=="arm_order" {lv["arms"].as_array_mut().unwrap().swap(0,1);}
 let lb=bytes(&lv);let launch=DiagnosticLaunch::from_bytes(&lb).unwrap();
 let options=runner::RunOptions{diagnostic_launch:Some(launch.clone()),cpus:isolation::CpuRequest::None,lock_path:"synthetic-never-opened".into(),wait_for_lock:false,allow_busy:false,thresholds:isolation::Thresholds::default(),exe:"synthetic-never-executed".into()};
 let env=if mutation=="session_environment" {"changed".to_string()}else{h.env_class_id.clone()};
 let s:Session=serde_json::from_value(json!({"schema":"ecbench.session/v1","session_id":sid,"label":"synthetic fixture","spec_id":spec_id,"spec_sha256":spec_sha,"env_class_id":env,"host_sha256":hh,"plan_sha256":ph,"binary_sha256":session_binary,"options":options,"child_env_sha256":"2".repeat(64),"child_env":{},"started_unix_ms":42,"status":"running","executions_planned":plan.executions.len(),"records_written":0,"status_counts":{}})).unwrap();let sb=bytes(&s);
 let prep=json!({"schema":"ecbench.session_preparation/v1","clock_type":"linux_CLOCK_MONOTONIC","origin_ns":12,"boot_id":"fixture-boot","time_namespace":"fixture-ns","time_namespace_offsets":"monotonic 0 0\n","origin_boundary":"run_session entry before parsing; excludes CLI/process startup and input acquisition","observations":{"lock_wait_started_ns":1,"lock_acquired_ns":2,"execute_entry_ns":3},"preparation_wall_ns":3,"lock_wait_wall_ns":1,"lock_wait_is_subset_of_preparation":true,"interval_kind":"wall_only","active_cpu_ns":null,"active_worker_ns":null,"external_alignment_supported":true,"external_alignment_conditions":"same verified boot/time namespace and raw CLOCK_MONOTONIC; authenticated owner receipt required","restore_alignment_supported":false,"external_exit_observation":null,"final_publication_tail_pending":true,"session_id":sid,"runner_id":format!("{sid}-runner-17"),"clock_identity_rule":"runner_id + '-preparation-sha256-' + lowercase SHA256 of exact session-preparation.json bytes; file deliberately omits clock_id to avoid a cycle","launch_sha256":sha256_hex(&lb),"spec_sha256":spec_sha,"plan_sha256":ph,"host_sha256":hh});let pb=bytes(&prep);
 let synthetic_owner=OwnerObservation{session_sha256:sha256_hex(&sb),host_sha256:hh,preparation_sha256:sha256_hex(&pb),binary_sha256:session_binary.unwrap(),runner_pid:17,boot_id:"fixture-boot".into(),time_namespace:"fixture-ns".into(),time_namespace_offsets:"monotonic 0 0\n".into(),origin_ns:12};
 let e=&plan.executions[0];let contexts=vec![bytes(&prepare_lifecycle_child(&launch,&sid,e,&plan.arms[e.arm],&plan.workloads[e.workload],std::path::Path::new("synthetic-never-written")).unwrap().envelope)];
 check_actual_session(&d.inputs(),&d.artifacts(),ActualSession{route,plan:planb,spec:specb,host:&hb,session:&sb,preparation:&pb,launch:&lb,actual_contexts:&contexts,owner_observation:&synthetic_owner})
}
#[test]fn typed_retained_nine_positive(){let d=Data::load();assert_eq!(d.routes.len(),9);let static_r=check_inventory(&d.inputs(),&d.artifacts()).unwrap();assert!(!static_r.software_bindings_complete);assert!(!static_r.scientific_execution_admitted);for i in 0..9{let r=check(&d,i,"").unwrap_or_else(|e|panic!("{}: {e}",d.routes[i].0));assert!(r.software_bindings_complete);assert!(!r.scientific_execution_admitted);assert_eq!(r.actual_context_hashes.len(),1);assert!(r.journal_upper_bound_bytes.unwrap()>0);}}
#[test]fn typed_rebound_host_joins(){let d=Data::load();for m in ["host_schema","host_binary","session_binary","class_recompute","session_environment"] {assert_eq!(check(&d,8,m).unwrap_err(),"host schema/build/environment mismatch", "mutation={m}");}}
#[test]fn typed_rebound_launch_spec_policy(){let d=Data::load();for m in ["launch_binary","spec_binding"]{assert_eq!(check(&d,8,m).unwrap_err(),"diagnostic launch spec/binary binding mismatch or missing binary hash", "mutation={m}");}assert_eq!(check(&d,8,"arm_order").unwrap_err(),"diagnostic launch differs from exact ordered spec arms");}
