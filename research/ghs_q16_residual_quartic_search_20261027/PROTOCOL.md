# Frozen high-prime residual quartic search

27 October 2026. Conductor T-5510. Literature checked before the draw:
Hess (2004) and Menezes--Teske (2004) for the cubic binary descent and
isogeny closure; Tian--Li--Wang--Yu (2018), Tian (2020/2023), and
Frey--Kani (2011/2012) for alternative cover/correspondence routes.
The previous GF(16) random plane-quartic screen and exact Hess-closure
certificate are frozen inputs. A newly generated quartic equation does
not by itself establish a new attack method or publication novelty.

## Fixed population, draw and gates

- Base field `k=GF(16)` and cubic extension `K=GF(4096)` in Sage's
  Conway polynomial basis.
- Source class: every order outside the complete cubic Hess genus-three
  isogeny closure, catalog-positive for a genus-three Jacobian, with
  largest prime `r>1900` and cofactor exactly two. The frozen closure
  certificate gives 13 such orders.
- Draw exactly 30,000 coefficient vectors with first coefficient one
  and the remaining 14 uniform in `{0,...,15}`, using Python
  `random.Random(2026102716)`. Monomials are ordered by descending
  `x` exponent, then descending `y` exponent within total degree four.
  This is a new disjoint deterministic random stream; do not adapt the
  seed or draw count to intermediate hits.
- For each draw, require `#C(k)=17`, then `#C(k²)=257`, then geometric
  smoothness. Count `#C(K)` and derive `c3=(#C(K)-4097)/3` when integral.
  A full Weil-restriction match has source order `4097+c3`.
- Retain the first exact smooth quartic for each of the 13 source
  orders and every complete gate tally. Mark any order already present
  in the preceding 2,048-draw witness list; the comparison is by
  isogeny class, not by equation string.

## Independent replay and claim boundary

Replay every retained quartic with alternate projective charts,
geometric smoothness, and full point counts at degrees one, two,
three. Verify one exact normalized source equation per order, a
selected prime-order `G`, and `Q=[7]G`. Check catalog labels and
previous witnesses for every retained order. The final labels are:
exact equation found in this deterministic screen; cataloged Jacobian
isogeny class; global publication novelty unresolved; evaluated
subgroup transfer missing until explicitly constructed. A point-count
match and a smooth quartic witness alone are not a vulnerability claim.
