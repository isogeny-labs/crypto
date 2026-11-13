# Cover-image factor bases, round 4: the sizes the container could not reach (RunPod, 2026-11-12)

Frozen before any measured cell of this round. Round 3
([`../cover_image_fb_r3_m3_20261112/`](../cover_image_fb_r3_m3_20261112/README.md)) closed the
cover-image line at the sizes a 16 GB container reaches: at m = 3, t = 4 every cover map was
1.55–1.99× wider than PKM's tower. It left the line open only at m = 3, t ≥ 5 and m = 4, which ran
out of memory locally. This round runs those cells on a RunPod CPU pod (memory-optimized,
16 vCPU, 128 GB, three arms at a time) with the same binary
(`examples/pkm_tower_pilot.rs`, unchanged since round 2; SHA-256 recorded in `runs/`).

## Cells (Kummer tower, p = 2013265921, pilot default seed, `--engine f4`)

- **A.** m = 3, t = 5, `tower` control; maps (none) = PKM tower, `quad`, `cubic`, `quart`,
  `generic3`.
- **B.** m = 4, t = 2 and 3, `tower` control; maps (none), `cubic`, `quart`, `generic3`.
- 1 planted + 1 random target per cell (`--planted 1 --random 1`). In rounds 1–3 W was the same
  for every target of a cell, so two targets suffice for W. With fewer targets the RNG stream
  differs from rounds 1–3, so these cells are new draws, not replays.
- Budget 3,600 s per system. The pod is deleted after the run, and in any case 10 hours after
  creation.
- **Nulls** (`--null-chain`, same seeds) run only for an arm that reaches R (below), as in
  round 3's rule that structure is read only after a positive result.

## Measure and gates

- **W** = widest F4 matrix (`max_cols`); a cell with a timed-out system is censored.
- **R (reopen).** For a map X at a cell above: `W_X / W_none ≤ 0.8`. In set B it must hold at
  t = 3.
- **C (closed).** `W_X / W_none ≥ 1` at the largest uncensored cell of each set.
- Otherwise inconclusive.
- Prior (stated before data): C for every map in both sets, 0.8.

## Not claimed

No `S`, no relation-phase cost, no rho comparison, no leaderboard or `benchmarks/` row. A stage
diagnostic of the decomposition solve, counts only.

## Amendment 1 (19:22Z, before any m = 4 cell had started)

- **Observed.** At 19:18Z the first (planted) system of `m3t5-none`, `m3t5-cubic` and
  `m3t5-quart` had each hit the 3,600 s budget. Set A is therefore censored for those three maps,
  and a censored cell enters no gate. The widths reached by the timeout (97,456 / 158,133 /
  94,973) are not final values and are not compared.
- **Change.** The remaining set-A work (the random targets of those three, and the `generic3` and
  `quad` arms at t = 5) cannot change a censored cell's status, so it is stopped. Set B (m = 4,
  t = 2 and 3) runs now, three arms at a time, unchanged otherwise (`pod_run_m4.sh`).
