#!/usr/bin/env bash
# Simulated transport control only: never invokes cryptographic worker code.
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
scratch=$(mktemp -d /tmp/ic-anchored42-transport.XXXXXX)
cpu=$(awk '/^Cpus_allowed_list:/{split($2,a,/[,-]/);print a[1]}' /proc/self/status)
for case_name in positive first-failure; do
  run=$scratch/$case_name
  mkdir "$run"
  cp "$here/run-anchored-descent-task.sh" "$run/wrapper"
  jq --arg cpu "$cpu" '. + {pinned_cpu:$cpu,source_revision:"1111111111111111111111111111111111111111"}' "$here/anchored-descent-plan.json" > "$run/runtime-plan.json"
  printf '{}\n' > "$run/input.json"
  cat > "$run/driver" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
set -C
[[ $# == 3 ]]
if [[ $1 == original-nagao && -e $(dirname "$0")/simulate-first-failure ]]; then exit 17; fi
printf '{"simulated_transport":true,"mode":"%s"}\n' "$1" > "$3"
SH
  chmod +x "$run/driver"
  if [[ $case_name == first-failure ]]; then touch "$run/simulate-first-failure"; fi
  bash "$run/wrapper" --execute "$run"
  [[ $(wc -l < "$run/flow.jsonl") == 6 ]]
  [[ $(cat "$run/full-only.exit-code.txt") == 0 && $(cat "$run/anchored.exit-code.txt") == 0 ]]
  if [[ $case_name == positive ]]; then [[ $(cat "$run/original-nagao.exit-code.txt") == 0 ]]; else [[ $(cat "$run/original-nagao.exit-code.txt") == 17 ]]; fi
  (cd "$run" && sha256sum -c pre-workers.sha256) > "$run/manifest-recheck.log"
  prior=$(sha256sum "$run/flow.jsonl")
  if bash "$run/wrapper" --execute "$run"; then echo 'repeated execution accepted' >&2; exit 1; fi
  [[ $(sha256sum "$run/flow.jsonl") == "$prior" ]]
done
printf 'PASS: positive, first-worker failure continuation, all-arm receipts, repeated execution refusal. Simulated transport only. Custody: %s\n' "$scratch"
