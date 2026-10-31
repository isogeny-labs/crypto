#!/usr/bin/env bash
set -euo pipefail
here=${1:?round};raw=${2:?raw};attempt=${3:?attempt}
repo=$(git -C "$here" rev-parse --show-toplevel)
build="/var/tmp/koblitz-r54-$attempt";[ ! -e "$build" ];mkdir -p "$build"
cp "$raw/Fiber54.lean" "$build/"
cp "$repo/tools/lean/AuditAxioms.lean" "$raw/";cp "$raw/AuditAxioms.lean" "$build/"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
python3 - "$here" "$raw" "$repo" <<'PY'
from pathlib import Path
import hashlib,subprocess,sys
h,r,repo=map(Path,sys.argv[1:]);rev=(r/'source-revision.txt').read_text().strip()
for local,source in [('driver.sh','LEAN_DRIVER.sh'),('launcher.sh','REPLAY_LEAN.sh'),('Fiber54.lean','lean/Fiber54.lean'),('LEAN-PROTOCOL.md','LEAN-PROTOCOL.md')]:
 assert subprocess.check_output(['git','show',rev+':'+str((h/source).relative_to(repo))],cwd=repo)==(r/local).read_bytes(),source
assert subprocess.check_output(['git','show',rev+':tools/lean/AuditAxioms.lean'],cwd=repo)==(r/'AuditAxioms.lean').read_bytes()
(r/'source.sha256').write_text(''.join(hashlib.sha256((r/n).read_bytes()).hexdigest()+'  '+n+'\n' for n in ['driver.sh','launcher.sh','Fiber54.lean','AuditAxioms.lean','LEAN-PROTOCOL.md']))
PY
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
deps=$(awk 'NR!=10' "$here/../51/source-data/proof-cache-roots51.txt" | paste -sd ':' -)
export LEAN_PATH="$build:$deps"
lean_bin=/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1/bin/lean
export PATH="$(dirname "$lean_bin"):$PATH"
"$lean_bin" --version > "$raw/version.txt"
git -C /home/ec2-user/crypto-modular-ic/research/koblitz_modular_ic_20261018/lean/.lake/packages/mathlib rev-parse HEAD > "$raw/mathlib-revision.txt"
[ "$(cat "$raw/mathlib-revision.txt")" = d13f23b723b8a846827a245b89c10fc7d3f11612 ]
set +e
(cd "$build";timeout 1200 taskset -c 0 "$lean_bin" -j1 -M4096 -o Fiber54.olean Fiber54.lean) > "$raw/compile.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/compile.exit"
if [ "$rc" = 0 ];then
 (cd "$build";timeout 1200 taskset -c 0 "$lean_bin" -j1 -M4096 --run AuditAxioms.lean Fiber54) > "$raw/audit.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/audit.exit"
fi
if [ "$rc" = 0 ];then cp "$build/Fiber54.olean" "$raw/";fi
exit "$rc"
