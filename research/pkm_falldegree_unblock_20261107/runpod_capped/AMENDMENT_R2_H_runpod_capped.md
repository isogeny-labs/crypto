# DRAFT Amendment R2-H (proposed number 18): the capped runs at t = 14 and the further t = 13 draw 0 attempt on reserved RunPod CPU pods

**Status: draft, NOT frozen.** Written 2026-11-14 for the owner's review. When frozen it is appended to
`research/prime_fall_degree_20261027/PREREGISTRATION.md` as Amendment 18, before any job named below is started. Every
number is `[pcc:unverified]`.

- **Numbering.** Amendment 15 is R2-G (stage B unit jobs) and Amendment 16 is stage B on one reserved RunPod pod (both
  in the preregistration). Amendment 17 is taken by another session's PR #4173 (rebuilt binaries admitted by exact
  reproduction, for stage B). So this is 18, appended after 17 once 17 is on main. Unlike Amendment 17's stage B pod,
  this route runs the binaries extracted from the frozen image itself, checked against `image/binaries.sha256`; it does
  not use the rebuild.
- **Scope.** It changes the host of three registered jobs, and how their results come back. It changes no hypothesis,
  §6 rule, reading, cell, seed, draw, cap, `--budget`, `--max-dense`, binary or command line.
- **Authority.** Owner decision 12 (2026-11-14, "Use runpod"): command jobs run on RunPod on-demand Secure Cloud pods
  (reserved, not interruptible), under dated amendments frozen before each run. Decision 10: t = 14, and the further
  t = 13 draw 0 attempt, run on non-Spot capacity. Decision 4: at most US$40 per job and US$400 for the study.
- **Reuse.** The route reuses Amendment 16's runner and client unchanged. It adds only a launcher and a collector
  (section 3) for what they do not provide.

## 1. Jobs

Each job runs one `cap:` stage. The stage equals `command[1]` of its frozen request, and `run.sh` refuses any other.

| job / pod name | request (frozen, not edited) | stage |
|:--|:--|:--|
| `pkm-x-t14-k0-c6` | `requests_r2i/pfd-x-t14-k0-c6.json` | `cap:14:0:6:12800000000` |
| `pkm-x-t14-k1-c6` | `requests_r2i/pfd-x-t14-k1-c6.json` | `cap:14:1:6:12800000000` |
| `pkm-x-t13-k0-c6-r3` (the further attempt; decision 10) | `requests_r2i/pfd-x-t13-k0-c6.json` | `cap:13:0:6:12800000000` |

- Every stage is Kummer m = 2, P1, cap 6, `--budget 86400`, `--max-dense 12800000000` (Amendment 14), and
  `RAYON_NUM_THREADS = 12` (Amendment 9).
- Order: serial, `pkm-x-t14-k0-c6` first (NONSPOT_T14.md). Cap 7 at t = 14 follows Amendment 7 B and needs its own request.

## 2. What changes

1. **Host.** One reserved RunPod CPU pod per job.
   - Request: `computeType CPU`, `cloudType SECURE`, `interruptible false`, `vcpuCount 16`, flavors
     `cpu5m, cpu3m, cpu5g, cpu3g` in that order (Amendment 16's list and priority), image `python:3.12-slim`, 20 GB disk.
   - Why 16 vCPU and not 12: at 12 vCPU the `g` flavors give 48 GB, below the registered 56Gi. At 16 vCPU every listed
     flavor gives at least 64 GB (`ramMultiplier` 4 or 8, read from the API).
   - The thread count stays 12. This matches an EKS 16-vCPU node running a 12-CPU pod.
   - `run.sh` refuses a pod whose API record has fewer than 12 vCPU, less than 61 GB (56 GiB = 60.1 GB), or
     `interruptible true`.
   - Smoke pod: `cpu5m`, 16 vCPU, 128 GB, US$1.04/h, AMD Ryzen Threadripper 7960X, EU-RO-1.
2. **Entry script file.** The jobs invoke `/usr/local/bin/entry_r2.sh STAGE`, where the frozen requests name
   `/usr/local/bin/entry.sh`. The reason is that Amendment 16's runner (`runpod/server.py`) runs only `entry_r2.sh`,
   and checks it by SHA-256 (`25b01841…`).
   - For a `cap:` stage the commands, file names and `progress.txt` lines are the same. `entry_r2.sh` differs from
     `entry.sh` only in its header comment and the added `m4n:` branch.
   - Its preamble (from `set -u` to the stage loop), its `cap:*)` branch and its tail (from `done`) are byte-identical
     to `entry.sh`'s (checked with `cmp` on the three extracted regions).
   - Same pilot command line, same `pfd_grid_zeros` step after it (run whatever the pilot's exit code), same
     `runs/X-m2-tT-kK-cC.{jsonl,out,log,gridzeros.jsonl,gridzeros.log}`, same `host.txt`, `binary.sha256` and
     `progress.txt`.
3. **Results path.**
   - `entry_r2.sh`'s `aws s3` calls go to the runner's shim, which copies into the pod's `/results` (Amendment 16).
     The results stay on the pod's disk until fetched.
   - `fetch.sh` downloads `/results.tar.gz` twice, extracts both, and requires the two per-file SHA-256 manifests to be
     byte-identical. Only then does it keep one tree in `results/rp-<job>/`, write its manifest and delete the pod.
   - From this session the files are then added to the job's `bench.record/v1` (`tools/benchrec.py add-raw`) and
     uploaded to S3 by content (`tools/benchrec.py upload`), as the EKS verify cards did.
4. **Time limit.**
   - The pilot's own `--budget 86400` is unchanged.
   - The registered job limit of 90,000 s becomes a pod deadline: `run.sh` records start + 90,000 s. At or after that
     time `fetch.sh` collects what exists and deletes the pod.
   - A run collected at the deadline is censored, as an EKS job past `seconds` would be (and on EKS that job would have
     lost its pod).
   - The deadline is enforced from the session, not by the pod (the runner has none). So check-ins are required: one
     about 4 h after start, then at most every 4 h, and one at the deadline.
   - Whatever happens, the pod is deleted within 26 h of creation. If the session cannot do it, the owner deletes it.
5. **Host lines.** `host.txt` is the frozen script's, so it has no RunPod pod id.
   - The pod id, flavor, `costPerHr`, data center and machine id are recorded in the job's receipts (section 6).
   - The results tree is fetched from that pod's own proxy URL with that pod's token.
   - On RunPod, `nproc` reads the pod's affinity (16), while `MemTotal` reads the **host's** memory (263,400,384 kB on
     the smoke pod). The memory available to the run is therefore the API record's `memoryInGb`, recorded with the
     row. This is a host difference like any node change.

## 3. Route files

The new files are `research/pkm_falldegree_unblock_20261107/runpod_capped/`: `common.sh`, `run.sh` and `fetch.sh`,
shell only. `SMOKE.md` records the development check.

- **`run.sh NAME STAGE`**, in order:
  1. Checks the name and stage against the request.
  2. Checks the 4 binaries, `entry_r2.sh` and `run.sh` against `image/binaries.sha256`, and `build-info.json`
     against `image/image.json`. On any mismatch it stops.
  3. Refuses a second live pod with the same name.
  4. Reads the price (section 5).
  5. Creates the pod with Amendment 16's body. The changes are the name, `vcpuCount 16` and an explicit
     `cloudType SECURE`.
  6. Checks the pod record and waits for the runner.
  7. Uploads with Amendment 16's `rp.sh upload` (2 MB chunks), and compares each file's pod-side SHA-256 with the
     expected one.
  8. Starts with `rp.sh start`. The runner re-checks every SHA-256 against `image/image.json` before it runs anything.
     It sends `rayon 12`.
  - Any failure after the pod is created deletes the pod.
- **`fetch.sh NAME [--abort]`:**
  - Before the runner is done and before the deadline, it records one status and exits 41 without touching the pod.
  - Otherwise it does the two-fetch check, keeps the tree, writes the receipts, deletes the pod and lists the
    account's pods. It fails (exit 7) if the pod is still listed.
- **Reused unchanged:**
  - `runpod/server.py` (SHA-256 `f56fd08e01318512713f8ee392d8539fb94dc969444d45234179789930d86550`).
  - `runpod/rp.sh` (SHA-256 `b4b6019934e02ffa32043e2e6b9aee4eb4dcc53b42442b2e01d1b30ccf536c5d`; only its `upload`,
    `start`, `status` and `fetch`, which talk to the pod with the pod's token).
  - Amendment 16's flavor list, base image, reserved flag and SHA-256 gate.
- **Not reused, and why:**
  - `rp.sh create`, `pod` and `delete` send the proxy-injected placeholder key. In this session the RunPod key is the
    environment variable `RUNPOD_API_KEY`, with no proxy injection: the placeholder got HTTP 401.
  - `rp.sh create` also takes the vCPU count with no memory check, which at 12 vCPU could fall back to a 48 GB flavor.
  - `common.sh` makes those API calls itself. The key goes in a mode-0600 header file, never in argv or receipts.
- **Not built:** a separate pod-side job script (`podjob.sh`). The reused runner already runs the frozen script
  unchanged, which writes the same commands and files. A second, re-typed copy of the `cap:` branch could drift from
  the frozen one.

## 4. Lost runs: the analogue of the Spot-loss rule

1. **When a run is lost.** Any of these:
   - the pod disappears from the API;
   - RunPod stops or restarts it (the runner reports `idle` or loses its state after a successful start);
   - the runner is unreachable at two consecutive check-ins at least 1 h apart;
   - RunPod has an outage that ends the pod.
   Nothing from a lost run is admitted or analysed, even if a partial tree is fetched. The loss is recorded with its
   reason in a `RUNPOD_LOSSES.md` table, as in `SPOT_LOSSES.md`.
2. **Relaunches.**
   - `pkm-x-t14-k0-c6` and `pkm-x-t14-k1-c6`: one relaunch per cell under a new name (`-r1`), per decision 3. Both
     attempts are kept.
   - `pkm-x-t13-k0-c6-r3`: this run is the further attempt that decision 10 allows. It gets no relaunch; if it is
     lost, the cell goes back to the owner.
3. **A pilot killed by signal** (`exit 137` in `progress.txt`) that the session did not cause (no `--abort`, no
   deadline). RunPod gives no container reason (`OOMKilled`) to classify it. It is recorded as "killed, cause not
   recorded" and is not the cell's row. It counts as a lost run under rule 2, never as censored-by-limit.
4. **Amendment 14's limit rule is unchanged.** The trigger is `oversize true` in the row. The pod's larger memory does
   not change `--max-dense`, so a run that stops on its limit here would stop on it on EKS too.
   - The `-m112` re-run needs a pod with at least 121 GB (`cpu5m` or `cpu3m` at 16 vCPU: 128 GB) and a request with
     `--max-dense 27900000000`.
   - **The route as built does not cover it.** `run.sh` takes the stage from `requests_r2i`, which has no `-m112`
     request, and its memory floor is 61 GB. A `-m112` run needs a generated request and a floor of 121 GB, written
     before it is launched.
5. **Budget stops.** A run stopped by `--budget` (`timed_out true`) is censored (Amendment 7 B) and is not re-run.

## 5. Cost guard

- **Price** (read 2026-11-14 23:24:47Z, RunPod GraphQL `cpuFlavors.specifics.securePrice`, 16 vCPU, Secure Cloud):

  | flavor | price per hour | worst case per run (26 h) |
  |:--|:--|:--|
  | `cpu5m` 16 vCPU / 128 GB | US$1.04 | US$27.04 |
  | `cpu3m` 16 vCPU / 128 GB | US$0.88 | US$22.88 |
  | `cpu5g` 16 vCPU / 64 GB | US$0.736 | US$19.14 |
  | `cpu3g` 16 vCPU / 64 GB | US$0.64 | US$16.64 |

  The smoke pod's record showed `costPerHr` 1.04.
- **Per run.** Worst case is price × (25 h job limit + 1 h) ≤ US$40.
  - `run.sh` refuses to create a pod if any listed flavor's price fails that test.
  - After creation, it deletes the pod if the record's `costPerHr` × 26 > 40.
  - Highest worst case today: US$27.04.
- **Study.**
  - Three runs: at most US$81.12. With one relaunch for each t = 14 cell: at most US$135.20.
  - Before each launch, the sum of the recorded `cost.json` values and the open worst cases is checked against
    US$400.
- **Every reading is recorded.** `price.jsonl`, `pod-record.json` and `cost.json` (price × pod lifetime, derived).

## 6. Admission and receipts

A run is admitted only if all of these hold. The first four are the EKS verify-card pairs.

1. `binary.sha256` has the four lines of `image/binaries.sha256` at `/usr/local/bin/…`.
2. `host.txt` has `argv: <stage>` and `rayon: 12`.
3. `progress.txt` has `end X-m2-tT-kK-c6 exit 0` and `end X-m2-tT-kK-c6.gridzeros exit 0`.
4. The runner completed: `runner_state.json` has `state done` and `exit 0`. This is the analogue of "Job succeeded,
   one pod, exit 0".
5. `runner_request.json`'s `verified_sha256` equals `image/image.json` for every uploaded name.
6. The two fetches' manifests are identical, and `local-sha256.txt` equals the kept tree.
7. `pod-record.json` shows at least 12 vCPU, at least 61 GB, `costPerHr` × 26 ≤ 40 and a Secure Cloud machine.
   `pod-request.json` shows `interruptible false` and `cloudType SECURE`.
8. The pod was deleted and is no longer listed (`pods-after-delete.jsonl`).

The CPU model, `nproc`, `MemTotal`, data center and flavor are recorded with the row, like any node change on EKS.

- **The bracket readings do not depend on the host.** They use degrees and `oversize`/`timed_out`.
- **Disclosure, as in Amendment 16:** `--budget` is wall-clock. A host faster or slower than the EKS nodes could
  censor a run that would have finished there, or the reverse. A censored row is reported with its host, never
  dropped.
- **`muladds`** are compared only between rows with the same thread count (Amendment 9). The thread count here is 12.

**Receipts of each run.**

- **Launch and collection**, in `runpod_capped/receipts/<job>/`:
  - `run.log`, `price.jsonl`, `pod-request.json`, `pod-create.json`, `pod-record.json`;
  - `status-ready.json`, `upload.jsonl` (pod-side SHA-256 of each upload), `start.json`, `pod-state.json`
    (pod id, created, deadline);
  - `status-<stamp>.json` for every check-in, `fetch.log`;
  - `tarball.sha256`, `local-sha256.txt`, `admission-lines.txt`;
  - `pod-record-final.json`, `delete.http`, `delete.response`, `pods-after-delete.jsonl`, `cost.json`.
- **The results tree** `results/rp-<job>/`, recorded in the job's `bench.record/v1` with S3 locations by SHA-256:
  - `host.txt`, `binary.sha256`, `progress.txt`;
  - `runs/X-m2-tT-kK-c6.{jsonl,out,log,gridzeros.jsonl,gridzeros.log}`;
  - `runner_request.json`, `runner_state.json`, `entry.stdout`, `entry.stderr`.
- **Records kept as on EKS:** the resume checkpoint before launch, the director's launch and verify cards, and their
  receipts.

## 7. Unchanged

- Binaries (static musl builds of `f13a690df016d0a457bd40579fbb9d054f486adf`, by SHA-256), commands, stages, cells,
  draws, caps.
- `--budget 86400`, `--max-dense 12800000000`, 12 threads.
- Hypotheses and §6 rules, Amendments 7 B, 9 and 14.
- The US$40 per job and US$400 study ceilings.
- The requests in `requests_r2i/`, which are not edited. They remain the record of the cell and stage; their `image`,
  `cpu`, `memory` and `seconds` are read as above.

## 8. Points for the reviewer before freezing

1. `entry_r2.sh` in place of `entry.sh` (section 2.2): the same commands, but a different file name from the request.
2. 16 vCPU with 12 threads (section 2.1).
3. The deadline is enforced by session check-ins, not by the pod (section 2.4).
4. An unexplained `exit 137` is counted as a lost run, not as `OOMKilled` (section 4.3).
5. The `-m112` re-run is not covered yet (section 4.4).
6. Development check (`SMOKE.md`): one pod, `cap:8:0:5`, everything passed, US$0.014. The one `run.sh` change made
   after it (writing `pod-request.json`) was not exercised on a pod.
