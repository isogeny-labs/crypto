#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel);raw="$here/runs/${1:?fresh attempt}"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
[ "$(cat "$here/runs/encode43-01/exit.txt")" = 0 ];rg -qx ENC43_ALL_CHECKS_PASS "$here/runs/encode43-01/native.log"
cp "$here/JET43.rs" "$raw/";cp "$0" "$raw/runner.sh"
for f in source-polynomials.txt original-sections.txt twist-sections.txt pairs2.txt pairs3.txt reference-sections154.txt;do cp "$here/source-data/$f" "$raw/";done
cp "$here/runs/encode43-01/reference-coordinates-units154.txt" "$raw/"
sha256sum "$raw/"*.rs "$raw/"*.txt "$raw/runner.sh" > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc -O --edition=2021 "$raw/JET43.rs" -o "$raw/jet43") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/jet43" "$raw/source-polynomials.txt" "$raw/original-sections.txt" "$raw/twist-sections.txt" "$raw/pairs2.txt" "$raw/pairs3.txt" "$raw" "$raw/output" "$raw/result.txt") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
[ "$rc" = 0 ];rg -qx JET43_ALL_CHECKS_PASS "$raw/native.log";echo "PASS ${1}"
