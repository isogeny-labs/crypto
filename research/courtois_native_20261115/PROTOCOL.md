# Native Courtois study

The request is to execute the five workstreams with the existing Rust code in
`isogeny-labs/crypto` and `isogeny-labs/cryptanalysis`, replacing the earlier
standalone Python execution path. Historical Python artifacts remain outside
this checkout at `/workspace/courtois_study` and are not native evidence.

## Frozen implementation and inputs

- Crypto source before this change: `b42bc6d1ae547d9cbc84d8a512c894db396c89af`.
- Cryptanalysis baseline: `9f467175fd5a0fe8fe6623a8715f9cb80e987305`; the replay
  uses the compatibility correction published at
  `1af49ed8489a8cf95ccae0ff3a4e1419b79c8bd5` (cryptanalysis PR 1572).
- `fixtures.json` preserves the exact field polynomials, coefficients, subgroup
  orders, cofactors and generators from the earlier public known-answer study.
- Field and group arithmetic: `semaev_decomp::Gf2` and
  `koblitz_fast_arith::FastBinaryCurve` (supports general b in point lifting).
- Binary elimination: `gf2_elim::rref_counted`; relation elimination:
  `ic_boundary::IncrementalGauss`; reference walk: `pollard_rho_dlp`.
- Point/scalar correctness replay uses the separate existing
  `binary_ecc::curve` representation. Cross-repository field replay uses
  cryptanalysis's `hashed-ecdl-tensor::binary::Field`, in a separate executable.

## Questions, controls, observations and bounds

1. Reproduce the trace constraints and common-divisor multiple search. Compare
   pair results with complete full-point enumeration, including zero-coordinate
   and identity targets. Compute rank rather than assuming bijectivity.
2. Retain the existing relation solver's row operations, scans and pivot
   inversions, native group counters, every failed attempt, binary elimination
   word XORs and nullity. These are separate units; do not add them together or
   rank whole pipelines from partial pricing.
3. For three, four and five terms, compare serial trace search, ordered-prefix
   symmetry and pair-table results with exact reachable-point sets.
4. Cache and symmetry variants preserve the fixed coordinate base. Charge
   cold pair-table construction. Bound the matrix cache at 256 systems; its
   eviction clears the cache deterministically. Budgets: 1,000,000 pair calls,
   1,000,000 prefix trials, 10,000,000 affine candidates, nullity at most 16.
   Exhaustion is an error, never a proof that a decomposition does not exist.
5. Recover known-answer targets without passing the secret to the native
   pipeline. Verify the scalar in the separate existing curve representation.
   The generic three-partition rho is a correctness control, not a claim to be
   the strongest available benchmark reference.

Local Cargo builds and unit tests are development checks. Do not classify them
as an admitted performance experiment. Native timing requires the checkpoint,
Iso Lab resource/publication/admission, matching strong-rho, independent replay
and PCC gates. This correctness study does not preregister a performance claim.
The proposed native follow-up has a total ceiling of USD 2, one build and one
900-second run, at most four worker CPUs and 16 GiB RAM, with no automatic
retry. It remains unlaunched until the repository's checkpoint, publication
and resource-admission requirements can actually be met. Unknown remaining
budget or missing identity blocks launch; it never licenses another backend.

## Acceptance and reporting

Retain failing cases and raw command output. A development pass is a producer
check and remains `[pcc:unverified]` until the independent gate gives standing.
Native implementation, correctness checks, cross-repository replay and admitted
performance measurement have separate statuses. Do not substitute this study
for a 2^(n/3) full-ECDLP theorem or reuse historical Python timings as native.

## Sources checked

Research KB is unavailable in this runtime: no MCP connection and
`scripts/research-kb status` reports no executable. Read the maintained primary
Rust modules above and the photographed Courtois 2016 equations. ecdb access
is unavailable without the configured AWS secret; the fixtures remain pinned
inputs rather than new catalogue assertions. No novelty claim is proposed.
