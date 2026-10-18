# Protocol: round 6, where solvable descent systems spend their work (E8) and the trace coset on every Koblitz curve (E9)

Frozen 2026-10-18, before E8 and E9 ran.  It follows
[round 5](../isogeny_conductor_gap_round5_20261012/README.md) and tests its
ranked leads 1–3.

**Pilot (disclosed).**
- One unaudited 12-run smoke session on the a = 1 Koblitz curve over F_2^17,
  dimension 8, 2 targets, was run while the code was written.  Its numbers
  are not results and are not reused.
- What it showed:
  - The early-stopping solver cut the F4 arm's priced solver work by 8 %
    (default base) and 20 % (coset base).
  - The coset base cost about 1.7× the default under full F4.
- These pilot numbers set the predictions below; the predictions keep the
  lead's own claim (≥ 2×) where the pilot argues against it, so the
  test can fail.

## What changed in code

- **`f4-f2-early`.** A solver equal to `f4-f2` up to the first degree step
  at which the basis's degree-1 elements leave at most `free` (default 8)
  variables unpinned.
  - It then enumerates the `2^free` points of that affine space and checks
    each against the original equations.
  - Same ideal, so the same solutions; a unit test checks this on 40
    random systems at four thresholds.
  - Its `ops` charge the extraction (one per monomial test and per word XOR
    of the affine elimination).  `f4-f2`'s `ops` do not charge its own
    extraction tests, so the comparison is conservative against the new
    solver.
- **`binary-subspace:trace_kernel=1` on a curve with Tr(a) = 1.** It now
  builds the affine base `t₀ + span(ker Tr basis)` (Tr(t₀) = 1).
  - Every point there has Tr(x) = Tr(a), so lies in 2E.
  - The base records itself as affine; only `descent-algebraic` (through
    `pq_descent_symbolic::descend_affine`) and the pair-table oracles
    accept it.
  - These configurations were refused before, so no replay changes.

## E8. Per-system F4 profile

**Question.** Round 5 lead 1 says solvable systems cost 3–10× unsolvable
ones and that deciding first would multiply every yield lever.  Where does
a solvable binary descent system spend its F4 work, and how much does
stopping at the affine part save?

**Inputs (frozen rule).**
- **Curves.** Every Koblitz curve `koblitz_instance(a, n)` with
  a ∈ {0, 1} and 17 ≤ n ≤ 29 that the repository constructs, with
  cofactor 2 or 4.
- **Bases.** Two per curve, at dimensions d = 8, 9, 10:
  - default `⟨1, z, …, z^{d−1}⟩`;
  - trace coset (`trace_coset_basis`: linear for a = 0, affine for a = 1).
- **Systems.** `S₃` descents (m = 2, 2d unknowns, n equations) for 200
  targets `[k]G`, k from a seeded generator (seed 20261018), with the same
  targets for both bases.
- **Per system, recorded:**
  - from full `f4-f2` and from `f4-f2-early` (free = 8): word XORs,
    extraction tests, steps, solving degree, whether 1 appeared, whether
    the early run stopped early, and free variables at the stop;
  - the solution sets (they must be equal);
  - the number of solutions that lift to base points summing to ±R.

**Predictions.**
- **P-E8a (replication).** On every configuration with at least 10
  solvable systems, the mean full-F4 work of a solvable system is ≥ 3× that
  of an unsolvable one.
- **P-E8b (exactness, a gate).** Zero solution-set mismatches between
  full and early runs.  One mismatch stops the lead and the solver.
- **P-E8c (the lead's claim).** On solvable systems, early stop cuts work
  (word XORs + extraction tests) by ≥ 2× in the pooled ratio of sums.
- **P-E8d (arm-level ceiling).** Over all systems of a configuration, the
  saving `1 − Σ early / Σ full` is at most the share of full-F4 work spent
  in solvable systems.  The pilot predicts 5–25 %.

**What would refute lead 1's value:** P-E8c failing with a pooled ratio
below 1.25×, or P-E8d savings below 10 % on every configuration.

## E9. The trace coset on every Koblitz curve

**Question.**
- Round 5 E7b gained 1.48× (F_2^19) and 1.94× (F_2^23) in yield from the
  trace-kernel base on a = 0 curves.  Lead 3 asks where that gain holds,
  and whether the F_2^19 shortfall is the default base already leaning
  into 2E.
- Lead 2 asks whether the affine coset extends the gain to the a = 1
  curves.

**E9a (from the E8 records).**
- **Inputs.** Per configuration of E8: the fraction of default-base points
  in 2E, and decompositions per system (lifted solutions per target) for
  both bases.
- **P-E9a.**
  - The yield ratio (coset ÷ default) is about `1 / f₂`, where `f₂` is the
    fraction of default-base points in 2E: within 20 % on every
    configuration with ≥ 30 default decompositions.
  - Equivalently, at F_2^19 the default base's `f₂` is above 0.6.
- **P-E9b.** On the a = 1 curves, the affine coset raises yield by a ratio
  whose 95 % Poisson interval contains 2 (or `1 / f₂`).
- **P-E9c.** On the a = 1 curves, the coset's F4 work per system is at
  least 1.3× the default's (the pilot's direction): the affine constant
  densifies the descent.

**E9b (whole method, `ecbench`, audited).**
- **Curves.** The a = 1 Koblitz curves E8 uses at n ≤ 23 (whole-method
  runs at n = 29 do not fit the host's uptime).
- **Arms.**
  - `rho.negation` (reference).
  - MITM (`mitm:negation_folded=1`) over the default and coset bases at
    d = 8.
  - The algebraic descent with `f4-f2` and `f4-f2-early` over both bases
    at d = 8.
- **Measurement.** 4 targets per curve, 2 rounds, alternate order,
  isolation L2 required.
- **P-E9d.**
  - MITM on the coset base has lower S than MITM on the default base on
    every a = 1 curve, across the §5a band (weights 0, 0.1, 1).
  - `f4-f2-early` has lower S than `f4-f2` on the same base at weights 0.1
    and 1.
  - All IC arms stay above rho.  The gap is reported as the distance
    remaining.

## Reporting

- Each run leaves a report, an SVG graph and a PDF (`research-visuals`).
- Ratios are pooled as ratios of sums, with 95 % intervals:
  - Poisson for counts;
  - a bootstrap over systems (2,000 resamples, seed 1) for work.
- Rows are ranked only where the order holds across the band.
