# Cached subspace scalars and the three-coordinate cubic Frobenius kernel

UNEXECUTED next hypothesis. No frozen source, fixture, manifest or raw result is
changed. These are classical quotient-ring/subspace-polynomial identities, not
a literature novelty claim. The proposed gain is reduced paid constant cost in
a conditional norm branch; no generic rootfinding complexity claim follows.

## Exact cubic squaring, including repeated roots

Let Q=X³+bX²+TX+U be monic over K of characteristic2. In K[X]/Q,

    X³=bX²+TX+U,
    X4=(b²+T)X²+(bT+U)X+bU.

Consequently the square of p=p0+p1X+p2X² has canonical coordinates

    S_Q(p)0=p0²+(bU)*p2²,
    S_Q(p)1=(bT+U)*p2²,
    S_Q(p)2=p1²+(b²+T)*p2².

This follows by expansion and monic remainder uniqueness. It assumes neither
squarefreeness nor separability. Precompute the three Q-specific multipliers
bU, bT+U, b²+T once per cubic, paying their cost. Each S_Q then uses three
field squares and three field multiplications. Do not count formal derivative
zeros as absent dependencies; S_Q is Frobenius-semilinear overK and linear
only overF2. Two applications compute p4 modQ exactly.

## Cache only data that is independent of the candidate cubic

Let v0,...,v(d-1) be the fixed independent source basis and Vj its firstj span.
Write Lj(X)=product_{v in Vj}(X+v)=sum_i c(j,i) X^(2^i), with L0=X.
The scalars lambda_j=Lj(vj) depend ONLY on the field representation and the
ordered source basis. Independence proves every lambda_j!=0. Exactly

    L(j+1)=Lj²+lambda_j*Lj,
    c(j+1,i)=c(j,i-1)²+lambda_j*c(j,i),

where absent coefficients are zero. Compute all d scalars once per declared
workload/arm using the sparse linearized coefficients, then retain the scalar
list. This costs O(d²) field operations and O(d) working field elements; its
cost must be paid before reporting any arm total. Reuse across cubics is exact
because neither h, ordinate, k nor Q appears in lambda_j.

For each candidate cubic initialize R0=X, represented by(0,1,0), and perform

    R(j+1)=S_Q(Rj)+lambda_j*Rj.

Induction and quotient-map homomorphism prove Rj=Lj modQ. The final certificate
is R=L_V modQ and S_Q(S_Q(R))=R4 modQ. Its equality to the generic polynomial
remainder calculation must be independently checked for EVERY emitted bounded
candidate, including rejected and regular-excluded candidates. Supported iff
R4=0: L_V is squarefree with precisely the rootsV, and exponent4 covers every
possible primary multiplicity of a cubic. Nilpotence detects repeated roots;
squarefree shortcuts cannot replace it without a separate guard/proof.
Recover all multiplicities/orderings with the existing complete factor contract.
Factor caps or candidate caps preserve UNKNOWN, never an empty global root set.

## Falsifiable measurement that identifies the source of any gain

Create new source and a new independent derivation/code review, then freeze
all hashes, inputs and budgets before execution. Three comparator arms share
the same exact family and outputs:

1. Current generic support code, including its per-candidate lambda generation.
2. Generic polynomial remainder/squaring with one paid cached lambda list.
3. Explicit three-coordinate S_Q and one paid cached lambda list.

The middle arm separates caching from the cubic-kernel gain. Use the strong
specialized comparator as well as the old code; a component ratio against old
code is not superiority over F4/F5/F6. Pay field parsing/setup, source cache,
formulation, exact Gaussian family, every candidate support residue, source
recovery, verification, output serialization and worker overhead. Record stage
costs, total wall and peak processRSS. Three alternating SAME-PROCESS reps may
calibrate costs but are not cold/isolated qualified release timings. Independent
post-run review must replay every k,Q,R,R4 against direct generic polynomial
arithmetic and reconstruct every supported multiset/ordering.

First compare the SAME current actual h=0x4208336d0d441df79c400,a=1,ya=0
fixture and its complete coefficient-relaxation family. The previous worker
observed rank79/nullity4 and16 k; the new worker must rederive/replay that family
rather than assume a universal16 bound. If measured nullity>12, preserve the
family as UNKNOWN instead of enumerating. Retain all bounded k/certificates.

Then use a PREREGISTERED TRAINING ladder of32 h values times BOTH rational
ordinates ya=0,1 at a=1 on the same actual A0/B1 curve. Define h order before
any timing/support observations: solve high-coefficient equations
(h²+h+z) modV=0 by frozen deterministic F2Gaussian pivots, form its affine
particular/kernel with free columns in increasing order, and take the first32
free-coordinate binary indices0..31. This is canonical Gaussian enumeration
order, NOT an unstated numerical sort or postselected first positive cells.
Verify the full h family (actual dimension18 follows from the trace image and
V containing1, whose absolute trace is1 in degree83). Include all64 registered
conditional cells in every arm, even empty or slower ones. No choosing source
ordinates, h values or roots after cost observations. The initially exposed
current cell and this ladder are calibration/training, not an unopened holdout.

Declare a full64-cell budget of90s/4GiB/1core and caps<=4096 candidates percell;
preflight before materialization, preserve cell-indexed UNKNOWN on exhaustion,
and do not turn partially completed ladder timings into complete speed ratios.
Use at most three experiments: exhaustive small controls, fixed-cell comparison,
then the dependent ladder. Small controls include GF4/nonprefix, GF8, trace-mixed
GF16, zero/repeated-root cubics, actual field scalar arithmetic, and independent
multiset support/order oracles. Report per-field positive/degenerate coverage;
any unobserved branch remains a gap.

## What can refute this optimization, and what remains hard

A missing scalar identity, R/R4 mismatch or incomplete output refutes that
implementation; a new corrected version must identify the changed assumption.
If cache preparation or verification swallows the total gain, preserve that
negative timing and test a new mechanism rather than increasing the same run's
timeout. If the current Ore profile fails, distinguish scalar-formula errors,
fixture admission, budget/cap and output-accounting failures before using any
of its claimed candidate counts in this experiment. Repair the exact cause in
a new variant with independent oracles; never retry identical failed code.

Even a fast64-cell result does NOT enumerate the outer charts. The actual
admitted h family has2^18 members, and a ranges over roughly2^18 source values
with rational-lift/exceptional branches. Their product is approximately2^36
conditional charts before other obligations;64 cells do not remove this cost.
A global advancement needs whole-cell rejection, shared outer elimination or a
new complete chart reduction. Common-factor/zero-a branches, original auxiliary
roots, odd fields, all requested prime curves and genuine matching F4/F5/F6
baseline comparisons remain separate obligations. The user8x goal stays open.
