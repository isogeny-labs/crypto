#!/usr/bin/env bash
# One managed owner; each science phase uses its own shared reservation.
set -Eeuo pipefail
cert_parent=$PPID
for cert_depth in {1..16}; do
  test "$cert_parent" -gt 1 || break
  test -r "/proc/$cert_parent/cmdline" || break
  mapfile -d '' -t cert_parent_args < "/proc/$cert_parent/cmdline"
  for ((cert_arg=0; cert_arg+1<${#cert_parent_args[@]}; cert_arg++)); do
    if [[ ${cert_parent_args[cert_arg]} == */isolated_bench.py && ${cert_parent_args[cert_arg+1]} == busy ]]; then
      printf 'Certificate driver owns per-phase reservations; invoke directly.\n' >&2
      exit 64
    fi
  done
  cert_parent=$(ps -o ppid= -p "$cert_parent" 2>/dev/null | tr -d ' ') || break
done
if test "${1:-}" != --frozen; then
  cert_out=${1:?fresh output}
  test ! -e "$cert_out"
  mkdir -p "$cert_out/metadata" "$cert_out/cases"
  cert_out=$(cd "$cert_out" && pwd)
  cp "$0" "$cert_out/metadata/executed-script.sh"
  exec bash "$cert_out/metadata/executed-script.sh" --frozen "$cert_out" "${@:2}"
fi
shift
cert_out=$1
cert_gate=$(cd "${2:?successful implementation gate}" && pwd)
cert_prepared=$(cd "${3:?successful prepared workload}" && pwd)
cert_workload_sha=${4:?registered workload SHA256}
cert_repo=$(git rev-parse --show-toplevel)
test ! -e "$cert_out/metadata/started-utc.txt"
test ! -e "$cert_out/runner"
test "$(cat "$cert_gate/metadata/exit")" = 0
test "$(cat "$cert_prepared/metadata/exit")" = 0
sha256sum -c "$cert_gate/metadata/sources.sha256" > "$cert_out/metadata/gate-sources-unchanged-before.txt"
sha256sum -c "$cert_gate/metadata/certificate.sha256" > "$cert_out/metadata/gate-binary-unchanged-before.txt"
test "$(sha256sum "$cert_prepared/prepared/workload.json" | cut -d ' ' -f 1)" = "$cert_workload_sha"
cp "$cert_gate/certificate" "$cert_out/runner"
chmod 500 "$cert_out/runner"
cp "$cert_gate/metadata/sources.sha256" "$cert_out/metadata/sources.sha256"
cp "$cert_prepared/metadata/workload-and-inputs.sha256" "$cert_out/metadata/workload-and-inputs.sha256"
cp "$cert_prepared/prepared/workload.json" "$cert_out/metadata/workload.json"
cp "$cert_prepared/prepared/workload.tsv" "$cert_out/metadata/workload.tsv"
cp "$cert_repo/research/twist_index_calculus_20261020/zero_fiber_collector/collision_certificate/run-phase.sh" "$cert_out/metadata/run-phase.sh"
git rev-parse HEAD > "$cert_out/metadata/source-commit.txt"
date -u +%FT%TZ > "$cert_out/metadata/started-utc.txt"
printf '%s\n' "$$" > "$cert_out/metadata/owner-pid.txt"
printf '%s\n' '42 registered case-arms; first plus one AA repeat (84 producer calls); one worker; each phase has 8 GiB, 1200-second watchdog, producer 1180-second cooperative deadline; counts/diagnostics only.' > "$cert_out/metadata/resource-envelope.txt"
sha256sum "$cert_out/runner" "$cert_out/metadata/workload.json" "$cert_out/metadata/workload.tsv" "$cert_out/metadata/executed-script.sh" "$cert_out/metadata/run-phase.sh" > "$cert_out/metadata/frozen-execution.sha256"
export LC_ALL=C RAYON_NUM_THREADS=1
ulimit -c 0
cert_phase() {
  local cert_tag=$1 cert_rc
  shift
  if test -e "$cert_out/metadata/$cert_tag.command.txt"; then
    printf 'Refusing existing phase %s\n' "$cert_tag" >&2
    exit 65
  fi
  printf '%q ' "$@" > "$cert_out/metadata/$cert_tag.command.txt"
  date -u +%FT%TZ > "$cert_out/metadata/$cert_tag.queued-utc.txt"
  if "$@" > "$cert_out/metadata/$cert_tag.stdout" 2> "$cert_out/metadata/$cert_tag.stderr"; then cert_rc=0; else cert_rc=$?; fi
  printf '%s\n' "$cert_rc" > "$cert_out/metadata/$cert_tag.exit"
  date -u +%FT%TZ > "$cert_out/metadata/$cert_tag.finished-utc.txt"
  printf '%s exited %s\n' "$cert_tag" "$cert_rc"
  return "$cert_rc"
}
cert_phase input-seals python3 "$cert_repo/tools/isolated_bench.py" busy -- \
  sha256sum -c "$cert_out/metadata/workload-and-inputs.sha256"
cert_index=0
while IFS=$'\t' read -r cert_id cert_input cert_sha cert_reference cert_reference_sha cert_audit cert_audit_sha; do
  cert_case="$cert_out/cases/$cert_id"
  mkdir -p "$cert_case"
  cert_phase "$cert_id-first" python3 "$cert_repo/tools/isolated_bench.py" busy -- \
    bash "$cert_out/metadata/run-phase.sh" "$cert_out/metadata/$cert_id-first" \
    "$cert_out/runner" produce "$cert_input" "$cert_sha" "$cert_case/first" || true
  if test -f "$cert_case/first/record.json"; then
    cert_phase "$cert_id-audit" python3 "$cert_repo/tools/isolated_bench.py" busy -- \
      bash "$cert_out/metadata/run-phase.sh" "$cert_out/metadata/$cert_id-audit" \
      "$cert_out/runner" audit "$cert_out/metadata/workload.json" "$cert_workload_sha" "$cert_id" "$cert_case/first" "$cert_case/audit" || true
  fi
  cert_phase "$cert_id-aa" python3 "$cert_repo/tools/isolated_bench.py" busy -- \
    bash "$cert_out/metadata/run-phase.sh" "$cert_out/metadata/$cert_id-aa" \
    "$cert_out/runner" produce "$cert_input" "$cert_sha" "$cert_case/aa" || true
  if test -f "$cert_case/first/record.json" && test -f "$cert_case/aa/record.json"; then
    cert_phase "$cert_id-parity" python3 "$cert_repo/tools/isolated_bench.py" busy -- \
      bash "$cert_out/metadata/run-phase.sh" "$cert_out/metadata/$cert_id-parity" "$cert_out/runner" parity "$cert_case/first" "$cert_case/aa" "$cert_case/parity.json" || true
  fi
  # The first two registered case-arms are also the actual implementation gate.
  # No extra p37 trial is repeated after this gate.
  if test "$cert_index" -lt 2; then
    for cert_phase_name in first audit aa parity; do
      if test ! -f "$cert_out/metadata/$cert_id-$cert_phase_name.exit" || test "$(cat "$cert_out/metadata/$cert_id-$cert_phase_name.exit")" != 0; then
        printf '2\n' > "$cert_out/metadata/terminal-exit"
        printf 'Actual p37 implementation gate did not pass\n' > "$cert_out/metadata/stop-reason.txt"
        exit 2
      fi
    done
  fi
  cert_index=$((cert_index+1))
done < "$cert_out/metadata/workload.tsv"
cert_terminal=0
cert_phase summarize python3 "$cert_repo/tools/isolated_bench.py" busy -- \
  bash "$cert_out/metadata/run-phase.sh" "$cert_out/metadata/summarize" \
  "$cert_out/runner" summarize "$cert_out" "$cert_out/metadata/workload.json" "$cert_workload_sha" "$cert_out/analysis" || cert_terminal=$?
sha256sum -c "$cert_out/metadata/sources.sha256" > "$cert_out/metadata/sources-unchanged-after.txt"
cert_phase final-input-seals python3 "$cert_repo/tools/isolated_bench.py" busy -- \
  sha256sum -c "$cert_out/metadata/workload-and-inputs.sha256" || cert_terminal=$?
sha256sum -c "$cert_out/metadata/frozen-execution.sha256" > "$cert_out/metadata/frozen-execution-unchanged-after.txt"
date -u +%FT%TZ > "$cert_out/metadata/finished-utc.txt"
printf '%s\n' "$cert_terminal" > "$cert_out/metadata/terminal-exit"
exit "$cert_terminal"
