#!/usr/bin/env bash
# Amendment 2, arm L. Outputs data/dlp_p*.jsonl.
set -euo pipefail
cd "$(dirname "$0")"
B=tool/target/release/ic-weakpoints
for p in 127 251 509 1021; do $B dlp --p $p --curves 2 --seed 20261104 > data/dlp_p$p.jsonl & done
$B dlp --p 2039 --c0 0 --curves 1 --seed 20261104 > data/dlp_p2039_c0.jsonl &
$B dlp --p 2039 --c0 1 --curves 1 --seed 20261104 > data/dlp_p2039_c1.jsonl &
wait
echo done
