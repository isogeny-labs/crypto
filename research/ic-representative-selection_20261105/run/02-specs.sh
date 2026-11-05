#!/usr/bin/env bash
# Stage 0c: every pre-selection job spec and label file, the job list and the inputs
# digest. Tier 24 runs arm (c) on all 12 classes, tier 28 on its first 6 (design.json).
# The digest goes into INPUTS.md, committed and pushed before the first launch.
source "$(dirname "$0")/common.sh"
specs=$D/inputs/specs
rm -rf "$specs"; mkdir -p "$specs"
: > $D/inputs/jobs.tsv
for tier in 24 28; do
  n=0
  for dir in $(ls -d $D/inputs/classes/t$tier/s* | sort -t s -k2 -n); do
    [[ -d $dir ]] || continue
    class=t$tier-$(basename "$dir")
    withc=(); if [[ $tier == 24 || $n -lt 6 ]]; then withc=(--with-c); fi
    $REPSEL specs --class-dir "$dir" --design $DESIGN --tier $tier --class "$class" "${withc[@]}" \
      --out "$specs/$class" >> $D/inputs/jobs.tsv
    n=$((n+1))
  done
done
( cd $D/inputs && find classes specs -type f \( -name '*.json' -o -name '*.jsonl' \) | LC_ALL=C sort \
    | xargs sha256sum ) > $D/inputs/MANIFEST.sha256
sha256sum $D/inputs/MANIFEST.sha256 $D/inputs/jobs.tsv
