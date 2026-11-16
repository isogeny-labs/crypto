#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=${1:?fresh attempt};raw="$here/runs/$attempt";build="/var/tmp/koblitz-r50-$attempt"
[ ! -e "$raw" ];[ ! -e "$build" ];mkdir -p "$raw" "$build"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/lean/Marked23_50.lean" "$build/";cp "$repo/tools/lean/AuditAxioms.lean" "$build/"
cp "$build/"*.lean "$raw/";cp "$0" "$raw/runner.sh"
sha256sum "$build/"*.lean > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
deps=$(awk 'NR!=10' "$here/source-data/proof-cache-roots50.txt" | paste -sd ':' -)
export LEAN_PATH="$build:$deps"
lean_bin=/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1/bin/lean
"$lean_bin" --version > "$raw/version.txt"
set +e
(cd "$build";timeout 1200 taskset -c 0 "$lean_bin" -j1 -M4096 -o Marked23_50.olean Marked23_50.lean) > "$raw/compile.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/compile.exit"
if [ "$rc" = 0 ];then
 (cd "$build";timeout 1200 taskset -c 0 "$lean_bin" -j1 -M4096 --run AuditAxioms.lean Marked23_50) > "$raw/audit.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/audit.exit"
fi
exit "$rc"
