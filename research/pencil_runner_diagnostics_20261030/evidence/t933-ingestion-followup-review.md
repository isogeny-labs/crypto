# T933 follow-up source review, pending two strict-history corrections

2026-10-30 /root/four_lead_state. Read-only, no tests, solvers, plans, cloud or repository edits. Reviewed current working-tree diagnostics.rs and runner.rs after implementer history/IO/assembly changes. This is additive to t933-ingestion-initial-review.md, not a final build qualification.

Closed by source inspection: exact initial rational-table then cold-setup then candidate-setup then walk0 ordering; explicit walk-bearing boundary shape; epoch reset only cold→candidate; no post-return row; same-epoch phase-map fields/time/allocation monotonic; prior IO attempts/committed/published equal sequence and prior byte totals equal inspected JSON lengths; terminal temporary restricted to current or last published position; field/arithmetic counters restricted to u64 range. Valid degree-not-two walk→next-walk skips sieve. Solver return after a non-milestone candidate can follow the last emitted processing milestone, so the validation does not require a nonexistent final milestone.

The new actual assemble fixture constructs a complete synthetic SolveReport, invokes assemble for absent/present output and timeout/interruption, and asserts absent recovered/verification/accepted cost/phases/counters/detail/online. It visibly invokes no solver; fixture plan construction is only software preparation. Test completion is not inferred here while parent compilation is pending.

Two residual findings were sent to implementer and root:

1. Same-walk processing milestones only require greater-than;1→16 is accepted although producer must emit4 in between. A write failure disables future emissions, so it cannot explain resumed later milestones. Require adjacent fixed milestones.
2. A lastpublished .tmp/.json pair is accepted with differing content. Actual producer uses hard_link then unlink; if both survive the error they are identical bytes. Require same-byte digest/size for this pair; preserve a lone current temporary as incomplete without parsing it into accepted evidence.

Supplied source/protocol/attempt assertions remain outside actual Record fields and must stay identified as trusted Context provenance. Producer contexts remain rejected before workload construction and runner constructs None: implementation is dormant ingestion/custody plus active timeout helper, not enabled scientific instrumentation. Further semantic phase content and final-counter correctness are not certified; manifest correctly retains counter_semantics_verified=false and durability_confirmed=null.
