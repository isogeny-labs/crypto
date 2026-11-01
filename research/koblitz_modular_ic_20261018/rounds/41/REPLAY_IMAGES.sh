#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=$1;raw="$here/runs/$attempt"
rg -qx 'gate=true' "$here/runs/split41-01/split-result.txt"
rg -qx SPLIT41_ALL_CHECKS_PASS "$here/runs/split41-01/native.log"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/IMAGES41.rs" "$raw/";cp "$0" "$raw/runner.sh"
cp "$here/source-data/evaluation-words.txt" "$raw/";cp "$here/source-data/old-R-basis.txt" "$raw/";cp "$here/source-data/complementary-euler.sing" "$raw/"
mkdir "$raw/input-classes"
cp "$here/runs/cofactor41-shift0-01/classes/cofactor-basis.txt" "$raw/input-classes/F-basis.txt"
cp "$here/runs/cofactor41-shift1-01/classes/cofactor-basis.txt" "$raw/input-classes/G-basis.txt"
cp "$here/runs/split41-01/classes/component-seven-basis.txt" "$raw/input-classes/P7-basis.txt"
cp "$here/runs/split41-01/classes/component-eight-basis.txt" "$raw/input-classes/P8-basis.txt"
sha256sum "$raw/IMAGES41.rs" "$raw/runner.sh" "$raw/"*.txt "$raw/"*.sing "$raw/input-classes/"* > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt";rustc --version > "$raw/version.txt"
set +e
(ulimit -v 4194304;timeout 1200 rustc -O --edition=2021 "$raw/IMAGES41.rs" -o "$raw/images41") > "$raw/build.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/build.exit"
if [ "$rc" = 0 ];then
 date -u +%FT%TZ > "$raw/started-utc.txt"
 (ulimit -v 4194304;timeout 1200 "$raw/images41" "$raw/evaluation-words.txt" "$raw/input-classes" "$raw/old-R-basis.txt" "$raw/complementary-euler.sing" "$raw/classes" "$raw/image-result.txt") > "$raw/native.log" 2>&1
 rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt"
fi
set -e
tar -cJf "$here/checkpoint-$attempt.tar.xz" -C "$here" "runs/$attempt"
[ "$rc" = 0 ];rg -qx IMAGES41_ALL_CHECKS_PASS "$raw/native.log";echo "PASS $attempt"
