#!/bin/bash
# Card-driven form of transfer_cost.sh (same ca flags, seed 20261023, reps 200). One invocation does one unit of work
# and is idempotent: it replaces the rows of its (volcano, src) in runs/transfer_cost.jsonl, never duplicates them.
#   transfer_cost_part.sh walk <V5|V6> <D|floor-a|floor-b|crater-b>   rows tagged src = the walk name
#   transfer_cost_part.sh tcr <id>                                     V5 vertex id 1..63; keeps the level-1 line (path v -> w),
#                                                                       src = tcr-v<id>; the last edge w -> j=0 is counted in the
#                                                                       analysis as 2 Velu point maps (ANALYSIS_PLAN 5b)
#   transfer_cost_part.sh publish <label>                              commit runs/transfer_cost.jsonl only, push (retry; pull --no-rebase on rejection; no force)
set -euo pipefail
STUDY=/home/user/crypto/research/volcano_ecdlp_census_followup_20261023
CA=/home/user/cryptanalysis/build/ca
OUT=$STUDY/runs/transfer_cost.jsonl
TMP=$STUDY/runs/tc_tmp
SEED=20261023
REPS=200
REPO=/home/user/crypto
REL=research/volcano_ecdlp_census_followup_20261023/runs/transfer_cost.jsonl
mode=${1:-}
mkdir -p "$TMP"
touch "$OUT"

replace_rows() { # $1 volcano, $2 src, $3 file of new tagged rows
  jq -c --arg v "$1" --arg s "$2" 'select((.volcano==$v and .src==$s)|not)' "$OUT" > "$TMP/keep.jsonl"
  cat "$TMP/keep.jsonl" "$3" > "$TMP/new.jsonl"
  mv "$TMP/new.jsonl" "$OUT"
  rm -f "$TMP/keep.jsonl"
}

case $mode in
walk)
  V=${2:?}; S=${3:?}
  case $V in V5) ELL=2;; V6) ELL=3;; *) echo "BAD_VOLCANO $V" >&2; exit 2;; esac
  W=$STUDY/census/$V/walks/$S/R.json
  [ -f "$W" ] || { echo "MISSING $W" >&2; exit 2; }
  P=$(jq -r .p "$W"); A=$(jq -r .a "$W"); B=$(jq -r .b "$W")
  RAW=$TMP/$V-$S.raw; TAG=$TMP/$V-$S.tag
  "$CA" volcano --p "$P" --a "$A" --b "$B" --ell "$ELL" --seed "$SEED" --trials 1 --per-level 0 --controls 0 --radius 40 --max-vertices 20000 --transport-bench "$RAW" --transport-reps "$REPS" > /dev/null 2> "$RAW.err"
  [ -s "$RAW" ] || { echo "EMPTY_BENCH $V $S" >&2; exit 3; }
  jq -c --arg v "$V" --arg s "$S" --argjson ell "$ELL" --argjson p "$P" '. + {volcano:$v,src:$s,ell:$ell,p:$p}' "$RAW" > "$TAG"
  replace_rows "$V" "$S" "$TAG"
  echo "OK $V $S rows=$(wc -l < "$TAG")"
  rm -f "$RAW" "$RAW.err" "$TAG"
  ;;
tcr)
  ID=${2:?}
  case $ID in ''|*[!0-9]*|0) echo "BAD_ID $ID" >&2; exit 2;; esac
  P=$(jq -r .p "$STUDY/census/V5/walks/D/R.json")
  ROW=$(jq -c --argjson id "$ID" 'select(.id==$id)|{a,b}' "$STUDY/census/V5/vertices.jsonl")
  [ -n "$ROW" ] || { echo "MISSING_VERTEX $ID" >&2; exit 2; }
  A=$(echo "$ROW" | jq -r .a); B=$(echo "$ROW" | jq -r .b)
  RAW=$TMP/tcr-v$ID.raw; TAG=$TMP/tcr-v$ID.tag
  "$CA" volcano --p "$P" --a "$A" --b "$B" --ell 2 --seed "$SEED" --trials 1 --per-level 0 --controls 0 --radius 40 --max-vertices 20000 --transport-bench "$RAW" --transport-reps "$REPS" > /dev/null 2> "$RAW.err"
  jq -c --argjson src "$ID" --argjson p "$P" 'select(.level==1) + {volcano:"V5",src:("tcr-v"+($src|tostring)),ell:2,p:$p,tcr_source_id:$src,note:"path v -> level-1 vertex w; add 2 Velu point maps for the last edge w -> j=0"}' "$RAW" > "$TAG"
  [ "$(wc -l < "$TAG")" = 1 ] || { echo "LEVEL1_ROWS_NOT_ONE tcr-v$ID rows=$(wc -l < "$TAG")" >&2; exit 3; }
  replace_rows V5 "tcr-v$ID" "$TAG"
  echo "OK V5 tcr-v$ID rows=1"
  rm -f "$RAW" "$RAW.err" "$TAG"
  ;;
publish)
  LABEL=${2:?}
  rmdir "$TMP" 2>/dev/null || true
  BRANCH=$(git -C "$REPO" rev-parse --abbrev-ref HEAD)
  git -C "$REPO" add -f -- "$REL"
  if git -C "$REPO" diff --cached --quiet -- "$REL"; then
    echo "NOTHING_TO_COMMIT $REL"
  else
    git -C "$REPO" commit -q -m "volcano census follow-up: transfer-cost microbenchmark rows ($LABEL)" -m "Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_0121ScZMwCLCc1YWonfhYhzx" -- "$REL"
    echo "COMMITTED $REL"
  fi
  for i in 1 2 3 4 5; do
    if git -C "$REPO" push -q -u origin "$BRANCH"; then echo "PUSHED $BRANCH"; exit 0; fi
    git -C "$REPO" pull -q --no-rebase origin "$BRANCH" || true
    sleep $((i*3))
  done
  echo "PUSH_FAILED $BRANCH" >&2; exit 4
  ;;
*) echo "BAD_MODE $mode" >&2; exit 2;;
esac
