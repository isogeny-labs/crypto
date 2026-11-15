# Compiled square root: tables (analyze.py)

G1 (counted fields identical, every REF/CHAIN pair, REF p1 vs p2, A/A): **PASS**, 8580 counted fields compared REF vs CHAIN; verification failures: 0.

A/A floor (|REF2/REF1 - 1| of summed relations.cpu, max over q = 2097091, 4194271): 0.1599 (per size: 2097091: 0.9890, 4194271: 1.1599).

Primary: relations.cpu CHAIN/REF (summed over D, DL1, DL arms and attempts; mean of the two passes). Max over the 15 cells: 0.9438. Below 1 everywhere: True. Below 1 - floor (0.8401): False. **Decision: below 1 at every (q, seed) but not separated from the A/A floor.**

| q | s1 | s2 | s3 | geomean | DL arm only | total CPU DL | total CPU DL1 | total CPU D |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 262111 | 0.689 | 0.785 | 0.854 | 0.773 | 0.773 | 0.889 | 0.891 | 0.925 |
| 524287 | 0.931 | 0.861 | 0.940 | 0.910 | 0.910 | 1.002 | 1.018 | 1.011 |
| 1048507 | 0.876 | 0.862 | 0.655 | 0.791 | 0.791 | 0.956 | 0.952 | 0.961 |
| 2097091 | 0.879 | 0.891 | 0.821 | 0.863 | 0.863 | 0.969 | 1.001 | 0.972 |
| 4194271 | 0.926 | 0.944 | 0.863 | 0.910 | 0.910 | 0.993 | 1.005 | 0.985 |

Cells with any unit at load1 > 2.0 at lock acquisition: none; max cell ratio without them: 0.9438.

Pass-to-pass REF relations.cpu ratio (p2/p1): 262111/1: 0.870, 262111/2: 0.894, 262111/3: 0.848, 524287/1: 1.113, 524287/2: 1.011, 524287/3: 1.050, 1048507/1: 0.927, 1048507/2: 0.998, 1048507/3: 1.279, 2097091/1: 0.983, 2097091/2: 0.799, 2097091/3: 1.096, 4194271/1: 1.061, 4194271/2: 0.958, 4194271/3: 1.033

