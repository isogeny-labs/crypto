#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=${1:?fresh attempt};raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/GROUP36.rs" "$raw/";cp "$here/../35/runs/jets35-01/evaluation-words.txt" "$raw/";cp "$here/../35/runs/inflate35-01/products/coupled-r1-f2-basis.txt" "$raw/";cp "$0" "$raw/runner.sh"
sha256sum "$raw/GROUP36.rs" "$raw/evaluation-words.txt" "$raw/coupled-r1-f2-basis.txt" "$raw/runner.sh" > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc -O --edition=2021 "$raw/GROUP36.rs" -o "$raw/group36") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/group36" "$raw/evaluation-words.txt" "$raw/coupled-r1-f2-basis.txt" "$raw/quotients") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
tar -cJf "$here/checkpoint-$attempt.tar.xz" -C "$here" "runs/$attempt"
[ "$rc" = 0 ];rg -qx GROUP36_ALL_CHECKS_PASS "$raw/native.log";echo "PASS $attempt"
