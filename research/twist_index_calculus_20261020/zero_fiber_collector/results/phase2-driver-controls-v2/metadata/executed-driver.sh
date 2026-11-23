#!/usr/bin/env bash
# Thin execution of the prospectively frozen Rust workload. One science worker.
set -euo pipefail
pipeline_parent=$PPID
for pipeline_depth in {1..16}; do
  test "$pipeline_parent" -gt 1 || break
  test -r "/proc/$pipeline_parent/cmdline" || break
  mapfile -d '' -t pipeline_parent_args < "/proc/$pipeline_parent/cmdline"
  for ((pipeline_arg=0; pipeline_arg+1<${#pipeline_parent_args[@]}; pipeline_arg++)); do
    if [[ ${pipeline_parent_args[pipeline_arg]} == */isolated_bench.py && ${pipeline_parent_args[pipeline_arg+1]} == busy ]]; then
      printf 'Phase 2 driver owns per-phase reservations; invoke directly.\n' >&2
      exit 64
    fi
  done
  pipeline_parent=$(ps -o ppid= -p "$pipeline_parent" 2>/dev/null | tr -d ' ') || break
done
if test "${1:-}" != --frozen; then
  pipeline_out=${1:?fresh output directory}
  test ! -e "$pipeline_out"
  mkdir -p "$pipeline_out/metadata" "$pipeline_out/cases"
  pipeline_out=$(cd "$pipeline_out" && pwd)
  cp "$0" "$pipeline_out/metadata/executed-script.sh"
  exec bash "$pipeline_out/metadata/executed-script.sh" --frozen "$pipeline_out" "${@:2}"
fi
shift
pipeline_out=$1
test ! -e "$pipeline_out/metadata/started-utc.txt"
test ! -e "$pipeline_out/runner"
pipeline_workload=${2:?frozen workload JSON}
pipeline_gate=$(cd "${3:?successful current native gate}" && pwd)
pipeline_meta=$(cd "${4:?successful native workload/reducer build directory}" && pwd)
pipeline_repo=$(git rev-parse --show-toplevel)
pipeline_study="$pipeline_repo/research/twist_index_calculus_20261020/zero_fiber_collector"
test "$(cat "$pipeline_gate/metadata/exit")" = 0
test "$(cat "$pipeline_meta/metadata/exit")" = 0
sha256sum -c "$pipeline_gate/metadata/sources.sha256" > "$pipeline_out/metadata/gate-sources-unchanged-before.txt"
sha256sum -c "$pipeline_meta/metadata/sources.sha256" > "$pipeline_out/metadata/panel-sources-unchanged-before.txt"
sha256sum -c "$pipeline_meta/metadata/panel.sha256" > "$pipeline_out/metadata/panel-binary-verified-before.txt"
cp "$pipeline_gate/bin/study" "$pipeline_out/runner"
cp "$pipeline_meta/panel" "$pipeline_out/panel"
chmod 500 "$pipeline_out/runner" "$pipeline_out/panel"
cp "$pipeline_workload" "$pipeline_out/metadata/workload.json"
cp "${pipeline_workload%.json}.tsv" "$pipeline_out/metadata/workload.tsv"
cp "${pipeline_workload%.json}.sha256" "$pipeline_out/metadata/public-reference-inputs.sha256"
cp "$pipeline_gate/metadata/sources.sha256" "$pipeline_out/metadata/compiled-sources.sha256"
cp "$pipeline_meta/metadata/sources.sha256" "$pipeline_out/metadata/panel-sources.sha256"
cp "$pipeline_study/PROTOCOL.md" "$pipeline_out/metadata/protocol.md"
cp "$pipeline_study/PHASE2_EXECUTION.md" "$pipeline_out/metadata/execution-protocol.md"
cp "$pipeline_study/results/phase2-baseline-v1.json" "$pipeline_out/metadata/baseline.json"
sha256sum "$pipeline_out/runner" "$pipeline_out/panel" "$pipeline_out/metadata/workload.json" "$pipeline_out/metadata/workload.tsv" "$pipeline_out/metadata/public-reference-inputs.sha256" "$pipeline_out/metadata/protocol.md" "$pipeline_out/metadata/execution-protocol.md" "$pipeline_out/metadata/baseline.json" "$pipeline_out/metadata/executed-script.sh" > "$pipeline_out/metadata/frozen-runner-workload.sha256"
git rev-parse HEAD > "$pipeline_out/metadata/source-commit.txt"
date -u +%FT%TZ > "$pipeline_out/metadata/started-utc.txt"
printf '%s\n' "$$" > "$pipeline_out/metadata/owner-pid.txt"
export RAYON_NUM_THREADS=1 LC_ALL=C
ulimit -c 0
printf '%s\n' 'One science worker; 8 GiB address-space cap; 1200-second watchdog per phase; collector cooperative deadline 1180 seconds; shared-host counts/diagnostics only; no timing admission.' > "$pipeline_out/metadata/resource-envelope.txt"
pipeline_phase() {
  local pipeline_tag=$1 pipeline_rc
  shift
  # Never automatically repeat a terminal or interrupted phase.
  if test -e "$pipeline_out/metadata/$pipeline_tag.command.txt"; then
    printf 'Refusing existing phase %s\n' "$pipeline_tag" >&2
    return 65
  fi
  printf '%q ' "$@" > "$pipeline_out/metadata/$pipeline_tag.command.txt"
  date -u +%FT%TZ > "$pipeline_out/metadata/$pipeline_tag.started-utc.txt"
  if "$@" > "$pipeline_out/metadata/$pipeline_tag.stdout" 2> "$pipeline_out/metadata/$pipeline_tag.stderr"; then
    pipeline_rc=0
  else
    pipeline_rc=$?
  fi
  printf '%s\n' "$pipeline_rc" > "$pipeline_out/metadata/$pipeline_tag.exit"
  date -u +%FT%TZ > "$pipeline_out/metadata/$pipeline_tag.finished-utc.txt"
  printf '%s exited %s\n' "$pipeline_tag" "$pipeline_rc"
  return "$pipeline_rc"
}
pipeline_phase input-seals python3 "$pipeline_repo/tools/isolated_bench.py" busy -- \
  sha256sum -c "$pipeline_out/metadata/public-reference-inputs.sha256"
while IFS=$'\t' read -r pipeline_id pipeline_p pipeline_seed pipeline_mode pipeline_arm pipeline_public pipeline_sha pipeline_zero pipeline_zero_sha pipeline_target pipeline_target_sha; do
  pipeline_case="$pipeline_out/cases/$pipeline_id"
  mkdir -p "$pipeline_case"
  for pipeline_pass in first aa1 aa2; do
    pipeline_tag="$pipeline_id-$pipeline_pass-collect"
    pipeline_phase "$pipeline_tag" python3 "$pipeline_repo/tools/isolated_bench.py" busy -- \
      /usr/bin/time -v -o "$pipeline_out/metadata/$pipeline_tag.time.txt" \
      prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s \
      "$pipeline_out/runner" --zero-pipeline "$pipeline_public" "$pipeline_sha" "$pipeline_arm" "$pipeline_mode" "$pipeline_case/$pipeline_pass" || true
    if test "$pipeline_pass" = first; then
      if test -f "$pipeline_case/first/record.json"; then
        pipeline_phase "$pipeline_id-audit" python3 "$pipeline_repo/tools/isolated_bench.py" busy -- \
          /usr/bin/time -v -o "$pipeline_out/metadata/$pipeline_id-audit.time.txt" \
          prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s \
          "$pipeline_out/runner" --zero-pipeline-audit "$pipeline_public" "$pipeline_sha" "$pipeline_case/first" \
          "$pipeline_zero" "$pipeline_zero_sha" "$pipeline_target" "$pipeline_target_sha" "$pipeline_case/audit" || true
      fi
    elif test -f "$pipeline_case/first/record.json" && test -f "$pipeline_case/$pipeline_pass/record.json"; then
      pipeline_phase "$pipeline_id-$pipeline_pass-parity" python3 "$pipeline_repo/tools/isolated_bench.py" busy -- \
        prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s \
        "$pipeline_out/runner" --zero-pipeline-parity "$pipeline_case/first" "$pipeline_case/$pipeline_pass" "$pipeline_case/$pipeline_pass-parity.json" || true
    fi
  done
done < "$pipeline_out/metadata/workload.tsv"
pipeline_terminal=0
pipeline_phase summarize python3 "$pipeline_repo/tools/isolated_bench.py" busy -- \
  prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s \
  "$pipeline_out/panel" summarize "$pipeline_out" "$pipeline_out/analysis" || pipeline_terminal=$?
pipeline_phase final-input-seals python3 "$pipeline_repo/tools/isolated_bench.py" busy -- \
  sha256sum -c "$pipeline_out/metadata/public-reference-inputs.sha256" || pipeline_terminal=$?
sha256sum -c "$pipeline_out/metadata/frozen-runner-workload.sha256" > "$pipeline_out/metadata/frozen-runner-workload-unchanged.txt"
sha256sum -c "$pipeline_out/metadata/compiled-sources.sha256" > "$pipeline_out/metadata/compiled-sources-unchanged.txt"
sha256sum -c "$pipeline_out/metadata/panel-sources.sha256" > "$pipeline_out/metadata/panel-sources-unchanged.txt"
date -u +%FT%TZ > "$pipeline_out/metadata/finished-utc.txt"
printf '%s\n' "$pipeline_terminal" > "$pipeline_out/metadata/terminal-exit"
exit "$pipeline_terminal"
