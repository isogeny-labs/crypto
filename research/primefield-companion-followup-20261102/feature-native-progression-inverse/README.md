# One shared inverse for progression setup

This additive native variant replaces the two progression-setup Fermat inverses with one inverse of step*d!. Both original multiplicative checks remain. No cache layout, stream identity, coefficient delivery or source-query policy changes. The method uses standard simultaneous inversion; the contribution here is applying it at this previously separate setup pair.

`generate.py` copies the exact qualified work-accounting source and records reversible edits to progression.rs and its checker entry point. Other copied modules remain byte-identical. `validate.py FRESH_DIR` reverses every edit, compiles with optimized overflow checking, runs the new small-field qualification and existing meter checks, then runs inherited recovery checks and compares their stdout byte for byte with the preserved planned-width output. Limits:90s build,30s new checks,60s inherited; each attempt has a new immutable directory.

DERIVATION.md gives the field identity and source-derived operation change. No empirical width-policy comparison, timed workload, cloud job or original cohort replacement is involved. The full width-panel protocol remains draft; future comparisons must give all arms the same selected progression implementation and rebind the source closure before freeze.

Sources checked: work-accounting/progression.rs, its exact binary pow helper, batch-reciprocals accounting sources and direct finite-field algebra. KB unavailable; source fallback explicit. This is not a novelty claim for batch inversion. [pcc:unverified]
