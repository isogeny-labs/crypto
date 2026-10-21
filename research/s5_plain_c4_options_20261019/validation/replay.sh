#!/usr/bin/env bash
# Replay certificate for the 20261019-s5-plain-c4-options benchmarks row.
#
# Rebuilds nothing itself: run it with a binary built from the committed
# source (cargo build --release --example gaudry_quartic_c4_options) in a
# fresh target directory. It re-runs every recorded run of the study into
# validation/replay/, then compares the replay with the committed records
# solve by solve: every counted phase, the decomposition set, the oracle's
# set and their agreement must be identical. The "scalars" of a
# decomposition oracle are its decompositions: a solve is verified when its
# replayed decomposition set equals the committed one and the
# meet-in-the-middle oracle's.
#
# Usage (from the repository root):
#   BIN=target/release/examples/gaudry_quartic_c4_options \
#     bash research/s5_plain_c4_options_20261019/validation/replay.sh \
#     > research/s5_plain_c4_options_20261019/validation/replay-certificate.json
set -euo pipefail
S=research/s5_plain_c4_options_20261019
BIN=${BIN:-target/release/examples/gaudry_quartic_c4_options}
OUT=$S/validation/replay
mkdir -p "$OUT"
export RAYON_NUM_THREADS=4

run() { # name args...
  local name=$1; shift
  if [ ! -s "$OUT/$name.json" ]; then
    "$BIN" "$@" --json "$OUT/$name.json" > "$OUT/$name.log" 2>&1
  fi
}
run s5_p269_seed1_r13 --p 269 --seed 1 --residuals 13 \
  --arms committed,sparsest+chains,sparsest+chains+replay
run s5_p269_seed1 --p 269 --seed 1 --residuals 4 \
  --arms committed,sparsest,sparsest+chains,sparsest+chains+replay
run s5_p269_seed1_constructed4_replay --p 269 --seed 1 --residuals 4 --constructed 4 \
  --arms sparsest+chains+replay
run generic_d4-7 --generic 4,5,6,7 --arms committed,sparsest,sparsest+chains

# Per-solve comparison: everything the run records except wall time.
KEYS='{index: .index, arm: .arm, s: (.solve | del(.build_ms, .echelon_ms, .nf_ms, .charpoly_ms, .eigen_ms, .total_ms)), q: .solver_quadruples, agree: .agree}'
cmp_s5() { # name
  local a b
  a=$(jq -c "[.residuals[] | .index as \$i | .oracle_quadruples as \$o | .arms[] | {index: \$i, oracle: \$o} + . | $KEYS + {oracle: .oracle}] | sort_by(.index, .arm)" "$S/raw/$1.json")
  b=$(jq -c "[.residuals[] | .index as \$i | .oracle_quadruples as \$o | .arms[] | {index: \$i, oracle: \$o} + . | $KEYS + {oracle: .oracle}] | sort_by(.index, .arm)" "$OUT/$1.json")
  jq -n --arg run "$1" --argjson a "$a" --argjson b "$b" '{
    run: $run,
    committed: ("research/s5_plain_c4_options_20261019/raw/" + $run + ".json"),
    replay: ("research/s5_plain_c4_options_20261019/validation/replay/" + $run + ".json"),
    solves: ($a | length),
    identical: ($a == $b),
    scalars_verified: (($a == $b) and ($b | all(.agree and (.q == .oracle)))),
    group_replay: ($a == $b)
  }'
}
cmp_generic() {
  local a b
  a=$(jq -c '[.[] | {d: .equation_degree, arm, s: (.solve | del(.build_ms, .echelon_ms, .nf_ms, .charpoly_ms, .eigen_ms, .total_ms)), planted: .planted_found}] | sort_by(.d, .arm)' "$S/raw/generic_d4-7.json")
  b=$(jq -c '[.[] | {d: .equation_degree, arm, s: (.solve | del(.build_ms, .echelon_ms, .nf_ms, .charpoly_ms, .eigen_ms, .total_ms)), planted: .planted_found}] | sort_by(.d, .arm)' "$OUT/generic_d4-7.json")
  jq -n --argjson a "$a" --argjson b "$b" '{
    run: "generic_d4-7",
    committed: "research/s5_plain_c4_options_20261019/raw/generic_d4-7.json",
    replay: "research/s5_plain_c4_options_20261019/validation/replay/generic_d4-7.json",
    solves: ($a | length),
    identical: ($a == $b),
    scalars_verified: (($a == $b) and ($b | all(.planted))),
    group_replay: ($a == $b)
  }'
}
runs=$( { cmp_s5 s5_p269_seed1_r13; cmp_s5 s5_p269_seed1; cmp_s5 s5_p269_seed1_constructed4_replay; cmp_generic; } | jq -s .)
commit=$(git rev-parse HEAD)
jq -n --argjson runs "$runs" --arg commit "$commit" --arg bin "$BIN" --arg rustc "$(rustc --version)" '{
  schema: "s5-plain-c4-replay/v1",
  status: (if ($runs | all(.identical and .scalars_verified)) then "verified" else "failed" end),
  independent: true,
  independence: "a fresh release build of the committed source in a new target directory, run as new processes, compared with the committed records by jq (validation/replay.sh), not by the runner. Same host as the original runs (Apple M4 Pro); not a second machine.",
  scalars_verified: ($runs | all(.scalars_verified)),
  source_commit: $commit,
  binary: $bin,
  rustc: $rustc,
  compared: "every counted phase of every solve (wall times excluded), the solver decomposition set, the oracle set and their agreement",
  runs: $runs
}'
