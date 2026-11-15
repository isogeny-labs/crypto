#!/usr/bin/env bash
# EXP-PKM-DENSE-DEC-001 runner (PROTOCOL.md). One jsonl per cell under runs/; completed cells are skipped.
set -uo pipefail
cd "$(dirname "$0")"
ROOT="$(git rev-parse --show-toplevel)"
BIN="${PKM_ORACLE_BIN:-$ROOT/target/release/examples/pkm_algebraic_oracle}"
export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-4}"
P1=1399680001; P2=3265920001
sha256sum "$BIN" > binary.sha256; git -C "$ROOT" rev-parse HEAD > commit.txt
mkdir -p runs
deg() { local s="" i; for i in $(seq 1 "$1"); do s="$s,2"; done; echo "${s#,}"; }
cell() {  # name p m t membership vkind
  local name="$1" p="$2" m="$3" t="$4" mem="$5" vk="$6" f="runs/$1.jsonl"
  if [ -f "$f" ] && [ "$(wc -l < "$f")" -ge 6 ]; then return 0; fi
  rm -f "$f"; local t0; t0=$(date +%s)
  if ! "$BIN" --p "$p" --m "$m" --degrees "$(deg "$t")" --membership "$mem" --vkind "$vk" \
      --planted 4 --random 2 --seed 20261114 --dmax 1024 --budget 1800 --cell "$name" \
      > "$f" 2> "runs/$name.err"; then echo "failed $name" >> progress.txt; fi
  echo "done $name rows=$(wc -l < "$f") secs=$(( $(date +%s) - t0 ))" >> progress.txt
}
for t in 4 5 6 7 8; do cell "m2-t$t-both" $P1 2 $t both coset; cell "m2-t$t-random" $P1 2 $t dense random; done
for t in 2 3 4; do cell "m3-t$t-both" $P1 3 $t both coset; cell "m3-t$t-random" $P1 3 $t dense random; done
cell "m3-t5-dense" $P1 3 5 dense coset; cell "m3-t5-random" $P1 3 5 dense random
cell "rep-P2-m2-t6-both" $P2 2 6 both coset; cell "rep-P2-m3-t3-both" $P2 3 3 both coset
echo "end $(date -u +%FT%TZ)" >> progress.txt
