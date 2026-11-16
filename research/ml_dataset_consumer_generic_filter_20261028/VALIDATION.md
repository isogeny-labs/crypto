# Independent generic v2 consumer validation, 2026-10-29

The corrected native consumer at
`086b4cd590d2e871ca9e119c389470c9a9232f94` passes 113 release tests
(106 library and seven CLI tests), strict all-target Clippy and formatting.
Three opt-in fixture tests remain excluded from that ordinary test count.
The retained generic-panel and root-panel tests were then invoked explicitly;
neither test launches a producer.

The copied CLI SHA256 is
`31ac84b125fcdaa382df7650833d4295ec99a4aa87b9e49dcbd36e60ee4f0047`.
The copied generic fixture test SHA256 is
`759a58584ed4e367a0a352d389e7eddfddcb4168fb6b490a29ed6b3afb7d3125`.
All tracked native files were hashed before build and checked unchanged after
the build. Source, compiler versions, binaries and complete logs are preserved
in the software checkpoint. Later documentation commits do not change this
tested native source.

## Retained panel and limitations

Migration reconciliation found terminal receipts for all 32 primary cells and
all eight predeclared supplement cells. No producer cell was regenerated.
The 40 sealed records contain 26 verified outcomes, 11 supervisor timeouts and
three budget-exhausted outcomes. Those status counts are not 40 successful
solver computations. Every record passed the independent consumer audit.

The generic v2 arithmetic checks covered 24 completed batches, 22 positive
scalar recoveries, three no-recovery cells, one cell with multiple cumulative
batches and 337 completed rows combining multiple source relations. Four
verified Koblitz cells exercised the legacy v1 route. The original primary-only
report incorrectly counted an additional unsealed legacy file; the corrected
count requires the manifest-admitted role and a verified outcome.

All ten retained, rehashed semantic mutation cases were rejected. Exported
features were unchanged by the restricted-label sentinel control for each of
the 40 records. Independently checked results establish supplied finite domain,
query, source, filter-prefix and completed-rank properties only. They do not
establish factor-base construction, subgroup-order completeness, execution
completeness, total machine cost, UNSAT or first/last fall degrees.

The panel produced **zero sealed native-policy censored prefixes**. Eleven
outer supervisor timeouts do not supply that evidence. Low-level hand-derived
censor tests pass, but production-policy censor coverage remains a gap. The
40-cell maximum is exhausted; no extra seeds, relaxed policies or replacement
generation were used.

The exact retained producer executable at source
`58bd5ab64424f7710540e0aea502225de6e6c438`, SHA256
`968a316daab4084ef61fd46c779832b9ebd9e482e58492674bc82174c4b1cfb7`,
also replayed all 40 retained runs successfully in a separate CLI gate. That
gate invoked `dataset verify`, not generation. Its reports remain distinct from
the independent arithmetic results.

## Compatibility and admission

The previously archived 51-cell root/policy fixture packet was freshly fetched
from canonical S3 and checked against SHA256
`40d88d8d0bdfbe57ab8c201970d4ced5c49792c86eea552067e44726921c4809`.
All 1,738 contained hashes passed. The new consumer rechecked all 51 cells and
30 completed root sets, including the existing policy versions and rehashed
prime, binary and Koblitz root mutations. This reused saved cells only.

[ADMISSION_CORRECTION.md](ADMISSION_CORRECTION.md) records the parent review's
pre-allocation finding and its correction. Streaming classification binds the
actual file hash, rejects duplicate or mistyped schema keys, and requires EOF.
The v2 loader applies its 32MiB raw limit and a non-materializing structural
admission pass before allocating a JSON tree. Sparse scratch, dense rows and
column vectors have explicit modeled reservations. This 512MiB storage model
is not an RSS proof. Early-refusal tests cover an invalid unvisited suffix,
oversized v2 input, escaped schema duplication and empty-rank column storage.
The historical failed preflight/lint gates and the parent's original review
remain retained, rather than being overwritten by this successful gate.

Root Cargo, build, runtime and test files are unchanged from parent
`88707e16b99f54976cfbcbe78c0f49379ab69da7`; the saved diff is empty.
The parent's documented 4,987-pass root-library gate is reused only through
that equivalence. No fresh root-library suite or producer build is claimed.

## Remaining delivery work

The parent owns final independent review, PR publication, stack integration and
current-head CI. The software checkpoint uses the existing controller's
conditional S3 write and fresh download verification. It cannot establish EKS
worker publication rights or completion of the large training corpus. The
original USD200 total cap and external-job restrictions remain unchanged.
