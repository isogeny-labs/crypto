# Complete-route destination field accounting

Session: `ECBS1he874fcb79410`

Source commit: `be75e8d4de74b5cef0f931427732b2149310c973` (clean)

Producer/auditor binary SHA-256:
`5953efb706195b16dad4220187bdcf9413c5c09d4670f726f7e4180a63224afe`

All 64 records verified, and all 64 deterministic replays were byte-identical.
The audit reported zero problems; its receipt SHA-256 is
`fd5165353a22730c7cf86c6148845eb7c33d03b449d03db0c8c919351f81a519`.

The table reports `median [min, max]` across eight fixed targets. Counts cover
the instrumented `F2mElement` calls in cover construction, generator and target
pullbacks, destination solve, and internal destination replay.

| `n` | method | additions | multiplications | squarings | inversions |
|---:|---|---:|---:|---:|---:|
| 9 | BSGS | 4,535 [3,175, 5,338] | 2,766 [1,939, 3,256] | 1,581 [1,149, 1,845] | 180 [126, 213] |
| 9 | rho3 | 11,443 [7,987, 17,634] | 6,967 [4,871, 10,744] | 3,689 [2,661, 5,757] | 443 [315, 702] |
| 13 | BSGS | 12,374 [9,262, 16,672] | 8,041 [6,020, 10,837] | 6,185 [4,673, 8,297] | 499 [373, 675] |
| 13 | rho3 | 31,915 [20,675, 51,893] | 20,689 [13,403, 33,620] | 15,335 [9,977, 24,737] | 1,261 [815, 2,045] |
| 19 | BSGS | 86,379 [66,103, 98,351] | 59,718 [45,695, 68,000] | 63,893 [48,917, 72,749] | 3,534 [2,702, 4,026] |
| 19 | rho3 | 219,563 [99,191, 329,752] | 150,905 [68,211, 226,678] | 156,323 [70,967, 234,929] | 8,669 [3,927, 13,036] |
| 23 | BSGS | 284,091 [216,191, 425,775] | 208,138 [158,393, 311,973] | 256,769 [195,477, 384,853] | 11,656 [8,870, 17,478] |
| 23 | rho3 | 1,105,662 [187,298, 1,351,185] | 804,010 [136,260, 983,233] | 963,552 [163,797, 1,181,407] | 43,782 [7,430, 53,685] |

The inversion count is informational: its internal multiplications and
squarings are already present in their respective columns and must not be
priced twice. Additions include characteristic-two XOR operations and are not
part of `field_ops`, but remain in the complete-route native counters.

This is an accounting result, not a timing result. The tallied adapters update
a thread-local map on each operation, so their wall, cycle, and instruction
measurements are deliberately inadmissible. The optimized source solver also
uses a different binary-field kernel, so these counts do not establish a
source/destination common unit.
