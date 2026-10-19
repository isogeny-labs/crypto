# Koblitz modular carrier: executable subgroup transfer and IC relations

- **Status:** open
- **Kind:** implementation gap; low-genus existence question; partial formalization
- **Affected work:** [investigation](../research/koblitz_modular_ic_20261018/REPORT.md), [original construction](../research/notes/ecc2k130/RESEARCH_ECC2K130_CURVE_CONSTRUCTION.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-18

## Problem

The modular construction realizes the ECC2K-130 trace-zero isogeny factor,
but no evaluated map preserving its prime subgroup or efficient IC relation
algorithm on that factor has been constructed.

The source is E_0/F_(2^131), registered as
`icv1-f2m131-tm22283658519494248867-115e0dc5`, prime subgroup order
`680564733841876926932320129493409985129`. The signed-character carrier
has genus 378,708,949 and desired factor dimension 130. Neither number is
a measured recovery cost. No challenge point is imported or attacked.

## Evidence and uncertainty

- **Observed:** the native prime-modulus scan and exact Frobenius checks
  in [results.json](../research/koblitz_modular_ic_20261018/results.json).
- **Derived:** the character modulo 787 supplies the trace sign without
  a separate conductor-3 twist. The written cyclic argument rules out
  geometrically cyclic degree-131 carriers of genus <394, for any quotient
  genus, with the descent and simplicity hypotheses in the report.
- **Suspected explanation:** no claim that every low-genus route is blocked.
  Noncyclic covers, gluing/polarization, other map degrees and projected
  relation algorithms remain separate possibilities.
- **Missing capability:** explicit map evaluation, r-torsion kernel/action
  certificate, destination relations/descent, and a full geometric Lean bridge.
- **Not established:** a carrier of genus 130--300, a useful modular
  quotient, faster IC, complete cost, or nonexistence outside the proved family.

## What would unblock it

- [ ] Supply a concrete F_2 curve/correspondence or a direct modular relation
  algorithm with exact inputs and an evaluator; retain all exceptional cases.
- [ ] Verify a nonzero order-r image and recoverability, including the map's
  kernel and any projector denominator, on public synthetic fixtures.
- [ ] Implement relation generation, rank, individual descent and recovery;
  compare complete cold costs with the current IC and signed-Frobenius rho
  baselines on identical one-target fixtures.

The first two conditions establish an executable transfer. The third is
needed for an IC improvement; existence of the transfer alone is insufficient.

## Next discriminating step

- **Action:** test a specified noncyclic construction, polarization/gluing
  candidate, or modular quotient at the n=5 rung; do not repeat the excluded
  degree-131 cyclic family as a search for a genus <=300 carrier.
- **Inputs and controls:** both Koblitz signs; known n=3 carriers; n=7 as
  a split-target control, not an analogue of simple A_131.
- **Limits:** preregister the selected construction and finite scope before
  running it; no execution is scheduled by this problem record.
- **Outcomes:** a verified curve and map justify scaling that specific
  mechanism; bounded failure leaves other constructions open; an abstract
  factor without a map leaves the executable-transfer condition unresolved.
- **Recheck trigger:** a specified construction or independently reviewed
  proof closes one of these missing bridges.

## Resolution and history

2026-10-18: new investigation records the signed-character construction,
exact arithmetic controls, and extension of the cyclic bound. Unresolved.

## Round 02: evaluated control and remaining constructions

[Round 02](../research/koblitz_modular_ic_20261018/rounds/02/REPORT.md)
adds an explicit n=3 control: all seven source subgroup points are transferred
through all 56 Klein quotient maps, 42 nonzero and 14 zero, with 393
principal-function certificates checked separately. Geometric line norms
produce 12,036 relations on 65 closed-point columns of rank64 modulo7;
anchoring the flex divisor recovers all seven public fixtures. The smaller
F8 factor base recovers four fixtures and retains three failed descents.
This closes the n=3 executable-control obligation; it does not resolve the
registered n=131 transfer or its relation algorithm.

The [ledger](../research/koblitz_modular_ic_20261018/rounds/02/ledger.json)
preserves finite searches of 722 genus-5 models and 2,424,825 genus-6 forms
or sections, with zero A5 polynomial candidates in the enumerated families.
Two del Pezzo Frobenius types (5 and 3+2), other genus-5 constructions and
higher genera remain untested. Frobenius-module gluing candidates at primes
5,11,19 have been enumerated; integral polarized gluing and curve realization
remain missing. The companion lattice calculation does not enumerate all
isogeny representatives.

At n=131 the separate-orbit recurrence matrices have rank260 and rank261
with a base anchor; a supplied cross row gives rank262 as an information
control. The Eisenstein character-pair screen fails its necessary condition
at five inert primes, but no general cusp/Hecke identification is proved.
Actual higher-n interior relations remain open.

Thirteen additional structural Lean lemmas compile and have standard-foundation
axiom audits. The full geometric bridges are still open and named in the
[proof map](../research/koblitz_modular_ic_20261018/rounds/02/PROOF_MAP.md).
Next discriminating constructions are the remaining del Pezzo twists,
Rosati-compatible integral gluing lattices, an evaluated n=5 map, and interior
principal-function relations on its modular factor. The user determines IC
speed assessment; this round contains no timing comparison.

## Round 03: completed searches, finite gluing, and n=5 controls

[Round 03](../research/koblitz_modular_ic_20261018/rounds/03/REPORT.md)
tests the remaining del Pezzo types (5) and (3,2) using five conic points,
131,070 anticanonical-square sections, six extension counts, and separate
conic-restriction controls. No A5 candidate appears in these systems.
Published genus-5 and genus-6 censuses also have no matching polynomial:
7,829 genus-5 records and 38,327 genus-6 classes representing 72,227 curves.
Together with the genus-4 count tests, this conditionally excludes F2
Jacobian carriers containing the full A5 factor through genus 6. The census
classifications and genus-4 maximum are external assumptions, not independent
Lean classification proofs. Finite-subgroup representations are a separate
question.

The integral invariant-Hermitian search now checks all five congruence cosets
within its frozen coefficient box: 12,005 forms, 29 positive, 24 retained
sign-specific candidates of determinants 5 and 19. Separate integer,
kernel-pairing and exhaustive matrix checks verify 14 gluing cases with
140 anti-isometries. These specify finite polarization data and, under the
written polarization/descent bridge, principally polarized quotients. No
quotient equations or Jacobian realization are implemented. Four determinant-19
cases have no norm-one vector in the tested coordinate box; no global
indecomposability claim follows.

Both n=5 order-11 subgroups now have evaluated augmentation tuples and
cubic pullbacks to genus-3 covers over F32. Geometric functions yield 1,770
rows of rank 42 on 43 columns for a=0, and 81 rows of rank 20 on 21 columns
for a=1. All 22 public fixture scalars are recovered and checked. Separate
verification reconstructs every principal row, both ranks, 64 projected
classes and all target descents. The initial deficient a=1 collections and
one corrected metadata count remain frozen. This is a same-field control;
the actual F2 modular-Jacobian evaluator and direct interior modular
relations remain missing.

Thirteen new named Lean lemmas cover determinant compatibility, norm and
alternating identities, graph isotropy/intersections/injectivity, cubic
substitution, conic intersections, conditional adjunction and count
obstructions. Their kernel build and transitive axiom audit pass. Full
geometry and census completeness remain explicit bridges in the
[proof map](../research/koblitz_modular_ic_20261018/rounds/03/PROOF_MAP.md).

The next discriminating work is a complete short-vector search using exact
inverse-Hermitian coordinate bounds, explicit quotient/polarization geometry,
and an actual divisor projector/evaluator on the specified n=5 modular factor.
Acceptance requires divisor inputs/outputs, exceptional cases, a nonzero
prime-subgroup image, and relations generated on that destination. The
same-field control does not discharge this obligation. No timing experiment
or efficiency assessment is part of this round.

## Round 04: explicit quotient and modular closed-divisor projection

[Round 04](../research/koblitz_modular_ic_20261018/rounds/04/REPORT.md)
constructs the natural determinant-five quotient A5(E) x E -> Res(E) for
both signs, with F2 affine equations, rational inverse, all rational group
additions, and both 25-point geometric kernels over F_(2^24). All fourteen
finite gluing presentations now have explicit integral quotient bases and
principal alternating forms with compatible Frobenius. This latter evidence
is prime-to-two polarization data; it is not coordinate equations for every
individual graph, particularly those with supersingular elliptic cofactor.
The two determinant-19 A5 forms admitting the tested cofactor gluing now have certified global minimum 3.
Decomposition and explicit equations for their glued fivefolds remain open.

The n=5 modular track now transports both cyclic-seven stages through actual
degree-eleven isogenies, reconstructs all twelve X0(49) targets for 24
supported inputs and verifies T11=[4]. Fixed Weil-paired bases and canonical
modular Frobenius give 15,840 geometric level points, grouped into 544 closed
columns of degrees 6 and 30. Their forward twisting-correspondence projections
are actual order-11 points: 510 nonzero for a=1 and 520 for a=0. Global
moduli/character identification is a written bridge and not fully formalized.
The evaluator's finite closed-divisor domain is explicit; it does not supply
arbitrary modular Jacobian normal forms or a reverse subgroup embedding.

Full-fiber pullbacks of principal norms give 102 genuine direct modular
relations, separately reconstructed and checked with maintained-library
point arithmetic and BigInt rank. Their rank is 4 on 544 columns. A new
Lean matrix theorem explains the ceiling: every such row factors through
only four closed base fibers. Collecting more of those functions cannot
supply the remaining 539 ranks. The proposed level-coordinate function
x(P)+1 is rejected: its zero divisor has degree 9,240 and none of the zeros
is in the current factor base; poles also remain unresolved.

Eleven new structural Lean lemmas build and pass the transitive axiom audit.
See the [proof map](../research/koblitz_modular_ic_20261018/rounds/04/PROOF_MAP.md)
for actual modular geometry, principal-divisor completeness and polarization
bridges. Failed integer lifts, first counts, deficient matrices, unsupported
branches, rejected partial divisors and failed builds remain archived.

Next actions have checkable acceptance conditions:

- Construct equations and a compatible principal polarization for a specified
  nonnatural quotient, or certify its polarized decomposition.
- Compute the forward/reverse correspondence composition and denominator on
  eleven-torsion; an evaluated forward point image alone does not discharge
  the subgroup-to-Jacobian transfer requirement.
- Factor psi11(1,b), construct its missing cyclic49/complementary-eleven
  closed orbits and pole support, and admit level-dependent principal rows
  only with a complete divisor certificate. Require rank beyond the proven
  four-fiber ceiling before attempting individual descent.

These steps do not resolve the original n=131 construction or establish a
cost advantage. The user assesses IC implications; no timing protocol or
background execution is scheduled by this record.

## Round 05: reverse composition and a level-dependent divisor

[Round 05](../research/koblitz_modular_ic_20261018/rounds/05/REPORT.md)
derives the Gram matrix of the twelve degree-660 maps: diagonal660,
off-diagonal20, using the two-transitive deck action and Hecke sum [4].
For both signed maps, the polarized adjoint composition is [32000],
including the actual H=I+ones target polarization of degree25. This is a
unit at11, yielding a written integral reverse correspondence and a split,
11-saturated Tate-module image. Lean checks the exact matrices and the
conditional torsion/Bezout implications. The reverse divisor formula has
positive/negative degree at most6600; its fibers and reduced Jacobian
class remain unevaluated. Full moduli/adjunction geometry is written.

All five determinant5 forms have explicit integral equivalences to the
natural form, with Lean inverse/pairing certificates. Their transported
diagonal graphs use the natural quotient equations. Only one of the ten
frozen first graphs matches that kernel directly, so nine remain open;
the determinant19 mixed quotient equations are still open.

The polynomial psi11(1,b) is irreducible of degree15. Its point-rational
twist has trace231. Maximal-order Frobenius on cyclic49 subgroups and
normalized eleven-complements yields44 closed zeros of degree210.
A supersingular deformation gives3080 poles, each of order3, with
zero projected images since the six target base points are killed by8
and A5(F2) has order11. This supplies one written level-dependent
principal relation beyond the four-fiber span. Its symbolic projected
matrix has rank5 on588 columns. New44 column coordinates/images have
not been evaluated; the evaluated matrix remains rank4/544.

The [proof map](../research/koblitz_modular_ic_20261018/rounds/05/PROOF_MAP.md)
separates18 new named Lean theorems from geometric arguments. Next gates:
evaluate and replay the44 new images and their relation; evaluate a nonzero
reverse pullback divisor; construct a specified unmatched graph quotient.
No n=131 transfer, individual modular descent or IC efficiency claim follows.

## Round 06: evaluated zero columns and explicit reverse coordinates

[Round 06](../research/koblitz_modular_ic_20261018/rounds/06/REPORT.md)
evaluates all 44 degree-210 columns. Both signs give zero on every column;
the relative 14-term traces already vanish. Eight degree-7 fibers have
constant target sum T=(0,1), checked for all 12 maps. Thus evaluated rank
5/588 adds no constraint on the old nonzero-image columns: those matrices
remain rank 4 on 510/520 columns. This resolves the prior image-evaluation
gate but leaves the original richer-relation requirement open.

All ten frozen determinant-5 graphs now have split E^5 equations, explicit
torsion basis binding, effective F2 descent and positive principal
polarizations. Their 250 kernel points and rational additions are checked.
The old direct-natural-kernel mismatches are preserved. Expanded F2
projective ideals, determinant-19 mixed quotients and Jacobian realization
remain open; no bounded mismatch is treated as a nonexistence proof.

The even reverse map on one order-11 generator is now evaluated as an
unreduced divisor with all 20 pullback fibers in explicit coordinates.
The positive seed splits into 660 points over F2^120; the negative seed
uses 60 irreducible degree-11 point algebras. Frobenius and deck changes
give 7,200 entries and degree 6,600 on each side, with F2 descent checked.
The forward check uses the round05 Gram identity and evaluated targets.
It does not independently norm-evaluate all components or reduce the
genus-2841 Jacobian class. The odd reverse divisor also remains open.

The [proof map](../research/koblitz_modular_ic_20261018/rounds/06/PROOF_MAP.md)
records 14 new named Lean proofs and the remaining geometric bridges.
Both failed fiber-splitting assumptions are retained. Next acceptance gates:
independently norm-evaluate target functions on the degree-11 components;
produce a complete principal row with nonzero new projected images from
functions separating second-7 choices; construct and verify a mixed
determinant-19 quotient. These are implementation/construction obligations,
not proved obstructions. No IC timing or n=131 transfer is supplied.
