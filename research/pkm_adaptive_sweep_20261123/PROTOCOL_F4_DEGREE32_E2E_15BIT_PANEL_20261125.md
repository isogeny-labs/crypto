# 15-bit multi-instance closed-pipeline panel — 2026-11-25

The 15-bit rung is the first one where instance replication is possible without
selection games: unlike 14 bits (one roster curve), every CLI seed here maps to
a different curve, and 54 of the 61 seeds in the scan window have an
equal-size t=5 auxiliary seed.

## Frozen instance selection

Scan window: CLI seeds `20261300..20261360`. For each seed, measure
`prime-abscissa:below=32`, then take the **first** ascending t=5 auxiliary seed
(`0..20`) whose abscissa count equals the interval's on the same instance.
Ledger: `scan15_matches.jsonl`.

Selection rule, fixed before any timed call: **the first three matching seeds in
ascending order**. That gives

| CLI seed (target) | Instance | Abscissae | Aux seed |
|---:|:--|---:|---:|
| 20261300 | `icv1-fp15-tm103-a317aba6` | 19 | 8 |
| 20261301 | `icv1-fp15-tm87-adf2b6f1` | 17 | 1 |
| 20261302 | `icv1-fp15-t271-e83c6554` | 14 | 0 |

No cost was observed before this selection; the differing base sizes (19, 17,
14) are a consequence of taking seeds in order, not of choosing convenient
instances.

## Frozen run

Binary `c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`
(commit `a5503f4b`, `git_commit` null). Per instance: `--bits 15`,
`--family generic`, `--targets walk`, 16 repeats (16 independent cold solves),
`--linalg incremental-gauss-full-rank-checked`, `--max-trials 20000`,
`--solver-budget-seconds 5`, `--rho-runs 1`, order interval, candidate,
candidate, interval, whole-call cap 3600 s.

- interval: `prime-abscissa:below=32`, oracle
  `pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain`
- candidate: `prime-isogeny:t=5,seed=<aux>,aux=ordinary`, oracle
  `pkm-semaev:m=2,engine=f4,degree=8,mode=chain`

## Gate

Per instance: all four calls exit zero; both arms keep the instance's matched
abscissa count; all 16 rows per call have `recovered` set and `verified: true`;
`full_rank_reached` on every row; recovered scalars agree across the four calls
at each repeat index. Panel: combined charged GAE below 0.9 and every mirrored
pair below 1 on GAE.

## Why this is a stronger object than the 13-bit panel

Three independent curves, three different base sizes, one predeclared selection
rule, and a closed pipeline. It still is not sampling from a curve family: the
three seeds are the first matches in a window, and the window was chosen for
the scan, not by a random draw. `s_over_rho` is expected in the 1e6 range and
is reported, not gated.

## Cost expectation

The F4 matrix shape is fixed by arity, so multiply-adds per call are the same
127.3M (interval) and 3.96M (candidate) as at 13–14 bits; what grows is trials
per relation, roughly fourfold per two bits. One 15-bit solve is therefore
minutes, not seconds, and the panel is expected to take hours rather than
minutes.
