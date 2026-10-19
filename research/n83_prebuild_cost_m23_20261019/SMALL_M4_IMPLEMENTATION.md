# Native smaller-base m=4 implementation and validation

The protocol was committed at `ca8520a2` before code changes or runs.
`small_m4.rs`, included by `examples/n83_prebuild_cost_m23.rs`, uses the
byte-pinned existing K2048/D12 importer and native chained-S3 model. A
separate `examples/n83_small_m4_replay.rs` rereads the immutable archive
and checks the returned points with general field and group arithmetic.
It checks Frobenius/sign/coefficient labels, exact public-target binding,
the sparse row and rank-one condition, and changed-target, point and
coefficient controls. No natural relation is admitted from solver status
alone. No Python execution path was added or used.

The shell card checks preflight and independently replayed positive
controls for both bases before the K1,D1,D2,K2 public gate. A cap remains
censored; missing instrumentation or invalid output stops the card.
Progress observation covers root propagation and search, with an
independent hard deadline and durable phase/cap files.

## Build environment

The shared Git object volume was full on resumption. A fresh sparse clone
was created under `target/n83-resume-20261019` on SSD990-2, at the current
PR head `f35a2a2ea3335bab2e61a0e41a248bc820adbb72`; the four preceding
catalog/backlog fixes were preserved. Initial native test invocations
failed because sparse path dependencies had not yet been materialized.
Their logs are retained. Two subsequent invocations using the shared
Cargo target directory reported only the two historical worker tests,
omitting the newly added lift test. Those runs are not accepted as
validation of the new module. A new isolated Cargo target directory is
used for the authoritative example and full release-library gates.

The native worker source is committed at
`611aad60e4d9952d6239a3382a77184da3640eeb`. A credential-free incremental
Git bundle from the prior runner revision
`d4badc176820cb8bad63246ad8db02e47ae4cde1` has SHA-256
`8610cf8b0127622c612c787da04583b5befaeadfb06c7b04772309db55455b81`.
The runner checked this hash, imported the bundle, and checked out the
exact worker commit into a separate `source-small-m4` worktree. Native
build output goes into a new `target-small-m4` directory. A host card
must bind the final binary hashes and all inputs before experimental
execution; a successful build alone does not pass the control gate.

The [host card](SMALL_M4_HOST_CARD.md) and
[progress-check addendum](SMALL_M4_HOST_CARD_V2.md) bound those hashes
before launch at 18:54:41 UTC. Both exact import preflights and constructed
controls passed, with independent general-field replay on Linux x86_64
and macOS arm64. See the [control report](reports/run-small-m4-controls-20261019/RUN_REPORT.md)
and its sealed archive. The focused isolated suite passed three worker
tests and two verifier tests. The initial full-library attempt failed on
nine missing sparse compile-time fixtures, all subsequently materialized.
The second full run completed with 4,387 passes, eight runtime-fixture
failures and 118 ignored tests. Every failure reported a missing file in
the sparse checkout: the CryptoPro-B chain constants or archived n37
source-42 fixture. After restoring the exact tracked fixtures, the chain
suite passed nine tests and the archived-orbit test passed. The final
whole-suite rerun is `small-m4-release-lib-3.log`; earlier logs remain
retained and are not presented as passing gates.

The final full release-library gate passed **4,395 tests, zero failures,
118 ignored**, in 164.65 seconds. The isolated worker/verifier suite passed
**3 + 2 tests**. Both constructed receipts passed separate general-field
replay on Linux x86_64 and macOS arm64. Shell syntax, staged diff whitespace,
all 78 archived file digests, deterministic archive repacking and byte
identity of benchmark/report copies were checked. The rendered one-page
PDF was visually inspected. No test was skipped or disabled to repair the
sparse-checkout failures. These checks validate the implementation and
controls; the [public-card observation](small-m4-v1/STATUS.md) remains a
separate partial execution record.

## Inherited PR gate

At the 2026-10-19 18:31 UTC refresh, PR #2271's certificate job failed in
two `hyperelliptic-cover` catalog-count assertions: observed/expected
2,572/2,100 models and 140/134 binary generators. The same failures occur
on base PR #2246, at the same source lines and values:

- Current PR job: <https://github.com/aburan28/crypto/actions/runs/39290618359/job/118190010091>.
- Base PR job: <https://github.com/aburan28/crypto/actions/runs/39279241332/job/118150287891>.

The N83 change does not modify that binary or registry. These failures
remain required CI blockers; they are not waived by local N83 tests.
