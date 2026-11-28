# Protocol: projective split-chain qualification before an m=4 exponent test

**Frozen before implementing `mode=projective` or running any new solver cell.**

## 1. Purpose and claim boundary

The active repository goal is a classical prime-field ECDLP algorithm with
expected total work `r^o(1)`.  This protocol does not assume that goal has been
reached.  It tests one mechanism needed by a split-Semaev relation finder:
whether an explicit projective auxiliary, including the identity branch, can
change solver cost without losing signed, group-verifiable relations.

The existing `pkm-semaev` `mode=chain` is the baseline.  It supports `m=3`
with one affine intermediate abscissa and exact post-solver lift verification.
It currently rejects `m=4`.  The work is therefore split into a qualification
stage and a separately gated exponent stage.  A passing qualification result
is not a subexponential result.

## 2. Fixed curve, base, engine, and accounting

- Curve family: `curve25519_family_instance(k)` / the identical miniatures
  constructed by `src/bin/curve25519_family.rs`.
- Qualification curve: `k=20`, `p=1,048,573`, subgroup order `r=131,203`.
- Factor base: `bits`, abscissae in `[0, 2^t)`, with `t in {3,4,5,6}`.
- Solver: `f4_fp_tower::f4_tower`, release mode, one process and one Rayon
  thread, default degree bound 12, no learned trace, no guessing, no cofactor
  lift sharing.
- Baseline: `m=3,mode=chain`.
- Candidate: `m=3,mode=projective`.
- Primary work unit: F4 `F_p` multiply-adds.  System specialization, grid
  extraction, lift attempts, group additions, setup, maximum rows/columns,
  and wall time are reported separately and never silently omitted.

Matched target scalars use the existing `f4cost` LCG with seed `20261118`.
Each cell requests 32 random subgroup targets and 8 planted decompositions.
The per-cell wall cap is 900 seconds.  A target that reaches a solver bound is
censored rather than assigned a successful cost.

## 3. Projective formulation to implement

For the single `m=3` intermediate use homogeneous coordinates `(X:Z)`.
The affine chart is `Z=1`; the identity chart is normalized as `(1:0)`.
The selector is Boolean and the normalization equation is
`(1-Z)(X-1)=0`.  Each adjacent `S3` is bihomogenized by substituting `X/Z`
and clearing exactly the squared denominators.  The implementation may reuse
`s3_in_u_frac`; it must not divide by `Z`.

Every returned relation is lifted to signed factor-base points and re-added in
the group.  Solver consistency alone is never a relation.  Counters distinguish
affine-chart solutions, identity-chart solutions, failed lifts, and invalid
rows.  The full cost of all charts/selectors is charged.

## 4. Correctness gates before measurement

Native Rust tests must establish all of the following:

1. the projective system agrees with exhaustive signed group truth on a small
   prime curve for every target;
2. a planted ordinary split is accepted and group-verified;
3. a planted inverse pair reaches `Z=0`, is accepted, and group-verifies;
4. `(X,Z)=(0,0)` is excluded;
5. the existing affine mode remains byte-for-byte unchanged on its frozen
   snapshot test;
6. all 8 planted targets in every measured cell are recovered exactly.

Any failure is retained.  Witnesses, seeds, cell sizes, and thresholds are not
changed after a failure without a frozen addendum.

## 5. Qualification estimator and decision

For each target, record the candidate/baseline F4-multiply-add ratio when both
complete.  The primary constant-factor estimate is the geometric mean of the
paired ratios over `t>=4`, with a 95% bootstrap interval using 10,000
deterministic resamples (seed `20261118`).  A candidate qualifies for the m=4
stage only if:

- every planted and returned relation group-verifies and invalid rows equal 0;
- no cell has a larger censoring fraction than the baseline;
- the upper 95% bound of the paired work ratio is below `0.80`; and
- candidate setup plus unpriced native work is no more than 2x baseline.

Independently fit `ln(F4 multiply-adds per target)` on `ln(2^t)` over the four
complete cell means, with an ordinary two-sided 95% t interval.  For `m=3`, an
exponent-moving result under the PKM cost model requires the upper bound on the
slope to be below 0.  Anything else is `no m=3 exponent lever` or
`undetermined`, not subexponential.

## 6. Gated m=4 stage

Only after the correctness gates pass may the same projective representation
be extended to two intermediates and `m=4`.  Before any m=4 solver data, freeze
an addendum specifying exact recovery, cells, resources, and estimator.  Its
minimum exponent rule is the existing PKM rule: the upper 95% bound on the
solver-cost slope in factor-base size must be below `0.5`.  A claim about total
subexponential ECDLP additionally needs setup, enough relations, sparse linear
algebra, and target recovery all bounded by `r^o(1)`; a component slope does
not suffice.

## 7. Evidence package

Retain exact revision, toolchain, host limits, commands, per-target JSONL,
failures, and hashes.  Export an `experiment.export/v1` record with the
planned/attempted/completed/excluded flow.  Produce a Markdown report, editable
figure source, SVG/PNG, Typst source, inspected PDF, and a benchmark copy/index
entry because counted performance is measured.  All results remain
`[pcc:unverified]` unless separately registered and signed.
