# Literature and prior-work basis

This is the literature-first record for the registry-wide optimization loop.
It distinguishes methods that choose candidates from methods that establish a
performance claim.

## External primary sources

1. James S. Bergstra, Rémi Bardenet, Yoshua Bengio and Balázs Kégl,
   “Algorithms for Hyper-Parameter Optimization,” NeurIPS 2011,
   <https://papers.nips.cc/paper/4443-algorithms-for-hyper-parameter-optimization>.
   This is the basis for conditional TPE candidate generation. It informs the
   development search only; it provides no protection against adaptive
   overfitting to the benchmark panel.
2. Lisha Li, Kevin Jamieson, Giulia DeSalvo, Afshin Rostamizadeh and Ameet
   Talwalkar, “Hyperband: A Novel Bandit-Based Approach to Hyperparameter
   Optimization,” JMLR 18(185), 2018,
   <https://jmlr.org/papers/v18/16-558.html>. This supports explicit resource
   rungs and early stopping. The repository uses deterministic retained rung
   outcomes; it does not reinterpret a pruned candidate as a measured loss at
   the target scale.
3. Jason Ansel et al., “OpenTuner: An Extensible Framework for Program
   Autotuning,” PACT 2014, <https://doi.org/10.1145/2628071.2628092>. This
   motivates separating the search manipulator, objective and measurement
   interface, and keeping failed configurations in the search history.
4. Todd Mytkowicz, Amer Diwan, Matthias Hauswirth and Peter F. Sweeney,
   “Producing Wrong Data Without Doing Anything Obviously Wrong!,” ASPLOS
   2009, <https://doi.org/10.1145/1508244.1508275>. This motivates matched
   binaries, order balancing, output identity and refusing environment/layout
   accidents as algorithmic gains.
5. Torsten Hoefler and Roberto Belli, “Scientific Benchmarking of Parallel
   Computing Systems,” SC 2015,
   <https://doi.org/10.1145/2807591.2807644>. This informs repeated paired
   measurements, uncertainty reporting and avoiding fastest-single-run
   selection.

The algebraic and ECDLP primary sources for the underlying algorithms are
already reviewed in the active frontier report:
[`../ecbench_e2e_frontier_20261031/LITERATURE.md`](../ecbench_e2e_frontier_20261031/LITERATURE.md).
That review covers Semaev systems, F4/F5, binary-field index calculus, generic
lower bounds, automorphism-aware rho, parallel collision search and Gaudry-style
index calculus. This loop changes candidate selection and evidence discipline,
not those mathematical claims.

## Repository prior work read before implementation

- [`docs/ic/SWEEPER.md`](../../docs/ic/SWEEPER.md) and
  `src/bin/ic/sweep/`: the existing native conditional TPE search, scale rungs,
  median pruning, append-only store and unit-mismatch rejection.
- [`docs/ic/perf/OPTIMIZATION_PLAN.md`](../../docs/ic/perf/OPTIMIZATION_PLAN.md):
  the established measure/change-one-thing/remeasure/keep-or-revert loop and
  the retained negative results.
- [`docs/ic/factor-base-sweep/README.md`](../../docs/ic/factor-base-sweep/README.md):
  independent training/holdout targets and native correctness controls for
  factor-base candidates.
- [`../f4_f5_f6_cross_family_20261011/RESULT.md`](../f4_f5_f6_cross_family_20261011/RESULT.md):
  historical current-question evidence for matrix F4/F5, inherited F4 and
  F6-IC. It shows why solver-local work and end-to-end ECDLP cost must remain
  separate.
- [`../ecbench_rho_optimized_20261014/RESULT.json`](../ecbench_rho_optimized_20261014/RESULT.json):
  the prior strong-rho instruction improvement and the still-open L2 wall
  confirmation.
- [`../ecbench_e2e_frontier_20261031/REPORT.md`](../ecbench_e2e_frontier_20261031/REPORT.md):
  the established frontier statement and the exact limitation that the fresh
  all-implementation EKS panel has not run.

The sibling lead-atlas brief returned an empty index, while its targeted match
command failed because its local `orchestration` module was unavailable. That
tooling failure is recorded as unknown prior-atlas match status, not as a
novelty claim.

## Design consequences

- TPE and successive halving are proposal mechanisms, never confirmation.
- Search histories may guide candidate selection, but fresh disjoint holdouts
  and independent replay determine promotion.
- Solver-native units are not made commensurate by fiat.
- Every fixed-parameter algorithm still receives an implementation A/B loop.
- A/A intervals, balanced order and exact output/count identity are mandatory.
- Caps, seeds, advice and target multiplicity cannot be optimized as shortcuts.
- Nulls, regressions, crashes, exhaustion and timeouts remain first-class rows.
