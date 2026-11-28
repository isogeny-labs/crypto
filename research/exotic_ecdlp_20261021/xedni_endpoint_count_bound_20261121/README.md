# Bounded rational-word tables have a linear endpoint coverage limit

A Lean theorem bounds any direct target lookup table built from `M` rational
multiples, `C` fixed word coefficients and two signs by **`2MC` distinct
public endpoints**. If a public target is uniform over `N` eligible points,
the table's exact-match success probability is at most
`min(1,2MC/N)`. Covering every eligible point by this lookup policy
therefore requires `M ≥ N/(2C)` when `C>0`. For fixed positive `C`, the number of enumerated
multiples grows linearly with subgroup size. This bound applies to endpoint
lookup under the stated table law; it does not bound methods that derive
additional target relations without enumerating those endpoints.

The fixed-curve order-43 panel provides the exact finite check with
`C=3` coefficients `{1,2,6}`. The generic bound permits full coverage by
`M=7`, but overlaps delay it until `M=14`. At `M=13`, 40 of 42 nonidentity
points are covered and the relation matrix has rank 19. At `M=14`, all 42
are covered and the rank is 20; the preceding height panel records a
7,241-bit maximum affine coordinate through that prefix.

| Prefix `M` | Exact covered targets / 42 | Bound `min(42,6M)` | Relation rank | Largest coordinate bits |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 6 | 6 | 2 | 37 |
| 5 | 24 | 30 | 10 | 924 |
| 7 | 28 | 42 | 12 | 1,810 |
| 10 | 38 | 42 | 18 | 3,695 |
| 13 | 40 | 42 | 19 | 6,242 |
| 14 | 42 | 42 | 20 | 7,241 |

The [protocol](protocol.json) freezes a uniform nonzero target law and the
endpoint set `{±cn mod 43: 1≤n≤M, c∈{1,2,6}}`, including source hashes for
the prior rank/height rows. [`run.py`](run.py) records all 21 prefix counts
and exact uniform hit fractions in [`result.json`](result.json). The
independent [`audit.py`](audit.py) enumerates the actual F₄₇ group points
with a separate modular group law and checks every prefix; its
[`audit-result.json`](audit-result.json) is `PASS`.

## Lean proof map

| Result | Declaration | Quantifiers and scope | Status |
| --- | --- | --- | --- |
| Endpoint image size at most `2MC` | `XedniEndpointCount.endpoint_card_le` | Every type with decidable equality, every natural `M,C`, every map `Fin M × Fin C × Bool → α` | Lean checked |
| Any covered target subset has size at most `2MC` | `XedniEndpointCount.covered_target_count_le` | Same universal map and every finite subset of its image | Lean checked |
| Exact prefix-13 count is 40 | `XedniEndpointCount.prefix13_covered_count` | Frozen `{1,2,6}` table in `ZMod 43` | Lean checked |
| Exact prefix-14 count is 42 and covers all nonzero residues | `prefix14_covered_count`, `prefix14_covers_nonzero` | Frozen table in `ZMod 43` | Lean checked |
| Uniform target hit fraction at most `min(1,2MC/N)` | `uniform_hit_fraction_le_min` | Every positive-cardinality target set, with covered points inside both the target set and endpoint image | Lean checked; fraction identifies uniform probability |

[`formal/Counting.lean`](formal/Counting.lean) is built with pinned Lean and
Mathlib revisions. Retained `#print axioms` output lists only standard Lean
foundations. The theorem counts table endpoints; it makes no assumption
about elliptic-curve rank, rational heights, or a target already known as a
scalar multiple.

For the fixed rank-one construction, this proves a linear-size requirement
for **direct matching to precomputed word endpoints** under a constant-size
coefficient set. The next target experiment should charge construction and
lookup for a previously unseen public point, while testing whether any
relation-discovery step can avoid that endpoint enumeration.

Replay from the repository root:

```sh
python3 research/exotic_ecdlp_20261021/xedni_endpoint_count_bound_20261121/run.py
python3 research/exotic_ecdlp_20261021/xedni_endpoint_count_bound_20261121/audit.py
sh research/exotic_ecdlp_20261021/xedni_endpoint_count_bound_20261121/formal/build.sh
```
