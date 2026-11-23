.protocol_id = "HASHED-ECDL-CPU-REPLAY-PIVOTS-20261030" |
.study_dir = "research/hashed_ecdl_cpu_20261030/round21" |
.question = "What complete cost and certificate behavior do projective replay and a contiguous pivot directory exhibit on all three fixed fixtures?" |
.claims[1].text = "Matched integrated parents isolate projective replay and the contiguous pivot directory while preserving exact full certificates." |
.arms |= map(if .id == "v" or .id == "w" then .instrument.commit=$candidate.repository_commit | .instrument.binary_sha256=$candidate.binary_sha256 | .description="Accumulated CPU candidate" elif .id == "y" then .instrument.commit=$previous.repository_commit | .instrument.binary_sha256=$previous.binary_sha256 | .description="Projective replay before compact pivot directory" | .parameters.oracle=.description elif .id == "z" then .instrument.commit=$inverse.repository_commit | .instrument.binary_sha256=$inverse.binary_sha256 | .description="Before projective replay and compact pivot directory" | .parameters.oracle=.description else . end) |
.variable_ledger.independent[0].levels=["original baseline","accumulated candidate batch64","accumulated candidate batch1","before compact pivots","before projective replay"] |
.variable_ledger.factors[0].value="Pinned original baseline and three integrated source snapshots" |
.variable_ledger.factors[0].how="y/v differs only in compact pivot directory and software tests; z/y differs only in projective replay and software tests. u/v measures the accumulated implementation, not an individual change. Source snapshots and delta files bind the contrasts." |
.variable_ledger.comparisons |= map(if .id=="C-endpoints" then .id="C-pivots" elif .id=="C-streaming" then .id="C-replay" else . end) |
.decision_rule.comparison_ids=["C-native","C-batch","C-pivots","C-replay"] |
.execution.placement.binary_source.candidate=$candidate.repository_commit |
.execution.placement.binary_source.previous=$previous.repository_commit |
.execution.placement.binary_source.inverse=$inverse.repository_commit |
.execution.placement.candidate_materialization={path:"candidate-materialization.json",sha256:$ch,integrated:true} |
.execution.placement.previous_materialization={path:"previous-materialization.json",sha256:$ph,integrated:true} |
.execution.placement.inverse_materialization={path:"inverse-materialization.json",sha256:$zh,integrated:true} |
.reporting.report_dir="research/hashed_ecdl_cpu_20261030/reports/round21" |
.literature += [{ref:"https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html",provenance:"retrieved",claim:"Jacobian equality follows by cross multiplying x=X/Z^2 and y=Y/Z^3, with identity handled separately.",verified_by:"Primary coordinate definition reviewed during projective-replay implementation; independent affine-reference tests retained."},{ref:"https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert",provenance:"retrieved",claim:"Contiguous traversal trades tree nodes for linear shifting on out-of-order pivot insertion; no latency improvement follows from the data structure alone.",verified_by:"Official Vec and slice partition_point contracts reviewed during pivot-directory implementation; independent tree-based reference checks exact dependency coefficients."}] |
.freeze={status:"draft",protocol_sha256:null,commit:null,frozen_at:null,filed_before_affected_data:null}
