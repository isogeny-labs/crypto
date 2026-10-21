#!/usr/bin/env bash
# Validator report for the 20261019-s5-plain-c4-options benchmarks row.
#
# Recomputes every headline number of the row from the raw per-solve JSON
# with jq alone. It shares no code with examples/gaudry_quartic_c4_options_summary.rs,
# which produced the numbers the row claims, and compares each recomputation
# with the claimed value (claimed values copied from the row and README).
#
# Usage (from the repository root):
#   bash research/s5_plain_c4_options_20261019/validation/validate.sh \
#       > research/s5_plain_c4_options_20261019/validation/validator-report.json
set -euo pipefail
S=research/s5_plain_c4_options_20261019
R13=$S/raw/s5_p269_seed1_r13.json
R4=$S/raw/s5_p269_seed1.json
CON=$S/raw/s5_p269_seed1_constructed4_replay.json
RHO=$S/validation/rho_reference.json
# Paths in the report are relative to the evidence directory
# (benchmarks/20261019-s5-plain-c4-options/).
rel() { echo "../../$1"; }

recompute=$(jq -n --slurpfile r13 "$R13" --slurpfile r4 "$R4" --slurpfile con "$CON" --slurpfile rho "$RHO" '
  def arm($res; $name): [$res[] | .arms[] | select(.arm == $name)];
  def mean(f): (map(f) | add) / length;
  ($r13[0].residuals) as $res
  | (arm($res; "committed")) as $c
  | (arm($res; "sparsest+chains")) as $sc
  | ([$res[] | select(.index > 0)]) as $later
  | (arm($later; "sparsest+chains+replay")) as $rp
  | (arm($later; "committed")) as $c_later
  | ($r4[0].residuals) as $res4
  | (arm($res4; "sparsest")) as $sp
  | (arm($res4; "committed")) as $c4arm
  | ([$res[] | .arms[]] + [$res4[] | .arms[]] + [$con[0].residuals[] | .arms[]]) as $all
  | {
      committed_mean_c4: ($c | mean(.solve.total_muls)),
      sparsest_chains_mean_c4: ($sc | mean(.solve.total_muls)),
      sparsest_chains_ratio_of_means: (($c | mean(.solve.total_muls)) / ($sc | mean(.solve.total_muls))),
      replay_mean_c4: ($rp | mean(.solve.total_muls)),
      replay_ratio_of_means: (($c_later | mean(.solve.total_muls)) / ($rp | mean(.solve.total_muls))),
      replay_n: ($rp | length),
      replay_fallbacks: ($rp | map(select(.solve.replay == "FellBack")) | length),
      sparsest_ratio_of_means: (($c4arm | mean(.solve.total_muls)) / ($sp | mean(.solve.total_muls))),
      solves_oracle_equal: ($all | map(select(.agree)) | length),
      solves_total: ($all | length),
      constructed_planted_found: ([$con[0].residuals[] | . as $r | .arms[] | select(.solver_quadruples | index([$r.planted])) ] | length),
      rho_mean_fp_muls: $rho[0].mean_fp_muls,
      rho_runs_verified: ($rho[0].runs | map(select(.verified)) | length)
    }')

claim() { # name claimed recomputed-key rel_tol
  jq -n --arg n "$1" --argjson c "$2" --argjson r "$(jq ".$3" <<<"$recompute")" --argjson t "$4" \
    '{name: $n, claimed: $c, recomputed: $r, rel_tol: $t}'
}
heads=$(jq -s . <<EOF
$(claim committed_mean_c4 1.2093e12 committed_mean_c4 5e-5)
$(claim sparsest_chains_mean_c4 1.6750e11 sparsest_chains_mean_c4 5e-5)
$(claim sparsest_chains_ratio_of_means 7.22 sparsest_chains_ratio_of_means 1e-3)
$(claim replay_mean_c4 8.69e10 replay_mean_c4 1e-3)
$(claim replay_ratio_of_means 13.90 replay_ratio_of_means 1e-3)
$(claim replay_n 12 replay_n 0)
$(claim replay_fallbacks 1 replay_fallbacks 0)
$(claim sparsest_ratio_of_means 3.93 sparsest_ratio_of_means 2e-3)
$(claim solves_oracle_equal 59 solves_oracle_equal 0)
$(claim solves_total 59 solves_total 0)
$(claim constructed_planted_found 4 constructed_planted_found 0)
EOF
)

ok=$(jq 'all(.[]; ((.recomputed - .claimed) | fabs) <= .rel_tol * (.claimed | fabs))' <<<"$heads")
raw=$(for f in "$R13" "$R4" "$CON" "$RHO"; do
  jq -n --arg p "$(rel "$f")" --arg h "$(shasum -a 256 "$f" | cut -d' ' -f1)" '{path: $p, sha256: $h}'
done | jq -s .)

jq -n --argjson heads "$heads" --argjson raw "$raw" --argjson ok "$ok" --argjson all "$recompute" '{
  schema: "s5-plain-c4-validator/v1",
  status: (if $ok then "pass" else "fail" end),
  independent: true,
  independence: "jq recomputation from the raw per-solve records; shares no code with the Rust summary (examples/gaudry_quartic_c4_options_summary.rs) that produced the claimed numbers. Same repository author; not a second person.",
  method: "research/s5_plain_c4_options_20261019/validation/validate.sh",
  raw_data: $raw,
  headline: $heads,
  recomputed_all: $all
}'
