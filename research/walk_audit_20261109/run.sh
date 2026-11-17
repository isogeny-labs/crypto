#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
: "${WALK_BINARY:?set WALK_BINARY to frozen census binary}"
sha256sum --check frozen-files.sha256
node - "$WALK_BINARY" <<'JS'
const fs=require('fs'),crypto=require('crypto');
const p=JSON.parse(fs.readFileSync('protocol.json'));
if(p.freeze.status!=='frozen') throw Error('protocol is not frozen');
const hash=crypto.createHash('sha256').update(fs.readFileSync(process.argv[2])).digest('hex');
if(hash!==p.arms[0].instrument.binary_sha256) throw Error('binary hash mismatch');
JS
if [ -e evidence/runs ]; then echo 'Refusing to overwrite existing output.' >&2; exit 2; fi
mkdir evidence/runs
good=1
for rep in 0 1; do
  for arm in mix rho; do
    prefix="evidence/runs/$arm-$rep"
    printf '%s\n' "$WALK_BINARY" "--$arm" > "$prefix.argv"
    date -u +%s > "$prefix.start-epoch"
    set +e
    timeout 120s "$WALK_BINARY" "--$arm" > "$prefix.jsonl" 2> "$prefix.stderr"
    rc=$?
    set -e
    printf '%s\n' "$rc" > "$prefix.exit"
    date -u +%s > "$prefix.end-epoch"
    if [ "$rc" -ne 0 ]; then good=0; fi
  done
done
set +e
for arm in mix rho; do
  cmp "evidence/runs/$arm-0.jsonl" "evidence/runs/$arm-1.jsonl" > "evidence/runs/$arm-repeat.diff" 2>&1
  rc=$?
  printf '%s\n' "$rc" > "evidence/runs/$arm-repeat.exit"
  if [ "$rc" -ne 0 ]; then good=0; fi
  cmp "evidence/runs/$arm-0.stderr" "evidence/runs/$arm-1.stderr" > "evidence/runs/$arm-summary-repeat.diff" 2>&1
  rc=$?
  printf '%s\n' "$rc" > "evidence/runs/$arm-summary-repeat.exit"
  if [ "$rc" -ne 0 ]; then good=0; fi
done
set -e
if [ "$good" -eq 1 ]; then exit 0; else exit 1; fi
