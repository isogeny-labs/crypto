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
