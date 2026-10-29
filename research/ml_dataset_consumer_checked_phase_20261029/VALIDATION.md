# Independent checked-phase consumer validation

This increment checks supplied, sealed phase evidence. It does not run a producer,
open a PMU, create a cloud worker, establish hardware truth, verify full execution,
or admit training cost. Partial snapshot transport/lineage remains a separate
unfinished increment. Source starts from PR2816 at67629e36fc45775ad2166f80f0e7b9c0eab1d144.

## Source review and corrections

The initial implementation freeze was committed as2576cf88a6e before runtime
edits; conservative raw/scratch admission was amended in9f4bb40b720 before first
execution. Later explicit clarifications are additive in IMPLEMENTATION_FREEZE.
The independently authored wire inventory is bound to producer9f1d54818; reviewed
native/schema sources are unchanged at corrected producer27679c4e59bc.

Independent native/schema review found no actionable mismatch in its pinned
snapshots. The formatted implementation is byte-identical to those snapshots
after rustfmt normalization; saved hashes and normalization evidence bind that
statement. The reviewer did not author those modules or execute their tests.

Root protocol review found that Rust lower-exp formatting chooses different
shortest decimal tie digits from the required codec. The original failed
comparison (40 disagreements among199891 finite bit patterns) remains retained.
The implementation now streams the pinned serde_json nearest-even shortest
encoding through fixed stack storage, then independently normalizes the required
layout. Five exact tie regressions supplement the unchanged14positive/11negative
frozen protocol vectors. Root's correction review endorses the corrected source;
it does not relabel the original failed comparison as passing.

Integration review required a declared-size-plus-one bound on the legacy full
hash reread, explicit canonical-plan/config scratch reservation, and fixed-envelope
admission before parser/path allocation. Those corrections have dedicated early
refusal/infinite-suffix tests. The existing upstream Value reader's integer-range
limit also required a narrow fallback: only its number-range error enters strict
frozen-protocol verification, with reservations carried into the checked-cell
ledger. Actual CLI coverage accepts a1000-digit integer and rejects1e400 with
valid outer hashes. No global serde numeric behavior changed.

## Executed deterministic coverage

The corrected serial standalone release suite passed136library tests,
12checked-phase integration tests and7existing integration tests (155total),
with four opt-in retained-fixture tests ignored by the ordinary suite. A final
rerun after the last fixed-envelope ordering correction is recorded separately
in the delivery receipt. Release all-target strict Clippy and formatting are
required on that final source. The execution logs and executable/source hashes
are retained in the delivery archive rather than inferred from this test plan.

The new tests execute these concrete cases:

- Exact policy-only, all four completed/start-only prefix lengths, end-without-start
  and gaps, continuation after operation error, explicit full/truncated UTF-8
  error checks, and unavailable sealed records. Actual CLI audit runs with and
  without arithmetic; export requires replay or explicit preview.
- Zero ID/raw, exact floor scaling, overflow, invalid/running-zero tuples, read-ID
  mismatch, failed/0/31/33-byte reads, reset/enable/disable/cleanup failures,
  start/finish context changes, leader open/ID failures, optional member ID or
  duplicate-ID failure with cleanup-gated fallback, closed-FD reuse and alias
  rejection. Supported x86_64/AArch64 and unsupported recorded big endian are
  checked without hardware access.
- Strict nullable omissions at16nested positions, escaped duplicate keys,
  unknown fields, integer-as-float/overflow, malformed/multikey result tags,
  trailing JSON, invalid calls/attrs/claims, and exact bounded-array refusal
  before parsing an invalid first-excess value.
- File cap and cap-plus-one at16KiB/64KiB/512KiB; resource refusal before opening
  an absent oversized input; cumulative reservations; protocol allocation
  boundaries; and a synthetic infinite suffix whose reader consumes only the
  declared length plus one. Existing ordinary-file/symlink/FIFO/transport tests
  remain in the unchanged suite.
- Rehashed semantic mutations in16sealed-cell families with arithmetic both
  enabled and disabled, four raw-authoritative whitespace changes with stale
  bindings rejected and legitimate rebinding accepted, and source/binary/attempt
  identity substitutions. The canonical plan digest and raw plan byte digest
  are deliberately different in the whitespace controls.
- Missing policy hidden through role/encoding changes; sticky decoded schema
  detection across escaped/late/duplicate markers; opaque malformed/oversized
  legacy diagnostics; literal complete/start-only v1 measurement byte goldens;
  and unchanged public feature bytes under private checked evidence changes.
- Actual CLI local content-addressed restore preserves policy/start/end bytes and
  independently audits the restored sealed cell. An unsealed directory produces
  no checked row. This mock transport result is not S3 or partial-lineage evidence.

The written ACCEPTANCE table is a specification, not an executed-row count.
The finite suite does not exhaust all JSON mutations or prove allocation/RSS,
all native traces, or general protocol compatibility. Count exact executed tests
and retained inputs from the delivery receipt; do not count bullet items as draws.

## Preserved failures and pending evidence

The first parallel full suite passed133library tests but hit an unchanged
transport test's expected-error substring assertion. That exact test passed in
isolation, and the complete serial suite passed. The failure log is retained;
transport runtime was not changed. Preliminary integration failures were a test
output-name reuse and a wrong CLI option, corrected in the tests. The first
legacy marker test exposed loss of recognition through later duplicate-key parse
errors; recognition is now sticky and the regression passes.

The producer initially retained logs but removed its temporary sealed worker and
registryless-prefix fixtures. Those logs alone are not interoperability evidence.
The parent subsequently authorized the producer owner to retain only the two
already-required mocked test outputs; consumer interoperability remains pending
until their terminal source-bound packet is received and audited. No generator
rerun is authorized by this consumer document.

A progress checkpoint was conditionally uploaded and freshly read/hash-verified
at `s3://setec-astronomy-artifacts/ml-dataset/20261029/consumer-checked-phase/progress/a8576ad6d03ac0ebb01d5d0d30e1dfaab12590be58b9946c2155ef8723adea99/checkpoint.tar.gz`
(1404406bytes,70internal file hashes). It is explicitly marked in-progress and
does not replace the required final source/evidence checkpoint or PR gates.
