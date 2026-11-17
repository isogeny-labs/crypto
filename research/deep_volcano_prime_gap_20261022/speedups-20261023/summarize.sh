#!/usr/bin/env bash
# Collect the recorded values of the 2026-10-23 speed-ups into summary.json
# (jq only; medians are the middle element of the sorted values, the lower
# middle for an even count). Sources:
#   rho-lanes/session/records.jsonl.gz, rho-lanes/session/comparisons/*.json,
#   rho-lanes/external_ca.jsonl, timing-plan.jsonl, timing-construct.jsonl,
#   the session.json files of ../run/sessions and rerun-sharded/sessions.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
cd "$here"
tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT
zcat rho-lanes/session/records.jsonl.gz | jq -c 'select(.warmup == false) | {arm, outcome: {status: .outcome.status}, time: {solve_wall_ns: .time.solve_wall_ns}, cost: {total_gae: .cost.total_gae, s: .cost.s}}' > "$tmp"
cmp=rho-lanes/session/comparisons/rho-neg__rho-neg-lanes.json
span() { jq -s '(map(.finished_unix_ms) | max) - (map(.started_unix_ms) | min)' "$@"; }
before_ms=$(span ../run/sessions/{spec,spec-aa,spec-bsgs}/session.json)
after_ms=$(span rerun-sharded/sessions/*/session.json)
jq -n \
  --slurpfile rec "$tmp" \
  --slurpfile cmp "$cmp" \
  --slurpfile ext rho-lanes/external_ca.jsonl \
  --slurpfile plan timing-plan.jsonl \
  --slurpfile con timing-construct.jsonl \
  --argjson before_ms "$before_ms" --argjson after_ms "$after_ms" \
  --arg session "$(jq -r .session_id rho-lanes/session/session.json)" '
  def median: sort | .[(length - 1) / 2 | floor];
  def arm($a): $rec | map(select(.arm == $a)) | {
      runs: length,
      verified: map(select(.outcome.status == "verified")) | length,
      median_ns_per_op: map(.time.solve_wall_ns / .cost.total_gae) | median,
      median_solve_ms: map(.time.solve_wall_ns / 1e6) | median,
      mean_ops_over_sqrt_r: (map(.cost.s) | add / length)
    };
  def stage($rows; $stage; $field): [$rows[] | select(.stage == $stage)] as $r | {
      before_s: ([$r[] | select(.side == "before") | .[$field]] | median),
      after_s: ([$r[] | select(.side == "after") | .[$field]] | median),
      rounds: ([$r[] | select(.side == "after")] | length)
    } | . + {speedup: (.before_s / .after_s)};
  def secs: map(.wall_s = .wall_ns / 1e9);
  ($plan | secs) as $plan | ($con | secs) as $con |
  {
    schema: "volcano-gap-speedups-summary/v1",
    rho: {
      session_id: $session,
      unit: "ns per counted group operation (GAE) of the solve; ca: its own internal timer and its own operation count",
      serial: arm("rho-neg"),
      lanes: arm("rho-neg-lanes"),
      external: {
        tool: "cryptanalysis ca solve --alg rho --threads 1 (aburan28/cryptanalysis 1aa48b5, version 0.1.0)",
        runs: ($ext[0:] | length),
        correct: ([$ext[] | select(.correct)] | length),
        median_ns_per_op: ([$ext[] | .seconds / .ops * 1e9] | median),
        median_solve_ms: ([$ext[] | .seconds * 1000] | median),
        mean_ops_over_sqrt_r: (([$ext[] | .ops] | add / length) / (1883199430181 | sqrt))
      },
      ecbench_compare: {
        ops_ratio_lanes_over_serial: $cmp[0].ops.ratio_b_over_a,
        ops_ci95: $cmp[0].ops.ci95,
        ops_pairs: $cmp[0].ops.pairs,
        wall_median_ratio_lanes_over_serial: $cmp[0].wall.median_ratio,
        wall_ci95: $cmp[0].wall.ci95,
        wall_pairs: $cmp[0].wall.pairs,
        admitted_pairs: $cmp[0].quality.admitted_pairs,
        wall_required_level: $cmp[0].wall.required_level,
        quality_levels: $cmp[0].quality.levels
      }
    },
    stages: [
      ({name: "ecbench plan, main spec (4560 runs), 1 CPU"} + stage($plan; "plan"; "wall_s")),
      ({name: "N-isogeny kernel and Vélu set-up, 1 CPU"} + stage($con; "construct"; "construct_s")),
      ({name: "N-isogeny map, one point, 1 CPU"} + stage($con; "construct"; "eval_s_per_point")),
      ({name: "N-isogeny map, one point, 4 CPUs (threads)"} + stage($con; "construct4"; "eval_s_per_point")),
      ({name: "volcano-gap construct, whole, 4 CPUs"} + stage($con; "construct4"; "wall_s")),
      {name: "measurement sessions, 6840 runs (3 sessions → 5, sharded), 4 vCPU",
       before_s: ($before_ms / 1000), after_s: ($after_ms / 1000), rounds: 1,
       speedup: ($before_ms / $after_ms)}
    ],
    identity: {
      plan_outputs: ([$plan[] | .output_sha256] | unique),
      isogeny_outputs: ([$con[] | .isogeny_sha256] | unique),
      sharded_records: "rerun-sharded/compare.sha256 (6840 records identical by arm, workload, target, round, cost, recovered value and counters)"
    }
  }' > summary.json
jq -c '.rho.serial, .rho.lanes, .rho.external, .rho.ecbench_compare, (.stages[] | [.name, .before_s, .after_s, .speedup]), .identity' summary.json
