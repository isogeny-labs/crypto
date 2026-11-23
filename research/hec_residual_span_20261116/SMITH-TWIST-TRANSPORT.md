# Smith twist transport: eigenspace and cost gate [pcc:unverified]

This is a mathematical deduction from Smith's correspondence, not a new attack, runtime measurement, independently reviewed proof or completed quotient construction. It applies to the original mixed-term genus-three sources; the earlier binomial-family self-isogeny exclusions have different hypotheses.

## Setup

Let H/Fp have the rational tractable maximal isotropic two-kernel S already verified in native-two-kernel. Let X/Fp denote the smooth projective normalization of Smith's affine equations, assuming the hypotheses of that construction. Write A=J_H and B=J_X. Let phi:A -> B over Fp² be the resulting degree-eight isogeny, with kernel S. Let pi_A, pi_B denote p-power Frobenius. The affine checks do not themselves certify this normalization or its nonhyperellipticity; those remain construction obligations.

For the four maps on p=211 and p=401, the recorded norm scalar alpha is nonsquare. Thus delta1=8 sqrt(s) changes sign under coefficient Frobenius. Smith's equation (11) exchanges correspondences R and R', and the induced maps satisfy phi^(p)=-phi (Smith, Remark 4). The two p=61 maps have square alpha and phi^(p)=phi.

## Transport theorem

For nonsquare alpha and any P in A(Fp),

    pi_B(phi(P)) = phi^(p)(pi_A(P)) = -phi(P).

Consequently the transported source and target belong to the anti-invariant subgroup

    B(Fp²)^- = ker(pi_B + 1),

rather than to B(Fp)=ker(pi_B-1). If ell is odd and P has exact order ell, then phi(P) also has exact order ell: the degree-eight kernel cannot intersect the order-ell subgroup nontrivially. The intersection of invariant and anti-invariant subgroups is killed by 2, since pi(Q)=Q=-Q. Therefore phi(P) is not an Fp-rational point when P has nonzero odd order.

This proves that passing these images directly to an Fp-rational Jacobian solver is invalid, even though all affine equations have Fp coefficients. It does not assert that ell cannot divide #B(Fp) for an unrelated eigenspace.

If T=[k]D, homomorphism and injectivity on <D> give

    phi(T)=[k]phi(D),

with precisely the original scalar k. An extension-field solver on the actual anti-invariant images can therefore recover the original scalar without changing H, D or T. Target transport is mathematically possible; it is not implemented or costed by the affine checks.

## Why a trace shortcut loses the target

For Q in the transported odd-order subgroup,

    (1+pi_B)Q=0,    (1-pi_B)Q=[2]Q.

The rational trace annihilates every transported point. Multiplying this trace by an inverse of 2 modulo ell still gives zero. The anti-trace, followed by inverse-2 multiplication, recovers Q but stays in the anti-invariant subgroup. No integer polynomial in pi_B changes this conclusion: f(pi_B)Q=[f(-1)]Q. If f(pi_B)Q is also Fp-rational, then 2 f(pi_B)Q=0, hence it is zero on this odd-order subgroup. This excludes repairing this particular descent with Frobenius-polynomial projection alone. It excludes neither a different kernel nor a separate descent/representation mechanism.

The corresponding twist PPAV A/S is Fp-defined. For a geometrically nonhyperelliptic genus-three quotient, it must not simply be called the Jacobian of an Fp curve: the Jacobian descent obstruction is exactly what Smith discusses. Twisting H instead changes the source problem and is not an admissible replacement for the fixed targets.

## Consequence for the speed objective

A generic plane-quartic algorithm with heuristic cost soft-O(Q) over field size Q becomes soft-O(p²) if used without exploiting the anti-invariant restriction over Fp². The original genus-three hyperelliptic double-large-prime heuristic is soft-O(p^(4/3)). This substitution supplies no exponent improvement. These are algorithm cost models, not complexity lower bounds; they do not exclude a purpose-built anti-invariant solver.

A legitimate extension successor needs all of:

1. A smooth model and an explicit source/target homomorphism, including exceptional support handling.
2. A factor base and relation generator that preserve and span the anti-invariant odd-order component. Generic rational-point support over Fp is insufficient. Differences P-pi(P) are anti-invariant but require actual Fp² points and correct relation accounting.
3. Complete construction, transport, factor-base, relation, linear algebra, descent and final verification charges on the same original target.
4. A concrete cost case against the strongest normalized original solver, with total new cost at most one tenth of the matched original cost for the claimed 10x result. A q-to-p substitution or uncharged precomputation is insufficient.

For p=61, the rationality obstruction is absent, so canonical normalization and a rational explicit map remain the next constructive gate. Both maps arise from one kernel; they are not two admitted independent quotient routes. This one source does not establish the requested genus-two-through-four speed claim. For p=211 and p=401, retain the obstruction and require the anti-invariant mechanism/cost case before building an extension solver.

## Evidence and primary source

The fixed norm-square outcomes, exact affine checks and their limits are recorded in native-smith-affine/RESULTS.md at commit 85ed51267b6. The identities above are deductions from Smith, *Isogenies and the Discrete Logarithm Problem in Jacobians of Genus 3 Hyperelliptic Curves*, Proposition 4, Remark 4, equations (11) and section 7; the paper also states the two heuristic genus-three algorithm exponents. https://arxiv.org/abs/0806.2995

The eigenspace argument is a proof on geometric group points under the stated isogeny hypotheses. No numerical experiment is needed or claimed here. Independent review and actual normalized-curve/transport implementation remain open.
