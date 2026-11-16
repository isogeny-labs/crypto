#!/bin/bash
# Q3 over every frozen/v2 pair; writes pair_check.json (an array, one object per pair).
set -eu
D=$(dirname "$0")/..
P=$D/sessions/pairs-prime/records.jsonl; K=$D/sessions/pairs-koblitz/records.jsonl; L=$D/sessions/pairs-large-prime/records.jsonl
{
jq -s --arg a bsgs --arg b bsgs-v2 --arg mode bsgs -f $D/analysis/pair_check.jq $P
jq -s --arg a bsgs-il --arg b bsgs-il-v2 --arg mode bsgs -f $D/analysis/pair_check.jq $P
jq -s --arg a bsgs-neg --arg b bsgs-neg-v2 --arg mode bsgs -f $D/analysis/pair_check.jq $P
jq -s --arg a kangaroo --arg b kangaroo-v2 --arg mode kangaroo -f $D/analysis/pair_check.jq $P
jq -s --arg a bsgs-neg --arg b bsgs-neg-v2 --arg mode bsgs -f $D/analysis/pair_check.jq $K
jq -s --arg a claw --arg b claw-v2 --arg mode bsgs -f $D/analysis/pair_check.jq $K
jq -s --arg a lp --arg b lp-v2 --arg mode large_prime -f $D/analysis/pair_check.jq $L
# A/A negative control: the reference arm against its repeat must be equal on every count
jq -s --arg a rho-neg-v2 --arg b rho-neg-v2-aa --arg mode aa -f $D/analysis/pair_check.jq $P
jq -s --arg a rho-frob-v2 --arg b rho-frob-v2-aa --arg mode aa -f $D/analysis/pair_check.jq $K
} | jq -s '.'
