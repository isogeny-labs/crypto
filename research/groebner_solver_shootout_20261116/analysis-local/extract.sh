#!/usr/bin/env bash
# Layout only: flattens run-local/rows.jsonl into one TSV row per call (no arithmetic).
# Usage: extract.sh <rows.jsonl> <out.tsv>
set -euo pipefail
in="$1"; out="$2"
{
  printf 'arm\tinstance\tstratum\tpass\tposition\texcluded\toutcome_class\toutcome\tcharged_cpu_s\tsolver_cpu_s\textract_cpu_s\tinternal_cpu_s\tprojection_cpu_s\tn_verified\toracle_count\tmatches_oracle\tspurious_roots\tnative_ops\tdegree\tmm_rows\tmm_cols\tpeak_rss_bytes\texit_code\tsignal\twatchdog_killed\tplanted_found\tm1_violation\tzero_abscissa_roots\tn_roots\n'
  jq -r '[.arm, .instance_id, .study.stratum, .study.pass, .study.position, .exclusion.excluded,
          .outcome_class, .study.outcome, .study.charged_cpu_s, .study.solver_cpu_s, .study.extract_cpu_s,
          .study.internal_cpu_s, .study.projection_cpu_s, .study.n_verified, .study.oracle_count,
          .study.matches_oracle, .study.spurious_roots, .study.native_ops, .study.degree,
          (.study.max_matrix // [null,null])[0], (.study.max_matrix // [null,null])[1],
          .study.peak_rss_bytes, .study.exit_code, .study.signal, .study.wall_watchdog_killed,
          .study.planted_found, .study.m1_violation, .study.zero_abscissa_roots, .study.n_roots]
         | map(if . == null then "NA" else tostring end) | @tsv' "$in"
} > "$out"
