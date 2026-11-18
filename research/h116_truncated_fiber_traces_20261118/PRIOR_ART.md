# H116 prior-art and overlap screen

Screen date: 2026-11-18. This is a scoped review, not evidence of publication
novelty. The shared research KB and lead atlas were unavailable in this runtime:
the crypto wrapper had no `research-kb` executable, and the only accessible
`crypto-autoresearcher` checkout stored the requested catalog and canonical
protocol files as `VLT1` encrypted data. Repository notes and primary papers
were used as the fallback.

## Primary sources read

1. Bostan, Morain, Salvy and Schost, *Fast algorithms for computing isogenies [pcc:unverified]
   between elliptic curves*, Mathematics of Computation 77 (2008),
   [arXiv:cs/0609020](https://arxiv.org/abs/cs/0609020), retrieved PDF SHA-256
   `0fdb9c1b5b0568e0149ec5dd67479562c090df4359c1f1070cda829d74dbc78e`.
   Sections 2.2 and 4.2--4.3 identify logarithmic derivatives as power-sum
   series, derive the differential equation for a normalized isogeny's
   `N/D`, and compute its Laurent coefficients and denominator power sums in
   quasi-linear time in the full degree. Their published recurrence is the
   source of H116's large-characteristic jet evaluator; that recurrence and
   the Laurent/power-sum connection are prior art.
2. Bernstein, De Feo, Leroux and Smith, *Faster computation of isogenies of [pcc:unverified]
   large prime degree*, [arXiv:2003.10118](https://arxiv.org/abs/2003.10118),
   retrieved PDF SHA-256
   `058162905a84b39fc6e23f12341b02899cce9328e9b45ac20750fa86cdfedd11`.
   Theorem 4.11 gives quasi-square-root evaluation of structured kernel
   polynomials, and Section 5 applies it to point evaluation. Section 4.14
   explicitly warns that an irrational kernel generator with Frobenius-orbit
   order typically linear in the isogeny degree does not admit the required
   square-root index system. H116 therefore records this as a matched external
   reference, not a generic substitute for the proposed short trace.
3. The H115 prior-art screen at crypto revision `10d380c5df`:
   `research/h115_frobenius_kummer_resolvent_20261117/PRIOR_ART.md`. Its primary
   sources include Robert's isogeny-torsor/biextension notes, explicit isogeny
   representations, elliptic nets and Kummer theory. H116 inherits those
   overlap boundaries.

## Internal obstruction checked

`problems/split-prime-pattern-scalar-only.md` records that prime-to-`r`
isogenies and endomorphisms restrict to known scalar actions on a cyclic
prime-order subgroup. It leaves coordinate-based methods open. H116 is a
coordinate calculation, but its output is still a public function of `Q`; an
efficient evaluator does not evade that obstruction or establish a digit
oracle. [pcc:unverified]

## Narrow question retained

The scoped search did not locate a source stating both denominator-cancelled
relative fiber moments for `F_u=N-uD` and their application to the special
degree-`r` endomorphism `V-1` in the H115 digit experiment. That search gap is
not a novelty claim. The identities themselves are elementary consequences to
prove and test [pcc:unverified]; the ECDLP-relevant question is whether a fixed public predicate
of the resulting moments returns scalar digits on held-out curves.

## Fixed-prefix certificate follow-up

Before formalizing the low-jet certificate, the shared KB wrapper was queried
for all of the following phrases:

```text
truncated power series congruence unit cancellation coefficient prefix isogeny
Bostan isogeny differential equation truncated jet recurrence formal power series
low jet certificate rational map elliptic curve isogeny numerator denominator
negative full degree isogeny map required low coefficients
```

The theorem lookup also queried truncation, multiplication, differentiation and
unit cancellation for `PowerSeries`. The shared wrapper returned no executable
result, so the fallback was a declaration-level search in the pinned Mathlib
source. That located `PowerSeries.trunc_trunc_mul_trunc`,
`PowerSeries.trunc_trunc_pow`, `PowerSeries.trunc_derivative`,
`PowerSeries.coeff_coe_trunc_of_lt` and `PowerSeries.inv_mul_cancel`, which are
used as stated in the Lean proof. The Bostan et al. paper remains the primary
source for the differential equation and recurrence. This scoped search found
no cited result asserting H116's exact `11 -> 10 -> 9` fixed-prefix certificate,
but absence from these queries is not a publication-novelty claim.
[pcc:unverified]

## Formal-scalar construction follow-up

Before freezing the formal-scalar experiment, the unavailable shared KB
wrapper was queried for normalized isogeny jets, source/target uniqueness,
`Verschiebung-1` formal-group coefficients, low division-polynomial
coefficients and the first coefficients of the Bostan recurrence. The fallback
repository and pinned-source search located Mathlib's
`PowerSeries.trunc_X_pow_self_mul` and `PowerSeries.coeff_X_pow`, which are used
as stated for the Lean cutoff. [pcc:unverified]

Bostan--Morain--Salvy--Schost remains the primary source for constructing a
normalized isogeny Laurent series from its source and target curves. H116's
implementation specializes that prior recurrence context to the formal
identity `V-1=[t-1]-pi` and tests whether the first fixed coefficients can be
obtained from `[t-1]` alone because the Frobenius term begins beyond the frozen
precision. The scoped search did not locate this exact fixed-prefix
specialization, but that search gap is not evidence of publication novelty.
[pcc:unverified]

## First omitted Frobenius coefficient follow-up

The unavailable KB wrapper was queried again for Verschiebung/Frobenius formal-
group local parameters, endomorphism equality as a power-series substitution,
and `pi(z)=z^p`; theorem lookup used the same terms. It returned no executable,
so the fallback searched this repository, the available sibling checkouts and
the pinned Mathlib source. Mathlib supplies ring Frobenius and Weierstrass base
change, but this search found no elliptic formal-group or Verschiebung instance.
[pcc:unverified]

The pinned revision does contain the new generic module
`Mathlib.RingTheory.FormalGroup.Basic`. It defines one-dimensional
`FormalGroup R` and proves `F.Xzero_eq_X`, `F.zeroX_eq_X`, substitution
associativity and zero laws. H116 uses that structure directly and derives the
filtered consequence `Y | F(X,Y)-X`; it does not define a competing formal
group. The same scoped search still found no theorem instantiating this object
with a short-Weierstrass elliptic curve and its Frobenius/Verschiebung maps.
[pcc:unverified]

Milne, *Elliptic Curves* (2006),
[online text](https://www.jmilne.org/math/Books/ectext6.pdf), PDF SHA-256
`14a89055a0a7f52b08fad1d53786c9165caba8be837491024b9b210afa384486`,
was read at Remark II.4.3 and Propositions IV.9.1--9.4. Remark II.4.3 states
the one-parameter commutative formal-group axioms and their consequences
`F(X,0)=X` and the existence of formal inverse. Proposition IV.9.1 gives the
coordinate-p Frobenius and its zero tangent action; Theorem IV.9.4 identifies
its characteristic polynomial `X^2-tX+p`. These support the filtered boundary
calculation, while the identification of the dual Frobenius with the actual
retained `V` remains an explicit endpoint premise. [pcc:unverified]

The new control therefore asks a narrower falsifiable question: does the first
coefficient beyond the previously checked scalar prefix agree with the formal-
group prediction on the excluded full map? It is a mechanism check, not a
claim that the missing elliptic instance already exists in the library.
[pcc:unverified]
