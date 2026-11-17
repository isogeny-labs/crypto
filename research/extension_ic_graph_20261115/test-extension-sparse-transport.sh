#!/usr/bin/env bash
# Process/custody simulation only; no scientific worker is executed locally.
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
scratch=$(mktemp -d /tmp/ic-sparse46-transport.XXXXXX)
cpu=$(awk '/^Cpus_allowed_list:/{split($2,a,/[,-]/);print a[1]}' /proc/self/status)
fixture=/workspace/ic-runpod-runs/runpod-extension-pipeline-20261116-41/returned/task/run/fixture.json
for case_name in positive first-failure bad-input-hash; do
  run=$scratch/$case_name
  mkdir "$run"
  cp "$here/run-extension-sparse-task.sh" "$run/wrapper"
  cp "$fixture" "$run/control-fixture.json"
  jq --arg cpu "$cpu" '. + {pinned_cpu:$cpu,source_revision:"1111111111111111111111111111111111111111"}' "$here/extension-sparse-plan.json" > "$run/runtime-plan.json"
  if [[ $case_name == bad-input-hash ]]; then jq '.public_sha256="bad"' "$run/runtime-plan.json" > "$run/changed-plan.json"; cp "$run/changed-plan.json" "$run/runtime-plan.json"; fi
  cat > "$run/driver" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
set -C
generate=0
method=unset
source=unset
out=unset
while [[ $# -gt 0 ]]; do
  case $1 in
    --generate) generate=1; shift ;;
    --method) method=$2; shift 2 ;;
    --source-revision) source=$2; shift 2 ;;
    --out) out=$2; shift 2 ;;
    *) shift ;;
  esac
done
if [[ $generate == 1 ]]; then jq --arg source "$source" '.source_revision=$source' "$(dirname "$0")/control-fixture.json" > "$out"; exit 0; fi
if [[ $method == ic && -e $(dirname "$0")/simulate-first-failure ]]; then exit 17; fi
printf '{"simulated_transport":true,"method":"%s"}\n' "$method" > "$out"
SH
  chmod +x "$run/driver"
  [[ $case_name != first-failure ]] || touch "$run/simulate-first-failure"
  if [[ $case_name == bad-input-hash ]]; then
    if bash "$run/wrapper" --execute "$run"; then echo 'changed public hash accepted' >&2; exit 1; fi
    [[ ! -e $run/ic.exit-code.txt && $(wc -l < "$run/flow.jsonl") == 2 ]]
  else
    bash "$run/wrapper" --execute "$run"
    [[ $(wc -l < "$run/flow.jsonl") == 20 ]]
    expected=0
    [[ $case_name != first-failure ]] || expected=17
    [[ $(cat "$run/ic.exit-code.txt") == "$expected" ]]
    for mode in extension-full-smooth extension-remainders extension-sparse-full extension-sparse-remainders rho-plain-1 rho-negation-1 rho-plain-16 rho-negation-16; do [[ $(cat "$run/$mode.exit-code.txt") == 0 ]]; done
    (cd "$run" && sha256sum -c pre-workers.sha256) > "$run/recheck.log"
  fi
  prior=$(sha256sum "$run/flow.jsonl")
  if bash "$run/wrapper" --execute "$run"; then echo 'repeat accepted' >&2; exit 1; fi
  [[ $(sha256sum "$run/flow.jsonl") == "$prior" ]]
done
printf 'PASS: nine-arm positive/first-failure continuation, public-hash refusal, atomic repeat refusal, immutable flow. Simulated transport only. Custody: %s\n' "$scratch"
