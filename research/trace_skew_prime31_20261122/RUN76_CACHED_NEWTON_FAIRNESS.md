# Run76 — cache-corrected S3 elimination comparison

**Status:** source-only experiment committed 2026-11-22 local / 2026-11-23 UTC; native Rust CI, timings and improvement remain unverified. This is an *x-only pair relation extractor*, not a complete ECDLP solver.

## Motivation and scientific correction

The original Run75 per-public-target comparison constructs `h(X)=∏_{w∈W}(X-w)` in the Newton arm every time it calls `construct(f,w,z)`. The balanced-tree arm never materializes `h` and uses its own target-specific algebra. That is a legitimate cold **single-extraction** comparison, but not a fair amortized multiple-target comparison if the registered factor base `W` is fixed.

The new Run76 experiment distinguishes:

1. **Uncached Newton**: same Run75 `construct`, with `membership(f,w)` in every call.
2. **Balanced rank-four tree**: same Run75 `construct_tree_full`, without an expanded factor-base polynomial.
3. **Cached Newton**: computes `membership(f,w)` **once** outside the target loop, then calls the exact same Newton recurrence and the same D/N, GCD, root recovery and post-check pipeline. Charges that one-time setup to the batch.

Source: `examples/prime31_monic_s3_pair_oracle.rs` (`construct_from_membership`, `extract_with_cached_h`, `compare_cached_construction`). No duplicate curve engine, new polynomial solver or generic pair-sum table.

## Frozen protocol

- Unchanged ordinary, cofactor-one elliptic curve over p = 2^31 - 1; immutable public x targets and factor-base constructor as in Run75.
- B = 64, 128, 256, 677. For each B: eight distinct public curve abscissae, exactly three x-only positive single-pair controls and five exact negatives. Independent reference is computed separately and never supplied to any candidate.
- Five source-identical repetitions of each arm per target; rotate arm order between repetitions. Report medians per arm per target and sum of target medians for each B. Verify exact recovered x-coordinate pair sets against the reference **for every repetition**, not just the first.
- Measure membership setup outside the loop once; report raw three-arm extraction sums and **cached-Newton setup + extraction** separately. The balanced tree and uncached Newton also share the same externally produced factor base; factor-base selection is excluded equally.
- All results are *algebraic x-only* candidates. This corpus does not establish signed point lifting, labelled RHS relation rank, public unknown-scalar recovery or total ECDLP cost.
- A cancelled job, failure to compile, a missing size/cell, failed negative, exception or exceeding memory/time is **unknown**, not zero cost, UNSAT or speedup.

## Commands

```bash
cargo test --locked --release --example prime31_monic_s3_pair_oracle
cargo run --locked --release --example prime31_monic_s3_pair_oracle -- --compare
cargo run --locked --release --example prime31_monic_s3_pair_oracle -- --compare-cached
```

The exact original-Rust workflow is `.github/workflows/prime31-trace-symmetric-rust-gate.yml`, now with a bounded Run76 step. This run still requires CI compilation and captured output; **no measurement or Rust test success is claimed for this newly added arm.**

## Prior work and interpretation

- Run74B original-Rust B677 Newton x-only stage: 29.34 ms median; compare to the post-hoc generic reference roughly 4.8–5.5 ms, not as the admissible non-generic solver.
- Run75 original 2-arm full extraction comparison: `RUN75_BALANCED_S3_COMPARISON_20261123.md`; its job was compiling when Run76 was added.
- Semaev (2004) and Faugère–Huot–Joux–Renault–Vitse (EUROCRYPT 2014) already study summation polynomials/symmetrization. Radical-pair DRL, original November 15 signed S3 quotient/GCD, PRs #4920/#4922, and authentic complete RHS solver PR #4901 remain mandatory comparators.
- No asymptotic gain follows from either subproduct trees or precomputed membership alone. A fast pair oracle could still lose on complete costs because two-summand relation success probability is small at p = 2^31 - 1.

## Promotion gate

Inspect CI logs and recompute the per-cell paired totals, including positive versus negative costs and the setup once. If the balanced-tree arm remains faster than setup-charged cached Newton, measure its constructor versus D/N, GCD, rootfinding and verification separately, and compare against the November 15 quotient/GCD signed S3 baseline. If cached Newton erases the apparent advantage, label the earlier gain as avoided repeated setup rather than new algebraic extraction speed.

Regardless of which wins, the next qualifying research task remains **nonenumerative higher-arity public-only extraction** integrated with authenticated RHS rank and independent `[k]G=Q` recovery, measured against fully charged existing IC, optimized negated BSGS and negation-safe rho on the same host.
