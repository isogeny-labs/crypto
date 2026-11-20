# EXP-KOBLITZ-REF-002 — the matched Koblitz rho reference with v5 fully charged

Frozen before any row (2026-11-20). Follows `research/koblitz_rho_reference_20261115/` (the audit).

## Owner decision (2026-11-20)

Below about `2^38` the matched rho reference for Koblitz rows is `min(strong, v5)`, with v5's jump-index selection
tallied first (audit section 5). Above it, strong stays the boundary reference. This study makes that tally, measures
both references again on the audit's curves and targets, and restates the distance of the validated IC arms.
Moving the reference changes the distance remaining, not any IC cost: nothing here is an IC improvement.

## Change under test

`ic_boundary::TunedWalker` (the walk of `rho.signed_frobenius_v5` and every tuned walk) now tallies each jump-index
selection — a shift and a mask or remainder of the point's mixed key — as two word operations, in the search phase's
`work_native` under `jump_select_word_ops` (priced one `word_op` each in `ecbench::work`), for every step and every
look-ahead re-test. Like the short-cycle window's compares it is never a counter, so committed records replay
unchanged. Strong's equivalent (`partition_hashes_uncharged`, 12 word operations) is unchanged.

## Arms (per curve, one ecbench session each; the audit's specs minus its two context arms)

`rho-strong` (`rho.signed_frobenius_strong`), `rho-v5` and `rho-v5-aa` (`rho.signed_frobenius_v5`, A/A),
`gen-c<c>` (the validated generator-orbit IC arm, `compact-orbit-scan:...,generator_orbit=1` with
`mitm-frobenius-layout-counted:m=2`), and `k1-l<d>` where the audit had it. Curves, columns, target seeds (48 public
targets), 3 rounds, 1 warm-up, alternating order and measurement seeds are the audit's: n17 (a = 1, c = 1),
n23 (a = 1, c = 3, 4, K1 d = 9), n29 (a = 1, c = 1, K1 d = 10), n31 (a = 0, c = 2), n37 (a = 0, c = 8).
`ecbench run --cpus none --allow-busy` (L0), `verify --replay-all`, `band` at w = 0, 0.1, 1, `compare` paired by target
with cluster-bootstrap 95% intervals (seed 20261002, 4000 resamples), as in the audit.

## Predictions

- **M1 (soundness)**: every solve of every arm verifies; every sampled replay reproduces; the v5 A/A ratio's interval
  contains 1 at all three weights.
- **M2 (only the tally changed)**: v5's median S at w = 0 equals the audit's v5 median S at w = 0 on every curve
  (the walk, seeds and targets are unchanged and the tally is priced only at w > 0). A difference invalidates the run.
- **M3 (v5 stays the cheaper reference below 2^28)**: v5 ÷ strong `< 1` with the interval excluding 1 at w = 0, 0.1
  and 1 on n17, n23, n29 and n31; reported, without a threshold, on n37.
- **M4 (distance, reported)**: the matched reference per curve and weight is the cheaper of strong and tallied v5;
  each IC arm's ratio to it is reported with intervals. Expected: n29 `gen-c1` ÷ matched at w = 0.1 in `[1.4, 1.9]`
  (1.69 against the untallied v5; the tally makes v5 dearer, so the distance can only shrink).

L0 only; counted GAE are deterministic, wall time is not used. No change to `docs/ic/*`, the leaderboard or
`boundary_targets.json` in this study; adopting the reference in those is a follow-up named in the report.
