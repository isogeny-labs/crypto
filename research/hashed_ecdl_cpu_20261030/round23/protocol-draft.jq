.protocol_id="HASHED-ECDL-CPU-SUFFIX-FILTER-20261101" |
.created="2026-11-01" |
.study_dir="research/hashed_ecdl_cpu_20261030/round23" |
.question="What complete cost and exact certificate behavior does a suffix rejection bitmap exhibit at batch64 and batch1 on all three fixed fixtures?" |
.claims[1].text="Filter-on and filter-off source snapshots preserve exact full certificates at both batch sizes." |
.arms |= map(if .id=="v" or .id=="w" then .instrument.commit=$c.repository_commit | .instrument.binary_sha256=$c.binary_sha256 | .description=(if .id=="v" then "Suffix filter batch64" else "Suffix filter batch1" end) | .parameters.oracle=.description elif .id=="y" or .id=="z" then .instrument.commit=$p.repository_commit | .instrument.binary_sha256=$p.binary_sha256 | .description=(if .id=="y" then "No suffix filter batch64" else "No suffix filter batch1" end) | .parameters.oracle=.description | .instrument.flags.arm=(if .id=="y" then "v" else "w" end) else . end) |
.variable_ledger.independent[0].levels=["original baseline","filter batch64","filter batch1","no filter batch64","no filter batch1"] |
.variable_ledger.factors[0].value="Pinned original baseline and matched filter-on/off CPU source snapshots" |
.variable_ledger.factors[0].how="y/v and z/w isolate the suffix rejection filter at fixed batch size; w/v and z/y isolate batching. candidate-source-delta.patch binds the runtime change. Original u/v measures accumulated implementation differences." |
.variable_ledger.comparisons |= map(if .id=="C-pivots" then .id="C-filter64" elif .id=="C-replay" then .id="C-filter1" | .arms=["z","w"] else . end) |
.decision_rule.comparison_ids=["C-native","C-batch","C-filter64","C-filter1"] |
.execution.placement.binary_source.candidate=$c.repository_commit |
.execution.placement.binary_source.previous=$p.repository_commit |
.execution.placement.binary_source.inverse=$z.repository_commit |
.execution.placement.candidate_materialization={path:"candidate-materialization.json",sha256:$ch,integrated:true} |
.execution.placement.previous_materialization={path:"previous-materialization.json",sha256:$ph,integrated:true} |
.execution.placement.inverse_materialization={path:"inverse-materialization.json",sha256:$zh,integrated:true} |
.reporting.report_dir="research/hashed_ecdl_cpu_20261030/reports/round23" |
.literature += [{ref:"research/hashed_ecdl_cpu_20261030/oracle-filter/README.md",provenance:"retrieved",claim:"Exact rejection filter followed by the original lookup; collisions affect cost only. Added memory and preparation must be charged.",verified_by:"Source inspection, 47 library tests and independent saturated-filter lookup checks; no timing inference."},{ref:"research/hashed_ecdl_cpu_20261030/round22/results/stage-summary.csv",provenance:"retrieved",claim:"Oracle is the largest recorded individual stage in fixtures1/2; profile clocks are descriptive.",verified_by:"Completed round22 native analyzer and 500-entry manifest verification."}] |
.freeze={status:"draft",protocol_sha256:null,commit:null,frozen_at:null,filed_before_affected_data:null}
