# Terminal arithmetic in coefficient construction

This additive variant removes the unused final square from the scalar power helper and introduces finish_batch_inverses for the reciprocal tail shared by compiled coefficients, ordinary/shared coefficient streams and progression evaluation. The original field and prefix contracts remain. The helper checks shape, accepts empty and singleton cases, and relies on each caller's existing inverse verification. The compiled coefficient path reuses its denominator vector as output.

`generate.py` copies the shared-progression-inverse predecessor and records exact reversible edits. The old Fermat self-test expectation changes from13 to12 multiplication calls because exponent95 no longer performs its unused final square. This amended expectation is explicitly in the generation manifest; old evidence is unchanged.

Run `python3 generate.py`, then `python3 validate.py FRESH_DIR`. The validator reverses source edits, compiles with optimized overflow checking, runs targeted power/batch checks plus predecessor checks, and compares inherited stdout byte for byte with saved planned-width output. Limits are90s build,30s focused checks and60s inherited checks. Every attempt uses a fresh directory. Rust performs arithmetic; Python only orchestrates and records hashes.

The source-derived reductions are one multiplication per positive exponent and two per integrated nonempty reciprocal tail. No empirical comparison panel, timed workload, cloud job or original cohort change is involved. The interval counter excludes outside-engine work unless a software control explicitly opens a wider region. No total recovery operation saving is inferred without a complete path count.

Sources checked: predecessor modes.rs, streamed.rs, context.rs and progression.rs, existing meter and exact native controls. KB unavailable; source fallback explicit. The implementation uses standard binary powering and prefix-product inversion, with boundary identities derived in DERIVATION.md. Future comparison arms must share this same instrument and updated helper before their source bindings are frozen.
