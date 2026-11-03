#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
: "${SKIP_FACTOR_OUT:?set a new factor output directory}"
: "${SKIP_FACTOR_BINARY:?set compiled native factor_target path}"
: "${SKIP_FACTOR_INPUT:=$here/inputs.tsv}"
export SKIP_FACTOR_INPUT SKIP_FACTOR_OUT SKIP_FACTOR_BINARY
mkdir -p "$SKIP_FACTOR_OUT"
for row in $(seq 0 20); do
  for target in minus plus twist disc; do
    printf '%s %s\n' "$row" "$target"
  done
done | xargs -n 2 -P 2 sh -c '
  row=$1
  target=$2
  prefix="$SKIP_FACTOR_OUT/curve-$row-$target"
  set +e
  timeout 30 "$SKIP_FACTOR_BINARY" "$SKIP_FACTOR_INPUT" "$row" "$target" > "$prefix.json" 2> "$prefix.stderr"
  status=$?
  printf "%s\n" "$status" > "$prefix.status"
' worker
