# Protocol: optimize the complete ECBench registry

Status: the methodology and admission rules below were frozen before new
measurements. The 111-spec all-method development roster now passes `ecbench
optimize audit --require-all` with 78/78 methods and 24/24 nested algorithms.
No paid job may launch before the independent checkpoint gate also passes.

## 1. Boundary and question

The boundary is the live `methods::registry()` plus the native nested solver
registry and the binary-linked PDP3 and prime PKM tower engines. At the current
source state, that is 78 top-level methods, 11 system solvers, nine PDP3
engines, and four PKM tower engines.

“Complete” means complete over that executable ECBench boundary. Standalone
research binaries, examples, and library-only experimental options are not
silently counted. They require an explicit registry or nested-inventory entry,
matched reference arm, complete pricing, and replay path before this protocol
can claim coverage. In particular, the newly merged N83 chained-S3
meet-in-the-middle lookup currently has separate decomposition-stage evidence
but no ECBench arm; it remains a named integration and end-to-end measurement
gap.

The `ic bench` chained-x oracle is a different implementation. It now has a
matched one-curve L0 diagnostic, but no ECBench spec, session receipt, replay
contract, scaling ladder, or confirmatory holdout. It therefore remains outside
the complete ECBench roster even though its IC-framework rows are end to end.

The question for each compatible method/domain cell is:

> Can a prospectively selected configuration or implementation reduce complete
> end-to-end work relative to its frozen same-method incumbent, without changing
> the answer, counted-work fingerprint, advice model, censoring envelope or
> accounting scope; and, separately, does it move the matched ECDLP frontier
> against the strongest applicable rho/reference arm?

The same-method engineering question and the frontier question are distinct.
A faster solver-local phase is retained even when the whole ECDLP does not
improve. A different advice or amortization model starts a different scope.

## 2. Frozen hypotheses and falsification

For each lane, the null is that no candidate clears both of these gates:

1. deterministic complete work is lower than the incumbent outside the
   incumbent A/A interval; and
2. admitted L2 wall time is lower on the fresh holdout outside the A/A interval.

For an ECDLP-frontier claim, add a third gate: the candidate's complete
symmetrically priced end-to-end cost must beat the strongest applicable rho arm
at every required pricing-band weight. A solver-native counter alone cannot
satisfy this gate.

A candidate is falsified by a wrong scalar or relation, output/fingerprint
drift outside its declared algorithmic change, a missing phase, incomparable
native units, a lower-bound-only cost at a claimed winning weight, excess
memory, timeout/exhaustion under the frozen envelope, holdout seed reuse, or a
ratio that does not clear the A/A interval.

## 3. Variables

### Independent variables

- method configuration parameters classified `knob` by the native inventory;
- one source-level implementation change at a time;
- curve/domain cell and size rung;
- explicitly separated accounting scope (`standard_band`, `full_price`,
  `word_ram`, Jacobian-native or external-native).

### Controlled variables

- exact source, lockfile, compiler, binary and container digest;
- curve identity, public target list and algorithm seed law;
- timeout, trial/state/memory caps and every other resource envelope;
- base/rank/table nuisance seeds;
- target multiplicity and advice model;
- thread count, CPU class, isolation policy and measurement order;
- pricing calibration and unpriced-counter treatment.

The auditor refuses same-method candidates that change a non-knob relative to a
baseline in the same spec. A study may deliberately change advice or target
multiplicity only by opening a separately named scope with its own reference.

### Outcomes

Primary development outcome: deterministic complete work in the method's
declared unit, including the `w = 0, 0.1, 1` pricing band where applicable.

Primary confirmatory outcomes:

- complete priced end-to-end work and normalized `S`;
- paired L2 wall ratio with its A/A interval;
- peak resident memory and retained timeout/exhaustion rate;
- correctness, replay and output/count fingerprint equality.

Solver-local operations, conflict counts, solving degree and phase wall times
are diagnostics. They explain a result but do not replace end-to-end cost.

## 4. Two non-overlapping datasets

### Development/search

- Deterministic target and measurement seeds reserved for development only.
- At least four balanced measured rounds after at least one warm-up for clocked
  micro-comparisons; count-only searches may prune on deterministic work.
- TPE for conditional high-dimensional spaces; prospectively fixed grids and
  successive halving for small spaces.
- Rungs increase curve size, target count or solver budget only according to
  the frozen lane definition. A resource cap is not itself optimized.
- Failures, invalid configurations, exhaustion, crashes, pruning and timeouts
  are appended to the study store and never converted into wins.
- Search results select at most three Pareto candidates per compatible lane.

Development observations are never copied into a performance claim.

### Confirmatory holdout

- Fresh, disjoint public-target and measurement seeds.
- At least eight independent public targets per curve cell, split into queue-
  safe shards when needed.
- One warm-up and at least six balanced measured rounds.
- Candidate, frozen same-method incumbent, strongest applicable reference and
  an exact reference/incumbent A/A duplicate.
- EKS `cpu-l2-spot` for clock evidence; fixed one-thread and explicit multi-
  thread panels when parallel behavior is claimed.
- No measurement retry. Only a classified infrastructure failure may receive a
  fresh attempt, and both attempts remain visible.

## 5. Search and promotion loop

1. **Inventory.** Generate `ecbench optimize inventory --json`; exact registry
   equality is mandatory.
2. **Baseline.** Freeze current source, binary, host class, spec and outputs.
3. **A/A.** Establish the count identity and L2 noise interval.
4. **Profile.** Use complete instruction/phase profiles to locate cost; profiles
   are explanatory, not speed evidence.
5. **Change one lever.** Configuration search may vary declared knobs;
   implementation work changes one source mechanism.
6. **Development evaluation.** Interleave incumbent/candidate, retain all
   outcomes, and prune only on the frozen rule.
7. **Candidate freeze.** Seal source/configuration, binary digest and holdout
   protocol before revealing holdout targets.
8. **Holdout.** Run candidate/incumbent/reference/A-A on EKS L2.
9. **Audit and replay.** Verify every record, replay deterministic executions
   and obtain the independent reading required by the validation gate.
10. **Promote or retain as negative.** Update frontier/leaderboards only for an
    admitted gain. Commit valid regressions/nulls to the study and backend
    browser without changing a winner row.
11. **Repeat.** The next iteration begins from the admitted incumbent; rejected
    candidates do not silently become the new baseline.

## 6. Solver and root-finding lanes

The 11 `SystemSolver` backends are separate because their native counters have
different meanings and completeness. They run on matched exact systems and
also inside the enclosing pipeline. F4, F5, SAT, FES/Gray-code and exhaustive
counts are never pooled as one unit merely because they solve the same system.

The PDP3 inventory separately requires:

- inherited F4;
- F6-IC;
- F6-IC with the exact pair-sum index;
- matrix F4;
- matrix F5;
- full root finding;
- fixed one-third window root finding;
- negation-folded root finding;
- windowed negation-folded root finding.

Root-finding table methods are the generic/reference track, not an algebraic
sub-square-root claim. Solver-local gains must survive base construction,
relation yield, linear algebra, recovery and final verification.

The prime PKM tower inventory separately requires F4, signature F5, F6
geometric closure, and direct geometric enumeration. Its `F_p` multiply-add
counter is not assigned a GAE price by default; comparisons retain it as
unpriced work and cannot promote a lower-bound-only winner.

## 7. Rho optimization lane

Rho is not a token baseline. The strong applicable rho arm receives its own
loop:

- lanes, distinguished-point bits and jump count where they are true knobs;
- jump-table/start/final-check setup and escape behavior;
- batch inversion width, layout, canonicalization and table probes;
- one-thread instruction counts, cache/memory behavior and L2 wall time;
- identical trajectory/output controls against legacy and current accounting;
- external fastest-tool comparison where the repository can export the exact
  matched workload.

Step caps remain fixed censoring envelopes. A rho candidate cannot win by
timing out hard targets or by omitting canonicalization, escape, final checks or
table costs.

## 8. Admission and publication

The all-method development roster is admitted only when the native auditor says
all 78 methods and all 24 nested paths are present and every structural error
list is empty. A successful run then additionally requires queue export checks,
native L2 admission where clocks are used, record audit, exact payload hashes
and independent replay.

No leaderboard or `benchmarks/` performance row is changed by this protocol
alone. A later result PR must include its visual report, raw/store manifest,
validation verdict and all synchronized dashboard updates.
