.protocol_id="HASHED-ECDL-CPU-NUMERATOR-SCRATCH-20261105" |
.created="2026-11-05" |
.study_dir="research/hashed_ecdl_cpu_20261030/round25" |
.question="What complete cost does eliminating numerator scratch storage exhibit versus the retained suffix-only implementation at batch64 and batch1 on all three fixtures?" |
.claims[1].text="Both source snapshots preserve exact full certificates and residual inversion counts at fixed batch size." |
.arms |= map(if .id=="v" or .id=="w" then .instrument.commit=$c.repository_commit | .instrument.binary_sha256=$c.binary_sha256 | .description=(if .id=="v" then "No numerator scratch batch64" else "No numerator scratch batch1" end) | .parameters.oracle=.description else . end) |
.variable_ledger.independent[0].levels=["original baseline","no numerator scratch batch64","no numerator scratch batch1","suffix-only parent batch64","suffix-only parent batch1"] |
.variable_ledger.factors[0].value="Original baseline and matched numerator-scratch versus no-numerator-scratch source snapshots" |
.variable_ledger.factors[0].how="y/v and z/w isolate numerator scheduling and scratch removal at fixed batch size. Reverse-pass point-y reads and branches are charged. w/v isolates batching. candidate-source-delta.patch binds the runtime change. u/v measures accumulated changes." |
.variable_ledger.comparisons |= map(if .id=="C-residual64" then .id="C-scratch64" elif .id=="C-residual1" then .id="C-scratch1" else . end) |
.decision_rule.comparison_ids=["C-native","C-batch","C-scratch64","C-scratch1"] |
.execution.placement.binary_source.candidate=$c.repository_commit |
.execution.placement.candidate_materialization={path:"candidate-materialization.json",sha256:$ch,integrated:true} |
.reporting.report_dir="research/hashed_ecdl_cpu_20261030/reports/round25" |
.literature |= map(select(.ref!="research/hashed_ecdl_cpu_20261030/residual-filter/README.md")) |
.literature += [{ref:"research/hashed_ecdl_cpu_20261030/residual-scratch/README.md",provenance:"retrieved",claim:"Numerators can be reconstructed from the original points in the reverse inversion pass with identical affine equations; storage savings do not establish latency savings.",verified_by:"Published source c47886258a0e and 47 release library tests, including exhaustive affine parity and archived certificates."}] |
.extensions.round25_cost_policy={currency:"USD",attempt_ceiling:2,total_attempts:1,total_retries:0,cloud_builds:0,concurrency:1,cpu:2,memory_gib:4,job_deadline_seconds:300,daily_shared_ceiling:200,monthly_ceiling:1000,includes:["whole worker provisioning and idle lifetime","failed scheduling and execution","Spot interruption","storage","transfer","builds"],admission:"STOP unless verified unreserved daily/monthly funds and live resource caps cover this all-in reservation; delayed Budget totals alone are insufficient. No alternate pool/provider or renamed retry.",ceiling_is_not_cost_estimate:true} |
.freeze={status:"draft",protocol_sha256:null,commit:null,frozen_at:null,filed_before_affected_data:null}
