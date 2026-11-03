# Round 02: carrier models, modular relations, and transfer

Frozen before execution, 2026-10-18. Parent commit: 047074d9912d64842995f9d590d8c0babdd48042.
User direction: review, generate experiments, track results methodically,
provide relevant Lean proofs, and pursue low-genus carriers, direct modular
relations and remaining items. The user assesses implications for faster IC.

No previous output is overwritten. Native Rust generates arithmetic and
experimental evidence; Python is restricted to document layout.

## LC0: bielliptic genus-5 curves

Enumerate Artin--Schreier double covers w^2+w=f of all five elliptic
isogeny classes over F_2. Models are E_0,E_1 and y^2+y=x^3,
y^2+y=x^3+x, y^2+y=x^3+x+1. Fix O as pole. Test exact rational pole
partitions (7), (5,1), (3,3), (3,1,1), (1,1,1,1), using the explicit
Riemann--Roch bases recorded by the native implementation. Include every
allowed lower coefficient, both constant twists, and every distinct
nonidentity rational pole choice, without choosing successful instances.
All listed poles have odd order; normalization is a smooth genus-5 cover.
Count through F_(2^5), construct P_C exactly, and test divisibility by
A_5 of both signs. A_3 is a positive-control target. Count any hits over
three additional extension fields. Preserve model coefficients and maps.

## LC1: exhaustive bidegree-(3,4) genus-6 family

Enumerate every nonzero F_2 bihomogeneous form of bidegree (3,4) on
P^1 x P^1: 2^20-1 forms, no equivalence or random sampling. Compute all
point-count vectors through degree 6 by additive-character orthogonality
and the Walsh--Hadamard transform. Compare deterministic selected forms
with direct point counting as an independent implementation control.
Test P_A5 divisibility for both signs; classify every candidate using
exact chart Jacobian ideals. Do not label singular forms as carriers.
If a smoothness computation exceeds its declared algorithmic cap, retain
it as unresolved. For a smooth hit, count additional fields and retain
the full equation. This family is not all genus-6 curves.

## MR0: direct Frobenius/diamond relation rank

Use n=3 (E_1, r=7), n=5 (both signs, r=11), n=7 (E_0,r=29),
and n=131 (E_0, registered r). Build the cyclic orbit rows
u_j - t*u_(j+1) + 2*u_(j+2)=0, t=+/-1. Measure exact rank modulo r
for one orbit, two separate orbits, fixing one base coordinate, and adding
one explicit cross-orbit row. Test actual elliptic point identities on
the small fields. At n=131 this is a scalar-module experiment, not an
evaluated modular-Jacobian correspondence. Record rank and nullity; the
experiment tests exactly what information these relations provide.

## MR1: Eisenstein-character necessary-condition screen

For signed level N=49*787^2 and control levels, take inert good primes
whose CM Hecke coefficient is zero. Compute D=exponent((Z/N)^*) and
gcd(r,(-p)^D-1) exactly, including Bezout witnesses. The check is a
necessary condition for an Eisenstein-character pair congruence, not an
implemented modular-unit relation collector or a blanket cuspidal theorem.
Do not extend its scope without the corresponding Hecke/cusp proof.

## KT0: Klein transfer control

Over F_8 enumerate C: x^3y+y^3z+z^3x=0. Use the cyclic quotient and its
diagonal conjugates, with explicit formulas and exceptional points checked.
Check degree-three fibers and elliptic push-pull on the order-7 subgroup.
Construct Frobenius traces of pullback divisors. Determine their classes
against D=(0:1:0)-(1:0:0), using exact Riemann--Roch interpolation and
certified principal-divisor identities. A constructed divisor is not
reported as a nonzero transfer without the class check.

## GL0 and PF0: gluing candidates and formal proofs

Compute real-Weil resultant data for A_5 and every elliptic cofactor,
including common Frobenius factors at each small candidate gluing prime.
These are candidate records; no polarization or Jacobian is asserted.
Formalize relevant relation-scaling, push-pull injectivity and polynomial
implications using pinned mathlib/Lean when feasible. Keep the written
geometry, checked algebra, numerical checks, and unformalized bridges
separate in a proof map. No theorem may assume its advertised conclusion.

## Tracking and controls

The experiment ledger records requirements, parameter coverage, commands,
source and input hashes, result paths, failures, exact unresolved obligations,
and the next discriminating step for every track. No speed verdict is used
as a stopping or promotion rule. There is no performance measurement in
this round. Stop each finite enumeration when its stated family is covered;
keep broader or unimplemented requirements open. Review findings and generate
the next bounded experiment from the evidence rather than declaring the
workstream finished. Raw receipts are write-once and archived with hashes.

## Amendment 1: LC0b closed branch places, before the first round run

The p-rank formula suggests a further explicit genus-5 family: an ordinary
elliptic quotient with four simple geometric branch points, comprising O
and one degree-3 place. E_0 has no degree-3 places; E_1 has four. For every
such place Q and c in F_8 with Tr(c)=1, form Tr(c*w_Q)+constant,
where w_Q=(y+y(Q)+x(Q))/(x+x(Q)), and both constants are included.
Express this as an F_2 rational function using the cubic denominator and
trace numerator. Enumerate all 32 models, verify the rational expression
and removable values, count through genus and beyond for hits, and record
LC0b separately from the original 690-model rational-pole experiment.

## Amendment 2: transfer replay and del Pezzo families

KT0's first run enumerated 56 quotient maps and checked G and 2G. Preserve
it, then check every scalar 0..6 for every map in a second output; record
the 7-torsion class and principal-divisor witness for each. Test recovery
through the first nonzero map without presenting the fixture labels as
unknown-scalar solving.

The completed LC1 search covers balanced trigonal models, not the remaining
genus-6 families. LC2 enumerates all nonzero anticanonical-quadric sections
of quintic del Pezzo surfaces obtained by blowing up four general-position
points of P^2, with Frobenius cycle types 1+1+1+1, 2+1+1, 2+2, 3+1, and 4.
For each type choose the first Frobenius-stable general-position configuration
in the native deterministic order. Compute its 16-dimensional F_2 space of
plane sextics double at those points, and enumerate all 65,535 nonzero
sections. Count on the blown-up surface, replacing base points by the
exceptional tangent-cone fibers; direct-count selected sections as controls.
Test both A_5 signs using all six extension counts. Record divisibility
candidates, not smooth carriers, until strict-transform smoothness is checked.
This omits the two del Pezzo twist types outside these four-point models
and the unbalanced trigonal genus-6 family; track these explicitly.

## Amendment 3: nonsplit Klein fibers

The all-scalar F_8 run exposes unsupported fibers at three of seven source
points. Preserve that partial output. Perform the same divisor calculation
over F_(8^3)=F_512, where those cubic fibers split; retain the source
order-7 subgroup over F_8, all 56 quotient maps, all seven scalars, and
Frobenius traces to F_2. Generate split denominator lines as needed and
verify every interpolation certificate. Report field-extension work and
the original incomplete coverage explicitly. This is a correctness/control
experiment, with no timing comparison.

## Amendment 4: remaining trigonal scroll and gluing modules

LC1b enumerates all 2^20-1 nonzero sections of 3C_0+7F on the split
Hirzebruch surface F_2, where C_0^2=-2. Coefficients of fiber degree i
have base degree 7-2i, i=0..3. Adjunction gives arithmetic genus 6.
Count both base charts, both fiber charts, and all six extension degrees;
record exact target divisibility and direct-count controls. Smoothness of
any candidate remains a separate check. This complements the balanced
trigonal family rather than repeating it.

GL1 solves Frobenius intertwiner equations at each prime in GL0 between
the two-dimensional elliptic companion module and the eight-dimensional
A_5 module. Enumerate the finite Hom space, check ranks of candidate maps,
and record full elliptic torsion field degrees. These are torsion-module
candidates, not algebraic polarization or curve constructions.

## Amendment 5: direct modular-curve line relations on the Klein control

MR2 generates principal-divisor relations from every fully split F_8 line
on the Klein model of X(7), traces them to F_2, and expresses them on the
closed-point factor base. Compute rank modulo 7 and, if there is a unique
scale freedom, anchor the rational flex divisor and solve the factor-base
logs. Check all solved classes by separate Riemann--Roch interpolation.
Use the verified nonzero transfer map to recover the seven known-answer
source fixtures using point/divisor inputs, retaining failed descents.
Do not add rows containing fixture logarithms or the previously solved
principal-divisor/class certificates to the relation matrix. This tests
an actual geometric source of cross-orbit relations at n=3; no transfer
of its coverage to n=5 or n=131 is assumed.

## Amendment 6: relation-based target recovery and extended factor base

Preserve MR2's rank output, then add target recovery to a new output. The
solver receives an elliptic point, evaluates pullback and Frobenius trace,
and uses only line-derived factor-base logs. Select the first map whose
base image has nonzero solved logarithm. Unsupported F_8 fibers remain
failed descents, not guessed logarithms. As MR2b enumerate all F_512 lines,
retain fully split intersections, norm their principal divisors through
nine Frobenius powers, and solve on all closed-point orbits supported over
F_512. Use this enlarged factor base for all seven source fixtures. Keep
its larger field and factor base explicit. No previously found target class
or fixture scalar enters the relation rows or the solver. Compare each
recovered scalar by elliptic multiplication after solving.

## Verification amendment: removable values and independent replay

Before final replay, add an independent comparison between the removable
limits of LC0b's combined rational expression and its sum of trace terms at
F_8 points. Repeat the frozen bounded runs into a new directory and compare
bytes. Verify line relation rows from their geometric sources and recompute
rank using the separate BigInt elimination routine, with explicit class
certificate checking. Add a degree-eight integer-lift identity certificate;
its odd-prime schema and the Lean characteristic-two theorem have distinct
scopes and are linked by the even residual coefficients.
