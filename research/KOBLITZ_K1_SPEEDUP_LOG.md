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
| n23 | sp-c4-d5 + lean_layout + sib + walk_sib=8 + ladder_verify + orbit_key_reuse | 0.927 [0.847, 1.020] / 1.237 / 2.577 (band unresolved) | 0.236 / 0.325 / 0.788 | koblitz_k1_sibwalk_20261118 (validated, VAL-20261118-99f8e0; the w = 1 margin is entirely the 88 orbit-key squarings priced at 1 GAE each) |
| n29 | gen-c1 + lean_layout + sib | 0.875 [0.775, 0.969] / 1.048 / 1.878 | 0.070 / 0.090 / 0.229 | koblitz_lean_sib_confirm_20261118 (validated, VAL-20261118-9a42f2) |
| n37 | sp-c8-d5 + lean_layout + sib + walk_sib=8 + ladder_verify + orbit_key_reuse | 0.900 [0.825, 0.983] / 1.225 / 2.673 (band unresolved) | 0.516 / 0.683 / 1.327 | koblitz_n37_sibwalk_k8_20261118 (validated, VAL-20261118-24d204; real w = 1 gain about 0.08% once the 288 key squarings are priced at 0) |

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
| 2026-11-18 | koblitz_k1_sibwalk_20261118 | sibling giant-step walk (walk_sib) + reuse of formed points (ladder_verify, orbit_key_reuse); k, m from a pilot | n23 0.703 / 0.787 / 0.966; n29 0.873 / 0.915 / 1.013; n37 (k 8 -> 9) 0.785 / 0.874 / 1.050 | n23 passed, validated (VAL-20261118-99f8e0). Caveat: at w = 1 the walk (+14.7 GAE) and ladder (-15.0) cancel, so the whole margin is 88 removed field squarings priced at 1 GAE each; with them unpriced, w = 1 reads 0.9999 [0.9997, 1.0001], and there is no calibrated w_cal for binary-field squarings. The robust gains are the walk at w <= 0.1 and the ladder at every weight. Solve wall time 281.9 -> 262.0 us median; n29 failed at w = 1 (as predicted: walk neutral at w = 1, nothing to reuse); n37 failed at w = 1, and also at w = 0.1 in replicate 3 (pilot's k = 9; derived k = 8 counterfactual 0.736 / 0.817 / 0.977, unmeasured) |
| 2026-11-18 | koblitz_n37_sibwalk_k8_20261118 | same levers as koblitz_k1_sibwalk_20261118 at the incumbent k = 8, walk_sib = 8 fixed before data (no pilot), n37 only, fresh seeds | n37 0.724 [0.705, 0.745] / 0.808 [0.794, 0.823] / 0.977 [0.976, 0.979] | passed, validated (VAL-20261118-24d204; pooled and all 3 replicates; max upper 0.981). At w = 1: key reuse -288.0, ladder -25.9, walk +15.0 GAE per solve; with the field squarings priced at 0 (worst case) w = 1 reads 0.99916, so the order holds but the real w = 1 gain is about 0.08%. Ladder verify wall 7.7 -> 3.05 us; matches the count model; gap to rho-v5 1.243 / 1.517 / 2.735 -> 0.900 / 1.225 / 2.673 (band unresolved, not below rho) |
| 2026-11-18 | koblitz_k1_setup_w1_20261118 | lazy_neg (factor base stops forming point negatives) + orbit_ids (table build skips pairs given by Z[tau] identities) on each curve's best arm | n23 0.991 / 0.976 / 0.953; n29 0.986 / 0.952 / 0.873; n37 0.996 / 0.986 / 0.971 | passed 12/12 cells per curve [pcc:unverified], validation pending. Only orbit_ids saves group ops (the whole w = 0 gain); most of the w = 1 margin is removed point negations priced at 1 GAE; solve wall time unchanged within noise. Gap to rho-v5 after: n23 1.030 / 1.355 / 2.763 (v5 ahead), n29 0.981 / 1.125 / 1.823, n37 0.901 / 1.215 / 2.614 (unresolved). Benchmark record left open (benchrec close blocked by a malformed main record) |
