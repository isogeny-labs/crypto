.protocol_id="HASHED-ECDL-CPU-RESIDUAL-FILTER-20261101" |
.created="2026-11-01" |
.study_dir="research/hashed_ecdl_cpu_20261030/round24" |
.question="What complete cost does early residual x rejection exhibit versus the measured suffix lookup filter at batch64 and batch1 on all three fixtures?" |
.claims[1].text="Early residual rejection and suffix-lookup-only source snapshots preserve exact full certificates at both batch sizes." |
.arms |= map(if .id=="v" or .id=="w" then .instrument.commit=$c.repository_commit | .instrument.binary_sha256=$c.binary_sha256 | .description=(if .id=="v" then "Early residual rejection batch64" else "Early residual rejection batch1" end) | .parameters.oracle=.description elif .id=="y" or .id=="z" then .instrument.commit=$p.repository_commit | .instrument.binary_sha256=$p.binary_sha256 | .description=(if .id=="y" then "Suffix lookup filter only batch64" else "Suffix lookup filter only batch1" end) | .parameters.oracle=.description else . end) |
.variable_ledger.independent[0].levels=["original baseline","early residual batch64","early residual batch1","lookup-only batch64","lookup-only batch1"] |
.variable_ledger.factors[0].value="Original baseline and matched early-residual versus lookup-only source snapshots" |
.variable_ledger.factors[0].how="y/v and z/w isolate early residual rejection at fixed batch size, including flags and extra scratch memory. w/v isolates batching. candidate-source-delta.patch binds the runtime change. u/v measures accumulated changes." |
.variable_ledger.comparisons |= map(if .id=="C-filter64" then .id="C-residual64" elif .id=="C-filter1" then .id="C-residual1" else . end) |
.decision_rule.comparison_ids=["C-native","C-batch","C-residual64","C-residual1"] |
.execution.placement.binary_source.candidate=$c.repository_commit |
.execution.placement.binary_source.previous=$p.repository_commit |
.execution.placement.binary_source.inverse=$z.repository_commit |
.execution.placement.candidate_materialization={path:"candidate-materialization.json",sha256:$ch,integrated:true} |
.execution.placement.previous_materialization={path:"previous-materialization.json",sha256:$ph,integrated:true} |
.execution.placement.inverse_materialization={path:"inverse-materialization.json",sha256:$zh,integrated:true} |
.reporting.report_dir="research/hashed_ecdl_cpu_20261030/reports/round24" |
.literature += [{ref:"research/hashed_ecdl_cpu_20261030/residual-filter/README.md",provenance:"retrieved",claim:"Skipping y after exact x absence preserves inverse recurrence and witness order; extra flags and branching must be charged.",verified_by:"Source inspection and 48 library tests including affine-reference rejection and archived certificates."},{ref:"research/hashed_ecdl_cpu_20261030/round23/EXECUTION.md",provenance:"retrieved",claim:"The suffix-only parent has complete-cost observations on every fixture and batch size; no old timing rows are pooled.",verified_by:"Completed 180-row panel, native export and 379-entry manifest verification."}] |
.freeze={status:"draft",protocol_sha256:null,commit:null,frozen_at:null,filed_before_affected_data:null}
