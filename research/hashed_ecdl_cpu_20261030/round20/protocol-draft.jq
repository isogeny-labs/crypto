.protocol_id = "HASHED-ECDL-CPU-STREAM-ENDPOINTS-20261030" |
.study_dir = "research/hashed_ecdl_cpu_20261030/round20" |
.question = "What complete cost and certificate behavior do streamed incidence staging and batch inversion endpoints exhibit on all three fixed fixtures?" |
.claims[1].text = "Matched integrated parents isolate streamed incidence staging and residual batch endpoints while preserving exact full certificates." |
.arms |= map(if .id == "v" or .id == "w" then .instrument.commit=$candidate.repository_commit | .instrument.binary_sha256=$candidate.binary_sha256 | .description="Accumulated CPU candidate" | .instrument.flags |= del(.effective_source_sha256) elif .id == "y" then .instrument.commit=$previous.repository_commit | .instrument.binary_sha256=$previous.binary_sha256 | .description="Before endpoint-product removal" | .parameters.oracle=.description | .instrument.flags |= del(.effective_source_sha256) elif .id == "z" then .instrument.commit="5cb7a4072078c0eadabd9aabfcc3be3baaa14d37" | .instrument.binary_sha256="3391976b69981891ccb5f133cfa8aeb54201e29b48c77e3c322dd5a2414753e8" | .description="Before streamed matrix staging" | .parameters.oracle=.description | .instrument.flags |= del(.effective_source_sha256) else . end) |
.variable_ledger.independent[0].levels=["original baseline","accumulated candidate batch64","accumulated candidate batch1","before endpoints","before streaming"] |
.variable_ledger.factors[0].value="Pinned original baseline and three integrated source snapshots" |
.variable_ledger.factors[0].how="y/v differs only in CPU endpoint products; z/y differs only in shared streamed reduction. Source-delta files and manifests bind each contrast. u/v measures the whole accumulated implementation, not an individual change." |
.variable_ledger.factors[-1].how="Analyst is unblinded. Frozen native analyzer computes exact certificate checks and descriptive ranges from neutral labels; labels do not provide human blinding." |
.variable_ledger.comparisons |= map(if .id=="C-relations" then .id="C-endpoints" elif .id=="C-affine" then .id="C-streaming" else . end) |
.decision_rule.comparison_ids=["C-native","C-batch","C-endpoints","C-streaming"] |
.randomization.blinding={analysis_labels_masked:false,how:"Unblinded analysis with fixed neutral machine labels; producer knows source mapping.",unmasked_at:null} |
.analysis_plan.blinded=false |
.execution.placement.binary_source.candidate=$candidate.repository_commit |
.execution.placement.binary_source.previous=$previous.repository_commit |
.execution.placement.binary_source.inverse="5cb7a4072078c0eadabd9aabfcc3be3baaa14d37" |
.execution.placement.candidate_materialization={path:"candidate-materialization.json",sha256:$ch,integrated:true} |
.execution.placement.previous_materialization={path:"previous-materialization.json",sha256:$ph,integrated:true} |
.execution.placement.inverse_materialization={path:"inverse-materialization.json",sha256:$zh,integrated:true} |
.reporting.report_dir="research/hashed_ecdl_cpu_20261030/reports/round20" |
.freeze={status:"draft",protocol_sha256:null,commit:null,frozen_at:null,filed_before_affected_data:null}
