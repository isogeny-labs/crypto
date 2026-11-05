# Quartic collapsed-signature compiler

Builds a polynomial for the mean of each public character-signature fiber. It is an exact target-construction component; conditional recovery inside ambiguous fibers is the next layer.

```sh
rustc --edition=2021 -O -C overflow-checks=yes --test research/primefield-companion-followup-20261102/feature-native-quartic-collapse/lib.rs -o /tmp/collapse-tests
/tmp/collapse-tests --test-threads=1
rustc --edition=2021 -O -C overflow-checks=yes research/primefield-companion-followup-20261102/feature-native-quartic-collapse/check.rs -o /tmp/collapse-check
/tmp/collapse-check research/primefield-companion-followup-20261102/feature-native-quartic-collapse/program.tsv
```

See DERIVATION.md for bounds and claim scope. No timing comparison was run.
