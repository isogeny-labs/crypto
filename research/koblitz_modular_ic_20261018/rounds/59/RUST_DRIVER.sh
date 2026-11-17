#!/usr/bin/env bash
set -euo pipefail
here=${1:?round};raw=${2:?raw};repo=$(git -C "$here" rev-parse --show-toplevel)
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
python3 - "$here" "$raw" "$repo" <<'PY'
from pathlib import Path
import hashlib,json,subprocess,sys
h,r,repo=map(Path,sys.argv[1:]);rev=(r/'source-revision.txt').read_text().strip()
names=['RUST_DRIVER.sh','REPLAY_RUST.sh','COHORT59.rs','PROTOCOL.md','protocol.json','RUST-PROTOCOL.md','donors.json']
for name in names:assert subprocess.check_output(['git','show',rev+':'+str((h/name).relative_to(repo))],cwd=repo)==(r/name).read_bytes(),name
for row in json.loads((r/'donors.json').read_text()):
 p=(h/row['path']).resolve();assert p.stat().st_size==row['bytes'] and hashlib.sha256(p.read_bytes()).hexdigest()==row['sha256'],row['path']
assert (h/'runs/data59-02/data59.txt').read_bytes()==(r/'data59.txt').read_bytes()
(r/'source.sha256').write_text(''.join(hashlib.sha256((r/n).read_bytes()).hexdigest()+'  '+n+'\n'for n in names+['data59.txt']))
PY
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
compiler=/home/ec2-user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
"$compiler" --version > "$raw/compiler-version.txt"
set +e
(cd "$raw";ulimit -v 4194304;timeout 1200 taskset -c 0 "$compiler" --edition=2021 -Copt-level=2 -Ccodegen-units=1 -Cdebuginfo=0 COHORT59.rs -o cohort59) > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
set -e
if [ "$rc" != 0 ];then exit "$rc";fi
sha256sum "$raw/cohort59" > "$raw/binary.sha256"
date -u +%FT%TZ > "$raw/started-utc.txt"
set +e
(cd "$raw";ulimit -v 4194304;timeout 1200 taskset -c 0 ./cohort59 data59.txt certificate59.txt cohort59.tsv) > "$raw/native.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt"
date -u +%FT%TZ > "$raw/finished-utc.txt"
set -e
tar -cJf "$here/checkpoint-$(basename "$raw").tar.xz" -C "$here" "runs/$(basename "$raw")"
if [ "$rc" != 0 ] || ! rg -qx COHORT59_ALL_CHECKS_PASS "$raw/native.log";then echo "FAIL $(basename "$raw") native=$rc";exit 1;fi
echo "PASS $(basename "$raw")"
