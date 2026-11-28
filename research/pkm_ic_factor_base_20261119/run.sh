#!/usr/bin/env bash
# EXP-PKM-IC-001: the protocol's cells. A cell with 18 rows (6 targets x 3 arms) is skipped.
set -euo pipefail
cd "$(dirname "$0")"; mkdir -p runs
B=../../target/release/examples/pkm_ic_factor_base
sha256sum "$B" | sed 's#  .*#  pkm_ic_factor_base#' > runs/binary.sha256
for cell in "20 8" "20 9" "24 9" "24 10" "28 9" "28 10"; do
  set -- $cell; f="runs/b$1-t$2.jsonl"
  if [ -f "$f" ] && [ "$(wc -l < "$f")" -ge 18 ]; then continue; fi
  rm -f "$f"
  (cd ../.. && timeout 21600 "$OLDPWD/$B" --curves research/pkm_ic_factor_base_20261119/curves.json \
     --bits "$1" --t "$2" --targets 6 --out "research/pkm_ic_factor_base_20261119/$f") > /dev/null \
     || echo "cell b$1-t$2 exited $?" >> runs/progress.txt
  echo "done b$1-t$2 $(date -u +%FT%TZ)" >> runs/progress.txt
done
echo "end $(date -u +%FT%TZ)" >> runs/progress.txt
