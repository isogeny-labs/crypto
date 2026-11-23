# Sources checked and exact reuse

KB unavailable: no kb_status/search_knowledge/search_theorems MCP tools loaded;
the repository wrapper invocation returned127 (no executable). Bounded fallback
queries covered elliptic-code subset sum, norm-polynomial divisibility,
finite-field degeneracy, and q-geometric product evaluation. No novelty or
absence conclusion follows from this coverage.

- [Bostan--Yurkevich, arXiv2012.08656v1](https://arxiv.org/pdf/2012.08656),
  section2.1 Algorithms1/2, pages5--7: read the homothety construction, geometric
  chirp evaluation, non-square tail and total field-operation bound. This is the
  product evaluator being instantiated, not a new product algorithm. Its
  arbitrary-algebra use here is justified by explicit operations; the quoted
  field theorem alone would not identify a nonreduced quotient with a field.
- [Bostan--Schost2005](https://mathexp.eu/bostan/publications/BoSc05.pdf),
  section1.6 assumptions, geometric subproduct construction and section5:
  multiplication-model dependence and geometric evaluation. Counting only
  nonscalar products while hiding linear scalar work is not accepted.
- [Cheng cs/0507026](https://arxiv.org/pdf/cs/0507026), section2 and Theorem1:
  classical elliptic section/subset-sum construction. Variable-arity hardness
  does not exclude the fixed-six distribution here.
- [Li--Wan--Zhang1501.01138](https://arxiv.org/pdf/1501.01138), sections1/4:
  elliptic-code/subset-sum equivalence and density hypotheses. Dense existence
  results do not supply a sparse coefficient constructor.
- [Cardinal--Sharir SoCG2023.22](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.SoCG.2023.22),
  full paper abstract/model and main algorithm: real semi-algebraic range
  search. No prime-field range-search bound is imported.
- [Prior RR9 norm protocol](https://github.com/isogeny-labs/crypto/blob/cdbf32ff70d6560f2d5928f644b613e8f72bf7b4/research/prime31_ic_20261115/rr9/RR9_NORM_FACTOR_PROTOCOL_20261117.md)
  read directly: U^2-CV^2, target sign and quotient-divisibility are existing
  project work, including repeated leaves. This round changes the structured
  torus membership/restriction representation, not the classical norm idea.
- Frozen section-operator-v1/DERIVATION.md: L(5O+R), poles, aliases and the
  low-value-rank/zero-predicate distinction. No old source/report is edited.
- tools/torus-coset-circuit/src/{lib.rs,index.rs,aggregation_controls.rs} and
  tools/torus-trace-lift/src/lib.rs at parent082588f3: actual raw map,
  canonical ordinate, q-power and decoder contracts. Earlier trace lift is
  reused; this proposal must return the original indexed/signed row.
- Mathlib Algebra/GroupWithZero/Units/Basic.lean at pinned
  cfc55fc58714a59dc06e6e79edc6d1d95140d3c2: IsUnit.mul_right_eq_zero,
  IsUnit.pow. These existing laws are the standalone assembly endpoint.

All source interpretations are producer unsigned/unverified. [pcc:unverified]
