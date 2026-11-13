# VALIDATION.json for research/fes_then_batched_20261112 (validator code).
# Usage: jq -n --slurpfile band validation/band.json --arg cert SHA --arg rep SHA -f validation/make_validation_json.jq
$band[0] as $B
| ($B.instances | map({key: .name, value: (if .regime == "char2" then "negation"
                                           elif (.rho_method | test("Frobenius")) then "frobenius" else "negation" end)}) | from_entries) as $kinds
| {
  schema: "validation-gate/v1",
  scope: "Evidence for the study's rows (six instances x pair table, inherited-f4, fes-then-inherited, fes-then-batched). No leaderboard row is proposed (owner); this file is the gate evidence only.",
  replay_certificate: {path: "validation/replay-certificate.json", sha256: $cert},
  validator_report: {path: "validation/validator-report.json", sha256: $rep},
  rho_reference: {
    kind: "negation_frobenius",
    kind_per_instance: $kinds,
    kind_note: "Koblitz: the signed-Frobenius walk (A = 2n, the automorphism fold) and the negation walk (A = 2) on the same 16 seeds, the lower mean S prices the reference (negation at degrees 13, 17, 19, where the Frobenius walk's set-up dominates; signed-Frobenius at 23); random binary: negation, the only automorphism.",
    protocol: "README 'Protocol': ic bench --sweep, 16 matched rho runs in the same process, rho_reference_rule of each report (Koblitz: lower mean of signed-Frobenius and negation walks on the same seeds; random binary: negation only); one planted target per repeat, 3 repeats per IC configuration, verified recovery.",
    unit: "group-addition equivalents (GAE) per sqrt(r)",
    binary_or_process: "the same ic bench process per instance writes every IC row and the matched rho (producer binary sha256 1f566eed..., built at 4900647bcd per the benchmark record; replayed with the validator's build of d81dc1241b, sha256 0836dc46..., every count identical)",
    matches_protocol: true,
    strongest_available: true,
    same_unit: true,
    same_binary_or_process: true,
    gae_band: $B.gae_band,
    gae_band_basis: ($B.gae_band_basis + "; the IC arm is fes-then-batched"),
    gae_band_detail: $B.gae_band_detail,
    gae_band_rows: "validation/band.json (every instance, every arm, w = 0, 0.1, 1)",
    band_formula: "S(w) = mean over repeats of [G - C + w (U + n_C)] / sqrt r: G total GAE; C and n_C the GAE and count of the calibration-priced native counters (price_phase's ten counters with a non-null unit, pinned on Koblitz and measured on random binary, plus the solver's ops at ns_per_op); U the arm's unpriced counts (no *_uncharged counter occurs; work_native.table_insert_attempts behind counted:table_insert_attempts, the 2026-11-08 default reading, i.e. the pair table's per-step inserts). Rho: no unpriced counter emitted, U_rho = 0 (lower bound at w > 0).",
    order_holds_across_band: true,
    order_holds_across_band_means: "The matched rho is below fes-then-batched, and below every IC arm (pair table, inherited-f4, fes-then-inherited), at w = 0, 0.1 and 1 on all six instances; no IC arm is ranked ahead of rho anywhere. It does NOT mean fes-then-batched ranks first among the IC rows: see component_orders.",
    component_orders: {
      fes_then_batched_vs_inherited_f4: {
        per_instance: ($B.instances | map({key: .name, value: {order: .orders.ftb_vs_if4.order, verdict: .orders.ftb_vs_if4.verdict, ratio: .ratio_ftb_over_if4}}) | from_entries),
        strict_order_holds_across_band_on: $B.summary.ftb_vs_if4_strict_holds_on,
        order_holds_across_band: false,
        reading: "A tie at w = 0 on random 15, Koblitz 17, 19, random 21, Koblitz 23 (identical decisions; solver word XORs free), fes-then-batched lower at w = 0.1 and 1; at Koblitz 13 fes-then-batched is above at w = 0 (fes-f2 tried more targets) and below at w = 0.1 and 1: unresolved there. Weak order (<=) on five of six."
      },
      fes_then_batched_vs_pair_table: {
        per_instance: ($B.instances | map({key: .name, value: {order: .orders.ftb_vs_pair.order, verdict: .orders.ftb_vs_pair.verdict}}) | from_entries),
        strict_order_holds_across_band_on: $B.summary.ftb_vs_pair_strict_holds_on,
        order_holds_across_band: false,
        reading: "Flips on all six: fes-then-batched below the pair table at w = 0, above at w = 0.1 and 1. 'Lowest IC row on five of six' is a calibration-point statement, unresolved under AGENTS.md 5a."
      }
    },
    sensitivity: "rho charged k = 1 or k = n + 1 native operations per walk step (validation/band.json diagnostics.rho_ext_*) and fes-then-batched / inherited-f4 priced by the complete bitop/v1 solver tally (diagnostics.bitop_band_*): no order verdict changes.",
    caveats: [
      "3 repeats per IC configuration, 16 rho runs per walk; no confidence intervals",
      "local container, not isolated (operation counts only; wall time is not part of S)",
      "both rows of every pair (IC arm and matched rho, and every IC-vs-IC pair) come from one ic bench process per instance",
      "random-binary rows are priced by host-measured ratios: their S at the run's calibration point is not reproducible (replay: -20% to -33% on the algebraic rows); the band, computed from counts, is",
      "the matched rho emits no unpriced counter (canonicalisations, distinguished-point tests, Frobenius maps): its band is flat, a lower bound at w > 0",
      "discrete-log recovery is a known-answer check (recovered = planted per repeat); the reports carry no base point or target for an independent [k]P = Q check",
      "the validator holds no keyring key; under docs/SABOTAGE_RESISTANCE.md the independent booleans here are unsigned and pcc grades these proofs not_covered"
    ]
  }
}
