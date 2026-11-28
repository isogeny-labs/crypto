# Growth-law cells: rerun with per-degree progress (frozen 2026-11-15, before any row)

Owner's ask (2026-11-15): "go ahead with the engine fix and rerun". Budget set by the
owner: all three cells, 24 h each, USD 80 ceiling. Every value this run produces is
`[pcc:unverified]` until a claim is registered and signed.

## Why a rerun

The RunPod attempt of 2026-11-15 (`../falsify_runpod/`) ran both draws of `W = 1`, `t = 12`
to `--hky-cmax 7` and hit its 3 h timeout during the degree-6 closure. The pilot wrote rows only
when a run ended, so the timeout discarded even the decided degrees: no new value. The
recorded value stays `d_F ≥ 6` at `t = 12` (`../falsify/`, degree-5 closures complete without
`1` on both draws).

## The engine change (this branch)

- `src/cryptanalysis/hky_fp_tower.rs`: `fall_degrees_with(…, on_closure)` calls back with each
  closure `V_{F,c}` as soon as it is computed. `fall_degrees` is unchanged in behaviour (it
  calls `fall_degrees_with` with a no-op). Tests: the callback sees exactly the closures
  `fall_degrees` returns; resuming at a later `c_min` gives the same last fall degree.
- `examples/pkm_tower_pilot.rs`: `--hky-progress PATH` appends one JSON line per closure
  (`pkm_tower_pilot.hky_progress/v1`: cell, target, `c`, `dims`, `one`, `censored`, counts, ms),
  flushed and `fsync`ed. Resume with `--hky-cmin` at the first undecided degree.
- Local checks (development, not measurement): `W = 1`, `t = 6` gives `d_F = 4` with progress
  rows equal to the final row; a resumed run at `--hky-cmin 3` gives `d_F = 4`; a run killed by
  `timeout 25` at `t = 11` keeps degrees 2–4 in the progress file while its final-row file is
  empty.
- Not changed: a closure still in progress when the process stops is lost. Each closure is
  computed from scratch, so resuming at its degree repeats it.

## Cells, frozen

Same instrument and parameters as `../falsify_runpod/cells_job.sh`: `--engine hky --kinds
kummer --controls null --planted 0 --random 2 --p 2013265921 --seed 1347106132 --m 2`, two
draws (`--targets 0`, `--targets 1`) run concurrently, `RAYON_NUM_THREADS=8`,
`--hky-max-entries 2000000000`. Each closure's `--budget` equals the cell timeout.

| cell | W | t | `--hky-cmin` | `--hky-cmax` | timeout | registered prediction |
|:--|--:|--:|:--|--:|--:|:--|
| (a) | 1 | 12 | 6 (degrees ≤ 5 decided) | 7 | 24 h | `d_F = 6` |
| (b) | 3 | 13 | default (least input degree) | 7 | 24 h | `d_F = 7` |
| (c) | 2 | 14 | default | 7 | 24 h | `d_F = 7` |

Run order (a), (b), (c), one pod (`cpu5m`, 16 vCPU, 128 GB memory limit, as in
`../falsify_runpod/host.txt`). Script: `cells_job.sh`. Binary: built from this branch's
commit (recorded in `source_commit.txt`), SHA-256 in `binary.sha256`, checked on the pod
before any cell.

## Reading (fixed before data)

Per draw, from `out/prog_<cell>.jsonl` (the final `out/<cell>.jsonl` row when present):

- `1 ∈ V_c` first at the predicted degree, every lower computed closure uncensored and without
  `1`: the prediction **holds** for that draw.
- `1 ∈ V_c` first at a lower degree (only possible for (b), (c)), or `1 ∉ V_7` with `V_7`
  uncensored: the prediction **fails** (Conjecture G falsified at that cell).
- Otherwise (timeout, `censored`, or out of memory before the deciding closure): **censored**,
  reported with the highest decided degree as a bound (`d_F ≥ c + 1` if `1 ∉ V_c`).

A cell holds only if both draws hold; one failing draw falsifies it. No retries beyond this
plan; a second attempt needs a new plan.

## Spend and stop rules

- Hard ceiling USD 80. At the last attempt's rate (USD 3.17 for 3 h) the three 24 h cells are
  about USD 76. The pod is deleted when `ALL_DONE` appears, on any `BINARY HASH MISMATCH`, or when
  the spend would exceed the ceiling.
- The session polls the pod at most every 4 h and copies `out/` back each time, so a lost pod
  loses at most the closures in progress.
