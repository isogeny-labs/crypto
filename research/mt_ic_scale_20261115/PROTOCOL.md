# Multi-target IC vs tuned rho at 2^32, 2^34 and 2^36: does the gap grow? (2026-11-15)

Frozen before any measured run. Setting: `../mt_ic_retune_20261115/` retuned IC-B to k = 4, μ = 0.85, which put
it at 1.06–1.15× the best rho at 2^32 on an EPYC 9575F [pcc:unverified]. Earlier rounds showed the IC's ratio to
rho growing with the group (about 0.88 at 2^28, about 1.05–1.1 at 2^32). This round measures 2^34 and 2^36, with
2^32 on the same host, to see whether the gap keeps growing and whether μ = 0.85 stays the best of its
neighbours.

## Instrument: `examples/mt_fieldops.rs`

- The flat table gets a 16-byte-slot variant (u64 key) used when p ≥ 2^32; for p < 2^32 the 8-byte form is
  unchanged. Insert, get and prefetch become generic helpers shared by both forms. Pre-run check: on W28-0,
  T = 1024, `ic5b flat 32`, `rho2xksp` and `rhodpup` give byte-identical output (all fields but times) to the
  previous binary.
- `ic5b` also reports the time at the end of each precompute phase (`ms_at_fb`, `ms_at_combos`, `ms_at_table`,
  `ms_at_relations`). Only timestamps are added; no counted operation changes.

## Instances

`instances/`: W34-{0..3} and W36-{0..3}, made with `../ic_e222_m221_20261106/gen_instances.py --families W --bits
34,36 --per 4 --seed mtfo-scale-20261115`. For 2^32: W32-{4..7} of `../mt_fieldops_v5_20261112/instances/`, the
retune holdout set.

## Keys per configuration (19)

- IC: `ic5b <inst> T 43 μ 4 1.2 flat 32` with μ ∈ {0.7, 0.85, 1.0}.
- Rho:
  - `rho2xksp` at κ ∈ {0.05, 0.1, 0.25, 0.5};
  - `rhodpu` and `rhodpup` at κ ∈ {0, 0.25} × offset ∈ {−1, 0, 1}.
- `rho2xks` is left out: it was never the best rho in the rematch or the retune holdout, where its tuned form
  `rho2xksp` was in every grid.

## Configurations and order

3 sizes × 4 instances × T ∈ {1024, 4096, 16384} = 36 configurations; seed 43; 36 × 19 × 3 repeats = 2,052
executions. Execution follows the rematch:
- one process at a time, pinned to one CPU;
- repeats interleaved in a fixed-seed shuffled order (20261117);
- one RunPod `cpu5c` pod, deleted after the run and in any case 4 h after creation.

## Measures and gates

A key's time is the median of its 3 repeats of `ms_arm`, the whole solve. Best rho is the min over the 16 rho
keys per configuration. A cell value is the median of the cell's 4 configurations.

- **Correctness (stops the reading):** every execution recovers every target.
- **Primary (size trend):** compare IC (μ = 0.85) / best rho at 2^36 with 2^32, per T:
  - **gap grows** if larger at 2^36 in all 3 T cells;
  - **gap shrinks** if smaller in all 3;
  - otherwise **mixed**.
- **Secondary:**
  - IC / best rho in every cell (the distance remaining);
  - whether μ = 0.85 has the smallest cell median of the three μ, per cell;
  - "IC ahead" in a cell if ≤ 1 in 4 of 4.

Priors (stated before data): gap grows, 0.75; μ = 0.85 best in at least 6 of 9 cells, 0.5.

## §5a pricing

Every arm is timed whole, so all native work is inside `ms_arm`. The 0 / 0.1 / 1 GAE band of unpriced native work
is empty for every arm, rho included.

## Not claimed

- No `S`, no single-target change, no BSGS result.
- No leaderboard or `benchmarks/` row: there is no replay certificate or validator report.
- Wall time on a shared cloud pod, one binary, read as paired ratios.

## Amendment A1 (before any measured run)

RunPod refused the pod ("account balance is too low"). The run moves to this session's container: Intel Xeon
@ 2.30 GHz, 4 vCPU, 15 GB, pinned to one CPU, with the same order, repeats and binary as above. The only change is
the host. Absolute times are not comparable to the RunPod rounds; ratios within this run are.
