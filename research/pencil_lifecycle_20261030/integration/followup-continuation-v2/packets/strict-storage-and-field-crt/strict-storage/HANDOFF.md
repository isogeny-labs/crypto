# Strict final storage interface

New producer work by /root/quintic_fidelity_review, following its independent review of published predecessor 55b56b1e04. This author does NOT provide an independent review verdict for these new changes.

Four new declarations preserve the existing assumptions/call wiring and expose construction facts already proved internally:
- step_exists_strict: additionally returns Normal(monic q), Normal r, and r.length < (monic q).length.
- initialized_run_strict: additionally returns Normal a, Normal b, and b.length<a.length.
- reduction_exists_strict: propagates those three fields through entry normalization/remainder.
- fallback_reduced_strict: propagates them through the two-EEA fallback endpoint.

All modules use unique names and new declaration names; predecessor source and caches are unmodified. StrictRun imports the original Run relation through StrictStep, so the strengthened result has exactly the existing execution relation.

Six modules compiled successfully, first attempt each: StrictStep, StrictRun, StrictEntry, StrictFallback, StrictControls, StrictReadback. Six named readbacks contain only propext/Classical.choice/Quot.sound or no axioms. Two controls show array capacity alone does not imply the strict assertion, and the identity storage [1],[] satisfies normalization and the strict assertion. StrictExpectedFailure returns 1 with decide proving 0=1 false. Every receipt records unchanged bound inputs. Dependency caches reused; no independent fresh dependency build or compiled Rust equivalence.

Artifacts: per-module attempt directories contain source snapshots, logs, before hashes, binding checks and JSON receipt. check.sh includes the published predecessor source/cache directory as the first helper root and snapshots its top-level Lean source/cache files. inventory.txt lists exact six readbacks. No accidental failed attempts occurred; the intended false theorem is retained.

No repository edits, cloud actions, scientific workload or changes to scientific authorization. Root should independently review these stronger outputs and integrate through the reserved repository workflow. The old independent review directory remains unchanged.
