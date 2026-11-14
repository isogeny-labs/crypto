# Cover-image factor bases, round 6: seed replication and the censored m = 4, t = 4 arms (2026-11-13)

Frozen before any measured cell of this round. Rounds 1–5 (`../cover_image_fb_20261112/` …
`../cover_image_fb_r5_m4t4_20261112/`) used one curve/tower/target draw per arm and cell, and left
`quart` and `generic3` censored at m = 4, t = 4. This round adds draws (set S) and fills the
censored arms, plus a second draw of the decisive m = 4, t = 4 comparison (set L).

Instrument: the frozen binary of rounds 2–5 (`examples/pkm_tower_pilot.rs` built at `3466f3be9e`,
SHA-256 `923a1c95…`), `--engine f4 --kinds kummer --p 2013265921`, RunPod CPU pods driven with
`../runpod_kit_20261112/rp.sh`. The `--seed` flag changes the curve, tower and targets drawn.

## Set S: seed replication (one pod, many arms in parallel)

- Seeds 101, 102, 103, 104, 105. Maps: (none) = PKM tower, `quad`, `cubic`, `quart`, `generic3`.
- Cell S3: m = 3, t = 4, controls `tower,null`, `--null-chain`, 1 planted + 1 random target.
- Cell S4: m = 4, t = 3, control `tower`, 1 planted + 1 random target.
- Budget 3,600 s per system. Pod deleted after the set, and in any case 3 h after creation.

## Set L: m = 4, t = 4 (one arm per pod, the whole 256 GB each)

- L-quart: `quart`, default seed (1347112276), the arm censored in round 5.
- L-generic3: `generic3`, default seed, the arm censored in round 5.
- L-none-202 and L-cubic-202: PKM tower and `cubic` at seed 202, a second draw of round 5's
  decisive comparison.
- 1 planted target, `--random 0`, control `tower`. Budget 25,200 s (7 h) per system. Each pod is
  deleted when its arm ends, and in any case 7.5 h after creation.
- L-quart and L-generic3 are compared with round 5's PKM reference at the default seed
  (W = 167,726). L-cubic-202 is compared with L-none-202.

## Measure and gates

- W = widest F4 matrix (`max_cols`); rows, D and time reported. A cell with a timed-out or
  OOM-killed system is censored and enters no gate.
- **Set S**, per cell and map X, with `r_s = W_X,s / W_none,s` for each seed s:
  - **closed** if `r_s ≥ 1` for at least 4 of the 5 seeds;
  - **reopen** if `r_s ≤ 0.8` for at least 4 of the 5 seeds;
  - otherwise **mixed**.
  - Also reported: the spread of `W_none,s` across seeds (is W fixed by the system's shape?), and
    in S3 the `W_X / W_null,X` ratios.
- **Set L**, per arm: **closed** if `W_X / W_ref ≥ 1`, **reopen** if `≤ 0.8`, otherwise
  inconclusive.
- Prior (before data): S3 closed for every map, 0.9. S4 mixed or closed for every map, 0.85 (round
  5's 0.986 was within 2 % of parity). Every L arm closed or censored, 0.85.

## Cost guard

At most about $60 in all: one `cpu3m` 32 vCPU pod for set S (at most 3 h) and four for set L (at
most 7.5 h each), at $1.76/h, with hard-stop reminders for every pod.

## Not claimed

No `S`, no relation-phase cost, no rho comparison, no leaderboard or `benchmarks/` row. A stage
diagnostic of the decomposition solve, counts only.
