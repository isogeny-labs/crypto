# CT25: absolute cross-factor obstruction and both quotient norms

2026-11-11. **[pcc:unverified]**. Regular stacked continuation of [CT24 #3928](https://github.com/isogeny-labs/crypto/pull/3928) and [root #3877](https://github.com/isogeny-labs/crypto/pull/3877). This is an independent mathematical/finite-field negative control, **not** the missing whole-Jacobian (7,7) evaluator, signed PCC proof, or DLP speedup.

## Hypothesis, outcome, and prior work

CT23 established only that rational homomorphisms between groups of orders 109 and 113 vanish. The stronger question was whether an algebraic cross-factor map might reappear after extending \\(\\mathbb F_{101}\\). CT03/CT04 (#3907) proved the general elliptic-factor obstruction but did not identify the two CM fields in CT20/CT24. The present fixed examples **cannot mix across these factor types even over an algebraic closure**. This uses ordinary elliptic CM, not a newly invented attack or a heuristic about point orders.

For the original factors

- \\(E_{109}:y^2=x^3+x+46\\), \\(\#E_{109}(\\mathbb F_{101})=109\\), \\(t=-7\\);
- \\(E_{113}:y^2=x^3+44x+21\\), \\(\#E_{113}(\\mathbb F_{101})=113\\), \\(t=-11\\).

Both are ordinary because 101 does not divide their Frobenius traces. The characteristic polynomials of Frobenius and their discriminants are

\\[
X^2+7X+101,\quad (-7)^2-4(101)=-355,
\qquad
X^2+11X+101,\quad (-11)^2-4(101)=-283.
\\]

Since \\(-355\\) and \\(-283\\) are different squarefree negative discriminants, the geometric endomorphism algebras are the distinct fields

\\[
\\operatorname{End}^0_{\\overline{\\mathbb F}_{101}}(E_{109})
=\\mathbb Q(\\sqrt{-355}),\qquad
\\operatorname{End}^0_{\\overline{\\mathbb F}_{101}}(E_{113})
=\\mathbb Q(\\sqrt{-283}).
\\]

**Proposition (standard CM obstruction).** \\(\\operatorname{Hom}_{\\overline{\\mathbb F}_{101}}(E_{109},E_{113})=0\\) and the reverse Hom group is zero. Indeed, any nonzero morphism of elliptic curves is an isogeny onto the other elliptic curve; conjugation by the isogeny identifies their rational endomorphism algebras. The two quadratic fields are nonisomorphic (different squarefree radicands). Contradiction. Hence the same vanishing holds over every algebraic extension of \\(\\mathbb F_{101}\\).

For any genus-two Jacobians in this specific \\(E_{109}\\times E_{113}\\) isogeny class, the induced rational-isogeny-category Hom matrices must have zero off-diagonal factor entries. This is a **geometric** obstruction to *cross-factor algebraic-group mixing*, not merely the CT23 order-coprimality observation. It does **not** rule out (i) maps within either factor's geometric isogeny class, (ii) higher-genus representations carrying that factor, (iii) general computation without group-homomorphism structure, or (iv) a more efficient DLP algorithm. In particular, it says nothing universal about cross-characteristic **computational** encodings.

Reference: Zarhin, *Isogeny classes and endomorphism algebras of abelian varieties over finite fields* (2023), https://doi.org/10.4213/im9332 ; standard ordinary elliptic endomorphism-algebra theorem and HLP gluing.

## Independent geometric norm calculation

For each accepted CT20/CT24 even-sextic chart

\\[
C:y^2=t^6+At^4+Bt^2+b
\\]

there are two degree-two elliptic quotient maps (with smooth projective completion):

\\[
\\pi_{\\mathrm{even}}(t,y)=(t^2,y),\quad
E_{\\mathrm{even}}:v^2=u^3+Au^2+Bu+b,
\\]
\\[
\\pi_{\\mathrm{rec}}(t,y)=(b/t^2,by/t^3),\quad
E_{\\mathrm{rec}}:v^2=u^3+Bu^2+bAu+b^2.
\\]

The reciprocal deck involution \\((t,y)\\mapsto(-t,-y)\\) fixes \\(\\pi_{\\mathrm{rec}}\\) and negates \\(\\pi_{\\mathrm{even}}\\). Conversely \\((t,y)\\mapsto(-t,y)\\) fixes \\(\\pi_{\\mathrm{even}}\\) and negates \\(\\pi_{\\mathrm{rec}}\\). Applying the norm/pushforward to both points of a generic fiber proves the **geometric identities**

\\[
(\\pi_{\\mathrm{even}})_*(\\pi_{\\mathrm{rec}})^*=0,
\qquad
(\\pi_{\\mathrm{rec}})_*(\\pi_{\\mathrm{even}})^*=0
\\]
on elliptic Picard groups. Both identities extend through ramification because the maps are regular on the projective completions and group homomorphisms agree on a dense open set. The diagonal relations remain \\(\\pi_*\\pi^*=[2]\\). These are special-case, independent quotient-level controls for the future actual \\(\\Phi\\), not a construction of \\(\\Phi\\).

The reciprocal pullback of a point \\(Q=(U,V),U\\ne0\\) is the degree-two closed divisor

\\[
t^2=b/U,\qquad y=(V/U)t.
\\]

Its **even** quotient images are \\((b/U,\\pm (V/U)\\sqrt{b/U})\\), so they sum to zero. Its **reciprocal** images are both \\(Q\\), so their sum is \\([2]Q\\). The reference divisor \\(D_0=\\pi_{\\mathrm{rec}}^*(O)\\) consists of the two points \\((0,\\pm\\sqrt b)\\); their even images also sum to zero. The exceptional \\(U=0,V=\\pm b\\) fibers are double genus-two infinities, whose even quotient image is \\(O\\). The zero elliptic point yields the zero Jacobian class.

## Reproducible independent oracle

The new standard-library Python oracle deliberately does **not** import CT20, CT23, CT24, Rust, Sage or a shared field engine. It separately implements \\(\\mathbb F_{101^2}=\\mathbb F_{101}[w]/(w^2-2)\\), affine long-Weierstrass arithmetic, all point counts and chart conversions.

Run with assertions enabled:

```sh
python3 -S research/chained_jacobian_transfers_20261110/ct25_absolute_cross_factor.py
python3 -m py_compile research/chained_jacobian_transfers_20261110/ct25_absolute_cross_factor.py
```

Producer observation for **both** B+A and B+B: all **109** reciprocal target rational points enumerated, including **58** split fibers, **48** nonsplit degree-two fibers, **2** ramified infinity fibers and the elliptic origin. Every nonexceptional pair was checked individually against the sextic and both quotient equations in the independent Fp² engine. The reciprocal projected sum was \\([2]Q\\), while the even projected sum vanished; Fp-Frobenius fixed the split points and swapped the nonsplit support. The two elliptic factor orders and distinct CM radicands were checked directly. The deliberate wrong even-quotient coefficient is rejected.

**Development check:** separate local Python execution passed on 2026-11-11; exact-committed-byte GitHub CI and required independent PCC review must be tracked separately. `[pcc:unverified]`

## What this eliminates and what remains

A chain of algebraic group homomorphisms through this particular split isogeny class **cannot migrate an E109 DLP into its E113 factor**, even by passing through higher extensions, because the geometric Hom is zero. This strengthens the required zero-cross-factor acceptance criterion; it does not block transfer along the E109 factor (which CT24 implements into explicit genus-two divisor classes).

**Next nonduplicative implementation:** arbitrary-class even-sextic balanced Cantor arithmetic; both quotient norms and pullbacks including infinity corrections; the actual \\(J_0\\rightarrow J_1\\) and dual map (not rational-product halving); 12,317-class and all-109-log replay, and a same-host, single-target complete cost ledger against direct E109, rho, and IC. Signed independent PCC still required. Do not edit root frozen ledger, archive chain or encrypted atlas. No EKS, S3, or paid compute was used. Parent PCC fork, CI/review, and budget admission remain gates.
