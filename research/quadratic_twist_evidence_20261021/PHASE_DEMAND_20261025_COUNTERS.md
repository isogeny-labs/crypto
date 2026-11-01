# Pre-execution counter contract

This addition fixes the accounting semantics before the first native research
execution. C++ and Rust have been compiled only. The controlling protocol is
`PHASE_DEMAND_20261025_PROTOCOL.md` at
`d25dd8cfe` (full commit recorded in validation).

All29 counters are nonnegative integers, emitted even when zero. They are
algorithm requests, not CPU instruction counts or equal-cost operations.

| Counter(s) | One counted request |
| --- | --- |
| `solver_calls` | one execution of the complete certificate-seeking solver |
| `membership_evals`, `constraint_evals` | one membership-polynomial or reduced-S3 evaluation in the finite root enumeration, including failures |
| `eval_term_visits` | one term visited by either such evaluation |
| `basis_calls` | each initial matrix build or lower-degree promotion/membership rebuild, including an empty matrix |
| `generated_rows` | one admissible monomial multiple of a nonzero pool row |
| `dense_column_slots` | number of degree-at-most-D columns for each generated row; a demand metric, not a sparse allocation measurement |
| `monomial_copies` | each polynomial or witness term copied into a generated row |
| `pivot_normalizations`, `scaled_terms` | one polynomial-row normalization/inverse request, and each polynomial/witness term scaled by it |
| `subtraction_steps`, `term_updates` | one polynomial-row forward/back elimination, and each source polynomial/witness term processed; membership-only subtraction also contributes its polynomial terms |
| `membership_subtractions` | each nonzero pivot subtraction when checking a proposed row against the lower-degree space |
| `evaluation_entries` | one actual-root/monomial matrix entry generated |
| `evaluation_row_updates`, `evaluation_row_scales`, `evaluation_inverses` | each scalar entry updated or scaled, and each pivot inverse in the prime-field triangular evaluation-space calculation |
| `linear_row_updates`, `linear_row_scales`, `linear_inverses` | the same requests for the subgroup-order augmented relation matrix; these remain distinct from field43 requests |
| `linear_backsub_updates` | each coefficient term processed in the two-log triangular back substitution |
| `point_add_requests` | a group-law call, including identity, equal-point and vertical branches; its field cost is not inferred from the count |
| `point_negations` | a negative-integer scalar routine's point negation request |
| `scalar_iterations` | one binary scalar loop iteration, with a conditional accumulator addition followed by a doubling even on the last bit |
| `signed_lifts` | one returned ordered root/base-index/sign-pair tested as an actual point sum |
| `map_multiplications` | the two coordinate multiplications of the explicit F43 source-to-twist map for a nonidentity input |
| `membership_builds`, `remainder_builds` | one factor-base polynomial construction or one reduced-S3 remainder construction |
| `scalar_recoveries` | one integer combination of verified base logs and the target shift |

Point arithmetic is the old producer's actual group law, independently
reimplemented with Euclidean inversion in Rust. Scalar counts use the same
declared binary chain in both ledgers; Rust's separate old semantic auditor
retains its linear scalar chain and batch RREF outside the charged model.
The recovery query charges all four signs for every returned ordered root,
including after its first successful lift; it stops only between queries.
Source public-point generation, source/certificate hash checking, independent
audit, JSON serialization and the fixed finite-field table constructor are
outside the demand interval. Their costs are unpriced rather than zero.
Likewise sorting, allocation, map traversal and internal exponentiation cost
are not converted to time. No scalar total or complete attack cost is claimed.

Setup is executed40 times, once per role/policy, and is target-independent.
The400 separate online/verification records reuse those recorded setups.
Each record's `cold` vector is its setup+online+verification demand, with the
setup added explicitly; summing these hypothetical cold vectors does not mean
400 cold setups were actually executed or admit a multi-target speed average.
The source point and its known scalar are created outside the recovery call;
only the point is passed into recovery.

The complete compiled source will be committed before the first execution,
fixing every loop, row/pivot order, sign order and counter placement. A later
implementation correction must retain the failed attempt and its source.
