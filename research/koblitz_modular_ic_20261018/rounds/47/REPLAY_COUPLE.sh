#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=$1;raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/COUPLE47.rs" "$raw/";cp "$0" "$raw/runner.sh"
cp "$here/source-data/evaluation-words.txt" "$raw/"
cp "$here/source-data/closed-points.txt" "$raw/"
cp -r "$here/runs/jets47-01/jets" "$raw/input-jets"
sha256sum "$raw/COUPLE47.rs" "$raw/runner.sh" "$raw/"*.txt "$raw/input-jets/"* > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc -O --edition=2021 "$raw/COUPLE47.rs" -o "$raw/couple47") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/couple47" "$raw/evaluation-words.txt" "$raw/closed-points.txt" "$raw/input-jets" "$raw/divisors") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
tar -cJf "$here/checkpoint-$attempt.tar.xz" -C "$here" "runs/$attempt"
[ "$rc" = 0 ];rg -qx COUPLE47_ALL_CHECKS_PASS "$raw/native.log";echo "PASS $attempt"
