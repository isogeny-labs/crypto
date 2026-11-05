#!/usr/bin/env bash
set -euo pipefail
here=${1:?round};raw=${2:?raw};repo=$(git -C "$here" rev-parse --show-toplevel)
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
python3 - "$here" "$raw" "$repo" <<'PY'
from pathlib import Path
import hashlib,json,subprocess,sys
h,r,repo=map(Path,sys.argv[1:]);rev=(r/'source-revision.txt').read_text().strip()
for local,source in [('driver.sh','RUST_DRIVER.sh'),('launcher.sh','REPLAY_RUST.sh'),('input.rs','CHECK61.rs'),('PROTOCOL.md','PROTOCOL.md'),('protocol.json','protocol.json'),('RUST-PROTOCOL.md','RUST-PROTOCOL.md'),('donors.json','donors.json')]:
 assert subprocess.check_output(['git','show',rev+':'+str((h/source).relative_to(repo))],cwd=repo)==(r/local).read_bytes(),source
for row in json.loads((r/'donors.json').read_text()):
 p=(h/row['path']).resolve();assert p.stat().st_size==row['bytes'] and hashlib.sha256(p.read_bytes()).hexdigest()==row['sha256'],row['path']
names=['driver.sh','launcher.sh','input.rs','PROTOCOL.md','protocol.json','RUST-PROTOCOL.md','donors.json']
(r/'source.sha256').write_text(''.join(hashlib.sha256((r/n).read_bytes()).hexdigest()+'  '+n+'\n' for n in names))
PY
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
compiler=/home/ec2-user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
"$compiler" --version > "$raw/compiler-version.txt"
sha256sum "$compiler" > "$raw/compiler.sha256"
set +e
(cd "$raw";ulimit -v 4194304;timeout 1200 taskset -c 0 "$compiler" --edition=2021 -Copt-level=2 -Ccodegen-units=1 -Cdebuginfo=0 input.rs -o rustcheck61) > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
set -e
if [ "$rc" != 0 ];then exit "$rc";fi
sha256sum "$raw/rustcheck61" > "$raw/binary.sha256"
date -u +%FT%TZ > "$raw/started-utc.txt"
set +e
(cd "$raw";ulimit -v 4194304;timeout 1200 taskset -c 0 ./rustcheck61 ../cubic61-02/cubic-space61.sing ../fix61-02/fixed-space61.sing ../cohort61-01/cohort-certificates61.sing) > "$raw/native.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt"
date -u +%FT%TZ > "$raw/finished-utc.txt"
set -e
tar -cJf "$here/checkpoint-$(basename "$raw").tar.xz" -C "$here" "runs/$(basename "$raw")"
if [ "$rc" != 0 ] || ! rg -qx RUSTCHECK61_ALL_CHECKS_PASS "$raw/native.log";then echo "FAIL $(basename "$raw") native=$rc";exit 1;fi
echo "PASS $(basename "$raw")"
