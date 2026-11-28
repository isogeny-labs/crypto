# Fixed-curve coordinate height and Mathlib naive height

The Lean module connects the exact rational curve's square-denominator coordinate model to Mathlib's point naive height. For every nonidentity point, it obtains a natural square denominator for the reduced rational x-coordinate. For any point with `max |a| b ≤ 2^130` and reduced `x = a/b²`, it proves `naiveHeight P ≤ 260 * log 2`.

## Proof map

| Mathematical step | Lean declaration | Status |
| --- | --- | --- |
| Every nonidentity point on `y²=x³−47x²+751x−3997` has a reduced square x-denominator and finite coordinate height | `XedniNaiveHeightBridge.exists_coordinate_height` | Checked in this module |
| Mathlib naive height of a fixed-curve affine point equals `log(max |x.num| x.den)` | `XedniNaiveHeightBridge.naiveHeight_eq_log_coordinate` | Checked in this module |
| Coordinate cap `max |x.num| b ≤ 2^130` implies point naive height `≤260 log 2` | `XedniNaiveHeightBridge.coordinate_cap_implies_naiveHeight_bound` | Checked in this module |

The last implication uses `x.den=b²`, so both the reduced numerator and denominator are at most `2^260`. The full fixed-curve height-exclusion result also needs a formal canonical-height comparison and the Mordell–Weil generator statement; those remain open as named dependencies in the earlier packets.

## Reproduction

Run `sh formal/build.sh` from any directory. The script compares the pinned Lean toolchain and Mathlib manifest against the checked repository installation, builds the predecessor square-denominator module, builds `NaiveHeight.lean`, and prints the axiom audit. The `#print axioms` output should contain only `propext`, `Classical.choice`, and `Quot.sound`. No generated `.olean` file is tracked.
