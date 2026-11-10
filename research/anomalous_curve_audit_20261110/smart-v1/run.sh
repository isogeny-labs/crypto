#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
: "${SMART_BINARY:?set pinned Smart binary}"
: "${PROTOCOL_TOOL:?set canonical pinned protocol checker}"
sha256sum --check frozen-files.sha256
python3 - "$SMART_BINARY" "$PROTOCOL_TOOL" <<'PY'
import hashlib,json,subprocess,sys
p=json.load(open('protocol.json'))
assert p['freeze']['status']=='frozen'
for path,digest in [(sys.argv[1],p['arms'][0]['instrument']['binary_sha256']),
                    (sys.argv[2],p['extensions']['protocol_tool_sha256'])]:
    assert hashlib.sha256(open(path,'rb').read()).hexdigest()==digest,path
assert subprocess.check_output(['git','show','HEAD:'+p['study_dir']+'/protocol.json'])==open('protocol.json','rb').read()
PY
python3 "$PROTOCOL_TOOL" validate protocol.json
if [ -e evidence/runs ] || [ -e evidence/analysis.jsonl ]; then
  echo 'Refusing to overwrite existing run data.' >&2
  exit 2
fi
mkdir evidence/runs
protocol_hash=$(python3 -c 'import json;print(json.load(open("protocol.json"))["freeze"]["protocol_sha256"])')
for arm in small offcurve large; do
  prefix="evidence/runs/$arm"
  args=("inputs/$arm.txt" "$prefix-data")
  date -u +%FT%TZ > "$prefix.start"
  printf '%s\n' "$SMART_BINARY" "${args[@]}" > "$prefix.argv"
  set +e
  FROZEN_SMART_PROTOCOL_SHA256="$protocol_hash" OMP_NUM_THREADS=1 \
    timeout 15s "$SMART_BINARY" "${args[@]}" > "$prefix.stdout" 2> "$prefix.stderr"
  rc=$?
  set -e
  printf '%s\n' "$rc" > "$prefix.exit"
  date -u +%FT%TZ > "$prefix.end"
done
set +e
node native/analyze.mjs "$PWD" > evidence/analysis.jsonl 2> evidence/analysis.stderr
rc=$?
set -e
printf '%s\n' "$rc" > evidence/analysis.exit
exit "$rc"
