. as $old | $plans[0] as $p | $protocol[0] as $protocol |
.export_id="CM-RESIDUAL-SPLIT-V1-census" | .created=$created |
.source.commit=$protocol.freeze.commit |
.protocol={path:"research/cm-marked-scalar/protocol/residual-v1/protocol.json",sha256:$protocol.freeze.protocol_sha256,protocol_id:$protocol.protocol_id,version:1} |
.host.path="research/cm-marked-scalar/residual-v1/host.json" | .host.sha256=$host_sha |
.host.summary.cpu_model=$host[0].cpu_model | .host.summary.kernel=$host[0].kernel |
.host.summary.logical_cpus=$logical_cpus | .host.summary.memory_bytes=$memory_bytes |
.arms=[$protocol.arms[]|{id,role,binary_sha256:.instrument.binary_sha256,commit:.instrument.commit}] |
.files=$files[0] |
.analysis.script={path:"research/cm-marked-scalar/residual.cpp",sha256:$protocol.analysis_plan.script.sha256,bytes:$source_bytes,kind:"analysis"} |
.analysis.outputs=[$files[0][]|select(.kind!="raw")] |
.flow={planned:60,attempted:60,completed:60,not_completed:0,interrupted:0,resumed:0,excluded:[]} |
.notes=[
 {kind:"observation",by:"native producer",text:"One complete42480-row census. The60 export rows are derived minimum records:5curves x3method arms x4weights, not60 process runs or timing trials."},
 {kind:"observation",by:"native producer",text:"Primary weight1 compares relation to both direct and binary controls. Weights2..4 are declared sensitivities. No population or statistical-power inference."},
 {kind:"observation",by:"native producer",text:"Independent GMP replay validates every raw row and20minimum sets. All300selected-plan point/count checks pass. Five corrupted raw/plan/protocol controls reject at intended checks."},
 {kind:"observation",by:"native producer",text:"Original mu and input points are fixed. No generator or target substitution. Entire prefix, both tables, signed residual and main loop are charged in each schedule."},
 {kind:"observation",by:"native producer",text:"No cryptographic clock or cloud submission. Search/plan preparation, field normalization, conversion, allocation, verification and destruction remain in future full first-use runtime accounting."}
] |
.runs=[$p.plans[] as $entry | ["direct","binary_control","relation"][] as $method | $entry[$method] as $plan |
 ($old.runs[0] |
 .run_id=($entry.curve+"-w"+($entry.addition_weight|tostring)+"-"+$method) |
 .arm=(if $method=="binary_control" then "binary" else $method end) |
 .instance_id=($entry.curve+":weight"+($entry.addition_weight|tostring)) |
 .measurements.counted.D_plus_A=($plan.doubles+$plan.adds) |
 .measurements.counted.D_A_N_tables=([$plan.doubles,$plan.adds,$plan.negations,$plan.table_points]|map(tostring)|join(",")) |
 .measurements.counters=($plan+{addition_weight:$entry.addition_weight}) |
 .verification.verifier="research/cm-marked-scalar/residual-v1/independent-replay.json and selected-qualification.json"
 )]
