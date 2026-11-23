#!/usr/bin/env bash
# Executed only from the fresh raw copy; never edit that copy while live.
set -euo pipefail
here=${1:?round};raw=${2:?raw};input=${3:?input};marker=${4:?marker}
repo=$(git -C "$here" rev-parse --show-toplevel)
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
python3 - "$here" "$raw" "$repo" "$input" <<'PY'
from pathlib import Path
import hashlib,json,subprocess,sys
h,r,repo=map(Path,sys.argv[1:4]);rev=(r/'source-revision.txt').read_text().strip()
for local,source in [('driver.sh','NATIVE_DRIVER.sh'),('launcher.sh','REPLAY_CAS.sh'),('input.sing',sys.argv[4]),('PROTOCOL.md','PROTOCOL.md'),('protocol.json','protocol.json'),('donors.json','donors.json')]:
 original=subprocess.check_output(['git','show',rev+':'+str((h/source).relative_to(repo))],cwd=repo)
 assert original==(r/local).read_bytes(),source
for row in json.loads((r/'donors.json').read_text()):
 p=(h/row['path']).resolve();assert p.stat().st_size==row['bytes'] and hashlib.sha256(p.read_bytes()).hexdigest()==row['sha256'],row['path']
names=['driver.sh','launcher.sh','input.sing','PROTOCOL.md','protocol.json','donors.json']
(r/'source.sha256').write_text(''.join(hashlib.sha256((r/n).read_bytes()).hexdigest()+'  '+n+'\n' for n in names))
PY
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
binary=/home/ec2-user/.cache/koblitz-native-cas/singular/bin/Singular
sha256sum "$binary" > "$raw/binary.sha256"
date -u +%FT%TZ > "$raw/started-utc.txt"
set +e
(cd "$raw";ulimit -v 4194304;timeout 1200 "$binary" --random=61 --threads=1 --flint-threads=1 -q input.sing) > "$raw/native.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt"
date -u +%FT%TZ > "$raw/finished-utc.txt"
set -e
tar -cJf "$here/checkpoint-$(basename "$raw").tar.xz" -C "$here" "runs/$(basename "$raw")"
if [ "$rc" != 0 ] || rg -q '^[[:space:]]*\?' "$raw/native.log" || ! rg -qx "$marker" "$raw/native.log";then echo "FAIL $(basename "$raw") native=$rc";exit 1;fi
echo "PASS $(basename "$raw")"
