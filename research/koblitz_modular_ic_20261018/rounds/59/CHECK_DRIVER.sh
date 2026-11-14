#!/usr/bin/env bash
set -euo pipefail
here=${1:?round};raw=${2:?raw};repo=$(git -C "$here" rev-parse --show-toplevel)
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
python3 - "$here" "$raw" "$repo" <<'PY'
from pathlib import Path
import hashlib,json,subprocess,sys
h,r,repo=map(Path,sys.argv[1:]);rev=(r/'source-revision.txt').read_text().strip()
names=['CHECK_DRIVER.sh','REPLAY_CHECK.sh','COHORT59.rs','PROTOCOL.md','protocol.json','RUST-PROTOCOL.md','donors.json']
for name in names:assert subprocess.check_output(['git','show',rev+':'+str((h/name).relative_to(repo))],cwd=repo)==(r/name).read_bytes(),name
for row in json.loads((r/'donors.json').read_text()):
 p=(h/row['path']).resolve();assert p.stat().st_size==row['bytes'] and hashlib.sha256(p.read_bytes()).hexdigest()==row['sha256'],row['path']
for name,source in [('data59.txt','runs/data59-02/data59.txt'),('certificate59.txt','runs/cohort59-01/certificate59.txt'),('cohort59','runs/cohort59-01/cohort59')]:assert (r/name).read_bytes()==(h/source).read_bytes(),name
assert (r/'COHORT59.rs').read_bytes()==(h/'runs/cohort59-01/COHORT59.rs').read_bytes()
(r/'source.sha256').write_text(''.join(hashlib.sha256((r/n).read_bytes()).hexdigest()+'  '+n+'\n'for n in names+['data59.txt','certificate59.txt','cohort59']))
PY
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
sha256sum "$raw/cohort59" > "$raw/binary.sha256"
date -u +%FT%TZ > "$raw/started-utc.txt"
set +e
(cd "$raw";ulimit -v 4194304;timeout 1200 taskset -c 0 ./cohort59 --check data59.txt certificate59.txt) > "$raw/native.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt"
date -u +%FT%TZ > "$raw/finished-utc.txt"
set -e
tar -cJf "$here/checkpoint-$(basename "$raw").tar.xz" -C "$here" "runs/$(basename "$raw")"
if [ "$rc" != 0 ] || ! rg -qx CHECKCOHORT59_ALL_CHECKS_PASS "$raw/native.log";then echo "FAIL $(basename "$raw") native=$rc";exit 1;fi
echo "PASS $(basename "$raw")"
