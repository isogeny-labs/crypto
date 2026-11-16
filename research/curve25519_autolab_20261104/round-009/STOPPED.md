# Round 9: stopped before assessment (censored)

Round 9 (generic, k = 32, 32 train targets) was frozen and started, then stopped by the operator after 10 train records (2026-11-04). Its panel held `rootfind-pairs:m=3` at N = 179. Every earlier screen of that family cost 70x to 320x the incumbent, and at k = 32 with 32 targets it could hold the host for hours under the 1800 s per-run cap.

The interrupted session `sessions/train-k32` is kept as evidence and is never rerun; its audit with `--allow-interrupted` is `analysis/verify-train-k32-interrupted.json`. The round is censored, not assessed: no selection, no holdout, no ratchet. Its train target seed counts as exposed in `ledger.json`. The families dominated in every screen (`rootfind-pairs:m=3`, `mitm:m=3`, `rootfind-subtract`, `subtract`) are disabled in `families.json`, and the next round re-proposes k = 32 without them.
