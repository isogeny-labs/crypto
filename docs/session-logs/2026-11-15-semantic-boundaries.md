# BSGS semantic-boundary maintenance - 2026-11-15

## 2026-11-15T22:53Z - BSGS planner and accumulated-cost boundaries

### What was done
Found two floating-point seeded square-root loops whose upward saturating-product condition remains true at u64::MAX. The exact old helper exceeds a bounded one-second local control; the owned control child was terminated. Replaced both loops with the integer square root already used elsewhere in this repository, sharing the helper between planning and reporting.

Cost reporting also summed three uint64 counters before conversion to floating point. The exact old code undercounts an overflowing accumulated sum in release and can panic in debug. Use checked integer addition for ordinary sums, preserving their prior rounding, and sum in floating point only when the integer total cannot fit.

The exact-source extraction passes both new boundary controls and 20,000 deterministic random square-root inequality cases checked with uint128 products. The first actual library replay passes 19 BSGS tests; final-source library replay, including the cost-sum control, is compiling. The maximum-width plan test does not allocate its table. No performance measurement or frozen experiment rewrite.

### Decisions made
Carry the canonical benchmark utility pair after owner PR3312 merges. Publish the combined consumer change with the existing prior-PR closure metadata, then follow required CI and merge policy.

### Open threads
Final library replay, owner landing, shared copy and this consumer PR. Original multi-target oracle semantics remain tracked in the owner problem record and require the missing original reference.

### Errors
First library compilation failed because sparse checkout omitted an existing include_str input. Materialized that exact tracked directory and retained the failed log. The selected Rust toolchain lacks rustfmt; used the existing Homebrew formatter for this file. No toolchain or dependency downgrade.

The initial attempt to append the shared daily log exposed five pre-existing prose-lint findings in earlier entries. Those entries remain byte-identical; this session uses its own dated log so its changed text can pass the existing lint without rewriting historical entries.

Final source replay: all 20 focused BSGS library tests pass, zero failures or ignored tests, 6,529 unrelated tests filtered. The build completes in the release profile; its duration is not a performance comparison. Existing compiler warnings are retained. Autonomy metadata verification passes 167 entries with zero errors.

## 2026-11-16T02:12Z - owner synchronization and base integration

Copied the exact benchmark utility pair from owner PR3312 merge76923ef45a6e734f7cda337dde387981940d2c5d and changed only their two lock digests. All58 utility tests pass; four report formats remain byte-identical. Merged current main and restored its committed PCC metadata so the owner keyring aligns; ledger1179 chain and parity21 pass. Initial record-gate failures were sparse omissions. Materialized committed manifests, report/include inputs, and54 raw/graph files totaling1,119,976bytes; all215 records now pass, retaining the existing hash-bound archived registry replay. No run status, payload or report was rewritten.

The current-base library build exposed two leftover Vec returns in the cached branch of lift_x_packed, whose API now returns a packed array/count. Changed the empty branch to (none,0) and the successful branch to two packed roots, preserving order. The original E0308 failure log remains. The new thread-local cache control covers0/1/2 results, and the existing ordered-root comparison passes across its declared fields. The complete release library builds; all20 BSGS tests and both packed-lift controls pass. Formatter checks pass.
