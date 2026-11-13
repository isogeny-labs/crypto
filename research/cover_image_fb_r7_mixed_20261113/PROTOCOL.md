# Cover-image factor bases, round 7: following the mixed m = 4 lead (2026-11-13)

Frozen before any measured cell of this round. Round 6 set S
([`../cover_image_fb_r6_replication_20261113/`](../cover_image_fb_r6_replication_20261113/PROTOCOL.md))
found the m = 4, t = 3 cell **mixed** for every cover map: W is 0.986–0.990× PKM's tower, exactly
the same in all five seeds. W is fixed by the shape of the system, so more seeds cannot move it, and
the same cells have about 1.17× more rows. This round asks whether that edge is a real saving, and
whether it grows with m.

Instrument: the frozen binary of rounds 2–6 (built at `3466f3be9e`, SHA-256 `923a1c95…`),
`--engine f4 --kinds kummer --p 2013265921 --ladder-t none`, default seed (1347112276) unless
stated. RunPod CPU pods via `../runpod_kit_20261112/rp.sh`. Every arm is one explicit command (no
`xargs -L`, whose line-continuation rule cost round 6 fifteen arms).

## T: cost at m = 4, t = 3, one process at a time

- Pod: `cpu3g`, 8 vCPU. Nothing else runs on it.
- Maps: (none), `quad`, `cubic`, `quart`, `generic3`. `--m 4 --t-min 3 --max-t-m4 3
  --controls tower --planted 1 --random 1`, budget 3,600 s.
- Order: three passes. Each pass runs the five maps in a different fixed order (pass 1: none,
  quad, cubic, quart, generic3; pass 2 reversed; pass 3: cubic, none, generic3, quad, quart), one
  at a time.
- Measures per map: median F4 wall time over its 6 systems (3 passes × 2 targets), W, rows,
  W × rows ("area").
- **T-reopen**: `time_X / time_none ≤ 0.8` and `area_X / area_none ≤ 1`.
- **T-closed**: `time_X / time_none ≥ 1` or `area_X / area_none ≥ 1.1`.
- Otherwise T-mixed.

## M5: does the edge grow with m?

- Pod: `cpu3m`, 32 vCPU, 256 GB. Arms in parallel.
- m = 5, t = 2 and t = 3, control `tower`, 1 planted + 1 random target, budget 7,200 s per system.
  Maps: (none), `quad`, `cubic`, `quart`, `generic3`.
- **M5-reopen** for map X: `W_X / W_none ≤ 0.8` at t = 3 (or at t = 2 if t = 3 is censored for X or
  for the reference).
- **M5-closed**: `W_X / W_none ≥ 1` at the largest uncensored t.
- Otherwise mixed.

## Prior and stopping

Prior (before data): T-closed for every map, 0.75 (the extra rows should cost more than the 1 % W
edge saves). M5 closed or mixed for every map, 0.85. If anything reaches a reopen gate, the next
step is a round with the null control and more targets at that cell, not a claim.

Hard stops: T pod 3 h, M5 pod 4 h after creation. Budget about $10.

## Not claimed

No `S`, no relation-phase cost, no rho comparison, no leaderboard or `benchmarks/` row. Wall time
on a RunPod pod is a noisy proxy; T is read as a ratio within one pod and one binary only.
