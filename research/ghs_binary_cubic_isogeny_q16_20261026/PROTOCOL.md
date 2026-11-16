# Binary cubic GHS extension: GF(16) to GF(4096)

Frozen before the quartic draw on 26 October 2026. Conductor T-5361.
This extends the independently replayed GF(4) and GF(8) structural screens
in `../ghs_binary_cubic_isogeny_20261026/`. A Weil-polynomial match is a
necessary structural gate, not an evaluated transfer or attack result.

## Literature and novelty boundary

Joux--Vitse already gives characteristic-two hyperelliptic cover,
decomposition and isogeny-walk methods. Hess's Magma implementation computes
a binary GHS plane curve and a divisor map when its descent applies.
Tian's cubic Weil-restriction isogeny paper uses an odd auxiliary prime
different from the field characteristic for its quotient, but its published
normal-Weil-set and plane-quartic reconstruction steps use a full symplectic
geometric `E[2]` basis, three nonzero two-torsion points on
`y^2=x^3+a_4*x+a_6`, and an Aronhold system of bitangents. An ordinary
elliptic curve in characteristic two has only one nonzero geometric
two-torsion point, so those displayed steps do not directly instantiate
for this binary cohort. This is a limitation of the cited construction,
not an impossibility theorem for odd-degree isogenies in characteristic two.
An exact new quartic equation, a new eligible family, and a new evaluated
attack method remain separate novelty labels.

Primary sources checked before execution:

- Joux--Vitse, https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf
- Tian, especially Sections 2.1 and 3, https://arxiv.org/pdf/2012.07173
- Hess/Magma binary Weil descent, https://www.math.uzh.ch/sepp/magma-2.19.8-cr/html/text1383.htm
- Diem--Thomé plane-quartic index calculus, https://members.loria.fr/EThome/files/non-he-genus3.pdf
- Local prior results: `../ghs_binary_cubic_isogeny_20261026/REPORT.md`,
  `../ghs_generalized_20261024/REPORT.md`, and
  `../ghs_cover_decomposition_20261025/COVER_DECOMPOSITION.md`.

## Frozen search

- Field `k=GF(2^4)` and cubic extension `K=GF(2^12)` use Sage's Conway
  polynomial-basis models. Record their exact moduli and chosen embeddings.
- Source population is the complete 2*(4096-1)=8190 normalized ordinary
  models `E: y^2+x*y=x^3+a*x^2+b`, with `a=0` or the first polynomial-basis
  element of absolute trace one and every nonzero `b` in `K`.
- Classify direct binary GHS source type B by relative trace
  `b+b^16+b^256=0`; class A has `b^16=b`; remaining models are C.
- Draw 2048 quartics with `random.Random(2026102616)`. The leading
  `x^4` coefficient is 1; each other coefficient is an independent uniform
  integer in `[0,16)`. Monomials are `x^i*y^j*z^(4-i-j)` for descending
  `i=4..0` and then descending `j=4-i..0`.
- Filter in order: `N1=#C(k)=17`, `N2=#C(GF(256))=257`, geometric smoothness,
  then independently count `N3=#C(K)` by the two standard projective charts.
  Retain a witness only if `(4097-N3)` is divisible by 3 and the resulting
  Weil polynomial `X^6-t X^3+4096`, `t=(4097-N3)/3`, has a source order
  `4097-t` in the complete population. Retain the first witness for each
  matching order; preserve all population rows and tallies.
- Highlight orders with no class-B source, a selected source prime subgroup
  `r>=1000`, and cofactor at most four. This gate was chosen before the
  draw and does not assert vulnerability.
- Independently recount all source groups and each retained quartic over
  `k`, `GF(256)`, and `K`; for highlighted rows check Jacobian order,
  subgroup points and rational-point class reach. Any uncompleted replay
  stays explicitly incomplete.

An attack claim additionally needs an evaluated source-to-Jacobian map
whose selected prime-subgroup image is nonzero, complete relation and
target-descent execution, independent scalar replay, and measured full
same-target cost against the strongest applicable rho. None follows from
the frozen structural filter.
