#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=$1;raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/JET47.rs" "$raw/";cp "$0" "$raw/runner.sh"
for f in source-polynomials.txt closed-points.txt evaluation-words.txt pairs2.txt pairs3.txt;do cp "$here/source-data/$f" "$raw/";done
cp "$here/source-data/original-sections.txt" "$raw/"
cp "$here/source-data/twist-sections.txt" "$raw/"
sha256sum "$raw/"*.rs "$raw/"*.txt "$raw/runner.sh" > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc -O --edition=2021 "$raw/JET47.rs" -o "$raw/jet47") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/jet47" "$raw/source-polynomials.txt" "$raw/original-sections.txt" "$raw/twist-sections.txt" "$raw/pairs2.txt" "$raw/pairs3.txt" "$raw/closed-points.txt" "$raw/evaluation-words.txt" "$raw/jets" "$raw/ranks.txt") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
tar -cJf "$here/checkpoint-$attempt.tar.xz" -C "$here" "runs/$attempt"
[ "$rc" = 0 ];rg -qx JET47_ALL_CHECKS_PASS "$raw/native.log";echo "PASS $attempt"
