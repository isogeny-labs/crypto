#!/bin/sh
# Usage: prov2.sh <protocol-file> <raw-path> <data-file>      (from the repo root)
# Read-only git queries. One JSON object per call:
#  p_first   : first commit that added the protocol file
#  raw_first : first commit that added anything under <raw-path>
#  data_first: first commit that added <data-file>
#  p_edits_after_raw : commits touching the protocol file that are descendants
#                      of raw_first (the protocol changed after data existed)
#  p_before_raw : is p_first a strict ancestor of raw_first
set -eu
p="$1"; raw="$2"; data="$3"
add() { git log --diff-filter=A --reverse --format='%H|%cI|%s' -- "$1" 2>/dev/null | head -1; }
pf=$(add "$p"); rf=$(add "$raw"); df=$(add "$data")
psha=$(echo "$pf" | cut -d'|' -f1); rsha=$(echo "$rf" | cut -d'|' -f1); dsha=$(echo "$df" | cut -d'|' -f1)
rel="unknown"
if [ -n "$psha" ] && [ -n "$rsha" ]; then
  if [ "$psha" = "$rsha" ]; then rel="same_commit"
  elif git merge-base --is-ancestor "$psha" "$rsha"; then rel="protocol_strictly_before_raw"
  else rel="protocol_NOT_before_raw"; fi
fi
edits=""
if [ -n "$rsha" ]; then
  for c in $(git log --format=%H -- "$p"); do
    if [ "$c" != "$rsha" ] && git merge-base --is-ancestor "$rsha" "$c" 2>/dev/null; then
      edits="$edits$(git log -1 --format='%H|%cI|%s' $c)
"
    fi
  done
fi
jq -n --arg p "$p" --arg raw "$raw" --arg data "$data" --arg pf "$pf" --arg rf "$rf" --arg df "$df" --arg rel "$rel" --arg edits "$edits" \
 '{protocol:$p,raw:$raw,data:$data,p_first:$pf,raw_first:$rf,data_first:$df,p_vs_raw:$rel,p_edits_after_raw:($edits|split("\n")|map(select(length>0)))}'
