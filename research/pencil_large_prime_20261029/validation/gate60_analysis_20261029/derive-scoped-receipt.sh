#!/usr/bin/env bash
# Metadata projection of completed native preparation; no instrument execution.
set -euo pipefail
repo=/home/ec2-user/crypto-pencil-signed-graph-20261029
study="$repo/research/pencil_large_prime_20261029"
out="$study/validation/gate60_analysis_20261029"
parent="$study/native_preparation/validation/prepared-parity-result.json"
plan=/home/ec2-user/pencil-build-40938439022/prepared-plan-parity/gate-p739.json
plan_sha=$(sha256sum "$plan" | cut -d' ' -f1)
[[ "$plan_sha" == bc57819e43862f5d9c9aee0f13b983fec186b2dd14dbc8b921d8999acc4b6776 ]]
jq -e --arg sha "$plan_sha" '.passed==true and .solver_executions_performed==0 and .binary_sha256=="a1c8ca89748a22b8a07ea00fd3983061be0d8197d5cae49fca883dcf5dd50eab" and .exact_retained_plan_bytes==true and .first_measured_seeds_match_archive==true and .all_three_arms_share_each_workload_round_seed==true and .reference_and_aa_resolved_methods_identical==true and .candidate_resource_parameters_match_manifest==true and .measurement_and_gate_contracts_match_manifest==true and ([.files[]|select(.spec=="gate-p739.json" and .sha256==$sha and .targets==60 and .planned_executions==1080 and .exact_retained_plan_bytes==true)]|length)==1' "$parent" >/dev/null
jq -e '(.workloads|length)==60 and (.executions|length)==1080 and ([.executions[]|select(.warmup)]|length)==180 and ([.executions[]|select(.warmup==false)]|length)==900' "$plan" >/dev/null
set -o noclobber
jq -n --slurpfile parent "$parent" --slurpfile plan "$plan" --arg parent_path "$parent" --arg parent_sha "$(sha256sum "$parent" | cut -d' ' -f1)" --arg plan_sha "$plan_sha" '{schema:"pencil-prepared-plan-validation/v1",passed:true,binary_sha256:$parent[0].binary_sha256,targets:($plan[0].workloads|length),planned_executions:($plan[0].executions|length),measured_executions:([$plan[0].executions[]|select(.warmup==false)]|length),warmup_executions:([$plan[0].executions[]|select(.warmup)]|length),solver_executions_performed:0,files:{"gate-p739.json":$plan_sha},parent_native_validation:{path:$parent_path,sha256:$parent_sha},scope:"Exact p739 gate metadata projection from completed native preparation; no new solver or planning invocation",launch_authorized:false}' > "$out/scoped-prepared-receipt.json"
