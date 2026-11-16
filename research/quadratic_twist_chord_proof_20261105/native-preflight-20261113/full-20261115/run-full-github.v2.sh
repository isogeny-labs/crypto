#!/usr/bin/env bash
# GitHub-checkpoint variant authorized by the user on 2026-11-15.
# Historical S3 launcher and official checkpoint script remain unchanged.
set -euo pipefail
if [ "${1:-}" = --help ]; then
  echo 'TRACE_PRODUCER=/outside/producer TRACE_AUDITOR=/outside/auditor TRACE_OUTPUT=/fresh/outside/output bash run-full-github.v2.sh'
  echo 'Run under timeout 3600. Requires current source/binary pins and matching published GitHub branch head.'
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
[ -z "${CHECKPOINT_SKIP:-}" ] || fail 'no checkpoint-skip flag is used'
# Live GitHub connector receipt; no shell credential substitution or auth probes.
: "${TRACE_GITHUB_REF_RECEIPT:?set fresh outside-checkout connected-GitHub ref receipt}"
remote_head=$(python3 - "$TRACE_GITHUB_REF_RECEIPT" <<'PY_REF'
import json,sys,datetime
from pathlib import Path
v=json.loads(Path(sys.argv[1]).read_text())
assert v['repository']=='isogeny-labs/crypto' and v['branch']=='codex/twist-chord-proof'
assert v['state']=='open' and v['auto_merge'] is None and v['source']=='connected-github-read'
t=datetime.datetime.fromisoformat(v['observed_at'].replace('Z','+00:00'))
assert 0 <= (datetime.datetime.now(datetime.timezone.utc)-t).total_seconds() <= 60
print(v['head'])
PY_REF
)
[ "$remote_head" = "$(git rev-parse HEAD)" ] || fail 'publish exact source/contract on approved GitHub branch before launch'
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
# User-selected durable checkpoint: exact source/contract must already be on GitHub.
# This variant does not claim an S3 gate call or S3 checkpoint success.
python3 - "$root" "$base" <<'PY_CHECKPOINT'
import hashlib,json,pathlib,sys
root,base=sys.argv[1:]
pins=json.loads((pathlib.Path(root)/base/'full-20261115/FULL_RUN_PINS.v2.json').read_text())
assert pins['checkpoint_transport']=='github-explicit-user-selection-20261115'
for path,want in pins['files'].items():
    assert hashlib.sha256((pathlib.Path(root)/path).read_bytes()).hexdigest()==want,path
PY_CHECKPOINT
mkdir -m 700 "$out"
cp "$base/ENGINEERING_INPUTS.json" "$base/ENGINEERING_PINS.json" "$base/full-20261115/FULL_RUN_PINS.v2.json" "$base/full-20261115/protocol.v2.json" "$base/GITHUB_CHECKPOINT_AMENDMENT.md" "$out/"
printf '%s\n' "$remote_head" > "$out/published-github-head.txt"
git rev-parse HEAD > "$out/head.txt"
cp "$TRACE_GITHUB_REF_RECEIPT" "$out/github-ref-receipt.json"
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
  run "produce-$repeat" "$TRACE_PRODUCER" produce "$root" "$manifest" "$manifest_hash" full "$out/$repeat.json"
  run "audit-$repeat" "$TRACE_AUDITOR" audit "$root" "$manifest" "$manifest_hash" full "$out/$repeat.json" "$out/$repeat.receipt.json"
done
run audit-positive "$TRACE_AUDITOR" audit "$root" "$manifest" "$manifest_hash" full "$out/A.json" "$out/positive.receipt.json"
cmp "$out/A.json" "$out/B.json"
cmp "$out/A.receipt.json" "$out/B.receipt.json"
for mode in wrong-d wrong-target missing-tangent duplicate-record wrong-inverse missing-attempt false-root missing-case source-pin spurious-infinity; do
  run "mutate-$mode" "$TRACE_PRODUCER" mutate "$out/A.json" "$out/$mode.json" "$mode"
  ! cmp -s "$out/A.json" "$out/$mode.json" || fail 'control did not change bytes'
  code=0
  run "reject-$mode" "$TRACE_AUDITOR" audit "$root" "$manifest" "$manifest_hash" full "$out/$mode.json" "$out/$mode.receipt.json" || code=$?
  [ "$code" = 101 ] || fail "control $mode expected rejection101, got $code"
  [ ! -e "$out/$mode.receipt.json" ] || fail "control $mode emitted a receipt"
done
step=full-coverage-complete
printf '%s\n' 'Full geometry replay and controls completed; unsigned observation only.' > "$out/ENGINEERING_STATUS.txt"
echo "Review actual outputs in $out; publish exact evidence and current handoff on the same GitHub branch."
