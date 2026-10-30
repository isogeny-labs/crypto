# Partial-consumer acceptance plan — proposed, not executed

Acceptance is software validation using finite saved or hand-derived local artifacts. It is not a generator, hardware test, experiment, fresh S3 proof, solver continuation, or training-data publication. Freeze fixture bytes, their checksums, source pins and expected reports before the implementation is treated as accepted. No test described below was run by this design task.

## Existing evidence and required fixture additions

Existing source-pinned producer fixture root:
`/home/ec2-user/ml-dataset-resume-20261028/checked-phase-validation-03/cli-compatibility`.

`candidate-anchor.json` plus `candidate-remote/` contains one ordinal-zero checked snapshot, seven context artifacts and 6564 artifact bytes, no capture delta and no phase start/end. It is a required interoperability fixture, not proof of multi-snapshot or phase coverage. The separately added `candidate-run/.../curve-construction-start.json` was added after that snapshot; it must not be silently inserted into the retained producer remote or cited as already restored. Existing producer unit-test logs lack reusable deleted TempRun bytes. A later retention-only packet may add fixtures only with its exact source/byte inventory.

No new producer execution is needed for this increment. Retain the existing immutable fixture unchanged. Add small independent static fixtures in the consumer crate: authoritative context copied from an admitted source-pinned fixture, new independently authored envelope/capture bytes, and phase bytes from existing retained checked-consumer fixtures or explicitly hand-authored mock-native records. Label the latter synthetic contract fixtures, never observed hardware/generation evidence. If a needed retained source is missing, author a finite synthetic fixture or leave that acceptance item incomplete; do not launch a producer to fill it silently.

Available phase-byte source: `/home/ec2-user/ml-dataset-resume-20261029/checked-phase-retained-fixtures-b7c11d1b/fixtures/{sealed-worker,sealed-registryless-prefix}`. Its 49-file `fixtures.sha256` manifest hashes to `df5d738c64f6b54ba4e696ac3cf0cc877d38a2fe6c605de09457eea02d1e651b` (manifest hash freshly read during this design; individual fixture replay was not performed). `PROVENANCE.md` identifies source `b7c11d1b5f8a1f37b769453d3ce0413696b9fc83`, a test-only retention derivative of production27679, actual worker software with mock counter acquisition, and synthetic run-binding placeholders `a*40`/`b*64`. The sealed worker has four starts/ends; the registryless fixture has one start. Reuse their original context and phase bytes without altering the source packet. These are not partial-lineage fixtures. In particular the retained `live/events.jsonl` begins with the older `{event,sequence}` envelope, not committed `crypto.ml-dataset.live-event/v2`; it cannot be relabeled as a captured delta. Independent committed-envelope fixtures remain required.

For every complete positive fixture save: the original anchor pin; complete catalog/receipt/manifest chain; all context/chunk/payload bytes; a fixture-provenance record; checksums; exact expected restricted report; expected I/O counts; expected restored path inventory. Expected report values are independently derived from the wire contract. Static digest vectors in `vectors/` exercise hashing alone and are not complete authoritative-context fixtures.

## Positive matrix

| ID | Fixture and invocation | Required observation |
| --- | --- | --- |
| P01 | Retained producer v2 ordinal zero, local restore then fresh audit | Exact seven artifacts/6564 bytes, policy-only checked prefix, capture not present, original metadata bytes preserved, all prohibited claims false |
| P02 | Independent legacy v1 ordinal zero, all five required context files, optional opaque-role context | Actual binding/canonical plan/config/protocol verified; no checked phase; legacy opaque role retained; no sealed output |
| P03 | v1 three snapshots, ordinary committed events split across two segments/deltas | Correct raw-line hashes, byte offsets, sequences and final cursor; all predecessors checked |
| P04 | v2 policy-only → start-only → successful end → next start, with observation-only terminal snapshot | Each prefix admitted only when valid; final interrupted/unfinished state; no inferred phase end |
| P05 | v2 start and recorded error end, terminal observation with no subsequent phases | Error digest/preview and native record checked; error state retained, no operation success claim |
| P06 | v2 complete four-phase recorded prefix with admitted unavailable counter states | Four ends may validate structurally; cell/science/complete execution/hardware/cost flags remain false/null |
| P07 | v2 finite saved records covering full acquisition, permitted fallback and descriptor reuse after close | Same checked-core lifecycle/projection decisions as sealed adapter; closed FD reuse permitted, legal zero IDs retained |
| P08 | Unsupported backend record, including big-endian request with no native calls/TIDs | Accepted diagnostic-unavailable structure only; no native success or platform claim |
| P09 | No delta versus an explicit zero-event genesis delta | Different report states `not_present` and `verified_empty_prefix`; only latter capture-present |
| P10 | Payload event with empty payload and a second nonempty binary payload containing non-UTF8 bytes | Exact artifacts once each, streamed hashes and totals; no UTF8 coercion of payload |
| P11 | Same cursor, no new artifacts, first stop; later unchanged stop/delta and then absent delta | Sticky stop and exact coverage persist; reported stop cause not independently established |
| P12 | Optional-null omission and explicit null where producer Option permits; reordered metadata keys | Equivalent normalized semantics; raw anchor/metadata hashes remain distinct and must match their own pins |
| P13 | Binding/raw-plan/canonical-plan separation and 1000-digit protocol integer | Raw identities and canonical commitments both verified with dedicated protocol parser; no lossy Value fallback |
| P14 | Local mirror relocated while its unchanged anchor declares the old file URI | Caller-selected object root used; original remote identity retained; zero network calls |
| P15 | Mock S3 backend, full chain with event and payload, HEAD then conditioned GET | Exact bucket/key/ETag and read order, bounded calls; local fixture mode does not report fresh actual S3 success |
| P16 | Fresh audit of restored tree after report file is altered to claim success | Auditor disregards prior report values and recomputes; output still has fixed claim boundaries |

P07/P08 are core-reuse checks, not a second hardware collector implementation. Freeze at least one fixture per state required by the existing checked adapter; that core's broader tests stay mandatory. All successful fixture audits must also inspect the exact output tree and report, not only an exit code.

## Rehashed negative matrix

Two layers are required. Transport corruption tests keep the external pin fixed and must reject byte mismatch. Semantic tests intentionally change an inner object and recompute every affected SHA/length/CAS reference/catalog/receipt/manifest/predecessor link up to a new externally supplied anchor pin. A semantic test that fails only because an outer hash was accidentally left stale does not count. Save the mutation description, new pin and expected earliest semantic gate. Use a separate finite fixture mutation helper; do not use verifier results to choose expected labels.

| ID | Mutation (all necessary outer hashes refreshed unless stated) | Required rejection point |
| --- | --- | --- |
| N01 | Wrong original anchor pin, same-length anchor corruption | Before destination creation or artifact I/O; fixed bookkeeping and source-root descriptor acquisition may precede pin verification |
| N02 | Duplicate decoded key (including escaped equivalent), unknown field, trailing JSON, wrong/null type, negative/overflow counter | Bounded parser; no artifact I/O |
| N03 | Unknown manifest schema, v1/v2 switch midchain, v1 policy/telemetry with misleading roles | Profile/context admission |
| N04 | Missing predecessor; ordinal gap; ordinal zero with previous; receipt digest cycle | Complete ancestry; no suffix-only success |
| N05 | Changed binding/run/attempt/remote between anchor/catalog/receipt/manifest/previous | Exact cross-binding |
| N06 | Catalog key substituted, CAS prefix/digest mismatch, uppercase digest, Unicode slash or `%2e` trick in remote | Reference/remote grammar before command |
| N07 | Receipt counts/bytes disagree; reordered objects relative to context+delta; extra object | Exact ordered membership/accounting |
| N08 | Duplicate artifact path; repeat an old path with equal bytes; file-as-directory conflict | Global namespace preflight |
| N09 | Traversal/absolute/backslash/control/empty path, overlong path or payload basename | Path admission before opening/writing |
| N10 | Missing one initial authoritative file; authoritative file introduced later; raw digest mismatch | Initial context binding |
| N11 | Rehash changed run source/binary, mismatching canonical plan/config/index/shard/protocol commitment | Actual context reconstruction, not merely manifest digest |
| N12 | Invalid/nonfinite protocol number or altered freeze digest with valid outer hashes | Independent protocol semantics |
| N13 | Observation time regression, terminal→running, running exit/signal, bad signal/reason limit | Observation rule, preserving other terminal combinations |
| N14 | Any true completion/restoration/science/coverage/admission flag at any envelope | Explicit false-claim gate |
| N15 | Coverage capture presence/count/end/stop disagrees with verified state | Exact coverage recomputation |
| N16 | Zero-event cursor not exact genesis; nonzero events without chunks; count/end monotonic mismatch | Cursor constraints |
| N17 | Wrong head schema/binding/cursor totals; stopped capture advances or changes/clears stop | Capture state machine |
| N18 | Segment filename/count/order/start/first sequence/predecessor digest mismatch | Segment boundary before replay |
| N19 | Empty segment, record without final LF, blank line, oversized line, invalid UTF8/envelope | Bounded exact committed-line replay |
| N20 | Event wrong schema/binding/sequence/predecessor; raw whitespace changed but old event hash retained | Raw-line chain (outer artifact hashes alone refreshed) |
| N21 | Segment last sequence/end/digest mismatch; final cursor/head undercounts/overcounts | Recomputed segment/cursor equality |
| N22 | Payload declaration missing artifact; artifact missing declaration; duplicate declaration; descriptor mismatch | Exact per-delta referenced set |
| N23 | Payload moved to earlier/later delta, unreferenced chunk, additional artifact | Same-delta exact membership and lineage uniqueness |
| N24 | Truncated/corrupt payload with fixed descriptor; rehashed descriptor but invalid namespace | Byte check / semantic path gate respectively |
| N25 | V2 policy missing or altered; phase at wrong path/role/encoding; policy downgrade hidden in telemetry | Fixed policy and profile dispatch |
| N26 | End without start; phase gap; start after unfinished/failed previous phase; mutated exact start hash | Prefix checked at each ordinal |
| N27 | Native/projection disagreement, impossible call/FD lifecycle, fake unsupported calls/TIDs, prohibited admissions | Existing independent checked-phase core |
| N28 | Corrupt earliest snapshot phase but valid final-looking prefix | Earlier-prefix check; later context cannot repair it |
| N29 | Uncommitted torn tail appended to admitted chunk or stray tail file in restored namespace | Exact bytes / final namespace; no tail inference |
| N30 | Report or helper ack claims completed/resumed even though chain is partial | Ancillary report is not trusted; final own output stays fixed or unexpected file rejects |

N20 must distinguish a valid new raw-event chain (all downstream event hashes updated, which may pass) from an internally stale event predecessor/digest (must fail). Hash-preserving semantic mutation is not required; supplying a new caller pin intentionally tests semantics under internally consistent transport hashes.

## Admission and TOCTOU tests

| ID | Deterministic test | Required evidence |
| --- | --- | --- |
| R01 | Exactly cap and cap+1 for anchor, metadata, object, line, segment, snapshot objects, chain, context | Bounded Reader reads at most cap+1; overrun fails before allocation/transfer dependent on it |
| R02 | Infinite input Reader and infinite stdout/stderr fixture child | Fixed read sentinel; child group terminated at bounded output/time; no drain of arbitrary suffix |
| R03 | Many small metadata objects crossing cumulative bytes/count; repeated CAS references | One cumulative ledger, no dedup discount to logical accounting; stopped before next admitted read |
| R04 | Deep/numerous tiny JSON values and long decoded keys | Preallocation/depth/node admission, no post-allocation-only accounting |
| R05 | Early refusal for canonical-plan clones, numeric canonical scratch, phase envelopes and output | Reservations occur before relevant allocator/test hook; child charges counted by parent |
| R06 | Multiple prefixes + final fresh audit exhaust parent while each alone fits | No fresh per-ordinal budget loophole; failure does not remove incomplete marker |
| R07 | Reused event arena across many events with retained payload/path identities | Arena capacity fixed/reused only after reset; retained nodes charged cumulatively; peak and cumulative models labeled separately |
| R08 | Checked arithmetic edge/u64→usize conversion and collection growth refusal | No overflow/panic/wrap; named resource error before allocation |
| R09 | Source FIFO, final symlink, ancestor symlink swap, destination preexistence, hardlink source | Reject unsafe namespace; never follow or write outside opened roots; no source-hardlink promotion |
| R10 | Same-size artifact/metadata replacement between initial hash and parse/second read | Consumed-byte digest detects substitution or opened immutable descriptor supplies the original; no stale check reuse |
| R11 | Remote changed ETag/length, HTTP412, corrupt GET, command failure, timeout, oversized file | No retry/fallback; incomplete marker retained; success flag false |
| R12 | Extra regular file, missing member, swapped metadata ordinal, extra directory/symlink in final tree | Exact final inventory rejects; previous report cannot mask it |
| R13 | Simulated write/fsync/final marker-removal failure | Nonzero exit; never print a durable-success report prematurely |
| R14 | Crash/failure at each staged rename boundary, followed by fresh command to same destination | Existing destination refuses; no automatic adoption/resume or silent replacement |

Where an existing AWS CLI download guard is polling-based, tests prove its enforced behavior and documented overshoot limitation; they must not call it an exact filesystem quota. No fault test may touch another task's source/destination/process or use a real cloud service.

## Leakage and compatibility gates

1. Snapshot report always has zero public rows and all false/null scientific, hardware, whole-worker and training-cost admissions. Compare policy-only, failed, start-only, unavailable, complete recorded-prefix, capture-stopped and terminal-running variants.
2. Change private witness/seed/outcome/native-counter provenance within fixtures while keeping independently valid bindings. Existing sealed public feature bytes remain exactly unchanged. No partial export option is added. Restricted report changes are expected and must not be copied into public feature objects.
3. Exact golden comparison of existing v1 sealed audit/export/fetch outputs and existing checked sealed audit output before/after the core extraction. Preserve `partial_snapshot_chain_verified=false` in sealed reports. Existing root-protocol large-number compatibility remains covered.
4. Standalone crate tests and strict Clippy only; no workspace/root rebuild unless the eventual scoped patch actually changes a root dependency. Run the actual new CLI through local restore and subsequent audit using an immutable executable, with hashes and source pin saved. Do not count helper-unit tests alone as CLI acceptance.
5. Mock transport tests inspect HEAD/GET arguments and receipts. Actual fresh-S3 verification remains separately incomplete until explicitly authorized and performed; local/mocked evidence cannot close that gate.

## Delivery evidence after implementation authorization

The future implementation packet must include exact frozen design/vector/fixture hashes; immutable source and executable hashes; scoped diff; full standalone test and strict-Clippy outputs; actual CLI invocation/exit/stdout/stderr; restored-tree inventory; report hashes; rehashed negative provenance and expected gates; independent source review of the new lineage/capture and core extraction. Any missing acceptance item is named incomplete. Existing local producer gate status, prior S3 archives and sealed consumer tests do not substitute for these gates.

Publication of that packet and PR work remain with the parent/assigned owner. There is no merge, cloud launch, fresh generator, worker-role escalation or spending authorization in this plan.
