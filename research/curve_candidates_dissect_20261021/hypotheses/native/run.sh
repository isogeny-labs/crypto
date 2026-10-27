#!/bin/sh
# Thin build/run orchestration; all research arithmetic uses the PARI C library.
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
: "${PARI_PREFIX:?set PARI_PREFIX to the native PARI installation}"
out=${COMPARE_OUT:-"$here/records"}
mkdir -p "$out"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror \
  -I"$PARI_PREFIX/include" "$here/native/compare.cpp" \
  -L"$PARI_PREFIX/lib" -Wl,-rpath,"$PARI_PREFIX/lib" -lpari -lcrypto \
  -o "$out/compare"
for row in 0 1 2 3 4 5; do
  if timeout 90 "$out/compare" "$here/inputs.tsv" "$row" \
      > "$out/curve-$row.json" 2> "$out/curve-$row.stderr"; then
    printf '%s\n' "row $row: complete"
  else
    status=$?
    printf '%s\n' "row $row: exit $status" >&2
    exit "$status"
  fi
done
