# Curve25519 projective split qualification

The frozen `m=3` projective-intermediate candidate is **not qualified for an
`m=4` exponent test**. It preserved exact relation recovery on every
completed cell, but did not reduce counted F4 work and could not complete the
larger cells. `[pcc:unverified]`

## Result

| factor-base bits | chain F4 mul-adds/target | projective F4 mul-adds/target | ratio | outcome |
|---:|---:|---:|---:|:--|
| 3 | 111,785,088.281 | 111,909,801.281 | 1.001116 | 32/32 random and 8/8 planted complete per arm |
| 4 | 22,255,293,450.656 | 22,265,619,555.656 | 1.000464 | 32/32 random and 8/8 planted complete per arm |
| 5 | — | — | — | both arms timed out at 900 s before one target |
| 6 | — | — | — | both arms were memory-killed before one target |

The descriptive geometric mean of the 32 paired `t=4` candidate/baseline
ratios is `1.000463984`, with deterministic bootstrap interval
`[1.000463984, 1.000463985]`. The frozen qualification target required the
upper bound to be below `0.80`. The four-point solver-cost slopes are
undefined because `t=5` and `t=6` have no completed means. This is therefore
neither an exponent improvement nor evidence of a subexponential ECDLP
algorithm.

## Correctness boundary

`pkm-semaev:mode=projective,m=3` replaces the affine intermediate abscissa
with normalized homogeneous coordinates `(X:Z)`. The Boolean rule `Z^2=Z`
selects the affine chart `Z=1` or identity chart `(1:0)`; `(1-Z)(X-1)=0`
excludes `(0:0)`. Both adjacent `S3` equations are bihomogenized without
dividing by `Z`. Candidate rows still pass exhaustive extraction, sign lift,
and exact group re-addition before a relation is returned.

The release tests cover an explicit signed inverse-pair identity witness,
planted decompositions, and agreement among affine-chain, projective-chain,
full-`S4`, and exhaustive-grid verdicts on 40 deterministic targets. All
completed measurement cells recovered 8/8 planted relations, returned no
invalid rows, and matched target scalars and coordinates. Projective planted
hits exercised both charts: 6 affine/2 identity at `t=3`, and 7 affine/1
identity at `t=4`.

The task branch itself still encounters unrelated root build errors already
tracked in `docs/ACTIVE.md`. The test and measurement artifacts used the
task diff over compile-repair PR #4650 in a disposable worktree, with the
exact hashes and unrelated shim disclosure retained in
[`raw/qualification-tests.json`](raw/qualification-tests.json). This does
not clear the task branch root gate.

## Evidence

- frozen protocol: [`PROTOCOL.md`](PROTOCOL.md) and [`protocol.json`](protocol.json)
- machine export and run flow: [`export.json`](export.json) and [`raw/attempts.json`](raw/attempts.json)
- analysis and per-target records: [`evidence/analysis.json`](evidence/analysis.json), [`evidence/per-target.jsonl`](evidence/per-target.jsonl)
- visual/report assets: [`evidence/figure.svg`](evidence/figure.svg), [`evidence/figure.png`](evidence/figure.png), [`evidence/report.typ`](evidence/report.typ), [`evidence/report.pdf`](evidence/report.pdf)

Empty `t=5`/`t=6` aggregate files are retained and SHA-bound as censored
outcomes. No censored cost is imputed. The independent review directory
records that this package remains unverified.
