#!/usr/bin/env bash
# Inspect retained tool outcomes and hashes; never rerun native calculations.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
check_case() {
 local attempt=$1 marker=$2 expected=$3 base="$here/runs/$1" observed=reject
 local expected_hash actual_hash
 expected_hash=$(awk '{print $1}' "$base/source.sha256")
 actual_hash=$(sha256sum "$base/input.sing" | awk '{print $1}')
 [ "$expected_hash" = "$actual_hash" ] || { echo "source hash mismatch: $attempt";exit 1; }
 if [ "$(cat "$base/exit.txt")" = 0 ] && ! rg -q '^ *\?' "$base/native.log" && rg -qx "$marker" "$base/native.log";then observed=accept;fi
 [ "$observed" = "$expected" ] || { echo "unexpected outcome: $attempt $observed";exit 1; }
 printf '%s %s\n' "$attempt" "$observed"
}
check_case cf24-factor-01 CF24_FACTORIZATION_ALL_CHECKS_PASS accept
check_case o24-charts-01 O24_CHARTS_ALL_CHECKS_PASS reject
check_case o24-charts-02 O24_CHARTS_ALL_CHECKS_PASS accept
check_case o24-series-01 O24_SERIES_ALL_CHECKS_PASS reject
check_case o24-series-02 O24_SERIES_ALL_CHECKS_PASS accept
check_case cf24-corrupt-01 CF24_FACTORIZATION_ALL_CHECKS_PASS reject
[ "$(rg -c '^BRANCH ' "$here/runs/o24-series-02/native.log")" = 4 ]
[ "$(rg -c '^ORDER unit' "$here/runs/o24-series-02/native.log")" = 32 ]
[ "$(cat "$here/runs/lean-01/exit.txt")" = 1 ]
[ "$(cat "$here/runs/lean-02/exit.txt")" = 0 ]
rg -qx 'audited 14 theorems in \[KoblitzRound24Geometry\]; 0 failing, 0 trusted \(native_decide\)' "$here/runs/lean-02/compile.log"
printf 'RETAINED_OUTCOMES_AND_SOURCE_HASHES_PASS\n'
