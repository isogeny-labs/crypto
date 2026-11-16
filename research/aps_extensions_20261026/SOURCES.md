# Sources and scope of the literature check

Reviewed 2026-10-26 before implementation selection, with a second primary-source
check during validation. Repository baseline: `f25c4602a1471d325c58e659391e34b2dcbd3d66`.
Sibling atlas snapshot: `fd387705a71cd2f4f3a896af827d3676e0a07659`.
This is a focused source map, not an exhaustive novelty search.

## Primary literature

1. **Amadori, Pintore, Sala**, *On the discrete logarithm problem for
   prime-field elliptic curves*, Finite Fields and Their Applications 51
   (2018); [ePrint 2017/609](https://eprint.iacr.org/2017/609).
   Full revised author manuscript read, especially §§3.1–3.4:
   [Trento manuscript](https://iris.unitn.it/retrieve/e3835194-60e8-72ef-e053-3705fe0ad821/Amadori%2C%20Pintore%2C%20Sala%20-%20On%20DLP%20for%20prime-field%20EC%20%28rev%29.pdf).
   Retrieved PDF SHA-256:
   `67442c30c2f14ab7891b6285f9b6d3c77cf5b1a6d2e838ec25b815a280029ba1`.
   This is a single signed zero-sum among **known** `(u,v)` combinations,
   using `S_m=0`. It is not the ordinary many-relation
   `S_(m+1)(...,x(R))=0` pipeline. Equations (10)–(11) give the partition
   constraints. §3.4 already uses two joined S3 equations for m=4.
   The reported experiments cover 11–22-bit fields and m=3,4,5. §3.2
   explicitly leaves the solver complexity unresolved and does not claim
   an improvement over generic cyclic-group algorithms.

2. **Petit, Kosters, Messeng**, *Algebraic approaches for the Elliptic Curve
   Discrete Logarithm Problem over prime fields*, PKC 2016.
   [Author manuscript](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf).
   Full body checked in this review; §§3.1, 3.3–3.4 inform the map-based
   follow-up. Multiplicative cosets and composed low-degree/rational/isogeny
   maps are prior work, not new mechanisms proposed by this study.

3. **Galbraith, Gebregiyorgis**, *Summation polynomial algorithms for elliptic
   curves in characteristic two*, INDOCRYPT 2014, pp. 409–427;
   [author manuscript](https://www.math.auckland.ac.nz/~sgal018/shishay.pdf),
   [ePrint 2014/806](https://eprint.iacr.org/2014/806).
   The disjoint factor bases, invariant-variable construction and probability
   discussion are relevant prior work. Their ambient setting is characteristic
   two, so those field-linear constructions are not automatically available
   over a prime field.

4. **Kudo, Yokota, Takahashi, Yasuda**, *Acceleration of Index Calculus for
   Solving ECDLP over Prime Fields and Its Limitation*, CANS 2018,
   LNCS 11124, pp. 377–393,
   [DOI](https://doi.org/10.1007/978-3-030-00434-7_19).
   Checked the [official university record](https://kyushu-u.elsevierpure.com/en/publications/acceleration-of-index-calculus-for-solving-ecdlp-over-prime-field/)
   and [author bibliography](https://sites.google.com/view/m-kudo-official-website/english/papers).
   Their abstract explicitly describes hybrid solving and summation-polynomial
   symmetries applied to APS. **Full-text gap:** no accessible full body was
   obtained in this session. Consequently this PR claims neither novelty nor
   reproduction of their exact implementation, quantitative results or bounds.

## Existing Crypto work reused or distinguished

| Source | Relevance and limit |
| --- | --- |
| [`ecdlp_leads_20261018` L4](../ecdlp_leads_20261018/README.md#l4-audit-of-unchecked-prime-field-ic-algorithms) | Existing request to reproduce APS natively; this study covers only tiny correctness, not its requested matched-rho ladder |
| [`f4_fp`](../../src/cryptanalysis/f4_fp.rs) | Native equation solving/substitution; reuse avoids a parallel solver implementation |
| [`symmetrized_semaev`](../../src/cryptanalysis/symmetrized_semaev.rs) | Established symmetric S3 representation; no novelty attributed to its rewrite |
| [`glv_invariant_base`](../../src/cryptanalysis/glv_invariant_base.rs), [`glv_invariant_experiments`](../../src/cryptanalysis/glv_invariant_experiments.rs), [GLV study](../notes/index-calculus/RESEARCH_GLV_INDEX_CALCULUS.md) | Existing orbit folding and invariant-system experiments, including negative results; the bounded APS orbit-label route is a distinct interface, not a new GLV principle |
| [`factor_base_designs_20261015`](../factor_base_designs_20261015/README.md) | Existing design register and map/quotient constraints; preserve its scoped formal closures |
| [`pdp_split`](../../src/cryptanalysis/pdp_split.rs), [split study](../pdp_split_dreg_20261011/README.md) | Prior topology/degree work in a different field setting; prime-field APS cost remains an empirical question |
| [`polynomial_reuse`](../../src/cryptanalysis/polynomial_reuse.rs) | Prior target-independent template reuse; cold versus amortized costs must remain distinct |
| [`semaev_rootfind_e2e_bench`](../../examples/semaev_rootfind_e2e_bench.rs), [`rootfinding-complete-cost-calibration`](../../problems/rootfinding-complete-cost-calibration.md) | Related native pipeline and unresolved complete-cost accounting; not a completed APS performance reproduction |

The sibling lead atlas was read before the local research survey. Relevant
rows are named in [TRACKER.md](TRACKER.md). In particular, bounded-degree
membership obstructions do not prove that every alternative algebraic
presentation costs the same.

## Interpretation constraints

- A nonzero univariate polynomial vanishing on a set of distinct coordinates
  has degree at least that set's size. Sparsity or a composed system can change
  representation without violating that root-count bound.
- Prime fields have no proper subfields. An extension-field trace trick needs
  an explicit extension model and its full cost; it is not a free prime-field
  factor base.
- Count distinct decompositions consistently. Ordered tuples, permutations,
  choices of point signs, polynomial roots, and informative verified relations
  are different populations. Use measured informative-relation yield in the
  planned panel rather than treating an expected relation count as an exact
  success probability.
- Selecting structured coordinates does not supply their known `(u,v)` labels.
  Rejection sampling or a changed many-relation algorithm must be charged.

No result from the earlier out-of-repository Python diagnostic is imported as
native evidence, and no Python research implementation is added by this PR.
