# Round 03 protocol — remaining twists, polarized gluing, n=5 relations

2026-10-18. Frozen before execution. Parent head:
`d6c722e19a381f2b6178b6d5d75ce6a87aea4335`.
User authorizes continued experiments on all three tracks and relevant Lean
proofs, with methodical results/failure tracking. Prior evidence is unchanged.
All research arithmetic and harnesses are native Rust. Python is layout only.

## DP3 — the two remaining degree-five del Pezzo twists

Use the five-point conic construction: choose the first squarefree degree-five
F2 polynomial with factor degrees (5), respectively (3,2), put its five roots
at (1:t:t^2) on xz+y^2=0, blow up these five points and contract that conic.
The anticanonical system pulls back to quintics double at all five points;
its square pulls back to decics with multiplicity at least four there.
Compute the F2 nullspace, require dimensions 6 and16, and enumerate all
65,535 nonzero sections of -2K for each twist. Count on the blowup using
order-four Hasse jets for exceptional fibers, then subtract q if the section
contains the contracted conic. Count through k=6. Walsh counts must agree
with direct counts for masks 0,1,31,137,12345,65535 in every field. Verify
surface counts against q^2 + q*(fixed points of the five-point permutation)
+1, and compare conic-contained counts independently.

Test both A5 signs. A polynomial hit is a candidate until strict-transform
smoothness and genus have been verified. Count hits through k=8 and retain
their full equations and remaining smoothness obligations. Existing blowup
models serve as dimension/jet controls. No universal nonexistence conclusion
is attached to a bounded family enumeration.

## GL2 — invariant integral polarizations and finite gluing graphs

Work on the actual augmentation lattice of E^5, with O=Z[w], w^2+w+2=0,
and basis e_i-e_4 (i=0..3). A cyclic Hermitian form is obtained by restricting
a five-by-five circulant with first row (0,u,v,bar(v),bar(u)). Enumerate
u=a+b*w and v=c+d*w for all four coefficients in [-3,3] (2401 forms).
Use exact leading principal minors to test positive definiteness; retain
forms with determinant 1,5,11,19. Check invariance under the order-five
augmentation action, the integral alternating Riemann form and Frobenius
similitude F^T J F=2J, for both Koblitz signs.

For prime determinant ell, compute the two-dimensional polarization kernel,
its pairing using integer lifts and J/ell, and its actual Frobenius action.
For all five elliptic cofactor traces, enumerate intertwining maps and retain
exact anti-isometries. Verify graph isotropy, half-dimension and trivial
intersection with each factor. Keep lattice evidence, geometric descent,
principal polarization and Jacobian realization as separate obligations.
No quotient is called a Jacobian without a curve or the required criterion.

## MR3 — determinant-compatible modular character screen at n=5

Add the nebentypus/determinant condition to round02's necessary Eisenstein
screen. After dividing by the twist character, u^2=-p at inert primes and
u^D=1 imply (-p)^(D/2)=1 when D is even. Test n=3,5,131, the same five
inert good primes, exact residues and Bezout witnesses. This is still a
character-pair screen, not a complete theorem about every modular relation.
Prove the sharpened algebraic implication in Lean.

## N5 — executable map and geometric relation controls

Evaluate the n=5 order-11 subgroup in the augmentation model, for both signs,
and verify all eleven inputs, Frobenius-fixedness and exact order. Test any
GL2 graph quotient against this embedding, including ell=11 kernel issues.
Use an explicit degree-three binary genus-three cover as an additional
same-field map/relation control over F32, clearly distinguished from an F2
carrier or an evaluated modular-Jacobian map. Implement principal-function
relations and individual descent on that concrete control; freeze its
collection bound before running. Keep actual higher modular correspondences
open if no evaluator has been obtained.

## Formalization, failures and delivery

Record each run with write-once output, parameters, source/input hashes,
controls, outcome, failed attempts and next experiment. Add faithful Lean
structural implications and identify unformalized geometric bridges. Ship
a report, proof map, diagram, inspected PDF and deterministic raw archive.
Run the native tests, exact replay, Lean kernel/axiom checks and required
repository checks before updating the existing PR. Efficiency assessment
remains with the user; this round specifies no timing experiment.

## GL2 amendment — integral congruence cosets

The initial 2401-form scan found five positive determinant-five forms and no
forms of determinant1,11,19 in its specified circulant sublattice. This does
not exclude other integral invariant forms. Before the next run, extend the
circulant parameters by the five congruence cosets r=0..4:
(a,b,c,d)=(A+r/5,B+2r/5,C+2r/5,D+4r/5), each capital coefficient in [-3,3].
Their restricted Hermitian matrices are integral: verify all numerator
entries are divisible by5. Scan all 12,005 forms under the same positivity,
determinant, kernel and anti-isometry checks, writing GL2b separately. Retain
GL2's output and describe its sublattice limitation explicitly.

## N5 collection bound — before native execution

For each sign use C_a/F32: v^2+u^2*v=u^7+a*u^4+u with its explicit
cubic elliptic quotient. Enumerate every one of the 32^4 polynomials v(u)
of degree at most3. A relation is admitted only when the degree-seven norm
v(u)^2+u^2*v(u)+f(u) splits completely over F32; record root multiplicities
and the function coefficients. Verify admitted relations with the existing
Mumford group implementation. Compute exact rank modulo11 on the entire
rational-point factor base. Proceed only with a unique nonzero scale freedom,
and verify every solved projected factor-base class using independent
library arithmetic. This collector is geometric on the same-field cover;
it is not relabeled as a modular-Jacobian evaluator.

For each of all eleven source points, attempt individual descent first
without an offset, then with the first eight factor-base point offsets.
For each reduced degree-d Mumford divisor use v=V+c*U, all c in F32 (32
trials), and accept only a fully split residual norm polynomial. Preserve
failed attempts and successful witnesses. Fixture scalars enter only final
multiplication checks. Record augmentation-model and cubic-pullback maps,
exceptional points, group compatibility, subgroup order and field explicitly.

## GL3 amendment — published genus-five census lookup

The expanded integral scan found determinant19 forms admitting elliptic
anti-isometries, in addition to determinant5 controls. To test curve
realization, pin the current commit of Dragutinovic's published MT_Curves
census, retain its raw point-count tables, equation files and README files,
and test every listed genus-five point-count vector for divisibility by A5
of either sign. This is a native Rust data scan, with no execution of its
Sage/Python programs. Retain all matches, equations and their original
indices. Independently count any extracted curve before promoting it.
A negative census lookup is conditional on its published completeness and
correctness; it is not an independently proved classification.

## DP4 amendment — published genus-six census cross-check

Before executing the lookup, pin the Huang--Kedlaya--Lau genus-six census
commit and its lmfdbgenus6_curves_F2.txt export (approximately14MB). Inspect
its declared format, then scan every exported curve polynomial for both A5
signs using native Rust. This cross-check includes strata not covered by the
seven del Pezzo models. Preserve matches and input bytes; independently
recount any selected carrier before claiming construction. Completeness is
attributed to the published census and is not formalized by the lookup.

## N5b amendment — vertical principal functions

The frozen y+v collector recovered all11 a=0 fixtures, but its ten a=1
relations have rank10 on21 columns and cannot proceed. Preserve that result.
Add all principal divisors of u-c for c in F32: each ordinary rational
fiber gives P+involution(P)-2*infinity; the branch fiber gives2P-2*infinity.
Verify each row by Mumford arithmetic, combine with the original geometric
rows and rerun rank and the same bounded descent. No target class or fixture
log is added as a relation. Write N5b separately and retain any failures.

## N5c amendment — quartic principal functions

N5b's vertical rows raise the a=1 rank to16 on21 columns, leaving five
scale freedoms. Preserve this failure. Add the 32^4 functions y+v(u) with
v monic of degree4, in addition to the original 32^4 degree-at-most3
functions and all vertical fibers. Their norms have degree8 and their
principal divisors have pole order8. Verify every admitted completely split
row by Mumford addition, then use the same rank and descent gates. This is
an additional bounded geometric source of rows, not supplied logarithms.
Write N5c separately; no further collector is silently assumed to succeed.

## N5d amendment — complete quartic split-relation coverage by interpolation

N5c's monic-quartic subfamily adds no a=1 relations; rank remains16/21.
Preserve it. For a=1 only, enumerate every effective degree-five divisor on
the21 rational affine points, in nondecreasing point-index order. Exclude
simultaneous conjugate points at one nonbranch abscissa, and repeated support
at the branch point u=0. Use exact Hasse jets to interpolate the unique
polynomial v of degree at most4, deduplicate coefficients, and test complete
splitting of its norm. This covers every completely split y+v function of
degree at most4: its degree7 or8 zero divisor contains such a degree-five
subdivisor. Check that every previously admitted y+v relation is present.
Keep the a=0 control from the prior bounded families, use the same descent
limits, and retain new rank or descent failures. Record interpolated divisors,
unique polynomials, accepted rows and exact bounds; no known logs enter rows.

## GL4 amendment — short-vector decomposition control

For each GL2b determinant19 form that admits a graph gluing, enumerate every
nonzero vector in O^4 with each of its eight integer coordinates in [-1,1].
Evaluate the exact Hermitian norm and retain the minimum and any norm-one
witness. A norm-one witness splits a principally polarized elliptic factor
from the ordinary factor; test the corresponding integral retraction and
its annihilation of the polarization kernel modulo19. A bounded failure to
find such a vector is not an indecomposability proof. This checks whether
the new graphs are already excluded by a concrete decomposition witness.

## G4 amendment — close the dimension-four carrier rung explicitly

Before making any combined small-genus conclusion, compute the point counts
of A5 as a putative genus-four Jacobian. For the positive sign test monotonicity
N2>=N1 from F2->F4. For the negative sign compare N5 with the published
maximum71 among F2-defined genus-four curves in Xarles, arXiv:2007.07822,
Example16 (PDF page10). Preserve that bound as an externally supplied census
result, not an independently reconstructed classification. Add conditional
Lean count-obstruction lemmas with the injection/bound hypotheses exposed.
The genus5/6 lookups retain their explicit published-completeness assumptions.
