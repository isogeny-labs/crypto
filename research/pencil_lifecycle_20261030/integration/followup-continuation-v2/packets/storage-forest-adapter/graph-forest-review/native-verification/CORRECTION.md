# Native verification correction

The historical review used Python accept.py to check formal acceptance and Python gate.py to orchestrate Lean. Those historical files and outputs remain unchanged, but that verifier workflow does not satisfy the repository no-Python verifier rule.

This additive correction uses compiled Rust for receipt validation, exact ordered declaration inventory, allowed axiom checking, and source-byte comparison. A thin shell wrapper executes the same ten fixed Lean modules with native exit propagation, per-module stdout/stderr, fresh invocation storage, and SHA256 input bindings. No Python verifier was executed for this correction.

Retained receipt replay accepted 34 names. Fresh native replay-V1UMwTXB passed all ten modules and unchanged bindings; native-acceptance.json records 10 main declarations, 9 producer controls and 15 independent controls. Its lean_reexecuted=false field refers to the Rust acceptance checker itself: the separate replay.sh DID rerun all ten Lean modules.

Six native checker rejection controls cover gate failure, native module failure, wrong module name, forbidden axiom, empty inventory and missing declaration. The retained False receipt was checked. Fresh false-rxDSzErH additionally records an actual unsolved False theorem with native exit 1 and propagated shell exit 1.

Failure history: replay-rMU08zgF failed on the first module because Lean requires the source to be within its working-directory root. replay-v1-failed.sh preserves that wrapper. The corrected wrapper changes directory to its fresh snapshot. All failure outputs remain retained.

Historical reports and manifests are not rewritten. This correction supersedes historical workflow-acceptance claims only; the theorem inventory and mathematical scope are unchanged. No scientific execution, cloud operation, or repository edit occurred.

The first fresh negative-control invocation, false-0EcGnOA7, failed for missing Init in its search path; it is not counted as a false-theorem test. false-control-v1-failed.sh and its outputs are preserved. The corrected false-rxDSzErH invocation binds the cached dependency search path and explicitly fails with the remaining goal False.
