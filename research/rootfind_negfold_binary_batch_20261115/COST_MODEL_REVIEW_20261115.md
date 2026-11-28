# K13 binary batch collector: cost-model boundary review

**Status:** source-level accounting review, 2026-11-15 PST. No scientific
execution or performance observation is reported here. The visual is
[editable DOT](cost-model-boundary.dot), [SVG](cost-model-boundary.svg) and
included in [PDF](COST_MODEL_REVIEW_20261115.pdf).

## Question and exact scope

Can the fixed `bitop/v1` primary criterion in the unrun
[`PROTOCOL.md`](PROTOCOL.md) detect the candidate's shared inversions and
lazy x-only sums? The panel is the registered Koblitz E0 curve over
`F_(2^13)`, `icv1-f2m13-t181-515ee569`, subgroup order `r=2003`, the
33-point dimension-five standard-subspace base, eight fresh public targets,
six measured rounds and one warmup. The current candidate source is bound
by [`publication-amendment-2.json`](publication-amendment-2.json) to
`99ecffd3172302f05d3df47bfc2eb810c6a05aa2`. The earlier scalar
versus batch revisions have separate receipts. No session directory or
outcome row exists for this panel.

## Checked sources and method

- **Internal, current source:** `src/cryptanalysis/koblitz_fast.rs` defines
  `add_many_lazy`: the sum's x-coordinate is final, y is left unfinished,
  and a shared inversion forms each row or eight-point scan block. The
  source and release parity tests in `src/cryptanalysis/ic_boundary.rs` and
  `src/bin/ecbench/pdp3_koblitz.rs` check key and first-witness identity
  for the tested K13/K17 tables and 263 K13 scan points.
- **Internal, current source:** `src/cryptanalysis/ic_boundary.rs` records
  the same abstract `GroupOps` additions for scalar and batched table
  construction; `src/bin/ecbench/pdp3_koblitz.rs` charges a completed
  eight-point scan block in full and records table insertion attempts.
  `BinaryGroup::counts_field_ops()` remains false.
- **Internal, current source:** `src/cryptanalysis/ecbench/work.rs::price`
  converts each phase's `adds` into `group_add` primitives;
  `docs/units/BITOPS.md` fixes a binary affine addition's weight at
  `I + 2M + S + 9A`. The rootfinding adapter's setup native map records
  table size and build additions, not the actual inversions and products
  performed by `FastCurve`. `src/cryptanalysis/semaev_decomp.rs::Gf2`
  does not feed a field-operation tally to this adapter.
- **KB pointer:** revision `9141e386254340a9bdba3036f2bb706a`,
  `source:8cde248d5b975dc7351e5e2e4d528f2e` at
  `crypto:docs/units/BITOPS.md`; local source was read because that index
  predates the candidate commits. The prior Koblitz lazy-build integration
  is `source:01d04869f74cfe4c52e86f5ebd1babba` at
  `crypto:docs/ic/perf/OPTIMIZATION_PLAN.md`. Neither pointer is used as
  evidence for a current test result.

## Derived result and limits

For a matched m=2 relation trajectory, scalar and batch have the same
table keys, first witnesses, recorded additions, table attempts and
lookups. The `bitop/v1` formula therefore assigns the same model price to
those changed operations. For m=3, a batch can perform up to seven extra
recorded additions after the first hit in its final block, so its modeled
price can rise even when physical field work falls. This is a deduction
from the source and the fixed pricing formula, conditional on the exact
relation controls passing in the planned session. It is not a measured
wall ratio, a field-operation census, or a claim about larger curves.

The unit is behaving as specified: its fixed `group_add` weight describes
an abstract affine operation. The study's **strictly lower `bitop/v1`**
primary rule is the mismatch, because the candidate changes the cost of
performing those operations. A new native field-operation accounting
scheme could resolve the model question, but it would need exact counts
for scalar and batched operations, SIMD and exceptional cases, and a
declared rule that avoids double-charging group additions. No such scheme
is implemented or validated here. No current run can establish a cost
ranking against F4/F5/F6 from this review alone.

## Corrective decision before data

[`AMENDMENT-3.md`](AMENDMENT-3.md) makes the already specified complete
cold L2 process wall comparison the primary m=3 decision and fixes its
target-cluster bootstrap and A/A separation rule before any study data.
It retains `bitop/v1` and phase primitives as model diagnostics and m=2
as a setup diagnostic because the six-arm spec has no m=2 A/A control.
The original F4/F5/F6 panel is unaffected. The next check is a new
pushed-source amendment receipt; the existing monthly EKS budget gate
must clear before the fixed 336-execution study can start. A passing wall
rule would still need full answer audit, every failed/censored row and
independent second-host replay before a performance claim.

**Graph check:** this study has no quantitative run graph because it has
no run data. The existing `docs/backend-leaderboard/LEADERBOARD.md`
and rootfinding reports cover other source revisions or target sets;
their graph sources and rendered forms were checked as out of scope for
this accounting correction. The diagram labels derived paths only.
