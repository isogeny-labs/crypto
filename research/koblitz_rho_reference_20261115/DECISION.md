# Owner decision: matched rho reference for small Koblitz rows (2026-11-19)

**Approved by the repository owner in-session on 2026-11-19.** Recorded here because the
repository has no central owner-decision register; this directory holds the evidence the
decision rests on (`README.md`, `results.json`, `PROTOCOL.md`).

## Decision

For index-calculus leaderboard rows on Koblitz curves (K1 / `icv1-f2m*` signed-Frobenius
groups) with **r below about 2^38**, the matched rho reference is

    min(rho.signed_frobenius_strong, rho.signed_frobenius_v5)

per curve and per band weight w in {0, 0.1, 1} (AGENTS.md §5a), the smaller priced cost of the
two arms measured in the same session as the IC arm.

## Scope

- Koblitz rows with log2 r < 38 only. At and above about 2^38 `rho.signed_frobenius_strong`
  remains the reference (the two coincide at w = 0 from about 2^39: v5/strong 0.94 at n41,
  0.96 at n53, README section 3).
- Table A (clock-unit one-target rho, rule v8/v9) and Table C (32-target batch rho) are not
  affected: they use different rho references.
- The validation gate (`docs/VALIDATION_GATE.md`) is unchanged: no row reaches the leaderboard
  without `VALIDATION.json`, a replay certificate, a validator report and a matched rho
  reference.
- Only v5 measured **in the same session** as the IC arm may set the reference. A row whose
  evidence has no in-session v5 keeps strong and is labelled "rho reference under review:
  strong only, v5 not measured in-session"; the cross-session v5/strong multiplier of the
  re-measurement may be shown beside it as context, never applied.

## Condition

v5 first gets the per-step jump-index tally that strong records as
`partition_hashes_uncharged` (README section 1, finding iii), so the §5a band prices both arms'
per-step native work alike. Implemented 2026-11-19: `rho.signed_frobenius_v5` reports
`partition_hashes_uncharged` = one per walk step (its `steps` counter), listed in
`cost.unpriced`; the walk, outcome, phases and group-operation counts are unchanged (unit test
`v5_jump_index_tally_adds_one_counter_equal_to_steps_and_changes_nothing_else` in
`src/cryptanalysis/ecbench/methods.rs`). v5 records written before the tally carry the same
quantity as their `steps` counter; a reader that prices such a record adds `w · steps` (an
exact per-record derivation, not a ratio).

## What is not an improvement

Changing the reference changes the distance-to-rho column only. Under the success criterion
(AGENTS.md, "Relabelling and accounting changes are not improvements") it is an accounting
change: it improves no IC arm and is never reported as progress. Since v5 is cheaper than strong
below 2^38, the change *increases* the reported distance to rho on these rows.

## Evidence

- `research/koblitz_rho_reference_20261115/README.md` (audit, v5/strong 0.079 at n29 to 0.96 at
  n53 at w = 0), `results.json` [pcc:unverified].
- Re-measurement with the tallied v5: `research/koblitz_v5_tally_remeasure_20261119/`.
