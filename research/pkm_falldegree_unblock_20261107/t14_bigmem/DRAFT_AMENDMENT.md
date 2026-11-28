# DRAFT (proposed Amendment 34): `t = 14` cap 7 at `--max-dense 54000000000` on one 256 GB reserved RunPod CPU pod

**Status: design draft, 2026-11-21. Not frozen, not numbered, not reviewed, not launched.** "Proposed Amendment 34"
assumes the route draft `runpod_capped/AMENDMENT_R2_H_runpod_capped.md` keeps the number 33; if that draft is never
frozen (it was not launched, because its cell was already measured), this text takes the next free number. Nothing here
changes any file outside `t14_bigmem/`. **Every number is `[pcc:unverified]`.** Evidence: `MEMORY.md` (sizes, model,
projection), `HOSTS.md` (catalog, threads, time, cost), `project.py` (re-derivation).

## 0. Owner instruction and state

- **Owner instruction (2026-11-21):** design a bigger-memory attempt for the `t = 14` cap-7 cell. `[pcc:unverified]`
- **State.** At `t = 14` cap 5 and cap 6 complete without refuting on both draws (`d_F ≥ 7`). Cap 7 at
  `--max-dense 12800000000` (56Gi) stopped `oversize` at degree 6 (Amendment 30); cap 7 at `27900000000` (Amendment 14's
  `-m112` step, Amendment 31 route F, 32 vCPU / 256 GB pods) stopped `oversize` at degree 7, step 35, on both draws,
  asking for `B' = 33,961,233,968` entries, not refuted (`results/SNAPSHOT_20261117/README.md`;
  `MEMORY.md` section 2a). Under Amendments 7 B and 14 both are censored and `d_F ≥ 7` stands. `[pcc:unverified]`

## 1. Jobs

| name | when | stage | host |
|:--|:--|:--|:--|
| `pkm-x-t14-k0-c7-d54` | first | `cap:14:0:7:54000000000` | `cpu5m`, 32 vCPU, 256 GB, cloud `SECURE` (reserved) |
| `pkm-x-t14-k0-c7-d54-r1` | only after the base is LOST with relaunch `auto` (owner decision 13 as the route applies it) | same | same |
| `pkm-x-t14-k1-c7-d54` | **not registered here**; only by a later owner decision, after the draw-0 outcome | `cap:14:1:7:54000000000` | same |

- Kummer `m = 2`, `P1`, `t = 14`, cap 7, seed `1347112276` (recorded, not passed), `--budget 86400` (Amendment 7 B),
  `RAYON_NUM_THREADS = 12` (Amendment 9), binaries and `entry_r2.sh` of the frozen layer pinned in
  `runpod_capped/common.sh`, unchanged. `[pcc:unverified]`
- **`--max-dense 54000000000`**: the largest multiple of 10⁹ with `4·C ≤ L − 36 GiB`, `L = 256·10⁹` bytes (238.4 GiB,
  the pod's cgroup limit). It is 1.59× the step-35 requirement; the projected process peak under it is at most about
  231 GiB (`MEMORY.md` section 5). `[pcc:unverified]`
- **Memory floor:** the pod record's `cpu.memory` at least 256 (GB), and the cgroup `memory.max` read before start
  either `max` or at least 256,000,000,000 bytes; otherwise the pod is deleted before start (NOT_AN_ATTEMPT, owner).
  `[pcc:unverified]`
- **Job limit:** unchanged. Run deadline = runner start + 90,000 s; hard deadline = pod creation + 26 h; runner start
  within 55 minutes of creation; reaper at intervals of at most 4 h. `[pcc:unverified]`
- **Draw 0 first, draw 1 by a later decision.** The trace does not depend on the draw (both draws agree count for count
  through step 35, `MEMORY.md` section 2), so draw 1 is expected to repeat draw 0; but Amendment 7 D.5 needs both draws
  for "holds at `t = 14`": a draw-0 `d_F = 7` with draw 1 at `≥ 7` reads "undecided at `t = 14`", as `t = 13` did.
  `[pcc:unverified]`

## 2. Reading (fixed before any row of these jobs)

The reading uses the row's `inconsistent` and `oversize` fields and the stop line of `runs/X-m2-t14-k0-c7.log`, not the
row's `timed_out` alone, because the pilot writes `timed_out = timed_out || oversize`
(`examples/pkm_tower_pilot.rs:1504-1506` at `f13a690df0`); a budget stop is the stop line with `budget`, a size stop
the one with `oversize: --max-nnz` (`:1186-1189`). `[pcc:unverified]`

| outcome (classified by the route's single classifier, then the row) | reading for draw 0 |
|:--|:--|
| ADMITTED, `inconsistent true` (refutes at cap 7) | `d_F = 7` |
| ADMITTED, degree 7 completed, `inconsistent false`, `oversize false`, `timed_out false` | `d_F ≥ 8` (projected out of reach: `MEMORY.md` section 4) |
| ADMITTED, `oversize true` at 54·10⁹, or class OOM (cgroup `oom_kill > 0`) | censored at the new size; `d_F ≥ 7` stands |
| ADMITTED, `oversize false`, stop line `budget` (`timed_out true`); or class LIMIT | censored; `d_F ≥ 7` stands |
| LOST, ABORTED, NO_EVIDENCE, NOT_ADMITTED, NOT_AN_ATTEMPT | no row (route section 4) |

No further bracket step follows from this amendment. Cap 8 is not registered: its first degree-8 step alone is projected
at 3.6 to 15 TiB (`MEMORY.md` section 4). `[pcc:unverified]`

## 3. What this changes relative to Amendments 7 B, 14 and 31 (a deviation, stated plainly)

- **Amendment 7 A/B.** 7 A fixed the memory request (112Gi at `t = 13, 14`) and the formula `4·C ≤ request − 8 GiB`.
  Here the request is the 256 GB pod itself and the headroom is 36 GiB, because the basis alone is 11.4 GiB before step
  35 (`MEMORY.md` section 3; with 8 GiB a step just under the `-m112` cap would have exceeded 112 GiB). 7 B's bracket
  rule is unchanged: cap 7 is still the next step, and a capped run stopped by a limit is censored. `[pcc:unverified]`
- **Amendment 14.** 14 made the `-m112` run, at `--max-dense 27900000000`, the cell's row, and an `-m112` run stopped by
  its limit censored with no further size. **Running the same cell at a larger `--max-dense` is a deviation from that
  registered last step.** The `-m112` rows stay recorded as they are (censored, `d_F ≥ 7`); this run is an additional,
  separately named attempt, and its row is used only under section 2. `[pcc:unverified]`
- **Amendment 31, route F.** Same host size (32 vCPU / 256 GB), same stage shape, same reading table; changes: the
  `--max-dense` value (27.9·10⁹ to 54·10⁹), the per-job cap (route F had an owner-approved US$60; this needs US$62.40,
  section 6), and the route (`runpod_capped/`: REST v2, reaper, hard deadline, git gate, pod deleted when the job ends).
  `[pcc:unverified]`
- **Owner decision 15** (a 112 GiB attempt with a peak above 112 GiB counts as OOM) existed to mirror an EKS 112Gi
  request on a larger pod. There is no EKS analogue here: the limit is the pod's own 256·10⁹ bytes, a peak above it
  cannot happen without a kill, and OOM means cgroup `oom_kill > 0`. `[pcc:unverified]`

**Why it is still the same measurement.** Same binaries (built from `f13a690df0`), same system (`t`, draw, seed), same
cap, same budget, same thread count, same entry script. `--max-dense` only stops a run: it is read once per step, as a
check before `B'` is allocated (`src/cryptanalysis/f4_fp_tower.rs:1577-1580`, the line Amendment 14 cites), and steers
nothing else. The measured traces show it: steps 1-30 are identical at 12.8·10⁹ and 27.9·10⁹, and steps 1-34 identical
on two CPU models and both draws. A run that ends without `oversize` at 54·10⁹ therefore has the row any larger limit
would give. **Disclosure:** the 54·10⁹ value was chosen after seeing that step 35 needs 33.96·10⁹; the choice of limit
depends on the data, but the limit cannot change what the run computes, only whether it may finish, and the reading
above is fixed before any row. `[pcc:unverified]`

## 4. Reuse of the reviewed route (`runpod_capped/`) and what must change

Reused unchanged in substance: REST v2 calls and field checks (`V2_FIELDS.md`), serial rule, catalog price and stock
check before create, the boot copy loop for the cgroup files, `server_pinned.py`, `rp_pinned.sh`, the run deadline and
hard deadline, `fetch.sh`'s collection (two downloads, manifest comparison), the single classifier, the git gate before
deletion, `reap.sh`, the ledger, the receipts list, the token handling. `[pcc:unverified]`

| file | what changes |
|:--|:--|
| `common.sh` | `FLAVOR=cpu5m VCPU=32 RAM_GB=256`; `MEM_FLOOR_GB=256`; `M112_LIMIT_BYTES` becomes a pod-limit constant `256000000000`; `STUDY_STAGE=cap:14:0:7:54000000000`; `STUDY_NAMES=(pkm-x-t14-k0-c7-d54 pkm-x-t14-k0-c7-d54-r1)`; `CEIL_USD` and `STUDY_GUARD_USD` to the owner's new values (section 6); `AMENDMENT_HEADING` to the frozen number; `CLASSIFIER_VERSION=a34-r1`; the request derivation (`c7_m112_request`) replaced by one that writes `command[1] = cap:14:0:7:54000000000`, a memory label for the 256 GB pod, and the job-name suffix `-c7-d54`, from the same frozen `requests_r2i/pfd-x-t14-k0-c6.json`, with its new SHA-256 pinned. |
| `run.sh` | the study-stage refusal (step 1) to the new stage; the D7 checks on the create response and GET record to `cpu.vcpuCount 32` and `cpu.memory ≥ 256`; the pre-start cgroup check to `max` or `≥ 256000000000`; the per-job guard `price × 30 h ≤ CEIL_USD`; development stages may add `54000000000` for a `t ≤ 10` dev check. |
| `fetch.sh` | admission rule 2 (`argv: cap:14:0:7:54000000000`), rule 7 (`cpu {cpu5m, 32}`, `cpu.memory ≥ 256`, `cost` = catalog price); rule P2p rewritten as in section 3 (no 112 GiB threshold); classifier version and its own hash pinned in the frozen text. |
| `make_requests_m112.sh`, `requests_m112/` | a new `make_requests_d54.sh` and `requests_d54/` (the old ones stay as records of the Amendment 33 draft). |
| `reap.sh` | unchanged (`pkm-x-` prefix, hard deadline). |
| `tests/` | constants and fixtures updated; the offline suite rerun and its count recorded (177/177 at the Amendment 33 revision). |
| frozen text | the SHA-256 of `common.sh`, `run.sh`, `fetch.sh`, `reap.sh` at freeze, as section 10 of the route draft does. |

Before the study run: one development smoke at `t ≤ 8` on a 32-vCPU `cpu5m` pod to read, with REST v2, the create
response, the GET record (`cpu.memory`), the cgroup `memory.max` of a 256 GB pod, `DELETE` 204 and GET 404 afterwards
(the route draft's section 9.5 lists the same open checks). Cost of a smoke: under US$0.50 at US$2.08/h for under 15
minutes (derived). `[pcc:unverified]`

## 5. Threads: the registered default and one owner option

- **Default (registered here): `RAYON_NUM_THREADS = 12`**, as Amendment 9. Projected wall time: the low band finishes
  inside the budget; the central band is stopped by the budget (`HOSTS.md` section 3). `[pcc:unverified]`
- **Option T32 (only if the owner chooses it before the freeze):** `RAYON_NUM_THREADS = 32` on the same pod, a second
  deviation (from Amendment 9). The reading does not depend on the thread count (Amendment 9's instrument note); `muladds`
  would not be comparable with any earlier row; the size of the wall-time gain is unmeasured. It does not change the
  memory projection or the cost cap. `[pcc:unverified]`
- **Not proposed:** a larger `--budget`. A 48 h budget would double the worst-case pod life and cost (about US$110 per
  job) and change Amendment 7 B's budget. `[pcc:unverified]`

## 6. Cost: per job and study (derived)

- **Per job.** `cpu5m` 32 vCPU at US$2.08/h (catalog 2026-11-21) over the route's worst-case pod life of 30 h:
  **US$62.40**. The study cap is US$40 (owner decisions 4 and 21): **the design needs a per-job cap of at least US$62.40,
  US$22.40 more; proposed US$65.** Expected: about US$44 if it refutes near 70,000 s; about US$53 if the budget stops
  it. `[pcc:unverified]`
- **Study.** Ledger `runpod_capped/ledger.tsv`: 53 rows, sum US$465.8424 (`runpod_capped/ledger_SOURCES.md` section 5;
  recomputed here from the `usd` column). Owner decision 21: guard US$510, ceiling US$540. `[pcc:unverified]`

| case | ledger + worst case | against guard US$510 | against ceiling US$540 |
|:--|--:|:--|:--|
| draw 0 | 465.84 + 62.40 = **528.24** | over by 18.24 | under by 11.76 |
| draw 0 and later draw 1 | 465.84 + 124.80 = **590.64** | over by 80.64 | over by 50.64 |
| draw 0, if the two snapshot pods of UNKNOWN-5 (US$72.96) are the study's | 538.80 + 62.40 = **601.20** | over | over by 61.20 |

  So draw 0 needs the guard raised to at least US$528.24 (proposed US$530) with the ceiling unchanged; draw 1 later
  needs the ceiling raised to at least US$590.64 (proposed US$600); the UNKNOWN-5 attribution changes both. The
  `cpu3m` alternative (US$52.80 worst case, 518.64 for draw 0) also exceeds the guard and the per-job cap.
  `[pcc:unverified]`

## 7. Owner decisions this draft needs before a freeze

1. Whether to run the deviation at all (section 3), knowing that it can return `d_F = 7` or a censored row, and not
   `d_F ≥ 8` in practice (`MEMORY.md` section 4). `[pcc:unverified]`
2. Per-job cap US$65 (or at least US$62.40) for this job. `[pcc:unverified]`
3. Study guard at least US$528.24 (proposed US$530); ceiling at least US$590.64 before any draw-1 job. `[pcc:unverified]`
4. Whether UNKNOWN-5 (two snapshot pods, US$72.96) are study spend. `[pcc:unverified]`
5. Threads: 12 (default) or option T32. `[pcc:unverified]`
6. No fallback flavor (D8): if `cpu5m` at 32 vCPU has no stock (the 2026-11-21 catalog listed no data center at that
   size), wait, or decide on `cpu3m` as a further deviation. `[pcc:unverified]`

## 8. Recommendation

Feasible on RunPod CPU pods only in a narrow sense `[pcc:unverified]`: the largest pod, `cpu5m` at 32 vCPU and 256 GB
(US$2.08/h), holds the measured step-35 requirement (33.96·10⁹ `B'` entries, about 144-148 GiB of process memory) and
the central band (about 155-168 GiB) under `--max-dense 54000000000`, and no RunPod CPU pod holds the high band (about
640 GiB, `t = 12`-like growth) or the `d_F ≥ 8` outcome (2.0 TiB or more). Worst case US$62.40 for draw 0 and US$124.80
for both draws, against a US$40 per-job cap and a study at US$465.84 of a US$510 guard and US$540 ceiling, so it needs
owner decisions on both caps. It returns `d_F = 7` only if degree 7 at `t = 14` behaves as degree 6 did at `t = 10`
(refutation through the falls that step 35 yields, finishing inside the 86,400 s wall budget); a `t = 11`-like second
all-pairs step most likely runs into the wall budget at 12 threads, and `t = 12`-like growth ends in `oversize` at the new
size. The main risks are: the projection rests on four degree-6 analogues, one without a trace; the wall budget, with host
variation of up to 1.56× between pods of one CPU model and no measured gain from more threads; and the outcome is
one-sided, since a censored result cannot tell `d_F = 7` with larger growth from `d_F ≥ 8`. The pilot's own limits
(`u32` columns, the 64-bit tower mask, the product cache) do not bind at degree 7 or 8; memory and wall time do.
I would run draw 0 only if the owner accepts paying about US$45 to US$62 for a result that is most informative when
C-step is right, and would register draw 1 only after a draw-0 refutation. `[pcc:unverified]`
