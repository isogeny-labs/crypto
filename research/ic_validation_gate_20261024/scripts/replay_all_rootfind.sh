#!/bin/sh
# Current-binary `ecbench verify --replay-all` of the root-finder session (the 168-row session the board's
# "96 mismatches" statement is about). Receipt written to OUT; the session directory is read only.
# usage: replay_all_rootfind.sh <ecbench> <outdir>   (repo root)
EB="$1"; OUT="$2"; mkdir -p "$OUT"
S=research/ecbench_rootfind_engine_20261012/sessions/claim-n13
[ -s "$OUT/claim-n13.receipt.json" ] || "$EB" verify --dir "$S" --replay-all --out "$OUT/claim-n13.receipt.json" > "$OUT/claim-n13.stdout" 2> "$OUT/claim-n13.stderr"
echo exit=$?
