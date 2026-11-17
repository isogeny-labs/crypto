# Independent runner publication v4 review

Read-only source review, 2026-10-30. Reviewed runner-publication-v4.patch SHA134a98c7cb37f795fff10089685bde0a36594ac2b24418334876a6578c1000af and runner-v4.rs SHA03fbcece81901111049ecbcba964b9ccebc06ec7d2dae1ad225caec82123a20f. No source application, builds, tests, solver or cloud execution. Prior v2/v3 reviews remain unchanged.

No new concrete source defect identified in the v4 delta. It addresses the quadratic full-journal snapshot publication and late capacity failure, without raising the8MiB cap or silently partitioning a plan.

## Capacity bound

The admission caller supplies the empty initial journal and actual session/runner/clock/scientific/protocol/source/binary identity. The helper serializes that actual header and a tail with actual escaped strings, fixed64hex prefix hash, and u64::MAX for seq, attempt and every numeric timestamp. Thus later sequence values including22000+ and all permitted u64 timestamps cannot have more decimal digits. Each tail keeps the same identity strings under the append invariant. Header already contains both brackets of the empty tails array; adding slots*tail_width and slots-1 commas is correct. Zero slots adds zero, and multiplication/addition overflow rejects. The bound is conservative, not a claim that the maximum timestamps actually occur.

The capacity gate occurs before the execution loop/first child, though some session preparation files are already created. It uses the actual execution count. A22680-slot single session rejects at admission; smaller planned sessions are separately checked with their actual identities. This does not itself authorize new session partitioning or demonstrate full-cohort readiness.

## Publication and failures

Each execution publishes its resource prefix, constructs a tail binding exact prefix bytes and observed publication interval, appends it to memory, and publishes that immutable tail once. The final compact journal is written once after the ordinary execution-loop Result, including error termination. If a tail-file write fails after append, the final journal can still retain the valid prefix-publication observation; it does not claim that failed tail-file publication completed. Prefix/tail files retained before a fatal kill remain separate recovery evidence, not a fabricated final journal.

Final journal failure is combined with an existing execution error rather than replacing it. It is published before subsequent terminal-session write, so it expressly cannot close that final tail. No ExternalExit or active-worker total is invented. Abrupt panic/kill and setup failure before the loop are outside the promised ordinary-result finalization path, as previously scoped.

V3's explicit reap_observed gating remains: successful wait observation is required for the ChildReaped mark and sidecar read. The original timeout and disabled launch gate remain unchanged. Clock coverage still starts at execute_session, not process startup/lock/spec/plan/host capture.

## Verification boundary

The new source fixture covers escaped identity strings,3240-slot admission,22680-slot rejection, arithmetic overflow and a one-tail serialized size comparison. It is not executed here. Successful compilation, a missing-reap fake-result regression, end-to-end failure-stage software fixtures and actual integrated journal custody remain separate verification tasks. No empirical timing, budget, scientific execution or all-cost admission is established.
