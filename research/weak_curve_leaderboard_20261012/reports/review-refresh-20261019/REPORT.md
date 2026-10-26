# Conditional GHS cost-model refresh — 2026-10-19

PR #2154 reconciles the genus-128 audit with the transfer registry's measured Jacobian-operation table and its stated extrapolation exponent. The previous audit read the retired quadratic-price field and failed before producing a current replay.

| Curve | Previous total | Current total | Matched formula rho |
| --- | ---: | ---: | ---: |
| c2pnb176w1 | 71.336033 | 74.834885 | 94.973 |
| c2pnb272w1 | 71.773568 | 74.879405 | 143.970 |

All values are **log2 modeled bit-operation costs**. They are conditional formula prices. No new timed experiment or confidence interval is attached to this refresh. The model still selects bound 15 from 84 tested degree bounds and 48,921,052 nominal half-split places on both exact source curves. The smooth-polynomial count, base count and modeled query count remain unchanged.

The operation table measures Cantor operations only through genus 32. Pricing genus 128 extrapolates with exponent 2.347; it also assumes that half the irreducibles supply usable places and that polynomial smoothness predicts divisor smoothness. The actual smooth cover, selected-subgroup map, usable places, measured relation yield, useful rank, final linear algebra and target descent remain separate witness gates. A formula gap to rho does not establish a measured complete attack cost.

[Previous receipt](previous-audit.json), [current receipt](current-audit.json), [provenance](provenance.json), [graph](cost-sweep.svg), and [PDF](report.pdf) retain the comparison. Full source curve identities and subgroup orders are in both receipts. Original dated reports and their graphs remain historical evidence; this report updates the study's current links. The website JSON mirrors use the current receipt and regenerated indexes.

Validation: six standalone release tests passed; the current audit and snapshot replay passed; 98 weak-curve and 991 P-192 manifest entries matched; attack-map and decomposition validators passed. These checks validate the model and its source bindings. Repository-wide CI remains a separate merge gate.

Reproduce from `research/weak_curve_leaderboard_20261012`:

```sh
cargo test --release --locked
cargo run --release --locked -- check snapshot.json
cargo run --release --locked --bin ghs_genus128_cost_audit -- --check
python3 reports/review-refresh-20261019/build_report.py
typst compile reports/review-refresh-20261019/report.typ reports/review-refresh-20261019/report.pdf
```
