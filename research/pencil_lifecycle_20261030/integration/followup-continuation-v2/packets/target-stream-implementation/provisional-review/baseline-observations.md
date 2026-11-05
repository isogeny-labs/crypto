# Baseline inspection before patches

GraphSolver::relation assembles a single sparse row from verified points. y=0 is skipped; otherwise y<=p/2 maps to+1 and upper half to r-1. Labels add target-column b and RHS-column -a. Zero-equation replay negates RHS only. Ordinal must retain actual verified-candidate insertion order (walk_index,lambda), not reconstructed walk choices.

Collector::insert checks terminal status and malformed columns before mutation. Graph insertion precedes residual basis insertion. RejectedThreeLarge is not admitted. A cap leaves residual basis unchanged, but prior graph mutations are not rolled back; pending_constraint is retained outside the basis coefficient limit and admitted_rows does not increase. Do not generalize residual nonincorporation to graph rollback.

GraphSolver sets symbolic failure for caps and calls no later relation. Independent admission may produce scalar; scalar is verified at source kP=Q later. A returned candidate must not be advertised as verified at relation callback time. solve_one may terminate with walk_cap without scalar. Validation Result errors and assertion panics differ; terminal observer records must distinguish successful closure versus interrupted stream.

No-observer path currently has no row serialization/cloning beyond actual native row assembly. Any new capture must avoid resetting shared counters or invoking offline rank routines in the native process. Observer walltime/allocation may differ even when arithmetic counts do not; no source inspection establishes timing equivalence.
