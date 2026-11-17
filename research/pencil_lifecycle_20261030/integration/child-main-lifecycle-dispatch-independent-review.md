# Private child dispatch patch review

Read-only 2026-10-30. Patch SHA256 cd60428346749925830d0d6f53bb132b8de4f3633c7c810f398e8bec38e076e5, child-main-lifecycle-dispatch.patch. Unapplied source review only; no builds, solver calls, tests or repository edits.

No concrete defect identified in the proposed closure ownership/window refactor. solve_call is legitimately FnOnce: it moves allocator and workload/method IDs into the returned tuple/output. Each exclusive branch consumes/invokes it once. The mutable out borrow ends on consumption before subsequent diagnostic output writes; &job use is immutable. This is compile plausibility, not successful compilation evidence.

Absent context directly invokes the same callback. The original PMU/probe/scheduler windows remain around solve_measured; complete remains outside those nested windows but inside method_call_wall_ns. Optional lifecycle installation and explicit finishes surround the callback without changing those timer statements. New diagnostic_call_wall_ns includes helper validation, sink allocation, callback and explicit finish but excludes prior workload rebuild/allocator preparation and subsequent summary serialization/output/parent tails. It must not be called whole-child or whole-run cost.

Ordinary callback errors pass through explicit prefix/lifecycle finish and retain a summary when those finish calls succeed. Setup or finish errors return diagnostic errors; wall observation is assigned before error propagation. Existing method error behavior retains no method interval when its earlier ? returns. Panics are not caught here, consistent with the prior child behavior; scoped Drops clear ownership and do not manufacture finished records. A panic therefore has no returned child summary, and must be represented by parent reap/custody evidence.

All materialized ChildOutput construction sites were located: record.rs initializer and two runner.rs diagnostic acceptance fixture literals. Patch supplies the new optional fields at each. serde defaults/skip-none preserve old JSON omission behavior. No fresh field should be treated as accepted algorithm cost.

The early v2 child refusal is unchanged, so the proposed wrapper remains unreachable by real diagnostic child execution until a separate reviewed activation change. This patch alone neither enables execution nor demonstrates full accounting. Outer runner custody/resources and original workload hash checks remain separately required. No performance or scientific admission follows.
