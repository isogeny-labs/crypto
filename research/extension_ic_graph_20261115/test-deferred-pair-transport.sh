#!/usr/bin/env bash
# Process/custody simulation only; no scientific worker is executed locally.
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
scratch=$(mktemp -d /tmp/ic-deferred50-transport.XXXXXX)
cpu=$(awk '/^Cpus_allowed_list:/{split($2,a,/[,-]/);print a[1]}' /proc/self/status)
fixture=/workspace/ic-runpod-runs/runpod-extension-pipeline-20261116-41/returned/task/run/fixture.json
for case_name in positive first-failure bad-input-hash; do
  run=$scratch/$case_name
  mkdir "$run"
  cp "$here/run-deferred-pair-task.sh" "$run/wrapper"
  cp "$fixture" "$run/control-fixture.json"
  jq --arg cpu "$cpu" '. + {pinned_cpu:$cpu,source_revision:"1111111111111111111111111111111111111111"}' "$here/deferred-pair-plan.json" > "$run/runtime-plan.json"
  if [[ $case_name == bad-input-hash ]]; then jq '.public_sha256="bad"' "$run/runtime-plan.json" > "$run/changed-plan.json"; cp "$run/changed-plan.json" "$run/runtime-plan.json"; fi
  cat > "$run/driver" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
set -C
generate=0
method=unset
source=unset
out=unset
root_table=false
pair_cap=0
edge_cap=0
rho_batch=1
rho_negation=true
while [[ $# -gt 0 ]]; do
  case $1 in
    --generate) generate=1; shift ;;
    --root-table) root_table=true; shift ;;
    --pair-split-cap) pair_cap=$2; shift 2 ;;
    --deferred-edge-cap) edge_cap=$2; shift 2 ;;
    --rho-batch) rho_batch=$2; shift 2 ;;
    --rho-plain) rho_negation=false; shift ;;
    --method) method=$2; shift 2 ;;
    --source-revision) source=$2; shift 2 ;;
    --out) out=$2; shift 2 ;;
    *) shift ;;
  esac
done
if [[ $generate == 1 ]]; then jq --arg source "$source" '.source_revision=$source' "$(dirname "$0")/control-fixture.json" > "$out"; exit 0; fi
if [[ $(basename "$out") == original-ic.raw.json && -e $(dirname "$0")/simulate-first-failure ]]; then exit 17; fi
printf '{"simulated_transport":true,"method":"%s","root_table":%s,"pair_cap":%s,"edge_cap":%s,"rho_batch":%s,"rho_negation":%s}\n' "$method" "$root_table" "$pair_cap" "$edge_cap" "$rho_batch" "$rho_negation" > "$out"
SH
  chmod +x "$run/driver"
  [[ $case_name != first-failure ]] || touch "$run/simulate-first-failure"
  if [[ $case_name == bad-input-hash ]]; then
    if bash "$run/wrapper" --execute "$run"; then echo 'changed public hash accepted' >&2; exit 1; fi
    [[ ! -e $run/original-ic.exit-code.txt && $(wc -l < "$run/flow.jsonl") == 2 ]]
  else
    bash "$run/wrapper" --execute "$run"
    [[ $(wc -l < "$run/flow.jsonl") == 18 ]]
    expected=0
    [[ $case_name != first-failure ]] || expected=17
    [[ $(cat "$run/original-ic.exit-code.txt") == "$expected" ]]
    for role in quadratic-table pair-discriminant pair-deferred rho-plain-b1 rho-neg-b1 rho-plain-b16 rho-neg-b16; do [[ $(cat "$run/$role.exit-code.txt") == 0 ]]; done
    jq -e '.root_table==true and .method=="extension-sparse-remainders"' "$run/quadratic-table.raw.json" >/dev/null
    jq -e '.root_table==false and .method=="discriminant-pairs" and .pair_cap==524288' "$run/pair-discriminant.raw.json" >/dev/null
    jq -e '.root_table==false and .method=="deferred-pairs" and .pair_cap==524288 and .edge_cap==262144' "$run/pair-deferred.raw.json" >/dev/null
    for batch in 1 16; do
      jq -e --argjson batch "$batch" '.method=="rho" and .rho_batch==$batch and .rho_negation==false and .pair_cap==0 and .edge_cap==0' "$run/rho-plain-b$batch.raw.json" >/dev/null
      jq -e --argjson batch "$batch" '.method=="rho" and .rho_batch==$batch and .rho_negation==true and .pair_cap==0 and .edge_cap==0' "$run/rho-neg-b$batch.raw.json" >/dev/null
    done
    if [[ $expected == 0 ]]; then jq -e '.root_table==false and .method=="ic"' "$run/original-ic.raw.json" >/dev/null; fi
    (cd "$run" && sha256sum -c pre-workers.sha256) > "$run/recheck.log"
  fi
  prior=$(sha256sum "$run/flow.jsonl")
  if bash "$run/wrapper" --execute "$run"; then echo 'repeat accepted' >&2; exit 1; fi
  [[ $(sha256sum "$run/flow.jsonl") == "$prior" ]]
done
printf 'PASS: eight-arm positive/first-failure continuation, exact methods/root/pair/edge caps and all four rho configurations, public-hash refusal, atomic repeat refusal, immutable flow. Simulated transport only. Custody: %s\n' "$scratch"
