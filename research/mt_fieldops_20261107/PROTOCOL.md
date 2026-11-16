# Multi-target IC in field operations: does the shared-inversion pair table beat rho? (protocol, frozen before the tune and holdout data)

Reopens the multi-target IC work stream of `research/ic_e222_m221_20261106/` (closed 2026-11-07: IC v3 was
1.03x to 1.07x the anchored batch rho and 1.40x to 1.53x a multi-target BSGS in counted group additions).

## Hypothesis and mechanism

In group additions the IC's pair-sum table and target trials cost one addition per x-class, exactly like a rho
database entry or walk step, so IC can at best tie a generic attack and pays its factor-base phase on top. In
field operations they do not: x(P+Q) and x(P-Q) share the inversion of x_Q - x_P, so two x-classes cost
I + 2M + 2S instead of 2(I + 2M + S), and the IC never needs y on its table or its target trials (its
progression W_k is shared by all targets). A sequential rho walk must carry y. Prediction: IC4 (this design)
costs less per target than the anchored batch rho (`rho`) at every I/M ratio (per sample (I + 2M + 2S)/2
against I + 2M + S, 0.58 at I = 3M and 0.52 at I = 30M), so IC4 / rho about 0.55 to 0.75 after IC's
factor-base phase. Against controls that get the same lever, the prediction
is weaker: `rho2` (rho whose every step also yields R - A) about 0.85 to 1.0; `bsgs2` (sum-set baby table,
paired giants) about 1.3 to 1.5 (BSGS tiles the group; random trials cannot).

## Arms (one binary, `examples/mt_fieldops.rs`, one counted u64 field layer)

- `ic4`: factor base = m smallest-x points (logs unknown; sqrt by counted Tonelli-Shanks), x-only table
  {x(F_i +- F_j), x(2F_i)} with shared inversions, factor-base logs from relations V_j +- W_k = +-(F_a +- F_b)
  solved on the signed relation graph, then paired trials x(Q_t +- W_k); m = ceil(mu (nT/2)^(1/4)), relations
  g m. **mu and g are chosen on the tune set (below) and frozen in `frozen.json` before the holdout.**
- `rho`: the anchored batch rho of `examples/ic_batch_rho.rs` (32 arms, restarts every 32 steps from 3-bank
  sums, x-only database with y kept), kappa in {0.5, 0.7, 1.0}.
- `rho2`: `rho` plus the sibling R - A in full per step (+2M + S), stored and looked up; same kappa grid.
- `bsgs2`: baby x-classes |k| <= Mx from {x(jU +- iG)} with shared inversions, Mx about mf sqrt(nT)/2,
  mf in {0.7, 1.0, 1.4}; paired giants x(Q_t +- k(2Mx+1)G), giants shared.
- Every arm: hits resolved by recomputing the two points (counted); every recovered log verified by a counted
  double-and-add; planted targets drawn from the same stream for every arm (uncounted).

## Cost unit

Counted I, M, S, hash operations H and solver edge operations L per run. Cost per target in GAE =
[(c_I I + M + S)/(c_I + 3) + h (H + L)] / T, i.e. one affine addition I + 2M + S = 1 GAE. c_I in {3, 10, 30}
(3 is a Montgomery-batched inversion, which every arm can use since all have independent parallel work);
h in {0, 0.1, 1} GAE per unpriced operation (AGENTS.md section 5a). The generic arms take their best
parameter per configuration at each (c_I, h) (favours them); IC uses one frozen (mu, g).

## Tune set (used only to choose mu, g)

Instances W22-0, W24-0, W25-0 (index 0, disjoint from the holdout), seeds 101 to 103, cells (22, 1024),
(24, 4096), (25, 16384); grid mu in {0.6, 0.8, 1.0, 1.2, 1.4} x g in {1.0, 1.5, 2.0}. Objective: mean over
the 9 tune configurations of cost(c_I = 10, h = 0.1) relative to the per-configuration best grid point.

## Holdout

Instances W{22,24,25}-1..11 (the instances of the earlier panels; the multi-target IC was designed on them in
earlier versions, disclosed), seeds 1, 2; cells (22, 1024), (24, 1024), (24, 4096), (24, 16384), (25, 4096),
(25, 16384): 22 paired configurations per cell, 10 runs per configuration (1 IC + 3 rho + 3 rho2 + 3 bsgs2).
All runs must verify every target; a failed or unverified run is reported, not dropped.

## Decision rule (fixed now)

Per cell and c_I: IC is "cheaper than X" only if the bootstrap 95% upper bound of the median per-configuration
ratio IC/X is below 1 at h = 0, 0.1 and 1; "dearer" if the lower bound is above 1 at all three; otherwise
unranked. The headline "IC beats rho" requires "cheaper than rho" in every cell at every c_I. The result is
reported against all three references; beating `rho` while losing to `rho2` or `bsgs2` is reported as such.

## Limits known in advance

Toy groups (2^22 to 2^26), u64 field (I, M, S are counted, not timed), local unisolated host (counts are
deterministic so host noise cannot change them), memory unbounded for every arm, own counters (no ecbench
harness, no validator yet), no isolab-eks / S3 / ecdb in this container (no AWS identity, no ecdb token).
