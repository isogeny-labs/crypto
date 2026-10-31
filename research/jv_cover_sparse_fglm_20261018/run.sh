#!/usr/bin/env bash
# Sparse-FGLM linear stage of jv_cover: A/B/C arms on ONE binary, selected by
# JV_LIN = dense (the old stage) | l1a (dense Hessenberg charpoly of M_xn +
# chain recovery) | sparse (Wiedemann/BM of M_xn + chain recovery, default).
#
#   run.sh <jv_cover binary> <outdir>
set -uo pipefail
BIN=$1; OUT=$2
mkdir -p "$OUT/raw"
{
  echo "host $(lscpu | sed -n 's/^Model name: *//p') nproc $(nproc)"
  echo "jv_cover sha256 $(sha256sum "$BIN" | cut -c1-64)"
} > "$OUT/raw/versions.txt"
now_ms() { echo $(( $(date +%s%N) / 1000000 )); }
run() { # arm label args...
  local arm=$1 label=$2; shift 2
  local a; a=$(now_ms)
  JV_LIN=$arm "$BIN" "$@" --json "$OUT/raw/${label}_${arm}.json" > "$OUT/raw/${label}_${arm}.log" 2>&1
  echo "$label $arm exit $? wall_ms $(( $(now_ms) - a ))" >> "$OUT/raw/walls.txt"
}
for arm in dense l1a sparse; do
  run $arm ccov --exp ccov --sizes 53,101,251 --seeds 1 --residuals 100 --constructed 20 --max-degree 24 --budget 300
done
for arm in dense sparse l1a; do
  run $arm dlp --exp dlp --sizes 53,101 --seeds 2 --rho-runs 4 --check-every 64
done
