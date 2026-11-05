# Protocol: torsion anchors at the N83 cofactor-to-base ratio (proxy on a = 0, n = 37), frozen before data

Frozen 2026-11-01, before the first run in this protocol.

## Why a proxy
The request was torsion anchors on the n = 83, a = 1 Koblitz curve (cofactor 1,128,547,018). The
producer cannot run it, and no variant could finish it: `ic_m2_cycle` asserts `3d - 1 <= 63` and
uses a 64-bit field; n = 83 needs d = 28, so 83 unknowns. Even with a wider field, the solve
takes about `2^(n - d + 1) = 2^56` trials (about 7e16; about 5,700 core-years at 4e5 trials per
second), over a base of about `2^27` columns (about 3 GB). So the real curve is out of reach.

What the real curve does have is a particular regime: cofactor `h` about 1.1e9 against about
`2^27` = 1.3e8 columns, so **h / K about 8**. The two earlier measurements (cofactor 4, K up to
8,111; cofactor 596, K = 2,011) had h / K between 0.0005 and 0.3. On a = 0, n = 37
(h = 596) the number of columns is `K about 2^(d-1)`, so lowering the factor-base dimension `d`
raises h / K while the producer still finishes. This protocol measures torsion on and off across
that range, including the N83 ratio.

## Question
At fixed curve (a = 0, n = 37, h = 596), how does the cost of a cold single-target solve with
torsion anchors compare with the cycle stop alone as h / K goes from about 1 to about 9?
Claim class: comparative (paired), exploratory, not for the leaderboard.

## Arms and design
- Arm OFF: `ic_m2_cycle 0 37 <d> 1 <seed> 0 <out>`. Arm ON: same with `torsion = 1`. Same
  binary (SHA-256 recorded), same host, one target per process.
- d in {10, 8, 7}. Expected K about 512, 128, 64 (recorded from `ic_columns`), so h / K about
  1.2, 4.7, 9.3. The merged d = 12 record (h / K about 0.3, 8 seeds, same seeds) is the reference
  row and is not rerun.
- Pairs: 6 seeds per d: 20261024 and 20261101 to 20261105, one target (ti = 0) each. The secret
  depends only on the seed, so both arms solve the same discrete log at the same d.
- Order: a job queue sorted by d descending in cost (d = 10, then 8, then 7), 4 processes at a
  time (4 vCPU, one thread each); within a d, ON and OFF of one seed are adjacent. Trials are
  counted, so order cannot move them; wall time is secondary and noisy.
- The same process runs strong rho on the same target (`rho_s`).

## Instrument check (before the sweep)
The binary was built from commit 0ef7595c9c; main has since gained 408 added lines in
`src/cryptanalysis/koblitz_index_calculus.rs` (no deletions). Before any sweep run counts, seed 20261024
at d = 12 with torsion on (`ic_m2_cycle 0 37 12 1 20261024 1 <out>`) must give exactly 14,720,702
trials, the merged record. Any other count means the binary or the dependency differs, and the sweep
is not started until that is resolved and reported.

## Metrics
- Primary: decomposition trials to recover `t` (`ic_trials`). Secondary: wall `ic_s`,
  `ic_relations`, `ic_columns`, `ic_largest_component`, `ic_equations`, `ic_useless_cycles`.
- Every recovered scalar must satisfy `[t]G = T` (`ic_verified`); a failing row is excluded and
  reported.

## Cap and censoring
- Each run has a hard cap of 1,800 s (`timeout`). A capped run has no trials count: it is a
  censored lower bound of 1,800 s. Report, for each (d, arm), how many of 6 finished.
- No run is restarted or given a longer cap after seeing results.

## Analysis (frozen)
- Per (d, arm): finished / 6, mean and range of trials over finished runs, mean relations, mean
  largest component, K, and h / K (h = 596 divided by the recorded K).
- Per d, over pairs where both arms finished: geometric mean of ON / OFF trials with a 95%
  bootstrap interval (10,000 resamples, seed 20261101). With fewer than 4 finished pairs the
  interval is not reported. Pairs with a censored arm are listed with their lower bound.
- A trend statement across d is made only if the per-d summaries are ordered the same way at all
  three values; otherwise the result is reported as no consistent trend.

## What this does not do
- It does not run, and does not estimate the success of, torsion anchors on n = 83. The N83
  projection stands at about `2^56` trials, and a plausible torsion effect of 0.1x to 10x leaves
  it at 570 to 57,000 core-years.
- The cofactor's structure differs (h = 596 = 4 x 149 here; N83's 1.13e9 factors differently),
  K scales as `2^(d-1)` here but depends on the lift-x density, and n differs: matching h / K is
  one necessary condition for the regime, not a proof that it transfers.
- Six pairs per d, heavy-tailed trials. Local shared 4-vCPU container, not the default
  isolab-eks route (no AWS identity in this session). Exploratory; no replay certificate or
  validator report.
