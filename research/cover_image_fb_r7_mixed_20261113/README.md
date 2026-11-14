# Cover-image factor bases, round 7: the mixed m = 4 lead is closed (2026-11-13)

**Question.** Round 6 found m = 4, t = 3 **mixed** for every cover map: W 0.986–0.990× PKM's tower,
exactly, in every seed, but with about 1.17× the rows. Is that edge a real saving, and does it grow
with m?

**Answer: no on both counts** [pcc:unverified].

- **T (cost at m = 4, t = 3): closed for every map.** Run one process at a time on an otherwise idle
  pod, 6 systems per map, every cover map is **1.26–1.42× slower** than PKM's tower, with **1.09–1.15×
  the matrix area**. Every cover map's time range lies wholly above PKM's.
- **M5 (m = 5, t = 3): closed.** The `cubic` system has W = 250,143 against PKM's 171,293
  (**1.460×**), 1.40× the rows, and took 18,850 s against 16,828 s.

- Protocol: [`PROTOCOL.md`](PROTOCOL.md), pushed in `92f6f6d1ae` before any cell, with Amendment 1
  (`dd41261a32`, written before any M5 cell ran; its heading says 17:30Z, but it was written at
  about 17:23Z).
- Frozen binary of rounds 2–6 (SHA-256 `923a1c95…`, matched on every pod). RunPod pods: `cpu3g`
  8 vCPU 32 GB for T (17:20–18:30Z); two `cpu3m` 32 vCPU 256 GB for M5 (17:23Z until each arm
  ended). All deleted and confirmed gone.
- Tables: [`cells_T.tsv`](cells_T.tsv) ([`tabulate_T.py`](tabulate_T.py)). Rows and logs: `runs/T/`,
  `runs/M5/<pod name>/`. 0 planted violations.

## T: m = 4, t = 3, one process at a time (3 passes × 2 targets per map)

| map | W | rows | area / PKM | median F4 time (min–max) | time / PKM | gate |
|---|---|---|---|---|---|---|
| PKM tower | 21,619 | 25,929 | 1 | 58.5 s (55.4–61.3) | 1 | – |
| quad | 21,413 | 28,494 | 1.088 | 73.5 s (70.5–76.3) | 1.257 | closed |
| cubic | 21,316 | 30,308 | 1.153 | 82.9 s (80.2–85.9) | 1.417 | closed |
| quart | 21,316 | 30,308 | 1.153 | 83.2 s (80.4–86.3) | 1.422 | closed |
| generic3 | 21,319 | 30,311 | 1.153 | 83.2 s (80.4–86.7) | 1.424 | closed |

The 1 % narrower width is outweighed by 9–17 % more rows. Round 6's "mixed" verdict was a
width-only reading.

## M5: m = 5, t = 3 (18 unknowns, one arm per 256 GB pod)

| arm | W | rows | D (input) | F4 time | status |
|---|---|---|---|---|---|
| PKM tower | 171,293 | 179,020 | 8 (4) | 16,828 s | complete |
| `cubic` | 250,143 | 250,407 | 12 (12) | 18,850 s | complete, **closed (1.460×)** |

m = 5 at t = 2 cannot be planted (|V| = 4 < 5 distinct abscissae); see Amendment 1.

## The cover-image line, rounds 1–7

| relation size | largest depth measured | cover W / PKM W | verdict |
|---|---|---|---|
| m = 2 | t = 7–8 | 1.2–5.5× | falsifier |
| m = 3 | t = 4 | 1.55–1.99× (5 seeds, exact) | closed; t = 5 censored for every arm |
| m = 4 | t = 3 | 0.986–0.990× W, but 1.26–1.42× time and 1.09–1.15× area | closed on cost |
| m = 4 | t = 4 | 1.466× (`cubic`, `quart`) | closed |
| m = 5 | t = 3 | 1.460× (`cubic`) | closed |

No cover map makes the decomposition solve cheaper than PKM's tower at any size measured.

## Not claimed

No `S`, no relation-phase cost, no rho comparison, no leaderboard or `benchmarks/` row. Wall time on
a RunPod pod is a noisy proxy, read only as a ratio within one pod and one binary.
