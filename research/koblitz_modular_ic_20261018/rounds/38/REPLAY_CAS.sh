#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
attempt=${1:?fresh attempt};input=${2:?relative input};marker=${3:?marker};raw="$here/runs/$attempt"
[ ! -e "$raw" ];mkdir -p "$raw"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
cp "$here/$input" "$raw/input.sing";cp "$0" "$raw/runner.sh"
sha256sum "$raw/input.sing" "$raw/runner.sh" > "$raw/source.sha256"
git -C "$repo" rev-parse HEAD > "$raw/source-revision.txt"
date -u +%FT%TZ > "$raw/started-utc.txt"
set +e
(cd "$raw";ulimit -v 4194304;timeout 1200 /home/ec2-user/.cache/koblitz-native-cas/singular/bin/Singular -q input.sing) > "$raw/native.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt";date -u +%FT%TZ > "$raw/finished-utc.txt";set -e
tar -cJf "$here/checkpoint-$attempt.tar.xz" -C "$here" "runs/$attempt"
if [ "$rc" != 0 ] || rg -q '^ *\?' "$raw/native.log" || ! rg -qx "$marker" "$raw/native.log";then echo "FAIL $attempt native=$rc";exit 1;fi
echo "PASS $attempt"
