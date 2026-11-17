#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
: "${MSOLVE_BINARY:?set pinned msolve binary}"
: "${ANALYZE_BINARY:?set pinned analyzer binary}"
: "${MSOLVE_LIBRARY_PATH:?set dependency directory}"
sha256sum --check frozen-files.sha256
python3 - "$MSOLVE_BINARY" "$ANALYZE_BINARY" <<'PY'
import hashlib,json,subprocess,sys
p=json.load(open('protocol.json'))
assert p['freeze']['status']=='frozen'
for path,digest in [(sys.argv[1],p['arms'][0]['instrument']['binary_sha256']),
                    (sys.argv[2],p['extensions']['analysis_binary_sha256'])]:
    assert hashlib.sha256(open(path,'rb').read()).hexdigest()==digest,path
# The committed frozen bytes must precede affected output.
assert subprocess.check_output(['git','show','HEAD:'+p['study_dir']+'/protocol.json'])==open('protocol.json','rb').read()
PY
if [ -e evidence/runs ] || [ -e evidence/analysis.jsonl ]; then
  echo 'Refusing to overwrite existing run data.' >&2
  exit 2
fi
mkdir evidence/runs
for arm in ordinary saturation; do
  prefix="evidence/runs/$arm"
  args=(-t 1 -l 2 -c 0 -v 0 -g 2 -f "inputs/$arm.ms" -o "$prefix.out")
  if [ "$arm" = saturation ]; then args+=(-S); fi
  date -u +%FT%TZ > "$prefix.start"
  printf '%s\n' "$MSOLVE_BINARY" "${args[@]}" > "$prefix.argv"
  set +e
  LD_LIBRARY_PATH="$MSOLVE_LIBRARY_PATH" OMP_NUM_THREADS=1 \
    timeout 30s "$MSOLVE_BINARY" "${args[@]}" > "$prefix.stdout" 2> "$prefix.stderr"
  rc=$?
  set -e
  printf '%s\n' "$rc" > "$prefix.exit"
  date -u +%FT%TZ > "$prefix.end"
done
set +e
"$ANALYZE_BINARY" "$PWD" > evidence/analysis.jsonl 2> evidence/analysis.stderr
rc=$?
set -e
printf '%s\n' "$rc" > evidence/analysis.exit
exit "$rc"
