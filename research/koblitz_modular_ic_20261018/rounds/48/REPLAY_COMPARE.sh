#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=$1;raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/COMPARE48.rs" "$raw/";cp "$0" "$raw/runner.sh";cp "$here/source-data/evaluation-words.txt" "$raw/"
mkdir "$raw/input-old" "$raw/input-new"
cp "$here/../41/runs/split41-01/classes/component-seven-basis.txt" "$raw/input-old/P7-basis.txt"
cp "$here/../41/runs/split41-01/classes/component-eight-basis.txt" "$raw/input-old/P8-basis.txt"
cp "$here/runs/split48-01/input-classes/F-basis.txt" "$raw/input-new/"
cp "$here/runs/split48-01/input-classes/G-basis.txt" "$raw/input-new/"
cp "$here/runs/split48-01/classes/component-seven-basis.txt" "$raw/input-new/P7-basis.txt"
cp "$here/runs/split48-01/classes/component-eight-basis.txt" "$raw/input-new/P8-basis.txt"
sha256sum "$raw/COMPARE48.rs" "$raw/runner.sh" "$raw/evaluation-words.txt" "$raw/input-old/"* "$raw/input-new/"* > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc -O --edition=2021 "$raw/COMPARE48.rs" -o "$raw/compare48") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/compare48" "$raw/evaluation-words.txt" "$raw/input-old" "$raw/input-new" "$raw/classes" "$raw/comparison-result.txt") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
tar -cJf "$here/checkpoint-$attempt.tar.xz" -C "$here" "runs/$attempt"
[ "$rc" = 0 ];rg -qx COMPARE48_ALL_CHECKS_PASS "$raw/native.log";echo "PASS $attempt"
