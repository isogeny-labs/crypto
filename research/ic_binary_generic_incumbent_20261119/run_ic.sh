#!/bin/bash
# IC arms through the example (counted cost; deterministic). Raw output stays under raw/ (gitignored).
cd "$(dirname "$0")"; EX=/home/user/crypto/target/release/examples/icbin_ksum_free; mkdir -p raw; sha256sum $EX > EXAMPLE.sha256
for b in 21 24 27; do for r in 1 2 3; do n=B$b-r$r; s=$(date +%s)
 RAYON_NUM_THREADS=1 $EX configs/$n.json raw/$n.jsonl; echo "$n rc=$? $(( $(date +%s)-s )) s"; done; done
