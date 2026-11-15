#!/usr/bin/env bash
# EXP-PKM-IDEAS-001 runner (PROTOCOL.md). One jsonl per cell under runs/; completed cells are skipped.
set -uo pipefail
cd "$(dirname "$0")"
ROOT="$(git rev-parse --show-toplevel)"
BIN="${PKM_IDEAS_BIN:-$ROOT/target/release/examples/pkm_ideas}"
export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-4}"
P1=1399680001; P2=3265920001; PC=1001089; PA=1000621
sha256sum "$BIN" > binary.sha256; git -C "$ROOT" rev-parse HEAD > commit.txt
mkdir -p runs
cell() {  # name expected-rows args...
  local name="$1" rows="$2"; shift 2; local f="runs/$name.jsonl"
  if [ -f "$f" ] && [ "$(wc -l < "$f")" -ge "$rows" ]; then return 0; fi
  rm -f "$f"; local t0; t0=$(date +%s)
  if ! "$BIN" "$@" > "$f" 2> "runs/$name.err"; then echo "failed $name (exit $?)" >> progress.txt; fi
  echo "done $name rows=$(wc -l < "$f") secs=$(( $(date +%s) - t0 ))" >> progress.txt
}
deg() { local s="" i; for i in $(seq 1 "$2"); do s="$s,$1"; done; echo "${s#,}"; }

# idea 2 (exact, fast)
for S in 18 36; do cell "aut-S$S" 1 aut --p $PA --S $S --targets 2000; done
# idea 5b (exact, fast)
for S in 16 32 64; do cell "coverage-S$S" 1 coverage --p $PC --S $S; done
# idea 5a
for t in 4 5 6 7 8; do cell "cosets-t$t" 6 cosets --p $P1 --degrees "$(deg 2 $t)" --draws 3; done
# ideas 3 and 4
for t in 4 5 6 7 8; do cell "replay-m2-A2-t$t" 5 replay --p $P1 --m 2 --degrees "$(deg 2 $t)" --draws 3; done
for k in 3 4 5; do cell "replay-m2-A3-k$k" 5 replay --p $P1 --m 2 --degrees "$(deg 3 $k)" --draws 3; done
for t in 2 3 4; do cell "replay-m3-A2-t$t" 5 replay --p $P1 --m 3 --degrees "$(deg 2 $t)" --draws 3; done
cell "xcurve-m2-A2-t6" 5 replay --p $P1 --m 2 --degrees "$(deg 2 6)" --draws 3 --replay-seed 77
cell "xcurve-m2-A3-k4" 5 replay --p $P1 --m 2 --degrees "$(deg 3 4)" --draws 3 --replay-seed 77
cell "xcurve-m3-A2-t3" 5 replay --p $P1 --m 3 --degrees "$(deg 2 3)" --draws 3 --replay-seed 77
cell "xprime-m2-A2-t6" 5 replay --p $P1 --m 2 --degrees "$(deg 2 6)" --draws 3 --replay-p $P2
cell "xprime-m2-A3-k4" 5 replay --p $P1 --m 2 --degrees "$(deg 3 4)" --draws 3 --replay-p $P2
cell "xprime-m3-A2-t3" 5 replay --p $P1 --m 3 --degrees "$(deg 2 3)" --draws 3 --replay-p $P2
# idea 1 (dense membership); largest last
for S in 4 8 16; do for v in random coset; do cell "dense-m3-$v-S$S" 3 dense --p $P1 --m 3 --S $S --vkind $v --draws 3; done; done
for S in 16 32 64 128 256; do for v in random coset; do cell "dense-m2-$v-S$S" 3 dense --p $P1 --m 2 --S $S --vkind $v --draws 3; done; done
echo "end $(date -u +%FT%TZ)" >> progress.txt
