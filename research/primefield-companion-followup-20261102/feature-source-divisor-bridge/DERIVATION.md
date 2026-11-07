# Exact source lift: moving multipliers, not a fixed kernel

T-1785, 2026-11-07. Producer derivation and native software checks, unsigned and
not independently verified. [pcc:unverified]

## The source object actually evaluated

The source grid takes a query Q, not a fixed kernel polynomial. Put N=(r+1)/2,
h=(r-1)/2=2wk+2s+e. Its baby multipliers are N+j (0<=j<w); center multipliers
are N+w+2wi (0<=i<k); the tail x-coordinate uses s+e. The implementation reduces
point multipliers modulo r. Fix those exact positive representatives for a
rational-function lift in the variable Q. Changing a representative by r or
negating it preserves appropriate coordinates on H, but does not generally
preserve the rational function on E.

For a,b positive and distinct, the standard division-polynomial identity is

 x([a]Q)-x([b]Q) = -psi_(a+b)(Q) psi_(a-b)(Q)/(psi_a(Q)^2 psi_b(Q)^2).

Here psi_-n=-psi_n. The negative sign must be retained; we use psi_2=2y.
Write D_n=[n]^*[O]. Divisors satisfy

 div(psi_n)=D_n-n²[O],
 div(x∘[a]-x∘[b])=D_(a+b)+D_|a-b|-2D_a-2D_b.          (1)

Pullbacks count multiplicity; they are not assumed reduced when p divides n.
Equation (1) concerns functions defined over the original prime field. Discussing
geometric support is a proof device, not an extension-field attack or computation.

## Compiler following the source recurrence

bridge.rs builds a signed integer exponent map for division polynomials.
Each coordinate difference uses the displayed identity. Products add maps;
quotients subtract; powers scale. It follows the source's initial previous=-1,
current=1, each baby update delta*current²/previous, the boundary negation,
and each center update. It then forms exactly

 G=(-1)^(wk+s+e) R_main R_tail A^(2w) B^(2k) C².

This is the unprojected geometry-only residual with seed inputs all1. Its
signed exponent map defines an exact rational function where the expressions
are defined, not merely an equality of divisors. Zero differences remain outside
the evaluator's domain. The compiler is a bounded audit implementation with
odd7<=r<=1000001; construction explicitly visits wk pairs, so is not a fast
large-order source compiler.

For p-1=q*m, m odd, q=2 or4, add the existing monomial's raw division factors
psi_N^alpha psi_(2w)^beta psi_(s+e)^gamma and reduce exponents modulo q.
The existing seed-cancellation derivation proves the omitted combined seed
power is divisible by4. The negative sign survives projection since m is odd.
Evaluation of this new expression and then the m-th power agrees with the
projected old source in the native tests. The reference is the old full-value
source, including its phase seed; the compiler does not assume kernel c_T
is that phase seed.

## An exact order-seven witness

For r7, w=k=e=1,s=0,N4, the geometry is

 G=(x([5]Q)-x([4]Q)) (x(Q)-x([4]Q))²
  = -psi_9 psi_3² / psi_4^6.

The compiler retains the harmless psi_1^-3 before simplification. Thus

 div(G)=D_9+2D_3-6D_4-3D_1.

For q4 the projected monomial adds psi_4^5, yielding the rational representative

 J=-psi_9 psi_3² / psi_4,
 pi(J)=pi(-psi_9 psi_3² psi_4³).                       (2)

On E/F5:y²=x³+2x+1, H has order7. A geometric point T of exact order9 exists
since9 is prime to5. At T, ord_T(J)=1: only D9 contributes. Hence J cannot
be a constant times a rational fourth power times a product of functions whose
divisors are supported on H. In particular the prior normalized subgroup
Miller functions and their H-translations cannot supply such a direct rational
replacement. This is a precise obstruction for this lift and factor family.

It does NOT prohibit a formula agreeing on the finite subgroup. The points of
order9 used in the proof are outside H. Replacing a multiplier by its residue
modulo r changes the ambient divisor while preserving values on H; finite-set
rewrites need a separate proof. Nor does this rule out Miller functions with
other support, pullbacks of Miller functions, additive circuits or implicit norms.

## Native checks and artifacts

validation-02: three selected methods pass;29 imported methods are filtered out.
Build and tests have90s/30s caps; separate audit build/output also90s/30s.
The13 transitive Rust/fixture inputs are hashed before and after execution.
Compiler, binary hashes, commands, exit codes and logs are in receipt.json.

* Exact r7 signed map and modulo4 expression (2).
* Projected source values: all6 nonzero points for E/F5,r7, all12 for E/F7:
  y²=x³+3,P=(1,2),r13, and8 fixed points for E/F509:y²=x³+2x+10,
  P=(3,68),r479. Twenty-six comparisons total; scalar indices are checker-only.
* Exact order9 support coefficient1, differing ambient lifts for representatives,
  and degree-zero geometry divisor control for r7,13,19,31,43,73,479.

The audit output gives respectively3,5,240 nontrivial division-polynomial terms
for (r,q)=(7,4),(13,2),(479,4), retaining exact signs and exponents. These are
symbolic expression sizes, not timings or performance rankings. The r479 program
is not a compressed evaluator: no runtime improvement follows from this rewrite.
The old source and new expression share the value-division engine, so this is
not independent verification. No scalar recovery or original cohort rerun.

validation-01 is preserved: the first build failed because q inferred as i128
was passed to a u64 evaluator. Its source snapshot and compiler errors remain;
explicit casts for the known2/4 fixture values fixed it in validation-02. Compiler
warnings from unused imported/research code remain in both successful build logs.

## Sources and next construction

KB unavailable (no loaded tools; wrapper status has no executable). Reviewed
feature-native-source/lib.rs, projected-residual/residual.rs, multiseed-factor
and seed-cancellation derivations, and the original shared-projective-grid and
normalized-partial-block source files, read without executing them. Reviewed
Stange, Division polynomials for arbitrary isogenies (2026), equations1.1 and
relation-to-x in section1: https://link.springer.com/article/10.1007/s40993-026-00741-2 .
Its even-index normalization is negative relative to this code; both numerator
factors have the same parity, so the x-difference formula is unchanged. Also
reviewed Sage's division-polynomial documentation:
https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_generic.html .
One guessed Stanford rational-function page was inaccessible; not used as evidence.
No novelty priority claimed.

Next target: a finite-subgroup reduction of the signed exponent program, with
normalization and odd-weight sign retained. In particular investigate whether
its regular interval blocks admit division-sequence product updates without
requesting all individual values. A plain substitution of the fixed-kernel Miller
ratio is no longer the integration plan. Existing source and original4563-case
cohort remain unchanged; this compiler is an audit/reference component only.
