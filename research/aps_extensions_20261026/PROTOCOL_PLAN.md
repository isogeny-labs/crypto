# Controlled panel design — not frozen, not executed

No performance experiment has run for this study. Fixed software correctness
tests are development checks, not an unbiased performance sample. This plan
preserves the larger requested validation rather than substituting tiny tests.
Before data collection, turn each panel below into a schema-valid
`experiment.protocol/v1`, freeze its analysis and commit/hash, and pass the
repository's S3 checkpoint gate. Do not reuse development fixtures or seeds
as held-out confirmation data.

Deferred execution is recorded in the repository's blocked
[backlog card](../../backlog/XP-20261026-aps-complete-cost-panel/README.md),
with the [implementation/accounting gap](../../problems/aps-single-relation-complete-cost.md).

## Claims and panels

1. **Presentation at fixed base.** Compare partitioned S3, cubic-divisor S3,
   and a complete covering family on the same labeled base. Report distinct
   verified relation sets and useful-relation probability. Different selection
   domains are deliberate; do not compare only solve time and ignore yield.
   For m=4 compare expanded, split and hybrid on the exact same disjoint
   lists. Treat two-pair blocks as a separate yield/cost comparison.
2. **Label construction.** Cross random versus fixed coefficients with the
   baseline encoding, keeping curve/target population, total base size,
   sampling distribution and retry caps fixed. Labels change the generated
   base, so these are distribution-matched randomized arms, not identical-base
   pairs. Do not relabel fixed points using a planted target logarithm.
3. **Sparse orbit structure.** On eligible j=0 curves compare whole-orbit
   bases with size-matched random bases and the established GLV folding
   implementation. Charge overlapping/short-orbit rejection and all `A=B=0`
   relations. Include an ineligible-curve control that must fail closed.
4. **Structured-map follow-up.** This requires new implementation and a
   separate preregistration. Do not treat the current orbit fixture as a test
   of arbitrary multiplicative cosets, trace constraints or composed maps.

## Controls and confounders to freeze

- A/A: two blinded copies of the baseline with identical inputs and binary,
  randomized/interleaved execution order. They must give equal counted
  outputs; any admitted clock needs the stated isolation tier and noise check.
- Positive: public labeled points with a known informative zero sum, passed
  through the same equation builder, solver, sign search and final verification.
  Also include an intrinsic automorphism tautology that must be rejected.
- Negative: an independently checked finite base with no allowed relation;
  corrupt labels, overlap, wrong subgroup order and exhausted budgets must
  not produce a verified result.
- Freeze field/curve identities, prime subgroup, generators, target distribution,
  m, all base sizes, budgets, encoding, monomial ordering, branch rule, compiler,
  backend options, threads, seed derivation, arm order and cold/warm policy.
- Use held-out curves and seeds derived from the frozen protocol hash. Choose
  sample size, stopping rule, effect size, uncertainty method and multiplicity
  correction before observing the affected data; do not pick them from these
  development tests. The runtime/power pilot is itself a separately recorded
  experiment and needs its own checkpoint and protocol.

## Required accounting and records

The primary cost is complete cold work per verified informative relation and
per solved public target, including failed bases and retries. Price label/base
generation, vanishing and divisor polynomial construction, F4, substitution/root
extraction, point-sign search, duplicate handling and final verification. Keep
memory, solving-degree metadata, matrix dimensions and yield alongside it.
F4 multiplications alone are a stage metric. Prime-fast setup arithmetic and
the new expression layer are not currently instrumented for a complete count.

Use the native ecbench runner, with same-process calibration and matched
strongest rho (including available automorphism folding). Compare the polynomial
computation with Magma or msolve on the same inputs and verify equal outputs.
If the external tool is unavailable, record the missing comparison and do not
claim fastest. Keep counts primary; publish wall time only at the isolation
level the measurement rules admit.

Every planned row gets an outcome: success, no useful relation, sampling
failure, timeout, OOM, invalid output or infrastructure failure. Retain seeds,
caps, base labels, roots/relations and full stderr/raw records. A bounded runner
error is not a reason to rerun until it succeeds. Publish one
`experiment.export/v1` with `flow` counts and raw rows, an independent replay,
an independent recomputation of headline metrics, and visual/PDF run reports.
No independent replay certificate exists for this study yet.

Evaluate improvement against the current verified IC baseline. Not beating
rho does not erase an IC improvement; a claim to beat rho needs complete
matched evidence at the 0, 0.1 and 1 GAE bands. Reject a speedup claim if it
omits setup/retries, changes the target workload, violates its interval rule,
or relies on inadmissible clocks. A null or negative result is retained.

## Launch blockers and continuation

1. APS-V2: add complete counters, ecbench/export integration and transcript replay.
2. Freeze a schema-valid protocol and analysis with an explicit population,
   sample size, independent seeds, runtime budget and comparison thresholds.
3. Configure the checkpoint upload credential through the environment's secret
   mechanism, complete `.resume/RESUME.md`, push a checkpoint and pass
   `scripts/checkpoint.sh gate`. No credential belongs in the PR or chat.
4. Execute, export, independently validate, and update the tracker and reports.

There is no automatic launch or background benchmark scheduled by this PR.
