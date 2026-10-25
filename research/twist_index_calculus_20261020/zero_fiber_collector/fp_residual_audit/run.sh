#!/usr/bin/env bash
# Thin single owner, two processes, common per-phase reservation; no retries.
set -Eeuo pipefail
if test "${1:-}" != --frozen; then
  fp_out=${1:?fresh run output}
  test ! -e "$fp_out"
  mkdir -p "$fp_out/metadata"
  fp_out=$(cd "$fp_out" && pwd)
  cp "$0" "$fp_out/metadata/executed-script.sh"
  exec bash "$fp_out/metadata/executed-script.sh" --frozen "$fp_out" "${@:2}"
fi
shift
fp_out=$1
fp_gate=$(cd "${2:?passing native gate}" && pwd)
fp_repo=$(git rev-parse --show-toplevel)
fp_catalog=$fp_repo/research/twist_index_calculus_20261020/zero_fiber_collector/collision_certificate/workload-v1.json
fp_helper=$fp_repo/research/twist_index_calculus_20261020/zero_fiber_collector/degree_closure_probe/run-phase.sh
test "$(cat "$fp_gate/metadata/exit")" = 0
test ! -e "$fp_out/metadata/started-utc.txt"
fp_finish() { local fp_code=$?; date -u +%FT%TZ > "$fp_out/metadata/finished-utc.txt"; printf '%s\n' "$fp_code" > "$fp_out/metadata/terminal-exit"; }
trap fp_finish EXIT
date -u +%FT%TZ > "$fp_out/metadata/started-utc.txt"
git rev-parse HEAD > "$fp_out/metadata/source-commit.txt"
printf '%s\n' "$$" > "$fp_out/metadata/owner-pid.txt"
printf '%s\n' '42 immutable case-arms, first and A/A process, common lock, one science worker, 8 GiB AS, 1200-second outer phase limit; no admitted timing.' > "$fp_out/metadata/resource-envelope.txt"
sha256sum -c "$fp_gate/metadata/sources.sha256" > "$fp_out/metadata/gate-sources-unchanged.txt"
sha256sum -c "$fp_gate/metadata/auditor.sha256" > "$fp_out/metadata/gate-binary-unchanged.txt"
cp "$fp_gate/auditor" "$fp_out/auditor"
chmod 500 "$fp_out/auditor"
cp "$fp_gate/metadata/sources.sha256" "$fp_out/metadata/sources.sha256"
cp "$fp_helper" "$fp_out/metadata/run-phase.sh"
sha256sum "$fp_out/auditor" "$fp_out/metadata/executed-script.sh" "$fp_out/metadata/run-phase.sh" "$fp_catalog" > "$fp_out/metadata/frozen-execution.sha256"
export LC_ALL=C RAYON_NUM_THREADS=1
ulimit -c 0
fp_phase() {
  local fp_tag=$1 fp_rc
  shift
  test ! -e "$fp_out/metadata/$fp_tag.command.txt"
  printf '%q ' "$@" > "$fp_out/metadata/$fp_tag.command.txt"
  date -u +%FT%TZ > "$fp_out/metadata/$fp_tag.queued-utc.txt"
  if "$@" > "$fp_out/metadata/$fp_tag.stdout" 2> "$fp_out/metadata/$fp_tag.stderr"; then fp_rc=0; else fp_rc=$?; fi
  printf '%s\n' "$fp_rc" > "$fp_out/metadata/$fp_tag.exit"
  printf '%s exited %s\n' "$fp_tag" "$fp_rc"
  return "$fp_rc"
}
fp_status=0
for fp_name in first aa; do
  fp_phase "$fp_name" python3 "$fp_repo/tools/isolated_bench.py" busy -- bash "$fp_out/metadata/run-phase.sh" "$fp_out/metadata/$fp_name-reserved" "$fp_out/auditor" audit "$fp_catalog" "$fp_out/$fp_name" || fp_status=$?
done
if test -d "$fp_out/first" && test -d "$fp_out/aa"; then
  fp_phase parity diff --recursive --no-dereference "$fp_out/first" "$fp_out/aa" || fp_status=$?
fi
sha256sum -c "$fp_out/metadata/sources.sha256" > "$fp_out/metadata/sources-unchanged-after.txt"
sha256sum -c "$fp_out/metadata/frozen-execution.sha256" > "$fp_out/metadata/frozen-execution-unchanged-after.txt"
exit "$fp_status"
