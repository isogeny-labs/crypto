# Incidence capacity of reciprocal P-192 query bundles

**Result.** The 250 complete query bundles that participate in the frozen
reciprocal relation rows have six distinct projected columns each, but every
column belongs to at most two bundles. Double counting therefore gives at
least 750 columns; the exact certificate finds **1,316 columns and rank 181**.
For any nonempty family of endpointwise bundle-difference rows with this
six-column, degree-at-most-two incidence pattern, the row rank is strictly
smaller than its active column count. This proves that the present row policy
cannot close its projected columns, regardless of which of its endpoints are
selected. The general incidence and conditional rank-gap theorems are in
[QueryBundleIncidence.lean](../../lean/QueryBundleIncidence.lean).

The [finite certificate](../../type_i_known_scalar_query_incidence_capacity.json)
reconstructs all bundles from the frozen
[reciprocal screen](../../type_i_known_scalar_query_cross_fan.json), checks
the independent [replay](../../type_i_known_scalar_query_cross_fan_replay.json),
and recomputes each panel's rank directly over the recorded subgroup order.
The 250 active bundles occur at 69 endpoints. Their 1,500 incidences consist
of 1,132 columns used once and 184 columns used twice. The 370 pairwise rows
attain the endpointwise rank ceiling `250 - 69 = 181`.

| Endpoint budget | Complete bundles in rows | Rows | Exact rank | Active columns |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 6 | 15 | 5 | 36 |
| 4 | 24 | 60 | 20 | 142 |
| 8 | 46 | 110 | 38 | 268 |
| 16 | 83 | 178 | 67 | 482 |
| 32 | 147 | 274 | 115 | 824 |
| 69 | 250 | 370 | 181 | 1,316 |
| 81 | 250 | 370 | 181 | 1,316 |

Panels choose endpoints in descending order of `b_u - 1`, then ascending
endpoint index, where `b_u` is the number of complete bundles. This is the
maximum possible endpointwise rank for each budget under the frozen bundle
counts. The exact columns and modular rank are recomputed independently for
each panel. The 81-endpoint panel includes 12 endpoints with fewer than two
bundles, which contribute no rows.

## Theorem and proof map

Let `I` be a finite bundle set, `S_i` the support of bundle `i`, `B` the
union of supports, `N = |I|`, and `d` an upper bound on the number of bundles
containing any column. If every support has `m` columns, then

`m N = sum_i |S_i| = sum_{x in B} #{i : x in S_i} <= d |B|`.

With `m=6` and `d=2`, this yields `|B| >= 3N`. If the bundle-difference
row rank `R` obeys `R + E <= N` for `E > 0` active endpoints, then
`R <= N-E < N <= 3N <= |B|`. Thus `R < |B|`.

| Statement | Lean declaration | Evidence and exact boundary |
| --- | --- | --- |
| `mN <= d|B|` for any finite support family with bounded multiplicity | `query_bundle_incidence_bound` | Complete finite-set double-counting proof. |
| Six-column, degree-two supports and `R+E<=N`, `E>0` imply `R<|B|` | `query_bundle_pair_reuse_rank_gap` | Complete conditional proof. The premise `R+E<=N` is explicit; the concrete ranks are independently recomputed from the frozen rows. |
| With 250 six-column bundles, `|B|<=181` requires `d>=9` | `query_250_bundle_closure_requires_ninefold_reuse` | Complete necessary-bound proof for fixed bundle count and rank target. |
| The frozen coordinate graph has 250 active bundles, 69 active endpoints, 1,316 columns, degree histogram 1,132/184, and rank 181 | Finite certificate, not a Lean declaration | [Analyzer](../../analyze_known_scalar_query_incidence.py) checks source hashes, reversed edges, six distinct columns per bundle, support multiplicity, and modular elimination. |

The rank premise follows mathematically for rows formed by differences of
bundles within each endpoint: a family with `b_u` bundles spans at most
`b_u-1` differences. This vector-space lemma is not yet a Lean declaration;
the formal theorem exposes the bound as a hypothesis, while the finite
certificate verifies that it holds with equality for this graph. The
coordinate-to-Lean support encoding is also open. These are the exact
remaining bridges to a fully formalized theorem about this P-192 graph.

The ninefold threshold is necessary **only if** 250 six-column bundles stay
in use and the active column count is reduced to at most the frozen rank
181: `1500 <= d * 181` gives `d >= 9`. It does not certify a collector or
predict that one exists. A new query geometry or relation type may change
the bundle count, rank, and support multiplicities. A useful next experiment
must produce repeated columns across more bundles while adding independent
rows, then measure unseen-target yield and the full one-target pipeline.
The finite receipt retains `candidate_id: null` until that pipeline is
specified and measured.

Lean is pinned to `leanprover/lean4:v4.34.1` and mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612` in the
[toolchain](../../lean/lean-toolchain) and
[Lake file](../../lean/lakefile.toml). Build and axiom-check output is in
[query-bundle-incidence-lean-build.log](../../lean/query-bundle-incidence-lean-build.log).
The three declarations use standard Lean foundations only. The module has
no `sorry`, `admit`, custom axioms, or native-evaluation proof step.

Reproduce from the `crypto` worktree root:

```sh
python3 -B research/p192_extension_cover_20261012/analyze_known_scalar_query_incidence.py
cd research/p192_extension_cover_20261012/lean
lake build QueryBundleIncidence
lake env lean QueryBundleIncidence.lean
lake env leanchecker --fresh --verbose QueryBundleIncidence
```
