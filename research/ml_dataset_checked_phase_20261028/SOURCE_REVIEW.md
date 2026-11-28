# Read-only checked PMU integration review

Reviewed the proposed integration against producer commit 9ab6260a2aa57ca4df3729d6d17f10fefea312f5 and independent reader commit 88707e16b99f54976cfbcbe78c0f49379ab69da7. This is source/design inspection only: no edits to a worktree, builds, tests, fixture generation, cloud action or new write scope. The native F4 source and parent-owned full gate were untouched.

The scope, raw-versus-estimate labels, tuple overflow handling, optional sensor op-once behavior, and all-false hardware/whole-worker claims are conservative and consistent with the checked collector. I found four concrete details to resolve in the frozen integration contract and implementation.

## 1. Make each published snapshot dependency-closed

DESIGN.md:141-158 proposes appending optional phase files when each appears. Existing live.rs:295-325 independently scans each path. An interleaving can check a missing phase start, then the worker atomically installs both start and end, then the copier observes and includes the end. Ordering the start before the end in CONTEXT_FILES alone does not prevent this. The proposed end-without-start rejection at DESIGN.md:175-181 would then reject a correctly produced fast phase; acknowledging that snapshot without such validation would leave an invalid remote prefix if the host disappears.

Freeze a rule that each admitted end has its exact start and policy in either the previously acknowledged set or the current prepared snapshot, and each later phase has the schedule prerequisites required by the supported profile. If a dependency was skipped earlier in this scan, include it explicitly or defer the dependent record. Do not mutate an already frozen PreparedSnapshot on retry. Add a deterministic interleaving fixture that installs start/end between the two path checks, plus a policy/start interleaving and a kill immediately after acknowledgment. No synchronized hardware snapshot is required or implied.

## 2. Preserve roles for interruption before worker registry persistence

The worker writes worker-artifacts.json only at mod.rs:813-818. A panic or SIGKILL can leave durable telemetry but no registry. Supervisor sealing loads an empty registry at mod.rs:1153-1156 and assigns unmatched telemetry paths role unregistered-partial-artifact and encoding native at 1176-1186. The independent reader audit.rs:439-443 selects phase records only by the phase-start or hardware-and-resource-telemetry roles, so it silently omits surviving start-only records. Producer report.rs:394-409 instead uses the telemetry path and retains these records, creating a real producer/consumer discrepancy.

The proposed v2 dispatch alone does not close this path. Add finite exact phase/policy path-to-role recovery during supervisor sealing, with strict v2 bytes/binding validation, or explicitly support these known interrupted paths in the reader without trusting arbitrary generic artifacts. Test panic/kill before worker-artifacts.json through sealed-cell audit as well as partial snapshot restore. A start-only record must remain interrupted_or_unfinished and cannot gain a hardware count.

## 3. Specify changed-file checks after acknowledgment

DESIGN.md:145-148 requires copy-once plus changed-file/hash rejection. Current HelperRequest.copied_context and HelperResult.copied_context are only sets of names (live.rs:164,177); prepare_snapshot immediately skips a previously copied name at 296-297. It therefore cannot detect replacement or deletion of an already acknowledged local context file. For a newly observed file, 306-315 checks only read length against metadata, while the snapshot gets its own hash.

Freeze whether the rejection requirement covers later local mutation. If it does, retain/recover acknowledged Artifact identities and validate each existing finite context member against its recorded hash/bytes before accepting further snapshots. Bound the extra reads and preserve the old frozen request on retry. Existing immutable writers prevent normal overwrite, but that is not an implementation of the proposed changed-file rejection test. Do not weaken remote immutable-chain guarantees or reinterpret a mutation as a new version at the same path.

## 4. Complete the native status and request-shape grammar

DESIGN.md:93-101 omits EventStatus::IdentityFailed, which is emitted on event-ID ioctl failure or a duplicate group event ID (checked.rs:608-625), and is also an allowed optional-cycles preparation fallback trigger (514-531). Map it explicitly to unavailable identity/preparation diagnostics with no admitted interval, retaining the attempt. It is different from IdentityMismatch after a read.

Also freeze state-dependent exact request checks. The initial cycles placeholder is built with group_fd=-1 at checked.rs:538-548 and is replaced with the opened leader fd only when preparation reaches that member at 563-569. UnsupportedPlatform or an earlier instructions preparation failure can therefore leave a valid unattempted cycles record with leader-style disabled attrs and no actual open. For an attempted cycles member, group_fd must equal the successful instructions Open result in that attempt; it cannot be a fixed numeric field in an immutable policy. For an unattempted member, validate the native placeholder shape without claiming that a syscall occurred. Preserve the native Serialize Result wire form and allowlist all lifecycle/status variants in the separately written checker. Add valid unavailable/fallback fixtures before malformed-record rejection tests so strict admission does not silently discard unavailable samples.

## Confirmed claim boundaries and op-once semantics

- checked.rs:199-226 distinguishes raw observations, floor-scaled estimates and overflow; equal positive times and raw zero are valid. ControlFailed can retain a separately valid tuple assessment (837-839), so top-level lifecycle admission must win over raw tuple arithmetic.
- checked.rs:721-748 starts at most once per collector; optional fallback only happens during preparation. No interval retry is justified by a sensor failure. CallingThread is the owner thread, while RUSAGE_SELF is process cumulative usage.
- The proposed start-before-open sequence and FnOnce operation preserve the existing op-once behavior within one worker invocation. A final persistence failure must keep solver Result and persistence error distinct; it does not authorize re-executing the solver. Existing worker mod.rs:800-819 converts returned errors into a recorded error outcome and then exits successfully, so process exit code alone must not be interpreted as phase success.
- No restored PMU FD, opaque process memory, whole-execution count, independently measured hardware truth, or platform admission follows from restored JSON.
- Partial restore currently retains names under artifacts/context/... (snapshot.rs:1075-1108). Freeze this mapping to original telemetry/policy paths and audit through the partial-attempt adapter; do not pretend the restore is a complete cell with a manifest.

These are integration acceptance gaps, not findings that the existing standalone checked collector returned an incorrect measurement. No fresh hardware or software measurement was run for this review.
