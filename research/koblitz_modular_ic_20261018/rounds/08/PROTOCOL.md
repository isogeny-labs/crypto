# Round08 frozen search protocol

2026-10-19 UTC. Parent244f1b850ce0d7bc9070b8b07d35e0e0ae3c5023.
Preserve rounds01–07. Native Rust performs all arithmetic, searches,
record assembly and checking. Retain unsuccessful attempts. No timing
or IC efficiency assessment is planned.

## M10 — remaining nonzero-line norm functions

Test Norm_F1024/F2(x(phi_a)+x(P0)) for a=1,2,3,4,5 using the frozen
positive and negative seeds and independently reconstructed target traces.
Use a=1 as exact round07 control; a and−a are expected to be the same
norm because Frobenius5 exchanges the two lines and fixes P0.
Each complete ordinary zero divisor has degree13200, with cusp pole
images zero as in round07. Use the exact determinant1 basis change
(P,Q)→(−aQ,a^−1 P+Q), normalized Frobenius and Frobenius10 cycles.

Join split columns by complete geometric basis keys. For degree11
components, change primitive generator to x(Pnew), compute its minimal
polynomial and express y(Pnew),x(Qnew),y(Qnew) in that generator by exact
11×11 field linear algebra. Check the change and inverse by substitution.
Use these field-presentation-independent coordinate keys, with simultaneous
basis sign removed, to join components and their ten Frobenius phases.
Bounds: five line functions;660 split points and60 degree11 components
per function; ten coefficient Frobenius phases; linear systems size≤11.
Do not join on equal projected images. Retain all multiplicities and
report rank on all columns and separately on nonzero-image populations.
Independently check new images in the small F32 arithmetic and ranks in
a separate modular elimination path. Unsupported joins remain partial.

## Q6 — intrinsic determinant19 Kummer charts

For each of the four frozen actual graph realizations Γ⊂E^4×E', select
the two recorded generators (indices1 and19). For each of their five
elliptic components T, construct the normalized Miller function f_T with
div(f_T)=19([T]−[O]); f_O=1. Store explicit coefficient arrays in the
form (A(x)+B(x)y)/D(x), reduced over pinned F2^360. Bound each reduced
polynomial degree by256; use the fixed binary19 Miller chain.

Check every chain point, normalized pole order19, denominator factor
support, and local valuations at all18 nonzero subgroup points through
order64. Compare function values at16 nonsupport points with a separate
pointwise Miller evaluation (search abscissas<4096). Preserve failures.

Form the two factored polynomial equations

    z_i^19 product_j D_ij(x_j) = product_j (A_ij(x_j)+B_ij(x_j)y_j).

Together with the five elliptic equations and localization at all displayed
numerator/denominator factors, these are proposed intrinsic dense charts.
Verify the two graph classes are independent and record the diagonal
derivative/Jacobian-unit argument. Explain the dual-isogeny/Kummer bridge:
diag(H,[19])=q^dual λ_A q, so the dual of the map from A to the product
has kernelΓ. Check the actual finite-kernel and polarization hypotheses;
do not equate an unverified radical cover with the original quotient.
Full projective chart gluing, explicit F2 descent and polarization on the
new coordinate model remain open unless constructed and checked.

## Proofs and evidence

Add feasible Lean results for generator changes, rank support, Kummer
identities and units, keeping global geometry explicit. Issue source-bound
identity certificates with64 issuer and128 extra checks. Fresh-replay new
records and round07, run the required native/root/site/workflow/Lean checks,
seal raw records and deliver an illustrated report/PDF and updated PR.
Copilot's pending base-merge request is separate; do not edit conflicting
canonical files while its resolution is pending.

## Amendment A — complete the fixed-line family

M10 generates rank11/1248 and nonzero-image rank10 on1115/1110 columns;
separate checks are still required. Add the remaining fixed line∞ function
Norm_F32/F2(x(phi_infinity)+x(P0)), degree6600. Use the determinant1 swap
(P,Q)→(−Q,P), the original Frobenius5 cycles and five phases. Join split
columns against every M8/M10 geometric orbit. For degree11 components,
Qnew is constant and Pnew nonconstant, distinguishing it from M8 (constant
P) and M10 (nonconstant Q); check those rationality claims explicitly.
Retain this as a separate M11 output and row. The bound is one additional
function on the same720 seed components, with no new field search.

## Amendment B — explicit translated Kummer chart gluing

Q6 passes40 functions,720 valuations and640 pointwise checks. Attempt six
diagonal translated charts k=0..5 for each frozen graph. On each factor
choose S among abscissas<4096 and put R=19S, requiring19*d*R≠O for
1≤d≤5. Then the translated bad sets {O,T_1,T_2}−kR are pairwise disjoint
on that factor; any point of the fivefold fails at most five charts.

For each nonzero kR and each recorded T, construct

    h_T,R(P)=g_(T−R,R)(P)/(x(P)+x(R)).

Check the exact rational identity

    f_T(P+R)=f_T(R)*h_T,R(P)^19*f_T(P).

Generic function arithmetic remains (A+By)/D with reduced degree≤256.
Use the conjugate/norm formula to invert generic functions and exact
elliptic translation to compose them. Since R is19-divisible, test that
f_T(R) is a19th power in K, then compute and verify its root. This yields
explicit transitions between the six Kummer charts. Bound: four graphs,
five factors,two generators,five nonzero translations,200 exact identities.
Record an unsupported reduction or failed root as partial. Establish the
overlap cocycle by the recorded root ratios and exact coordinate changes;
do not claim global gluing from value samples alone. F2 coordinate descent
remains a separate obligation.
