# Cost removed by reusing validated tower paths

Scope: exact source-derived multiplication counts for the retained Prime31
validation path, not a measured runtime or complete ECDLP cost. Source revision:
`559302c7df0cb411ce345b98b2650f33ee6e137d`. All claims below have unsigned PCC
standing. [pcc:unverified]

## Count the duplicated work

`native/src/oracle.rs` validates a full auxiliary coordinate through each map
`r(x) = x + tau/(x - xi)`. The counted inverse raises the nonzero denominator
to `p - 2`. For the fixed `p = 2147483647`, that exponent has 31 bits and 30
set bits. The loop performs 30 result multiplications and 30 squarings. The
following multiplication by `tau` brings the count to 61 per map. Additions,
subtractions, allocation and control flow are outside this multiplication count.

There are 1,024 coordinates and ten degree-two maps. One successful full-V
Vélu path validation, with no pole or earlier rejection, therefore contributes

`C = 1024 * 10 * 61 = 624640` field multiplications.

| Implementation | Fixed-path multiplications over q queries | Other costs |
| --- | ---: | --- |
| Legacy query wrapper | qC | Per-query validation overhead and query work |
| Plain prepared wrapper | C | Preparation and per-query target/limit checks |
| Candidate 1 fiber wrapper | 2C | Validation, second index traversal, index structure and extraction |
| Candidate 2 fiber wrapper | C | Validation, path capture, index structure and extraction |

Candidate 2 consumes the private `ValidatedTowerPaths` token produced by the
original validation. It removes exactly the second traversal's C multiplications
from candidate 1. Relative to q legacy validations, the fixed-path saving is
`(q - 1)C`, where q counts calls completing that validation. This comparison
applies to Tower/Tower4; Dense skips this path, and Dickson uses different
arithmetic. These statements do not claim that total cost halves or that total
query cost decreases by that fraction. [pcc:unverified]

## Work retained and the memory tradeoff

The token cannot be supplied by a caller or deserialized. Fixed field, base,
full-fiber, pole and terminal validation still precedes its construction.
Auxiliary kernel/coset certification remains part of earlier common setup;
the token carries the original-map full-fiber validation result.
The index then checks path shapes, full splitting, allowed leaves, parent
consistency and local degree. Its additional map-multiplication count is zero;
its wall time, allocation, memory and structural checks remain chargeable.
The public standalone `FiberIndex::prepare` still evaluates the maps itself.

With five paired-map blocks, 1,024 paths have six stored u64 endpoints each:
`1024 * 6 * 8 = 49152` logical bytes. Vector metadata, allocator overhead and the
retained index are additional. This is not a peak-RSS claim. [pcc:unverified]

The new fixed contracts compare prepared and standalone indexing reports and
public extraction results on the Vélu fixture and shifted-Dickson degree-two
and degree-four fixtures. The independent metadata review checks all104 cases,
source inventory and binary custody. It is a development correctness check,
not an independent timing experiment or signed mathematical proof.

## Complete-cost question left open

The comparison must charge auxiliary/factor-base construction, validation,
indexing, natural queries, F4, root extraction, signed witness replay,
relation retention, matrix reduction, scalar recovery and final `[s]G = Q`.
The maintained complete IC and optimized negation-rho controls need the same
accounting boundary. The bounded pilot determines whether saved traversal and
conditional extraction work repay their additional setup/memory costs. None of
the component identities decides that empirical comparison.

Evidence: `DEVELOPMENT_RECEIPT_v2.json`,
`evidence/qualification-review-v2.json`, `native/src/oracle.rs`,
`native/src/fiber_extract.rs`, and the prepared/fiber contract files in the
neighboring native package. Candidate 1's evidence remains unchanged.
