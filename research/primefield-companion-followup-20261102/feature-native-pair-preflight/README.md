# One-call homogeneous-source preflight

A separate native executable supplies the runtime-planning call missing from the
matched homogeneous-source runner. It imports both existing generated recovery
modules and the same instrumented allocator/meter, without changing them or the
existing panel runner. Baseline and A/A select the scalar source; candidate
selects the paired source. One invocation performs exactly one selected arm.

The fixed public point is Q=(235,254) on the existing F509/r479 fixture. The
checker compares the returned scalar with173; recovery receives only Q and
performs its existing independent point-multiplication verification. The target
is fixed before any preflight execution and exercises the repaired character
feature. It is not selected from timing observations and is not claimed to bound
all targets or sizes.

## Interface and receipt

`preflight --describe` executes no cryptography. The execution interface is
`preflight --run-admitted-preflight ARM CONTRACT_SHA256`, with ARM one of
baseline/candidate/aa and exactly64 hexadecimal hash characters. The hash is an
external identity field, not proof of authorization, freeze or file integrity.
The caller must verify the actual contract, binary and source before execution.

The process flushes a start marker before entering the timed call. Its final
JSON receipt records:

- setup_prepare_ns: decoder construction plus prepared-state construction;
- query_verify_ns: one public-point recovery, including independent verification;
- cold_call_ns: meter reset, both phases, timing instrumentation, local return
  handling and destruction of decoder/prepared state before returning;
- returned scalar, checker verification, status and clock validity.

These intervals exclude process startup, CLI parsing, receipt formatting/output,
cloud queueing, deployment and upload. They are not the complete job cost. Fixed
and query intervals are contained within the cold interval, not additive to it.
Use external process/job wall time for the omitted work and preserve it separately.
The new executable has a distinct binary hash from the panel and cold arm order;
its durations are planning inputs, not comparative speed evidence.

A backwards clock produces null for that interval and prevents success. Setup,
preparation and recovery errors retain their failure stage and exit1. A wrong
expected label also exits1. Invalid CLI exits2 before decoder construction.
Panic, kill or timeout may leave only a start marker; absence of a complete final
record is failure, never a runtime estimate. Retain exit status and stderr.

## Validation and provenance

32 source/data dependencies are locked before each build. Historical matched
source locks remain unchanged. The selected tests exercise CLI acceptance and
rejection, fixed-fixture provenance, error/wrong-label classification and reverse
clock detection without calling elliptic arithmetic. The production executable's
describe and invalid-panel-CLI modes are checked separately. Imported crypto
tests are compiled but filtered out. See receipt.json and raw logs for actual
results; no cryptographic preflight or comparative panel has run.

The initial successful build had only a cold interval. Before execution it was
refined to expose fixed and per-query intervals; initial logs/identity are retained
as superseded development receipts. The final source-lock.json binds the current
source. The initial source is archived as initial-main.rs and verified against the
historical source hash. The current main.rs has the final source hash.

## Sources and execution dependency

Read the existing matched runner, recovery API, targets.tsv, comparison draft,
and experiment-protocol skill before implementation. Rust's Instant documentation
explains that checked_duration_since detects backwards timestamps while ordinary
elapsed can saturate; use checked subtraction here:
https://doc.rust-lang.org/std/time/struct.Instant.html (read2026-11-03).
Nanosecond representation does not establish nanosecond clock resolution; timer
steadiness and suspension semantics remain host-dependent.

The draft launch plan is not a canonical frozen experiment protocol or admission
receipt. Complete authoritative quota and charged/reserved allowance reconciliation,
publish pinned launch delivery and an admitted preflight contract, then run each
arm once. Preserve raw receipts and process costs, derive a documented conservative
runtime estimate, and bind it into the comparison protocol before freezing and
publishing that protocol. A single fixed target is an estimate, not a deadline
guarantee; retain timeouts and do not rerun to obtain a favorable estimate.
The original4563-case cohort and eight-target component-panel scope are unchanged.
