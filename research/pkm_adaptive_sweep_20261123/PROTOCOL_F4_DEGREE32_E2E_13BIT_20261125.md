# End-to-end cold-solve comparison at 13 bits — 2026-11-25

Question this answers: does the degree-32 equal-size component lead survive a
**complete cold solve** — relation collection, rank-20 linear algebra and
scalar recovery, all charged — or does it disappear once the pipeline is
closed? The three published equal-18 panels only compare relation extraction.

## Frozen design

Same binary and instances as `RESULT_F4_DEGREE32_SOURCE_PINNED_EQUAL18_20261125.md`,
binary `c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`
(commit `a5503f4b`, `git_commit` null). Instances
`20261185`, `20261221`, `20261276`; in each, the isogeny and interval bases
have 18 abscissae, so the matrix has 20 columns including the target column.

Per instance, one CLI seed, 16 repeats (16 independent cold solves), order
**interval, candidate, candidate, interval**:

- interval: `prime-abscissa:below=32`, oracle
  `pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain`
- candidate: `prime-isogeny:t=5,seed=5,aux=ordinary`, oracle
  `pkm-semaev:m=2,engine=f4,degree=8,mode=chain`
- both: `--linalg incremental-gauss-full-rank-checked --max-trials 20000
  --solver-budget-seconds 5 --rho-runs 1 --targets walk`

The only difference from the published panel is the closed pipeline
(`--linalg` full-rank and a 20,000-trial cap, which the 16-row panel needed
3,000–6,000 of anyway) and the 16 solves rather than 16 single targets.

## Gate

Passes only if, for every instance: all four calls exit zero; both arms stay
at 18 abscissae; all 16 rows per call have `recovered` set and
`verified: true`; the recovered scalar agrees across the four calls at each
repeat index; the linear algebra reports `full_rank_reached` for every row;
and both mirrored pairs have a full-pipeline GAE ratio below 1 with the
combined ratio below 0.9.

`s_over_rho` is reported, not gated: at this size it is expected to stay in
the 1e5–1e6 range and cannot be a crossover.

## What it can and cannot show

The linear algebra, target walk and failure accounting are the same code on
both arms, so a pass attributes the difference to the factor base and
decomposition oracle only. A pass is still a 13-bit statement about three
selected instances with one map seed; it is not instance replication, not a
14-bit result, and not evidence about s59, P-192 or the NIST curves. It is the
first closed-pipeline number this lane can report, which is a different and
stronger object than the component ratios.
