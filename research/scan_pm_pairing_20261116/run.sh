#!/bin/sh
# Thin run orchestration for the measured rows (PROTOCOL.md §4). At most two
# processes at a time, each capped at 590 s. Usage:
#   T=<target dir> OUT=<new raw dir> sh research/scan_pm_pairing_20261116/run.sh
set -u
cd "$(dirname "$0")/../.."
B=$T/scan_pm
SEED=20261116
test ! -e "$OUT" || { echo "OUT exists"; exit 1; }
mkdir -p "$OUT"
sha256sum "$B" > "$OUT/binary.sha256"
timeout 590 "$B" run --cells 1:23:8,1:29:10,0:31:11,0:37:13,0:41:14,1:47:16 --seed $SEED --reps 3 --out "$OUT" 2> "$OUT/log-small.txt" &
P1=$!
timeout 590 "$B" run --cells 0:53:18 --seed $SEED --reps 3 --out "$OUT" 2> "$OUT/log-53.txt" &
P2=$!
wait $P1; echo "small: exit $?" >> "$OUT/exit.txt"
wait $P2; echo "53: exit $?" >> "$OUT/exit.txt"
timeout 590 "$B" run --cells 1:59:20 --seed $SEED --reps 2 --yes 0 --shard 0/2 --out "$OUT" 2> "$OUT/log-59-0.txt" &
P1=$!
timeout 590 "$B" run --cells 1:59:20 --seed $SEED --reps 2 --yes 0 --shard 1/2 --out "$OUT" 2> "$OUT/log-59-1.txt" &
P2=$!
wait $P1; echo "59 shard 0: exit $?" >> "$OUT/exit.txt"
wait $P2; echo "59 shard 1: exit $?" >> "$OUT/exit.txt"
"$B" summarize --dir "$OUT" --out research/scan_pm_pairing_20261116 > /dev/null
cat "$OUT/exit.txt"
