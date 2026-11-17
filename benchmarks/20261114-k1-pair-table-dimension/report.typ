#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set par(justify: true)
#show link: set text(fill: rgb("#08519c"))

= K1 pair-table factor base sized for n: benchmarks entry (2026-11-14)

*What this is.* A benchmarks copy of the two K1 Koblitz rows that landed on the
IC leaderboard in crypto PR \#4149 (merged 2026-11-14 21:10 UTC). Nothing was
re-run: every number below is copied from the frozen summary
`research/ml_cryptanalysis_pt_lgrow_k1_20261114/summary.json`, which the
leaderboard builder reads and cross-checks against the validation manifest
`fb-pt-lgrow-k1/VALIDATION.json` (validation `VAL-20261114-d9190c`, gate-valid).
The experiment is ml-cryptanalysis `experiments/fb-pt-lgrow-k1` at commit
`7d4e8e0f`, copied into this repository with its sessions, replay certificate and
validator report. Claim `PCC-20261114-8ba038`, standing unverified
(unsigned replay certificate) \[pcc:unverified\].

*Protocol.* `ic.pipeline` with `pdp3-koblitz:m=3,engine=rootfind-negfold` and a
`koblitz-standard-subspace` pair-table factor base, dimension 7 (before) against the
dimension chosen for n (9 at n = 23, 10 at n = 29), and the reference
`rho.signed_frobenius_strong` (all 2n negation × Frobenius automorphisms), all three
arms in the same ecbench session on the same 8 targets × 5 rounds (40 verified
solves per arm). One target, cold. Unit: ecbench counted GAE, S = GAE / sqrt(r);
IC native operations and rho's uncharged counters priced at w = 0, 0.1 and 1 GAE
each (AGENTS.md §5a). ecbench sha256 `a0be1b2a…8796d29`; one RunPod CPU pod,
isolation L0 (counted cost only, wall time not certified).

#figure(image("gap.svg", width: 100%),
  caption: [Gap to matched strong signed-Frobenius rho, bits (log2 of IC S over rho S), before
  (dimension 7) and after, with the recorded 95% intervals. Source `summary.json`.])

#table(columns: 6, align: (left, right, left, left, left, left), stroke: 0.4pt,
  [*curve*], [*n*], [*r*], [*dimension*], [*gap to rho, bits, w = 0 / 0.1 / 1*], [*dimension 7, bits*],
  [`icv1-f2m23-tm5197-1f85e9e1`], [23], [2^22.00], [9], [5.59 / 5.66 / 5.95], [6.91 / 6.94 / 7.09],
  [`icv1-f2m29-tm40309-30c52b96`], [29], [2^15.37 (cofactor 12646)], [10], [9.47 / 9.52 / 9.86], [10.36 / 10.46 / 11.06],
)

#table(columns: 5, align: (left, right, right, right, right), stroke: 0.4pt,
  [*S (GAE / sqrt r), w = 0 / 0.1 / 1*], [*IC after*], [*IC dimension 7*], [*rho-strong*], [*after - before, bits [95% CI]*],
  [n = 23, w = 0], [43.90], [109.35], [0.908], [1.32 [1.26, 1.36]],
  [n = 23, w = 0.1], [49.39], [120.22], [0.978], [1.28 [1.23, 1.33]],
  [n = 23, w = 1], [98.88], [218.08], [1.601], [1.14 [1.08, 1.20]],
  [n = 29, w = 0], [3181.6], [5891.5], [4.490], [0.89 [0.67, 1.06]],
  [n = 29, w = 0.1], [3375.5], [6478.3], [4.592], [0.94 [0.72, 1.11]],
  [n = 29, w = 1], [5119.8], [11759.0], [5.510], [1.20 [0.98, 1.38]],
)

*Reading.* At fixed n, sizing the pair table for n lowers the cost against
dimension 7 at every weight; the reduction's 95% interval excludes zero in every
cell. This is an engineering improvement (parameter re-selection) over the
dimension-7 configuration at fixed n. Strong rho is cheaper than both arms at every
weight: the gaps (5.59 to 9.86 bits after) are the distance remaining, and the
order of IC against rho holds across the band. In the calibration-free bitop/v1
cross-check the gaps are 5.52 (n = 23) and 9.23 (n = 29) bits after, 5.96 and
10.13 before.

*Scope and limits.* n is the field degree. K1 n = 29 has r = 42457 = 2^15.37,
smaller than K1 n = 17's subgroup, so the two rows are not a size ladder. The slope
statistic D of the source report equals (R_n23 + 7 R_n29) / 84 by construction and
is not a row. The leaderboard's earlier K1 n = 23 figure (table C) is a 32-target,
pre-gate batch in another unit; these rows are not ranked against it.

*Evidence.* Study `research/ml_cryptanalysis_pt_lgrow_k1_20261114/` (README, summary,
`summarize.py --check`); evidence directory `fb-pt-lgrow-k1/` (PROTOCOL.md,
report.md, results.json, sessions and sizing records in the artifact store,
`validation/replay-certificate.json`, `validation/validator-report.json`,
`validation/VALIDATOR.md`). This entry's `VALIDATION.json` pins the study's replay
certificate and VALIDATOR.md in place and a path-rebased copy of the validator
report (`path_rebase` names the source file and its sha256; no other field changed).
The source study's own figure, `fig_lgrow_gap.svg`/`.png` (gap against n = 17, 19,
23, 29 for both arms), is copied here unchanged. Graph source: `plot.py` in this
directory, which only plots values read from `summary.json`.
