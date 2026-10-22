#!/usr/bin/env bash
# Assemble validator-report.json (validation-gate shape) and
# validation_report.yaml (agents/validator.md shape) from the validator's
# recompute outputs.  Run after recompute.sh, alike_band.jq runs,
# verify_scalars.py and make_certificate.sh.
# Usage: bash research/ecbench_generator_orbit_20261022/validation/make_report.sh VAL-ID
set -euo pipefail
V="$(cd "$(dirname "$0")" && pwd)"
HERE="$(dirname "$V")"
ROOT="$(git -C "$HERE" rev-parse --show-toplevel)"
VAL_ID="$1"
SESSIONS="a1-n29 a0-n13 a1-n17 a0-n19 a0-n23 a0-n31 a0-n37"
sha() { sha256sum "$1" | cut -c1-64; }
raw='[]'; tables='[]'
for s in $SESSIONS; do
  raw=$(jq -c --arg p "sessions/$s/records.jsonl" --arg rp "research/ecbench_generator_orbit_20261022/sessions/$s/records.jsonl" \
        --arg h "$(sha "$HERE/sessions/$s/records.jsonl")" --argjson n "$(wc -l < "$HERE/sessions/$s/records.jsonl")" \
        '. + [{path: $p, repo_path: $rp, sha256: $h, records: $n}]' <<< "$raw")
  tables=$(jq -c --arg p "tables/band-$s.json" --arg h "$(sha "$HERE/tables/band-$s.json")" \
        '. + [{path: $p, sha256: $h, role: "claimed side of the band headlines only"}]' <<< "$tables")
done
scripts='[]'
for f in band_recompute.jq h1_check.jq alike_band.jq verify_scalars.py recompute.sh make_certificate.sh make_report.sh claimed-3dec.json findings.json; do
  scripts=$(jq -c --arg p "validation/$f" --arg h "$(sha "$V/$f")" '. + [{path: $p, sha256: $h}]' <<< "$scripts")
done
jq -n \
  --arg val_id "$VAL_ID" \
  --argjson raw "$raw" --argjson tables "$tables" --argjson scripts "$scripts" \
  --slurpfile sum "$V/recompute-summary.json" \
  --slurpfile fnd "$V/findings.json" \
  --slurpfile cert "$V/replay-certificate.json" \
  --arg cert_sha "$(sha "$V/replay-certificate.json")" \
  --arg cal_sha "$(sha "$ROOT/docs/ic/calibration.json")" \
  --arg fp_sha "$(sha "$ROOT/src/cryptanalysis/ecbench/full_prices.json")" \
  --arg sc_sha "$(sha "$V/scalar-check.json")" '
  ($sum[0]) as $s | ($fnd[0]) as $f | ($cert[0]) as $c
  | ($s.band_headlines + $s.h1_headlines + [$s.n29_headline]) as $heads
  | ($heads | map(.agrees) | all) as $all
  | { schema: "validator-report/v1",
      study: "research/ecbench_generator_orbit_20261022",
      snapshot: $c.snapshot,
      status: (if $all then "pass" else "fail" end),
      independent: true,
      independence: "Recomputed by the validator, a separate agent from the measuring session, from raw records.jsonl with validator-written jq (validation/band_recompute.jq, validation/h1_check.jq). It does not read tables/ or ecbench band output except as the claimed side of each headline, and does not reuse band.rs or check_same_algorithm.sh. Formula read from src/cryptanalysis/ecbench/band.rs to confirm the rule only.",
      raw_data_paths_relative_to: "research/ecbench_generator_orbit_20261022/",
      raw_data: $raw,
      inputs: ([ {path: "../../docs/ic/calibration.json", repo_path: "docs/ic/calibration.json", sha256: $cal_sha, role: "pinned calibration ratios (C, n_C)"},
                 {path: "../../src/cryptanalysis/ecbench/full_prices.json", repo_path: "src/cryptanalysis/ecbench/full_prices.json", sha256: $fp_sha, role: "full-pricing units (diagnostics F2-F4 only)"} ] + $tables),
      scripts: $scripts,
      rule: "S(w) = (G - C + w*(U + n_C))/sqrt(r) per non-warm-up verified record, G = cost.total_gae, U = counts of cost.unpriced counters (top-level counters, else phase natives by name or name without _uncharged), C/n_C = sum over phases of count x pinned ratio / count for PRICED_NATIVE counters whose unit is in detail.calibration_pinned_units, ratio from docs/ic/calibration.json koblitz/K_<a> / GF(2^<n>); per-arm mean. Matches band.rs priced_gae / calibrated_native / unpriced_counts (no record has a solver block, so its word-XOR branch is moot).",
      headline_summary: { total: ($heads | length), agree: ($heads | map(select(.agrees)) | length),
                          band_means: ($s.band_headlines | length),
                          band_means_agree_3dec: ($s.band_headlines | map(select(.agrees_3dec)) | length),
                          max_band_rel_diff: ($s.band_headlines | map(.rel_diff | fabs) | max),
                          claimed_side: "tables/band-<session>.json mean_s (unrounded); claimed_3dec = the brief figures" },
      headline: $heads,
      h1_detail: $s.h1,
      replay_certificate: {path: "validation/replay-certificate.json", sha256: $cert_sha, status: $c.status,
                           records: $c.records, replays: $c.replays, reproduced: $c.reproduced, scalars_verified_count: $c.scalars_verified_count},
      scalar_check: {path: "validation/scalar-check.json", sha256: $sc_sha},
      findings: $f.findings,
      limitations: $f.limitations,
      validation_report: {
        id: $val_id,
        task_id: "none given (validation brief from the Coordinator session for the validation gate)",
        run_ids: ($c.runs | map(.session_id)),
        artifact_checks: [
          "study committed at 45a11de2eb, tree clean; validator writes only under validation/",
          "records.jsonl sha256 = session.json records_sha256 for all 7 sessions; line counts = executions_planned = records_written",
          "binary sha256 f7ac8fb0... = every record binary_sha256 (5760) and every session.json; git_commit de57ca373f, git_dirty false",
          "spec.json byte-identical to docs/ecbench/specs/claim-koblitz-generator-orbit-<s>.json, unchanged since freeze d2b3a56c32",
          "all sessions started after Amendment 1 (de57ca373f, 08:46:33Z)" ],
        metric_recomputations: [
          "84 band means: 84/84 within rel_tol 1e-6 of band JSON (max rel diff 0), 84/84 equal the 3-decimal claims",
          "H1: 960 pairs, 960 equal (claimed 960)",
          "a1-n29 ic-gen-layout-c1 mean trials 34.96875 (claimed 34.96875)",
          "G re-derived from counts x full_prices.json on 4320 measured records, max rel err 5e-16" ],
        control_checks: [
          "rho-frob-aa / rho-frob: identical records on all 960 keys; contains 1 by construction, no power (F5)",
          "replay: 4320/4320 identical; scalars 5760/5760 by ecbench audit and by independent Python check" ],
        heuristic_validation_checks: ["not applicable: engineering study, no heuristic prediction is validated"],
        cost_model_checks: [
          "unit ecbench.gae declared; prices one host microbenchmark table (full_prices.json), flagged in PROTOCOL.md",
          "band C subtracted at pinned ratios from G priced at full ratios (F2)",
          "band not applied alike to rho and IC native work; alike diagnostic flips n29 and n19 IC-vs-rho to unresolved (F3)",
          "n19 has no pinned calibration: band is a single full-price point (F4)",
          "memory recorded per record (time.max_rss_kib), not part of S" ],
        proof_architecture_checks: ["not applicable"],
        verdict: (if $all and $c.status == "verified" then "passed" else "failed" end),
        verdict_scope: "Admissible receipts and reproduced headline numbers. This does not support H5, rank any arm against rho, or authorize promotion; see F2-F4 before filling rho_reference.order_holds_across_band.",
        limitations: $f.limitations,
        artifact_paths: (["validation/replay-certificate.json", "validation/validator-report.json", "validation/scalar-check.json", "validation/recompute-summary.json"]
                         + [ $c.runs[] | .receipt ]) },
      review_attestation: {
        joints_owned: ["replay certificate", "headline recompute (band means, H1, n29 trials)"],
        sources_read: [
          "/home/user/crypto-autoresearcher/docs/agent-runtime-core.md", "/home/user/crypto-autoresearcher/agents/validator.md",
          "docs/VALIDATION_GATE.md", "scripts/fixtures/validation_gate/positive/*", "scripts/validation_gate.py (98-190)",
          "AGENTS.md section 5a (714-760)",
          "research/ecbench_generator_orbit_20261022/PROTOCOL.md, run.sh, check_same_algorithm.sh",
          "research/ecbench_generator_orbit_20261022/sessions/*/session.json, spec.json, records.jsonl (by key)",
          "research/ecbench_generator_orbit_20261022/tables/band-*.json (mean_s, ranks), tables/band-a1-n29.md",
          "research/ecbench_generator_orbit_20261022/audits/audit-a0-n13.json (summary), logs/verify-a0-n13.log (head), cards/ listing and card-2 receipt status",
          "src/cryptanalysis/ecbench/band.rs", "src/cryptanalysis/ecbench/methods.rs (PRICED_NATIVE 1898-1909, ic_report 1977-2004)",
          "src/cryptanalysis/ecbench/record.rs (Record::counts)", "src/cryptanalysis/ic_boundary.rs (pinned_calibration_ratios 260-320)",
          "src/cryptanalysis/ecbench/full_price.rs (1-178, 180-300)", "src/cryptanalysis/ecbench/audit.rs (395-470)",
          "src/cryptanalysis/koblitz_index_calculus.rs (844-930, 970-985)",
          "docs/ic/calibration.json", "src/cryptanalysis/ecbench/full_prices.json (instance keys)" ],
        read_sibling_reports: false,
        verdict: (if $all then "holds" else "breaks" end) } }
' > "$V/validator-report.json"
python3 -I -c '
import json, sys, yaml
rep = json.load(open(sys.argv[1]))
doc = {"validation_report": rep["validation_report"], "review_attestation": rep["review_attestation"],
       "gate_report": "validation/validator-report.json", "findings": rep["findings"]}
open(sys.argv[2], "w").write(yaml.safe_dump(doc, sort_keys=False, width=100, allow_unicode=True))
' "$V/validator-report.json" "$V/validation_report.yaml"
jq -r '"status=\(.status) headlines=\(.headline_summary.agree)/\(.headline_summary.total) 3dec=\(.headline_summary.band_means_agree_3dec)/\(.headline_summary.band_means) verdict=\(.validation_report.verdict)"' "$V/validator-report.json"
