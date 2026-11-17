#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
: "${PARI_PREFIX:?set PARI_PREFIX}"
: "${SKIP_SEARCH_OUT:?set a new output directory outside frozen records}"
mkdir -p "$SKIP_SEARCH_OUT"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror \
  -I"$PARI_PREFIX/include" "$here/native/features.cpp" \
  -L"$PARI_PREFIX/lib" -Wl,-rpath,"$PARI_PREFIX/lib" -lpari -lcrypto \
  -o "$SKIP_SEARCH_OUT/features"
for row in $(seq 0 20); do
  prefix="$SKIP_SEARCH_OUT/curve-$row"
  if timeout 120 "$SKIP_SEARCH_OUT/features" "$here/inputs.tsv" "$row" "$prefix" \
      > "$prefix.stdout" 2> "$prefix.stderr"; then
    cat "$prefix.stdout"
  else
    status=$?
    printf '%s\n' "row $row exited $status" >&2
    exit "$status"
  fi
done
