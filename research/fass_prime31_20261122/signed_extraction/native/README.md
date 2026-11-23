# Signed three-summand extractor

This standalone Rust development component consumes the unchanged quotient
query engine and returns every ordered triple of admissible factor-base points
whose sum is the supplied target. Repetitions are allowed. The identity is a
valid target and is excluded from the factor base. This directory contains no
Python arithmetic, solver, replay or numerical harness.

## Interface

`extractor::extract(&Input)` accepts only p=97 or p=2147483647, canonical
nonsingular short-Weierstrass coefficients, and a power-selector degree
1<=d<=128 dividing p-1. Finite target coordinates must be canonical and on the
curve. The selector is exactly the roots of L(z)=z^d-1, found through a bounded
primitive-root search. Exhausting that search is an error, never an empty result.

The signed base consists of every distinct rational point above the selector
roots, intersected with an optional exact cyclic subgroup. A subgroup specifies
a finite generator and exact order 2..512. The constructor enumerates the cycle,
rejects an earlier identity or failure to return at the declared order, and uses
the actual point set for membership. It does not substitute the weaker test
[n]P=O for membership in the chosen cyclic subgroup. Targets outside that subgroup
remain valid curve inputs; an empty output is determined by the complete search.

The historical field name `restrict_liftable` means **admissible** in this new
API: when true, F is the product of the actual base abscissas, including the
subgroup condition. When false, F=L is a superset. For a finite target, G is the
complete rational radical of the right selector message. For Q=O, G is the
product of actual base abscissas in both F modes.

All positive quotient components are expanded using their supplied rational
root inventories and checked degree counts. Each (x,t) gets the complete
eliminated-coordinate gcd bucket. Every right t gets its own complete right
bucket. A coordinate may occur in several signed relations; neither the
cancellation-coordinate mask nor the first collision witness is used as a filter.

For every admissible signed first pair in a bucket, the extractor computes its
finite group sum R, checks x(R)=t, forms P3=Q-R, checks the exact signed base,
requires its right backpointer, and replays the entire three-point sum. The
separate identity-prefix chart emits (P,-P,Q) exactly when Q belongs to the base.
No such chart exists for Q=O because the base excludes the identity.

## Output contract

Base points are sorted lexicographically as `[x,y]`. Every ordered triple stores
three indices into that base; the parallel `routes` vector records its chart and
bucket indices. A finite prefix has chart=0, t and both backpointers. An identity
prefix has chart=1 and all three optional fields null. Left buckets are sorted
by (x,t), right buckets by t, and the ordered triples by their three indices.

Permutation groups retain every ordered triple index. Integer row groups use one
column per actual base x, sorted increasingly, with canonical y=min(y,p-y) sign
+1. The y=0 point has one lift and sign +1. A target already in the base is
subtracted from its own column; an external finite target has a distinct
coefficient -1. The identity target contributes no column. These are exact integer
rows and are not reduced modulo a subgroup order. Thus two copies of a
2-torsion point yield coefficient 2 even though their group sum is the identity.
Inverse-pair classification is computed from points and is not inferred from a
zero integer row. Row grouping is not a rank or useful-relation claim.

The output includes the quotient certificate, all root buckets, every ordered
triple, route, permutation group, integer row and operational diagnostic. The
qualifier serializes and hashes the full object for all 132 fixed cases. Exactly
42 predeclared cases retain the full certificate and input files; the others
retain exact serialized hashes and byte counts and can be reconstructed from the
frozen source. The serialization cap is 32 MiB per case; each retained file has a
64 KiB cap. A cap violation is a retained failure and does not change the panel.

## Construction and operation diagnostics

For d<=21 the unchanged binary Lucas norm circuit is evaluated in the tensor
quotient. The predeclared d=63 and d=96 cases use the unchanged direct selector
product, reduced first modulo L and then modulo F,G. This choice is fixed by the
protocol, not selected using outputs. Each call reconstructs its own selector,
base, subgroup and right message. No cross-case setup is reused.

Counters count the following source events; they are **not field-operation
counts or a complete cost model**:

| Counter | Counted event |
| --- | --- |
| `selector_search_candidates` | Bounded primitive-root candidate tried |
| `selector_pow_calls` | Power call in the selector search/order checks |
| `selector_multiplications` | Successive primitive-root multiplication |
| `square_root_calls` | Rational square-root request for base lifts or quadratics |
| `subgroup_additions` | One step of the explicitly generated subgroup cycle |
| `subgroup_base_checks` | Membership query for a candidate base point |
| `base_index_entries` | Signed base point inserted into its x bucket |
| `base_x_bucket_lookups` | Retrieval of a first/second-point x bucket |
| `right_quadratics` | Specialized right S3 polynomial constructed |
| `right_polynomial_products` | Right message multiplied by one factor |
| `right_radical_queries` | Complete Frobenius-gcd radical construction |
| `right_candidate_roots` | Quadratic roots before deduplication across factors |
| `tensor_ring_multiplications` | Multiplication calls reported by the unchanged tensor norm |
| `tensor_largest_reduced_slots` | Largest stored reduced tensor output in that constructor, not peak memory |
| `scalar_norm_factors` | Selector factors in the direct product route |
| `positive_block_root_tests` | Modulus evaluation at one supplied t root for a positive block |
| `positive_fibre_root_tests` | Specialized gcd evaluation at one supplied F root |
| `left_gcd_queries` | One complete eliminated-coordinate gcd for a support cell |
| `right_gcd_queries` | One right-witness gcd for a positive t |
| `witness_root_calls` | Quadratic root isolation invoked by a witness gcd |
| `zero_quadratic_branches` | Full selector returned for an identically zero quadratic |
| `left_lift_pair_attempts` | Admissible signed first-pair candidate examined |
| `prefix_additions` | Group addition used to obtain its first-pair sum |
| `residual_subtractions` | Group operation forming Q-R for a retained finite prefix |
| `base_membership_queries` | Signed residual/target membership query |
| `replay_additions` | Group additions in full three-point replay |
| `dedup_checks` | Ordered triple inserted into the exact output map |
| `coefficient_updates` | Summand/target update to an integer base column |

The quotient certificate also retains its existing split, inversion and
Euclidean-step counters. Sorting, tree comparisons, complete scalar field
arithmetic, polynomial construction and reductions, allocation, serialization,
root inventories, independent checking and cost calibration are not fully priced.
This ledger does not establish wall-time, throughput, complete cost or an
external comparison. Output-class counters (`affine_triples`,
`identity_prefix_triples`, `inverse_pair_triples`, `repeated_point_triples`,
`two_torsion_prefix_triples`, `masked_affine_triples`,
`masked_noncancelling_triples`, `support_cells_without_signed_output`) describe
sets, not operation costs. The cancellation mask applies only to finite prefixes.

## Qualification entry points

The executable binds the exact compiled `../PROTOCOL.md` bytes to its argument:

```sh
fass-signed-extraction-qualifier NEW_OUTPUT_DIRECTORY PROTOCOL_SHA256
fass-signed-extraction-qualifier --check CERTIFICATE_DIRECTORY [RAW_JSONL]
fass-signed-extraction-qualifier --check-controls CERTIFICATE_DIRECTORY RAW_JSONL
```

Builds and calls use immutable source and binary copies with separate pre-call
and post-call manifests under the parent protocol. The qualifier never replaces
an output directory or file. The reader uses an independently specified fixed
registry, checks exact inputs and all signed output with different arithmetic,
and checks both certificate and input bytes when raw records are supplied.
Neither reader mode calls the producer query solver or extractor. Its mutation
controls work on copies and do not alter retained evidence.
