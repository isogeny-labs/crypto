# Normalized fixed-index transport

Read DERIVATION.md for the conditional identity, phase assumptions, anchors and
consumer-integration obligation. Source/Prepared takes public points and explicit
lane/window limits. It returns homogeneous normalized-value columns, not a scalar
logarithm. Complete subgroup tables are checker artifacts only.

From the repository root:

```sh
rustc --edition=2021 -O -C overflow-checks=yes --test research/primefield-companion-followup-20261102/feature-native-normalized-transport/lib.rs -o /tmp/normalized-transport-tests
/tmp/normalized-transport-tests --test-threads=1 --nocapture
rustc --edition=2021 -O -C overflow-checks=yes research/primefield-companion-followup-20261102/feature-native-normalized-transport/check.rs -o /tmp/normalized-transport-check
/tmp/normalized-transport-check research/primefield-companion-followup-20261102/feature-native-normalized-transport/columns.tsv
```

Substitute negative-columns.tsv to reproduce the intentional nonzero checker exit.
The native build cap is90seconds; test/check caps60seconds. Clear
NORMALIZED_TRANSPORT_OUT and inherited experiment export variables when replaying.
Saved packaging scripts write artifacts and must be run only in a scratch copy.
No comparative timing or cloud job. Claims in claims.json remain unsigned.
