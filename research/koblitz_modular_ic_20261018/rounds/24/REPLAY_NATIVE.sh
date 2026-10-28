#!/usr/bin/env bash
# Thin native invocation/storage only. Arithmetic stays in Singular.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
repo=$(git -C "$here" rev-parse --show-toplevel)
name=${1:?choose cf-factor}
attempt=${2:?fresh attempt name required}
base="$here/runs/$attempt"
mkdir -p "$here/runs"
[ ! -e "$base" ] || { echo 'fresh attempt required' >&2; exit 1; }
mkdir "$base"
"$repo/scripts/checkpoint.sh" gate > "$base/gate.log" 2>&1
case "$name" in
 cf-factor|cf-corrupt)
  printf 'ring r4=(2,a),(u,l),dp;minpoly=a^2+a+1;\n' > "$base/input.sing"
  sed -n '/^poly f=/p' "$here/../22/runs/cm22-plane-places.sing" >> "$base/input.sing"
  cat "$here/CF24.sing" >> "$base/input.sing"
  marker=CF24_FACTORIZATION_ALL_CHECKS_PASS
  if [ "$name" = cf-corrupt ];then sed -i 's/print("PRODUCT_RESIDUAL");product-f;/product=product+1;print("PRODUCT_RESIDUAL");product-f;/' "$base/input.sing";fi ;;
 o-charts)
  cp "$here/O24-charts.sing" "$base/input.sing"
  marker=O24_CHARTS_ALL_CHECKS_PASS ;;
 *) echo 'unknown case' >&2; exit 1 ;;
esac
sha256sum "$base/input.sing" > "$base/source.sha256"
git -C "$repo" rev-parse HEAD > "$base/source-revision.txt"
date -u +%FT%TZ > "$base/started-utc.txt"
set +e
(cd "$base";ulimit -v 4194304;timeout 1200 "${SINGULAR_BIN:-/home/ec2-user/.cache/koblitz-native-cas/singular/bin/Singular}" -q input.sing) > "$base/native.log" 2>&1
native_rc=$?
set -e
printf '%s\n' "$native_rc" > "$base/exit.txt"
date -u +%FT%TZ > "$base/finished-utc.txt"
tar -cJf "$here/checkpoint-evidence.tar.xz" -C "$here" runs
if [ "$native_rc" -ne 0 ] || rg -q '^ *\?' "$base/native.log" || ! rg -qx "$marker" "$base/native.log";then
 echo "FAIL $attempt; native exit=$native_rc; see $base/native.log";exit 1
fi
printf 'PASS %s (%s)\n' "$attempt" "$marker"
