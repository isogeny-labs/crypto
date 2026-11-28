#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=$1;shift_arg=$2;raw="$here/runs/$attempt";donor="$here/runs/signed47-shift$shift_arg-01/classes"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/COFACTOR47.rs" "$raw/";cp "$0" "$raw/runner.sh"
cp "$here/source-data/evaluation-words.txt" "$raw/"
cp "$here/source-data/complementary-euler.sing" "$raw/"
cp "$donor/signed-first-basis.txt" "$raw/";cp "$donor/signed-second-basis.txt" "$raw/"
sha256sum "$raw/COFACTOR47.rs" "$raw/runner.sh" "$raw/"*.txt "$raw/"*.sing > "$raw/source.sha256"
printf '%s\n' "$shift_arg" > "$raw/shift.txt";git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc -O --edition=2021 "$raw/COFACTOR47.rs" -o "$raw/cofactor47") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/cofactor47" "$raw/evaluation-words.txt" "$raw/signed-first-basis.txt" "$raw/signed-second-basis.txt" "$raw/complementary-euler.sing" "$raw/classes") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
tar -cJf "$here/checkpoint-$attempt.tar.xz" -C "$here" "runs/$attempt"
[ "$rc" = 0 ];rg -qx COFACTOR47_ALL_CHECKS_PASS "$raw/native.log";echo "PASS $attempt"
