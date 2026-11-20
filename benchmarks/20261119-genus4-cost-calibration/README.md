# Genus-4 IC native-operation prices (lead A), 2026-11-19

Timing-tier calibration of the §5a price convention. Results: `research/genus4_cost_calibration_20261119/RESULTS.md`
with `ERRATA.md` (the errata take precedence). Validator disposition: validated-with-errata
(`validator-report.md`).

**Not a gate row.** There is no `VALIDATION.json` and no line in `benchmarks/README.md`. The solves'
scalars cannot be independently certified (validator F8): no row records the recovered logarithm,
and the k = 0 keys are not among the prime-order study's certified keys. This is a calibration of
operation prices, not a speed claim. Every number is `[pcc:unverified]`.

Headline: w_rel ≈ 0.0025 GAE and w_la ≈ 0.0042–0.0045 GAE per counted operation. The split between
the two classes is model-dependent (E2). Charging all non-group time to one class bounds w_la ≤ 0.0097
and w_rel ≤ 0.042. The w = 1 end of the §5a band is therefore conservative for genus-4 IC at
p ∈ {16411, 32771}.
