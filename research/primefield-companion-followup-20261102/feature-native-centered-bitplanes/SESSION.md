## 2026-11-05 — Centered coefficient bitplanes (T-1604)

### What was done (commands, files, features)

Implemented centered magnitudes and fused coefficient signs into monomial masks. Proved exactness for every ternary input, retained one-reduction range bounds and passed 331 optimized overflow-checked native tests. New checks cover 25,704 small-residue cases and 105 midpoint/extreme cases. All 39 source/data hashes verify; recovery source is byte-identical. Saved packet derived counts: 144 to 128 population counts, 9 to 8 Horner steps, unchanged 328-word payload and one remainder.

### Decisions made (and why)

Keep the original evaluator and all evidence intact; add this variant to the open PR3469. Consulted the existing derivation, Umuroglu et al. 2019 sections 2.1-2.2 and Rust rem_euclid documentation. Keep the arbitrary-program sign-row storage tradeoff explicit.

### Open threads (unfinished, pick up next time)

Complete-cost accounting and compact coefficient-program construction remain next tasks. The original 4563-case cohort stays separate from eight recovery integration fixtures. No timing comparison or cloud experiment was run.

### Errors (what went wrong, how it was handled)

No test failed. Preserve 23 compiler warnings in the raw log. Source-lock generation and build commands overlapped during observation; all source writes preceded compilation and all 39 hashes were rechecked afterward. No prebuild lock-completion ordering is claimed.
