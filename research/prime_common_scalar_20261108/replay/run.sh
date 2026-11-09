#!/usr/bin/env bash
set -euo pipefail
# No argument silently opts out of the repository checkpoint gate.
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/prime_common_scalar_20261108/replay
checker=${1:?usage: run.sh CHECKER ANALYZER}
analyzer=${2:?usage: run.sh CHECKER ANALYZER}
contract=${3:?usage: run.sh CHECKER ANALYZER NATIVE_CONTRACT}
"$contract" schema /workspace/crypto-autoresearcher/schemas/experiment-protocol.v1.json "$study/protocol.json"
for entry in checker analyzer; do
  if [ "$entry" = checker ]; then artifact=$checker; else artifact=$analyzer; fi
  actual=$(sha256sum "$artifact" | cut -d' ' -f1)
  expected=$(jq -r --arg key "$entry" '.native_artifacts[$key].sha256' "$study/instrument.json")
  test "$actual" = "$expected"
done
bash scripts/checkpoint.sh gate
# A completed or partial invocation is never overwritten/retried.
test ! -e "$study/runs"
mkdir -p "$study/runs"
uname -a > "$study/runs/host.txt"
lscpu >> "$study/runs/host.txt"
cat /proc/loadavg >> "$study/runs/host.txt"
cpulist=$(awk '/Cpus_allowed_list:/ {print $2}' /proc/self/status)
cpuid=${cpulist%%[-,]*}
for runid in 0 1; do
  mkdir "$study/runs/$runid"
  set +e
  (ulimit -v 2097152; ulimit -f 32768; exec timeout --signal=KILL 600 taskset -c "$cpuid" "$checker") > "$study/runs/$runid/panel.tsv" 2> "$study/runs/$runid/stderr.log"
  status=$?
  set -e
  printf '%s\n' "$status" > "$study/runs/$runid/exit-code.txt"
  test "$status" -eq 0
  test "$(wc -l < "$study/runs/$runid/panel.tsv")" -eq 184
  bash scripts/checkpoint.sh gate
done
"$analyzer" "$study/runs/0/panel.tsv" "$study/runs/1/panel.tsv" > "$study/summary.json"
