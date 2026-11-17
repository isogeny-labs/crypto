#!/usr/bin/env bash
# EXP-PKM-IDEAS-001 Amendment 1: idea-1 m=2 dense cells at S>=64 with --dmax 1024 (rows in runs/ext/).
set -uo pipefail
cd "$(dirname "$0")"
ROOT="$(git rev-parse --show-toplevel)"
BIN="${PKM_IDEAS_BIN:-$ROOT/target/release/examples/pkm_ideas}"
export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-4}"
P1=1399680001
mkdir -p runs/ext; sha256sum "$BIN" > runs/ext/binary.sha256; git -C "$ROOT" rev-parse HEAD > runs/ext/commit.txt
for S in 64 128 256; do for v in random coset; do
  f="runs/ext/dense-m2-$v-S$S.jsonl"; [ -f "$f" ] && [ "$(wc -l < "$f")" -ge 3 ] && continue
  t0=$(date +%s)
  "$BIN" dense --p $P1 --m 2 --S $S --vkind $v --draws 3 --dmax 1024 > "$f" 2> "${f%.jsonl}.err" || echo "failed $f" >> runs/ext/progress.txt
  echo "done $f rows=$(wc -l < "$f") secs=$(( $(date +%s) - t0 ))" >> runs/ext/progress.txt
done; done
echo "end $(date -u +%FT%TZ)" >> runs/ext/progress.txt
