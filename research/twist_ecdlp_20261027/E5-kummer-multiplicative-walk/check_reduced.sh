#!/bin/bash
# Compares reduced/ against the full run: controls byte-identical; cell files identical after dropping the wall-clock fields (hdr done_ms and the per-trial ms field);
# operation-count rows (ops_table.gp) identical to out/opcounts.csv.
set -u
cd /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk
fail=0
strip() { grep -v 'done_ms' "$1" | sed -E 's/, [0-9]+, ([0-9]+)\]\);$/, \1]);/'; }
for f in curves identity chains exhaustive; do cmp -s raw/$f.txt reduced/raw/$f.txt && echo "same $f" || { echo "DIFF $f"; fail=1; }; done
for f in reduced/raw/cell_*.txt; do g=raw/$(basename $f)
  if diff <(strip $f) <(strip $g) > /dev/null; then echo "same $(basename $f)"; else echo "DIFF $(basename $f)"; fail=1; fi; done
RAWDIR=reduced/raw OUTDIR=reduced/out gp -q -f ops_table.gp < /dev/null > reduced/out/opcounts_pooled.txt 2> reduced/out/opcounts.err
while IFS= read -r row; do grep -qxF "$row" out/opcounts.csv && echo "opcount row ok: ${row%%,*}" || { echo "opcount row DIFF: $row"; fail=1; }; done < <(tail -n +2 reduced/out/opcounts.csv)
echo "reduced replication fail=$fail"
