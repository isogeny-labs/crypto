# K600 m6 two-revision source admission — 2026-11-15

This amendment repairs the native preflight/arm source-commit cycle recorded
in [the problem record](../../../../problems/n83-k600-preflight-lock-commit-cycle.md).
It changes the admission contract, not the frozen K600 object, target,
search arms, solver caps or proposed charged allowance.

The source and Linux arm64 image are built at clean commit `C`. Native
preflight `prepare` records `C`, the image ID, worker digest and exact source
file hashes in a lock under the source checkout. Committing **only that new
lock** produces evidence HEAD `H`. The preflight `run` already accepts `C`
as an ancestor of `H` while rechecking the source hashes and the committed
lock bytes. Native arm `stage` and `freeze` operate at clean `H`; `freeze`
now accepts the committed preflight lock as an explicit input:

```text
n83_k600_m6_native_arms freeze STAGED_DIR EXPORTER_REPLAYER_BINARY SOLVER_BINARY IMAGE_ID COMMITTED_PREFLIGHT_LOCK NEW_LOCK_JSON
```

`freeze` requires the preflight lock to match `git show HEAD:path`, `C` to
be the sole direct parent of `H`, and the complete tree diff `C..H` to contain exactly
one added preflight lock. It rehashes the lock's source-file set against `H`
and requires the lock's image ID to match. Docker image inspection now
requires the source-revision label `C`; the arm lock stores both
`build_source_commit=C` and `source_commit=H`, plus the preflight lock hash.
The arm commands continue to require clean HEAD `H`. Their preflight gate
requires the same preflight lock hash, image ID, `C` in the preflight lock
and zero-trial worker config/summary, and `H` in the arm lock.

**Validation:** Rust 1.99.1 macOS arm64 release arm example tests pass
14/14, including a clean temporary Git repository that accepts the
`C -> H` lock addition and rejects a substituted lock, changed source
bytes, wrong image ID and an extra later commit. The dependent preflight
example passes 6/6 and the arm release binary builds. The root release
library run at the prior example-only head `9428ed7e275b` remains red
(6,306 passed, 59 failed, 138 ignored); this amendment changes only the
arm example and study files, so the library target's source is unchanged.

**Remaining gate:** The temporary Git test checks the two-revision predicate
and the separate synthetic preflight-gate test checks receipt binding. A
source-matched Linux image, committed real lock, full zero-trial preflight
and arm freeze have not yet been run together. The full-invocation budget
receipt and separate host/input/allowance gates also remain open. This
code change measures no natural relation yield, rank or cold IC runtime;
no performance graph or leaderboard row changes.
