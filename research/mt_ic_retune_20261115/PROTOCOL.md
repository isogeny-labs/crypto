# Multi-target IC at 2^32: retune k and μ for the batched table (2026-11-15)

Frozen before any measured run. Setting: `../mt_rho_rematch_20261115/` left the multi-target IC with the batched,
prefetched flat table (`ic5b flat 32`, "IC-B") at 1.05× the best tuned rho at 2^32 on an EPYC 4564P (1.017–1.226
per instance) [pcc:unverified]. IC-B still uses `k = 4, μ = 1.0`, which `../mt_fieldops_v3_20261111/` chose by
a counted-operation cost model (I = 10M, h = 0.1) on one instance, with k ≤ 4 allowed. The batched table made
target probes cheaper than that model assumed. A smaller table (smaller μ) moves work from the
precompute/relation phases into target trials, and a larger k gives the same table from a smaller factor base.
This round picks (k, μ) by wall time on a tune set and checks the choice on a holdout set against the default and
against the full rho grid.

## Instrument: `examples/mt_fieldops.rs`

One change: `ic5b` accepts k = 5 (the assert was `2..=4`). Nothing in the arm depends on k ≤ 4. Local pre-run
check on W32-0: every k ∈ {3, 4, 5} at μ ∈ {0.6, 1.0} (T = 4096), and (5, 0.4), (3, 1.2), (4, 1.0) at
T = 1024, recover every target.

## Keys

`ic5b <inst> T 43 μ k 1.2 flat 32`, for each grid point of k ∈ {3, 4, 5} × μ ∈ {0.4, 0.5, 0.6, 0.7, 0.85, 1.0,
1.2}: 21 keys. The third argument (g = 1.2) only sets a panic budget in this arm. Default = (k = 4, μ = 1.0) = IC-B.

Rho keys, holdout only: the rematch's 20 keys: `rho2xks` and `rho2xksp` at κ ∈ {0.05, 0.1, 0.25, 0.5}; `rhodpu`
and `rhodpup` at κ ∈ {0, 0.25} × offset ∈ {−1, 0, 1}.

## Phases

- **smoke:** W32-0, T = 1024, 4 keys, 1 repeat (also run locally; 0 errors).
- **tune:** instances W32-{0..3} of `../mt_fieldops_v5_20261112/instances/`; T ∈ {1024, 4096, 16384}; seed 43;
  12 configurations × 21 keys × 3 repeats = 756 executions.
- **choice:** `python3 analyze.py tune` writes `frozen_choice.json` and is committed before the holdout runs.
  Rule: argmin over the 21 keys of the geometric mean over the 12 tune configurations of
  `ms_arm(key) / ms_arm(default)`, where a key's time is the median of its 3 repeats. Ties go by key name. One
  (k, μ) for every T: μ already scales the table with √T.
- **holdout:** instances W32-{4..7}, never used in the tune; the same 3 T cells; keys IC-default, IC-chosen and
  the 20 rho keys: 12 × 22 × 3 = 792 executions.

Execution as in the rematch: one process at a time, pinned to one CPU, repeats interleaved in a fixed-seed
shuffled order (tune 20261115, holdout 20261116). Host: one RunPod `cpu5c` pod for both phases, deleted after the
run and in any case 4 h after creation. The binary is built once locally; its SHA-256 is checked on the pod.

## Gates (holdout)

Per configuration, a key's time is the median of its 3 repeats of `ms_arm` (the whole solve). Best rho = min over
the 20 rho keys. Each T cell has 4 configurations; a cell decision is a 4-of-4 sign rule.

- **Correctness (stops the reading):** every execution recovers every target.
- **Primary (IC baseline cost):** IC-chosen / IC-default < 1 in 4 of 4 instances:
  - **lower:** in all 3 T cells;
  - **not lower:** ≥ 1 in all 3 cells;
  - otherwise **mixed**.
- **Distance remaining:** IC-chosen / best rho, median and per instance, per cell; IC-default / best rho beside
  it. This is reported, not a gate. "IC ahead" in a cell means ≤ 1 in 4 of 4.

If the tune picks the default, the holdout still runs; the primary is then "not lower" by construction and the
round reports the default's distance to rho on new instances.

Priors (stated before data): the choice differs from the default, 0.8; primary "lower", 0.55; IC ahead of rho
in at least 2 of 3 cells, 0.2.

## §5a pricing

Every arm is timed whole: all of its native work is inside `ms_arm`, so the 0 / 0.1 / 1 GAE band of unpriced
native work is empty for every arm, rho included. Rankings are in wall time on one host.

## Not claimed

No `S`, no single-target change, no BSGS result, no leaderboard or `benchmarks/` row. Wall time on a shared cloud
pod, one binary, read as paired ratios. No replay certificate or validator report. Only 2^32 is measured, so a
trend in group size (2^34 and up) is out of scope.
