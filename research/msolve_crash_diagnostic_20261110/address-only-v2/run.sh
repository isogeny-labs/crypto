#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
sha256sum --check frozen-files.sha256
python3 - <<'CHECK'
import hashlib,json,subprocess
from pathlib import Path
j=json.loads(Path('protocol.json').read_text());assert j['freeze']['status']=='frozen'
assert subprocess.check_output(['git','show','HEAD:'+j['study_dir']+'/protocol.json'])==Path('protocol.json').read_bytes()
for arm in j['arms']:
 p=Path('/workspace/msolve-deps/address-only-20261110/msolve')
 assert hashlib.sha256(p.read_bytes()).hexdigest()==arm['instrument']['binary_sha256']
a=Path('/workspace/msolve-deps/address-only-20261110/analyze')
assert hashlib.sha256(a.read_bytes()).hexdigest()==j['extensions']['analyzer_binary_sha256']
CHECK
[ ! -e evidence/runs ] || { echo 'Refusing existing data' >&2; exit 2; }
ulimit -c 0
mkdir evidence/runs
for la in 2 1; do
  for build in address; do
    binary=/workspace/msolve-deps/address-only-20261110/msolve
    for mode in ordinary saturation; do
      arm="$build-l$la-$mode"
      prefix="evidence/runs/$arm"
      args=(-t 1 -l "$la" -c 0 -v 0 -g 2 -f "inputs/$mode.ms" -o "$prefix.out")
      if [ "$mode" = saturation ]; then args+=(-S); fi
      printf '%s\n' "$binary" "${args[@]}" > "$prefix.argv"
      date -u +%FT%TZ > "$prefix.start"
      set +e
      LD_LIBRARY_PATH=/workspace/msolve-deps/root/usr/lib/x86_64-linux-gnu OMP_NUM_THREADS=1 \
        ASAN_OPTIONS=detect_leaks=0:abort_on_error=1:symbolize=1 \
        timeout 5s "$binary" "${args[@]}" > "$prefix.stdout" 2> "$prefix.stderr"
      rc=$?
      set -e
      printf '%s\n' "$rc" > "$prefix.exit"
      date -u +%FT%TZ > "$prefix.end"
    done
  done
done
set +e
/workspace/msolve-deps/address-only-20261110/analyze "$PWD" > evidence/analysis.jsonl 2> evidence/analysis.stderr
rc=$?
set -e
printf '%s\n' "$rc" > evidence/analysis.exit
exit "$rc"
