# Branch coefficient replay: partial native result

Eleven compiler calls completed successfully: the ten canonical prerequisites and Nonbranch. The twelfth call, BranchConjugateOrders, exited with code one. Four unsolved proof goals were reported at lines 56-59. The local axiom print for actual_branch_conjugate_orders includes sorryAx after those failures; that output is retained as failed evidence, not an accepted theorem readback.

The remaining six calls were not reached: BranchNormTransport, BranchRootMultiplicity, BranchCoefficientMultiplicity, IndependentControls, Readback and ExpectedFailure. Consequently the planned thirteen-name axiom audit and mathematical negative control have no result. The original candidate source remains unchanged; this run identifies an elaboration defect in a dependency that the prospective source review had not compiled.

Precheck and postcheck both passed, with equal recorded hashes for 16736 dependency files totaling 3111892318 bytes. All eighteen copied Lean sources match the frozen source hashes. Runtime pre/post records match the frozen runtime pins. All twelve launched compiler processes were reaped with no remaining process group recorded. All three native phases have captured terminal ownership in completion evidence.

The outer phase times were precheck 45.819173402 seconds, compiler 395.732865542 seconds and postcheck 33.282053872 seconds. The total attempt time was 474.944561981 seconds. The full 3600-second reservation is charged, bringing recorded campaign charges to 10740 seconds. Limits were one CPU, 8 GiB, no swap and 128 tasks. These are wall durations, not CPU measurements or a performance comparison.

The immutable archive includes every native and outer file, including raw failed output and compiled intermediate artifacts. File hashes were rechecked before and after archiving. This independent archival readback did not rerun a compiler. Shared-cache pre/post checks retain their stated limitation concerning transient concurrent changes; no independent binary kernel recheck or official status promotion is asserted.
