.protocol_id = "HASHED-ECDL-CPU-STAGE-PROFILE-20261101" |
.study_dir = "research/hashed_ecdl_cpu_20261030/round22" |
.question = "Which instrumented stages contribute to complete CPU recovery cost, with ordinary controls on all three fixed fixtures?" |
.claims[1].text = "Profiled and ordinary candidate paths retain identical full certificates; stage clocks are diagnostics and are not an exhaustive partition." |
.arms |= map(
  if .id=="u" or .id=="x" then .
  else .instrument.commit=$candidate.repository_commit | .instrument.binary_sha256=$candidate.binary_sha256 |
    .description=(if .id=="v" then "Ordinary candidate batch64" elif .id=="w" then "Ordinary candidate batch1" elif .id=="y" then "Profiled candidate batch64" else "Profiled candidate batch1" end) |
    .parameters.oracle=.description |
    .instrument.flags.arm=(if .id=="w" or .id=="z" then "w" else "v" end) |
    .instrument.flags.profile_stages=((.id=="y" or .id=="z")|tostring)
  end) |
.variable_ledger.independent[0].levels=["original baseline","ordinary batch64","ordinary batch1","profiled batch64","profiled batch1"] |
.variable_ledger.factors[0].value="Original baseline and one pinned candidate executable" |
.variable_ledger.factors[0].how="y/v and z/w differ only by additional stage clocks; w/v differs in batch size. u/x is the original baseline A/A pair. Profile clocks cannot establish an uninstrumented speedup." |
.variable_ledger.comparisons |= map(if .id=="C-pivots" then .id="C-profile64" elif .id=="C-replay" then .id="C-profile1" | .arms=["z","w"] else . end) |
.decision_rule.comparison_ids=["C-native","C-batch","C-profile64","C-profile1"] |
.decision_rule.text="All180cells must complete with full certificate equality and negative controls. Require all60 profiled stage rows to match their execution identities and complete/replay values. Stage medians guide follow-up only; do not add medians or claim an exhaustive partition or isolated speedup." |
.analysis_plan.inputs=["rows.csv","stages-{y,z}-{case}-{round}.csv","cert-{arm}-{case}-{round}.txt"] |
.analysis_plan.outputs+=["stage-summary.csv"] |
.analysis_plan.script.sha256=$analyze |
.metrics.secondary += (["solve","bases","targets","oracle","linear","internal_replay","external_replay","complete"] | map({
  id:("stage_"+.+"_ms"),name:("profiled "+.),kind:"timed",unit:"milliseconds",aggregation:"median",resolution:"Rust Instant; instrumented L0 diagnostics",uncertainty:"range",source:("stages CSV "+.),definition:("Profiled "+.+" interval accumulated across trials; extra clock overhead, not a complete partition. Available only in y/z.")})) |
.execution.placement.binary_source.candidate=$candidate.repository_commit |
.execution.placement.binary_source.previous=$candidate.repository_commit |
.execution.placement.binary_source.inverse=$candidate.repository_commit |
.execution.placement.candidate_materialization.sha256=$ch |
.execution.placement.previous_materialization.sha256=$ph |
.execution.placement.inverse_materialization.sha256=$zh |
.reporting.report_dir="research/hashed_ecdl_cpu_20261030/reports/round22" |
.literature += [{ref:"research/hashed_ecdl_cpu_20261030/stage-profile/README.md",provenance:"retrieved",claim:"Reduction/other includes target generation, relation checking, linear work and internal replay. Existing stage clocks separate these without changing query/certificate content.",verified_by:"Source review of ReductionStats and solve_reduction; 46 tests include profiled/ordinary parity. Round21 raw stage summaries motivate this descriptive experiment."}] |
.freeze={status:"draft",protocol_sha256:null,commit:null,frozen_at:null,filed_before_affected_data:null}
