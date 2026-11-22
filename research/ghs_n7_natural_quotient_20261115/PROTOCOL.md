# Natural quotient hyperellipticity audit

Frozen before the finite census on 2026-11-15 UTC. The question is whether the
natural rank-three Artin--Schreier quotient of the published n=7 W1 genus-14
cover is hyperelliptic outside the classical cubic-kernel locus. This is a
method-class question, not a claim of a new weak curve.

## Sources checked

- Menezes--Teske, *Cryptographic implications of Hess' generalized GHS
  attack* (2004), Remark 2 and Section 5.2:
  <https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf>.
- Hess, *Generalising the GHS attack* (2004), construction and quotient-map
  caveats: <https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf>.
- Maurer--Menezes--Teske, *Analysis of the GHS Weil descent attack* (2002):
  <https://www.cambridge.org/core/services/aop-cambridge-core/content/view/D4B264D082FF67354EE9968D9803102A/S1461157000000723a.pdf/analysis_of_the_ghs_weil_descent_attack_on_the_ecdlp_over_characteristic_two_finite_fields_of_composite_degree.pdf>.
- KB revision `9141e386254340a9bdba3036f2bb706a`, last refreshed
  `2026-11-14T21:44:48Z`; searches for the exact natural quotient, negative
  hyperellipticity form, and adjacent Prym cover found the GHS sources but no
  exact criterion. Relevant KB source IDs: `source:16682eb7fcba0356f5106537db703de6`
  and `source:ad284d00fb0970a6ed5f9d9eb2f5b29d` (internal literature
  notes; primary papers above control interpretation). A missing KB hit does
  not establish novelty.

## Exact finite census

For q=2,4, both irreducible cubic factors p of T^7+1, enumerate
V=ker p(sigma), H=k+V, and every W1 decomposition gamma1 in V\{0},
gamma2=v+d with v in V\{0} and d in k\{0}. This is the exact
sigma-order `(T+1)p` condition; merely taking `gamma2 in H\V` would
also admit elements of k with sigma-order T+1. Set beta=gamma1*gamma2
and b=beta^2. The predicted
inversion condition is `(sigma+1)gamma2 = c*(sigma+1)gamma1` for some
`c in k*`. Falsification: any nonclassical b satisfying this condition,
or any classical b without one, disproves the finite criterion or the
implementation. Compare enumerated W1 set counts against the previous
independent census: 42 per arm for q=2, 2898 per arm for q=4.

Record modulus, source hash, compiler version, elapsed time, raw stdout,
and failed assertions. The run is local and small; checkpoint push and gate
precede execution under repository rules. Finite enumeration checks only
the coefficient condition. Geometry is argued separately in `THEOREM.md`.
