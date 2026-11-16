# Independent hosted replay of the frozen N83 cross-family panel

The [dispatch](dispatch.json), SHA-256
`7a9addb02606f8b3f3202c50900bd0c001ff6d034adc6a1e06bb30e85f15f894`,
pins the four provider artifact IDs, ZIP sizes
and SHA-256 digests, raw `analysis.json` digests, source commit, run ID, and
attempt. The source is the same frozen commit used for the a=1 N83 four-family
Tier 1 panel; this replay executes no new IC or rho timing measurements.

The workflow starts four separate GitHub Actions runners. Each runner checks
out the pinned source commit, downloads one exact provider artifact, checks
archive size, SHA-256, member paths, member count and uncompressed size,
extracts `results/`, then runs the source verifier over archived base,
relation, scalar, timing, resource and calibration receipts. It requires exit
zero, empty stderr, `STRATUM_ADMITTED`, and canonical JSON equality with the
provider analysis whose raw SHA-256 is pinned. Success and failure receipts
are retained as workflow artifacts. Runner IDs and source hashes are explicit.

After all four replay receipts pass, bind them to the frozen local aggregate
and selector using the locally retained provider ZIPs. The local aggregate
has already admitted the four strata and selected K2816 for complete cold
process time within this panel; its online selector is `BOUNDED_UNKNOWN`.
This hosted replay checks that verdict's arithmetic and source provenance;
it adds no timing measurements. Keep `selected_best_total_runtime` null for
the broader factor-base sweep, which also includes smaller a=1 and a=0
families.
