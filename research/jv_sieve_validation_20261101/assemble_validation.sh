#!/usr/bin/env bash
# Assemble the three passing independent panel artifacts into validation-gate/v1.
set -euo pipefail

STUDY=research/jv_sieve_validation_20261101
VAL=$STUDY/validation
mkdir -p "$VAL"

for p in 1009 1511 1823; do
  test -s "$STUDY/runs/p$p/analysis.json"
  test -s "$VAL/replay/p$p/replay-certificate.json"
  test -s "$VAL/p$p/validator-report.json"
done

jq -s '
  {
    schema:"jv_sieve_validation_replay/v1",
    status:(if all(.[]; .status == "verified") then "verified" else "fail" end),
    independent:true,
    independence_basis:"three separate clean workflow jobs after producer artifacts were sealed",
    panels:map({p,status,source_commit,producer_sha256,kernel}),
    runs:[.[] | .runs[]],
    scalars_verified:all(.[]; .scalars_verified == true),
    counts_identical:all(.[]; .counts_identical == true)
  }' \
  "$VAL/replay/p1009/replay-certificate.json" \
  "$VAL/replay/p1511/replay-certificate.json" \
  "$VAL/replay/p1823/replay-certificate.json" \
  > "$VAL/REPLAY_CERTIFICATE.json"

jq -s '
  {
    schema:"jv_sieve_validation_validator/v1",
    status:(if all(.[]; .status == "pass") then "pass" else "fail" end),
    independent:true,
    independence_basis:"independent verifier executable in three clean workflow jobs; it does not call producer analysis",
    raw_data:[.[] | .raw_data[]],
    headline:[.[] as $panel | $panel.headline[] | .name = ("p" + ($panel.p|tostring) + "_" + .name)],
    panels:map({p,status,scalars_verified,tolerance})
  }' \
  "$VAL/p1009/validator-report.json" \
  "$VAL/p1511/validator-report.json" \
  "$VAL/p1823/validator-report.json" \
  > "$VAL/validator-report.json"

replay_sha=$(sha256sum "$VAL/REPLAY_CERTIFICATE.json" | awk '{print $1}')
validator_sha=$(sha256sum "$VAL/validator-report.json" | awk '{print $1}')
order_holds=$(jq -s 'all(.[]; all(.comparison.gae_band[]; .result.ratio < 1))' \
  "$STUDY/runs/p1009/analysis.json" "$STUDY/runs/p1511/analysis.json" \
  "$STUDY/runs/p1823/analysis.json")
by_prime=$(jq -s 'map({key:(.p|tostring),value:{verdict:.comparison.verdict,
    gae_band:(.comparison.gae_band | map({key:(.w_gae|tostring),
      value:{ic:.ic_mean,rho:.rho_mean,ratio:.result.ratio,ci95:.result.ci95}}) | from_entries)}}) | from_entries' \
  "$STUDY/runs/p1009/analysis.json" "$STUDY/runs/p1511/analysis.json" \
  "$STUDY/runs/p1823/analysis.json")
band1823=$(jq '.comparison.gae_band | map({key:(.w_gae|tostring),value:{ic:.ic_mean,rho:.rho_mean}}) | from_entries' \
  "$STUDY/runs/p1823/analysis.json")

jq -n \
  --arg schema 'validation-gate/v1' \
  --arg replay_sha "$replay_sha" --arg validator_sha "$validator_sha" \
  --argjson order_holds "$order_holds" --argjson by_prime "$by_prime" \
  --argjson gae_band "$band1823" \
  '{
    schema:$schema,
    rho_reference:{
      kind:"negation",
      protocol:"PROTOCOL.md: R3 corrected-hash strong/equal-advice rho (negation, batch 16, r=256, matched challenge curve and target)",
      unit:"bitop/v1 in the same producer process; word/hash work repriced identically at 0, 0.1 and 1 GAE",
      binary_or_process:"Within each panel, IC and R3 are emitted by the same jv_cover_ic_vs_rho_bitops binary; execution.json records its SHA-256",
      matches_protocol:true,
      strongest_available:true,
      strongest_available_basis:"The prospective symmetry diagnostic has j in neither F_(p^2) nor F_(p^3) for every used instance; negation is the available fold",
      same_unit:true,
      same_binary_or_process:true,
      band_p:1823,
      gae_band:$gae_band,
      by_prime:$by_prime,
      order_holds_across_band:$order_holds
    },
    replay_certificate:{path:"validation/REPLAY_CERTIFICATE.json",sha256:$replay_sha},
    validator_report:{path:"validation/validator-report.json",sha256:$validator_sha}
  }' > "$STUDY/VALIDATION.json"

python3 scripts/validation_gate.py evidence "$STUDY"
