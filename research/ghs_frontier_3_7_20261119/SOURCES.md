# Sources checked

## Knowledge-base receipt

The mandatory repository wrapper was invoked first:

```text
scripts/research-kb status
research-kb: no executable found; set RESEARCH_KB_BIN or put research-kb on PATH
```

No research-KB MCP tool is loaded in this runtime. The searches for GHS
IC/rho, parallel distinguished-point rho, F4/F5/F6/SAT/FES/Gray-code/root
engines, and factor-base frontier evidence therefore could not execute. No
novelty, absence, or “not tried” inference is drawn from that failure.

## Primary literature receipt retained by the repository

The primary-source table in
[`../ecbench_e2e_frontier_20261031/LITERATURE.md`](../ecbench_e2e_frontier_20261031/LITERATURE.md)
was reviewed before choosing the implementation or measurement design:

- Shoup, EUROCRYPT 1997, DOI `10.1007/3-540-69053-0_18`: generic-group
  square-root boundary; a table or root-finding implementation does not by
  itself establish a sub-square-root algebraic attack.
- Gallant–Lambert–Vanstone, Math. Comp. 2000, DOI
  `10.1090/S0025-5718-99-01119-9`: usable automorphisms belong on the matched
  rho arm too.
- van Oorschot–Wiener, J. Cryptology 1999, DOI `10.1007/PL00003816`:
  parallel collision search uses one transition function, distinguished-point
  accounting, and explicit table/restart/worker policy.
- Faugère F4 (1999), DOI `10.1016/S0022-4049(99)00005-5`, and F5 (2002), DOI
  `10.1145/780506.780516`: symbolic preprocessing, matrix construction, and
  signature work must remain inside the scoped cost.
- Semaev, IACR ePrint 2004/031, and Gaudry, JSC 2009, DOI
  `10.1016/j.jsc.2008.08.005`: factor-base constraints, relation probability,
  representation, linear algebra, and target recovery belong in end-to-end
  accounting.
- Faugère–Perret–Petit–Renault, EUROCRYPT 2012, DOI
  `10.1007/978-3-642-29011-4_4`: Weil descent, Boolean-system construction,
  solving degree, and algebraic solving are distinct obligations.

The GHS structural scope and Hess/MMT correction were also checked in
[`../../docs/GHS_TRANSPORT.md`](../../docs/GHS_TRANSPORT.md) and
[`../../docs/audits/GHS_MAGIC_NUMBER_MMT_CORRECTION_20261013.md`](../../docs/audits/GHS_MAGIC_NUMBER_MMT_CORRECTION_20261013.md).
They prevent transferring the restricted-family result to a curve without an
explicit subgroup-preserving, costed map.

## Repository evidence that controls this round

- [`../quartic_ghs_r0_r99_20261119/REPORT.md`](../quartic_ghs_r0_r99_20261119/REPORT.md):
  R00–R99 dispositions, production R17/R36/R64 mechanisms, source-bound q=223
  correctness capsule, and unmeasured obligations.
- [`../quartic_ghs_r0_r99_20261119/ranges/R67_R83_MEASUREMENT_PLAN.md`](../quartic_ghs_r0_r99_20261119/ranges/R67_R83_MEASUREMENT_PLAN.md):
  D00–D10 correctness gates and the q=4051/16363/65479 P1A/P1B/P2/P4 plan.
- [`../quartic_ghs_strong_rho_parallel_20261118/IMPLEMENTATION_SPEC.md`](../quartic_ghs_strong_rho_parallel_20261118/IMPLEMENTATION_SPEC.md):
  composition, M1–M4, live solver audit, and publication dependencies.
- [`../ecbench_e2e_frontier_20261031/establishment/README.md`](../ecbench_e2e_frontier_20261031/establishment/README.md):
  executable status of the standing frontier and its open obligations.
- [`../ecbench_e2e_frontier_20261031/IMPLEMENTATIONS.md`](../ecbench_e2e_frontier_20261031/IMPLEMENTATIONS.md):
  historical 74-method/11-solver roster. It is a comparison point, not a
  substitute for a live inventory.
- [`../../docs/bounds/FRONTIER.md`](../../docs/bounds/FRONTIER.md) and
  [`../../docs/ic/LEADERBOARD.md`](../../docs/ic/LEADERBOARD.md): canonical
  admitted views, unchanged until generated evidence satisfies every gate.
- [`../../problems/ghs-strong-rho-measurement-admission.md`](../../problems/ghs-strong-rho-measurement-admission.md):
  checkpoint, budget, pool, identity, and image blockers.

## Effect on the plan

The sources require one shared transition for parallel rho, strongest-symmetry
matched references, cold one-target cost as the primary interval, separate
solver-local and end-to-end conclusions, complete pricing-band accounting,
and restricted-family wording for every GHS win. WD-SAT remains excluded from
promotion until its retained false-UNSAT path is repaired and replayed.

