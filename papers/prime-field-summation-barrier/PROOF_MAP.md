# Proof and verification map

Status date: 2026-11-26.  All entries are `[pcc:unverified]` unless a later
review record explicitly promotes them.  This map distinguishes elementary
proof obligations from computational-model assumptions.

| Claim family | Manuscript location | Dependency | Current status | Next verification |
|---|---|---|---|---|
| uniform-target success identity and tuple-count upper bound | section 2, equations (2)--(3) | finite counting; heuristic only for Poisson asymptotics | hand proof plus bounded native finite enumeration; unreviewed | independent proof review |
| complete relation-collection balance | section 2, equations (4)--(6) | declared cost model, rank/yield parameters, sparse-LA upper estimate | accounting derivation, unreviewed | reconcile notation with repository IC cost rules |
| entropy--length lemma | section 3, Lemma 1 | zero-dimensional scheme length and product multiplicativity | hand proof, unreviewed | algebraic-geometry review; formalize finite reduced special case if useful |
| univariate/fibre degree bounds | section 3 | root bound and degree of a morphism fibre | hand proof, unreviewed | check rational-map exceptional loci and saturation language |
| candidate/batching exponent `m/(2m-1)` | APS barrier theorem and sanity checks | explicit candidate/output model plus relation count | hand optimization plus exact displayed-fraction check; unreviewed | independent algebra review |
| quotient-materialization lower bound | APS barrier theorem | definition requires basis/matrix-domain materialization | definitional/model-relative | verify every listed mechanism actually falls in the class |
| split-tree dimension and exceptional-component audit | split-tree and Macaulay sections | generic dimension count, saturation, rationality filtering | hand audit, unreviewed | check degeneracies against PR #4829 implementation notes |
| static-provenance eliminant direct sum | successor Theorem 18 | explicit pre-target leaf provenance, uniform targets, charged leaf records | exact counting proof, unreviewed `[pcc:unverified]` | independent proof review and finite native replay |
| group-`k`SUM-indexing reduction and upper-bound audit | successor Proposition 19 | associativity, exact relation count, Dinur--Golovnev 2026 parameters | exact reduction and algebra audit, unreviewed `[pcc:unverified]` | independent review; determine whether the indexing construction transports to elliptic groups |
| implicit-data-structure exponent criterion | successor Theorem 20 | one-witness query model, tuple-count yield bound, optional conventional `N^2` linear algebra | exact balance calculation plus passing native grid check, unreviewed `[pcc:unverified]` | independent algebra and scope review |
| unrestricted compact-circuit barrier | successor programme | new preprocessing/query direct-sum theorem | **open; not claimed** | formulate a narrow model with a falsifiable lower-bound target |

## Explicit non-implications

The scheme-length bound does not imply arithmetic-circuit size.  Dense
multilinear normal form does not imply a large rational circuit.  First-fall
degree does not prove degree of regularity or solver work.  Generic-group [pcc:unverified]
lower bounds do not automatically apply to coordinate algorithms.  None of
the manuscript's model-relative barriers proves a general ECDLP lower bound. [pcc:unverified]

## Formalization triage

The most realistic Lean target is a finite combinatorial kernel: for a map
from unordered factor-base tuples to a finite group, uniform targets give
success probability `|image|/|G|`, and image size is bounded by the number of
tuples.  Scheme length, rational-map fibres, and Macaulay/resultant statements
should not be advertised as formalized until an appropriate algebraic-geometry
library statement and a clean model boundary have been identified.
