# Rank-1 semantic-cut and equality-quotient result

Date: 2026-11-01

Overall disposition: **legacy exact census rejected at feasibility scope;
minimal typed global-wiring mechanism rejected; broader theorem and full null
remain open**

## Outcome

Stable semantic-region tags now bind the actual hashed decomposition encoder's
mid-slot cut into canonical predicate identities. On the disjoint F5
development construction, the draft E1 contractor would enumerate `2^15959`
assignments for one left slot and `2^31918` for two. Its raw cut width is 154,
already above any materializable exact tensor. The original production census
path is therefore rejected at implementation-feasibility scope; this is not a
rank observation.

The apparent width has an exact redundancy: the 154 raw cut edges are copies
of seven typed encoder signals. Quotienting those equality classes and
enumerating only source bits gives the exact natural boundary tensor. A
symmetry argument computes the exact rank over every balanced raw-edge split
without expanding the copies:

- each class is column-only, split, or row-only;
- splitting a class makes the matrix block diagonal;
- moving a split class wholly to either side cannot increase rank;
- while preserving row-edge balance, mixed classes can be resolved pairwise,
  so an exact minimum occurs with at most one split class.

Forty-eight synthetic quotient tensors of raw width at most eight matched
exhaustive raw-split ranks exactly. A separate fanout network's quotient tensor
and ranks matched direct raw-edge contraction. Missing functional metadata,
region mutation, and stale rewired graph/program combinations fail closed.

## Held-out natural panel

The four cells and gate were frozen in
[`RANK1_QUOTIENT_PROTOCOL.md`](RANK1_QUOTIENT_PROTOCOL.md) after the F5 row and
before their construction.

| field bits | raw cut width | quotient width | support | canonical rank | best exact raw rank | balanced maximum | rank fraction |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 4 | 204 | 9 | 6 | 6 | 2 | 16 | 0.125000 |
| 5 | 254 | 11 | 8 | 8 | 3 | 32 | 0.093750 |
| 6 | 304 | 13 | 19 | 19 | 6 | 64 | 0.093750 |
| 7 | 354 | 15 | 40 | 40 | 9 | 128 | 0.0703125 |

All four networks, tensors and ranks reproduced exactly. Every support matched
independent factor-base enumeration, and every nonzero completion count was
one. The frozen gate advanced to a matched-null implementation. The descriptive
`log2(best rank)` slope against field bits is `0.7509775004`; it is not an
asymptotic estimate.

## Null sequence

### Support-matched sparse null

Thirty-two deterministic unit-valued supports per cell preserved quotient
width, equality-class multiplicities and support count. The frozen sparse-null
gate advanced:

| bits | natural rank | null Q1 / median / Q3 | nulls at or below natural | natural / median |
| ---: | ---: | --- | ---: | ---: |
| 4 | 2 | 4 / 4 / 4 | 1/32 | 0.50 |
| 5 | 3 | 5 / 5 / 6 | 0/32 | 0.60 |
| 6 | 6 | 12 / 12 / 13 | 0/32 | 0.50 |
| 7 | 9 | 26 / 27 / 28 | 0/32 | 0.3333 |

Thus support sparsity alone does not explain the natural ranks.

### Typed topology null

The first topology null swapped complete input bundles between definitions
with identical region, tag, typed predicate and port shape. Every swap changed
only left-internal consumer endpoints and updated the acyclic functional
program. Ordered predicates, degrees, interface, pins and cut shape remained
identical and every selected rewired predicate replayed exactly.

The unrestricted 32-draw panel was **inconclusive**: 127/128 rows had support
zero and one had support one, so no row matched a natural support stratum.
This is null-language annihilation, not a favorable rank effect.

The follow-up dose grid was frozen before capped data. At the primary one-pair
dose, all four fields matched the natural stratum and every conditioned rank
equaled the natural rank exactly. The frozen gate therefore rejected the
minimal typed topology mechanism:

| bits | natural support/rank | cap-1 support range | conditioned support/rank | rank ratio |
| ---: | ---: | ---: | ---: | ---: |
| 4 | 6 / 2 | 6--6 | 6 / 2 | 1.0 |
| 5 | 8 / 3 | 6--8 | 7 / 3 | 1.0 |
| 6 | 19 / 6 | 19--19 | 19 / 6 | 1.0 |
| 7 | 40 / 9 | 39--40 | 40 / 9 | 1.0 |

Conditioned cap-2 and cap-4 rows also had rank ratio 1.0 in every field; cap 8
had no matched row. These data reject the claim that the observed low rank is
a fragile consequence of the tested global typed wiring. They instead point
to local curve/hash geometry and equality-variable identification.

## Complete-cost interpretation

Collapsing equality copies is exact and useful engineering, but it is standard
variable alias elimination. By itself it does not change an index-calculus
exponent. The one-left-slot tensor is the factor-base membership indicator;
precomputing it still costs at least its support/output representation, and
this panel deliberately used half-density tiny factor bases rather than the
asymptotic relation-collection regime.

The raw-width normalization is also not an admissible speed claim: duplicated
fanout edges inflate the denominator. A solver would need a compact quotient
message that composes across multiple slots, plus complete setup, relation
yield, linear algebra and precision costs. None was established here.

The full draft E1 null remains open because arbitrary left-interior rewiring
destroys the parsimonious program and requires an independently exact
projected model counter. Ordinary SAT decision is insufficient: every
boundary value is a projected model count. A source-bound CNF/finite-field
encoding, projected-count checker, and matched-null completion gate are the
remaining obligations if this route is pursued further.

## Evidence identities

| Artifact | SHA-256 |
| --- | --- |
| [`rank1-semantic-cut-shape-preflight.json`](rank1-semantic-cut-shape-preflight.json) | `b8e62d1e425407ebf79b9d3acc9d97be8fc59da4294448d506d649d5061f419a` |
| [`rank1-semantic-cut-quotient-preflight.json`](rank1-semantic-cut-quotient-preflight.json) | `c8196bccfcb6057bb311748915a838ec9e95797d75222be7521dbac8d7b44d61` |
| [`rank1-equality-quotient-heldout.json`](rank1-equality-quotient-heldout.json) | `045eb440e4a4348930b07f63418b2c98dfb8264e1771b7206bbfaa8c2bb4b0b6` |
| [`rank1-support-matched-sparse-null.json`](rank1-support-matched-sparse-null.json) | `54fb67930ebc3fec9d21a1d79fcf9c6fe62cf9dbccc4f7dfdcc9b205e0699ef6` |
| [`rank1-typed-functional-rewire.json`](rank1-typed-functional-rewire.json) | `00f7f86c5dfaf6e26eccdabbef2791f34c62cd00c8d25502156a867364fffa02` |
| [`rank1-typed-rewire-dose.json`](rank1-typed-rewire-dose.json) | `952f6e70a44602c4cc53bd9da9295819c6840e1474504d9b590a1ac6a5bd9529` |

These are exact local development and held-out structural records. No timing
was collected, and no ECDLP, relation-oracle, asymptotic, or novelty claim is
made.
