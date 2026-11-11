# Frozen protocol: algebraic relation-oracle search, k = 20 (2026-11-07)

Frozen before any script in this directory was run. Edits after the first data file appears are
appended under "Amendments" with a time, never made in place.

## Object

Curve E: y² = x³ + 926x² + x over F_p, p = 1,048,573, #E = 8r cyclic, r = 131,203 (prime).
One target Q per solve; trial points R = aG + bQ (1 group operation each). No ecbench run.

## Counted units

- Native op: one F_p multiplication or squaring, counted by the wrapper in `scripts/fp.py`.
  Additions and subtractions are not counted. An inversion is counted as the multiplications of its
  Fermat chain (p − 2: 19 squarings + 18 multiplications = 37), done by the counted wrapper.
- Group operation: 1 GAE. Native ops are priced at w GAE, w ∈ {0, 0.1, 1}.
- Two pricings of an oracle that itself computes group elements (for example x(R ± P1)):
  - P_N (the screen's convention): those field multiplications are native, priced at w.
  - P_G (primary): each group element computed is a group operation, 1 GAE, as rho's own steps are.
  The verdict uses P_G. P_N is shown so the numbers can be read against ALGEBRAIC_SCREEN.md.
- Model: S(w) = [G_setup + w·N_setup + T·(1 + g + w·c) + w·N_LA] / √r, T = (F + 1)·r / C, with
  g = group operations per trial beyond the trial point and c = native ops per trial. Rho reference:
  measured rho-neg S = 1.600, 1.716, 2.764 at w = 0, 0.1, 1 (T13 band, as in the screen).
  Gap = S ÷ S(rho-neg). Setup and linear algebra are charged (the screen ignored them); the
  setup-free number is also reported.
- Coverage C: number of points of E[r] that are ±P1 ± P2 (m = 2) or ±P1 ± P2 ± P3 (m = 3) with base
  points P_i, counted by exhaustive enumeration where feasible, otherwise derived and labelled.

## Classification rule (frozen)

A design whose relations come only from group operations and equality tests between computed
elements is **generic**. The collision floor (curve25519_rho_parity_bound_20261104 README) bounds it
from below, and its comparison belongs to that floor and to the existing generic incumbent (wwalk,
about 1.8x rho). It is reported but is not an algebraic oracle and does not trigger a test design:
proposing it would relabel BSGS or rho as index calculus. Only a non-generic design within 10x of
rho-neg at w = 0.1 under P_G, with setup and linear algebra charged, triggers a single-target
ecbench test design.

## Designs and frozen predictions

| id | design | prediction |
|:--|:--|:--|
| D1 | m = 2, random base in E[r], full coverage, one root-find per base point | P1: the two roots in x2 of S3(x1, x2, xR) are exactly x(R + P1) and x(R − P1) on every one of 2,000 random instances, so the root-find is a group addition and D1 is generic. P2: measured c with shared inversion is between 1,500 and 4,000 native ops at F = 256. |
| D2 | base in a multiplicative coset g·μ_d, norm/resultant decision in F_p[X]/(X^d − c) | P3: the counted polynomial decision costs at least 10x the per-point decision at the d needed for full coverage; its cost grows at least linearly in d. |
| D3 | base closed under E[8] (2-torsion linear in Edwards form) | P4: coverage per unknown and decision cost per trial equal those of an E[r]-only base with the same number of unknowns, within 5 %. |
| D4 | precomputed eliminant Φ of the covered set: hash table (generic) and polynomial with batched multipoint evaluation (algebraic) | P5: the polynomial version is at least 10x behind rho at w = 0.1; the hash version is generic. |
| D5 | m = 3 at full coverage (F about 46): direct and meet-in-the-middle decisions | P6: both are generic; under P_G neither beats the collision floor for its F. |
| D6 | coverage against decision cost, per family, at the optimum F | P7: no non-generic family has an optimum within 10x at w = 0.1. |
| D7 | small-abscissa base x ≤ X with a lattice (linearised S3) oracle, O(1) native work per trial | P8: LLL recovers planted pairs only for X ≤ 4; the design is at least 30x behind rho at w = 0. |

P9: every non-generic design's gap grows with r (exponent of S(w) in r above 0 at w = 0.1).

Failed predictions stay failed and are reported as such.

## Amendments (after data)

- 2026-11-07, D1 first run: `fp.S3` had the x3² and constant coefficients swapped, so the identity
  check reported 6,000 failures. The polynomial was corrected to the Montgomery form
  (x1 − x2)² x3² − 2((x1 + x2)(x1x2 + 1) + 2A x1x2) x3 + (x1x2 − 1)², and the run repeated: 0 failures.
  The decision and coverage numbers of the first run did not use `S3` and were unchanged.
- 2026-11-07, D2: the first polynomial run used (ae)^(2n) instead of (ae)^(n+1) in the Lucas ladder
  step W_(2n+2); the consistency check against the per-point oracle caught it (27/30 and 28/30). After
  the fix both checks are 30/30, and planting was strengthened so half the checks are true "yes" cases.
- 2026-11-07, D5: the direct oracle also reports decompositions with fewer than three terms; the
  agreement check was widened to compare it with the MITM oracle plus the base itself (30/30 at every F).
- D1 was measured for F = 128 ... 448 only; the best F under both pricings is the smallest measured
  value, so the D1 optimum is a boundary value (see README).
