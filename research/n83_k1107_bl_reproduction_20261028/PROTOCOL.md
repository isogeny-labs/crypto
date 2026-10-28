# N83 K1107 baseline against matched precomputed rho: reproduction protocol

**Status: FROZEN 2026-10-28, before any run.** It is committed before
`run.sh` executes. Any change after the first run is a dated amendment
below, never an edit.

## 1. Question and class

On the legacy N83 Koblitz curve, the current cold IC baseline is
K1107/root11 dynamic (`research/n83_cold_base_confirm_20261021/RESULT.md`).
Does it beat Pollard rho online once rho gets matched precomputation?
The 2026-10-16 control (`research/20261016-n83-precomputed-rho-control/`)
answered this for K3300 and K600 only.

**Class:** reproduction and diagnostic. This is not an improvement claim,
a §8a gate result or a leaderboard row (§4).

## 2. Frozen inputs

| item | value |
|:--|:--|
| curve | `EC1N83Ckb1a1haacb69ee9f37`, `y²+xy=x³+x²+1`, r = 8569786107849059, cofactor 1128547018 (the legacy curve, **not** §8a E₀; see [E0_FEASIBILITY.md](E0_FEASIBILITY.md)) |
| targets | the 16 public points of the 2026-10-16 control, `research/20261016-n83-precomputed-rho-control/targets.jsonl`, checked against its `targets.sha256`. The points are already exposed, so this is a reproduction panel, not a fresh holdout |
| answers | that study's `reference_scalars.txt`, read only by the analyzer |
| IC arm | `koblitz_orbit_dlp_fast construct:83:1:1107 targets.jsonl 29 records.jsonl`, one process for all 16 targets, `taskset -c 0-3`, with the exact `root11_dynamic` environment of `research/n83_cold_neighborhood_20261016/run.py` `environment()` (listed in `run.sh`) |
| plain rho (positive control) | `n83_rho_optimization_eval`, one empty-table fold-rho process per target, seed `2026102800 + i`, config `research/n83_dense_root_20261013/rho_config.json` (SHA-256 pinned in `run.sh`) |
| precomputed rho (a) | `configs/bl_a_time_matched.json` of the 2026-10-16 control: one thread, 650 s table walks |
| precomputed rho (a′) | `configs/bl_a2_wall_equal.template.json`, budgeted at this job's IC target-independent wall minus 2 s (that study's amendment 1 rule) |
| precomputed rho (b) | `configs/bl_b_memory_matched.template.json`: table bytes equal this job's IC max RSS, 4 threads |
| analysis | `n83_precomputed_rho_analyze <job> k1107 analysis.json`. It replays every recovered scalar with the generic library curve, requires all arms to agree, and reports online medians with 10,000-resample bootstrap intervals and §5a operation counts at 0, 0.1 and 1 GAE |
| source | the commit this protocol is committed in; binaries are built from it with `cargo build --release --locked` and hashed in `host/binaries.sha256` |

## 3. Host and isolation

This is an L0 run on a shared 4-vCPU cloud VM with about 15 GB RAM and no
hardware PMU. `run.sh` records lscpu, meminfo, cgroup limits and the load
average at start and end. It refuses to start while the 1-minute load
average is above 1.0, so that it does not overlap other measurement jobs,
unless `N83_ALLOW_BUSY=1` is set. When the override is used, the host
receipt says so.

## 4. Decision rule (frozen)

For each matched arm (a), (a′) and (b), compute the median paired ratio of
IC online to precomputed-rho online over the 16 targets, with its 95%
interval. If the interval lies entirely above 1, precomputed rho is faster
online. If it lies entirely below 1, IC is faster. Otherwise the result is
unresolved.

Cold cost (IC whole process against rho table build plus online) is
reported beside it. No arm is ranked on operation counts unless the order
holds across the §5a band.

This decision describes this L0 host only. It does not count as a
validation-gate rho reference, because the gate needs a replay
certificate, a validator report and a Tier 1 or L2 host. It promotes
nothing to `docs/ic/LEADERBOARD.md` or `benchmarks/README.md`.

## 5. Prediction (before any run)

Precomputed rho is faster online in every matched arm, as it was for K600
and K3300 (IC/rho median 3.7–395×). The 2026-10-21 Tier 1 measurement put
K1107 online at about 63 ms per target. That is slower than the
precomputed arms' 1–8 ms on the 2026-10-16 runners.

## 6. What does not count

Comparing against empty-table rho (the positive control) does not count;
those ratios are the contradicted readings on the leaderboard. Neither
does any wall ratio taken while the load gate was overridden, or any
change of K, seed, targets or configs after the first run.

## Amendments

None.
