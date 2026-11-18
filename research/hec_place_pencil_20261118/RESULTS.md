# Rational-place walks solve all 22 original fixtures

Both variants produced verified scalars on all 22 fixtures across eleven curves. Each audited production outcome, number of visited anchors and accepted pencil row count agreed. The source-multiple certificate was available in 17 of 22 cases. The two-stream arm used no source-multiple certificate. Times are ratios of independently computed nine-sample per-arm medians. Ratios above one favor the new arm.

| Genus | Cases | Two-stream wall ratio | Certificate-first wall ratio | Certificate-first equal-price REDC ratio |
|---|---:|---:|---:|---:|
| 2 | 12 | 0.832–1.192 | 0.765–1.193 | 0.759–1.184 |
| 3 | 6 | 0.914–1.526 | 1.004–1.321 | 1.068–1.363 |
| 4 | 4 | 0.951–1.571 | 1.011–1.528 | 1.059–1.594 |

The certificate-first arm improved median wall time on 20 of 22 fixtures, including all ten genus-three and genus-four fixtures. Its unsuccessful probes and public normalization are charged. The largest observed improvement across both arms was 1.571 on genus four, p=31, source multiple 17, with a 1.647 ratio in equal-price REDCs. This two-stream solve visited 259 anchors against the original 187; its cheaper translations outweighed the extra visits. Ratios near one should not be interpreted as significant from this single-host run. The source construction favors small-multiple certificates; independent-source performance is open. No arm reached tenfold. COMPARISON.tsv retains all 44 matched comparisons and CONTROL.txt all counters and timings.
