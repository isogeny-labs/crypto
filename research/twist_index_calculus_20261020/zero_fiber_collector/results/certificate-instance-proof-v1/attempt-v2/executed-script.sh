#!/usr/bin/env bash
# A single managed proof owner; each native/Lean phase uses the common lock.
set -Eeuo pipefail
if test "${1:-}" != --frozen; then
  instance_out=${1:?fresh run output}
  test ! -e "$instance_out"
  mkdir -p "$instance_out/metadata"
  instance_out=$(cd "$instance_out" && pwd)
  cp "$0" "$instance_out/metadata/executed-script.sh"
  exec bash "$instance_out/metadata/executed-script.sh" --frozen "$instance_out" "${@:2}"
fi
shift
instance_out=$1
instance_gate=$(cd "${2:?successful native gate}" && pwd)
instance_repo=$(git rev-parse --show-toplevel)
instance_base=$instance_repo/research/twist_index_calculus_20261020/zero_fiber_collector
test ! -e "$instance_out/metadata/started-utc.txt"
test "$(cat "$instance_gate/metadata/exit")" = 0
sha256sum -c "$instance_gate/metadata/sources.sha256" > "$instance_out/metadata/sources-unchanged-before.txt"
sha256sum -c "$instance_gate/metadata/verifier.sha256" > "$instance_out/metadata/binary-unchanged-before.txt"
cp "$instance_gate/verifier" "$instance_out/verifier"
chmod 500 "$instance_out/verifier"
cp "$instance_gate/metadata/sources.sha256" "$instance_out/metadata/sources.sha256"
cp "$instance_base/degree_closure_probe/run-phase.sh" "$instance_out/metadata/run-phase.sh"
date -u +%FT%TZ > "$instance_out/metadata/started-utc.txt"
git rev-parse HEAD > "$instance_out/metadata/source-commit.txt"
uname -a > "$instance_out/metadata/host.txt"
printf '%s\n' "$$" > "$instance_out/metadata/owner-pid.txt"
printf '%s\n' 'One sealed candidate instance; native export, Lean kernel proof, axiom audit and standard IDC; per phase one worker, common reservation, 8 GiB AS and 1200-second outer bound. No relation collection or DLP execution.' > "$instance_out/metadata/resource-envelope.txt"
sha256sum "$instance_out/verifier" "$instance_out/metadata/executed-script.sh" "$instance_out/metadata/run-phase.sh" > "$instance_out/metadata/frozen-execution.sha256"
instance_finish() {
  local instance_rc=$?
  trap - EXIT
  date -u +%FT%TZ > "$instance_out/metadata/finished-utc.txt"
  printf '%s\n' "$instance_rc" > "$instance_out/metadata/terminal-exit"
  exit "$instance_rc"
}
trap instance_finish EXIT
export LC_ALL=C RAYON_NUM_THREADS=1
ulimit -c 0
instance_phase() {
  local instance_tag=$1 instance_rc
  shift
  test ! -e "$instance_out/metadata/$instance_tag.command.txt"
  printf '%q ' "$@" > "$instance_out/metadata/$instance_tag.command.txt"
  date -u +%FT%TZ > "$instance_out/metadata/$instance_tag.queued-utc.txt"
  if python3 "$instance_repo/tools/isolated_bench.py" busy -- \
    bash "$instance_out/metadata/run-phase.sh" "$instance_out/metadata/$instance_tag" "$@" \
    > "$instance_out/metadata/$instance_tag.stdout" 2> "$instance_out/metadata/$instance_tag.stderr"; then instance_rc=0; else instance_rc=$?; fi
  printf '%s\n' "$instance_rc" > "$instance_out/metadata/$instance_tag.exit"
  date -u +%FT%TZ > "$instance_out/metadata/$instance_tag.finished-utc.txt"
  printf '%s exited %s\n' "$instance_tag" "$instance_rc"
  return "$instance_rc"
}
instance_phase export "$instance_out/verifier" export \
  "$instance_base/degree_closure_probe/workload-v1.json" \
  "$instance_base/runs/degree-closure-probe-panel-v1/cases/p37-seed0-phi-l0-active-sys0-raw16/first/record.json" \
  "$instance_base/collision_certificate/proof/RepeatedFactor.lean" "$instance_out/exported"
sha256sum "$instance_out/exported/"* > "$instance_out/metadata/exported-source-seals.sha256"
instance_lean=/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1/bin/lean
"$instance_lean" --version > "$instance_out/metadata/lean-version.txt"
instance_phase lean "$instance_lean" -j1 -R "$instance_out/exported" \
  -o "$instance_out/exported/ConcreteCertificate.olean" "$instance_out/exported/ConcreteCertificate.lean"
instance_phase lean-axioms "$instance_out/verifier" check-lean \
  "$instance_out/metadata/lean.stdout" "$instance_out/exported/ConcreteCertificate.lean" \
  "$instance_out/exported/statement.json" "$instance_out/lean-audit.json"
instance_phase idc "$instance_out/verifier" idc "$instance_out/exported/statement.json" \
  "$instance_out/exported/ConcreteCertificate.lean" "$instance_out/lean-audit.json" "$instance_out/idc"
sha256sum -c "$instance_out/metadata/exported-source-seals.sha256" > "$instance_out/metadata/exported-sources-unchanged-after.txt"
sha256sum -c "$instance_out/metadata/sources.sha256" > "$instance_out/metadata/sources-unchanged-after.txt"
sha256sum -c "$instance_out/metadata/frozen-execution.sha256" > "$instance_out/metadata/frozen-execution-unchanged-after.txt"
