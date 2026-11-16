#!/usr/bin/env bash
# Stage 0b: the class selection rule of PROTOCOL.md §3 for one tier (24 or 28).
# Seeds 2401, 2402, ... (tier 24) or 2801, ... (tier 28) are tried in order; every
# rejected seed is logged with its reason; the first 12 accepted are the classes.
# Writes $D/inputs/classes/t<tier>/s<seed>/{spec*.json,curves.jsonl,transport.jsonl}.
source "$(dirname "$0")/common.sh"
tier=$1
bits=$(jq -er ".tiers[\"$tier\"].bits" $DESIGN)
out=$D/inputs/classes/t$tier
log=$D/inputs/classes/t$tier.selection.log
mkdir -p "$out"
: > "$log"
# p of the v1 and v2 volcano studies (a class must be fresh).
used=$(cat research/isogeny-volcano-cost_20261017/classes/*/spec.json \
           research/isogeny-volcano-cost-v2_20261017/classes/*/spec.json 2>/dev/null \
       | jq -r '.workloads.curves[0].p' | sort -u)
accepted=0
seed=$((tier * 100 + 1))
scratch=$(mktemp -d)
while (( accepted < 12 )); do
  dir=$out/s$seed
  rm -rf "$dir"
  if ! nice -n 10 $VOLCANO_RHO --bits "$bits" --depth 3 --max-cofactor 4096 --ell 2 --crater split \
       --seed "$seed" --controls 0 --level-caps 16,0,0,64 --models 1 --crater-models 16 --targets 8 \
       --ecbench "$dir" --out "$scratch/s$seed" > "$dir.log" 2>&1; then
    echo "s$seed rejected: volcano-rho failed (see s$seed.log)" >> "$log"; rm -rf "$dir"; seed=$((seed+1)); continue
  fi
  p=$(jq -r '.workloads.curves[0].p' "$dir/spec.json")
  if grep -qx "$p" <<< "$used"; then
    echo "s$seed rejected: p=$p was used by v1/v2" >> "$log"; rm -rf "$dir"; seed=$((seed+1)); continue
  fi
  if ! $REPSEL ascend --class-dir "$dir" > "$dir/ascend.json" 2> "$dir/ascend.err"; then
    echo "s$seed rejected: repsel ascend failed: $(cat "$dir/ascend.err")" >> "$log"; rm -rf "$dir"; seed=$((seed+1)); continue
  fi
  if ! $REPSEL specs --class-dir "$dir" --design $DESIGN --tier "$tier" --class "t$tier-s$seed" \
       --out "$scratch/probe-s$seed" > /dev/null 2> "$dir/accept.err"; then
    echo "s$seed rejected: $(cat "$dir/accept.err")" >> "$log"; rm -rf "$dir"; seed=$((seed+1)); continue
  fi
  echo "s$seed accepted: p=$p" >> "$log"
  accepted=$((accepted+1)); seed=$((seed+1))
done
rm -rf "$scratch"
cat "$log"
