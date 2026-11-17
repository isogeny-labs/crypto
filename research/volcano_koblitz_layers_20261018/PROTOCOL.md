# Protocol: ECDLP cost across the layers of a Koblitz curve's ℓ-volcano

**Frozen 2026-10-18, before any session below was run.** Native tools only:
`crypto volcano` builds the curves, `ecbench` measures them (AGENTS.md §12,
"Implementation language: no Python").

## Question

The isogeny class of the Koblitz curve `K_0 : y² + xy = x³ + 1` over `F_(2^n)`
is a union of ℓ-volcanoes. `K_0` itself sits on the crater of each (End = O_K,
conductor 1); for a prime ℓ dividing the conductor `f` of `Z[π]`, the curves at
level 1 have End of conductor ℓ. All of them have the same group order `N`.

Does the layer change the cost of the ECDLP in ecbench's counted unit?

## Curves

For each `n` the crater is ecbench's `koblitz` constructor (`a = 0`). The
level-1 neighbours are `binary_explicit` curves written by
`crypto volcano ecbench-spec`: `j` from the roots of `Φ_ℓ(X, j₀) mod 2`
(`j₀ = −3375`, `D = −7`), the twist of order `N` by `[N]P = O`, and a generator
of the prime-order subgroup `r = N/cofactor`. The generator command, the
j-invariants, twists and generators are in `specs/n*/curves.json`.

| n | ℓ | h(O_ℓ) = neighbours at level 1 | r | cofactor | neighbours measured | targets per curve |
|--:|--:|--:|--:|--:|--:|--:|
| 19 | 457 | 456 | 130873 | 4 | 12 | 8 |
| 23 | 967 | 968 | 2095853 | 4 | 12 | 8 |
| 37 | 73 | 74 | 230603167 | 596 | 12 | 8 |
| 41 | 409 | 410 | 549756390943 | 4 | 12 | 8 |
| 61 | 1951 | 1952 | 162888033982417 | 14156 | 6 | 4 |

The crater runs `4 ×` as many targets per curve, since it is one curve.

## Methods (ecbench ids)

* `rho.negation`, the generic reference (`A = 2`), on crater and neighbours.
* `rho.plain`, the unmodified baseline, on both.
* `rho.signed_frobenius_strong`, the Koblitz reference (`A = 2n`), on the crater
  only. It needs the Frobenius endomorphism, which a level-1 curve does not have.
* `rho.negation` repeated as the A/A control.

Counts only (`isolation_required: L0`); no wall-time claim.

## Predictions

* **H1.** The generic solvers cost the same on the crater and on level 1:
  the ratio of mean counted operations of `rho.negation` on neighbours to the
  same on the crater lies inside the A/A interval of the control arm.
* **H2.** On the crater, `rho.signed_frobenius_strong` costs fewer operations
  than `rho.negation` by a factor that grows like `√n` (ideal `√(2n/2) = √n`,
  reduced by the solver's efficiency). Level-1 curves have no counterpart.

## Falsification

H1 is falsified at a given `n` if the neighbours' mean exceeds or undercuts the
crater's by more than the control arm's interval, in either direction. H2 is
falsified if the Frobenius arm is not cheaper than `rho.negation` on the
crater. A single `n` failing is reported as failing; sizes are not dropped.

## What is not claimed

No transfer is computed: an explicit isogeny between a level-1 curve and the
crater would give the former the crater's cost plus the evaluation cost of a
degree-ℓ map. That map is not built here, so nothing below says a level-1 curve
is easier or harder than the crater against an attacker who can walk.
The n = 83 curve is treated structurally in `../volcano_koblitz_n83_20261018/`;
its rho cost (`~2^41` operations) is not run.

## Stop rules

Every session is verified with `ecbench verify --replay 8`. A failed or
timed-out execution is reported with its recorded reason and the sizes it
affects. Nothing is rerun after the results are read except to repair a
recorded harness failure, and then both runs are kept.

## Addendum, 2026-10-18 (written after the n = 19 table was read, before any other size was analysed)

1. **The A/A control cannot supply an interval.** `rho-neg-aa` repeats `rho-neg`
   with the same seeds, and ecbench's counted operations are deterministic, so
   the two arms are bit-identical (zero spread). The H1 criterion "inside the
   control arm's interval" is therefore replaced by: the 95% hierarchical
   bootstrap interval of the ratio (neighbours / crater) of mean counted
   operations, resampling curves and then targets, contains 1. H1 is falsified
   at a size when that interval excludes 1. This is the criterion reported.
2. **Rounds are not samples.** For the same reason repeated rounds of one
   workload are identical; the unit of replication is the workload (curve,
   target). The analysis (`examples/volcano_layers_analyze.rs`) collapses
   rounds first.
3. **H1 is stated for both generic solvers**; a size counts as failing H1 for a
   solver when that solver's interval excludes 1. Eight such intervals are
   reported (four sizes, two solvers); at a 95% level about 0.4 are expected to
   exclude 1 by chance alone, which is noted in the report, not used to discard
   a failure.
4. **H2 is stated per size**: the paired ratio `rho.signed_frobenius_strong /
   rho.negation` on the crater's identical targets, against the ideal `1/√n`.
   A size where the ratio exceeds 1 falsifies H2 there (the Frobenius solver's
   fixed costs dominate at small `r`).
