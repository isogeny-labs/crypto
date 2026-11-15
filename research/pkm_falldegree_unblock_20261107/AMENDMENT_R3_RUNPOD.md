# DRAFT Amendment R3 (proposed number 15): reserved RunPod CPU pods as the execution route for the unrun jobs

**Status: draft, NOT frozen.** Written 2026-11-14 by the main session after the owner's instruction of the same day:
"unblock the fall-degree study and root solver. Run this on runpod". Target when frozen: appended to
`research/prime_fall_degree_20261027/PREREGISTRATION.md` as Amendment 15, before any job named below is started on the
new route. It changes **where and on what host** the registered jobs run. It does not change any hypothesis, §6 rule,
reading, cell, seed, draw, cap, budget, `--max-dense`, `max_entries`, binary or stage. Every number is `[pcc:unverified]`.

The unfrozen draft R2-G (stage B as resumable units) is **not used**: it was written only because Spot interrupted stage B.
A reserved host removes that reason, so stage B runs as its registered single job.

## 1. Why (cited, measured)

- The EKS `cpu-spot` pool is Spot-only and capped at 16 vCPU / 128 GiB (`CAPACITY_R2.md`).
- Stage B was lost twice to Spot interruptions (`STAGE_B_LOST.md`). Cell `pfd-x-t13-k0-c6` was lost three times
  (`SPOT_LOSSES.md`). The `t = 14` cap-6 runs were held back because no Spot run longer than about 3.9 h survived.
- The `m = 4` cells at `t = 5, c = 7` and `t = 6, c = 6` project to 80–510 GB and cannot be placed on the pool at all.
- RunPod pods created with `interruptible: false` are reserved: they run until they exit or are stopped (RunPod REST
  `PodCreateInput.interruptible`). That removes the failure mode behind every lost cell.

## 2. The route (plan)

- One reserved RunPod CPU pod per job, `computeType CPU`, `interruptible false`. The pod's vCPU count and memory are
  recorded from the API's pod record and from `host.txt` (`nproc`, `MemTotal`), as before.
- **Same binaries.** The four study binaries are the static musl builds of `f13a690df016d0a457bd40579fbb9d054f486adf`.
  Each file uploaded to the pod must have the SHA-256 recorded in `image/image.json` (`study_binary_sha256`) for that
  name: `pkm_tower_pilot f78f3959…`, `prime_fall_degree_tools 7e8a6703…`, `hky_fp_tower_check c36d324e…`,
  `pfd_grid_zeros 96283361…`, `entry_r2.sh 25b01841…`, `run.sh c35ea88e…`. The runner refuses to start on any mismatch
  (`runpod/server.py`, `/start`). The binaries are obtained by a rebuild from that commit with the recipe of
  `image/build_layer_r2.sh`. If the rebuild is not bit-identical, nothing runs on this route until the frozen image's
  layer is copied out of ECR instead.
- **Same entry script.** `entry_r2.sh` runs byte-identical. Its `aws s3 …` calls go to a shim that copies into the pod's
  `/results` (the `ISOLAB_RESULTS_URI` is `s3://local`). `/results` is fetched over HTTPS when the job ends. Its
  SHA-256s are recorded, and it is then uploaded to the lab's S3 under `runs/<job>/runpod/<pod id>/`, as the EKS route
  did.
- **The host differs.** The pod's base image is `python:3.12-slim` (Debian) instead of the aws-cli image. The binaries are
  static, so only `host.txt` and the timing change. The study reports degrees and multiply-add counts, not times
  (Amendment 9), so no reading depends on the host.
- **Threads.** `RAYON_NUM_THREADS` stays at 12 (Amendment 9), whatever the pod's vCPU count.

## 3. Jobs on this route, with their registered arguments (unchanged)

| job | entry stages | memory needed (registered request) |
|:--|:--|:--|
| `pfd-b-rp` | `g1 hky-B d-B g3` | 56Gi |
| `pfd-x-t13-k0-c6-rp` | `cap:13:0:6:12800000000` (Amendment 14) | 56Gi; `-m112` rule of Amendment 14 unchanged |
| `pfd-x-t14-k0-c6-rp`, `pfd-x-t14-k1-c6-rp` | `cap:14:K:6:12800000000` (Amendment 14) | 56Gi; `-m112` rule unchanged; cap 7 only as Amendment 7 B says |
| `pfd-m4-t4-k{0,1}-c7-rp`, `pfd-m4-t5-k{0,1}-c6-rp` | `m4:T:K:C:13958643712:4000000` (Amendment 8) | 112Gi |
| `pfd-m4n-*-rp` | as Amendment 13 | as Amendment 13 |

The pod's memory must be at least the registered request. A pod with more memory does not change any limit: the
`--max-dense` and `max_entries` values above are the registered ones, so a run censored by its limit here would be
censored on EKS too.

**Not included.** The `m = 4` cells at `t = 5, c = 7` and `t = 6, c = 6` need a larger `max_entries`. That is a change of
limit, not of route, so it gets its own amendment, written once the `t = 5, c = 6` row has measured its entry count
(`CAPACITY_R2.md` §1).

## 4. Spot rule and cost

- Owner decision 3 (one relaunch after a Spot loss) does not apply to a reserved pod. A pod that dies for any other
  reason (host failure, a stop by RunPod) is recorded as lost with its reason, and is relaunched at most once.
- Cost guard: owner decision 4 (US$40 per job, US$400 for the study) is unchanged. The pod's `costPerHr` is read when
  the pod is created, and a job whose `costPerHr × seconds limit` exceeds US$40 stops at the hour that reaches US$40.
  That stop is recorded as censored-by-cost, not as a row.

## 5. Records

Each job keeps the EKS route's records: a resume checkpoint before launch, a `bench.record/v1` (`benchrec.py`), and a
director card and receipt. The pod id, `costPerHr`, the runner's `runner_request.json` (with the verified SHA-256s)
and the results tarball's SHA-256 go into that job's receipt.
