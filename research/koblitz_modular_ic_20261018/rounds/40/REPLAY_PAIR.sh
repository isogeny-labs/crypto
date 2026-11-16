#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=$1;raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/PAIR40.rs" "$raw/";cp "$0" "$raw/runner.sh"
cp "$here/source-data/evaluation-words.txt" "$raw/"
mkdir "$raw/input-classes"
for f in large-o1-d1-basis.txt large-o2-d0-basis.txt;do cp "$here/runs/couple40-01/divisors/$f" "$raw/input-classes/";done
sha256sum "$raw/PAIR40.rs" "$raw/runner.sh" "$raw/evaluation-words.txt" "$raw/input-classes/"* > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc -O --edition=2021 "$raw/PAIR40.rs" -o "$raw/pair40") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/pair40" "$raw/evaluation-words.txt" "$raw/input-classes" "$raw/classes" "$raw/pair-result.txt") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
tar -cJf "$here/checkpoint-$attempt.tar.xz" -C "$here" "runs/$attempt"
[ "$rc" = 0 ];rg -qx PAIR40_ALL_CHECKS_PASS "$raw/native.log";echo "PASS $attempt"
