#!/bin/bash
# Validator V, 2026-11-14: compare/challenge/bounds/band outputs of one ecbench
# binary on committed sessions without a w_cal (target B, check B2).
# Usage: b2_outputs.sh ECBENCH_BINARY OUTDIR ROOT RECORD_LIST
# ROOT is a scratch copy (git archive of the merged commit) of docs/bounds and the
# sessions those bounds and the comparisons name; nothing is written inside it.
set -u
E=$1; O=$2; R=$3; L=$4
mkdir -p "$O"
cd "$R" || exit 1
run() { # name cmd...
  local n=$1; shift
  "$@" > "$O/$n.stdout" 2> "$O/$n.stderr"; echo "$n exit $?" >> "$O/exits.txt"
}
: > "$O/exits.txt"
# bounds: re-derive the 94 committed records whose sessions are in Git, then fit two version-2 bounds afresh
run bound_check "$E" bound check --root . --record $(cat "$L")
run fit_cal_prime_rho_neg_v2 "$E" bound fit --dir research/ecbench_calibration_20261002/sessions/prime --arm rho-neg \
  --audit research/ecbench_calibration_20261002/audit-prime.json --root . --out "$O/calibration-prime-rho-neg-v2.json"
run fit_session_prime_e1_incumbent_v2 "$E" bound fit --dir research/ecbench_bounds_challenges_20261006/sessions/prime-toy-reference-e1 \
  --arm incumbent --audit research/ecbench_bounds_challenges_20261006/audit-prime-toy-reference-e1.json --root . \
  --out "$O/session-prime-toy-reference-e1-incumbent-v2.json"
run fit_koblitz_toy_rho_frob_v2 "$E" bound fit --dir research/ecbench_calibration_20261002/sessions/koblitz --arm rho-frob \
  --audit research/ecbench_calibration_20261002/audit-koblitz.json --tier toy --root . --out "$O/calibration-koblitz-toy-rho-frob-v2.json"
# challenge: the committed procedure-1 verdict, every run replayed
run verdict_prime_e1 "$E" challenge verdict --challenge docs/bounds/challenges/prime-toy-reference.json \
  --dir research/ecbench_bounds_challenges_20261006/sessions/prime-toy-reference-e1 --procedure 1 --epoch 1 --replay-all \
  --bounds docs/bounds/records --root . --out "$O/verdict-prime-toy-reference-e1.json" \
  --bound-out "$O/challenge-prime-toy-reference-e1-bsgs-negation.json"
# compare: whole comparison JSON, three readings, two sessions
S1=research/generator_effect_ic_20261019/e3b/sessions/f4-c1-sem-chain-t2
S2=research/ic_planner_validation_20261020/sessions/r24
run cmp_e3b_default "$E" compare --dir $S1 --a rho-neg --b sem-chain-t2 --json
run cmp_e3b_pre1108 "$E" compare --dir $S1 --a rho-neg --b sem-chain-t2 --json --no-price-table-work
run cmp_e3b_w01 "$E" compare --dir $S1 --a rho-neg --b sem-chain-t2 --json --weight 0.1
run cmp_r24_default "$E" compare --dir $S2 --a rho-neg --b plan --json
run cmp_r24_pre1108 "$E" compare --dir $S2 --a rho-neg --b plan --json --no-price-table-work
run cmp_cross_r24_c24 "$E" compare --dir $S2 --b-dir research/ic_planner_validation_20261020/sessions/c24 --a rho-neg --b plan --json
# band: no w_cal flags
run band_e3b "$E" band --dir $S1 --json "$O/band_e3b.json" --ranks
run band_r24 "$E" band --dir $S2 --json "$O/band_r24.json" --ranks
# replay: the two rho runs of the e3b session (its IC runs took 1.5-3.7 h each) and two r24 runs, receipt printed
run verify_e3b_rho "$E" verify --dir $S1 --replay-seq 0 --replay-seq 2 --replay-seed 7
run verify_r24_seq "$E" verify --dir $S2 --replay-seq 70 --replay-seq 71 --replay-seq 78 --replay-seq 79 --replay-seed 7
echo done >> "$O/exits.txt"
