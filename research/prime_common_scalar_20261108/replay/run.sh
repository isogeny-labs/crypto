#!/usr/bin/env bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/prime_common_scalar_20261108/replay
checker=${1:?usage: run.sh CHECKER ANALYZER NATIVE_CONTRACT}
analyzer=${2:?usage: run.sh CHECKER ANALYZER NATIVE_CONTRACT}
contract=${3:?usage: run.sh CHECKER ANALYZER NATIVE_CONTRACT}
branch=$(git branch --show-current)
checkpoint() {
  git add "$study/runs"
  if [ -e "$study/summary.json" ]; then git add "$study/summary.json"; fi
  if ! git diff --cached --quiet; then git commit -m "$1"; fi
  git push origin "HEAD:refs/heads/$branch"
  local published
  published=$(git ls-remote origin "refs/heads/$branch" | cut -f1)
  test "$published" = "$(git rev-parse HEAD)"
}
"$contract" schema /workspace/crypto-autoresearcher/schemas/experiment-protocol.v1.json "$study/protocol.json"
jq -e '.version==2 and .freeze.status=="frozen" and .freeze.filed_before_affected_data==true' "$study/protocol.json" >/dev/null
expected=$(jq -r '.freeze.protocol_sha256' "$study/protocol.json")
actual=$(jq -cS 'del(.freeze)' "$study/protocol.json" | tr -d '\n' | sha256sum | cut -d' ' -f1)
test "$actual" = "$expected"
jq -e '.checkpoint_backend=="git-origin" and .max_census_invocations==2 and .retries==0 and .timeout_seconds_each==600 and .memory_limit_bytes==2147483648 and .cpus==1 and .cloud_spend==false' "$study/checkpoint-authorization.json" >/dev/null
for entry in checker analyzer; do
  if [ "$entry" = checker ]; then artifact=$checker; source=check.cpp; else artifact=$analyzer; source=analyze.cpp; fi
  actual=$(sha256sum "$artifact" | cut -d' ' -f1)
  expected=$(jq -r --arg key "$entry" '.native_artifacts[$key].sha256' "$study/instrument.json")
  test "$actual" = "$expected"
  sourcecommit=$(jq -r '.source_commit' "$study/instrument.json")
  expected=$(git show "$sourcecommit:$study/$source" | sha256sum | cut -d' ' -f1)
  actual=$(sha256sum "$study/$source" | cut -d' ' -f1)
  test "$actual" = "$expected"
done
git diff --quiet
git diff --cached --quiet
test -z "$(git ls-files --others --exclude-standard "$study")"
published=$(git ls-remote origin "refs/heads/$branch" | cut -f1)
test "$published" = "$(git rev-parse HEAD)"
test ! -e "$study/runs"
mkdir -p "$study/runs"
uname -a > "$study/runs/host.txt"
lscpu >> "$study/runs/host.txt"
cat /proc/loadavg >> "$study/runs/host.txt"
cpulist=$(awk '/Cpus_allowed_list:/ {print $2}' /proc/self/status)
cpuid=${cpulist%%[-,]*}
for runid in 0 1; do
  mkdir "$study/runs/$runid"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$study/runs/$runid/started-at.txt"
  printf '%s\n' "$cpuid" > "$study/runs/$runid/cpu-affinity.txt"
  checkpoint "Checkpoint native recovery invocation $runid before launch"
  set +e
  (ulimit -v 2097152; ulimit -f 32768; exec timeout --signal=KILL 600 taskset -c "$cpuid" "$checker") > "$study/runs/$runid/panel.tsv" 2> "$study/runs/$runid/stderr.log"
  status=$?
  set -e
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$study/runs/$runid/finished-at.txt"
  printf '%s\n' "$status" > "$study/runs/$runid/exit-code.txt"
  checkpoint "Checkpoint native recovery invocation $runid raw outcome"
  test "$status" -eq 0
  test "$(wc -l < "$study/runs/$runid/panel.tsv")" -eq 184
done
set +e
(ulimit -v 2097152; exec timeout --signal=KILL 60 taskset -c "$cpuid" "$analyzer" "$study/runs/0/panel.tsv" "$study/runs/1/panel.tsv") > "$study/summary.json" 2> "$study/runs/analysis-stderr.log"
status=$?
set -e
printf '%s\n' "$status" > "$study/runs/analysis-exit-code.txt"
checkpoint 'Checkpoint native recovery exact recount outcome'
exit "$status"
