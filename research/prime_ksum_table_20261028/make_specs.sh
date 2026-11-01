#!/bin/sh
# Specs for PROTOCOL.md, derived from the frontier campaign's frozen prime specs
# (frontier-specs/spec-P*.json, copied verbatim from
# research/ic_frontier_campaign_20261015/phase1/).  Thin orchestration only.
#   ./make_specs.sh train        train-panel grids around the model optimum
#   ./make_specs.sh test         test panel from the frozen params.json
set -eu
cd "$(dirname "$0")"
mode=${1:?train or test}
# Model optimum A* per size (PROTOCOL.md "Predictions").
astar() { # BITS K
  case "$2:$1" in
    3:20) echo 16 ;; 3:24) echo 27 ;; 3:28) echo 46 ;; 3:32) echo 80 ;; 3:36) echo 138 ;; 3:40) echo 239 ;;
    4:20) echo 10 ;; 4:24) echo 14 ;; 4:28) echo 21 ;; 4:32) echo 30 ;; 4:36) echo 44 ;; 4:40) echo 64 ;;
  esac
}
ksum_arm() { # NAME K A
  jq -n --arg n "$1" --arg k "$2" --arg a "$3" '{name:$n, role:"candidate",
    method:{id:"ic.pipeline", params:{factor_base:("prime-abscissa:size="+$a),
    oracle:("ksum:m="+$k), max_trials:"13528399621"}}}'
}
mkdir -p "$mode"
for b in 20 24 28 32 36 40; do
  src=frontier-specs/spec-P$b.json
  if [ "$mode" = train ]; then
    arms=$(for k in 3 4; do
      a0=$(astar $b $k)
      for f in 50 70 100 140 200; do
        a=$(( (a0 * f + 50) / 100 )); [ $a -lt $((k + 1)) ] && a=$((k + 1))
        ksum_arm "ksum$k-A$a" $k $a
      done
    done | jq -s 'unique_by(.name)')
    jq --argjson arms "$arms" --arg b "$b" '
      .label = ("prime k-sum table: train panel, " + $b + "-bit a=-3 curve")
      | .description = "PROTOCOL.md selection: k-sum grids around the model optimum, train target seed. Counts only; AGENTS.md 5a band. Public synthetic; no security claim."
      | .workloads.targets_per_curve = 4
      | .workloads.target_seed = 20261015100
      | .arms = ([.arms[] | select(.name == "rho-neg")] + $arms)
      | .measurement.rounds = 1 | .measurement.warmup = 0' "$src" > "$mode/spec-P$b.json"
  else
    a3=$(jq -r --arg b "$b" '.prime[] | select(.bits == ($b|tonumber)) | .ksum3' params.json)
    a4=$(jq -r --arg b "$b" '.prime[] | select(.bits == ($b|tonumber)) | .ksum4' params.json)
    arms=$( { ksum_arm ic-ksum3 3 "$a3"; ksum_arm ic-ksum4 4 "$a4"; } | jq -s .)
    jq --argjson arms "$arms" --arg b "$b" '
      .label = ("prime k-sum table: test panel, " + $b + "-bit a=-3 curve")
      | .description = "PROTOCOL.md test panel: the frontier campaign Phase 1 arms plus the frozen k = 3 and k = 4 k-sum arms. Counts only; AGENTS.md 5a band. Public synthetic; no security claim."
      | .arms = (.arms + $arms)' "$src" > "$mode/spec-P$b.json"
  fi
done
ls "$mode"
