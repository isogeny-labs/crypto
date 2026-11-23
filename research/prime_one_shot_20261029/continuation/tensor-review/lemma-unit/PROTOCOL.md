# Prospective exact-arithmetic Hermite lemma development check

Frozen 2026-10-30 before implementation or execution. This is a fixed native
C++17/GMP rational unit check of AppendixD.4.3-D.4.4, LemmasD.4-D.5, in
/workspace/audit/tensor-primary/openai-oct25.{pdf,txt}, printedpp59-61.
It is not a contraction implementation, asymptotic experiment, runtime
comparison, theorem certificate or support for a cryptanalytic claim.

## Fixed panel and representation

For every a=1,2,3, every u=1..a, and M in {u,u+1}, use L=2^a,
S={-L,...,L-1}, alpha=2^(a+1), h=ceil(3M/u),
q_t=h*max(0,u-v2(t)) for nonzero t, and q_0=0.
Use output precisions p in {1,3,5} and two polynomial families below.
That is12 parameter tuples,36 parameter/precision tuples and72 polynomial
cases. All arithmetic and research calculations are native exact GMP mpq/mpz.
No Python research computation. A coordinate vector has four rational entries,
representing the fixed basis1,U,V,UV in
Q[U]/(U^2+U+1)[V]/(V^2). Reconstruction weights are rational scalars, so
all operations act coordinatewise. This checks coordinatewise reconstruction,
not arbitrary extension-ring multiplication or general contraction semantics.

Family1: four explicit rational polynomials of degree D=M(2L-1), with
coefficient at T^j in coordinate c=0..3 equal to
(-1)^(j+c)*(j+1)*(c+1)/(2*(j+c)+1). Their charts have odd denominators.
Their known F(0) entries are(-1)^c*(c+1)/(2c+1).

Family2: form actual Cauchy kernels
W_delta(T)=prod_{q in S,q!=delta}(T-q)/C'(0),
C'(0)=prod_{q in S,q!=0}(-q).
Use coordinates W_0^M; W_1*W_0^(M-1); (-3/5)*W_0^M;
7/3+(1/3)*W_(-1)^M. Known F(0) is(1,0,-3/5,7/3).
These rational polynomials have locally integral charts even where their
ordinary coefficients need not be2-integral.

## Required hypotheses and independent checks

Before reconstruction, require exact degree<=D, F(0) equal to the stated
expected vector with odd coordinate denominators, and every coefficient of
F(t+alpha*z)2-integral for every t in S(including0).
Separately check every actual kernel delta in {-(L-1),...,L-1} on every t in S:
W_delta(0)=1 if delta0 else0; each chart is2-integral; for t!=delta all
chart coefficients are divisible by2. Check v2(C'(t))=v2(C'(0)) on the grid.

Construct E_t from the source product, its chart series and its inverse by
exact rational series arithmetic, not existing Rust code. Verify E_t chart
constant1, integral inverse coefficients, and series product1 modz^q_t.
Compare v2(E_t(0)) from the direct rational product with the source identity
D_t*sum_{j=1}^{a-i}(j/2^j)*(q(i+j)-q(i)), including the factorj.
Require v2(E_t(0))>=-6D_t*h and sum q_t>D.

## Reconstruction/lift variants

For each case apply(D.9) with four fixed variants:
1.Exact input jets: reconstructed vector must equal F(0) as a rational identity.
2.Canonical input lifts: reduce every used input-jet coefficient modulo2^p_t
  to the integer in[0,2^p_t), then divide by E_t using its integral inverse.
3.Perturbed input lifts: add2^p_t times the deterministic odd-over-odd rational
  (-1)^(case+t+j+c)*(2*(j+c)+1)/(2*(c+1)+1) to every canonical input coefficient.
4.Perturbed quotient lifts: compute exact quotient series, canonically reduce
  each coefficient modulo2^p_t, then add the same deterministic2^p_t perturbation.

Here p_t=p+8hD_t+8M(a+1). Retain all node, coordinate and final errors.
For each non-exact variant, every node contribution error and final coordinate
error must have v2>=p(zero is+infinity). Require the final sum2-integral and
its residue modulo2^p equal to the independently supplied known F(0) residue.
No omitted failures or tuned parameters. Counters describe arithmetic checks,
not calibrated work, timing or an exponent fit.

## Fixed negative controls

Reject the hypothesis of F(T)=1/2 (nonintegral F(0)/charts), F(T)=T/2
(integral F(0), nonintegral odd charts), and a degree D+1 monomial.
For a=u=M=p=1, change one quotient coefficient at node t=-1,j=2 by1,
rather than a2^p_t multiple. Its reconstruction error must be1/128 withv2=-7,
so the requested precision/integrality check must reject it. This intentionally
violates the lift hypothesis; it tests detection, not the lemma.

## Build, records and scope

Strict build:-std=c++17-O2-Wall-Wextra-Wpedantic-Werror-lgmpxx-lgmp.
Keep source, protocol digest, compiler identity, exact command, build attempts,
all exits, fixed raw TSV records, terminal summary and manifest. Native binaries
are outside the public-copy payload. Repeat once and require byte-identical
scientific output. Do not use elapsed times as results. A failed build is
preserved and corrected explicitly; no scope expansion to an attack or tensor
contraction algorithm is authorized by this protocol.
