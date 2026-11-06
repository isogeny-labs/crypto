# Native signed-collector restart coverage

Software validation, 2 November 2026. All 64 fixed controls completed and passed graph, group-label and scheduler replay. They contain four native walk-duplicate restarts and eight native chosen-duplicate restarts. Both restart paths now have positive coverage from actual collector traces.

The controls use the existing p=739 Scholten instance, generator seed 23 and target 2G. Collector seeds 0 through 31 were fixed before execution; each seed exercises affine/component and projective/target-rank configurations at candidate cap 256. All 64 controls and all raw streams are retained. No seeds or caps were extended after observing results.

The verifier checked 13,647 candidate transitions and 6,793 choices, including the restart state and random-draw reconstruction. Thirty-nine controls returned scalars, all verified on the source curve. Twenty-five controls reached the fixed cap without returning a scalar; those outcomes remain in the table. These are software fixtures on one known-answer instance, not distinct cohort targets or a branch-frequency estimate.

The single integration test passed in 118.11 seconds according to the test harness, within the prospectively fixed 180-second command limit. That duration is a software-check receipt, not an algorithm timing comparison. The wrapper used one Cargo job, locked offline dependencies and a 3 GiB virtual-memory limit. Source-binding and formatting checks passed. The raw archive contains 192 files: one header, stream and result for each control, with per-member hashes.

Arithmetic uses the existing affine Cantor checker; rank priorities rely on the strict replay's native graph score. Independent source-fidelity review remains pending. Native restart coverage supplements the earlier constructed exceptional controls and six saved streams; it does not establish a universal invariant. The full 1,260-target matched cohort and its recorded execution gates remain unchanged, and this report adds no rho cost ratio.

Runtime reconciliation in this turn found no containers on the local Docker daemon. The historical Q1 qualification remains failed and the replacement mountpoint recipe remains unassembled. This local observation says nothing about EKS job idleness. The migration receipt was unreadable due to filesystem permission denial. No container, cloud job or controller was started by this work.
