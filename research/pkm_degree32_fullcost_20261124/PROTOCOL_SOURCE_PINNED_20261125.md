# Quiet-load full-rank successor to the censored 13-bit panel

The original `fullrank.jsonl` is immutable: its frozen executable is no
longer retained, its first interval call was censored, and the source-pinned
build gives unequal 16/18 factor-base sizes on seed `20261149`. This successor
uses the retained `ic` from commit
`a5503f4bb66ac3aa4eac3125fba01534d2574096`, SHA-256
`c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`.

Use 13-bit generic seed `20261276`, instance
`icv1-fp13-tm137-9a89da76`, where prior source-pinned preflights found 18
abscissae in both `prime-abscissa:below=32` and ordinary
`prime-isogeny:t=5,seed=5`. Repeat the preflight into this new store. Run
16 cold `walk` targets in each whole invocation, in control/candidate/
candidate/control order, with m=2 Semaev F4 chain degree 8,
`incremental-gauss-full-rank-checked`, 20,000 target trials, five seconds
per solver call, 16 same-target counted-rho runs, and a 900-second outer cap.

Before and during each timed invocation, check that Cargo, Rust, and other
`ic` benchmarks are idle. A competing process censors that invocation and
ends the qualifying panel; retain the partial raw output, command, exit,
wall, child CPU and process snapshot in the fresh append-only JSONL store.
Require all 16 planted scalars to be recovered, group-law verified, matched
across arms, and equal to the rho reference's planted list. Require factor
base size 18, LA rank 19 and `full_rank_reached=1` for every row.

Report per-call and mirrored charged GAE, external wall, child CPU, relation
trials, useful relations, F4 multiply-adds, LA row operations, rank, and
`s_over_rho`; include failed and censored attempts. The component gate is
both mirrored candidate/control GAE ratios below 0.9 and wall/CPU ratios
below 1. The cold-rho crossover gate additionally requires both candidate
aggregate `total_gae` values below their exactly matched rho GAE totals.
Keep prior 13-bit results separate from this fresh quiet-load store.
