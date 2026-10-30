# Amendment 1 (2026-10-29, before panels B-E were run)

**What happened.** `run.sh` was started with the registered binary. Panel A (`U`, 60 draws) completed. Panel B (`P`) hung at `s = 2`: `digit_fall`'s planted-target search looped forever when the drawn curve has no point of abscissa `< 2^s` (reachable at `s = 2`, where only 4 abscissas qualify). The run was stopped. Panel B had written 7 of its 8 `s = 2` draws; panels C, D and E had not started. Those partial lines are discarded and all of B-E is rerun.

**Change.** `digit_fall.rs`: the planted search is a separate function `plant(...)` with a 2000-attempt limit. After 2000 misses the curve is resampled (same prime, next curve from the same seed stream). Nothing else changes.

**Why panel A stands.**
- The `U`, `Usupp` and `Ublock` code paths do not call the planted search, and the random stream they consume is unchanged.
- Check: the amended binary was rerun on panel A's `s = 3` cell (12 draws, the registered seed). Its output equals the registered binary's output line for line, except for `secs`.
- Panel A keeps the registered binary (`BINARY.sha256`). Panels B-E use the amended binary (`BINARY-2.sha256`).

**What is unchanged.** `PROTOCOL.md`, the panel table, the predictions and the decision rules.
