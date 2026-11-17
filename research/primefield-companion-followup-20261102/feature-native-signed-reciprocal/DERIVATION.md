# Reciprocal signed-mode norm constraint

Let r be an odd prime, h=(r-1)/2, m>=3 divide h, and zeta have exact order m in F_r. Put L=h/m, sigma=(-1)^h and
c_n=[X^n](X-1)^h, F_e(V)=sum_j c_(e+jm)V^j.
This is the scalar prime field associated with the subgroup; the elliptic-curve base field is a separate prime field in the existing public source adapter.

## Coefficient reversal

Since c_n=(-1)^(h-n) binom(h,n), c_(h-n)=sigma c_n. Both F_1 and F_(m-1) have L coefficients. Substituting n=1+jm sends its complement to (m-1)+(L-1-j)m. Therefore, as a Laurent-polynomial identity,

    F_(m-1)(V) = sigma V^(L-1) F_1(V^-1).

This needs m|h. It is not the adjacent-coefficient ratio used in the triple reader. When m=3 it recovers mode2; otherwise it replaces mode2 by a different weighted observable.

## Public observations and the norm

For b != 0, write x=k/b and V=x^m. Given existing public-source signs s_j=chi(k-b*zeta^j), define

    f_e = sum_(j=0)^(m-1) zeta^((e-h)j) s_j.

Expanding (k-b*zeta^j)^h and summing roots of unity leaves exactly exponents congruent to e modulo m. Thus f_e=A*x^e*F_e(V), A=m*b^h. This is an algebraic interpretation of signs; implementation signed_observed accepts the sign vector and never takes k or x. No new sign queries are needed to compute f_(m-1), but computing its weights and sums costs arithmetic.

For x != 0 let epsilon=x^h in {1,-1}. Since A^2=m^2 and V^L=epsilon,

    f_1*f_(m-1) = m^2*sigma*epsilon*F_1(V)*F_1(V^-1).

Epsilon is a branch hypothesis. Test both branches unless an independently maintained phase fixes it. Under the earlier quartic phase condition, n=(r-1)/(4m) and V^n=u imply epsilon=u^2.

Set W=V+V^-1. In F_r[W,V]/(V^2-WV+1), write F_1(V)=a(W)+V*b(W). Conjugation takes V to W-V=V^-1, hence

    N(W)=F_1(V)*F_1(V^-1)=a^2+W*a*b+b^2.

The right side is a polynomial in W of degree at most L-1: expand the Laurent product and group terms V^j+V^-j. These satisfy D_0=2, D_1=W, D_(j+1)=W*D_j-D_(j-1). This is the standard Chebyshev/Dickson reciprocal representation; no novelty is claimed for that algebra.

## Implemented quotient filter

Given a monic nonconstant g(W), compute a+bV with existing QuadRing.mode using only the mode1 coefficient stream, then QuadRing.norm modulo g. Form

    E(W)=m^2*sigma*epsilon*N(W)-f_1*f_(m-1).

Return gcd(g,E). Quotient reduction commutes with the norm polynomial, so this gcd equals gcd(g, the unreduced expression), including non-squarefree g. There is no division by either observed mode or by F_1, and no adjacent-coefficient denominator batch. Existing stream-transition inversions, interval setup, polynomial products/reductions and gcd remain. No whole coefficient vector is retained by this path. The existing recursive quotient evaluator has its own allocation/power costs; these are not bounded by the coefficient block cap alone.

## Boundary and next construction

The V inverse chart excludes x=0. At x=0 both signed modes vanish and provide no endpoint test; the existing public endpoint check must handle it. If N=0, both epsilon branches satisfy the product equation. Even otherwise W identifies {V,V^-1}, not an orientation. The filter is necessary and can retain extraneous candidates. Retain phase information and individual f_1 for orientation, replay public signs and perform final point verification. This patch does not swap the production recovery adapter or claim equal pruning to its mode2 equation.

Next integrate this filter with the existing canonical reciprocal fibers, preserving the phase and oriented-root state. The decisive question is whether removing the second coefficient stream pays for the changed constraint's retained candidates and quotient work. That requires a common frozen comparison before a selection policy.

## Sources and proof map

- Existing ../feature-native-triple-modes/{quadratic.rs,reciprocal.rs,canonical.rs,context.rs}: supplies the quotient norm, reciprocal mode0 representation and bounded mode1 stream.
- NIST DLMF18.5.1, https://dlmf.nist.gov/18.5.E1 (read 2026-11-08): T_j((V+V^-1)/2)=(V^j+V^-j)/2. Its integer polynomial recurrence explains why the representation reduces in odd characteristic.
- KB unavailable: MCP tools not loaded; scripts/research-kb status reports no executable. No novelty or exhaustive-literature claim.
- Written proofs above cover reversal, weighted projection, product and quotient filter. Rust exact controls are finite checks, not universal proofs. Lean scope and checked declarations are recorded separately in PROOF.md.
