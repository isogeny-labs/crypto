#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=${1:?fresh attempt};raw="$here/runs/$attempt";binary="/var/tmp/koblitz-r30-$attempt"
[ ! -e "$raw" ];[ ! -e "$binary" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/DESCENT30.rs" "$raw/";cp "$0" "$raw/runner.sh"
sha256sum "$raw/DESCENT30.rs" "$raw/runner.sh" > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
sha256sum "$here/runs/color12-01/native.log" "$here/runs/color436-01/native.log" > "$raw/inputs.sha256"
date -u +%FT%TZ > "$raw/started-utc.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc --edition=2021 -O "$raw/DESCENT30.rs" -o "$binary") > "$raw/compile.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/compile.exit"
if [ "$rc" = 0 ];then (ulimit -v 4194304;timeout 1200 "$binary" "$raw/certificates" "$here") > "$raw/native.log" 2>&1;rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";fi
date -u +%FT%TZ > "$raw/finished-utc.txt"
set -e
[ "$rc" = 0 ];rg -qx DESCENT30_ALL_CHECKS_PASS "$raw/native.log"
echo "PASS $attempt"
