# Round09 frozen protocol

2026-10-19 UTC. Parent92688745f0ffadf854367ad1d2f33fa05bf7ce2a,
the main merge after round08. All rounds01–08 remain frozen. Native Rust
performs arithmetic, search, record assembly and checking. No timing or
IC efficiency assessment. Preserve unsuccessful attempts and exact gaps.

## Q7 — Frobenius on the intrinsic Kummer coordinates

Use the four frozen graph kernels and six-chart atlases over pinned
K=F2^360. The quotient chart's base is the dual product: the ordinary
augmentation descent D must be replaced by B=D^(-transpose), since the
product principal polarization used to identify the split dual is not
invariant under D. Verify the matrices, actual torsion classes and actions.

For graph generators in the recorded order(Frob S,S), verify
M=[[2t,1],[-2,0]] and signed inverse N=[[0,9],[1,t]] modulo19, t=±1.
The proposed root action has columns N: z'_0=sigma(z_1)*H_0 and
z'_1=sigma(z_0)^9*sigma(z_1)^t*H_1, covering B*sigma on the base.
Do not substitute the original D action into the dual base.

Compute all univariate Miller corrections explicitly. Use products of
fixed-point line quotients for integer combinations of the two sigma
graph generators, checking their19th-power identities exactly. Derive
the mixed addition correction h_T(X,Y) from the normalized Miller line
quotient, with a written divisor/normalization proof and symmetry check.
Store the full Frobenius action as factored rational circuits, not an
uncomputed symbol H. Reduced polynomial degree stays≤256.

Check two sets of five one-variable slices per graph/root, giving80 exact
rational identities. Choose constants by deterministic searches at
abscissas<4096; use19-divisible generic points avoiding poles and all five
base-action iterates. At most128 candidate tuples per graph. Check two
K-rational fixtures per graph through360 semilinear iterations; determine
the deck cocycle explicitly. Do not call a sample alone a global proof:
the written argument must identify any base-trivial iterate with a
constant deck action and show how the fixture determines it.

Express the action on every source/target chart using the recorded rho
transitions, and check compatibility on all36 chart pairs for both roots
and both fixtures. Bind all200 old transition factors to coefficient
Frobenius and verify their evaluation compatibility. Effective invariant
F2 polynomial equations and a closed projective embedding remain open
unless actually generated and verified.

## L7 — bounded lower-genus quotient diagnostic

Test the five diagonal twists g^u of the n=5 carrier and its five reflection
subgroups g^j*w. Work in the exact four-dimensional augmentation module
and its tensor with the elliptic Frobenius companion, for both traces±1.
Verify the deck/Frobenius conjugation and every commutator. Compute exact
characteristic polynomials for the known trace-zero component, before and
after each twist. Distinguish a Frobenius-stable reflection quotient from
one preserving that known component. No universal nonexistence or complete
Jacobian-factor exclusion follows from this bounded representation test.

## M12 — coupled-function restrictions

Evaluate the following F2 functions on every known norm-family column:
x0+xinf+c, y0+yinf+c, x0*xinf+c, x0*yinf+xinf*y0+c, c=0,1.
Use actual modular target coordinates in split/degree11 point algebras,
after each exact deck change. Test identity in the coordinate field;
retain zero counts, signs and geometric degrees. These are restrictions
to known support, not complete principal divisors. Do not promote a new
relation rank without the missing outside zeros, multiplicities and poles.

## Proofs, validation and publication

Add feasible Lean proofs for dual actions, normalized Kummer identities,
descent/cocycle implications and bounded quotient controls, without custom
axioms/native trust. Source-bind identities with64 issuer and128 extra
checks. Fresh-replay new records and round08; run required native,root,
Lean,site/workflow/dashboard checks. Preserve an immutable archive,
illustrated editable report/PDF, exact requirement ledger and failures.
Update the existing PR and canonical narratives. The main conflict is
resolved locally; review exact pushed-head CI before any merge.

## Amendment A — explicit invariant rational generators

Q7 passes the signed inverse action,80 exact slices and two360-step
cocycles per graph. Construct the trace-dual power basis of K/F2 by exact
360×360 binary linear algebra. Verify all129600 trace-duality entries,
all360 separability-idempotent sums and64 direct field-trace controls.

For each of the twelve chart coordinate functions f and360 basis
elements b_k, record the invariant rational circuit

    I_f,k = sum_(j=0..359) rho^j(b_k*f).

Here rho is the semilinear function action induced by the explicit point
Frobenius alpha: rho^j(f)(P)=sigma^j(f(alpha^(-j)P)). Supply the dual-basis
recovery identity f=sum_k d_k I_f,k. These4320 rational functions may be
redundant; do not claim polynomial relations or a minimal affine model.

Evaluate all360 trace weights for x0,y0,z0,z1 at one fixture per graph,
5760 values total, using1440 additional point-Frobenius steps. Check all16
coordinate recoveries and128 equivariance controls (eight weights for
each sampled coordinate). The universal idempotent identity supplies
the formal recovery argument for all twelve functions. Effective F2
polynomial equations, saturation and a closed embedding remain open.
