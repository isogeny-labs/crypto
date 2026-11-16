//! Ignored, explicitly supervised software-validation child. Not a corpus producer.
use super::*;
use crate::ml_dataset::{self as dataset, storage, execution, capture};
use super::koblitz_m3::sat_original_owner::boolean_search::publication::test_observer;
use std::io::Write;
use std::time::{Instant, Duration};

// Fixed executable and cleared environment: inherited GIT_DIR/WORK_TREE and
// PATH must not redirect source identity checks away from the built checkout.
fn trusted_git()->std::process::Command {
    let mut command=std::process::Command::new("/usr/bin/git");
    command.env_clear().env("PATH","/usr/bin:/bin").env("GIT_CONFIG_NOSYSTEM","1").env("GIT_CONFIG_GLOBAL","/dev/null");
    command
}

struct Fixture {
    root: PathBuf,
    config: CaseConfig,
    binding: capture::AttemptBinding,
}
// Deliberately no Drop cleanup: every failure and acknowledged prefix survives.
impl Fixture {
    fn create(root: PathBuf, boundary: &Value) -> Result<Self, String> {
        if !root.is_absolute() { return Err("absolute new child root required".into()); }
        fs::create_dir(&root).map_err(|e| e.to_string())?; // Never reuse an attempt.
        let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
        let output = trusted_git().args(["rev-parse", "HEAD"]).current_dir(repository).output().map_err(|e| e.to_string())?;
        if !output.status.success() { return Err("source revision lookup failed".into()); }
        let source = String::from_utf8(output.stdout).map_err(|e| e.to_string())?.trim().to_owned();
        if source.len()!=40 || !source.bytes().all(|b|b.is_ascii_hexdigit()) {return Err("source revision invalid".into());}
        // Delivery must commit the test module and hook before this child runs.
        let clean = trusted_git().args(["status", "--porcelain", "--untracked-files=all"]).current_dir(repository).output().map_err(|e|e.to_string())?;
        if !clean.status.success() || !clean.stdout.is_empty() {return Err("authentic child requires clean committed source".into());}
        let executable=std::env::current_exe().map_err(|e|e.to_string())?;
        let (binary_hash, binary_bytes)=crate::store::hash_file(&executable)?;
        let channels:dataset::types::ChannelRequests=serde_json::from_value(json!({"schema":"crypto.ml-dataset.channel-requests/v5","owned_boolean_search":{"mode":"matrix_continuation","pattern":0,"policy":"conservative_affine_envelope_checkpoint_v1"}})).map_err(|e|e.to_string())?;
        channels.validate()?;
        let mut protocol=json!({"schema":"experiment.protocol/v1","purpose":"authentic original-cell historical replay software validation","extensions":{"dataset_channels":channels},"software_control":{"boundary":boundary,"native_cells":1,"retries":0,"timing_claim":false,"source_commit":source,"binary_sha256":binary_hash,"binary_bytes":binary_bytes}});
        let canonical=dataset::protocol_canonical_bytes(&protocol)?;
        let epoch=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e|e.to_string())?.as_secs();
        protocol["freeze"]=json!({"status":"frozen","protocol_sha256":dataset::digest(&canonical),"filed_before_affected_data":true,"frozen_at":format!("unix-seconds:{epoch}")});
        let protocol_raw=serde_json::to_vec(&protocol).map_err(|e|e.to_string())?;
        let plan=dataset::DatasetPlan {schema:"crypto.ml-dataset.plan/v3".into(),channels:Some(channels),factor_base_policies:vec![dataset::types::FactorBasePolicy::Legacy{}],field_sizes:vec![23],arities:vec![2],families:vec!["binary".into()],solvers:vec!["f4".into()],curve_seeds:1,generators:1,keys:1,seed:42,factor_base_size:2,degree_bound:3,work_limit:16384,twists:true,protocol_sha256:Some(dataset::digest(&protocol_raw)),split_rule:"curve-twist-component-sha256-v1".into()};
        plan.validate()?;dataset::verify_protocol(&plan,&protocol_raw)?;
        let plan_sha=plan.binding_sha256()?;let config=plan.cell(0)?;
        let run=json!({"schema":"crypto.ml-dataset.run/v1","plan_sha256":plan_sha,"source_commit":source,"binary_sha256":binary_hash,"execution_context_required":true,"seconds_per_case":60,"memory_mib":256,"trace_mib":128});
        storage::write_json(&root,"run.json","run",&run)?;
        storage::write_json(&root,"plan.json","plan",&plan)?;
        storage::write_json(&root,"config.json","config",&config)?;
        storage::write_json(&root,"attempt.json","attempt",&json!({"schema":"crypto.ml-dataset.attempt/v1","cell_index":0,"plan_sha256":plan_sha}))?;
        storage::write_bytes(&root,"protocol.json","protocol","json",&protocol_raw)?;
        dataset::telemetry::checked::install_policy(&root)?;
        let binding=capture::AttemptBinding::from_cell_files(&root)?;
        let limits=capture::capture_limits(128)?;
        storage::write_json(&root,"capture-binding.json","partial-capture-binding",&binding)?;
        storage::write_json(&root,"capture-limits.json","partial-capture-limits",&limits)?;
        if dataset::admit_capture_context(&root)?.as_ref()!=Some(&(binding.clone(),limits)){return Err("authentic child capture setup mismatch".into());}
        Ok(Self{root,config,binding})
    }
    fn load(root:PathBuf,boundary:&Value)->Result<Self,String>{
        let plan:dataset::DatasetPlan=dataset::read_json(&root.join("plan.json"))?;
        let raw=fs::read(root.join("protocol.json")).map_err(|e|e.to_string())?;
        dataset::verify_protocol(&plan,&raw)?;
        let protocol:Value=serde_json::from_slice(&raw).map_err(|e|e.to_string())?;
        if &protocol["software_control"]["boundary"]!=boundary{return Err("child boundary differs from frozen control".into());}
        // Loading a self-consistent arbitrary plan is not authority to broaden
        // this fixed software fixture. Check every variable and original limit.
        let expected_channels:dataset::types::ChannelRequests=serde_json::from_value(json!({"schema":"crypto.ml-dataset.channel-requests/v5","owned_boolean_search":{"mode":"matrix_continuation","pattern":0,"policy":"conservative_affine_envelope_checkpoint_v1"}})).map_err(|e|e.to_string())?;
        if plan.schema!="crypto.ml-dataset.plan/v3" || plan.channels.as_ref()!=Some(&expected_channels)
            || plan.factor_base_policies!=vec![dataset::types::FactorBasePolicy::Legacy{}]
            || plan.field_sizes!=vec![23] || plan.arities!=vec![2] || plan.families!=vec!["binary".to_string()] || plan.solvers!=vec!["f4".to_string()]
            || plan.curve_seeds!=1 || plan.generators!=1 || plan.keys!=1 || plan.seed!=42 || plan.factor_base_size!=2 || plan.degree_bound!=3 || plan.work_limit!=16384
            || !plan.twists || plan.split_rule!="curve-twist-component-sha256-v1"
            || protocol["software_control"]["native_cells"]!=1 || protocol["software_control"]["retries"]!=0 || protocol["software_control"]["timing_claim"]!=false {
            return Err("child setup exceeds frozen fixture scope".into());
        }
        let config:CaseConfig=dataset::read_json(&root.join("config.json"))?;
        if config!=plan.cell(0)?{return Err("child config differs from frozen plan".into());}
        let run:Value=dataset::read_json(&root.join("run.json"))?;
        let executable=std::env::current_exe().map_err(|e|e.to_string())?;
        let (binary_hash,binary_bytes)=crate::store::hash_file(&executable)?;
        if run["binary_sha256"]!=binary_hash || protocol["software_control"]["binary_sha256"]!=binary_hash || protocol["software_control"]["binary_bytes"]!=binary_bytes{return Err("prepare/producer binary mismatch".into());}
        if run["schema"]!="crypto.ml-dataset.run/v1" || run["plan_sha256"]!=plan.binding_sha256()? || run["execution_context_required"]!=true || run["seconds_per_case"]!=60 || run["memory_mib"]!=256 || run["trace_mib"]!=128 || run["source_commit"]!=protocol["software_control"]["source_commit"] {return Err("child original limit/source joins differ".into());}
        let output=trusted_git().args(["rev-parse","HEAD"]).current_dir(env!("CARGO_MANIFEST_DIR")).output().map_err(|e|e.to_string())?;
        let head=String::from_utf8(output.stdout).map_err(|e|e.to_string())?;
        if !output.status.success() || run["source_commit"]!=head.trim(){return Err("prepare/producer source revision mismatch".into());}
        let clean=trusted_git().args(["status","--porcelain","--untracked-files=all"]).current_dir(env!("CARGO_MANIFEST_DIR")).output().map_err(|e|e.to_string())?;
        if !clean.status.success() || !clean.stdout.is_empty(){return Err("producer requires unchanged clean committed source".into());}
        let binding=capture::AttemptBinding::from_cell_files(&root)?;
        if binding.cell_index!=0{return Err("fixture is restricted to cell zero".into());}
        Ok(Self{root,config,binding})
    }
    fn produce(&self) -> Result<(),String> {
        // Consume once BEFORE original admission/generation or any public/factor
        // writes. Keep this marker on every failure; no implicit retry/reset.
        let mut consumed=fs::OpenOptions::new().write(true).create_new(true).open(self.root.join("authentic-child-consumed.json")).map_err(|e|e.to_string())?;
        consumed.write_all(b"{\"schema\":\"owned-authentic-child-consumed/v1\",\"native_cells\":1,\"retries\":0}\n").and_then(|_|consumed.sync_all()).map_err(|e|e.to_string())?;
        fs::File::open(&self.root).and_then(|d|d.sync_all()).map_err(|e|e.to_string())?;
        // Preparation created the original launch before this worker was spawned.
        let context=execution::admit(&self.root)?.ok_or("authentic child launch missing")?;
        let mut registry=Vec::new();
        let generated=dataset::telemetry::measured(&self.root,&mut registry,"curve-construction",||dataset::curves::generate(&self.config))?;
        let token=super::verify_and_store_worker_originals(&self.config,&generated,&self.root,Some(&self.binding),&mut registry)?;
        let (_,limits)=dataset::admit_capture_context(&self.root)?.ok_or("authentic child capture missing")?;
        CONTROL.with(|c| {if let Some(control)=c.borrow_mut().as_mut(){control.deadline=Some(context.deadline);}});
        let sink=capture::CaptureEmitter::create(&self.root.join("live"),self.binding.clone(),limits.clone())?;
        // The real worker calls OriginalOwner::admit, builds/publishes its map,
        // descends/normalizes and runs the actual checkpoint-enabled search.
        let retained=super::koblitz_m3::sat_original_owner::owned_search_worker::run(&token,&generated,&context,sink,&limits);
        // Returning is not the requested interrupted positive. Preserve custody
        // until the child failure; do not write a counterfeit successful report.
        std::hint::black_box(&retained);
        Err("native producer returned before supervisor termination; fixture not qualified".into())
    }
}
struct Control { file:fs::File,snapshot:u64,chunk:Option<u64>,deadline:Option<Instant>,fired:bool }
thread_local! {static CONTROL:std::cell::RefCell<Option<Control>>=const{std::cell::RefCell::new(None)};}
fn observe(boundary:test_observer::Boundary<'_>){
    let notification=CONTROL.with(|c|{
        let mut c=c.borrow_mut();let s=c.as_mut().expect("control installed before observer");
        if s.fired{return None}
        let record=match boundary{
            test_observer::Boundary::ChunkAcknowledged{pattern,snapshot,chunk,bytes,receipt}=>{
                if snapshot!=s.snapshot || Some(chunk)!=s.chunk{return None}
                json!({"boundary":"chunk_acknowledged","pattern":pattern,"snapshot":snapshot,"chunk":chunk,"bytes":bytes,"receipt":receipt})
            },
            test_observer::Boundary::SnapshotComplete{pattern,snapshot,chunks,bytes,receipt}=>{
                if snapshot!=s.snapshot || s.chunk.is_some(){return None}
                json!({"boundary":"snapshot_complete","pattern":pattern,"snapshot":snapshot,"chunks":chunks,"bytes":bytes,"receipt":receipt})
            },
        };
        s.fired=true;
        serde_json::to_writer(&mut s.file,&record).expect("control notification write");s.file.write_all(b"\n").expect("control newline");s.file.sync_all().expect("control notification sync");
        Some(s.deadline.expect("original deadline installed before producer"))
    });
    if let Some(deadline)=notification {
        // The external supervisor must terminate this parked real child. Never
        // resume producer work after the observation or renew the original clock.
        while let Some(left)=deadline.checked_duration_since(Instant::now()) {std::thread::park_timeout(left.min(Duration::from_millis(100)));}
        panic!("supervisor did not terminate at frozen boundary before original deadline");
    }
}
fn settings()->(PathBuf,PathBuf,u64,Option<u64>,Value){
    let root=PathBuf::from(std::env::var_os("OWNED_HISTORICAL_CHILD_ROOT").expect("child root required"));
    let control=PathBuf::from(std::env::var_os("OWNED_HISTORICAL_CONTROL_PATH").expect("control path required"));
    assert!(root.is_absolute()&&control.is_absolute()&&!control.starts_with(&root),"absolute control channel must be outside cell artifacts");
    let snapshot=std::env::var("OWNED_HISTORICAL_SNAPSHOT").expect("frozen snapshot required").parse::<u64>().expect("snapshot number");
    assert!(snapshot>0 && snapshot<=16,"only bounded post-initial checkpoint indices");
    let chunk=match std::env::var("OWNED_HISTORICAL_BOUNDARY").expect("boundary required").as_str(){"complete"=>None,"chunk"=>Some(std::env::var("OWNED_HISTORICAL_CHUNK").expect("frozen chunk required").parse::<u64>().expect("chunk number")),_=>panic!("unknown boundary")};
    assert!(chunk.is_none_or(|n|n<65536),"chunk cap");
    let boundary=json!({"snapshot":snapshot,"chunk":chunk,"deadline_seconds":60,"no_fallback":true});
    (root,control,snapshot,chunk,boundary)
}
#[test]
#[ignore = "requires separate bounded authentic-fixture authorization and immediate supervisor dispatch"]
fn prepare_authentic_historical_publication_child(){
    let(root,_,_,_,boundary)=settings();
    let fixture=Fixture::create(root,&boundary).expect("authentic frozen setup");
    // Same real launch constructor as the production supervisor. A later child
    // must consume this launch, never replace it to compensate for startup time.
    execution::create(&fixture.root).expect("original pre-spawn launch");
}
#[test]
#[ignore = "requires separate bounded authentic-fixture execution authorization and supervisor"]
fn authentic_historical_publication_child(){
    let(root,control,snapshot,chunk,boundary)=settings();
    let fixture=Fixture::load(root,&boundary).expect("admit frozen child setup");
    let canonical_root=fs::canonicalize(&fixture.root).expect("canonical cell root");
    let control_parent=fs::canonicalize(control.parent().expect("control parent")).expect("canonical control parent");
    assert!(!control_parent.starts_with(&canonical_root),"control channel is outside actual cell artifacts");
    let file=fs::OpenOptions::new().write(true).create_new(true).open(control_parent.join(control.file_name().expect("control filename"))).expect("exclusive control output");
    CONTROL.with(|c|*c.borrow_mut()=Some(Control{file,snapshot,chunk,deadline:None,fired:false}));
    let _observer=test_observer::install(observe).expect("exclusive observer");
    fixture.produce().expect("supervisor should terminate the child at the frozen boundary");
}
