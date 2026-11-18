# Koblitz K1 small rows: speed-up log

This log tracks the speed-up line of work on the generator-orbit index-calculus arm on the
Koblitz K1 curves n23, n29 and n37. Each row is one frozen study.

- "Cost vs incumbent" is the new arm's paired cost divided by the incumbent's, at weights
  w = 0 / 0.1 / 1 of the §5a band.
- "Gap" is the new arm's cost divided by the rho reference's cost, from the same sessions.
- The latest validated arm on each curve is the incumbent for the next study.

Numbers marked [pcc:unverified] have no pcc claim. "Validated" means an independent validator
report exists, but the VALIDATION.json gate is not met: the raw sessions are only in local
scratch, because the artifact store has refused access since 2026-11-16. So none of these
rows is on the leaderboard yet.

Two rho references appear below:

- `rho.signed_frobenius_strong` is the leaderboard's current reference.
- `rho.signed_frobenius_v5` is cheaper on these curves. See
  `research/koblitz_rho_reference_20261115`. Which one the leaderboard should use is an
  owner decision that is still pending.

## Current best arm per curve

| curve | arm | gap to rho-v5 (w = 0 / 0.1 / 1) | gap to rho-strong (w = 0 / 0.1 / 1) | evidence |
|:--|:--|:--|:--|:--|
| n23 | sp-c4-d5 + lean_layout + sib | 1.370 / 1.631 / 2.762 | 0.341 / 0.419 / 0.823 | koblitz_lean_sib_confirm_20261118 (validated, VAL-20261118-9a42f2) |
| n29 | gen-c1 + lean_layout + sib | 0.875 [0.775, 0.969] / 1.048 / 1.878 | 0.070 / 0.090 / 0.229 | koblitz_lean_sib_confirm_20261118 (validated, VAL-20261118-9a42f2) |
| n37 | sp-c8-d5 + lean_layout + sib | 1.236 / 1.517 / 2.771 | 0.667 / 0.795 / 1.289 | koblitz_lean_sib_confirm_20261118 (validated, VAL-20261118-9a42f2) |

These arms are a Frobenius- and negation-folded baby-step giant-step under the IC pipeline.
That makes them a generic base, so they claim at T0 only. Their asymptotic ceiling is about
1.6x a folded rho.

## Study log

| date | study | lever | cost vs incumbent (w = 0 / 0.1 / 1) | verdict |
|:--|:--|:--|:--|:--|
| 2026-11-15 | koblitz_ksum_fold_study_20261115 | Frobenius-folded k-sum oracle | — | failed validation (VAL-20261115-f7df34); the gain came from the gen-orbit base |
| 2026-11-18 | koblitz_rho_reference_20261115 | re-check of the rho reference (v5 vs strong) | v5/strong 0.079 (n29) to 0.96 (n53) at w = 0 | recommendation only; owner decision pending |
| 2026-11-18 | koblitz_known_log_columns_20261118 (A0) | known-log columns, seeds [1]G..[k]G | n23 1.11 (null); n37 0.494 | A0 n23 null |
| 2026-11-18 | koblitz_known_log_columns_20261118 (A1) | spread seeds [2^(d(j-1))]G | n23 0.588 / 0.605 / 0.645; n37 0.381 / 0.398 / 0.442; n29 none | validated, VAL-20261118-4ae594 |
| 2026-11-18 | koblitz_n29_orbit_levers_20261118 | lean_layout + sib + tau_verify (n29) | 0.713 / 0.614 / 0.487 | validated, VAL-20261118-43658a; tau_verify is not an improvement (count relabel), so it is excluded |
| 2026-11-18 | koblitz_lean_sib_confirm_20261118 | lean_layout + sib, tau off (n23, n29, n37) | n23 0.770 / 0.753 / 0.720; n29 0.814 / 0.639 / 0.432; n37 0.703 / 0.732 / 0.796 | validated, VAL-20261118-9a42f2: passed on all 3 curves (12/12 cells each). At w = 1 the band prices each removed hash or curve check like a full group add; measured wall time per replicate is 0.79–0.93 of the incumbent |
