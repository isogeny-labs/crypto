# Checked producer phase telemetry: final software validation

The checked phase implementation is validated at source
`b7c11d1b5f8a1f37b769453d3ce0413696b9fc83`. Its immutable release-library
executable completed **5231 passed, 0 failed, 123 ignored, 0 filtered out** in
the full unfiltered serial gate. This is software correctness and compatibility
evidence. It does not establish hardware/platform admission, whole-worker cost,
a completed scientific corpus, fresh worker S3 publication, or a fall-degree
result.

The change is stacked on the original native collector parent,
[PR #2801](https://github.com/isogeny-labs/crypto/pull/2801), branch
`codex/ml-pmu-collector-20261028`. At delivery preparation its head was
`843a31522de11e4d12f9a38990c1e2ec4a585fdf`. The producer contains the validated
native eligibility correction by lineage merge at `148a7ac47c3534f3c29722dda05a9232207eff30`.
The parent's later changes are documentation only and are not merged merely to
open the stacked PR. No feature-parent auto-merge, main merge, or retarget is
authorized by these local results.

## Resulting behavior

Every supervised checked attempt installs the exact 785-byte fixed policy before
the first partial snapshot and before worker launch. Each of the four phases
persists its start before opening counters or performing its operation, preserves
the original result and one operation invocation, and emits bounded diagnostics.
The producer checks native report consistency, phase ordering,
start/end identity, fixed calling-thread policy, optional cycles fallback,
cleanup outcomes and null/false admission fields. Streaming error length/hash
and a bounded preview avoid constructing a full error string for the summary.

Checked helper/snapshot profile v2 carries exact acknowledged artifact identities
and captures a dependency-closed policy/phase prefix. Previously acknowledged
files are rechecked for mutation or deletion. Frozen retries retain their exact
member set. Restore reconstructs AttemptBinding from the actual restored context,
checks the fixed policy and phase content, and retains artifact-only status.
Policy-present unknown/mixed versions fail closed. A valid start without an end
survives registryless collection, sealing and reporting as unfinished evidence.

The legacy writer dispatch and wire defaults remain unchanged. The no-policy
reader retains the former opaque-diagnostic tolerance while rejecting any decoded
top-level checked-v2 schema within the bounded discriminator. The discriminator
reads at most 524289 bytes, enough for every valid bounded v2 record. A malformed
prefix before a recognizable schema, or an oversized opaque diagnostic with a
later marker, is not validated telemetry and is never promoted by reporting or
public export. This compatibility boundary is explicit rather than a claim of
universal rejection of arbitrary malformed bytes.

## Exact source and validation matrix

| Evidence | Source and result |
| --- | --- |
| Final full release library | `b7c11d1b5f8a1f37b769453d3ce0413696b9fc83`: 5231 passed, 0 failed, 123 ignored, 0 filtered; serial, exit 0 |
| Retained interoperability tests | Same source: two exact existing tests passed separately; actual worker uses mock acquisition for all four phases; registryless valid-prefix test uses the existing synthetic start fixture |
| Final strict release Clippy | Same source: `--lib --tests --bin crypto -- -D warnings`, exit 0 |
| Checked contract / worker-report / snapshot groups | `27679c4e59bc38e8c18e34dbc9687f9a2d5eb317`: 12 / 4 / 6 passed |
| Dataset subset | Source `27679`: 180 passed, 0 failed, 1 existing ignored |
| Existing CLI integration | Source `27679`: 8 passed, 0 failed |
| Expanded native CLI compatibility | Source `27679`: initial v2 request before worker, frozen local helper replay and restore, separate synthetic start-only report, historical v1 helper/snapshot/report comparison all passed |

These counts overlap. The selected groups are contained in the dataset subset,
which is contained in the full library suite; they must not be added. The two
retention tests are existing members of that suite, not new scientific draws.

Source `b7c11` differs from `27679` only by 43 lines inside the existing
`#[cfg(test)]` module in `report.rs`: an opt-in bounded retention helper and two
calls. Every other path is unchanged. The production portion of `report.rs`
before `#[cfg(test)]` is byte-identical, SHA256
`34009b2b53017ccd9ba4cdafd2b85844d9c7aee05fbd1c92c11936d2fc5502b4`.
The final delivery changes only documentation/receipts. Runtime, dependency,
build-input and test equivalence to the tested commit must be checked through
the actual PR head; documentation does not change an executable's embedded pin.

| Immutable executable | SHA256 and binding |
| --- | --- |
| Final library test executable | `1e7496533221060ea6202ff4fc78c80b0d5a11cb1bdabf1bfb68c430551eec8b`, source `b7c11` |
| Earlier library test executable | `e3057678558120c6a324e3d66fde6a75e8f8dd765f8686628fd65b6df86b0551`, source `27679` |
| Production CLI | `05861e81c0e69f5c489ddb91d2412bf5f2520d435330d70e65e1c6acc0b8b5ec`, source `27679` |
| CLI integration executable | `bb3ed19c90a4be7b40e265e5258d9b5dfff41b3d3ea56a93244ba7cf2c59033e`, source `27679` |
| Historical compatibility CLI | `968a316daab4084ef61fd46c779832b9ebd9e482e58492674bc82174c4b1cfb7`, source `58bd5ab64424f7710540e0aea502225de6e6c438` |

The fixed v1 phase fixtures establish reader acceptance and unchanged bytes, not
fresh dynamic writer byte equivalence. Actual historical helper/snapshot/report
outputs are compared byte for byte. The old telemetry/report/live/snapshot source
is identical to base `590b7668c964126b006f64ddfcc962094500f8c5`; unchanged writer
construction is separate source evidence. Existing software regressions may
incidentally open counters, as authorized after applicability review. New
contract and retained-worker tests mock acquisition. No new PMU campaign ran.

## Failures and interrupted validation remain visible

- The source `9f1d54818cd791a25d9beeb74840fd0b94bf2de9` compile failed with one
  missing borrow in the extracted collector. Its log is retained; `27679` fixes
  that compilation error and the two independent acceptance-review findings.
- Source `27679`'s eight-thread full run ended **5230 passed, 1 failed, 123 ignored**,
  exit **101**. The unchanged `sealed_root_censoring_cannot_be_promoted_by_manifest_tampering`
  test reported an already-live writer during lock reacquisition. Two isolated
  runs passed, including a syscall trace. The concurrent failure's cause is
  unconfirmed; no speculative lease patch or test exclusion was introduced.
- The following source `27679` serial run was interrupted with exit **143** when
  the test-only retention derivative superseded it. It has **no terminal test
  verdict**. Its executable/command/PID/signal/reason receipt is retained.
- The first retention orchestration script exited **127** after both exact
  native tests passed because its narrowed PATH lacked `rg`. The failed metadata
  step is retained. Only hashing/readback and Clippy resumed; the two tests were
  not repeated.
- The final source `b7c11` serial run passed with no test exclusions. Its recorded
  1665.08 seconds is diagnostic gate duration, not performance evidence.

## Durable artifacts and replay

All objects below are in `setec-astronomy-artifacts`, region `us-east-2`, under
`ml-dataset/20261028/software/checked-phase-telemetry/`. Their complete receipts,
including verification timestamps, are in [VALIDATION_RECEIPTS.json](VALIDATION_RECEIPTS.json).

| Archive suffix | Bytes | SHA256 |
| --- | ---: | --- |
| `dc0a2447aa6da09536c2f37b5326043907bfaca21190047008f3027bbdde85ad/validation.tar.gz` | 98682039 | `dc0a2447aa6da09536c2f37b5326043907bfaca21190047008f3027bbdde85ad` |
| `7beedbd6f194f41c275f10facdb92bae5100ff6a388d44aed33dcb38b5aab975/fixtures.tar.gz` | 86480 | `7beedbd6f194f41c275f10facdb92bae5100ff6a388d44aed33dcb38b5aab975` |
| `4615e5ffab61a424c276154e507e41e8756054e0d39d157a70f35804eb398feb/source.tar.gz` | 291571 | `4615e5ffab61a424c276154e507e41e8756054e0d39d157a70f35804eb398feb` |

Each used conditional PUT and ETag-bound fresh GET, followed by original-byte
comparison and archive/contained-file hash verification. The final archive also
verified its restored Git bundle and all 49 retained fixture files. It includes
both exact source validation packets, immutable executables, the original
parallel failure, superseded serial interruption, isolated diagnostics, earlier
compile failure, independent source-review packets and the partial-consumer
interface inventory. Initial uncompiled/pending checkpoints remain immutable.

After downloading the final archive, verify its stated hash before extracting.
In `payload`, run `sha256sum -c SHA256SUMS`; in
`payload/validation-b7c11/fixtures`, run `sha256sum -c ../fixtures.sha256`.
The source bundle is based on existing repository commit `590b7668`; verify it
with `git bundle verify` in a repository containing that prerequisite. These
are byte/integrity checks, not a new hardware measurement or solver campaign.

The retained roots are `validation-b7c11/fixtures/sealed-worker` and
`validation-b7c11/fixtures/sealed-registryless-prefix`. They preserve original
manifest members plus run/plan/protocol bytes. The fixture run's existing
`a`-repeated source and `b`-repeated binary placeholders are synthetic binding
values; actual producing source/executable identities are recorded separately.
The independent consumer receives these immutable fixtures and owns its own
audit/export/restore evidence. This producer document does not grant consumer
admission or assert that an independent partial-snapshot adapter is complete.

The retained CLI partial replay is one initial local snapshot containing seven
context artifacts and no phase records or committed events. A later synthetic
start was added solely for an unsealed report fixture. It must not be relabeled
as a restored phase lineage, sealed worker result, fresh S3 transport replay, or
solver-memory recovery. The separate 49-file sealed packet supplies the missing
sealed interoperability fixtures.

## Review provenance and remaining work

Independent source reviews identified and checked the FIFO-open race correction,
legacy admission boundary, acknowledged identity checks, authoritative restore
binding, actual entrypoint coverage, direct worker registry assertions and
authoritative AWS sentinel. Exact reports and follow-up hashes are preserved in
the final archive's `reviews/` directory. They are source reviews, not independent
hardware validation. Primary references were the Linux
[perf_event_open](https://man7.org/linux/man-pages/man2/perf_event_open.2.html),
[open](https://man7.org/linux/man-pages/man2/open.2.html),
[FIFO](https://man7.org/linux/man-pages/man7/fifo.7.html) and
[flock](https://man7.org/linux/man-pages/man2/flock.2.html) documentation, plus
[Serde container attributes](https://serde.rs/container-attrs.html); their role
and the unresolved lock hypothesis are recorded in the source/review notes.

Hosted checks and review must be assessed against the delivered PR head. The
parent owns later CI cadence and stack integration. The original USD200 total
cap, external job owners, deadlines, worker publication gates and unresolved
broader corpus requirements remain unchanged. T766's implementation scopes were
released after source/binary verification; documentation delivery uses T784 only.
