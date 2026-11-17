# Bounded quadratic-monomial cancellation

Rust constant-weight row combinations now expose checked units from quadratic
polynomial controls. On the28fixed curve inputs the outcome classes and node
counts remain unchanged:15Root/13Unsupported/0Cap. All producer standing is
unsigned/unverified. Core1ab7fd, literature3187dc, separate engine contract6a68d9.

- [Core protocol](PROTOCOL.md), [pre-code control note](DESIGN_NOTE_PREIMPLEMENTATION.md).
- [Separate runtime contract](ENGINE_ROLLBACK_PROTOCOL.md), [frozen custody](FROZEN.md).
- [Core native aggregate](native-v1/controls.json), [actual engine results](engine-rollback-native-v1/results.json).
- [Reproduce](REPRODUCE.md), [criteria and next hypothesis](NEXT.md).
- [Report](../reports/monomial-cancellation-v1/report-cited.md), [PDF](../reports/monomial-cancellation-v1/report.pdf).

The core borrows the original System/cache, uses a fixed32row/64variable/
256input-coefficient connected slice and pivots quadratic columns before linear.
It multiplies no equations, raises no degree and implements neither full XL nor
F4. Only unit/constant explanations affect search. Previous sources, public
fixtures, criteria, reports and claims remain unchanged.
