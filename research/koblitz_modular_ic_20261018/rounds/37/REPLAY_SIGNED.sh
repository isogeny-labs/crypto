#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=${1:?fresh attempt};raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/SIGNED37.rs" "$raw/";cp "$here/../35/runs/jets35-01/evaluation-words.txt" "$raw/";cp "$here/runs/order37-01/deck-order.txt" "$raw/";mkdir "$raw/divisors"
cp "$here/../35/runs/inflate35-01/products/"*-basis.txt "$raw/divisors/";cp "$0" "$raw/runner.sh"
sha256sum "$raw/SIGNED37.rs" "$raw/evaluation-words.txt" "$raw/deck-order.txt" "$raw/runner.sh" "$raw/divisors/"* > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc -O --edition=2021 "$raw/SIGNED37.rs" -o "$raw/signed37") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/signed37" "$raw/evaluation-words.txt" "$raw/divisors" "$raw/deck-order.txt" "$raw/classes") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
tar -cJf "$here/checkpoint-$attempt.tar.xz" -C "$here" "runs/$attempt"
[ "$rc" = 0 ];rg -qx SIGNED37_ALL_CHECKS_PASS "$raw/native.log";echo "PASS $attempt"
