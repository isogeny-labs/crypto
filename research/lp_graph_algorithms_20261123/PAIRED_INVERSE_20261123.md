# Paired coefficient inversion for graph-IC LP witness admission

Date: 2026-11-23. Owner: [draft PR #4988](https://github.com/isogeny-labs/crypto/pull/4988). Status: **original-source Rust modification and regression committed; fresh CI pending**. All performance conclusions here are component-level operation counts, not timing claims. [pcc:unverified]

## Mechanism and correctness

The original LP witness extractor stores both inverses of each admitted independent equation edge `a*x_u + b*x_v=0` over the **relation field** `F_ell`. Before this change, admission eagerly performed `inv(a)` and `inv(b)` separately for every canonical two-LP input row, whether or not it became an independent tree edge. The previous cache already removed inversions from repeated tree-path traversals; this study addresses the remaining admission cost. It does not change `F_p` curve arithmetic.

For nonzero `a,b` in a field, compute `t=(a*b)^(-1)` once. Then

```
a^(-1) = b*t,      b^(-1) = a*t.
```

Thus both precomputed inverse entries equal the old values, regardless of which endpoint a rerooted tree traverses. Computing `a*b`, `b*t` and `a*t` uses **three modular multiplications**, instead of one extra extended-Euclidean inversion. The exact original source-row coefficients, left-kernel witness basis, LP cancellation, and sparse small/RHS reconstruction are unchanged.

Validate the full input row (arity <= 2, distinct bounded columns, coefficients in `1..ell`) before computing inverses and before changing forest/rank state. Arity zero needs no inverse; singleton uses its original single inverse; arity two uses the product inverse. The caller contract only guarantees field behavior for prime `ell`, but the code also preserves the previous **failure attribution** for composite odd moduli: if `a*b` is noninvertible, test `a` on this refusal path and report the first nonunit input coefficient; a unit `a` with nonunit `a*b` implies nonunit `b` in any commutative ring. Refused rows do not alter retained witnesses or counters. Failure-path diagnostic work is not included in accepted-stream `inverse_calls` and must be charged separately in whole-stream evaluation.

## Cost and scope

Let `I` be LP input incidences, `U` nonempty admitted rows, `H` parent-path hops, and `A` anchor normalization inversions. The previous cached-edge version used `I+A` inversions on accepted rows (two for arity-two inputs, one for singletons). The paired version uses `U+A` inversions. It adds three field multiplications per arity-two input, allocates no new persistent state, and retains `O(H)` parent-path and output-sensitive source-map work. The bound is a *constant-factor* improvement in inversions for two-entry rows, **not** a new index-calculus exponent or guaranteed wall-clock improvement. [pcc:unverified]

The existing synthetic F101 32-vertex path with 48 repeated closing edges has `R=79` and `I=158`; the expected accepted-stream counter changes from 158 to **79** inversions, with the same 48 exact balanced-cycle witnesses, source replay and long parent paths. An additional regression checks zero-, one- and two-LP rows over F3/F7/F101/F1000003 and both orders of noninvertible inputs modulo 9; only prime-field tests support the rank theorem.

## Validation, controls and falsification

- Frozen preceding control: [GitHub Actions run 45149347868](https://github.com/isogeny-labs/crypto/actions/runs/45149347868), original-source release Rust test **24/24 PASS**, on commit `0dc8dc835b811285075ea2c062746839b0d46a48`, before this pairing change; not the new test result.
- Modified original source: `src/cryptanalysis/large_prime_graph/witness.rs`; exact independent source-replay/inverse counters: `tests/large_prime_graph.rs`. The source-bound workflow links these original files and is expected to execute **25 tests**, including 450 seeded exact-rank cases, 160 source replay cases, 96 synthetic same-stream pivot comparisons and the long-cycle regression.
- Reject the change if any witness, source coefficient, LP cancellation, projected useful rank, refusal, or Rust test differs unexpectedly. Verify the native CI and source head before reporting pass. Full-root `cargo test --release --lib` is **not** supplied by the source-bound gate.
- For a genuine eligible Prime31 at-most-two-LP stream, compare paired vs frozen cached-edge `0dc8dc8`, rooted-parent `21418acd`, pivot and MST on identical canonical inputs, verifying original group equations and replay before extracting rank. Charge field inversions **and** multiplies, rejected rows, output/source support, memory, wall time, target extraction, and complete cold cost. Do not promote synthetic counts to a performance result.

**Sources checked (KB):** No research-KB MCP tools are exposed in this execution; there is no local repo checkout or runnable research-KB wrapper. Bounded fallback: original branch source `witness.rs`, `large_prime_graph.rs`, same-stream tests and [merged catalog #4917](https://github.com/isogeny-labs/crypto/pull/4917); [prior edge inverse cache derivation](EDGE_INVERSE_CACHE_20261123.md) and primary finite-field inversion identity. No novelty assertion or external fastest-tool claim.

**Execution constraints:** One coordinator (no parallel workers available in this tool runtime), no local `rustc/cargo`, no authenticated EKS allocation. Hosted source-exact GitHub Actions gate is allowed. No paid compute, merge, auto-merge or publication. [pcc:unverified]
