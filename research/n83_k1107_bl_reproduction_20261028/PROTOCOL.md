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

**Amendment 1 (2026-10-28, after the first run and before any IC measurement).** The first run,
`records/run-20261028T1156Z`, started at 11:56:52Z. The IC arm and all three precomputed-rho arms
exited with status 127 at launch: `env: '/usr/bin/time': No such file or directory`. This host lacks
GNU time, which `run.sh` uses for the resource receipts, while the GitHub runners the 2026-10-16
control used have it.

Only the empty-table plain-rho positive control ran (16 targets in about 45 s). No IC, precomputed-rho
or analysis output exists. That directory is kept unchanged as the record of the failed attempt.

The fix is to install the distribution package `time` (GNU Time) at `/usr/bin/time`. Nothing in
`run.sh`, the binaries (`scratchpad/n83bin/`, same SHA-256s), the configs, the targets or the seeds
changes. The run is repeated once into a new directory, and that second run is the decision-bearing
one. Its plain-rho control is a fresh re-run under the same seeds, not a reuse of the first.

**Amendment 2 (2026-10-28, before any precomputed-rho output existed).** Run 2,
`records/run-20261028T1159Z`, finished the IC arm (16/16 targets, 11:59:49 to 12:02:28Z) and the
plain-rho control. It was then killed during the precomputed-rho arm (a), which started at
12:03:11Z, by a restart of the container itself (`uptime` reset at about 13:47Z). Arm (a) left
an empty `records.jsonl` and no stderr, and no analysis ran. The directory is kept unchanged as
evidence.

The run is repeated once more as run 3, into a new directory, under the same inputs, binaries,
configs, seeds and `run.sh`. Run 3 is the decision-bearing run. Run 2's partial IC records are
not combined with run 3's, because AGENTS.md forbids combining results from different
experiments.

A further infrastructure kill would need another dated amendment, not a silent retry.

The owner's default EKS route (`isolab-eks`, 2026-10-25) does not fit this study as written. The
AWS request contract (`isolab.eks.request/v1`) takes only an `ecbench.spec/v1` job, `ecbench`
cannot run IC at m = 83, and custom commands need the EKS MCP `isolab_k8s_run`, which this session
lacks. The study therefore stays L0 on this VM, as section 3 already states.

**Amendment 3 (2026-10-28, before any precomputed-rho output existed).** Run 3,
`records/run-20261028T1354Z`, finished the IC arm (16/16 targets, 13:55:00 to 13:57:41Z) and the
plain-rho control. It was then killed during precomputed-rho arm (a), which started at 13:58:25Z,
by another container restart (uptime reset; observed at 15:10Z). Arm (a) left no records, and no
analysis ran. The directory is kept unchanged as evidence.

Cause: both kills fell in long stretches when the session issued no commands, while the run was a
detached `setsid nohup` process the session harness does not track. The environment reclaims idle
containers, and an untracked process does not keep one alive. On the 2026-10-16 runners arm (a)
took about 11 minutes, and every whole job finished in under 2.2 h.

Fix: run 4 executes the same `run.sh` with the same inputs, binaries, configs and seeds as a
harness-tracked background task. Nothing in the experiment changes. Run 4 is the decision-bearing
run. Runs 2 and 3 are not combined with it.
