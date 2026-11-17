#!/usr/bin/env bash
# One owner and per-phase common reservations. No phase retries.
set -Eeuo pipefail
probe_parent=$PPID
for probe_depth in {1..16}; do
  test "$probe_parent" -gt 1 || break
  test -r "/proc/$probe_parent/cmdline" || break
  mapfile -d '' -t probe_parent_args < "/proc/$probe_parent/cmdline"
  for ((probe_arg=0; probe_arg+1<${#probe_parent_args[@]}; probe_arg++)); do
    if [[ ${probe_parent_args[probe_arg]} == */isolated_bench.py && ${probe_parent_args[probe_arg+1]} == busy ]]; then
      printf 'Probe driver owns per-phase reservations; invoke directly.\n' >&2
      exit 64
    fi
  done
  probe_parent=$(ps -o ppid= -p "$probe_parent" 2>/dev/null | tr -d ' ') || break
done
if test "${1:-}" != --frozen; then
  probe_out=${1:?fresh output}
  test ! -e "$probe_out"
  mkdir -p "$probe_out/metadata" "$probe_out/cases"
  probe_out=$(cd "$probe_out" && pwd)
  cp "$0" "$probe_out/metadata/executed-script.sh"
  exec bash "$probe_out/metadata/executed-script.sh" --frozen "$probe_out" "${@:2}"
fi
shift
probe_out=$1
probe_gate=$(cd "${2:?successful implementation gate}" && pwd)
probe_prepared=$(cd "${3:?successful preparation}" && pwd)
probe_sha=${4:?frozen workload SHA256}
probe_repo=$(git rev-parse --show-toplevel)
probe_root=$probe_repo/research/twist_index_calculus_20261020/zero_fiber_collector/degree_closure_probe
test ! -e "$probe_out/metadata/started-utc.txt"
test "$(cat "$probe_gate/metadata/exit")" = 0
test "$(cat "$probe_prepared/metadata/exit")" = 0
sha256sum -c "$probe_gate/metadata/sources.sha256" > "$probe_out/metadata/gate-sources-unchanged.txt"
sha256sum -c "$probe_gate/metadata/probe.sha256" > "$probe_out/metadata/gate-binary-unchanged.txt"
test "$(sha256sum "$probe_prepared/prepared/workload.json" | cut -d ' ' -f 1)" = "$probe_sha"
cp "$probe_gate/probe" "$probe_out/runner"
chmod 500 "$probe_out/runner"
cp "$probe_gate/metadata/sources.sha256" "$probe_out/metadata/sources.sha256"
cp "$probe_prepared/metadata/workload-and-inputs.sha256" "$probe_out/metadata/workload-and-inputs.sha256"
cp "$probe_prepared/prepared/workload.json" "$probe_out/metadata/workload.json"
cp "$probe_prepared/prepared/workload.tsv" "$probe_out/metadata/workload.tsv"
cp "$probe_root/run-phase.sh" "$probe_out/metadata/run-phase.sh"
date -u +%FT%TZ > "$probe_out/metadata/started-utc.txt"
git rev-parse HEAD > "$probe_out/metadata/source-commit.txt"
printf '%s\n' "$$" > "$probe_out/metadata/owner-pid.txt"
printf '%s\n' '6 fixed systems, 12 configurations, 24 producers; 12 first audits, 12 parities. One worker, common lock, 8 GiB AS, 1200-second outer and 1180-second cooperative producer deadline per phase. Diagnostic clocks only.' > "$probe_out/metadata/resource-envelope.txt"
sha256sum "$probe_out/runner" "$probe_out/metadata/workload.json" "$probe_out/metadata/workload.tsv" "$probe_out/metadata/executed-script.sh" "$probe_out/metadata/run-phase.sh" > "$probe_out/metadata/frozen-execution.sha256"
export LC_ALL=C RAYON_NUM_THREADS=1
ulimit -c 0
probe_phase() {
  local probe_tag=$1 probe_rc
  shift
  if test -e "$probe_out/metadata/$probe_tag.command.txt"; then
    printf 'Refusing existing phase %s\n' "$probe_tag" >&2
    exit 65
  fi
  printf '%q ' "$@" > "$probe_out/metadata/$probe_tag.command.txt"
  date -u +%FT%TZ > "$probe_out/metadata/$probe_tag.queued-utc.txt"
  if "$@" > "$probe_out/metadata/$probe_tag.stdout" 2> "$probe_out/metadata/$probe_tag.stderr"; then probe_rc=0; else probe_rc=$?; fi
  printf '%s\n' "$probe_rc" > "$probe_out/metadata/$probe_tag.exit"
  date -u +%FT%TZ > "$probe_out/metadata/$probe_tag.finished-utc.txt"
  printf '%s exited %s\n' "$probe_tag" "$probe_rc"
  return "$probe_rc"
}
probe_phase input-seals python3 "$probe_repo/tools/isolated_bench.py" busy -- sha256sum -c "$probe_out/metadata/workload-and-inputs.sha256"
while IFS=$'\t' read -r probe_id probe_mode probe_input probe_input_sha; do
  probe_case="$probe_out/cases/$probe_id"
  mkdir -p "$probe_case"
  probe_phase "$probe_id-first" python3 "$probe_repo/tools/isolated_bench.py" busy -- \
    bash "$probe_out/metadata/run-phase.sh" "$probe_out/metadata/$probe_id-first" \
    "$probe_out/runner" produce "$probe_input" "$probe_input_sha" "$probe_mode" "$probe_case/first" || true
  if test -f "$probe_case/first/record.json"; then
    probe_phase "$probe_id-audit" python3 "$probe_repo/tools/isolated_bench.py" busy -- \
      bash "$probe_out/metadata/run-phase.sh" "$probe_out/metadata/$probe_id-audit" \
      "$probe_out/runner" audit "$probe_out/metadata/workload.json" "$probe_sha" "$probe_id" "$probe_case/first" "$probe_case/audit" || true
  fi
  probe_phase "$probe_id-aa" python3 "$probe_repo/tools/isolated_bench.py" busy -- \
    bash "$probe_out/metadata/run-phase.sh" "$probe_out/metadata/$probe_id-aa" \
    "$probe_out/runner" produce "$probe_input" "$probe_input_sha" "$probe_mode" "$probe_case/aa" || true
  if test -f "$probe_case/first/record.json" && test -f "$probe_case/aa/record.json"; then
    probe_phase "$probe_id-parity" python3 "$probe_repo/tools/isolated_bench.py" busy -- \
      bash "$probe_out/metadata/run-phase.sh" "$probe_out/metadata/$probe_id-parity" \
      "$probe_out/runner" parity "$probe_case/first/record.json" "$probe_case/aa/record.json" "$probe_case/parity.json" || true
  fi
done < "$probe_out/metadata/workload.tsv"
probe_terminal=0
probe_phase summarize python3 "$probe_repo/tools/isolated_bench.py" busy -- \
  bash "$probe_out/metadata/run-phase.sh" "$probe_out/metadata/summarize" \
  "$probe_out/runner" summarize "$probe_out" "$probe_out/metadata/workload.json" "$probe_sha" "$probe_out/analysis" || probe_terminal=$?
sha256sum -c "$probe_out/metadata/sources.sha256" > "$probe_out/metadata/sources-unchanged-after.txt"
probe_phase final-input-seals python3 "$probe_repo/tools/isolated_bench.py" busy -- sha256sum -c "$probe_out/metadata/workload-and-inputs.sha256" || probe_terminal=$?
sha256sum -c "$probe_out/metadata/frozen-execution.sha256" > "$probe_out/metadata/frozen-execution-unchanged-after.txt"
date -u +%FT%TZ > "$probe_out/metadata/finished-utc.txt"
printf '%s\n' "$probe_terminal" > "$probe_out/metadata/terminal-exit"
exit "$probe_terminal"
