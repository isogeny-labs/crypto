#!/bin/sh
# Specs for PROTOCOL.md.  base-specs/spec-P20..P40.json are the frontier
# campaign's frozen prime specs (copied from research/prime_ksum_table_20261028/
# frontier-specs/); spec-P44/P48.json put the prime_fast::find_a3_curve rungs
# at 44 and 48 bits (examples/ic_frontier_prime_curves.rs --bits 44,48) into
# the same frame.  Thin orchestration only.
#   ./make_specs.sh train        train-panel grids around the model optimum
#   ./make_specs.sh test         test panel from the frozen params.json
set -eu
cd "$(dirname "$0")"
mode=${1:?train or test}
SIZES="20 24 28 32 36 40 44 48"
# Model optimum A* per size (PROTOCOL.md "Predictions").
astar() { # BITS K
  case "$2:$1" in
    4:44) echo 95 ;; 4:48) echo 140 ;;
    5:20) echo 9 ;; 5:24) echo 11 ;; 5:28) echo 15 ;; 5:32) echo 19 ;;
    5:36) echo 25 ;; 5:40) echo 33 ;; 5:44) echo 44 ;; 5:48) echo 59 ;;
  esac
}
# k = 4 below 44 bits is frozen by research/prime_ksum_table_20261028/params.json.
frozen4() {
  jq -r --arg b "$1" '.prime[] | select(.bits == ($b|tonumber)) | .ksum4' \
    ../prime_ksum_table_20261028/params.json
}
# Predicted table entries 2^(k-1)·C(A,k); grid points above CAP are dropped.
CAP=300000000
entries() { awk -v a="$1" -v k="$2" 'BEGIN { c = 1; for (i = 0; i < k; i++) c = c * (a - i) / (i + 1); printf "%.0f", c * 2^(k - 1) }'; }
ksum_arm() { # NAME K A
  jq -n --arg n "$1" --arg k "$2" --arg a "$3" '{name:$n, role:"candidate",
    method:{id:"ic.pipeline", params:{factor_base:("prime-abscissa:size="+$a),
    oracle:("ksum:m="+$k), max_trials:"13528399621"}}}'
}
generic='[.arms[] | select(.name == "rho-neg" or .name == "bsgs-neg" or .name == "rho-neg-aa")]'
mkdir -p "$mode"
for b in $SIZES; do
  src=base-specs/spec-P$b.json
  if [ "$mode" = train ]; then
    ks="5"; [ "$b" -ge 44 ] && ks="4 5"
    arms=$(for k in $ks; do
      a0=$(astar "$b" "$k")
      grid="50 70 100 140 200"; [ "$b" -ge 44 ] && grid="60 75 90 100 115 130"
      for f in $grid; do
        a=$(( (a0 * f + 50) / 100 )); [ $a -lt $((k + 1)) ] && a=$((k + 1))
        [ "$(entries "$a" "$k")" -gt "$CAP" ] && continue
        ksum_arm "ksum$k-A$a" "$k" "$a"
      done
    done | jq -s 'unique_by(.name)')
    jq --argjson arms "$arms" --arg b "$b" '
      .label = ("prime k-sum extension: train panel, " + $b + "-bit a=-3 curve")
      | .description = "PROTOCOL.md selection: k-sum grids around the model optimum, train target seed. Counts only; AGENTS.md 5a band. Public synthetic; no security claim."
      | .workloads.targets_per_curve = 4
      | .workloads.target_seed = 20261015100
      | .arms = ([.arms[] | select(.name == "rho-neg")] + $arms)
      | .measurement.rounds = 1 | .measurement.warmup = 0' "$src" > "$mode/spec-P$b.json"
  else
    if [ "$b" -ge 44 ]; then
      a4=$(jq -r --arg b "$b" '.prime[] | select(.bits == ($b|tonumber)) | .ksum4' params.json)
    else
      a4=$(frozen4 "$b")
    fi
    a5=$(jq -r --arg b "$b" '.prime[] | select(.bits == ($b|tonumber)) | .ksum5' params.json)
    arms=$( { ksum_arm ic-ksum4 4 "$a4"; ksum_arm ic-ksum5 5 "$a5"; } | jq -s .)
    jq --argjson arms "$arms" --arg b "$b" "
      .label = (\"prime k-sum extension: test panel, \" + \$b + \"-bit a=-3 curve\")
      | .description = \"PROTOCOL.md test panel: rho-neg reference, bsgs-neg, the A/A control and the k = 4 and k = 5 k-sum arms. Counts only; AGENTS.md 5a band. Public synthetic; no security claim.\"
      | .arms = ($generic + \$arms)" "$src" > "$mode/spec-P$b.json"
  fi
done
ls "$mode"
