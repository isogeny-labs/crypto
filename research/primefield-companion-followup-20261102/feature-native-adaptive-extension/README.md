# Reusable adaptive source and incremental cursor

The source retains prepared public strides and anchors across recoveries. A bounded cursor retains projective grid state and the last two seeds for a lazy extra sign. Existing adaptive coefficients and predicates are unchanged. See DERIVATION.md for the state invariant, anchor cases and evidence standing.

```sh
rustc --edition=2021 -O -C overflow-checks=yes --test research/primefield-companion-followup-20261102/feature-native-adaptive-extension/lib.rs -o /tmp/extension-tests
/tmp/extension-tests --test-threads=1
rustc --edition=2021 -O -C overflow-checks=yes research/primefield-companion-followup-20261102/feature-native-adaptive-extension/check.rs -o /tmp/extension-check
/tmp/extension-check research/primefield-companion-followup-20261102/feature-native-adaptive-extension/recovery.tsv
```

The checker reexecutes the frozen pre-cursor source; metadata checks are not execution proofs. Raw receipts and test diagnostics do not constitute a timing comparison.
