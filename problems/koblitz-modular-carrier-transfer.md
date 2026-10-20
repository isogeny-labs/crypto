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

## Round07: independent norms, odd transfer and nonzero-image rank6

[Round07](../research/koblitz_modular_ic_20261018/rounds/07/REPORT.md)
checks every even reverse entry by actual modular target coordinates and
finite-algebra point traces, independently of the Gram calculation.
The corrected odd seed has55 split points and55 degree11 components over
F2^660. Its formally antisymmetric representative [8T]−[−8T] yields all20
pullbacks with2200 coordinate entries, degree6600 each side and checked
formal F2 descent. Its independent forward image returns the source.
General genus2841 Jacobian reduction remains an implementation gap.

Two complete full-map principal functions have ordinary zero degrees6600
and13200 and poles at modular cusps with zero projected images. They add
60 and120 closed columns of degrees60/660. Verified geometric identity
checks find no joins to the prior columns. Evaluated rank is7/768;
nonzero-image ranks are6 on675/678 columns. The row sums, source degrees,
etale Galois steps and ranks have separate checks. This meets the requested
rank-beyond-five experiment, while collection costs, further families and
individual descent remain open.

All four frozen determinant19 graphs now have actual mixed ordinary/
supersingular coordinates, matching pairing bases and361-point Frobenius
checks. The finite etale translation atlases and factored invariant circuits
are explicit. The supersingular cofactor[19] cover is expanded into
rational polynomials of degrees361/360 and540/540, with a kernel-checked
binary polynomial identity and pointwise tests for both cofactor traces.

**The full mixed invariant image ideal is still uncomputed.** These cover
equations and translation atlases do not supply an intrinsic global
five-dimensional quotient equation system. This is a construction and
elimination implementation gap, not a mathematical nonexistence result.
To close it, produce explicit generators and relations for a separating
mixed invariant coordinate algebra or a verified theta embedding, include
its saturation and projective charts, descend it to F2, and verify the
four specified graph kernels and polarization pullbacks on that model.
Computing only the cofactor cover does not meet this acceptance condition.

The initial odd adapter used an incorrect q-mod11 entry and a degree5
embedding for a degree10 parameter. Those probes produced no accepted
fiber; interrupted attempts and the stopped stale process are retained.
The corrected adapter checks full group orders and bounds primary loops.
Seventeen new named Lean results pass, with geometric bridges specified in
the [proof map](../research/koblitz_modular_ic_20261018/rounds/07/PROOF_MAP.md).
The next discriminating quotient step is the mixed invariant image ideal;
the next relation step is a complete function family with checked new
nonzero images and collection accounting. No IC efficiency verdict follows.

## Round08: complete line norms and global Kummer atlases

[Round08](../research/koblitz_modular_ic_20261018/rounds/08/REPORT.md)
completes the nonzero-line norm family and the fixed line∞. Its exact
geometric joins use full split bases and primitive-generator presentations
of degree11 components. Six additional functions beyond the first round07
row give final evaluated rank12/1308 and nonzero-image rank11 on1165/1160
columns. Separate small-field arithmetic checks every image coordinate
and principal sum; independent elimination agrees with the ranks. The
larger rank comes with540 additional columns. Collection costs and
individual descent remain unmeasured/unimplemented.

For the four determinant19 mixed quotients, the actual graph generators
give40 explicit Miller functions. Their denominators reduce to1, with
degrees9 and8 in A(x)+B(x)y. Exact divisor checks and a written
dual-isogeny/Kummer argument identify two intrinsic equations per quotient
chart over F2^360. Six translated charts cover the global quotient;200
exact rational identities and verified19th roots give their transitions.
The parsed checks include100 curve identities and400 point translations.
Lean proves the six-chart coverage and cocycle implications, along with
the Miller-divisor and Kummer identities.

This discharges the missing intrinsic chart and explicit global gluing
over the splitting field. **Effective F2 equations and a closed projective
embedding remain open.** The polarized fivefold has not been identified
as a Jacobian. General genus2841 reduction and individual modular descent
also remain open. These are construction gaps, not nonexistence results.

The next quotient experiment must compute Frobenius on the actual Kummer
coordinates, verify its compatibility with the200 transitions, and produce
an effective F2 model whose base change recovers this atlas and the retained
polarization pullback. The next direct-relation experiment must use coupled
functions beyond the now-complete single-line norm family, with complete
zero support, geometric joins and projected pole accounting. Neither
experiment is represented as already executed.

## Round09: explicit Frobenius and invariant rational generators

[Round09](../research/koblitz_modular_ic_20261018/rounds/09/REPORT.md)
computes the Frobenius lift on each of the four intrinsic Kummer quotients.
The split dual base carries B=D^(-transpose), rather than D. All correction
factors are explicit rational circuits. Forty-eight univariate identities,
40 coefficient-Frobenius identities and80 exact slices pass. The parsed
checker verifies576 chart squares and repeats the two360-step cocycles
per graph. A written constant-deck argument explains why those fixtures
fix the global cocycle, using connectedness of the retained Kummer cover.

A trace-dual power basis gives4,320 invariant rational generating circuits
per quotient. All129,600 trace-pairing entries and360 idempotent sums pass;
5,760 sampled invariant values yield16 coordinate recoveries and128
Frobenius-equivariance checks. Seventeen new named Lean lemmas prove the
linear/matrix, root, orbit-sum and recovery implications. Native field
semantics and the global geometric bridges remain written, not fully Lean.

**Remaining quotient gap:** compute a manageable polynomial relation ideal
for an explicit invariant generating set, saturate its denominators,
verify the base-change isomorphism to all six charts, and carry the retained
polarization to that F2 model. Rational generating circuits do not supply
this ideal or a closed embedding. Jacobian identification remains separate.
The next discriminating experiment is to reduce the trace-generator family
using a Frobenius-adapted base coordinate system, then eliminate the two
root variables on one declared open chart with exact inverse substitutions.
Success must include denominator saturation and gluing; a degree/resource
failure is recorded without a nonexistence conclusion.

All50 signed diagonal-twist/reflection controls are negative for retaining
the known A5 component in an F2-stable reflection quotient. The stable
case collapses the known module to E^4 and its fixed part to E^2. This is
not a full Jacobian-factor exclusion. Actual reflection quotient equations
and genus, other factors and other carrier families remain open.

Eight coupled functions are nonzero on all720 known norm columns.
Outside zeros, multiplicities and poles have not been computed, so no
complete new relation is promoted. Existing ranks remain12/1308 and11 on
1165/1160 nonzero-image columns. The next direct-relation step must locate
complete outside support for a coupled function with checked pole degree,
then evaluate and join every new column before reporting its rank. General
Jacobian reduction and individual descent remain open. The user assesses
IC efficiency; this round contains no timing or solver measurement.

## Round10: F2 polynomial models and the remaining boundary/intersection gaps

[Round10](../research/koblitz_modular_ic_20261018/rounds/10/REPORT.md)
supplies four explicit quadratic F2 polynomial systems for specified
descended dense opens of the determinant19 quotients. Two have424080
scalar equations each and two415800. The lossless coefficient-block
format contains every operation,coefficient reduction and residual;
72 literal expanded equations and4608 controls supplement it. Inverse
witnesses give saturation by the product of denominator norms. The
scheme-theoretic equalizer/base-change argument is written separately
from the exact split-fixture checks. Coefficient variables are indeterminates,
with no Boolean equations. Twenty-three new named Lean proofs pass.

**Quotient exit criterion still open:** compute a global projective
presentation whose saturated boundary recovers all six retained charts
and whose polarization pullback is verified. The new affine graph model
discharges the missing effective F2 open equations and localized closed
embedding,not that global obligation. An eliminated ideal in only the
original4320 coefficient variables is also uncomputed. A two-gate prototype
has an explicit false point under naive homogenization;its repaired
three-generator ideal has a full Groebner saturation certificate. The full
quotient's homogenizing-variable saturation is not inferred from it.
The next discriminating geometry step is a controlled elimination and
boundary chart on one graph,with exact inverse substitutions and h-saturation.

The exact divisor of x0*xinf is complete on22 closed cusp points,with
zero/pole degrees2640 each and every projected image zero. All eight pole
divisors and all cuspidal zeros are known. The other seven residual
noncuspidal zero degrees are2100,2520,3420,3780,2640,3450,3900,including
possible supersingular points. None is promoted as a complete new row.

The equal-map component has degree1280 and cuspidal degree420. Its
collision resultant is exactlyj^88*(j+1)^8. The ordinaryj=1 case has
zero matching cyclic49 lines among56,with the finite exclusion in Lean.
The actual supersingular automorphisms are bound to Hurwitz units by
point identities and a global degree bound;the torsion-module bridge is
written. Enumeration of3080 moduli classes finds400 matching labels in
40 degree10 Frobenius orbits. Their current multiplicity lower sum is400
against860 required. **The degree-exhaustion attempt fails with a460 gap.**
The labels are not explicit49/11 torsion coordinates. The next discriminating
calculation is the local deformation/intersection multiplicity at those40
closed labels,with an exact local model or a verified applicable CM
length theorem. It must account for the460 gap before claiming that
component complete. The antidiagonal component remains separate.

These are implementation and proof gaps,not nonexistence results.
Relation ranks remain12/1308 and nonzero11 on1165/1160 columns. General
Jacobian reduction,individual descent and Jacobian identification remain
open. No solver or timing run is supplied;the user assesses IC efficiency.

## Round11: diagonal gap resolved; whole-divisor and quotient exits remain

[Round11](../research/koblitz_modular_ic_20261018/rounds/11/REPORT.md)
resolves the previous460 gap using explicit characteristic2 local series,
not an imported odd-residue-field formula or a fit to the total degree.
The400 labels have contacts1,2,6,14 with counts260,100,20,20. The sum860
exhausts the retained diagonal residual degree. All40 closed degree10
orbits carry exact multiplicities. Actual11-torsion coordinates,unit
bindings and twelve quaternion/Velu maps are checked;the local moduli
bridge is written and explicit49-torsion coordinates remain open.

A coupled multiplication/two-inverse graph has a five-quadric h-saturated
projective ideal,ten S-pair checks and six dehomogenized charts. This
discharges the specified prototype only. The whole quotient exit criterion
from round10 still requires an actual coefficient-circuit boundary,
saturation,all retained charts and a verified polarization on a proper
model. No nonexistence is inferred from the uncomputed full ideals.

All seven whole noncuspidal supports remain open. The x-sum diagonal860
is complete,leaving antidiagonal1240. The product-plus-one function is
exactly two translated T-fibers of degrees1280 and1360 with polar terms;
these fibers are not evaluated. The x-sum-plus-one ambient normalization
has explicit AS equations and genus3;no cover or transfer to it has been
constructed. Equal-y factors into the diagonal and a degree2680 residual
curve after pullback,before common polar cancellation.

Next acceptance tests: enumerate the antidiagonal and translated T-fiber
closed support with local lengths and projected point images;construct a
full finite principal divisor before adding a row. For quotient geometry,
certify a boundary chart on an actual expanded graph and verify its
transition to the retained Kummer atlas. Twenty-two additional Lean
proofs pass;the full gate checks168 theorems. Native semantics and global
moduli/deformation geometry remain written obligations. Relation ranks
and IC assessment are unchanged.

## Round12: complete SS strata; actual circuit saturation step

[Round12](../research/koblitz_modular_ic_20261018/rounds/12/REPORT.md)
constructs actual coordinates for all56 cyclic49 labels through local
7-isogeny chains. All3080 supersingular source points form308 closed
degree10 orbits. Their complete zero degrees for the eight functions are
1600,760,1540,1020,0,2060,2080,1720. Exact ordinary degrees still requiring
support are500,1760,1880,2760,0,580,1370,2180. The antidiagonal SS degree
740 has contacts1,2,4 and leaves ordinary500. All400 old diagonal contacts
are reproduced,including order14. No whole new principal row is promoted.

All SS target points are killed by8;their11-primary projections vanish.
Thus these completed strata do not increase nonzero-image rank. The next
relation exit is an enumerated ordinary support,especially the antidiagonal
500 or product-plus-one580,with all closed points,local multiplicities
and projected images. Mere degrees or a finite-field point screen do not
discharge that exit. All seven whole divisors remain open.

The four full homogenized quotient circuits now have explicit false
projective points satisfying all1679760 scalar equations but violating
certified saturated linear relations. Eight kernel-checked Lean results
verify the actual restricted circuits and missing relations. The defects
are removable auxiliary constant gates,not a nonexistence obstruction.
Constant propagation adds592 K-linear relations,or213120 generated scalar
linear relations,of h-membership depth<=5. They can be redundant. This
is an intermediate ideal with the same h-chart,not full saturation. The
proper quotient exit still needs the full boundary charts and polarization.

Twenty-four new named Lean results are supplied. Global moduli/etale
geometry and native field semantics remain written bridges. Explicit49
torsion point coordinates are still uncomputed. Failed preliminary bounds
and censored jets are retained;the protocol expands precision before the
accepted128/64/16-coefficient construction. No IC efficiency claim is made.
