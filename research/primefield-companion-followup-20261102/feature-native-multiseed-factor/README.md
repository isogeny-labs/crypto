# Normalized character consumer

Read DERIVATION.md for the four-input factorization, inherited transport domain,
retained costs and the adaptive cursor integration step. lib.rs is the public
point API. It shares geometry, phase and value initialization across the seed
and all normalized columns. Production residual.rs contains no division-value
context; its scalar-input reference is compiled only for tests.

From the repository root:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-multiseed-factor/generate.py --check
rustc --edition=2021 -O -C overflow-checks=yes --test research/primefield-companion-followup-20261102/feature-native-multiseed-factor/lib.rs -o /tmp/multiseed-tests
/tmp/multiseed-tests --test-threads=1 --nocapture
rustc --edition=2021 -O -C overflow-checks=yes research/primefield-companion-followup-20261102/feature-native-multiseed-factor/check.rs -o /tmp/multiseed-check
/tmp/multiseed-check research/primefield-companion-followup-20261102/feature-native-multiseed-factor/characters.tsv
```

Substitute negative-characters.tsv to reproduce the intentional rejection.
Native build cap90seconds; execution/checker caps60seconds. Clear MULTISEED_OUT
and other inherited artifact export variables during read-only replay. Packaging
scripts write artifacts; run them only in a scratch copy. The data tables are
checker fixtures, not runtime answer inputs. No timing panel or cloud job.

receipt.json and checker-receipt.json preserve actual invocations. Native and
checker dependency closures contain14 and15 hashes. checker-build-initial.log
preserves the initial type-inference error before adding an explicit point type.
Both claims in claims.json remain unsigned. Original4563-case cohort unchanged.
