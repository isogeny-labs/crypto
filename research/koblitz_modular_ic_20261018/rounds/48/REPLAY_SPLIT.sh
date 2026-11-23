#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=$1;raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/SPLIT48.rs" "$raw/";cp "$0" "$raw/runner.sh";cp "$here/source-data/evaluation-words.txt" "$raw/"
mkdir "$raw/input-classes"
cp "$here/../47/runs/cofactor47-shift0-01/classes/cofactor-basis.txt" "$raw/input-classes/F-basis.txt"
cp "$here/runs/cofactor48-shift1-01/classes/cofactor-basis.txt" "$raw/input-classes/G-basis.txt"
cp "$here/runs/cofactor48-shift2-01/classes/cofactor-basis.txt" "$raw/input-classes/H-basis.txt"
sha256sum "$raw/SPLIT48.rs" "$raw/runner.sh" "$raw/evaluation-words.txt" "$raw/input-classes/"* > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc -O --edition=2021 "$raw/SPLIT48.rs" -o "$raw/split48") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/split48" "$raw/evaluation-words.txt" "$raw/input-classes" "$raw/classes" "$raw/split-result.txt") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
tar -cJf "$here/checkpoint-$attempt.tar.xz" -C "$here" "runs/$attempt"
[ "$rc" = 0 ];rg -qx SPLIT48_ALL_CHECKS_PASS "$raw/native.log";echo "PASS $attempt"
