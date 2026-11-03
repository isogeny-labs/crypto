# Binary cubic Weil restrictions and plane-quartic witnesses

Frozen before the first run on 2026-10-26. This is a structural screen, not a
discrete-logarithm benchmark. The question is whether an ordinary binary
elliptic curve over `K = GF(2^6)` whose direct GHS genus is four can share its
`GF(4)` Weil-restriction isogeny class with a smooth plane quartic. A matching
Weil polynomial is evidence of an isogeny class, not an explicit isogeny or a
subgroup-preserving transfer.

## Fixed population and field identity

- `k = GF(2^2, modulus="conway")`, `K = GF(2^6, modulus="conway")` in Sage.
- Enumerate every `b != 0` in `K` and the two `a` values `0` and the first
  element with absolute trace one, in the field's integer encoding.
- Use `E: y^2 + xy = x^3 + a*x^2 + b`. Record `a`, `b`, `#E(K)`, largest
  prime-order subgroup and cofactor. Class B means `Tr_{K/k}(b) = 0`; class C
  means `b not in k` and nonzero relative trace. This is the criterion proved
  in `../ghs_n3_genus3_census_20261024/PROTOCOL.md`.
- The predicted Weil polynomial for `Res_{K/k} E` is
  `1 - t*T^3 + 4^3*T^6`, with `t = 4^3 + 1 - #E(K)`.

## Quartic search and checks

- Draw 2048 homogeneous plane quartics over `k`, using Python's
  `random.Random(20261026)` and all 15 degree-four monomials in Sage's
  lexicographic exponent order. Set the coefficient of `x^4` to one and draw
  the other coefficients uniformly from integer encodings `0..3`.
- Reject geometrically singular curves using Sage's `Curve(f).is_smooth()`.
- Count projective points over `GF(4)` and `GF(16)`. Only if the counts are
  `5` and `17`, respectively, count over `GF(64)`. These first two equalities
  characterize the zero degree-one and degree-two coefficients of the Weil
  polynomial for a genus-three curve. Accept a quartic as a Weil-restriction
  class witness if its `N3` gives an integral `t=(65-N3)/3` and the matching
  `#E(K)=65-t` occurs in the enumerated elliptic cohort.
- For every retained witness, recompute `N1,N2,N3` with Sage's projective
  curve rational-point enumeration as an independent count. Record the
  quartic coefficients, source equation, class, group order and polynomial.
- The fixed sample may miss existing quartics. A null search does not prove
  their nonexistence. A found quartic does not establish the map or cost.

## Prior-work and attack gates

Joux--Vitse already describes characteristic-two genus-three hyperelliptic
cover/decomposition attacks and isogeny walks; Tian's Weil-restriction
isogeny construction addresses prime-order cubic curves. An exact quartic in
the same isogeny class is a structural witness, not yet a new vulnerability.
Before a vulnerability label, require an explicit isogeny or correspondence
evaluated on source points, a nonzero map on the selected subgroup, checked
scalar transport, complete quartic relation collection and target descent,
independent replay, and a same-target cold cost comparison with rho.

Primary sources:

- Joux--Vitse, https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf
- Tian, https://arxiv.org/abs/2012.07173
- Diem--Thomé, https://members.loria.fr/EThome/files/non-he-genus3.pdf
