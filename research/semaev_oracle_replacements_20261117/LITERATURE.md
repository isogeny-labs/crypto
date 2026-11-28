# Literature and repository receipt

Checked before implementation on 2026-11-17.

## Knowledge base

`scripts/research-kb status` could not run because no `research-kb` executable
or MCP tool is available. The sibling `crypto-autoresearcher` checkout exists
at commit `e098f29460ceb76130ca05eb30d8e03487fd269a`, but its atlas, schema and
canonical experiment skill are encrypted `VLT1` blobs in this environment.
Receipt: **KB unavailable**. No atlas-absence or novelty claim is made.

Fallback repository search found:

- `research/pkm_ideas_20261115`: F4 trace replay already tested across targets,
  curves and primes. It saved work but its frozen `<=0.5` prediction failed in
  5 of 11 cells. This study does not relabel trace replay as parametric
  compilation.
- `research/postgb_linear_stage_20261026`: the repo already has a fast
  multiplication-matrix/Krylov/RUR stage after F4. This study moves the
  boundary before Gröbner-basis construction and reuses none of that result as
  evidence of a new speedup.
- `research/relation_degree_trace_20261115/QUOTIENT_PROTOCOL.md`: a certified
  quotient-algebra/RUR route is already a development lead. The root-key join
  here is a narrower projection of quadratic Semaev links, not a claimed
  general quotient solver.
- `src/cryptanalysis/sparse_macaulay.rs`: structured `F_2` Macaulay
  elimination, not the prime-field Semaev object here.

## Primary sources

- Igor Semaev, *Summation polynomials and the discrete logarithm problem on
  elliptic curves*, IACR ePrint 2004/031. The HTML title/metadata were
  accessible; the PDF returned HTTP 403. The exact `S_3` formula is already
  implemented and tested in `pkm_smooth_tower`, and this study cross-checks
  every generated quadratic against it.
- Hung Q. Ngo, Ely Porat, Christopher Ré and Atri Rudra, *Worst-case Optimal
  Join Algorithms*, arXiv:1203.1952. Full PDF read from arXiv. Relevant scope:
  natural joins, AGM fractional-cover bounds, and avoiding a sequence of
  wasteful binary joins.
- Mahmoud Abo Khamis, Hung Q. Ngo and Atri Rudra, *FAQ: Questions Asked
  Frequently*, arXiv:1504.04044v7. Full PDF read from arXiv. Relevant scope:
  InsideOut variable elimination, indicator projections and fractional FAQ
  width. These frameworks do not promise a favorable width for the Semaev
  chain; the experiment measures the projected relation rather than assuming
  one.
- Fabrice Rouillier, *Solving Zero-Dimensional Systems Through the Rational
  Univariate Representation*, AAECC 9 (1999), DOI
  `10.1007/s002000050114`; metadata and abstract-level record checked.
- Jean-Charles Faugère and Chenqi Mou, *Fast algorithm for change of ordering
  of zero-dimensional Gröbner bases with sparse multiplication matrices*,
  ISSAC 2011, DOI `10.1145/1993886.1993908`; metadata checked. Both concern
  post-basis or quotient-algebra structure and do not establish this
  projection join as a new asymptotic algorithm.

The minimal-polynomial key observation is elementary: two monic irreducible
quadratics over a field share an algebraic root iff they are equal. No novelty
claim is attached to using it as a hash key.
