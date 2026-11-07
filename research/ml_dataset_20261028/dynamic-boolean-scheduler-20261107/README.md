# Scheduler V2: per-generator and initial-task admission

Narrow correction following independent root review of V1. The V1 directory is unchanged. scheduler.rs adds exactly two admission calls: one before validating each original generator, including empty generators; one before enqueuing each original task. No algorithm, limit, queue order or proof semantics change.

The new zero-generator control uses64empty polynomials. Refusal at owner check5 must stop validation with5charged work units, no task/basis allocation or enqueue, and no completed tasks. Refusal at check70 occurs at the third original-task admission (after65validation/entry checks and2vector allocation checks); exactlyOriginal(0) andOriginal(1) remain queued and no execution starts. The exact check indices are intentional controls of these two formerly unadmitted loops, not a performance claim.

The existing six correctness/refusal tests and seven unchanged donor tests are rerun, including the1499point refusal sweep and exhaustive tiny-ideal tests. Authoritative result, exact source/binary hashes and unchanged readback are recorded in test.receipt.json. No shared source/build, dataset, cloud, F5, signed-search or original-limit changes. Simultaneous same-degree batch assembly remains the next implementation scope after review of this correction.
