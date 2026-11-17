#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=$1;shift_arg=$2;raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/SIGNED48.rs" "$raw/";cp "$0" "$raw/runner.sh"
cp "$here/source-data/evaluation-words.txt" "$raw/";cp "$here/source-data/closed-points.txt" "$raw/"
mkdir "$raw/input-classes";cp "$here/../47/runs/couple47-01/divisors/"large-o1-d*-basis.txt "$raw/input-classes/"
sha256sum "$raw/SIGNED48.rs" "$raw/runner.sh" "$raw/evaluation-words.txt" "$raw/closed-points.txt" "$raw/input-classes/"* > "$raw/source.sha256"
printf '%s\n' "$shift_arg" > "$raw/shift.txt";git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc -O --edition=2021 "$raw/SIGNED48.rs" -o "$raw/signed48") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/signed48" "$raw/evaluation-words.txt" "$raw/input-classes" "$raw/closed-points.txt" "$raw/classes" "$shift_arg") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
tar -cJf "$here/checkpoint-$attempt.tar.xz" -C "$here" "runs/$attempt"
[ "$rc" = 0 ];rg -qx SIGNED48_ALL_CHECKS_PASS "$raw/native.log";echo "PASS $attempt"
