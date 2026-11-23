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
        let run=json!({"schema":"crypto.ml-dataset.run/v1","plan_sha256":plan_sha,"source_commit":source,"binary_sha256":binary_hash,"execution_context_required":true,"shard_count":1,"shard_index":0,"seconds_per_case":60,"memory_mib":256,"trace_mib":128});
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
        if run["schema"]!="crypto.ml-dataset.run/v1" || run["plan_sha256"]!=plan.binding_sha256()? || run["execution_context_required"]!=true || run["shard_count"]!=1 || run["shard_index"]!=0 || run["seconds_per_case"]!=60 || run["memory_mib"]!=256 || run["trace_mib"]!=128 || run["source_commit"]!=protocol["software_control"]["source_commit"] {return Err("child original limit/source joins differ".into());}
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
        // Requires the shared production original-artifact checkpoint fix in
        // verify_and_store_worker_originals/collect_cell_artifacts. No fixture
        // writes a replacement worker registry or invents receipts after kill.
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

#[cfg(target_os="linux")]
fn spawn_test_child(executable:&Path,test:&str,root:&Path,control:&Path,snapshot:u64,chunk:Option<u64>,logdir:&Path,label:&str)->Result<dataset::SupervisedChild,String>{
    use std::os::unix::process::CommandExt;
    // Native logs are actual process streams in the mandatory cell locations;
    // preparation logs remain outside the not-yet-created cell.
    let stdout_path=if label=="native"{root.join("stdout.log")}else{logdir.join(format!("{label}.stdout"))};
    let stderr_path=if label=="native"{root.join("stderr.log")}else{logdir.join(format!("{label}.stderr"))};
    let stdout=fs::OpenOptions::new().write(true).create_new(true).open(stdout_path).map_err(|e|e.to_string())?;
    let stderr=fs::OpenOptions::new().write(true).create_new(true).open(stderr_path).map_err(|e|e.to_string())?;
    let mut command=std::process::Command::new(executable);
    command.args(["--ignored","--exact",test,"--test-threads=1","--nocapture"])
        .env_clear().env("PATH","/usr/bin:/bin").env("RAYON_NUM_THREADS","1")
        .env("OWNED_HISTORICAL_CHILD_ROOT",root).env("OWNED_HISTORICAL_CONTROL_PATH",control)
        .env("OWNED_HISTORICAL_SNAPSHOT",snapshot.to_string()).env("OWNED_HISTORICAL_BOUNDARY",if chunk.is_some(){"chunk"}else{"complete"})
        .stdout(std::process::Stdio::from(stdout)).stderr(std::process::Stdio::from(stderr));
    if let Some(chunk)=chunk{command.env("OWNED_HISTORICAL_CHUNK",chunk.to_string());}
    let parent=std::process::id();
    unsafe{command.pre_exec(move ||{
        if libc::setpgid(0,0)!=0{return Err(std::io::Error::last_os_error())}
        if libc::prctl(libc::PR_SET_PDEATHSIG,libc::SIGKILL)!=0{return Err(std::io::Error::last_os_error())}
        if libc::getppid() as u32!=parent{return Err(std::io::Error::other("fixture supervisor exited"))}
        let limit=libc::rlimit{rlim_cur:256*1024*1024,rlim_max:256*1024*1024};
        if libc::setrlimit(libc::RLIMIT_AS,&limit)!=0{return Err(std::io::Error::last_os_error())}
        Ok(())
    });}
    Ok(dataset::SupervisedChild{child:command.spawn().map_err(|e|e.to_string())?,finished:false})
}

#[cfg(target_os="linux")]
fn read_observation(path:&Path)->Result<Option<Value>,String>{
    use std::io::Read;
    use std::os::unix::fs::OpenOptionsExt;
    let file=match fs::OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW|libc::O_NONBLOCK).open(path){
        Ok(f)=>f,Err(e) if e.kind()==std::io::ErrorKind::NotFound=>return Ok(None),Err(e)=>return Err(e.to_string()),
    };
    if !file.metadata().map_err(|e|e.to_string())?.is_file(){return Err("observation is not a regular file".into())}
    let mut bytes=Vec::new();file.take(4097).read_to_end(&mut bytes).map_err(|e|e.to_string())?;
    if bytes.len()>4096{return Err("observation exceeds frozen cap".into())}
    if !bytes.ends_with(b"\n"){return Ok(None)}
    serde_json::from_slice(&bytes).map(Some).map_err(|e|e.to_string())
}
fn observation_matches_head(root:&Path,v:&Value,snapshot:u64,chunk:Option<u64>)->Result<(),String>{
    if v["pattern"]!=0 || v["snapshot"]!=snapshot || v["boundary"]!=if chunk.is_some(){"chunk_acknowledged"}else{"snapshot_complete"}{return Err("observation boundary differs from frozen request".into())}
    if let Some(chunk)=chunk{if v["chunk"]!=chunk{return Err("observation chunk mismatch".into())}}
    let ack:capture::CommittedEventPosition=serde_json::from_value(v["receipt"].clone()).map_err(|e|e.to_string())?;
    let head:capture::CommittedHead=dataset::read_json(&root.join("live/committed.json"))?;
    if ack.binding_sha256!=head.binding_sha256||ack.event_count!=head.event_count||ack.byte_end!=head.byte_end||ack.last_event_sha256!=head.last_event_sha256{return Err("observation does not match installed durable head".into())}
    Ok(())
}

fn write_control_receipt(dir:&Path,name:&str,role:&str,value:&Value)->Result<(),String>{
    struct Bounded {bytes:Vec<u8>,remaining:usize}
    impl std::io::Write for Bounded {
        fn write(&mut self,b:&[u8])->std::io::Result<usize>{
            if b.len()>self.remaining{return Err(std::io::Error::other("control receipt cap"))}
            self.bytes.try_reserve_exact(b.len()).map_err(|_|std::io::Error::other("control receipt allocation"))?;
            self.bytes.extend_from_slice(b);self.remaining-=b.len();Ok(b.len())
        }
        fn flush(&mut self)->std::io::Result<()>{Ok(())}
    }
    let used=dataset::tree_bytes_while_running(dir)?;
    let remaining=(1024u64*1024).checked_sub(used).ok_or("external log threshold exceeded; receipt refused")? as usize;
    let mut encoded=Bounded{bytes:Vec::new(),remaining};
    serde_json::to_writer(&mut encoded,value).map_err(|e|e.to_string())?;
    encoded.write_all(b"\n").map_err(|e|e.to_string())?;
    storage::write_bytes(dir,name,role,"json",&encoded.bytes)?;Ok(())
}
fn qualify_projection(v:&Value,snapshot:u64,chunk:Option<u64>,observation:&Value)->Result<(),String>{
    if v["schema"]!="crypto.ml-dataset.owned-search-historical-rows/v1" || v["status"]!="verified_rows_from_prior_checkpoint" || v["original_descent_verified"]!=true || v["verified_rows"].as_u64().is_none_or(|n|n==0){return Err("no authentic checked historical rows".into())}
    let terminal=&v["terminal_capture_receipt"];
    for key in ["binding_sha256","event_count","byte_end","last_event_sha256"]{if terminal[key]!=observation["receipt"][key]{return Err("projected terminal differs from actual observed receipt".into())}}
    if let Some(chunk)=chunk {
        if v["tail"]["disposition"]!="bound_truncated_prefix" || v["tail"]["snapshot_index"]!=snapshot || v["snapshot"].as_u64()!=snapshot.checked_sub(1){return Err("requested partial tail was not qualified".into())}
        let chunks=v["tail"]["chunks"].as_array().ok_or("tail descriptors absent")?;
        if chunks.len() as u64!=chunk+1{return Err("partial tail chunk count differs".into())}
        let expected=format!("payloads/boolean-owned-pattern-0-snapshot-{snapshot}-chunk-{chunk}.bin.part");
        let last=chunks.last().ok_or("tail empty")?;
        if last["artifact"]["path"]!=expected || last["receipt"]!=observation["receipt"]{return Err("partial tail endpoint differs".into())}
    }else if v["snapshot"]!=snapshot || v["tail"]["disposition"]!="none_observed" || v["tail"]["snapshot_index"]!=Value::Null || v["selected_checkpoint_receipt"]!=observation["receipt"]{return Err("requested complete checkpoint was not selected exactly".into())}
    Ok(())
}

#[cfg(target_os="linux")]
fn supervise_authentic_fixture()->Result<(),String>{
    let(root,control,snapshot,chunk,boundary)=settings();
    if root.exists()||control.exists(){return Err("existing fixture/control; reconcile, never retry".into())}
    // This sibling directory is also the one-shot supervisor receipt/attempt marker.
    let mut name=root.as_os_str().to_os_string();name.push(".supervisor");let logs=PathBuf::from(name);
    fs::create_dir(&logs).map_err(|e|e.to_string())?;dataset::sync_dir(logs.parent().ok_or("supervisor parent missing")?)?;
    let executable=std::env::current_exe().map_err(|e|e.to_string())?;
    let binary_hash=crate::store::hash_file(&executable)?.0;
    // The parent must independently validate the build receipt before invocation.
    // These joins preserve that externally verified reference, not prove a build.
    let build_receipt=PathBuf::from(std::env::var_os("OWNED_HISTORICAL_BUILD_RECEIPT").ok_or("externally verified exact-source build receipt required")?);
    let expected_receipt=std::env::var("OWNED_HISTORICAL_BUILD_RECEIPT_SHA256").map_err(|e|e.to_string())?;
    crate::store::index::validate_sha256(&expected_receipt)?;
    if !build_receipt.is_absolute(){return Err("absolute build receipt required".into())}
    {
        use std::io::Read;
        use std::os::unix::fs::OpenOptionsExt;
        let file=fs::OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW|libc::O_NONBLOCK).open(&build_receipt).map_err(|e|e.to_string())?;
        let meta=file.metadata().map_err(|e|e.to_string())?;
        if !meta.is_file()||meta.len()>1024*1024{return Err("bounded regular build receipt required".into())}
        let mut raw=Vec::new();file.take(1024*1024+1).read_to_end(&mut raw).map_err(|e|e.to_string())?;
        if raw.len()>1024*1024 || raw.len() as u64!=meta.len() || dataset::digest(&raw)!=expected_receipt{return Err("build receipt reference changed or exceeded cap".into())}
    }
    write_control_receipt(&logs,"invocation.json","software-fixture-invocation",&json!({"schema":"owned-authentic-supervisor/v1","binary_sha256":binary_hash,"external_build_receipt":build_receipt,"external_build_receipt_sha256":expected_receipt,"build_source_relation_independently_verified_here":false,"boundary":boundary,"native_cells":1,"retries":0,"prepare_seconds":30,"producer_seconds":60,"replay_seconds":60,"public_export_replay_seconds":60,"public_export_work":400000000,"public_export_owner_bytes":536870912,"public_copy_bytes":167772160,"public_output_acceptance_bytes":8388608,"cooperative_total_seconds":240,"proposed_external_timeout_seconds":300,"memory_bytes":268435456,"trace_bytes":134217728,"external_child_log_watchdog_bytes":1048576,"external_control_receipt_total_limit_bytes":1048576}))?;
    let started=Instant::now();let overall_deadline=started+Duration::from_secs(240);
    let mut prepare=spawn_test_child(&executable,"ml_dataset::solvers::authentic_historical_child::prepare_authentic_historical_publication_child",&root,&control,snapshot,chunk,&logs,"prepare")?;
    let prepare_deadline=Instant::now()+Duration::from_secs(30);
    let prepare_exit=loop{
        if Instant::now()>=prepare_deadline || dataset::tree_bytes_while_running(&logs)?>1024*1024 {
            let exit=prepare.stop()?;write_control_receipt(&logs,"prepare-stop.json","software-fixture-failure",&json!({"status":"preparation_stopped","exit_code":exit.code(),"retry":false}))?;
            return Err("preparation exceeded frozen bound; retained, no native dispatch".into());
        }
        if let Some(exit)=prepare.poll()?{break exit}
        std::thread::sleep(Duration::from_millis(10));
    };
    write_control_receipt(&logs,"prepare.json","software-fixture-preparation",&json!({"exit_code":prepare_exit.code(),"success":prepare_exit.success(),"retry":false}))?;
    if !prepare_exit.success(){return Err("preparation failed; artifacts retained, no native dispatch".into())}
    let fixture=Fixture::load(root.clone(),&boundary)?;
    let launch=execution::load(&root)?.ok_or("original preparation launch absent")?;
    if execution::deadline_expired(launch.deadline_monotonic_ns,execution::monotonic_ns()?){return Err("original launch expired before native dispatch".into())}
    let mut child=spawn_test_child(&executable,"ml_dataset::solvers::authentic_historical_child::authentic_historical_publication_child",&root,&control,snapshot,chunk,&logs,"native")?;
    storage::write_json(&root,"worker-process.json","process-provenance",&json!({"worker_pid":child.child.id(),"supervisor_pid":std::process::id(),"output":root,"process_group":child.child.id()}))?;
    let native_start=Instant::now();let mut stop_reason=None;let mut observation=None;let mut supervision_error=None;
    let supervised=(||->Result<std::process::ExitStatus,String>{loop{
        let polled=child.poll()?;
        if execution::deadline_expired(launch.deadline_monotonic_ns,execution::monotonic_ns()?){stop_reason=Some("timeout");}
        if dataset::tree_bytes_while_running(&root)?>128*1024*1024{stop_reason=Some("trace_budget_exhausted");}
        if dataset::tree_bytes_while_running(&logs)?>1024*1024{return Err("external supervisor log cap exceeded".into())}
        if let Some(exit)=polled{return Ok(exit)}
        if stop_reason.is_some(){return child.stop()}
        if let Some(v)=read_observation(&control)?{
            observation_matches_head(&root,&v,snapshot,chunk)?;observation=Some(v);
            // Deliberate early termination is a real worker_failed outcome. Never
            // fabricate timeout merely because the test reached its target.
            return child.stop();
        }
        if Instant::now()>=overall_deadline{return Err("overall fixture watchdog expired".into())}
        std::thread::sleep(Duration::from_millis(10));
    }})();
    let exit=match supervised{Ok(exit)=>exit,Err(e)=>{supervision_error=Some(e);child.stop()?}};
    // Termination/readback can cross the original deadline after the loop sample.
    if execution::deadline_expired(launch.deadline_monotonic_ns,execution::monotonic_ns()?){stop_reason=Some("timeout");}
    let final_bytes=dataset::tree_bytes_while_running(&root)?;
    if final_bytes>128*1024*1024{stop_reason=Some("trace_budget_exhausted");}
    let mut outcome=execution::terminal_outcome(json!({"status":"worker_failed","solver_completed":false}),stop_reason,exit.success(),exit.code());
    if let Some(disposition)=super::requested_channel_disposition(&fixture.config)?{outcome["requested_channels"]=disposition;}
    outcome["supervisor"]=json!({"exit_code":exit.code(),"elapsed_ms_diagnostic":native_start.elapsed().as_secs_f64()*1000.0,"clock_valid_for_comparison":false,"retired_instructions":null,"instruction_counter_status":"whole_worker_not_counted; see phase telemetry for available hardware counts","memory_limit_bytes":268435456u64,"worker_output_bytes_before_seal":final_bytes,"output_budget_bytes":134217728u64,"output_budget_scope":"all_cell_files_before_manifest_seal","checkpoint_mid_call":false,"interrupted_trace_loss_possible":true});
    let artifacts=dataset::collect_cell_artifacts(&root)?;
    let plan:dataset::DatasetPlan=dataset::read_json(&root.join("plan.json"))?;
    let manifest=dataset::CellManifest{schema:"crypto.ml-dataset.cell/v1".into(),cell_index:0,plan_sha256:plan.binding_sha256()?,config:fixture.config,outcome,artifacts};
    storage::write_json(&root,"manifest.json","cell-manifest",&manifest)?;
    write_control_receipt(&logs,"supervision.json","software-fixture-supervision",&json!({"exit_code":exit.code(),"stop_reason":stop_reason,"matched_observation":observation,"supervision_error":supervision_error,"elapsed_seconds":started.elapsed().as_secs_f64(),"native_cells":1,"retries":0}))?;
    if supervision_error.is_some()||observation.is_none(){return Err("frozen boundary not reached cleanly; sealed artifacts retained".into())}
    if Instant::now()>=overall_deadline{return Err("no remaining overall replay allowance".into())}
    let replay_deadline=(Instant::now()+Duration::from_secs(60)).min(overall_deadline);
    let policy=super::SatReplayPolicy{work:200_000_000,bytes:512*1024*1024,deadline:replay_deadline,stream_bytes:128*1024*1024,payload_bytes:128*1024*1024};
    let mut owner=super::verify_owned_search_historical_cell(&root,&manifest.config,&manifest.outcome,&manifest.artifacts,policy);
    let before=owner.charged();let admitted=owner.verdict();
    let revalidated=if admitted==super::OwnedReplayVerdict::HistoricalCheckedScope{Some(owner.revalidate(&root,&manifest.config,&manifest.outcome,&manifest.artifacts))}else{None};
    let projected=if revalidated==Some(super::OwnedReplayVerdict::HistoricalCheckedScope){owner.project_identity()}else{Err("authentic historical scope not admitted/revalidated".into())};
    let qualification=projected.as_ref().map_err(|e|e.clone()).and_then(|v|qualify_projection(v,snapshot,chunk,observation.as_ref().ok_or("observation absent")?));
    write_control_receipt(&logs,"replay.json","software-fixture-replay",&json!({"initial_verdict":format!("{admitted:?}"),"revalidated_verdict":format!("{revalidated:?}"),"charged_before":before,"charged_after":owner.charged(),"projection":projected.as_ref().ok(),"error":projected.as_ref().err(),"same_owner":true,"fresh_policy_count":1,"requested_boundary_qualified":qualification.is_ok(),"qualification_error":qualification.as_ref().err(),"cli_export_verified":false}))?;
    projected?;
    qualification?;
    // Independent, preregistered public acceptance phase only AFTER successful
    // direct scope/qualification. The consumed direct owner is never reused.
    drop(owner);
    public_historical_export(&root,&logs,&manifest,snapshot,chunk,observation.as_ref().ok_or("observation absent")?,overall_deadline)?;
    Ok(())
}
#[cfg(target_os="linux")]
#[test]
#[ignore="requires exact-source build receipt and separately authorized bounded native software validation"]
fn authentic_historical_supervised_acceptance(){supervise_authentic_fixture().expect("authentic historical acceptance; inspect preserved supervisor receipts on failure");}

#[cfg(target_os="linux")]
fn read_bounded_regular(path:&Path,cap:u64)->Result<Vec<u8>,String>{
    use std::io::Read;use std::os::unix::fs::OpenOptionsExt;
    let file=fs::OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW|libc::O_NONBLOCK).open(path).map_err(|e|e.to_string())?;
    let meta=file.metadata().map_err(|e|e.to_string())?;
    if !meta.is_file()||meta.len()>cap{return Err("bounded regular acceptance input required".into())}
    let mut raw=Vec::new();file.take(cap+1).read_to_end(&mut raw).map_err(|e|e.to_string())?;
    if raw.len()as u64>cap||raw.len()as u64!=meta.len(){return Err("acceptance input changed or grew".into())}Ok(raw)
}
#[cfg(target_os="linux")]
fn public_historical_export(root:&Path,logs:&Path,manifest:&dataset::CellManifest,snapshot:u64,chunk:Option<u64>,observation:&Value,overall:Instant)->Result<(),String>{
    use std::io::{Read,Write};use std::os::unix::fs::OpenOptionsExt;
    if Instant::now()+Duration::from_secs(60)>overall{return Err("insufficient remaining frozen public acceptance allowance".into())}
    let mut run_name=root.as_os_str().to_os_string();run_name.push(".public-run");let public_run=PathBuf::from(run_name);
    let mut out_name=root.as_os_str().to_os_string();out_name.push(".public-export");let destination=PathBuf::from(out_name);
    fs::create_dir(&public_run).map_err(|e|e.to_string())?;
    fs::create_dir(public_run.join("cells")).map_err(|e|e.to_string())?;
    let copied_cell=public_run.join("cells/000000000000");fs::create_dir(&copied_cell).map_err(|e|e.to_string())?;
    // Fresh bounded byte-for-byte custody copy. No source file, manifest outcome,
    // plan or attempt binding is edited to suit export's run-directory layout.
    let mut paths=manifest.artifacts.iter().map(|a|a.path.clone()).collect::<Vec<_>>();paths.push("manifest.json".into());paths.sort();
    if paths.len()>4097||paths.windows(2).any(|p|p[0]==p[1]){return Err("public fixture copy descriptor cap/duplicate".into())}
    let mut copied_bytes=0u64;let mut copies=Vec::new();
    for relative in paths{
        if Instant::now()>=overall{return Err("public custody copy deadline".into())}
        if Path::new(&relative).components().any(|c|!matches!(c,std::path::Component::Normal(_))){return Err("nonrelative public custody path".into())}
        let source=root.join(&relative);let dest=copied_cell.join(&relative);
        let input=fs::OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW|libc::O_NONBLOCK).open(&source).map_err(|e|e.to_string())?;
        let meta=input.metadata().map_err(|e|e.to_string())?;
        if !meta.is_file(){return Err("public copy source not regular".into())}
        copied_bytes=copied_bytes.checked_add(meta.len()).filter(|n|*n<=160*1024*1024).ok_or("public copy byte cap")?;
        let before=crate::store::hash_file(&source)?;
        if before.1!=meta.len(){return Err("public copy source changed before copy".into())}
        fs::create_dir_all(dest.parent().ok_or("copy parent absent")?).map_err(|e|e.to_string())?;
        let mut output=fs::OpenOptions::new().write(true).create_new(true).open(&dest).map_err(|e|e.to_string())?;
        let mut input=input.take(meta.len()+1);let mut buffer=[0u8;8192];let mut count=0u64;
        loop{if Instant::now()>=overall{return Err("public copy original acceptance deadline".into())}
            let n=input.read(&mut buffer).map_err(|e|e.to_string())?;if n==0{break}
            count+=n as u64;if count>meta.len(){return Err("public copy source grew".into())}output.write_all(&buffer[..n]).map_err(|e|e.to_string())?;
        }
        output.sync_all().map_err(|e|e.to_string())?;
        if count!=meta.len()||crate::store::hash_file(&dest)?!=before||crate::store::hash_file(&source)?!=before{return Err("public copy source/destination digest mismatch".into())}
        copies.push(json!({"path":relative,"bytes":before.1,"sha256":before.0}));
    }
    for name in ["run.json","plan.json","protocol.json"]{
        let raw=read_bounded_regular(&copied_cell.join(name),65536)?;
        copied_bytes=copied_bytes.checked_add(raw.len() as u64).filter(|n|*n<=160*1024*1024).ok_or("public run copy byte cap")?;
        storage::write_bytes(&public_run,name,"byte-identical-source-run","json",&raw)?;
    }
    let run_sha=crate::store::hash_file(&public_run.join("run.json"))?.0;
    let policy_path=logs.join("public-replay-policy.json");let receipt_path=logs.join("public-replay-receipt.json");
    let policy=json!({"schema":"crypto.ml-dataset.owned-replay-policy/v1","mode":"original_owned_search_historical_rows_v1","run_sha256":run_sha,"receipt_path":receipt_path,"cell_indices":[0],"seconds":60,"work_per_cell":400000000u64,"bytes_per_cell":536870912u64,"stream_bytes_per_cell":134217728u64,"payload_bytes_per_cell":134217728u64,"aggregate_work":401000000u64,"aggregate_bytes":629145600u64});
    write_control_receipt(logs,"public-copy.json","software-fixture-custody",&json!({"copied_bytes":copied_bytes,"copies":copies,"source_cell_unchanged":true,"new_acceptance_policy_count":1,"retry_of_direct_owner":false}))?;
    write_control_receipt(logs,"public-replay-policy.json","software-fixture-public-policy",&policy)?;
    if Instant::now()+Duration::from_secs(60)>overall{return Err("insufficient remaining public replay deadline after copy".into())}
    let args=dataset::DatasetArgs{owned_replay_policy:Some(policy_path),owned_replay_receipt:Some(receipt_path.clone()),search_replay_policy:None,search_replay_receipt:None,sat_replay_policy:None,sat_replay_receipt:None,affine_replay_policy:None,affine_replay_receipt:None,command:dataset::DatasetCommand::Export{out:public_run.clone(),destination:destination.clone()}};
    let result=dataset::run(&args);
    // Real invocation writes its own exclusive start/terminal receipts. Never
    // synthesize a passing receipt if the public caller or exporter refused.
    let receipt_raw=read_bounded_regular(&receipt_path,65536);
    write_control_receipt(logs,"public-dispatch.json","software-fixture-public-result",&json!({"returned_ok":result.is_ok(),"error":result.as_ref().err(),"receipt_read_error":receipt_raw.as_ref().err(),"typed_public_dispatch":true,"cli_argv_parser_verified":false,"separate_acceptance_policy":true}))?;
    result?;let receipt:Value=serde_json::from_slice(&receipt_raw?).map_err(|e|e.to_string())?;
    if receipt["schema"]!="crypto.ml-dataset.owned-replay-invocation/v1"||receipt["run_sha256"]!=run_sha||receipt["slots"][0]["cell_index"]!=0||receipt["slots"][0]["consumed"]!=true||receipt["verification_returned_ok"]!=true||receipt["mode"]!="original_owned_search_historical_rows_v1"||receipt["slots"].as_array().is_none_or(|v|v.len()!=1)||receipt["slots"][0]["failed"]!=false||receipt["slots"][0]["verdict"]!="checked_historical_owned_row_scope"||receipt["slots"][0]["restricted_row_projections"]!=1{return Err("public caller receipt did not verify one historical projection".into())}
    if dataset::tree_bytes_while_running(&destination)?>8*1024*1024{return Err("public output exceeds fixed acceptance byte limit; preserved".into())}
    let summary:Value=serde_json::from_slice(&read_bounded_regular(&destination.join("manifest.json"),1024*1024)?).map_err(|e|e.to_string())?;
    if summary["owned_search_historical_rows"]["restricted_records"]!=1||summary.get("owned_search_rows").is_some(){return Err("historical public aggregate not distinct".into())}
    let read_one=|name:&str|->Result<Value,String>{let bytes=read_bounded_regular(&destination.join(name),4*1024*1024)?;let text=std::str::from_utf8(&bytes).map_err(|e|e.to_string())?;let mut lines=text.lines();let value=serde_json::from_str(lines.next().ok_or("export record absent")?).map_err(|e|e.to_string())?;if lines.next().is_some(){return Err("expected exactly one public restricted record".into())}Ok(value)};
    let prefix=read_one("restricted/prefix-records.jsonl")?;let target=read_one("restricted/targets.jsonl")?;let index=read_one("restricted/index.jsonl")?;
    qualify_projection(&prefix["record"],snapshot,chunk,observation)?;
    for key in ["producer_attempt_complete","report_capture_join_verified","solver_completed","scalar_recovered","all_patterns_negative"]{if prefix["record"].get(key)!=Some(&Value::Bool(false)){return Err("historical public record overclaims completion".into())}}
    for key in ["first_fall_degree","last_fall_degree","degree_of_regularity"]{if prefix["record"].get(key)!=Some(&Value::Null){return Err("historical public record promotes unknown degree".into())}}
    let manifest_sha=crate::store::hash_file(&root.join("manifest.json"))?.0;
    let base=dataset::digest(format!("{run_sha}\n{manifest_sha}\n0").as_bytes());let expected_target=dataset::digest(format!("{base}\nowned-search-historical-target").as_bytes());let expected_record=dataset::digest(format!("{expected_target}\nowned-search-historical-row-record").as_bytes());
    if prefix["record_id"]!=expected_record||prefix["source_target_id"]!=expected_target||target["target_id"]!=expected_target||index["target_id"]!=expected_target||index["record_id"]!=expected_target||index["source_run_sha256"]!=run_sha||index["source_manifest_sha256"]!=manifest_sha||target["labels"]!=prefix["record"]||target["feature_status"]!="restricted_historical_owned_row_identities_only"||index["status"]!="restricted_historical_owned_search_scope"{return Err("public historical identity/label join mismatch".into())}
    for key in ["feature_id","known_answer_scalar","independently_recovered_scalar","witness_artifact"]{if target.get(key)!=Some(&Value::Null){return Err("restricted historical target promoted unavailable label".into())}}
    if Instant::now()>=overall{return Err("public acceptance overall deadline exceeded".into())}
    write_control_receipt(logs,"public-acceptance.json","software-fixture-public-acceptance",&json!({"passed":true,"source_run_sha256":run_sha,"source_manifest_sha256":manifest_sha,"historical_target_id":expected_target,"historical_record_id":expected_record,"typed_public_dispatch_verified":true,"cli_argv_parser_verified":false,"direct_and_public_acceptance_policies":2,"native_attempts":1,"retries":0}))?;
    Ok(())
}
