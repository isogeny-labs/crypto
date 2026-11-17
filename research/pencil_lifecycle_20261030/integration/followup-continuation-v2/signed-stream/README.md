# Native signed collector capture

The production two-choice collector now accepts an optional fallible `SignedObserver` through `solve_genus2_observed`. Existing entry points pass no observer. This package supplies an in-memory `Capture` with record and serialized-byte limits and deterministic regression controls. Its limits exclude producer/callback allocations.

Events retain header, ordered candidates, first-reference duplicate dispositions, identity/nonsplit outcomes, newly allocated node identities, exact signed edges and results, choices, restarts and terminal state. The header binds the actual source/transfer coordinates and algorithm parameters, but source revision/workload custody must be supplied externally. `source_verified=false` is intentional: the native solver returns a candidate; its caller must verify it. A callback failure leaves an incomplete prefix and does not promise rollback.

The signed equation is `su*xu+sv*xv=a+b*k`. Offline rows are `[A,-b,-a]` and node0 is known zero. New nuisance columns go before the target and constant columns. Tests replay the actual captured edges through a fresh native graph and separately compute whole-prefix ranks using the existing offline information package. Offline rank calculations are never inserted into the producer.

Reproduce the controls with `bash check.sh` from the repository root using the full path to this file's sibling script. `check-native.sh` runs all six existing follow-up tests through a scoped manifest that points directly at the original native library source. Both commands preserve before/after source hashes, exit status and logs. A fresh machine must materialize the path dependencies; caches are not part of the evidence.

These controls use one fixed known-answer target and seed. They are not the frozen 1,260-target cohort and do not measure timing or all costs. Duplicate, identity and restart hooks remain unexercised in these controls. The in-memory capture has no atomic-file publication protocol or independently authenticated custody; those and independent source review remain next integration work. Kummer-specific capture is still separate.
