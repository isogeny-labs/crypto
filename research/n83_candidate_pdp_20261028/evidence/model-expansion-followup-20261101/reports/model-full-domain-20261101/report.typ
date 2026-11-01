#set page(paper: "us-letter", margin: 0.65in)
#set text(size: 10pt)
#set par(justify: true, leading: 0.55em)
= Full-domain N83 top-form model
Seven fixed factor bases, 2026-11-01

*Measured result.* Seven fixed factor bases (spike27–spike33), each evaluated on the same 48 family/m/target configurations, produced 336 top-form certificates. Independent replay verified all 336. Across all 48 matched configurations, certificate status and both interval bounds were identical across the seven bases. The grouped ridge model found no predictive lead in these labels: 7-fold grouped out-of-base MAE was 2.939, versus 0.000 for the matched-configuration mean baseline; ridge won 0/7 folds.

== Scope
This models the *top-form certificate finite-interval midpoint proxy*, not exact degree of regularity, FFD, LFD, affine solving degree, or a general factor-base ranking. There are 238 finite-interval rows and 98 positive-dimensional-top-ideal rows without finite labels (34 and 14 per base, respectively). Equality compares exact (status, lower bound, upper bound) tuples for each matched family/m/target configuration.

The measured lead is that changing among these seven factor bases did not change the observed label for these configurations. A next model needs a downstream degree/cost label responsive to factor-base selection, under the same fixed-base grouped validation. This does not imply equality of underlying ideals or unmeasured solving costs.

== Grouped holdout error
#figure(image("grouped-oof-mae.svg", width: 100%), caption: [Recorded ridge and matched-configuration baseline MAE across seven leave-one-base-out folds; n=34 finite rows per held-out base. Label: attempt-05, protocol v3, top-form interval midpoint proxy.])
#table(columns: 4, [Held-out base], [n], [Ridge MAE], [Baseline MAE], [spike27], [34], [2.973], [0.000],  [spike28], [34], [3.270], [0.000],  [spike29], [34], [3.399], [0.000],  [spike30], [34], [2.476], [0.000],  [spike31], [34], [2.171], [0.000],  [spike32], [34], [3.231], [0.000],  [spike33], [34], [3.049], [0.000], )

== Protocol and records
Seven leave-one-base-out folds; ridge alpha 10, train-only standardization, unpenalized intercept; features include dimensions, family/target indicators, and point-coordinate/selector summaries. Finite labels use certified interval midpoints. Baseline predicts the training mean for matched family, m, and target kind. The frozen exploratory-lead rule required 5/7 fold wins and lower aggregate MAE; it was not met.

Protocol v3 SHA-256: `280a78bd11e6fc20ae274c47525070a2cb6e455401593774b1688dbdd9537e94`. Labeler binary SHA-256: `4749b49ba7e98cd916c9d36d4b14c0396041fd6ff31e3708e17e7a91749fe53a`. Full raw inputs, certificates, replay summaries, rows, predictions and result are in the sibling `attempt-05/` directory; see `report.md` for paths, commands and generator verification. Producer/replay wall times are diagnostic only.
