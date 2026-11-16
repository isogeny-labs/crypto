# PKM: measurement goals, improvement targets and novelty audit

2026-10-29. Proposed work program, not a frozen experiment or a measured result.
Execution status: blocked. Tracking card:
`backlog/XP-20261029-pkm-verified-oracle-program/`.

The first decision is whether symmetric PKM is a useful **complete decomposition
oracle**. The second is whether improving that oracle lowers the cost of a cold,
single-target discrete logarithm. A fast refutation or a low degree alone decides
neither question. Start with m=4; use m=2,3 for controls and mechanism studies.

## What is different, and what is already known

| Ingredient | Prior work | Status of this follow-up |
| --- | --- | --- |
| Compositions of low-degree maps define a prime-field factor base | Petit–Kosters–Messeng, PKC 2016, sections 3–5 | Published framework; not our invention. Its complexity was explicitly left open. |
| Ignore permutations of summands | PKM Algorithm1, step 4c, explicitly says to work modulo permutations | Known symmetry. An unordered representation alone is not a new exponent improvement. |
| Use elementary symmetric coefficients and a root polynomial, with repeated squaring modulo that polynomial | crypto PR2632 and PR2650, October26 | Already implemented in this lab. The present PR inherits this construction. Global priority for this particular presentation has not been established. |
| Symmetric m=4 system and S5 construction | PR2650, source adde64fdc22d51dc180fe0b9f30a2d28adf992fe | Already tested as an F4-stage instrument. Construction is outside the reported solver timer. |
| Native decoding, point lifting, signs and verification against a supplied actual finite target | PR2868, published source 8baeee2d446968936882adc503570fb957bb1429 | New integration in this reviewed symmetric-driver path. Existing root-finding/F4 machinery is reused. This is an implementation and correctness contribution. |
| Lean membership, recovery and coverage lemmas | PR2868 | Checked structural implications. They do not prove fast solving or a new ECDLP complexity bound. |
| Full oracle cost on actual targets, followed by cold relation/LA/target recovery | Not supplied by the reviewed symmetric stage study or PR2868 | The important missing experiment. A favorable result could justify further engineering or a structural hypothesis; it would not establish an asymptotic theorem. |

The symmetric tower still has m*t coordinate unknowns in this implementation.
The plain S5 arm also has m*t; the plain S3 chain has m*t+(m-2). Three arms are
needed to distinguish eliminating partial-sum variables from symmetrizing the
summands. For fixed m=4, removing up to 24 orderings is a constant combinatorial
factor, not by itself a different exponent. It is not a 24-fold upper bound on
the possible change in Gröbner runtime.

The old m=4 symmetric input already has degree 8. At N=12 and16 its reported
maximum degree was8, while its reported field work rose from 4.01e5 to 1.08e10.
The raw plain N=20 study reached productive degree 8 steps before a memory cap;
the symmetric N=20 attempt aborted under 6 GB. These are historical stage reports,
not independently replayed results of this follow-up. N=20 is a completion and
coverage goal, not an untouched cell or proof of boundedness.

## Proposed goals and completion criteria

### G1 — Establish a faithful m=4 oracle

On independently enumerable small curves, compare the complete sets of valid
distinct-abscissa decompositions for every supported finite target against a
native exhaustive control. Include planted positives, independently classified
natural positives, certified misses, off-curve refusals and capped/unknown cases.
Keep the target scalar outside the recovery interface. Verify the original
curve equation, factor-base membership and signed point sum for every witness.

Done: no lost valid decomposition or invalid witness on the frozen tiny corpus;
all representations agree on the same allowed output domain. Repeated abscissae
and the identity target remain explicit exclusions until implemented. No timeout
is a refutation. The existing F17 tests are the starting control, not this goal's
entire corpus.

### G2 — Measure the complete cost and valid yield

Instrument these phases separately: curve/base setup; target-specific polynomial
construction; F4; rational-root extraction; unrestricted branches; point lifts;
sign handling; verification; memory; and every failed/capped call. Record native
counts and their coverage before converting to a common unit. Preserve residual
unpriced work and the required 0/0.1/1 pricing band; clocks need host admission.

Report setup plus all calls on a fixed public target stream, number of verified
decompositions, misses and unknowns, and cost per verified decomposition when
that denominator is positive. Later report cost per rank-increasing relation.
Zero successes is a zero-success outcome, not a zero-cost oracle. A reusable
curve/base cache is charged once in the cold run and subsequent queries are
charged individually. Declare the reuse count; do not silently assume free
precomputation or unlimited amortization.

Done: every material phase is priced or visibly unpriced, all witnesses verify,
and the comparison does not rely on an incomplete ledger. This is the first
measurement priority, before buying larger machines or promising a speedup.

### G3 — Identify what actually helps

Compare plain S3 chain, plain S5 and symmetric S5 on identical E,V,Q, the same
distinct-abscissa domain and equal process/memory limits. Separately compare a
qualified, pinned external solver such as msolve on equal polynomial systems,
with output verification. A system-form comparison and a solver comparison are
different comparisons. Include deterministic A/A, successful point-extraction
controls and shape-matched nulls. Do not pool planted and unconditional targets.

Done: a paired result identifies whether the gain comes from presentation,
construction, solving or extraction; losses and censored pairs are retained.
Measure maximum/input/last-productive degree, matrix width/rows and peak memory,
but rank candidates by the complete measured cost and verified yield.

### G4 — Improve the dominant measured component

Prioritized candidates, each a hypothesis requiring its own ablation:

1. **Reuse curve/base algebra.** Cache validated target-independent tower and
   conversion data. Test a precomputed target-parameter representation of S5
   instead of rebuilding its 9^4 evaluation/interpolation grid for each target.
   Derive the dependency and cache key; charge generation, storage and cold
   setup. Verify byte-canonical systems or algebraic equivalence independently.
2. **Extract algebraically.** Reuse triangular/quotient information and native
   factorization; qualify FGLM or another exact extractor where appropriate.
   Make unrestricted-field scans explicit. An algebraic-only arm must return
   capped/unsupported rather than hide a grid or field scan.
3. **Reduce the active matrices.** Target the largest measured matrix block,
   live basis and allocation copies. Prototype a symmetric/tower-aware layout
   or ordering only after its equivalence and actual cost are checked.

Proposed engineering objective: at least 20% lower complete cold oracle-batch
cost than the best frozen equal-output native baseline on the held-out panel,
with a paired 95% interval excluding no improvement under every required pricing
weight, with unchanged or better completion and verified coverage. Fewer
verified outputs or more unknowns do not earn a cost improvement. A memory
objective is at least 2x lower peak memory with no more than 10%
extra complete cost, and completion of an additional predeclared cell under the
same cap. Precisely: cost point ratio <=0.8 and paired95% upper ratio <1;
memory ratio <=0.5 with complete-cost ratio <=1.10. These are proposed targets to freeze before data, not measured gains
or new repository admission rules. If a candidate misses, preserve the outcome;
do not tune on the held-out panel. An external loss remains an external loss.

Do not repeat the old signature/F5 experiment as a new idea without a new
mechanistic reason: the earlier tower study already reports higher degree and
memory despite fewer zero reductions.

### G5 — Test scaling and the m=4 reach boundary

Start with m=4, t=2,3,4 (N=8,12,16) where a valid factor base/control exists;
attempt t=5 (N=20) only after G1–G3 and sizing within an approved resource cap.
Use multiple primes and independently generated target streams, followed by
held-out curves/seeds. Existing candidate primes are 786433, 2013265921 and
3221225473; they are workload candidates, not three independent structural
families or a frozen curve/subgroup roster. The present driver supports p<2^32.

Separate the fixed-prime depth ladder (presentation/resource growth) from a
field-size ladder (possible cost scaling). Increasing t at fixed p changes base
size and relation probability. Match or explicitly stratify target consistency,
base size, actual liftable points, supported-domain coverage and valid yield.
Do not fit an ECDLP exponent to one fixed-p depth ladder. A constant observed
degree is not an asymptotic bound, especially when degree 8 is an input floor.

Done: at least three predeclared completed depth points with a held-out check,
or an explicit censored reach boundary. If N=20 exceeds the cap, report the
boundary and change representation/memory strategy before requesting a larger
machine. The old 256–512GB plain-arm projections exceed the default lab pool's
128GiB capacity; no quota/pool expansion is implied. m=3,N=21 is secondary
mechanism work, not the main exponent target.

### G6 — Connect to a cold ECDLP and pursue the actual proof gap

On certified public prime-order instances, collect independently useful rows,
solve the factor-base logs, recover independently generated targets, and verify
the final scalar equation. Charge setup, every query/failure, scalar-field LA,
target recovery and verification. Compare to the frozen best IC baseline and
the strongest matched rho/BSGS on the same targets. Primary scope: one cold
target; reusable multi-target results remain separately labelled.

The existing rough model with quadratic-scale sparse LA gives exponent
2(1+gamma)/(m+1) if per-query cost scales as r^gamma and setup/yield/rank
assumptions hold. At m=4, this exponent is below 1/2 exactly when gamma<1/4.
`lean/ThreeLeads/PKMGoalModel.lean` checks only this arithmetic equivalence.
Its parameter is rational; no actual oracle exponent is supplied to the theorem.
It does not validate that model, estimate gamma, bound Rust costs or establish
a sub-rho algorithm. Measure the actual LA/rank/yield terms before applying it.

The substantive Lean goal is to define the precise symmetric ideal and
filtration, connect its solutions to supported curve decompositions, and prove
an extraction/solving-cost bound for a quantified growing family. Small-case
correctness and the existing membership lemma are dependencies. A plateau,
constant first fall degree or arithmetic model certificate is not that theorem.

Done for engineering: a complete held-out cold IC cost improvement with required
validation and external/reference comparisons. Done for a complexity claim:
an independently reviewable mathematical argument with the missing hypotheses
discharged and faithful formalization. These are different milestones.

## Order, stop rules and execution status

Run G1–G3 first, then one justified G4 change, then G5. Attempt G6 only once
verified useful relations and full accounting justify it. Any correctness or
independent-verifier disagreement stops the affected comparison. Freeze inputs,
target-generation law, sample sizes, budgets, controls, source/binary pins and
decision rules in experiment.protocol/v1 before measured data. Use admitted L2
timing with warmup/five rounds when wall time is an outcome. Include censored
arms; neither a baseline timeout nor a larger-host result creates a finite ratio.

If G4 misses its frozen target, keep the old baseline and reject that candidate.
Repeated caps before the next depth produce a resource-boundary result and a
specific representation/memory task, not automatic hardware escalation. A
shape-matched null performing equally well removes evidence for a Semaev-specific
advantage at that scope. Unfavorable finite scaling withdraws support for the
tested implementation/model extrapolation; it is not a universal PKM no-go.

No experiment is launched or funded by this proposal. Missing pieces include
the full-cost native panel/replay harness, supported resumability, a qualified
external arm, checkpoint/artifact access and an authenticated supported runner.
The existing checkpoint gate and execution requirements remain in force.

## Sources and audit limits

* [PKM primary text](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
  sections 1,3.1–3.2,4–5, read for this audit: low-degree-map framework, explicit
  permutation symmetry, preliminary experiments and unresolved complexity.
* [PR2632](https://github.com/isogeny-labs/crypto/pull/2632) and
  [PR2650](https://github.com/isogeny-labs/crypto/pull/2650), particularly pinned
  `research/pkm_symmetric_lead1_20261026/README.md` and its constructor/measure
  call sites; stage reports, not new admitted results here.
* `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md`, sections 3.3–3.7
  and rounds1–8; prior memory/degree/F5/early-exit results and cost-model scope.
* `research/ic_next10_20261014/03-pkm-degree/README.md`; N=20 raw plain degree 8
  evidence and the XL/F4 distinction. Existing backlog card
  XP-20261015-pkm-m3-n21-m4-n20 already records that outcome.
* `research/prime_fall_degree_20261027/PREREGISTRATION.md`; separate closure,
  first/last/solving degree and null-control definitions.
* [PR2868](https://github.com/isogeny-labs/crypto/pull/2868), source 8baeee2d:
  current native extraction, finite-domain restrictions and partial counters.

The APS primary PDF at ePrint 2017/609 returned HTTP403 during this audit; the
full Yokoyama text remains unavailable from the earlier review. The audit does
not establish global novelty or a universal negative theorem. No canonical
performance/curve graph changes: this proposal adds no measured value or curve.

## Planning validation and reproduction

The new blocked card passes the native `backlog validate` check. The new
arithmetic module builds, passes the repository axiom gate and passes the
official module kernel replay. The project audit counts10 declarations: the
original eight named structural theorems, their generated equation lemma and
the one new model-arithmetic lemma, with zero trusted exceptions. See
`planning-validation/` for source/report hashes and receipts. The three-page
`pkm-goals.pdf` was rendered and every page inspected.

```sh
cargo build --release --locked --bin backlog
target/release/backlog validate XP-20261029-pkm-verified-oracle-program
cd research/prime_ecdlp_three_leads_20261029/lean
lake build +ThreeLeads.PKMGoalModel
lake env leanchecker ThreeLeads.PKMGoalModel
cd ../../..
tools/lean/proof-gate.sh research/prime_ecdlp_three_leads_20261029/lean
```

No Rust research implementation changed in this planning update. The prior
eight new-example test passes/strict lint and the explicitly incomplete broad
library check remain historical validation of their pinned source. No new
scientific run, frozen protocol or admitted performance result is supplied.
