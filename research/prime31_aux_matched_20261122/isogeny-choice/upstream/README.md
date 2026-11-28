# Exact upstream source snapshot

The four Rust files in this directory are complete, unchanged Git blobs from
`isogeny-labs/crypto` revision
`33b3bc3c7485df0fe80aae41e2093cf940d87ff3`.

`manifest.json` records each original repository path, immutable GitHub URL,
Git blob identifier, local snapshot path, and SHA-256. It also records the 39
production/path-dependency files that are byte-identical between that revision
and the study checkout. These 43 files are the production dependency inventory
of the isolated crate; study source, tests, compiler, and registry dependencies
are separate parts of its build provenance.

`native/build.rs` verifies all 43 recorded hashes before selecting source. It
copies the full F4 parent and its unchanged children into a layout-preserving
build directory. For the other source modules it selects the declared exact
byte ranges, recording their offsets and hashes without inserting or rewriting
algorithm bodies. The fourth full snapshot, `groebner_f4.rs`, differs from the
lane checkout only by formatting outside its selected monomial-order span.

The snapshots include current finite-domain F4 root extraction and sibling
batching options, the counted negation rho variants, and the single-thread
configuration of `prime_fast::rho_parallel`. Importing these methods does not
establish which configuration is fastest, qualify a benchmark, or certify a
result. Correctness receipts and admitted execution records are separate.

## Upstream syntax defect and selected closure

The complete `ic_boundary.rs` blob at this revision is not valid Rust: its
`CountedGroup::x_sibling_pair` default has no body or closing brace, the following
`x_sibling_pair_counted` default lacks a closing brace, and the corresponding
`PrimeCurve` counted override also lacks a closing brace. These defects occur
in the raw upstream Git blob, not in a rewritten local copy. The snapshot has
Git blob ID `9d3bb77a9ae3b4c57efa054517a20650f4389d79` and SHA-256
`1f94ec22dbf7f865fd309e4fb770478dd555afd3282808143526fa775b81726d`.

The isolated crate omits the two unused source spans below through exact byte
selection. It inserts no braces, method bodies, or replacement implementations.

| Omitted upstream lines | Members | Byte interval, half-open | Omitted span SHA-256 |
| --- | --- | --- | --- |
| 1796–1819 | Two `CountedGroup` sibling-pair defaults | `[73674, 75151)` | `967ae3c02fe6c3638fb92ba287f5d14f103ea5f9d1843a2a72d192221a371853` |
| 7054–7065 | `PrimeCurve` counted sibling-pair override | `[265163, 265697)` | `927f8981e897c9c7821bc6b5643d32fc48096c3aa293bcf502e1679b0d4d2a7c` |

No reference to either omitted member occurs in the retained `ic_boundary`
closure, the other compiled upstream modules, the 39 unchanged production and
path dependencies, or the study source/tests. Existing `sibling_pair` and
`sibling_lift` methods and every selected rho backend remain intact. The
`selection_limitations` object in `manifest.json` records the exact anchors,
line and byte intervals, hashes, and dependency check. This does not claim
that the complete upstream trait, implementation, file, or repository compiles.
Qualification applies only to the declared independent source closure.

Do not format or edit these snapshots in place. An upstream update requires a
new exact revision, refreshed complete bytes and hashes, review of the selected
dependency closure, and fresh correctness qualification.
