# Cover-image factor bases, round 4: m = 3 t = 5 and m = 4 on RunPod (2026-11-12)

**Question.** Round 3 ([`../cover_image_fb_r3_m3_20261112/`](../cover_image_fb_r3_m3_20261112/README.md))
closed the cover-image line at the sizes a 16 GB container reaches. It left only m = 3, t ≥ 5 and
m = 4 open. Does either reopen it on a larger host?

**Answer: no** [pcc:unverified]. m = 3, t = 5 is censored: every arm, PKM's tower included, timed
out at the 3,600 s budget on its first system. At m = 4 every cover map is at parity with PKM's
tower, with W 0.986× at t = 3. That is inside neither gate, so inconclusive on the letter. With
1.17× more rows, the cover matrices are about 1.15× larger by area. Nothing here reopens the line.

- Protocol: [`PROTOCOL.md`](PROTOCOL.md), pushed in `1e6d4d0d37` before any cell, with Amendment 1
  (`4e1093bfbb`), written before any m = 4 cell started.
- Host: RunPod CPU pod `bdr-coverfb-r4` (`cpu5m`, 16 vCPU, 128 GB, $1.04/h), 18:18–20:10Z,
  deleted after the run. Driven with [`../runpod_kit_20261112/`](../runpod_kit_20261112/README.md).
  The binary's SHA-256 on the pod (`runs/binary.sha256`, `923a1c95…`) matches the local build of
  `main`'s pilot.
- Scripts: [`pod_run.sh`](pod_run.sh) (set A), [`pod_run_m4.sh`](pod_run_m4.sh) (set B). Rows and
  logs: `runs/`. 1 planted + 1 random target per cell; 0 planted violations.

## Set A: m = 3, t = 5 (censored)

| map | W reached at timeout | D reached (input) | rows | resident memory at timeout |
|---|---|---|---|---|
| PKM tower | 97,456 | 7 (4) | 116,926 | 15 GB |
| cubic | 158,133 | 12 (12) | 155,390 | 32 GB |
| quart | 94,973 | 16 (16) | 122,470 | 41 GB |

Each arm's first (planted) system hit 3,600 s. A censored cell enters no gate. These widths are
where F4 was stopped, not final values. Per Amendment 1 the rest of set A (random targets,
`generic3`, `quad`) was stopped, because it could not un-censor these cells. The binding limit
here is time, not memory.

## Set B: m = 4 (complete)

| map | t = 2: W (ratio) | t = 3: W (ratio) | t = 3: rows | D (input) | gate |
|---|---|---|---|---|---|
| PKM tower | 1,911 (1) | 21,619 (1) | 25,929 | 7 (4) | reference |
| cubic | 1,815 (0.95) | 21,316 (0.986) | 30,308 | 12 (12) | inconclusive |
| quart | 1,815 (0.95) | 21,316 (0.986) | 30,308 | 16 (16) | inconclusive |
| generic3 | 1,815 (0.95) | 21,319 (0.986) | 30,311 | 13 (13) | inconclusive |

The protocol reopens a map at W ratio ≤ 0.8 and closes it at ≥ 1. 0.986 is neither. The three
cover maps give nearly identical matrices whatever their degree, so the shape of the m = 4 chain
dominates, not the map. Times (46–159 s per system against PKM's 95–154 s) come from three
processes sharing the pod, and are descriptive only.

## Reading across rounds 1–4

| relation size | largest depth measured | cover W / PKM W | verdict |
|---|---|---|---|
| m = 2 | t = 7–8 | 1.2–5.5× | falsifier (H0) |
| m = 3 | t = 4 | 1.55–1.99× | falsifier (H0); t = 5 censored even at 128 GB |
| m = 4 | t = 3 | 0.986× (rows 1.17×) | parity, inconclusive |

No cover map tested makes the decomposition solve cheaper than PKM's tower at any measured size,
and at m = 2 and m = 3 every map is clearly costlier. The line stays closed. Reopening it would
need m = 4 at t ≥ 4 with a much larger time budget (m = 3, t = 5 already exceeds an hour per
system for PKM's own tower), and the m = 4 parity gives little reason to expect a gain there.

## Not claimed

No `S`, no relation-phase cost, no rho comparison, no leaderboard or `benchmarks/` row. A stage
diagnostic, counts only. Nulls were not run: the protocol runs them only for a map that reopens.
