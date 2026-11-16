#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=${1:?fresh attempt};raw="$here/runs/$attempt";build="/var/tmp/koblitz-r52-$attempt"
[ ! -e "$raw" ];[ ! -e "$build" ];mkdir -p "$raw" "$build"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/lean/FibreIrred52.lean" "$build/";cp "$repo/tools/lean/AuditAxioms.lean" "$build/"
cp "$here/lean/Closure52.lean" "$build/";cp /var/tmp/koblitz-r52-closure-lean52-02/Closure52.olean "$build/"
cp "$build/"*.lean "$raw/";cp "$build/Closure52.olean" "$raw/";cp "$0" "$raw/runner.sh"
sha256sum "$build/"*.lean > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
deps=$(awk 'NR!=10' "$here/../51/source-data/proof-cache-roots51.txt" | paste -sd ':' -)
export LEAN_PATH="$build:$deps"
lean_bin=/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1/bin/lean
"$lean_bin" --version > "$raw/version.txt"
set +e
(cd "$build";timeout 1200 taskset -c 0 "$lean_bin" -j1 -M4096 -o FibreIrred52.olean FibreIrred52.lean) > "$raw/compile.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/compile.exit"
if [ "$rc" = 0 ];then
 (cd "$build";timeout 1200 taskset -c 0 "$lean_bin" -j1 -M4096 --run AuditAxioms.lean FibreIrred52) > "$raw/audit.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/audit.exit"
fi
exit "$rc"
