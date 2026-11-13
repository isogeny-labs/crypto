# Cover-image factor bases, round 5: m = 4 at t = 4 (RunPod, 2026-11-12)

Frozen before any measured cell of this round. Round 4
([`../cover_image_fb_r4_runpod_20261112/`](../cover_image_fb_r4_runpod_20261112/README.md)) left
one open cell: at m = 4, t = 3 every cover map was at parity with PKM's tower (W 0.986×, rows
1.17×), and t = 4 was not measured. This round measures m = 4, t = 4 (|F| = 16, 16 tower unknowns
plus 2 chain unknowns) with a much longer budget.

## Cell

`examples/pkm_tower_pilot.rs --engine f4 --kinds kummer --m 4 --t-min 4 --max-t-m4 4 --controls
tower --planted 1 --random 0`, p = 2013265921, pilot default seed, budget 28,800 s (8 h) per
system. Maps: (none) = PKM tower, `cubic`, `quart`, `generic3`. The binary is the same one as
rounds 2–4 (SHA-256 `923a1c95…`). Host: RunPod CPU pod, `cpu5m`, 32 vCPU, 256 GB, four arms at
once. The pod is deleted after the run, and in any case 9.5 h after creation.

One planted target per map: in rounds 1–4 W was the same for every target of a cell. With
`--random 0` the RNG stream differs from round 4, so this is a new draw of curve and target, not a
replay of round 4's t = 3 cell.

## Measure and gates

- **W** = widest F4 matrix (`max_cols`), with rows and D reported. A timed-out system is censored
  and enters no gate.
- **R (reopen).** `W_X / W_none ≤ 0.8`.
- **C (closed).** `W_X / W_none ≥ 1`.
- Otherwise inconclusive. If R holds for a map, its `--null-chain` null at the same cell runs next
  (a new protocol amendment), before any structural reading.
- Prior (before data): C or inconclusive for every map, 0.85; at least one censored arm, 0.4.

## Not claimed

No `S`, no relation-phase cost, no rho comparison, no leaderboard or `benchmarks/` row. A stage
diagnostic of the decomposition solve, counts only.

## Amendment 1 (23:06Z, at launch, before any result)

- **Host.** No `cpu5m` 32 vCPU capacity was available. The run uses `cpu3m` (32 vCPU, 256 GB, the
  same memory, $1.76/h). Pod `bdr-coverfb-r5` (`c667gkge6yjsiv`), ready 23:05Z. Arms started
  23:05:50Z. The container's memory limit reads 256,000,000,000 bytes.
- **Binary.** The binary is the frozen one from rounds 2–4 (built at `3466f3be9e`, SHA-256
  `923a1c95…`, verified on the pod). `main` has since gained a performance pass on
  `src/cryptanalysis/f4_fp.rs` (`fd55624eed`, `3afdf60d9b`, `dc2e674c47`). It is not used here,
  to keep this cell comparable with rounds 2–4.

## Amendment 2 (01:09Z, before any arm had written a row)

- **Observed.** `m4t4-generic3` was killed by the kernel at 00:26:32Z (exit 137; the pod's
  `memory.events` shows `oom_kill 1`). At 01:08Z the remaining arms held 89 GB (PKM), 67 GB
  (`cubic`) and 67 GB (`quart`), and the container 242 of 256 GB. No arm had finished a system.
- **Change.** `m4t4-quart` was stopped by PID at 01:08:39Z, freeing about 67 GB, so that the
  reference arm (PKM) and one cover arm (`cubic`) can finish. A further OOM kill would most likely
  take the largest process, the reference, and then no gate could be read. `cubic` and `quart` gave
  identical W at m = 4, t = 2 and t = 3 in round 4, so `cubic` stands for both.
- **Consequence.** `generic3` and `quart` are censored at m = 4, t = 4. The gates are read for
  `cubic` only.
