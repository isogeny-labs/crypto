## 2026-11-05 — One-reduction bitplane accumulator publication

### What was done (commands, files, features)

Implemented an exact i128 Horner fold followed by one Euclidean remainder. Proved the bound T(2^h-1), including intermediate doubling, for T <= 2^64-1 and h <= 63. The saved packet uses one reduction instead of nine. The saved optimized overflow-checked native run passed all 283 tests, including 1024 extreme digit vectors and 4096 modular comparisons. Verified all 36 prebuild source/data hashes and visually inspected the one-page report.

### Decisions made (and why)

PR3367 has merged. Publish this incremental component on research/primefield-wide-accumulator-20261105 against main. Preserve raw logs, warnings, original cohort and all historical evidence. Reduction counts are source-derived; no runtime comparison was performed.

### Open threads (unfinished, pick up next time)

Complete-cost instrumentation and an admitted comparison remain next research work. Eight integration fixtures do not replace the original 4563-case cohort. GitHub checks and review remain publication gates.

### Errors (what went wrong, how it was handled)

The build retained 19 compiler warnings. Migration receipt access remains permission-denied. Diem reports deadline-expired and pending=false; no generator or cloud job was started. Reconciled saved files and the absent branch PR before publication; no native test rerun was needed.
