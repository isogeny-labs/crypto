#!/usr/bin/env bash
# Thin orchestration; all research arithmetic remains in Rust/C++.
set -euo pipefail
if [ "${1:-}" = --help ]; then
  echo 'TRACE_PRODUCER=/outside/producer TRACE_AUDITOR=/outside/auditor TRACE_OUTPUT=/fresh/outside/output bash run-fixed.sh'
  echo 'Run under timeout 3600. Requires current source/binary pins and approved official checkpoint.'
  exit 0
fi
[ "$#" = 0 ] || { echo 'use --help' >&2; exit 2; }
root=$(git rev-parse --show-toplevel)
cd "$root"
base=research/quadratic_twist_chord_proof_20261105/native-preflight-20261113
fail() { echo "trace preflight: $*" >&2; exit 2; }
[ "$(git branch --show-current)" = codex/twist-chord-proof ] || fail 'wrong approved branch'
[ "$(git remote get-url origin)" = https://github.com/isogeny-labs/crypto.git ] || fail 'wrong approved repository'
[ -z "$(git status --porcelain)" ] || fail 'commit all source changes first'
[ -z "${CHECKPOINT_SKIP:-}" ] || fail 'checkpoint bypass forbidden'
[ "${CHECKPOINT_BUCKET:-setec-astronomy-artifacts}" = setec-astronomy-artifacts ] || fail 'wrong bucket'
[ "${CHECKPOINT_PREFIX:-checkpoints}" = checkpoints ] || fail 'wrong prefix'
git rev-parse --verify refs/remotes/origin/main >/dev/null 2>&1 || fail 'fetch current main before launch'
[ "$(git rev-parse HEAD:scripts/checkpoint.sh)" = "$(git rev-parse refs/remotes/origin/main:scripts/checkpoint.sh)" ] || fail 'reconcile current official checkpoint tooling first'
token_file=${CHECKPOINT_TOKEN_FILE:-$HOME/.config/checkpoint/upload-token}
[ -n "${CHECKPOINT_UPLOAD_TOKEN:-}" ] || [ -s "$token_file" ] || fail 'configure approved narrow checkpoint token through environment settings'
: "${TRACE_PRODUCER:?set external compiled producer path}"
: "${TRACE_AUDITOR:?set external compiled auditor path}"
: "${TRACE_OUTPUT:?set a fresh outside-checkout output directory}"
out=$(realpath -m "$TRACE_OUTPUT")
case "$out" in "$root"|"$root"/*) fail 'output must be outside checkout';; esac
[ ! -e "$out" ] || fail 'output already exists'
# Metadata validation only: no field/group/study calculation in Python.
python3 - "$root" "$base" "$TRACE_PRODUCER" "$TRACE_AUDITOR" <<'PY'
import hashlib,json,pathlib,sys
root,base,producer,auditor=sys.argv[1:]
pins=json.loads((pathlib.Path(root)/base/'ENGINEERING_PINS.json').read_text())
assert pins['schema']=='twist.trace-fiber-engineering-pins/v1'
for path,want in pins['repository_files'].items():
    assert hashlib.sha256((pathlib.Path(root)/path).read_bytes()).hexdigest()==want,path
for name,path in [('producer',producer),('auditor',auditor)]:
    assert hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()==pins['binaries'][name],name
PY
bash "$root/scripts/checkpoint.sh" gate
mkdir -m 700 "$out"
cp "$base/ENGINEERING_INPUTS.json" "$base/ENGINEERING_PINS.json" "$out/"
git rev-parse HEAD > "$out/head.txt"
manifest="$out/ENGINEERING_INPUTS.json"
manifest_hash=$(sha256sum "$manifest" | cut -d' ' -f1)
step=none
finish() {
  local code=$?
  printf '%s\n' "$code" > "$out/launcher-exit.txt"
  printf '%s\n' "$step" > "$out/last-step.txt"
  # Never mark this as a finish checkpoint: update RESUME from actual outcomes first.
}
trap finish EXIT
run() {
  step=$1; shift
  printf '%s\n' "$step" >> "$out/steps.txt"
  printf '%q ' "$@" > "$out/$step.command"; printf '\n' >> "$out/$step.command"
  local code=0
  timeout --signal=TERM --kill-after=10 600 "$@" > "$out/$step.log" 2>&1 || code=$?
  printf '%s\n' "$code" > "$out/$step.exit"
  return "$code"
}
for repeat in A B; do
  run "produce-$repeat" "$TRACE_PRODUCER" produce "$root" "$manifest" "$manifest_hash" fixed "$out/$repeat.json"
  run "audit-$repeat" "$TRACE_AUDITOR" audit "$root" "$manifest" "$manifest_hash" fixed "$out/$repeat.json" "$out/$repeat.receipt.json"
done
cmp "$out/A.json" "$out/B.json"
cmp "$out/A.receipt.json" "$out/B.receipt.json"
for mode in wrong-d wrong-target missing-tangent duplicate-record wrong-inverse missing-attempt false-root missing-case source-pin spurious-infinity; do
  run "mutate-$mode" "$TRACE_PRODUCER" mutate "$out/A.json" "$out/$mode.json" "$mode"
  ! cmp -s "$out/A.json" "$out/$mode.json" || fail 'control did not change bytes'
  code=0
  run "reject-$mode" "$TRACE_AUDITOR" audit "$root" "$manifest" "$manifest_hash" fixed "$out/$mode.json" "$out/$mode.receipt.json" || code=$?
  [ "$code" = 101 ] || fail "control $mode expected rejection101, got $code"
  [ ! -e "$out/$mode.receipt.json" ] || fail "control $mode emitted a receipt"
done
step=engineering-complete
printf '%s\n' 'Fixed-point engineering replay and controls completed; unsigned observation only.' > "$out/ENGINEERING_STATUS.txt"
echo "Review actual outputs in $out; update RESUME and take approved finish/failure checkpoint."
