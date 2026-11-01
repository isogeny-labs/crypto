# Round nine, attempt 1 — failed, kept as evidence

Card `CARD-HEIC-ROUND9-1` (commit `251ff11e`) stopped at command 1: the
binary panicked after 54 s, exit 101, having printed seven of eighteen
rows (`R9.partial-panic.txt`; isolation record `isolated_bench.jsonl`).

Cause: `HyperellipticCurveP::new` on `main` now validates its input and
panics on a non-squarefree `f`. The thread's instance rule constructs
each candidate `c` and only then skips non-squarefree ones, so the first
invalid `c` (genus 3, `p = 31`) aborted the run. The same change breaks
the frozen round-seven and round-eight examples on current `main`.

Fix: `thread_curve` uses `try_new` and returns `None` for an invalid
model, which the rule skips, as before. The rule's choices are pinned by
a test against round seven's frozen `N`, now including genus 3 `p = 31`
(`N = 7333`). The rerun is a fresh card.

The seven rows printed are not the registered measurement and are not
used for any verdict.
