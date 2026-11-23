# Adaptive combined-monomial recovery

The single-pair source is connected to the preserved quartic decoder through a consuming cursor. See DERIVATION.md for the invariant, assumptions, exact checks and remaining cost comparison.

From the repository root, `python3 research/primefield-companion-followup-20261102/feature-native-monomial-recovery/generate.py --check` checks generated source bytes without crypto execution. `run-record.py` builds/runs native tests (90s/60s caps); `check-record.py` builds/runs the separate record checker (60s per step). Use both recorders only in a scratch checkout: they overwrite evidence. The source lock includes Rust include! dependencies.

The separate checker imports no producer implementation; it checks the saved program and recovery records, not the full conditional transport proof. Claim PCC-20261107-16444b is unsigned. No matched timing comparison or cloud job.
