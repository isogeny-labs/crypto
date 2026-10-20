# Round12 frozen protocol

2026-10-20 UTC. Parent539325b5e66719f4cea7ee1315659808103d5bcb.
Native Rust only for arithmetic, search, assembly and hashes. No timings
or IC efficiency assessment. Earlier rounds remain write-once.

## S18 actual supersingular level49 coordinates and antidiagonal

Construct all8 actual7-kernels on E0:y²+y=x³ over F4096,calibrate
P,I(P) against the reduction modulo7 of the retained49-module I,J.
Bind norm7 Hurwitz maps to actual Velu maps and units,with global degree
bounds supplementing finite point identities. Enumerate all56 cyclic49
chains by reduction modulo7 and dividing alpha7(S) by7 modulo49.
The actual49 torsion coordinates are not inferred from this module map.

Use E_s:y²+sxy+y=x³ and prime-to2 etale7-division roots. Lift source
and quotient7-kernel x-roots through order63,using simple-root Newton
iteration and substitution checks. Compute all first/second7 quotient
Weierstrass coefficients and discriminants. Recover each7 Hauptmodul
from j'/j=t^6 and A(t)+j*t^7=0,A=t^8+t^4+1,retaining the two
supersingular constant candidates until one is distinguished. Precision64
for torsion/discriminants,32 for the square-root ratio and Hauptmodul,
and8 for the fourth-root eta coordinate. Recover the minimal y coordinate
from (eta_y+x+1)/(x²+x+1),including leading-term division when its
constant denominator vanishes. Abort rather than choose an unresolved
branch. Export all local equations and coordinate identifications.

Check all56 labels lie among the six actual noncuspidal F4 points of
X0(49),with retained Hurwitz orbit sizes and Frobenius. Use these target
coordinates on all3080 source supersingular classes to locate diagonal,
antidiagonal and translated T components. Retain only justified local
multiplicity lower bounds until exact local contacts or degree exhaustion.
Do not infer ordinary support exclusion from a supersingular enumeration.
Antidiagonal1240 and the seven whole divisors stay open unless complete
supports and multiplicities actually exhaust the independent degrees.

## Q10 full quotient homogenized boundary controls

Parse the four actual round10 coefficient-block circuits. Restrict
extension-coordinate values at h=0 to0,1 in F2 subset K. Constants
then vanish;Add and coefficient-Frobenius constraints are linear;
Mul/Inverse require the product of their two input values to vanish.
Use native bit Gaussian elimination and deterministic branching on those
product constraints,at most100000 search nodes per graph. Seek an actual
projective point satisfying every homogenized circuit equation but
violating a relation certified to enter the ideal after multiplication
by h. Start with multiplication by a Constant(1) gate. Retain all failed
models/search caps. Check every scalar coefficient of every equation.
This is a test on the full emitted systems,not another toy prototype.
A counterpoint rejects naive homogenization but does not compute the
whole saturated ideal or proper quotient embedding.

## Formalization,evidence and CI

Add faithful Lean identities and structural implications for the new
coordinate recovery,saturation certificate and exact finite arithmetic;
keep global modular/etale semantics explicitly written where not formal.
Two source-bound IDC identities,64 issuer plus128 extra checks each.
Fresh byte replay and round11 replay,native/root tests,pinned full Lean
gate,site/workflow checks,deterministic archive and fresh repack,editable
SVG and inspected PDF. Track every requested requirement and failed run.
Update canonical reports,problem and existing PR. Diagnose CI failures
against the base;do not modify unrelated proofs or weaken checks to merge.

## Amendment A — local contacts after the target adapter

The adapter distinguishes six targets and shows that modular stabilizer
size can change under elliptic negation. Preserve the preliminary export:
its one-branch stabilizer count is not a justified antidiagonal lower
bound. Use the minimum of both branch ramification indices unless the
actual local difference is evaluated.

Before local contact evaluation: check that kernel-sum and level49
x-series coefficients lie in F4. Normalize every lifted11 quotient back
to E_z with its recorded quaternion/Velu special-fiber unit. The four
Weierstrass coefficient equations determine the normalization;z=s/u.
Compose the eight known coefficients of each level49 x-series with z(s).
At a noncuspidal target,x is etale,so x-difference gives the diagonal or
antidiagonal local contact,selected by the already computed y constants.
Contacts found below8 are exact;an all-zero order8 jet gives only a lower
bound8. Check the old diagonal contacts truncated this way as a positive
control. If further precision is needed,freeze it before expanding.
Compute F2 Frobenius on every source label and the resulting closed
supersingular orbits. Do not infer ordinary exclusion from this census.

## Amendment B — complete supersingular strata for the eight functions

The exact8-jet antidiagonal contacts are420 of order1,40 of order2,
and60 of order4,degree740. The diagonal positive control has20 jets
censored at8,matching the retained order14 points. Before increasing
precision: raise torsion/discriminant precision from64 to128,Hauptmodul
precision to64,and eta x precision to16. Keep all earlier exports.
Normalize11 quotients to order16 and recover minimal y by the etale
implicit elliptic equation. Evaluate all eight coupled functions at all
3080 supersingular source labels,with complete16-jets. If any vanishing
jet is still censored at16,retain only its lower bound. Compare the400
diagonal contacts with round11 exactly. Compute all source Frobenius
orbits and verify the function valuations are orbit invariant.

Only if every supersingular zero contact is resolved may the independent
noncuspidal zero degrees be split into exact supersingular and remaining
ordinary degrees. This does not enumerate the ordinary supports. Check
8-torsion of all six target points and state the resulting obstruction
to new11-primary image rank under the previously proved projection maps.

## Amendment C — certify the actual constant-gate saturation step

The full-system false point exploits an auxiliary constant multiplication.
Do not present it as an obstruction intrinsic to the abelian quotient.
Before constant propagation arithmetic: evaluate all constant-only circuit
nodes over the pinned F2^360. Emit the resulting homogeneous linear
relations and every multiplication-by-constant linear relation,with a
conservative explicit h-power membership depth propagated along the DAG.
Retain the original equations;this gives an intermediate ideal inside
the h-saturation,not a complete saturated ideal. Recovering a simplified
projective model remains open. Limit10000 gates per graph and preserve
all exact constants and the shared coefficient tensor reference.

Emit the four actual Boolean-restricted boundary circuits as Lean data
with kernel-checked residual and missing-relation claims. This formalizes
the finite restricted certificates;the scalar embedding into the original
360-coefficient algebra remains an explicitly identified bridge.
