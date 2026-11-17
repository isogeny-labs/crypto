#!/bin/sh
# Q1 exhaustive cells (PROTOCOL.md "Measurements", Q1). Each cell <= 1 h wall,
# at most 2 in parallel. Usage: run_exhaustive.sh BIN OUTDIR
BIN="$1"; OUT="$2"
mkdir -p "$OUT"; cd "$OUT" || exit 1
run() { p=$1; tag=$2; shift 2
  /usr/bin/time -l timeout 3600 nice -n 10 "$BIN" exhaustive --p "$p" --out "$OUT" "$@" > "log_exh_$p$tag.txt" 2>&1
  echo "exit=$?" >> "log_exh_$p$tag.txt"; }
run 7 _unreduced --graph3 --unreduced & run 7 "" --graph3 --graph-json & wait
run 11 "" --graph3 --graph-json & run 13 "" --graph3 --graph-json & wait
run 17 "" --graph3 --graph-json & run 19 "" --graph3 --graph-json & wait
run 23 "" --graph3 --graph-json & run 29 "" --graph3 --graph-json & wait
run 31 "" --graph3 --graph-json
echo ALLDONE
