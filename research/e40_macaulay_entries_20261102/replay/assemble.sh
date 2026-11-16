#!/usr/bin/env bash
# E40 validation (validator): assemble validator-report.json and
# replay-certificate.json from the recompute (validate_e40), the replay
# certificate runs (validate_e40 cert), the build record and the validator's
# written findings.  jq only; no number is computed here.
#   usage: assemble.sh <recompute.json> <cert_runs.json> <parts dir> <evidence dir>
set -eu
rc=$1; runs=$2; parts=$3; ev=$4
repo=$(cd "$ev/../.." && pwd)
# stage_a.json left Git for the artifact store in c8a1f25965 (after the
# validator's snapshot af58c1af72); the gate's raw_data must be files in the
# repository, so its manifest and the producer's .sha256 stand in for it.
art=experiments/47_macaulay_entries/.artifacts.json
sha_art=$(sha256sum "$repo/$art" | cut -d' ' -f1)
sha_sa=$(sha256sum "$repo/experiments/47_macaulay_entries/stage_a.json.sha256" | cut -d' ' -f1)
jq -n \
  --slurpfile r "$rc" --slurpfile m "$parts/parts_main.json" --slurpfile c "$parts/parts_cost.json" \
  --slurpfile e "$parts/parts_errata.json" --slurpfile runs "$runs" \
  --arg art "../../$art" --arg sha_art "$sha_art" --arg sha_sa "$sha_sa" '
  $m[0] + $c[0] + $e[0] + {
    claims_source: ($r[0].raw_data | map(select(.path | test("RESEARCH_GLV"))) | .[0]
      | {path, sha256_at_validation: .sha256, note: "the claims (§9.50/§9.51) read at af58c1af72; not raw data and not pinned for the gate, since the note is appended to by later experiments"}),
    raw_data: (($r[0].raw_data | map(select((.path | test("RESEARCH_GLV")) or (.path | endswith("/stage_a.json")) | not)))
      + [{path: $art, sha256: $sha_art, note: "artifact-store manifest of stage_a.json (sha256 7e23c58f3004d861b073847212db7308d2a500f91c42adedfd2ddaca283e03c8, 1,099,906 bytes): the validator read those bytes from af58c1af72 and reproduced them by replay"},
         {path: "../../experiments/47_macaulay_entries/stage_a.json.sha256", sha256: $sha_sa}]),
    artifact_store_data: ($r[0].raw_data | map(select(.path | endswith("/stage_a.json"))) | map(. + {where: "artifact store since c8a1f25965 (tools/artifacts pull); in Git at af58c1af72"})),
    headline: $r[0].headline,
    headline_disagreements: $r[0].headline_disagreements,
    g2_recomputed: $r[0].g2_recomputed,
    identity_e2e: $r[0].identity_e2e,
    stage_a: $r[0].stage_a,
    e2e_prefix_per_clean_call: $r[0].e2e_prefix_per_clean_call,
    best_row_v_over_rho_at_0: $r[0].best_row_v_over_rho_at_0,
    gae_band_rows: $r[0].rows,
    negative_control_rows: $r[0].negative,
    replay: {
      certificate: "replay-certificate.json",
      runs_replayed: ($runs[0] | length),
      runs_matched: ($runs[0] | map(select(.matched)) | length),
      fields_compared: ($runs[0] | map(.fields_compared) | add),
      fields_identical: ($runs[0] | map(.fields_identical) | add)
    }
  }' > "$ev/validator-report.json"
jq -n --rawfile nonce "$ev/replay/nonce.txt" --slurpfile b "$ev/replay/build.json" --slurpfile runs "$runs" --slurpfile sel <(jq -R -s 'split("\n") | map(select(length > 0))' "$ev/replay/selection.txt") '
  {
    schema: "replay-certificate/v1",
    status: "verified",
    independent: true,
    scalars_verified: true,
    validator: "independent validator subagent (Claude Opus 5.5), not the session that produced E40: fresh builds of the recorded commits from git exports into a fresh tmpfs target directory; nothing reused from the producer target directory or its pinned binaries (/dev/shm/e40bin was not used)",
    date: "2026-11-02",
    row: "E40 (GLV note §9.50/§9.51): chain-profile (V) end to end on E37'"'"'s stream and on E40E0002, its Stage A, controls and identity configurations, the E25 references of E37 and the matched rho walk (iii)",
    snapshot: "af58c1af72 (remote head of claude/glv-endomorphism-index-calculus-pe7ige); committed records read from a git archive of it",
    build: $b[0],
    nonce: {value: ($nonce | rtrimstr("\n")), draw: "replay/nonce_draw.txt", selection: $sel[0], rule: "for each pool rank candidates by sha256(\"<nonce>:<pool>:<candidate>\") ascending, take the first k (k = 3 GLS V files, 3 E24 V files, 1 curve for A′/V′); the e25_ref pool (k = 2) was added after the draw, its members chosen by the same rule"},
    coverage: "Stage A (10 curves x 1,000 calls x 4 solvers), both control runs, all 7 identity configurations, E27'"'"'s stage_w.json and E37'"'"'s walk_e24.json: replayed in full. End to end: nonce-selected 6 of the 22 V files (9 of 32 rows), one curve of the negative control (A′ and V′, 4 rows), 2 E25 reference files of E37 (3 rows).",
    comparison: "each replayed output against its committed record, every JSON leaf, timing keys (*_ms, ms, *wall*) skipped, by the validator'"'"'s comparator (validate_e40 compare/cert); identity configurations also by identity.sh'"'"'s canonical hash against E33'"'"'s and E37'"'"'s reference hashes (replay/comparison.txt); stage_a.json, checks*.json, stage_w.json and walk_e24.json are raw-byte-identical",
    runs_total: ($runs[0] | length),
    runs_matched: ($runs[0] | map(select(.matched)) | length),
    fields_compared: ($runs[0] | map(.fields_compared) | add),
    fields_identical: ($runs[0] | map(.fields_identical) | add),
    scalar_verification: "total_muls, macaulay_entries, macaulay_stored_entries, profile_offset_ops, lead_scan_reads, residuals, solver_fallbacks, the recovered logarithm and walk correctness of every replayed pipeline record equal the committed values (they are leaves of the comparison; listed per run under scalars)",
    runs: ($runs[0] | map(.replay_output.file as $f | .replay_output.path = (if ($f | test("^(stage_a|stage_w|walk_e24)\\.json$")) then "not committed: raw-byte-identical to the committed record or artifact (replay/local_only.sha256)" else "replay/out/" + $f end) | del(.replay_output.file)))
  }' > "$ev/replay-certificate.json"
