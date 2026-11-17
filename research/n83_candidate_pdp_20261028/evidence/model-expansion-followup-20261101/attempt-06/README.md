# Full-domain model training replay

This is a fresh execution of the frozen grouped leave-one-base-out ridge procedure in `train_full_domain.py`, using the 336 preserved, replay-verified top-form systems from sibling `attempt-05`. Outputs were written to this new directory so the original run remains untouched. The rows, predictions, and result JSON hashes match `attempt-05/model/` byte-for-byte.

The model used the protocol's 15 point-derived and presentation-size features, standardized from each training fold, with ridge alpha 10 and an unpenalized intercept. Validation held out each coordinate-bit base in turn and compared MAE against the matched family/m/target training mean. Result: 238 finite interval-midpoint proxy labels, 98 positive-dimensional rows excluded from numeric regression, and 48/48 matched configurations identical across bases. The matched-configuration baseline MAE is 0; the ridge grouped out-of-fold MAE is 2.93856, it wins 0/7 held-out bases, and the frozen exploratory-lead condition is false.

This replay finds no predictive lead in this panel. Its response is only a top-form interval midpoint proxy/status, not exact degree of regularity, FFD, LFD, or solving degree.
