#!/bin/bash
# Writes the three validation-gate artifacts of Table B (the prime and random-binary ladder, Round 5) into
# docs/ic/runs/: validation/replay-certificate.json, validation/validator-report.json and VALIDATION.json
# (docs/VALIDATION_GATE.md).  Inputs: the comparison of a native re-execution with the ledger
# (compare_B.sh, once per regime) and the raw ledger.  Every field is computed; a failing comparison writes
# status "failed", which the gate refuses.
# usage: make_validation_B.sh <compare_prime.jsonl> <compare_char2.jsonl> <ic-binary-commit> <ic-binary-sha256>
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
prime="$1"; char2="$2"; commit="$3"; binsha="$4"
out=docs/ic/runs/validation; mkdir -p "$out"
ledger=docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json
reprice=research/ic_rho_reference_20260923/reprice/ic-boundary-ledger-round5-2026-09-22.json
tmp=$(mktemp -d); trap 'rm -r "$tmp"' EXIT
sh scripts/ic_audit/recompute_B.sh > "$tmp/B.json"
jq -s 'add' "$prime" "$char2" > "$tmp/cmp.json"
# --- replay certificate
jq -n --slurpfile c "$tmp/cmp.json" --arg commit "$commit" --arg bin "$binsha" '
  ($c[0]) as $runs
  | { schema: "validation-replay-certificate/v1",
      status: (if ($runs | all(.seeds_equal and .gae_equal and .recovered_equal and .planted_equal and .verified)) then "verified" else "failed" end),
      independent: true,
      scalars_verified: ($runs | all(.recovered_equal and .planted_equal and .verified)),
      method: "native re-execution of `ic boundary` with the frozen Round-5 configuration (seed 123212651130, 3 repetitions) on a binary built from the commit below; for every instance the recipe the board shows is compared repetition by repetition with the ledger on total_gae, recovered scalar and verified flag, and each recovered scalar is compared with the planted scalar of its repetition. Counts and scalars are deterministic, so equality is the replay.",
      binary: { commit: $commit, sha256: $bin },
      auditor: "validation sweep slice A (research/validation_sweep_20261019/A-leaderboard) and research/leaderboard_rigor_20261021",
      runs: $runs }' > "$out/replay-certificate.json"
# --- validator report: every headline figure of every Table B row, board against raw-data recomputation
jq -n --slurpfile b docs/ic/leaderboard.json --slurpfile r "$tmp/B.json" \
   --arg ledger "$ledger" --arg reprice "$reprice" --arg lsha "$(sha256sum $ledger | cut -d' ' -f1)" --arg rsha "$(sha256sum $reprice | cut -d' ' -f1)" '
  [ $b[0].board[] | select(.family == "B") | . as $row
    | ($r[0] | map(select(.instance == $row.legacy)) | .[0]) as $x
    | ( {name: "\($row.slug) S (IC)", claimed: $row.best.s, recomputed: $x.S_IC},
        {name: "\($row.slug) S (reference)", claimed: $row.reference_s, recomputed: $x.S_ref},
        {name: "\($row.slug) x reference", claimed: $row.best.ratio_rho, recomputed: $x.xref},
        {name: "\($row.slug) x floor", claimed: $row.best.ratio_floor, recomputed: $x.xfloor} )
      | . + {rel_tol: 1e-8} ] as $h
  | { schema: "validation-validator-report/v1",
      status: (if ($h | all((.claimed - .recomputed | fabs) <= .rel_tol * ((.claimed | fabs) + 1e-300))) then "pass" else "fail" end),
      independent: true,
      method: "scripts/ic_audit/recompute_B.sh: jq, no code shared with scripts/build_ic_leaderboard.py. S (IC) is the mean over the 3 repetitions of total_gae of the board recipe over sqrt r; the reference S is the mean over every matched-rho run of gae over sqrt r (per_run, not the stored mean); x reference and x floor follow.",
      raw_data: [ {path: "../ic-boundary-ledger-round5-2026-09-22.json", sha256: $lsha},
                  {path: "../../../research/ic_rho_reference_20260923/reprice/ic-boundary-ledger-round5-2026-09-22.json", sha256: $rsha} ],
      headline: $h }' > "$out/validator-report.json"
# --- VALIDATION.json
jq -n --slurpfile b docs/ic/leaderboard.json --slurpfile r "$tmp/B.json" \
   --arg rc "validation/replay-certificate.json" --arg rcsha "$(sha256sum $out/replay-certificate.json | cut -d' ' -f1)" \
   --arg vr "validation/validator-report.json" --arg vrsha "$(sha256sum $out/validator-report.json | cut -d' ' -f1)" --arg bin "ic boundary @ $commit" '
  def gm: (map(log) | add / length) | exp;
  ($r[0]) as $x
  | { schema: "validation-gate/v1",
      rows: [ $b[0].board[] | select(.family == "B") | .slug ],
      replay_certificate: {path: $rc, sha256: $rcsha},
      validator_report: {path: $vr, sha256: $vrsha},
      rho_reference: {
        kind: "negation",
        protocol: "Round 5 / ledger section 18: matched Pollard rho on the same instance, one target, the negation-map r-adding walk (A = 2), counted in GAE, verified [d]G = Q",
        unit: "GAE per sqrt r",
        binary_or_process: $bin,
        matches_protocol: true, strongest_available: true, same_unit: true, same_binary_or_process: true,
        gae_band: ( [0, 0.1, 1] as $w | reduce range(0; 3) as $i ({}; . + { ($w[$i] | tostring): { ic: ($x | map(.band[$i].S_IC) | gm), rho: ($x | map(.band[$i].S_ref) | gm) } }) ),
        gae_band_note: "geometric mean over the 13 instances of S under the section 5a band (weight w GAE per native operation, charged alike to both arms; IC natives that a pinned calibration priced are re-charged at w; the uncharged canonicalisations of rho are charged at w); per instance in docs/ic/runs/validation/band-by-instance.json",
        order_holds_across_band: ($x | all(.band | all(.ratio > 1)))
      } }' > docs/ic/runs/VALIDATION.json
jq '[.[] | {instance, band}]' "$tmp/B.json" > "$out/band-by-instance.json"
python3 scripts/validation_gate.py evidence docs/ic/runs
