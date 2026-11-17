#!/usr/bin/env bash
set -Eeuo pipefail
multiset_parent=$PPID
for multiset_depth in {1..16}; do
  test "$multiset_parent" -gt 1 || break
  test -r "/proc/$multiset_parent/cmdline" || break
  mapfile -d '' -t multiset_parent_args < "/proc/$multiset_parent/cmdline"
  for ((multiset_arg=0; multiset_arg+1<${#multiset_parent_args[@]}; multiset_arg++)); do
    if [[ ${multiset_parent_args[multiset_arg]} == */isolated_bench.py && ${multiset_parent_args[multiset_arg+1]} == busy ]]; then
      printf 'Multiset driver owns per-phase reservations; invoke directly.\n' >&2
      exit 64
    fi
  done
  multiset_parent=$(ps -o ppid= -p "$multiset_parent" 2>/dev/null | tr -d ' ') || break
done
multiset_root=research/twist_index_calculus_20261020/zero_fiber_collector/multiset_root_audit
if test "${1:-}" != --frozen; then
  multiset_out=${1:?fresh output directory}
  multiset_gate=${2:?passing native gate directory}
  test "$(cat "$multiset_gate/metadata/terminal-exit")" = 0
  sha256sum -c "$multiset_gate/metadata/sources.sha256"
  sha256sum -c "$multiset_gate/metadata/binary.sha256"
  test ! -e "$multiset_out"
  mkdir -p "$multiset_out/metadata" "$multiset_out/cases"
  multiset_out=$(realpath "$multiset_out")
  multiset_gate=$(realpath "$multiset_gate")
  git rev-parse HEAD > "$multiset_out/metadata/source-commit.txt"
  sha256sum "$multiset_root/native/Cargo.toml" "$multiset_root/native/Cargo.lock" "$multiset_root/native/src/main.rs" "$multiset_root/native/src/fixture.rs" "$multiset_root/PROTOCOL.md" "$multiset_root/run-panel.sh" "$multiset_root/run-phase.sh" > "$multiset_out/metadata/sources.sha256"
  cp "$multiset_root/run-panel.sh" "$multiset_out/metadata/executed-script.sh"
  cp "$multiset_root/run-phase.sh" "$multiset_out/metadata/run-phase.sh"
  cp research/twist_index_calculus_20261020/zero_fiber_collector/collision_certificate/workload-v1.json "$multiset_out/metadata/workload.json"
  cp "$multiset_gate/runner" "$multiset_out/runner"
  sha256sum "$multiset_out/metadata/executed-script.sh" "$multiset_out/metadata/run-phase.sh" "$multiset_out/metadata/workload.json" "$multiset_out/runner" > "$multiset_out/metadata/frozen-execution.sha256"
  exec bash "$multiset_out/metadata/executed-script.sh" --frozen "$multiset_out" "$multiset_gate"
fi
multiset_out=${2:?frozen output}
multiset_gate=${3:?original native gate}
test ! -e "$multiset_out/metadata/started-utc.txt"
trap 'multiset_rc=$?; date -u +%FT%TZ > "$multiset_out/metadata/finished-utc.txt"; printf "%s\n" "$multiset_rc" > "$multiset_out/metadata/terminal-exit"' EXIT
date -u +%FT%TZ > "$multiset_out/metadata/started-utc.txt"
printf '%s\n' "$$" > "$multiset_out/metadata/owner-pid.txt"
printf '%s\n' 'One science worker; common reservation per phase; 8 GiB AS; outer 1200s; cooperative decode/audit 1180s; queue wait separate; diagnostic clocks only' > "$multiset_out/metadata/resource-envelope.txt"
export LC_ALL=C RAYON_NUM_THREADS=1
ulimit -c 0
sha256sum -c "$multiset_out/metadata/sources.sha256" > "$multiset_out/metadata/source-check-before.txt"
sha256sum -c "$multiset_out/metadata/frozen-execution.sha256" > "$multiset_out/metadata/execution-check-before.txt"
jq -r '.workloads[].id' "$multiset_out/metadata/workload.json" > "$multiset_out/metadata/ids.txt"
multiset_phase() {
  local label=$1
  shift
  local prefix="$multiset_out/metadata/$label"
  if test -e "$prefix.exit" || test -e "$prefix.command.txt"; then
    printf 'Refusing existing phase %s\n' "$label" >&2
    exit 65
  fi
  date -u +%FT%TZ > "$prefix.queued-utc.txt"
  local command=(python3 tools/isolated_bench.py busy -- bash "$multiset_out/metadata/run-phase.sh" "$prefix" "$@")
  printf '%q ' "${command[@]}" > "$prefix.command.txt"
  if "${command[@]}" > "$prefix.stdout" 2> "$prefix.stderr"; then multiset_last=0; else multiset_last=$?; fi
  printf '%s\n' "$multiset_last" > "$prefix.exit"
  date -u +%FT%TZ > "$prefix.finished-utc.txt"
  printf '%s %s\n' "$label" "$multiset_last"
}
while IFS= read -r id; do
  multiset_case="$multiset_out/cases/$id"
  mkdir -p "$multiset_case"
  multiset_phase "$id-first" "$multiset_out/runner" decode "$multiset_out/metadata/workload.json" "$id" "$multiset_case/first"
  multiset_first=$multiset_last
  if test "$multiset_first" = 0; then
    multiset_phase "$id-audit" "$multiset_out/runner" audit "$multiset_out/metadata/workload.json" "$id" "$multiset_case/first" "$multiset_case/audit"
  else
    printf 'not-run-first-failed\n' > "$multiset_out/metadata/$id-audit.exit"
  fi
  multiset_phase "$id-aa" "$multiset_out/runner" decode "$multiset_out/metadata/workload.json" "$id" "$multiset_case/aa"
  if test "$multiset_first" = 0 && test "$multiset_last" = 0; then
    multiset_phase "$id-parity" "$multiset_out/runner" parity "$multiset_case/first" "$multiset_case/aa" "$multiset_case/parity.json"
  else
    printf 'not-run-decode-failed\n' > "$multiset_out/metadata/$id-parity.exit"
  fi
done < "$multiset_out/metadata/ids.txt"
multiset_phase summarize "$multiset_out/runner" summarize "$multiset_out/metadata/workload.json" "$multiset_out" "$multiset_out/analysis"
multiset_summary=$multiset_last
sha256sum -c "$multiset_out/metadata/sources.sha256" > "$multiset_out/metadata/source-check-after.txt"
sha256sum -c "$multiset_out/metadata/frozen-execution.sha256" > "$multiset_out/metadata/execution-check-after.txt"
exit "$multiset_summary"
