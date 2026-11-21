# Cover-image factor bases, round 5: m = 4 at t = 4 closes the last open cell (2026-11-13)

**Question.** Round 4 ([`../cover_image_fb_r4_runpod_20261112/`](../cover_image_fb_r4_runpod_20261112/README.md))
left one open cell. At m = 4, t = 3 the cover maps were at parity with PKM's tower (W 0.986×).
Does a cover map pull ahead at t = 4?

**Answer: no. The cell is closed (gate C)** [pcc:unverified]. At m = 4, t = 4 the cubic cover's
system has W = 245,930 against PKM's 167,726 (1.47×), 1.79× the rows, about 2.6× the matrix
area, and took 1.84× as long. The near-parity at t = 3 does not survive one more level. The
`quart` and `generic3` arms are censored (memory, below).

- Protocol: [`PROTOCOL.md`](PROTOCOL.md), pushed in `cfaa2fa6bf` before any cell, with
  Amendments 1 (`aa4acf7fc7`, host and binary, at launch), 2 (`f715e44ae5`, memory, before any
  row) and 3 (`4f1f8f6871`, cubic rerun alone, before any cover-map row).
- Host: RunPod CPU pod `bdr-coverfb-r5` (`cpu3m`, 32 vCPU, 256 GB, $1.76/h), 23:04Z–08:36Z
  (about 9.5 h, about $17), then deleted and confirmed gone. The binary is the frozen one from
  rounds 2–4 (SHA-256 `923a1c95…`, matched on the pod).
- Rows and logs: `runs/`. One planted target per arm; both finished systems contain their planted
  solution (0 planted violations).

## Result

| arm | W | rows | D (input) | F4 time | status |
|---|---|---|---|---|---|
| PKM tower | 167,726 | 159,661 | 7 (4) | 10,884 s | complete (shared the pod with three arms) |
| `cubic` | 245,930 | 286,400 | 12 (12) | 20,059 s | complete (alone, Amendment 3) |
| `quart` | – | – | – | – | censored: stopped by PID at 01:08Z to free memory |
| `generic3` | – | – | – | – | censored: OOM-killed at 00:26Z |

`W_cubic / W_PKM = 1.466 ≥ 1`, so **gate C (closed)**. The time ratio is biased in the cubic's
favour, since it ran alone while the reference shared the pod, and it is still 1.84×. Each arm
draws its own curve and target (the cover-map draw moves the RNG stream), as in rounds 1–4.

## What happened to memory

All four arms at once did not fit in 256 GB. `generic3` was OOM-killed after 1 h 20 m. At 2 h the
three survivors held 89 GB (PKM), 67 GB and 67 GB, so `quart` was stopped to protect the
reference (Amendment 2). `cubic` was then OOM-killed (the second `oom_kill`) while sharing with
PKM, and was rerun alone: it held about 190 GB for most of its 5.6 h run. A degree-3 cover system
at m = 4, t = 4 needs about twice the memory of PKM's tower system.

**Correction to the raw log.** `runs/progress.txt` ends with the line `hard stop: m4t4-cubic still
running, censored`, written at 08:36:21Z. That line is wrong. The arm had already ended at
08:17:11Z with exit 0 (the preceding line). The process check before the hard stop returned an
empty list, which was misread. The row and its log are complete and uncensored.

## Across rounds 1–5

| relation size | largest depth measured | cover W / PKM W | verdict |
|---|---|---|---|
| m = 2 | t = 7–8 | 1.2–5.5× | falsifier (H0) |
| m = 3 | t = 4 | 1.55–1.99× | falsifier (H0); t = 5 censored for every arm at 1 h |
| m = 4 | t = 3 | 0.986× (rows 1.17×) | parity |
| m = 4 | t = 4 | **1.47×** (rows 1.79×) | **closed (C)** |

No cover map made the decomposition solve cheaper than PKM's tower at any size measured. Wherever
a depth step was measured, the cover systems fell further behind. The cover-image factor-base line
is closed.

## Not claimed

No `S`, no relation-phase cost, no rho comparison, no leaderboard or `benchmarks/` row. A stage
diagnostic, counts only. One target per arm; W has been identical across targets in every
earlier cell, but this cell has not been checked for that.
