# Repaired finite-count proof passes

Gate `uniform-lean-v3` checks the repaired source in
[ResidualMass.lean](../../proof/uniform-v1/ResidualMass.lean).
The exact launch and finish timestamps, source hash, source snapshot and
command are retained in `runs/uniform-lean-v3/metadata/`.

![Counting and algorithmic obligations](../../figures/joint-mechanism.svg)

The build and transitive-axiom gate pass. The audit covers 12 declarations:
the six named finite-count theorems and generated recursive equations.
There are zero failing and zero trusted-native declarations. Standard
foundations in the public theorem proofs are `propext` and `Quot.sound`.
The loose [audit log](../../results/proof-checks/uniform-v3/audit.log)
and build/status files preserve this result.

The theorem proves that all finite buckets together contain exactly the
candidate-list length when labels are bounded by n. Applying two bounded
labellings to the same list therefore gives equal total mass. It does not
assume or prove independent sampling, rank, a physical domain-size formula,
curve-map disjointness or runtime. Those are separate bridges in the
[proof map](../../proof/PROOF_MAP.md).

The command uses pinned Lean 4.34.1, one worker, 8 GiB, a 900 s watchdog
and the busy reservation. This is correctness evidence; no timing
comparison is admitted and the visual is explanatory.
