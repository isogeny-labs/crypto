# Actual-curve parity gate for the paged binary pair index

Frozen on 2026-11-16 before implementing or running the new integration test.
Source ancestor: `4bbc8b51d46a4ff48d5ef995065d6b15ef59eda0`.
This checks the candidate index on actual curve pair-sum streams; it is not a
timing or end-to-end comparison. [pcc:unverified]

## Exact inputs and prior work

Use `koblitz_instance(0,n)`, `build_standard_subspace_factor_base`, and
`koblitz_factor_base(..., SignedFrobeniusOrbit, ...)` for `(n,d)` equal to
`(13,5)`, `(19,7)`, `(23,8)`. The full ICV1 curve IDs and base SHA-256 values
are pinned in `../rootfind_paged_pair_index_20261116/storage.json`, derived
from the committed collision-census JSON. Recompute and compare the three
base digests in the test if the API exposes a stable byte encoding; otherwise
compare exact abscissa counts, signed point counts and curve IDs and record
the digest gap. [pcc:unverified]

The current production torsion builder is
`PairTable::build_negation_folded_binary_batch_torsion_with_collisions` in
`src/cryptanalysis/ic_boundary.rs`. The candidate is the source-pinned
`../rootfind_paged_pair_index_20261116/paged_index.rs` prototype. The
rootfinding program paper
(`papers/ecdlp-index-calculus-program/sections/rootfinding.tex`, KB
`source:e9db8cc2be204429e8fb1daa12a3324c`) already describes binary
pair tables and dense-index engineering. This test claims no new relation
construction. [pcc:unverified]

## Fixed checks and stopping rule

- Reconstruct the torsion builder's ordered stream using the public curve
  group law: one nonzero first-row point per later abscissa group, then
  every `ai >= 1` first representative against each signed point in groups
  `ai..`, skipping identity sums. Insert each `s.x + 1` into the paged index
  and an independent standard hash map. The first witness and each duplicate
  tuple must match insertion by insertion.
- Compare the complete duplicate tuple stream with the production torsion
  builder's returned collision vector, and compare retained key count and
  insertion attempts. Verify every duplicate pair's full group equation up
  to sign. Check every retained key and the boundary/absent keys against
  the hash map. Record used 16-word pages and fixed page capacity per cell.
- Expected base abscissae are 17/70/138, signed points 33/139/275,
  retained keys 261/4,800/18,842 and production non-torsion collision
  counts 11/30/64 for K13/K19/K23. Any mismatch, group-check failure or
  missing source identity invalidates the candidate on that cell. Retain the
  raw output and exit status, including failure. [pcc:unverified]

Passing this parity gate does not rank the hash, flat and paged layouts.
The next production step, after Conductor T-6708 releases its source paths,
is a guarded method-parameter integration and same-target answer checks.
Only a separately frozen complete-call timing panel can decide speed against
the fastest comparable F4/F5/F6 arms. Conductor T-6342 still reserves
`.resume`; do not run a timing panel under the stale checkpoint.
