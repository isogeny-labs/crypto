# Cross-endpoint known-scalar query triples on the P-192 cover

**Result.** Joining the three endpoint roots of a query cubic gives a new
known-scalar relation across different endpoints. The frozen P-192 graph has
23 query cubics whose three endpoints all admit complete six-column bundles.
Comparing one deterministic bundle triple from each of 22 cubics to the first
cubic adds **20 independent rows** to the prior 181-rank endpointwise system:
392 verified rows have exact rank **201** on **1,348 active projected columns**.
The selected 257 bundles still have column multiplicity at most two. This
quantifies the gain from a new relation type and the remaining column-reuse
problem for this exact geometry.

Let `u_{j,1}+u_{j,2}+u_{j,3}+k q_j=C` be the projected-root sum of
query cubic `j`, with `k=2h` and known `q_j=[-2a_j]G`. A complete bundle at
endpoint `u` cut by query cubic `i` has projected residual sum

`X_i(u)=3S-3u-C+k q_i`.

Choose one bundle `i_t` at each of the three endpoints of `j`. Then

`sum_t X_{i_t}(u_{j,t})=9S-6C+k(3q_j+sum_t q_{i_t})`.

Subtracting the corresponding expression at a different query cubic `j'`
cancels both shared constants. The right side is the known scalar
`[-4h(3a_j+sum_t a_{i_t}-3a_j'-sum_t a_{i'_t})]G`, computed modulo the
recorded subgroup order. The checked
[Lean theorem](../../lean/CrossEndpointQueryTriple.lean)
proves this consequence of the two cubic-root and aggregate bundle equations.
The finite-field input equations and coordinates are checked by the frozen
query-transfer and cross-fan receipts; the theorem states them as hypotheses.

| Result | Lean declaration | Hypotheses and verification |
| --- | --- | --- |
| Sum three individual bundle equations | `aggregate_three_query_bundles` | An arbitrary additive commutative group, natural `k`, and each six-column sum equation; checked proof. |
| Cancel two cubic-root sums in the aggregate relation | `cross_endpoint_query_triple_difference` | Two cubic-root sums and two aggregate bundle equations in the same group with the same `S`, `C`, `k`; checked proof. |
| Derive the complete cross-cubic identity from six bundle equations | `cross_endpoint_query_triple_from_six_bundles` | Both root-sum equations and all six individual bundle equations; checked proof. |
| Identify P-192 coordinate points, query scalars, and exact rank | Frozen replay receipt and analyzer | Exact group and modular calculations; the coordinate-to-Lean bridge is not formalized. |

The Lean theorem covers the complete additive-group cancellation, including
its structural step from six bundle equations. The concrete quartic line-cut
construction and its subgroup projection remain coordinate-verified inputs.
The module contains no placeholders, research-specific axioms, or native
proof evaluation. Its `#print axioms` results use `propext` and `Quot.sound`
where reported; `aggregate_three_query_bundles` uses `propext` only.
Lean 4 v4.34.1 and mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612` are pinned by the
[toolchain](../../lean/lean-toolchain) and [Lake file](../../lean/lakefile.toml).


| Frozen policy | Bundle rows | Cross-cubic rows | Exact rank | Active columns | New columns | Selected bundles | Maximum column multiplicity |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Endpointwise baseline | 370 | 0 | 181 | 1,316 | 0 | 250 | 2 |
| First available bundle at each root | 370 | 22 | 201 | 1,348 | 32 | 257 | 2 |
| Prefer existing columns, then lowest query index | 370 | 22 | 201 | 1,348 | 32 | 257 | 2 |

Eligible query-cubic indices are `0, 2–19, 23–26`. The anchor is index 0.
The other four cubics lack a complete bundle at at least one endpoint.
Two cross-cubic rows are dependent on the combined system and remain in
the 392-row measurement; this is an exact modular rank, not an assumed
per-triple independence claim. Both selection policies happen to have the
same counts on the frozen graph; the [receipt](../../type_i_cross_endpoint_query_triples.json)
retains their exact selected query indices and row scalars.

The [analyzer](../../analyze_cross_endpoint_query_triples.py) checks frozen
source hashes, all 27 cubic sums, all 258 bundle group equations, every new
cross-cubic scalar equation, the represented column coefficients, and sparse
modular rank. It uses an independent Python P-192 group implementation
already used by the cross-fan verifier. Reproduce from the `crypto` root:

```sh
python3 -B research/p192_extension_cover_20261012/analyze_cross_endpoint_query_triples.py
cd research/p192_extension_cover_20261012/lean
lake build CrossEndpointQueryTriple
lake env lean CrossEndpointQueryTriple.lean
lake env leanchecker --fresh --verbose CrossEndpointQueryTriple
```

The coordinate replay establishes the cross-endpoint mechanism and its
measured row gain. The present selected bundles have 1,154 columns used in
one bundle and 194 in two. A different geometry that repeats projected
columns across more bundles is the next construction target; the stored
`candidate_id` remains null until a factor base, target handling, and a
complete measured pipeline are specified. The single-target accounting
contract in the repository governs any subsequent online comparison.
