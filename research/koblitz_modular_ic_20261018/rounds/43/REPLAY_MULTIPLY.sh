#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel);raw="$here/runs/${1:?fresh attempt}"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
[ "$(cat "$here/runs/jets43-02/exit.txt")" = 0 ];rg -qx JET43_ALL_CHECKS_PASS "$here/runs/jets43-02/native.log"
cp "$here/MULTIPLY43.rs" "$raw/";cp "$0" "$raw/runner.sh"
cat "$here/runs/jets43-02/output/space1.txt" "$here/runs/jets43-02/output/space2.txt" "$here/runs/jets43-02/output/space3.txt" > "$raw/evaluation-words615.txt"
sha256sum "$raw/MULTIPLY43.rs" "$raw/evaluation-words615.txt" "$raw/runner.sh" > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc -O --edition=2021 "$raw/MULTIPLY43.rs" -o "$raw/multiply43") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/multiply43" "$raw/evaluation-words615.txt" "$raw/output") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
[ "$rc" = 0 ];rg -qx MULT43_ALL_CHECKS_PASS "$raw/native.log";echo "PASS ${1}"
