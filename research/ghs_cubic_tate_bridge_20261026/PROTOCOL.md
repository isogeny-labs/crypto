# Cubic Weil-restriction subgroup-survival audit

Frozen on 26 October 2026 before running `audit.py`. Conductor T-5377.
This is an algebraic eligibility audit, not an attack-cost claim.

## Prior work and scope

- Tate's integral isogeny theorem: https://pazuki.perso.math.cnrs.fr/index_fichiers/Tate66.pdf
- Galbraith--Hess--Smart isogeny extension: https://www.iacr.org/cryptodb/archive/2002/EUROCRYPT/2290/2290.pdf
- Hess generalized binary descent: https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf
- Joux--Vitse binary genus-three direct-cover condition: https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf
- Tian cubic Weil-restriction isogeny: https://arxiv.org/pdf/2012.07173
- Previous exact GF(16) cohort: `../ghs_binary_cubic_isogeny_q16_20261026/{PROTOCOL.md,results.json,verification.json,REPORT.md}`.

The direct characteristic-two genus-three hyperelliptic cover with
`Tr_{K/k}(b)=0` is prior work. This audit asks whether the *different*
plane-quartic isogeny-class matches have a guaranteed nonzero image on the
selected prime subgroup under some `k`-isogeny. It does not identify an
efficiently evaluable isogeny or classify all GHS-vulnerable curves.

## Frozen theorem and checks

Let `k=F_q`, `K=F_{q^3}`, `A=Res_{K/k} E`, and `B=Jac(C)` for a smooth
genus-three curve. Put `t=q^3+1-#E(K)` and
`P(X)=X^6-tX^3+q^3`. If `#C(k)=q+1`,
`#C(F_{q^2})=q^2+1`, and `#C(K)=q^3+1-3t`, then `A` and `B` have the
same Frobenius polynomial and are `k`-isogenous by Tate. If a prime
`r != char(k)` divides `P(1)` and `gcd(r,P'(1))=1`, Tate's *integral*
Hom/Tate-module theorem implies the existence of a `k`-isogeny `A->B`
whose action on `A(k)[r]` is nonzero. The proof and its limitations belong
in `REPORT.md`.

Audit the three preregistered orders 4036, 4124, 4126 against the previous
raw cohort. Independently verify each source order with the Walsh point-count
identity, exact `r`-primality and cofactors, absence of direct class B,
`P'(1)` coprimality, and unique normalized `(a,b)` counts. Expected counts
from the exploratory read are 96, 96, 108 respectively. Preserve the input
SHA-256 and an output certificate. The checked Lean file may certify finite
integer gates; it must not be represented as a formalization of Tate's theorem.

## Claim boundary

These hypotheses bound a family of *algebraically transfer-eligible* curves.
Neither a concrete source-to-quartic map nor a full cost below same-target
rho follows. Publication novelty of the exact quartic equations remains
unresolved; no new attack method is claimed.
