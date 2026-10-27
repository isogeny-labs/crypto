#!/usr/bin/env bash
# Thin native-CAS invocation and storage; all mathematics stays in Singular.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
repo=$(git -C "$here" rev-parse --show-toplevel)
name=${1:?choose ss-compact,ss-corrupt,ss-unit,z-components,z-smooth}
attempt=${2:?supply a fresh attempt basename}
mkdir -p "$here/runs"
base="$here/runs/$attempt"
[ ! -e "$base.sing" ] && [ ! -e "$base.log" ] || { echo 'fresh attempt required' >&2;exit 1; }
"$repo/scripts/checkpoint.sh" gate > "$base.gate.log" 2>&1
case "$name" in
 ss-unit) cp "$here/SS23-unit.sing" "$base.sing"; marker=SS23_UNIT_LIFT_ALL_CHECKS_PASS ;;
 ss-compact|ss-corrupt|z-components|z-smooth)
  awk '/^poly unitprod=1;/{exit} {print}' "$here/../22/runs/cm22-incidence.sing" > "$base.sing"
  case "$name" in
   ss-compact|ss-corrupt) cat "$here/SS23-certificate.sing" "$here/SS23-compact.sing" >> "$base.sing";marker=SS23_COMPACT_ALL_CHECKS_PASS ;;
   z-components|z-smooth)
    sed -n '2p' "$here/../22/runs/cm22-saturated.dump" >> "$base.sing"
    if [ "$name" = z-components ];then cat "$here/Z23-components.sing" >> "$base.sing";marker=Z23_COMPONENTS_ALL_CHECKS_PASS
    else cat "$here/Z23-smooth.sing" >> "$base.sing";marker=Z23_SOURCE_SMOOTH_ALL_CHECKS_PASS;fi ;;
  esac ;;
 *) echo 'unknown native case' >&2;exit 1 ;;
esac
if [ "$name" = ss-corrupt ];then sed -i 's/^poly inv=/poly inv=1+/' "$base.sing";fi
sha256sum "$base.sing" > "$base.source.sha256"
git -C "$repo" rev-parse HEAD > "$base.source-revision.txt"
date -u +%FT%TZ > "$base.started-utc.txt"
set +e
(ulimit -v 4194304;timeout 1200 "${SINGULAR_BIN:-/home/ec2-user/.cache/koblitz-native-cas/singular/bin/Singular}" -q "$base.sing") > "$base.log" 2>&1
native_rc=$?
set -e
printf '%s\n' "$native_rc" > "$base.exit"
date -u +%FT%TZ > "$base.finished-utc.txt"
# Previous ignored raw outputs must survive the NEXT checkpoint as well.
tar -cJf "$here/checkpoint-evidence.tar.xz" -C "$here" runs
if [ "$native_rc" -ne 0 ] || rg -q '^ *\?' "$base.log" || ! rg -qx "$marker" "$base.log";then
 echo "FAIL $attempt;native exit=$native_rc;see $base.log";exit 1
fi
printf 'PASS %s (%s)\n' "$attempt" "$marker"
