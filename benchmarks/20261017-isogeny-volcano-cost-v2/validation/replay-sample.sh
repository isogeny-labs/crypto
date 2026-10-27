#!/usr/bin/env bash
# Replay sample, fixed before any replay ran. Inputs: nonce (nonce.json), manifest sizes.
set -euo pipefail
NONCE=$1
h() { printf '%s' "$1" | sha256sum | cut -c1-16; }
declare -A SEL
order=()
add() { if [ -z "${SEL[$1]:-}" ]; then SEL[$1]=$2; order+=("$1"); fi; }
# 1. forced: the 4 largest main sessions by uncompressed records.jsonl bytes
for s in $(awk -F'\t' '$1~/\/spec(-r2)?\/records.jsonl$/{print $3"\t"$1}' manifest_check.tsv | sort -n | tail -4 | cut -f2 | sed 's|^sessions/||; s|/records.jsonl$||'); do add "$s" largest4; done
# 2. forced: the interrupted session
add "l2-s1002/spec" interrupted
# 3. nonce-drawn: per ell and session type, the 2 classes with the smallest sha256(nonce:ell:type:class) not yet selected
for ell in l2 l3; do for t in main spec-aa spec-bsgs spec-icctl; do
  n=0
  for c in $(for i in $(seq 1001 1014); do c=$ell-s$i; echo "$(h "$NONCE:$ell:$t:$c") $c"; done | sort | cut -d' ' -f2); do
    if [ $t = main ]; then if [ -d ../wtv2/research/isogeny-volcano-cost-v2_20261017/sessions/$c/spec-r2 ]; then s=$c/spec-r2; else s=$c/spec; fi; else s=$c/$t; fi
    [ -n "${SEL[$s]:-}" ] && continue
    add "$s" "nonce:$ell:$t"; n=$((n+1)); [ $n -ge 2 ] && break
  done
done; done
for s in "${order[@]}"; do
  N=$(( 17 + 0x$(h "$NONCE:N:$s" | cut -c1-4) % 24 )); [ $((N % 12)) -eq 0 ] && N=$((N+1))
  printf '%s\t%s\t%s\n' "$s" "$N" "${SEL[$s]}"
done
