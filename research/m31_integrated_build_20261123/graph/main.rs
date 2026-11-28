use m31_graph_compare::{admission::{admit, parse_rows, Manifest}, compare::{run, Arm, Limits, Record}, sha256, MAX_INPUT_BYTES};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs::{self, File}, io::{Read, Write}, path::PathBuf, process::{Command, Stdio}, time::{Duration, Instant}};

#[derive(Serialize, Deserialize)]
struct WorkerRecord {
    schema: String,
    pid: u32,
    executable_sha256: String,
    manifest_sha256: String,
    input_loading_ns: u128,
    group_admission_ns: u128,
    component_call_ns: u128,
    process_peak_rss_kib: Option<u64>,
    process_user_cpu_us: Option<u64>,
    process_system_cpu_us: Option<u64>,
    group_admission_counts: serde_json::Value,
    record: Record,
}
fn read_bounded(path:&str)->Result<Vec<u8>,String>{
    let file=File::open(path).map_err(|e|format!("{path}: {e}"))?;
    if file.metadata().map_err(|e|e.to_string())?.len()>MAX_INPUT_BYTES{return Err("file byte cap".into());}
    let mut bytes=Vec::new();file.take(MAX_INPUT_BYTES+1).read_to_end(&mut bytes).map_err(|e|e.to_string())?;
    if bytes.len() as u64>MAX_INPUT_BYTES{return Err("file grew beyond byte cap".into());}Ok(bytes)
}
fn args_map(args:&[String],allowed:&[&str])->Result<BTreeMap<String,String>,String>{
    if args.len()%2!=0{return Err("each option requires one value".into());}
    let mut out=BTreeMap::new();
    for pair in args.chunks(2){if !allowed.contains(&pair[0].as_str())||out.insert(pair[0].clone(),pair[1].clone()).is_some(){return Err(format!("unknown/duplicate option {}",pair[0]));}}Ok(out)
}
fn required<'a>(args:&'a BTreeMap<String,String>,key:&str)->Result<&'a str,String>{args.get(key).map(String::as_str).ok_or_else(||format!("missing {key}"))}
fn process_usage()->(Option<u64>,Option<u64>,Option<u64>){
    #[cfg(target_os="linux")]
    {
        let mut value=std::mem::MaybeUninit::<libc::rusage>::zeroed();
        // SAFETY: the pointer is valid writable storage of the expected C type.
        if unsafe{libc::getrusage(libc::RUSAGE_SELF,value.as_mut_ptr())}==0{
            let v=unsafe{value.assume_init()};
            let micros=|t:libc::timeval|Some(t.tv_sec as u64*1_000_000+t.tv_usec as u64);
            return (Some(v.ru_maxrss as u64),micros(v.ru_utime),micros(v.ru_stime));
        }
    }
    (None,None,None)
}
fn worker(args:&BTreeMap<String,String>)->Result<(),String>{
    let start=Instant::now();
    let manifest_bytes=read_bounded(required(args,"--manifest")?)?;
    let manifest:Manifest=serde_json::from_slice(&manifest_bytes).map_err(|e|e.to_string())?;
    let input=read_bounded(required(args,"--input")?)?;
    let input_loading_ns=start.elapsed().as_nanos();
    let start=Instant::now();let admitted=admit(manifest,&input)?;let group_admission_ns=start.elapsed().as_nanos();
    let prefix=args.get("--prefix").map(|s|s.parse::<usize>().map_err(|e|e.to_string())).transpose()?.unwrap_or(admitted.rows().len());
    let arm=Arm::parse(required(args,"--arm")?)?;
    let start=Instant::now();let record=run(&admitted,prefix,arm,&Limits::default())?;let component_call_ns=start.elapsed().as_nanos();
    let executable=std::env::current_exe().map_err(|e|e.to_string())?;
    let executable_sha256=sha256(&fs::read(executable).map_err(|e|e.to_string())?);
    let (process_peak_rss_kib,process_user_cpu_us,process_system_cpu_us)=process_usage();
    let envelope=WorkerRecord{schema:"m31-graph-worker/v1".into(),pid:std::process::id(),executable_sha256,
        manifest_sha256:sha256(&manifest_bytes),input_loading_ns,group_admission_ns,component_call_ns,
        process_peak_rss_kib,process_user_cpu_us,process_system_cpu_us,
        group_admission_counts:serde_json::to_value(admitted.group_check_counts()).unwrap(),record};
    let mut output=serde_json::to_vec_pretty(&envelope).map_err(|e|e.to_string())?;output.push(b'\n');
    std::io::stdout().write_all(&output).map_err(|e|e.to_string())
}

fn child_limits(command:&mut Command)->Result<(),String>{
    #[cfg(target_os="linux")]
    {
        use std::os::unix::process::CommandExt;
        // SAFETY: only async-signal-safe setrlimit calls occur after fork.
        unsafe { command.pre_exec(||{
            for (resource,value) in [(libc::RLIMIT_AS,4u64*1024*1024*1024),(libc::RLIMIT_CPU,60)] {
                let limit=libc::rlimit{rlim_cur:value as libc::rlim_t,rlim_max:value as libc::rlim_t};
                if libc::setrlimit(resource,&limit)!=0{return Err(std::io::Error::last_os_error());}
            }Ok(())
        }); }
        Ok(())
    }
    #[cfg(not(target_os="linux"))]
    { let _=command; Err("coordinator requires Linux resource-limit enforcement".into()) }
}
fn coordinator(args:&BTreeMap<String,String>)->Result<(),String>{
    let total_start=Instant::now();
    let input_path=fs::canonicalize(required(args,"--input")?).map_err(|e|e.to_string())?;
    let manifest_path=fs::canonicalize(required(args,"--manifest")?).map_err(|e|e.to_string())?;
    let bytes=read_bounded(input_path.to_str().ok_or("UTF-8 input path")?)?;
    let manifest_bytes=read_bounded(manifest_path.to_str().ok_or("UTF-8 manifest path")?)?;
    let manifest:Manifest=serde_json::from_slice(&manifest_bytes).map_err(|e|e.to_string())?;
    // No curve/group work runs in this preflight. Each child performs full
    // identity/subgroup/equation admission under its watchdog and rlimits.
    let parsed_rows=parse_rows(&bytes)?;
    let input_hash=sha256(&bytes);let manifest_hash=sha256(&manifest_bytes);
    if input_hash!=manifest.input_sha256{return Err("input SHA-256 mismatch".into());}
    let prefixes:Vec<usize>=required(args,"--prefixes")?.split(',').map(|s|s.parse().map_err(|_|"invalid prefix".to_owned())).collect::<Result<_,_>>()?;
    let rounds=args.get("--rounds").map(|s|s.parse::<usize>().map_err(|e|e.to_string())).transpose()?.unwrap_or(1);
    if prefixes.is_empty()||prefixes.len()>3||rounds==0||rounds>5||prefixes.iter().any(|p|*p==0||*p>parsed_rows.len()){
        return Err("require <=3 valid prefixes and 1..=5 rounds".into());
    }
    if prefixes.iter().collect::<std::collections::BTreeSet<_>>().len()!=prefixes.len(){return Err("duplicate prefix".into());}
    let out=PathBuf::from(required(args,"--out-dir")?);
    fs::create_dir(&out).map_err(|e|format!("output directory must be new: {e}"))?;
    let exe=std::env::current_exe().map_err(|e|e.to_string())?;
    let binary_hash=sha256(&fs::read(&exe).map_err(|e|e.to_string())?);
    let arms=[Arm::Eager,Arm::Lazy,Arm::Pivot,Arm::Mst];
    let mut attempts=Vec::new();
    let mut successful=Vec::<WorkerRecord>::new();
    let mut equivalence=Vec::new();
    let coordinator_preflight_ns=total_start.elapsed().as_nanos();
    for prefix in prefixes {
        for round in 0..rounds {
            let records_start=successful.len();
            // Deterministic rotation avoids always privileging the same arm.
            for offset in 0..4 {
                let arm=arms[(round+offset)%4];
                let stem=format!("prefix-{prefix}-round-{round}-{}",arm.name());
                let output_path=out.join(format!("{stem}.json"));let error_path=out.join(format!("{stem}.stderr"));
                let mut command=Command::new(&exe);
                command.args(["run","--input"]).arg(&input_path).arg("--manifest").arg(&manifest_path)
                    .args(["--prefix",&prefix.to_string(),"--arm",arm.name()])
                    .env("RAYON_NUM_THREADS","1").env("OMP_NUM_THREADS","1")
                    .stdin(Stdio::null()).stdout(File::create(&output_path).map_err(|e|e.to_string())?)
                    .stderr(File::create(&error_path).map_err(|e|e.to_string())?);
                child_limits(&mut command)?;
                let start=Instant::now();let mut child=command.spawn().map_err(|e|e.to_string())?;
                let mut timeout=false;
                let status=loop{
                    if let Some(status)=child.try_wait().map_err(|e|e.to_string())?{break status;}
                    if start.elapsed()>Duration::from_secs(60){timeout=true;child.kill().map_err(|e|e.to_string())?;break child.wait().map_err(|e|e.to_string())?;}
                    std::thread::sleep(Duration::from_millis(10));
                };
                let process_wall_ns=start.elapsed().as_nanos();
                let mut accepted=false;let mut validation_error=None;
                if status.success()&&!timeout{
                    match serde_json::from_slice::<WorkerRecord>(&fs::read(&output_path).map_err(|e|e.to_string())?){
                        Ok(record)=>{
                            let prefix_hash=sha256(&serde_json::to_vec(&parsed_rows[..prefix]).unwrap());
                            if record.manifest_sha256!=manifest_hash||record.executable_sha256!=binary_hash||record.record.input_sha256!=input_hash||record.record.canonical_prefix_sha256!=prefix_hash||record.record.rows!=prefix||record.record.arm!=arm
                                || record.record.source_manifest_sha256 != m31_graph_compare::SOURCE_MANIFEST_SHA256
                                || serde_json::to_value(&record.record.limits).unwrap()!=serde_json::to_value(Limits::default()).unwrap(){
                                validation_error=Some("child identity mismatch".to_owned());
                            }else{accepted=record.record.status=="accepted";successful.push(record);}
                        },Err(e)=>validation_error=Some(e.to_string()),
                    }
                }
                attempts.push(serde_json::json!({"prefix":prefix,"round":round,"arm":arm,"exit_code":status.code(),"timed_out":timeout,
                    "process_wall_ns":process_wall_ns,"accepted":accepted,"validation_error":validation_error,
                    "record":output_path.file_name().unwrap(),"stderr":error_path.file_name().unwrap()}));
            }
            let cell=&successful[records_start..];
            let complete=cell.len()==4&&cell.iter().all(|x|x.record.status=="accepted");
            let same=complete&&cell.iter().all(|x|{
                let a=&x.record;let b=&cell[0].record;
                (a.lp_rank,a.useful_small_rank,a.augmented_rank,a.target_scalar,a.target_group_verified)
                    ==(b.lp_rank,b.useful_small_rank,b.augmented_rank,b.target_scalar,b.target_group_verified)
            });
            let eager=cell.iter().find(|x|x.record.arm==Arm::Eager).and_then(|x|x.record.output_sha256.as_ref());
            let lazy=cell.iter().find(|x|x.record.arm==Arm::Lazy).and_then(|x|x.record.output_sha256.as_ref());
            let same_forest_witnesses=complete&&eager.is_some()&&eager==lazy;
            equivalence.push(serde_json::json!({"prefix":prefix,"round":round,"all_four_complete":complete,
                "same_rank_and_target_outcome":same,"same_eager_lazy_witnesses":same_forest_witnesses}));
        }
    }
    // Source/input mutations invalidate the comparison, including changes between children.
    let custody_unchanged=sha256(&read_bounded(input_path.to_str().unwrap())?)==input_hash
        &&sha256(&read_bounded(manifest_path.to_str().unwrap())?)==manifest_hash
        &&sha256(&fs::read(&exe).map_err(|e|e.to_string())?)==binary_hash;
    let all_accepted=custody_unchanged&&attempts.iter().all(|a|a["accepted"]==true);
    let comparison_equivalent=all_accepted&&equivalence.iter().all(|e|e["same_rank_and_target_outcome"]==true&&e["same_eager_lazy_witnesses"]==true);
    let summary=serde_json::json!({"schema":"m31-graph-comparison/v1","purpose":"development component comparison; no complete cold-cost inference",
        "input_sha256":input_hash,"manifest_sha256":manifest_hash,"executable_sha256":binary_hash,"custody_unchanged":custody_unchanged,
        "all_arms_accepted":all_accepted,"comparison_equivalent":comparison_equivalent,"equivalence":equivalence,"attempts":attempts,"records":successful,
        "coordinator_preflight_ns":coordinator_preflight_ns,"group_admission_in_coordinator":false,"total_process_wall_ns":total_start.elapsed().as_nanos(),
        "resource_limits":{"max_children":60,"concurrent_children":1,"child_wall_seconds":60,"child_cpu_seconds":60,"child_address_space_bytes":4294967296u64,"threads":1},
        "complete_cold_solving_cost":null,"performance_claim":null});
    fs::write(out.join("summary.json"),serde_json::to_vec_pretty(&summary).unwrap()).map_err(|e|e.to_string())?;
    println!("{}",out.join("summary.json").display());
    if !comparison_equivalent{return Err("comparison incomplete or unequal; preserved all per-arm outputs and summary".into());}Ok(())
}
fn entry()->Result<(),String>{
    let args:Vec<String>=std::env::args().skip(1).collect();
    match args.first().map(String::as_str){
        Some("run")=>worker(&args_map(&args[1..],&["--input","--manifest","--prefix","--arm"])?),
        Some("compare")=>coordinator(&args_map(&args[1..],&["--input","--manifest","--prefixes","--rounds","--out-dir"])?),
        _=>Err("usage: m31-graph-compare run --input ROWS --manifest MANIFEST --arm eager|lazy|pivot|mst|matrix-free [--prefix N]; or compare --input ROWS --manifest MANIFEST --prefixes N[,N] --rounds 1..5 --out-dir NEW_DIR".into())
    }
}
fn main(){if let Err(e)=entry(){eprintln!("{e}");std::process::exit(1);}}
