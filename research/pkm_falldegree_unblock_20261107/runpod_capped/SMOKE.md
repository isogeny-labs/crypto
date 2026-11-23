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

Revision 3 (after the second independent review of `7305d2ee8d`; `DECISIONS_ROUTE.md` lists the decisions). Also not
exercised on a pod; offline tests only (`tests/README.md`):
- `run.sh` now fetches the pod's `/results` once before `/start`, to read the cgroup memory limit the boot loop copied
  there, and refuses a limit below 112 GiB or a missing limit file;
- the classifier counts only positive evidence of loss, retries "no information" until the hard deadline (class
  NO_EVIDENCE then), routes a LOST to an automatic relaunch only on a recorded RunPod-side status change, and fails
  closed on unparsable receipts;
- collections are numbered and never overwritten; the reaper deletes only past the hard deadline;
- `fetch.sh` recovers a missing pod token from the REST record; the scripts print a reminder to commit and push the
  receipts and the ledger.
- Facts about RunPod that the next development smoke should record: the cgroup files, what `lastStatusChange` reads,
  whether `GET /pods/{id}` returns `env`, and whether a pod sees `RUNPOD_POD_ID`.

## Second development smoke (2026-11-15, revision a18-r3): what only a pod could show

Pod `pkm-x-dev-smoke2` (id `bpxojdig3u54d4`), stage `cap:8:0:5:12800000000`, receipts in `receipts_dev/pkm-x-dev-smoke2/`,
results in `results/pkm-x-dev-smoke2/collect-1/` (git-excluded; raw tree in S3 under bench record
`benchmarks/records/20261115T0156Z-pkm-fd-rpc-dev-smoke2.json`, closed as `invalid`: a development check, not data).
Every number is `[pcc:unverified]`.

- Launch at commit fd5a92da08 with the a18-r3 scripts (hashes logged in `run.log`): price read, pod created on the
  pinned flavor `cpu5m` (16 vCPU, 128 GB, US$1.04/h, data center EUR-IS-1, `secureCloud` true in the create response),
  the new start command booted, the runner answered, the seven uploaded files' pod-side SHA-256 equal the local ones.
- cgroup: version 2. `memory.max` reads 128,000,000,000 before start, so the pre-start check (at least 112 GiB) passes.
  `memory.peak`, `memory.events` (`oom_kill 0`) and `memory.max` were copied into the results tree.
- Memory figures for this run: pilot `VmHWM` 63 MiB (used), cgroup `memory.peak` 104,304,640 bytes (recorded).
- `lastStatusChange` at creation reads `Rented by User: <date>`, so the classifier's "by user" exclusion matches
  RunPod's text. Its value after a RunPod-side event was not observed (none happened).
- Host: `model name : AMD EPYC 9575F 64-Core Processor`, `MemTotal` 527,605,596 kB, `nproc` 16 in the container and
  128 from the runner (`host.txt` in `admission-lines-1.txt`, `status-ready.json`). The first smoke ran on an AMD Ryzen
  Threadripper 7960X, so the pinned flavor `cpu5m` does not pin the CPU model.
- `GET /pods/{id}` was seen by the session to return `env` (keys `PUBLIC_KEY`, `RUNNER_TOKEN`). No committed receipt
  shows it, since receipts drop `env`, and token recovery was not exercised on the pod: this is the session's
  observation only.
- Collection: one collection, transfer check true, outcome ADMITTED P2d; the offline classifier
  (`fetch.sh --classify-dir`) gives the same verdict. The raw tree was uploaded with `tools/benchrec.py`, and
  `fetch.sh --delete RECORD` confirmed the upload, deleted the pod (HTTP 204) and confirmed it gone. Pod lifetime 188 s,
  about US$0.054 (derived: costPerHr x lifetime).
- `reap.sh` read the live list afterwards: no `pkm-x-` pod, nothing touched.
- Not checked: whether `lastStartedAt` changes on a container restart (no restart was provoked), whether the pod sees
  `RUNPOD_POD_ID` or a pod-scoped key (S12), and whether `GET /pods` is complete for large accounts.

## The Amendment 33 port (2026-11-21): what the two smokes above do not cover

Both smokes above ran with REST v1 and the GraphQL price read, the route's earlier names and the runner at
`runpod/server.py`. The port changes, none of them exercised on a pod yet (offline tests only, `tests/README.md`):
- every REST call is REST v2 (`api.runpod.io/v2`): the price from the CPU catalog, the create body of main's
  `runpod/rp.sh` (`cloud`, `cpu {id, vcpuCount}`, `image`, `disk`, `ports`, `env`, `entrypoint`, `cmd`), the GET record,
  the paginated list and the delete; the field mapping is in `V2_FIELDS.md`;
- the image is main's pinned digest of `python:3.12-slim`; the runner is `server_pinned.py`, byte-identical to the
  `runpod/server.py` the smokes ran (`f56fd08e…`);
- the create must answer 201; the data center is read from a record taken once the runner answers;
- the git gate replaces the bench-record gate before a completed pod is deleted (owner decision 22).

The development smoke at t = 8 that the main session runs after review should record, with REST v2: the create
response (HTTP code and its `cloud`, `cpu`, `cost`, `dataCenterId`), the live GET record (`cpu.memory`, `status`,
`dataCenterId`, `startedAt`, and whether `env` carries `RUNNER_TOKEN`), the runner answering through the proxy with the
bash entrypoint and the pinned image, the cgroup files before start, the git gate on a committed and pushed tree (a
development tree is git-ignored, so it is added with `git add -f` for the check), `DELETE` (204), the list afterwards
and a GET of the deleted pod (404). Every statement here is `[pcc:unverified]`.

## Owner decision 24 (2026-11-21): the smoke moves to the 32-vCPU pod

The route's one flavor is now `cpu5m` at 32 vCPU and 256 GB (US$2.08/h), so the development smoke runs on that size
(a `t <= 8` stage costs under US$0.50 for under 15 minutes). In addition to the list above it should record the
cgroup `memory.max` of a 256 GB pod (the pre-start check needs `max` or at least 256,000,000,000 bytes) and the GET
record's `cpu.memory` (the floor is 256). A development stage may use `--max-dense` 54000000000. Every statement here is
`[pcc:unverified]`.

## Third development smoke (2026-11-21, REST v2, 32 vCPU / 256 GB)

Pod `pkm-x-dev-smoke3` (`klogu58zoe2owr`), stage `cap:8:0:5:12800000000` (t = 8, not data), receipts in
`receipts_dev/pkm-x-dev-smoke3/` [pcc:unverified]: create HTTP 201 with `cloud` `SECURE`, `cpu` `{cpu5m, 32, 256}`,
`cost` 2.08 equal to the catalog price, `dataCenterId` `EUR-IS-1` already at create; GET and ready records `RUNNING`;
the runner answered through the proxy with the bash entrypoint and the pinned image; `memory.max` 256,000,000,000
before start; classified ADMITTED P2d; the git gate passed on commit `88b64ac5d5`; `DELETE` 204 and the pod gone from
the list; pod life 155 s, about US$0.09. Not exercised: token recovery from the record's `env`, and `startedAt` and
`status` on a container restart. `memory.oom.group` and `vm.max_map_count` were not yet recorded (they are from
revision 7 on).
