.protocol_id="CM-RESIDUAL-SPLIT-V1" |
.question="Can an exact nonzero remainder reduce the full fixed-mu closed-loop group schedule below both optimized direct and binary-radix controls?" |
.claims=[
 {id:"C-fixed",text:"Every relation and binary split evaluates the original fixed mu point action.",class:"exact_computation",decided_by:[]},
 {id:"C-cost",text:"Relation residual split has strictly fewer D+A calls than both optimized direct and binary-radix controls within the frozen finite census.",class:"comparative",decided_by:["C-route-direct","C-route-binary"],if_supported:"Exact source-specific count result; no timing implication.",if_not_supported:"Preserve the tie or loss and all raw schedules."}
] |
(.arms[0]) as $template |
.arms=([
 {id:"direct",role:"reference",description:"Regular direct signed-window multiplication, every translation -256..256 and width2..7."},
 {id:"binary",role:"baseline",description:"Same joint split evaluator, radices1,2,4,8,16,32, offsets-64..64 and widths2..7; complete prefix and both tables."},
 {id:"relation",role:"candidate",description:"Same joint split evaluator at the certified ell, offsets-64..64 and widths2..7; full closed prefix and both tables."}
] | map(. + {instrument:($template.instrument | .commit=null | .build=[["make","-B","-j1","-C","research/cm-marked-scalar","residual-binary","residual-replay-binary"]] | .binary_sha256=null | .flags.claim="Exact fixed-mu joint group schedules; no clock" )})) |
.controls.aa.not_applicable_reason="Exact deterministic complete finite census; native counters qualified against actual operations, with independent GMP replay. No clock or stochastic noise floor." |
.controls.positive.not_applicable_reason="Separate native qualification checks3150 point/count cases and450 explicit-map cases on all five curves;300 selected-plan cases must pass." |
.controls.negative.not_applicable_reason="Five deliberately changed remainders must disagree with the target point; identity and signed inputs are checked. Post-scan mutations must reject. No stochastic null inferred." |
.variable_ledger.independent=[{name:"method",levels:["direct","binary","relation"]},{name:"offset",levels:["every -64..64 for split; every -256..256 for direct"]}] |
.variable_ledger.factors=[
 {name:"curve and fixed mu",treatment:"fixed",value:["P-192","P-224","P-256","P-384","P-521"],how:"Same original seven fixture hashes. No scalar, generator or input changes across arms."},
 {name:"offset and radix",treatment:"crossed",value:"Binary radices1,2,4,8,16,32 and relation ell; every offset-64..64",how:"Complete split design; independent direct translation census -256..256."},
 {name:"window",treatment:"crossed",value:[2,3,4,5,6,7],how:"Both split arms use identical windows; width2 prefix and residual term. Direct receives same six widths."},
 {name:"instrument",treatment:"fixed",value:"One producer binary, OpenSSL backend and original mu",how:"Published hashes before data. Independent GMP replay reconstructs every row."},
 {name:"host",treatment:"measured",value:"Local, one process/thread and2GiB address-space ceiling",how:"Exact counter census only. No cryptographic timing, no cloud submission."},
 {name:"score",treatment:"fixed",value:"Primary D+A, weights2..4 sensitivity",how:"Tie: score,N,D,A,T,radix,abs(offset),offset,width. No latency interpretation."}
] |
.variable_ledger.comparisons=[
 {id:"C-route-direct",arms:["relation","direct"],differs_in:["method"],design:"one_factor"},
 {id:"C-route-binary",arms:["relation","binary"],differs_in:["method"],design:"one_factor"}
] |
.randomization.arm_order_reason="Exact complete deterministic schedules; no hardware-dependent metric." |
.randomization.instance_sampling.rule="Five curves;7radices x129offsets x6widths plus513direct shifts x6widths:42480 rows." |
.metrics.primary[0].definition="Exact D+A including full prefix and both tables; minimum separately over each arm finite design." |
.metrics.secondary[0].definition="Actual D,A,N,cumulative-table-point vector; sensitivity D+weight*A for weights2,3,4 only." |
.execution.placement=(.execution.placement | del(.shift_min,.shift_max) | .offset_min=-64 | .offset_max=64 | .direct_shift_min=-256 | .direct_shift_max=256 | .binary_radices=[1,2,4,8,16,32] | .independent_replay_path="research/cm-marked-scalar/residual_replay.cpp" | .independent_replay_binary_sha256=null | .independent_replay_source_sha256=null) |
.sample_size.power.note="One exact complete census. No population or statistical-power claim. Strict inequalities against both reference arms decide each curve separately." |
.sample_size.preflight_runtime.measured_on="No cryptographic timing pilot; all invocations capped120 seconds and one scientific scan attempt." |
.stopping_rules=["Stop after exactly42480 rows or retain the first timeout/failure; no unchanged retry.","Any point/counter/protocol/hash/design failure invalidates the run."] |
.decision_rule.text="Per curve at weight1 compare relation minimum to both direct and binary minima. Strictly below both: supported_within_scope; equal best reference: tied; otherwise candidate_higher_count. Preserve full vectors and all sensitivities." |
.decision_rule.comparison_ids=["C-route-direct","C-route-binary"] |
.decision_rule.statistic="Exact minimum; tie by score,N,D,A,T,radix,abs(offset),offset,width" |
.analysis_plan.script={path:"research/cm-marked-scalar/residual.cpp",sha256:null} |
.analysis_plan.inputs=["scan.csv","plans.json","independent GMP replay","300 selected-plan point checks"] |
.analysis_plan.frozen_before_data=false |
.inadmissible=["Changing mu or orbit generator","Omitting residual, prefix, table, negation or point-normalization work","Claiming runtime from D+A","Optimizing only the candidate","Changing design after data","Hiding plan-generation work in first-use accounting","Claiming a global noninteger quotient from subgroup output agreement"] |
.literature=[{ref:"https://www.helmut-prodinger.at/pdffiles/joint.pdf",provenance:"retrieved",claim:"Simultaneous scalar linear combinations and signed expansions are established background. This implementation uses separate window expansions, without optimality or novelty claims.",verified_by:"Introduction and section2 read2026-11-08"}] |
.freeze={status:"draft",protocol_sha256:null,commit:null,frozen_at:null,filed_before_affected_data:false}
