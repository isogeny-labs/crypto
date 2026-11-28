# Hosts for cap 7 at `t = 14` above 112Gi: RunPod CPU pods, threads, wall time and cost

**Every number in this file is `[pcc:unverified]`.** The catalog figures are RunPod's own answers to read-only GET
calls; the rest is derived from them and from `MEMORY.md`. No pod was created and no paid call was made.

## 1. The catalog (measured, 2026-11-21 20:45-20:46Z)

Calls: `GET https://api.runpod.io/v2/catalog/cpus?include=AVAILABILITY&product=POD`, then
`GET /v2/catalog/cpus/{cpu5m,cpu3m}?include=AVAILABILITY&product=POD&vcpuCount={16,32}`, with the session's key in a
mode-0600 header file (`curl -H @file`). The responses are saved unchanged except for key order and indentation in
`catalog/` (no account fields). Memory per pod is `ramGbPerVcpu × vCPU`; price is `price.securePerVcpu × vCPU`, the
reserved (on-demand) tier, which is the only pod tier v2 offers (`runpod_capped/V2_FIELDS.md`). `[pcc:unverified]`

Every CPU flavor RunPod lists, at its largest size (32 vCPU for all six):

| flavor | group | RAM per vCPU | largest pod | price per vCPU-hour | largest pod per hour | catalog availability | data centers listed |
|:--|:--|--:|--:|--:|--:|:--|:--|
| `cpu3c` | CPU3 Compute-Optimized | 2 GB | 64 GB | US$0.030 | US$0.96 | HIGH | 16 |
| `cpu3g` | CPU3 General Purpose | 4 GB | 128 GB | US$0.040 | US$1.28 | HIGH | 16 |
| **`cpu3m`** | CPU3 Memory-Optimized | 8 GB | **256 GB** | US$0.055 | **US$1.76** | HIGH | 16 (at 32 vCPU: 9, eight HIGH and `US-NE-1` MEDIUM) |
| `cpu5c` | CPU5 Compute-Optimized | 2 GB | 64 GB | US$0.035 | US$1.12 | HIGH | 2 |
| `cpu5g` | CPU5 General Purpose | 4 GB | 128 GB | US$0.046 | US$1.47 | HIGH | 2 |
| **`cpu5m`** | CPU5 Memory-Optimized | 8 GB | **256 GB** | US$0.065 | **US$2.08** | HIGH | 2 (`EU-RO-1`, `EUR-IS-1` at 16 vCPU; **none listed at 32 vCPU**) |

**Flavors with memory of at least 256 GB: two, `cpu3m` and `cpu5m`, each only at 32 vCPU, each exactly 256 GB.**
Nothing larger exists among RunPod CPU pods. `cpu5m` at 32 vCPU answered `availability HIGH` with an empty
`dataCenters` list (`catalog/cpu5m-32_20261121T2045Z.json`); whether a 32-vCPU `cpu5m` pod can be placed now is
therefore not shown by the catalog. The two 112Gi pods of Amendment 31 ran on exactly this size on 2026-11-15
(`results/SNAPSHOT_20261117/pfd-x-t14-k0-c7-m112-rp/status.json`: `cost_per_hr 2.08`, `cpus 32`,
`memory_bytes 256000000000`; host CPUs Threadripper 7960X and EPYC 9575F). The CPU models behind `cpu3m` are not
recorded in any study file. `[pcc:unverified]`

## 2. Which host fits which band (derived)

A 256 GB pod has a cgroup limit of 256·10⁹ bytes = 238.4 GiB (the 128 GB smoke pods showed `memory.max`
128,000,000,000, `runpod_capped/SMOKE.md`; to be read before start). `[pcc:unverified]`

| band (`MEMORY.md` section 4) | degree-7 `B'` | process peak (model) | 128 GB pod (119.2 GiB) | 256 GB pod (238.4 GiB) |
|:--|--:|--:|:--|:--|
| measured lower bound / low (`t = 10`-like) | 33.96·10⁹ | 144-148 GiB | no | **yes**, about 90 GiB spare |
| central (`t = 11`-like) | 35.9·10⁹ | 155-168 GiB | no | **yes**, about 70 GiB spare |
| high (`t = 12`-like) | 159.5·10⁹ | about 640 GiB | no | no (2.7× too small) |
| beyond (`t = 13`-like; or no refutation at degree 7) | 554·10⁹ to 7.08·10¹¹ | 2.0-2.6 TiB | no | no (9× to 11× too small) |
| degree 8 at all | 1.0·10¹² to 4.0·10¹² for its first step | 3.6-15 TiB | no | no |

So `cpu3m` or `cpu5m` at 32 vCPU fits the low and central bands, with `--max-dense 54000000000` (`MEMORY.md` section 5)
as the limit that turns any larger step into a clean `oversize` stop before memory runs out. No RunPod CPU pod fits the
high band. `[pcc:unverified]`

**Recommended: `cpu5m`, 32 vCPU, 256 GB, US$2.08/h.** It is the flavor the route already pins (decision D8), the size
the earlier 112Gi attempts used, and the CPU generation whose step times `MEMORY.md` section 6 is calibrated on. Time is
the binding constraint (section 3), so a slower generation is the larger risk. `cpu3m` is US$0.32/h less (US$9.60 over a
30 h worst case) on unrecorded CPUs; as a fallback it would be a second deviation (D8: no fallback) and could put the
two draws on different generations. If `cpu5m` at 32 vCPU has no stock, the route exits without creating anything
(`run.sh` exit 10) and the owner decides. `[pcc:unverified]`

### 2a. vCPUs and `RAYON_NUM_THREADS = 12`

- **Registered.** Amendment 9 fixes `RAYON_NUM_THREADS = 12` for every relaunch job; it came from the EKS node limit
  (16-vCPU nodes, 12 whole cores allocatable), not from the instrument. Its instrument note: the F4 chunk size depends on
  the thread count (`src/cryptanalysis/f4_fp_tower.rs:1694` at `f13a690df0`: `chunk = per_task * 2 *
  current_num_threads()`), so `muladds` compare only between rows with the same count; the reading (degree reached,
  `inconsistent`, `oversize`) does not depend on it. `[pcc:unverified]`
- **What 32 vCPUs change at 12 threads: memory only.** The 32 vCPUs are bought because RunPod sells memory at 8 GB per
  vCPU; with 12 rayon threads the other 20 do no F4 work (the runner and the copy loop use a little). `nproc` on the
  112Gi pods was 32 (`host.txt`). `[pcc:unverified]`
- **What 32 threads would change.** Both parallel phases scale with threads: the `B'` formation runs each 64-row block
  in parallel (`:1629-1665`, at most 64-way, with a serial fix-up per block at `:1670-1680`), and the S-row pass runs
  chunks of `per_task × 2 × threads` rows. A shorter wall time is plausible but unmeasured; the serial fix-up and the
  memory bandwidth bound it, and no 32-thread trace of this system exists. It would not change the reading, would make
  `muladds` incomparable with the 12-thread rows, and would be a deviation from Amendment 9. It is offered in
  `DRAFT_AMENDMENT.md` as an owner option, not as the default. `[pcc:unverified]`

## 3. The time side (derived from the code and the logs)

- **What `timed_out true` meant in the 112Gi rows.** The pilot sets the row's `timed_out` to `timed_out || oversize`
  (`examples/pkm_tower_pilot.rs:1504-1506`). Both 112Gi rows have `oversize true`, and their wall times (22,598 s and
  21,587 s) are 25 to 26 percent of the 86,400 s budget; the stop lines say `oversize`, not `budget`
  (`pkm_tower_pilot.rs:1186-1189`). So the budget did not bind there: `timed_out true` is the size stop copied into
  that field. The F4 deadline is wall clock, `Instant::now() + 86400 s` at the F4 start (`f4_fp_tower.rs:1013-1014`),
  checked between symbolic frontiers (`:2535-2539`), per pivot row and per S-row chunk in the elimination
  (`:1582-1592`), and in the step loop (`:2435`). `[pcc:unverified]`
- **Will `--budget 86400` bind at the larger `--max-dense`?** Steps 1-34 take about 21,000 to 22,000 s on the two CPU
  models seen. Step 35 is projected at 5,600 to 44,000 s more (symbolic 560-620 s measured; `B'` formation 4,900-11,100
  s; S-rows up to 27,300-32,200 s), so it ends 27,600 to 66,000 s after the F4 start: inside the budget. Each further
  all-pairs degree-7 step costs at least 5,500 to 12,000 s plus its S-rows.
  - Low band (falls after step 35 lead to `1` at lower degree, as at `t = 10`): projected finish about 30,000 to
    70,000 s, inside the budget.
  - Central band (a second all-pairs degree-7 step, as at `t = 11`): about 90,000 to 140,000 s at 12 threads; the
    budget binds and the row is censored (`timed_out true`, `oversize false`).
  - High band: `oversize` at 54·10⁹ well before the budget.

  Pods of one CPU model differed by 1.27× and 1.56× in wall time on identical runs (`MEMORY.md` section 6), which is as
  large as the margin of the low band. `[pcc:unverified]`
- **Projected wall time of the run (pilot only):** low band 8 to 20 h; central band 24 h (stopped by the budget); high
  band about 8 to 19 h (stopped by size at a later degree-7 step); plus up to 55 minutes from pod creation to start and
  the collection and git gate after it (`runpod_capped` route, at most 2 h). `[pcc:unverified]`

## 4. Cost per job (derived)

| flavor (32 vCPU) | per hour | route worst-case pod life 30 h (hard deadline 26 h + one 4 h reaper interval) | 26 h | run to the budget (about 25.5 h) | refutes at about 70,000 s (about 21 h of pod) |
|:--|--:|--:|--:|--:|--:|
| `cpu5m` | US$2.08 | **US$62.40** | US$54.08 | about US$53 | about US$44 |
| `cpu3m` | US$1.76 | US$52.80 | US$45.76 | about US$45 | about US$37 |

Both exceed the study's per-job cap of US$40 at the route's worst case: `cpu5m` by US$22.40. A reaper interval of 2 h
(28 h worst case) would lower `cpu5m` to US$58.24, still above the cap. The study arithmetic is in
`DRAFT_AMENDMENT.md` section 6. `[pcc:unverified]`
