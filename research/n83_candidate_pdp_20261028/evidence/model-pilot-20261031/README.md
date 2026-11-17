# N83 fixed-factor-base model pilot (2026-10-31)

Exploratory grouped-holdout fit on completed coordinate-bit bases r27, r28, r30, r31, r32, and r33. The pilot has 288 system rows: 204 finite certified top-form intervals and 84 positive-dimensional top ideals (14 per base). The fitted target is only the midpoint of each finite interval, not an exact degree.

The first fit is retained as `result-v1-superseded.json`; its global-mean baseline was invalid for the repeated family/m/target configurations. Protocol v2 amended that comparison before the corrected rerun. Under the corrected six-fold leave-one-base-out evaluation, the ridge fit has MAE 2.707 and the matched family/m/target training-fold mean has MAE 0.000. The model loses on all six held-out bases.

All 48 `(family,m,target)` configurations have exactly identical status and degree intervals across these six bases. The model found no transferable factor-base lead in this panel. It cannot rank bases, and the interval midpoint is not the exact degree of regularity. No LFD closure claim is made.

`protocol-v1-superseded.json` and `result-v1-superseded.json` preserve the initial exploratory run and correction. `protocol-v2.json` is the corrected frozen protocol. `source-manifest.json` records SHA-256 and byte length for each source input and top summary. `train.py`, `rows.json`, and `result.json` provide the analysis and row-level replay data. Source artifacts remain in `/tmp/n83-candidate-inputs` and `/tmp/n83-candidate-top` on this host and are already separately preserved in the fixed-base evidence packages.
