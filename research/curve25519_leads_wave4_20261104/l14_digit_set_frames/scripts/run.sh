#!/usr/bin/env bash
# L14 digit-set frames: exactly the runs planned in PROTOCOL.md section 3.
# Pure Python, one light process, no engine, no ecbench, no bench lock.
# Deterministic (seeds 20261104+t for part a; 20261104+t+1000*(rep+1) for greedy
# replicate rep), so a container restart is handled by re-running this script;
# each k20_t*.json is written atomically and complete files are skipped.
set -euo pipefail
LEAD="$(cd "$(dirname "$0")/.." && pwd)"
DATA="$LEAD/data"
mkdir -p "$DATA"
cd "$LEAD"

# Family miniature k=20 (family rule): A=926, p=1048573, r=131203, n=8r.
K=20; P=1048573; A=926; N=1049624
TS=()
for t in 4 5 6; do
  [ -s "$DATA/k${K}_t${t}.json" ] || TS+=("$t")
done
if [ "${#TS[@]}" -gt 0 ]; then
  python3 -I "$LEAD/scripts/digit_census.py" --k "$K" --p "$P" --A "$A" --n "$N" \
    --t "${TS[@]}" --K 2000 --B 2000 --R 5 --seed-base 20261104 \
    --out-dir "$DATA" 2>&1 | tee -a "$DATA/run.log"
fi
sha256sum "$LEAD/scripts/digit_census.py" "$LEAD/scripts/analyse.py" \
  "$LEAD/../../curve25519_frame_search_20261103/scripts/frame_census.py" \
  "$DATA"/k${K}_t*.json > "$DATA/SHA256SUMS"
python3 -I "$LEAD/scripts/analyse.py" | tee "$DATA/analysis.txt"
