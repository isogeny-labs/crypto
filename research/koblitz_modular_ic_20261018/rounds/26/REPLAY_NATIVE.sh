#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd);repo=$(git -C "$here" rev-parse --show-toplevel)
mode=${1:?char,corrupt,charts};attempt=${2:?fresh basename};base="$here/runs/$attempt"
[ ! -e "$base" ];mkdir -p "$base"
"$repo/scripts/checkpoint.sh" gate > "$base/gate.log" 2>&1
case "$mode" in
 char|corrupt) cp "$here/CHAR26.sing" "$base/input.sing";marker=CHAR26_ALL_CHECKS_PASS
 if [ "$mode" = corrupt ];then sed -i 's/int delta=34;/int delta=1;/' "$base/input.sing";fi ;;
 source-orbit) cp "$here/SOURCEORBIT26.sing" "$base/input.sing";marker=SOURCEORBIT26_ALL_CHECKS_PASS ;;
 transport) cp "$here/TRANSPORT26.sing" "$base/input.sing";marker=TRANSPORT26_ALL_CHECKS_PASS ;;
 charts) cp "$here/CD26.sing" "$base/input.sing";marker=CD26_CHARTS_ALL_CHECKS_PASS ;;
 *) exit 1 ;;
esac
sha256sum "$base/input.sing" > "$base/source.sha256";git -C "$repo" rev-parse HEAD > "$base/source-revision.txt"
date -u +%FT%TZ > "$base/started-utc.txt"
set +e
(cd "$base";ulimit -v 4194304;timeout 1200 /home/ec2-user/.cache/koblitz-native-cas/singular/bin/Singular -q input.sing) > "$base/native.log" 2>&1
rc=$?;set -e
printf '%s\n' "$rc" > "$base/exit.txt";date -u +%FT%TZ > "$base/finished-utc.txt"
tar -cJf "$here/checkpoint-live.tar.xz" -C "$here" runs
if [ "$rc" != 0 ] || rg -q '^ *\?' "$base/native.log" || ! rg -qx "$marker" "$base/native.log";then echo "FAIL $attempt native=$rc";exit 1;fi
echo "PASS $attempt"
