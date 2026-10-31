#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=${1:?fresh attempt}
build="/var/tmp/koblitz-r27-$attempt"
raw="$here/runs/$attempt"
[ ! -e "$build" ];[ ! -e "$raw" ];mkdir -p "$build" "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/lean/KoblitzTorsion27.lean" "$build/"
cp "$here/lean/KoblitzCollector27.lean" "$build/"
cp "$repo/tools/lean/AuditAxioms.lean" "$build/"
cp "$build/KoblitzTorsion27.lean" "$raw/"
sha256sum "$build"/*.lean > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
deps=$(awk 'NR!=10' "$here/../22/runs/proof-cache-source-roots.txt" | paste -sd ':' -)
export LEAN_PATH="$build:$deps"
lean_bin=/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1/bin/lean
set +e
(cd "$build";timeout 1200 taskset -c 0 "$lean_bin" -j1 -M4096 -o KoblitzCollector27.olean KoblitzCollector27.lean) > "$raw/collector-compile.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/collector-compile.exit"
if [ "$rc" != 0 ];then exit "$rc";fi
(cd "$build";timeout 1200 taskset -c 0 "$lean_bin" -j1 -M4096 -o KoblitzTorsion27.olean KoblitzTorsion27.lean) > "$raw/compile.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/compile.exit"
if [ "$rc" = 0 ];then
 (cd "$build";timeout 1200 taskset -c 0 "$lean_bin" -j1 -M4096 --run AuditAxioms.lean KoblitzCollector27 KoblitzTorsion27) > "$raw/audit.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/audit.exit"
fi
exit "$rc"
