# Smoke test of the capped-run RunPod route (2026-11-14): development check, not data

One pod, one tiny stage that is not a study cell, end to end: hash check, create, upload, start, fetch, transfer
check, delete, list. Every number here is `[pcc:unverified]` (measured by this session from the RunPod API, the pod's
runner and the fetched files; no verifier has checked it). The fetched row is a development check and enters no rule,
median or reading.

## What ran

| item | value |
|:--|:--|
| stage | `cap:8:0:5:12800000000` (Kummer m = 2, P1, t = 8, draw 0, cap 5): **development check, not data**; t = 8 is not an Amendment 7 B cell |
| pod name | `pkm-x-dev-smoke1` |
| pod id | `gwfmrulvmua8ye` |
| flavor and size (create response, `pod-create.json`; the later GET record `pod-record.json` has `machine: {}`) | `cpu5m` (Memory-Optimized), 16 vCPU, `memoryInGb` 128, `costPerHr` 1.04, `machine.secureCloud` true, data center `EU-RO-1` |
| reserved | request body had `interruptible: false`; the pod record returns `interruptible: null` (the API does not echo the field) |
| image | `python:3.12-slim`, running Amendment 16's `runpod/server.py` (SHA-256 `f56fd08e01318512713f8ee392d8539fb94dc969444d45234179789930d86550`) unchanged |
| client | Amendment 16's `runpod/rp.sh` (SHA-256 `b4b6019934e02ffa32043e2e6b9aee4eb4dcc53b42442b2e01d1b30ccf536c5d`) unchanged, for upload, start, status and fetch |
| entry script | `entry_r2.sh` (`25b01841…`), whose preamble, `cap:` branch and tail are byte-identical to `entry.sh`'s (checked with `cmp` on the three extracted regions) |

Commands (from `research/pkm_falldegree_unblock_20261107/runpod_capped/`):

```
export RPC_STATE=<private scratch dir> RPC_BIN=<scratchpad>/r2g_layer/usr/local/bin \
       RPC_RECEIPTS=$PWD/smoke/receipts RPC_RESULTS=<scratch>/rpc_smoke_results
./run.sh pkm-x-dev-smoke1 cap:8:0:5:12800000000
./fetch.sh pkm-x-dev-smoke1
```

Before the smoke, the refusal paths were exercised without creating any pod: a study name with a stage that differs
from its request (`cap:14:0:6:27900000000` for `pkm-x-t14-k0-c6`), a development stage with t = 12, a name outside
`pkm-x-*`, a non-`cap:` stage, and a `pfd_grid_zeros` with one byte appended. Each was refused with exit 3 before any
API call that creates anything.

## Timeline (UTC, from `smoke/receipts/run.log` and `fetch.log`)

| step | time | elapsed |
|:--|:--|:--|
| local SHA-256 of the 4 binaries, `entry_r2.sh`, `run.sh` against `image/binaries.sha256` | 23:24:43 | 0 s |
| price read (GraphQL `securePrice`, 16 vCPU): cpu5m 1.04, cpu3m 0.88, cpu5g 0.736, cpu3g 0.64 US$/h | 23:24:47–49 | 6 s |
| pod created | 23:24:51 | 8 s |
| runner answering (`state idle`) | 23:25:15 | 32 s |
| 7 files uploaded in 2 MB chunks; each pod-side SHA-256 equal to the expected one | 23:25:28 | 45 s |
| runner started (it re-checked every SHA-256 against `image/image.json`), job ran | 23:25:28–29 | 1 s of job |
| two fetches of `results.tar.gz`, per-file manifests identical (12 files) | 23:25:36–38 | |
| `DELETE /pods/gwfmrulvmua8ye` → HTTP 204 | 23:25:38 | pod lifetime 47 s |
| pod no longer listed; `pkm-x-*` pods listed after delete: 0 | 23:25:44 | |

## What came back (fetched tree, `smoke/receipts/local-sha256.txt`)

- `binary.sha256`: the four lines `f78f3959… /usr/local/bin/pkm_tower_pilot`, `7e8a6703… prime_fall_degree_tools`,
  `c36d324e… hky_fp_tower_check`, `96283361… pfd_grid_zeros`, equal to `image/binaries.sha256`.
- `host.txt`: `argv: cap:8:0:5:12800000000`, `nproc: 16`, `rayon: 12`, `seed: 1347112276`,
  `model name : AMD Ryzen Threadripper 7960X 24-Cores`, `MemTotal: 263400384 kB`, `6.8.0-138-generic`, then
  `build-info.json`.
  - `nproc` reads 16 (the pod's CPU affinity); the runner's `os.cpu_count()` reads 48 (the host's threads).
  - `MemTotal` is the host's memory (about 251 GiB), not the pod's 128 GB allocation. On this route the memory a run
    may use is read from the API pod record, not from `host.txt`.
- `progress.txt`: `start X-m2-t8-k0-c5`, `end X-m2-t8-k0-c5 exit 0`, `end X-m2-t8-k0-c5.gridzeros exit 0`.
- `runner_state.json` (the final exit-status file): `state done`, `exit 0`.
- `runs/`: `X-m2-t8-k0-c5.{jsonl,log,out,gridzeros.jsonl,gridzeros.log}`: the same file names `entry.sh` writes.
- The two tarballs have different SHA-256s (gzip stores a time stamp); their extracted contents are identical, which is
  what the transfer check compares.
- Development observation only: the same stage run locally with `RAYON_NUM_THREADS=4` gave the same
  `degree_reached` 5 and `inconsistent` true but a different `muladds` (5,680,719,625 locally, 5,776,982,981 on the
  pod at rayon 12), as Amendment 9's instrument note says (F4 chunk size depends on the thread count).

## Cost

- Pod price read from the API: US$1.04/h (`costPerHr` of the pod record, cpu5m, 16 vCPU).
- Pod lifetime 47 s, so about US$0.014 (derived: price × lifetime; if RunPod bills a whole first minute, US$0.017).
- No other paid resource was created.

## Cleanup

- `fetch.sh` deleted the pod (HTTP 204) and listed the account's pods: no pod named `pkm-x-*` remained.
- A later re-listing at the end of this session also showed no `pkm-x-*` pod. The pods running at that time
  (`pfd-b-rp`, `pkm-odd-highmem-20261114`, `icgpu-ric`, `bdr-wallfix`) belong to other sessions and were not touched.

## Changed after the smoke

Revision 1 (before the review):
- `run.sh` also writes `pod-request.json` (the create body with the runner token and the base64 runner withheld), so that
  `interruptible: false` and the reserved-tier `cloudType` are on record even though the pod record returns
  `interruptible: null`. The change is one `jq` line, checked offline on a sample body; not exercised on a pod.

Revision 2 (after the independent review of `59e487150f`; draft Amendment 18 sections 2-6). None of this was exercised
on a pod; it is covered by the offline tests in `tests/README.md` and needs one more development smoke before the
freeze (amendment section 9, point 5):
- the pod's start command is now `bash -c` with a cgroup copy loop before the unchanged runner. **Which cgroup files a
  RunPod CPU pod exposes is not known**: this smoke did not read `/sys/fs/cgroup`. The loop copies the v2 files or the
  v1 files, whichever exist, and records which;
- one pinned flavor (`cpu5m`, 16 vCPU, 128 GB) with no fallback, checked on the create response (reserved tier, flavor,
  size, `costPerHr` equal to the price read before create);
- the client is a pinned copy (`rp_pinned.sh`, unchanged, SHA-256 `b4b60199…`), and `run.sh` checks both pins;
- the allow-list, the registered order, the receipts and results directories that are never reused, the study ledger,
  the protocol check, the layer check, the cleanup trap set before create, the deadlines written before start, the
  outcome classifier in `fetch.sh`, the raw-upload gate before delete, and the backstop `reap.sh`;
- receipts and results now live at fixed paths (`receipts/`, `receipts_dev/`, `results/`): the `RPC_RECEIPTS` and
  `RPC_RESULTS` overrides used in the command above were removed, so they cannot bypass the reuse checks;
- new API records drop `consumerUserId`. The smoke receipts above are left as committed (the field is an account
  identifier, not a credential, and it is already in the history).
