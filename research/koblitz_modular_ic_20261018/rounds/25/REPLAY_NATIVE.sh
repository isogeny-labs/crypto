#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
repo=$(git -C "$here" rev-parse --show-toplevel)
name=${1:?positive or corrupt}
attempt=${2:?fresh basename}
base="$here/runs/$attempt"
[ ! -e "$base" ] || { echo 'fresh attempt required';exit 1; }
mkdir -p "$base"
"$repo/scripts/checkpoint.sh" gate > "$base/gate.log" 2>&1
awk '/^poly unitprod=1;/{exit} {print}' "$here/../22/runs/cm22-incidence.sing" > "$base/input.sing"
sed -n 's/^generic\[2\]=/poly nuEquation=/p' "$here/../22/runs/cm22-generic-qr-01.log" | sed 's/$/;/' >> "$base/input.sing"
marker=DECK25_ALL_CHECKS_PASS
if [ "$name" = quotient ];then cat "$here/Q25.sing" >> "$base/input.sing";marker=Q25_REGISTERED_QUOTIENT_ALL_CHECKS_PASS
else cat "$here/DECK25.sing" >> "$base/input.sing";fi
case "$name" in
 positive|quotient) ;;
 corrupt) sed -i 's/print("KERNEL_COEFFICIENT_INVARIANCE");/shiftedKernel[2]=shiftedKernel[2]+1;print("KERNEL_COEFFICIENT_INVARIANCE");/' "$base/input.sing" ;;
 *) exit 1 ;;
esac
sha256sum "$base/input.sing" > "$base/source.sha256"
git -C "$repo" rev-parse HEAD > "$base/source-revision.txt"
date -u +%FT%TZ > "$base/started-utc.txt"
set +e
(cd "$base";ulimit -v 4194304;timeout 1200 "${SINGULAR_BIN:-/home/ec2-user/.cache/koblitz-native-cas/singular/bin/Singular}" -q input.sing) > "$base/native.log" 2>&1
rc=$?
set -e
printf '%s\n' "$rc" > "$base/exit.txt"
date -u +%FT%TZ > "$base/finished-utc.txt"
tar -cJf "$here/checkpoint-live.tar.xz" -C "$here" runs
if [ "$rc" != 0 ] || rg -q '^ *\?' "$base/native.log" || ! rg -qx "$marker" "$base/native.log";then echo "FAIL $attempt native=$rc";exit 1;fi
echo "PASS $attempt"
