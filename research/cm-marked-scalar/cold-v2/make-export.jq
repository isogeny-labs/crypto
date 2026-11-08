. as $old |
{
schema:"experiment.export/v1",export_id:"CM-CACHE-COLD-V2-admission-001",created:$utc,
protocol:{path:"research/cm-marked-scalar/protocol/cache-cold-v2/protocol.json",sha256:$protocol.freeze.protocol_sha256,protocol_id:$protocol.protocol_id,version:$protocol.version},
producer:{runtime:"codex_cli/native-cpp",model:null,session:"T-1888",role:"producer"},
source:{repo:"isogeny-labs/crypto",commit:$protocol.freeze.commit,dirty:true},
backend:"local",host:{path:"research/cm-marked-scalar/cold-v2/host.json",sha256:$host_sha,class_id:"native-admission-refused",summary:{cpu_model:$protocol.execution.placement.expected_host.cpu_model,logical_cpus:$protocol.execution.placement.expected_host.logical_cpus,memory_bytes:null,kernel:$protocol.execution.placement.expected_host.kernel,hypervisor:null,isolation_tier_max:"none"}},
arms:[$protocol.arms[]|{id:.id,role:.role,binary_sha256:.instrument.binary_sha256,commit:.instrument.commit}],
runs:[{run_id:"calibration-admission-001",arm:"cache-default",instance_id:"all-five-curves:calibration-panel-preflight",seed:null,replicate:0,warmup:false,status:"infra_error",outcome_class:"not_completed",attempts:1,resumed:0,checkpoint:null,
measurements:{counted:{},wall_seconds:null,wall_valid:false,cpu_user_seconds:null,cpu_sys_seconds:null,max_rss_bytes:null,counters:{preflight_other_cpu_seconds:$receipt.other_cpu_seconds,preflight_cpu_psi_avg10:$receipt.psi_cpu_avg10,preflight_memory_psi_avg10:$receipt.psi_memory_avg10}},
isolation:{tier:"none",contended:null,blockers:["busy preflight: other CPU1.8s >0.2s limit; worker never started"]},verification:{status:"not_run",verifier:null},exclusion:{excluded:true,rule:"Native preflight refusal; no worker or timing rows"},artifacts:[],started_at:null,finished_at:null,worker:"native-isolate preflight; no measurement child"}],
flow:{planned:2,attempted:1,completed:0,not_completed:1,interrupted:0,resumed:0,excluded:[{rule:"Native preflight refusal",run_ids:["calibration-admission-001"]}]},
files:$files,
analysis:{script:{path:$protocol.analysis_plan.script.path,sha256:$protocol.analysis_plan.script.sha256,bytes:$analysis_bytes,kind:"analysis"},blinded:false,blinding_key_sha256:null,outputs:[],unmasked_at:null},
notes:[{kind:"observation",by:"native producer",text:"The export row represents the shared calibration admission gate for all25 configurations, not a candidate function call. Heldout was conditional and never attempted. No measured point outputs or clocks exist."},{kind:"observation",by:"native producer",text:"Source was published before the frozen protocol; both hosted receipts preceded admission. Dirty=true records untracked publication receipts and new output artifacts at launch; implementation and nine frozen inputs were unchanged."},{kind:"observation",by:"native producer",text:"No unchanged retry, cloud submission, generator/controller restart or role/budget/cap change. Summary records zero measurement workers and rows. Software qualification is separate from empirical timing."}],reviews_dir:"reviews"
}
