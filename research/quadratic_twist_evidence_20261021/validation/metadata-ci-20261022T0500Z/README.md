# Standards metadata integration follow-up

This record follows CI on PR head `3e442fd3310d192495e5277a278aa2a0c0aab605`.
The [native parity/audit](https://github.com/aburan28/crypto/actions/runs/39702418773),
validation gate and perf-history workflows passed. The catalog assessment and
the quadratic-twist Lean target also passed. The old benchmark/archive failure
was already superseded and was not revisited.

## PR-caused metadata failure

The [cover certificate job](https://github.com/aburan28/crypto/actions/runs/39702418810/job/119574523159)
passed its native constructor, order-certificate and standards checks, then
failed `python3 scripts/update_curve_standards.py --check` with
`standard catalog join is stale`. `cover-job-failure.log` preserves the failing
step, and `join-before.log` reproduces it locally (exit 1).

The Rust importer's 1,921 appended registry rows were compact JSON, while the
existing metadata join requires one-space indentation. No standard identity or
representation was missing. The fix makes future Rust imports use the expected
layout and reformats only the active catalog. All 4,493 model values and every
alias mapping remain identical. The native cover checker changes only the
registry digest in its report and graph. Its equations, memberships, exact
identities, partitions and support statuses remain unchanged.

Refreshing the metadata display also adds the 1,921 missing roster entries.
The roster's extension-field branch now renders `GF(p^k)` for all 2,287 odd
extension models. Every measured leaderboard entry, protocol and non-registry
source field is unchanged. Canonical alias-index sorting changes its bytes,
but no mapping. No frozen measurement, benchmark history or validation gate
was edited.

The downstream metadata tests then exposed an additional PR-caused display
error: the browser joined coefficient arrays assuming strings; the native
import supplies integers. The renderer now stringifies each coefficient for
display, preserving the registered values. Its data index is regenerated for
all 4,493 models. The three catalog metadata tests and three sparse-input
metadata controls pass. `verify-preservation.mjs` checks every prior browser
curve row and all methods, factor bases, sessions, yields, rounds, candidates
and unrelated source digests unchanged; only the 1,921 new curve rows and the
four dependent metadata digests are added/refreshed.

The first corrected-renderer check reached a legitimately sparse-excluded Git
blob. The source inventory names 3,637 off-disk metadata paths and 3,352 unique
objects, of which 3,310 were missing. The first explicit fetch failed; the normal
promisor-fetch options succeeded (exit 0). The generator's `GIT_NO_LAZY_FETCH=1`
guard and missing-input test are retained. Both failures, fetch attempts and
the final complete checks remain in this directory. No frozen source file was
changed to resolve an object-cache issue.

`preservation.json` is produced by `verify-preservation.mjs`. JSON number tokens
are preserved with Node 24's `JSON.rawJSON`; no large integer is rounded through
a JavaScript number. The receipt compares the complete registry values, alias
dictionary, geometry reports, graph, existing roster values and frozen board
fields. It checks the earlier report/source/run paths for modifications and
binds all three predecessor and current catalog inputs by SHA-256.

The assessment retains exact Git-blob dependencies. The original
`a11984a573441d5bf0d60ab1d26cfc34e71badf5` blobs remain mandatory and hash-pinned;
the corrected inputs are pinned at a descendant commit. No byte-identity check
was replaced by semantic comparison. The source-linked current
[report](../../reports/metadata-20261022T0500Z/README.md), diagrams and PDF are
versioned separately from every earlier report.

## Independent inherited failure and review access

The [SEA job](https://github.com/aburan28/crypto/actions/runs/39702418749/job/119603074629)
built successfully, then Lean 4.19 rejected `--trusted` before running its axiom
auditor. `sea-lean-receipts.zip` is the original artifact (ID 12398779023,
SHA-256 `23c9969c6bfa10dba8f0aeb3d6fac2b224adb25663f2b0f231d70f2c6d056493`);
its unmodified files are extracted under `sea-receipts/`. Zero audited theorems
does not establish that the source contains no theorems or that a proof is false.

The entire SEA Lean directory, `tools/lean` tree and Lean workflow are identical
at all four revisions checked in `preservation.json`:

- Declared PR base `f8e72869e284a99f16d17b545f0d7666ef943c35`.
- Actual CI merge's base parent `412b60289db97ad1ba13a2bffe1120fadc7fb7d6`.
- PR head `3e442fd3310d192495e5277a278aa2a0c0aab605`.
- Tested merge `ca2236699e36cd8ef391e208059d03d6933020ad`.

This PR leaves those inherited sources and the global proof gate unchanged.
Repository maintenance must resolve the legacy-toolchain argument failure before
that gate can pass. No exemption or merge bypass is claimed.

`exact-head-checks-before.json` records all 89 exact-head check runs returned by
the complete check-run endpoint: 59 success, two failure, 24 queued, two cancelled,
one skipped and one neutral. This supplements the workflow wrapper, which lists
only pull-request workflow runs. Cursor's neutral check says its approval agent
could not start because the on-demand usage limit was reached. It supplies no
review approval. There were no submitted reviews or inline review threads.

## Validation and scope

The individual exit files and raw transcripts name the actual checks. Native
metadata/backend tests pass 40/40, including a layout regression with an integer
larger than JavaScript's exact range. `metadata-replay/` reproduces all native
model records and three certificates byte for byte, makes zero registry/alias
additions on the current input, and rejects a changed result before output.

`test-fresh-import.sh` imports the accepted native results into the original
2,572-model catalog. It reproduces the complete current 4,493-model registry
byte for byte and the same alias dictionary, proving the producer correction
rather than only formatting an already-generated file. Its outputs, source-bound
hashes and exit 0 are retained under `fresh-import/` and `fresh-import.log`.

The local standards metadata check now passes. Native cover generation/check,
native standards replay, the complete release library suite and 26 affected
binary tests are recorded separately. Deterministic assessment, Draft 2020-12
schema, independent full assessment and missing-extension-membership control
receipts bind the final sources and catalog. Every original strict per-model,
formula, partition, membership, semantic and digest guard stays enabled.

The final release suite passes 4,474 library tests with the same 119 existing
ignores, plus 11 cover-checker, two standards and 13 standalone binary tests.
`cargo-tests.log.xz` retains the full transcript and `cargo-tests.exit` is zero.
No additional ignore, quarantine or test exemption was introduced.

The first preservation command ran before the asynchronous native cover
generation finished and rejected the expected old registry digest. Its empty
stdout, stderr and exit are retained as `preservation-before-cover-generation.*`;
the final preservation receipt passes. No geometry assertion was weakened.

`commands.md`, `environment.log` and final checksums provide reproduction.
Compressed Cargo logs retain the original output. The report's three pages and
both diagrams were visually inspected. Diagrams and canonical cost graphs stay
unchanged because no population, support status or cost result changed.

Only the existing Python **metadata join, display renderers and metadata tests** were run: JSON
layout, alias assembly, roster labels and rendering already-recorded board
values. No Python field/curve arithmetic, experiment harness, research test,
certificate verifier or historical result generator ran. All cryptographic
generation/replay used the existing native Rust or accepted native receipts.

The earlier three-page report draft is retained under `report-before-browser/`;
the final version also records the browser correction. All earlier published
study and catalog reports remain byte-identical.

The recorded Conductor checks still exit 127; no task reservation or substitute local
coordinator is claimed. The twist embedding's Frobenius negation, the remaining
geometric bridges and shared Conductor access remain open in the
[problem record](../../../../problems/quadratic-twist-evidence-native-parity.md).
Current-head CI and review gates must pass independently; these local results
do not authorize a merge or establish general ECDLP hardness, GHS or a speedup.
