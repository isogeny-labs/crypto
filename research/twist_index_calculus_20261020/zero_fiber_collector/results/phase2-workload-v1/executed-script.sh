#!/usr/bin/env bash
# Thin invocation of the native metadata tool; run through isolated_bench.py busy.
set -euo pipefail
prepare_out=${1:?fresh preparation directory}
prepare_build=${2:?successful native metadata build}
prepare_gate=${3:?successful native pipeline gate}
prepare_study=research/twist_index_calculus_20261020/zero_fiber_collector
test ! -e "$prepare_out"
test "$(cat "$prepare_build/metadata/exit")" = 0
test "$(cat "$prepare_gate/metadata/exit")" = 0
mkdir -p "$prepare_out"
cp "$0" "$prepare_out/executed-script.sh"
sha256sum -c "$prepare_build/metadata/panel.sha256" > "$prepare_out/panel-verified.txt"
sha256sum -c "$prepare_build/metadata/sources.sha256" > "$prepare_out/panel-sources-verified.txt"
date -u +%FT%TZ > "$prepare_out/started-utc.txt"
prepare_command=(prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s
  "$prepare_build/panel" prepare
  "$prepare_study/results/phase2-baseline-v1.json"
  "$prepare_study/runs/panel-v2/inputs"
  "$prepare_study/runs/panel-v2/cases"
  /home/ec2-user/.cache/ephemeral/runs/20261023-041850-twist-image-nine-ref-0cbad7/out/reference-first
  /home/ec2-user/.cache/ephemeral/runs/20261023-052011-twist-image-doubling-e8f20f/out/reference-first
  "$prepare_study/phase2/workload-v1.json")
printf '%q ' "${prepare_command[@]}" > "$prepare_out/prepare.command.txt"
prepare_rc=0
"${prepare_command[@]}" > "$prepare_out/prepare.stdout" 2> "$prepare_out/prepare.stderr" || prepare_rc=$?
printf '%s\n' "$prepare_rc" > "$prepare_out/prepare.exit"
test "$prepare_rc" = 0
sha256sum "$prepare_study/phase2/workload-v1.json" "$prepare_study/phase2/workload-v1.tsv" "$prepare_study/phase2/workload-v1.sha256" > "$prepare_out/workload.sha256"
for prepare_mode in zero plain; do
  for prepare_arm in phi-l0-active double-l0-active; do
    prepare_record="$prepare_gate/test-artifacts/p37-roundtrip/$prepare_mode-$prepare_arm/record.json"
    printf '%q ' "$prepare_build/panel" inspect-record "$prepare_record" > "$prepare_out/inspect-$prepare_mode-$prepare_arm.command.txt"
    "$prepare_build/panel" inspect-record "$prepare_record" > "$prepare_out/inspect-$prepare_mode-$prepare_arm.json"
  done
done
date -u +%FT%TZ > "$prepare_out/finished-utc.txt"
printf '0\n' > "$prepare_out/exit"
