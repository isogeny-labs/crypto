#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=${1:?fresh attempt};raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/TORSION37.rs" "$raw/";cp "$here/../35/runs/jets35-01/evaluation-words.txt" "$raw/";cp "$here/runs/signed37-01/classes/signed-first-basis.txt" "$raw/";cp "$here/runs/signed37-01/classes/signed-second-basis.txt" "$raw/";cp "$0" "$raw/runner.sh"
sha256sum "$raw/TORSION37.rs" "$raw/evaluation-words.txt" "$raw/signed-first-basis.txt" "$raw/signed-second-basis.txt" "$raw/runner.sh" > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc -O --edition=2021 "$raw/TORSION37.rs" -o "$raw/torsion37") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/torsion37" "$raw/evaluation-words.txt" "$raw/signed-first-basis.txt" "$raw/signed-second-basis.txt" "$raw/classes") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
tar -cJf "$here/checkpoint-$attempt.tar.xz" -C "$here" "runs/$attempt"
[ "$rc" = 0 ];rg -qx TORSION37_ALL_CHECKS_PASS "$raw/native.log";echo "PASS $attempt"
