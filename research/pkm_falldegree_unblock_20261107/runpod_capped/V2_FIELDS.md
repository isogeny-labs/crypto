# RunPod REST v2: the fields this route reads (2026-11-21)

What the Amendment 33 port of `runpod_capped/` reads from RunPod's REST v2 API (`https://api.runpod.io/v2`), and the
REST v1 field each one replaces. Sources, in the order of precedence used: read-only `GET` calls made on 2026-11-21
(about 20:00Z) with this session's key (never in argv: a mode-0600 header file); the public specification
`https://api.runpod.io/v2/openapi.json` (version `2.0.0`, read the same day); main's v2 client `runpod/rp.sh`
(Amendment 32). No pod was created and no paid call was made. Environment values, the container command, SSH details and
account or user identifiers were dropped before anything was saved. Every statement here is `[pcc:unverified]`.

REST v1 (`rest.runpod.io/v1`) answered with a proxy 502 from this container; v2 answered 200.

## Calls

| purpose | v1 (draft Amendment 18) | v2 (Amendment 33) | observed |
|:--|:--|:--|:--|
| price and stock | GraphQL `cpuFlavors { ramMultiplier }`, `specifics { stockStatus securePrice }` | `GET /catalog/cpus/cpu5m?include=AVAILABILITY&product=POD&vcpuCount=32` (16 until owner decision 24) | 200: `availability` `HIGH`, `ramGbPerVcpu` 8, `vcpu` `{min 2, max 32}`, `price.securePerVcpu` 0.065, so 16 vCPU cost 1.04 and 32 vCPU 2.08 per hour; at 32 vCPU the data centers `EU-RO-1` and `EUR-IS-1` were listed at about 21:30Z |
| create | `POST /v1/pods` | `POST /pods` | not called (paid); the spec answers 201 (created) or 202 (queued, only with the `queue` option) |
| pod record | `GET /v1/pods/{id}` | `GET /pods/{id}` | 200 on an existing pod; 404 with body `{"detail":"pod not found","status":404,"title":"Not Found"}` on an unknown id |
| list | `GET /v1/pods`, a bare array | `GET /pods` | 200: `{"pods":[...], "pagination":{"hasNextPage":false, "nextCursor":null}}`; 26 pods, all `EXITED`, none named `pkm-x-` or `pfd-` |
| delete | `DELETE /v1/pods/{id}` | `DELETE /pods/{id}` | not called; the spec lists 204, 404 and 409 |
| spend (ledger only) | `GET /v1/billing/pods` | `GET /billing/pods?startTime=…&endTime=…&bucketSize=day` | 200: `records[]` of `{podId, startTime, endTime, totalAmount, cpuAmount, gpuAmount, diskAmount}` and `metadata.totals`; used for `billing_v2_20261121.tsv` |

The list is paginated: `list_pkm` (`common.sh`) follows `pagination.nextCursor` with the `cursor` query parameter while
`hasNextPage` is true, and fails (no information) if a page is unreadable or the envelope is missing. A pod in a
cluster is left out of `GET /pods` unless `includeClusterPods=true`; this route creates none.

## Pod record fields

Field names of `GET /pods/{id}` on an existing CPU pod (keys only; [pcc:unverified]): `actions`, `args`, `cloud`, `cost`, `cpu`,
`createdAt`, `cudaVersion`, `dataCenterId`, `disk`, `env`, `globalNetworking`, `id`, `image`, `locked`, `mounts`, `name`,
`ports`, `registry`, `runtime`, `ssh`, `startedAt`, `status`, `tags`, `template`. A list entry of `GET /pods` has the
same keys, `env` and `args` included (review 4 read all 26 entries: `env` is an object, `args` a string, so every pod's
environment, a runner token included, is in the list response), and `runtime` is null there. `list_pkm` keeps only
`{id, name, status, cost, createdAt, startedAt}` of each entry, so nothing else is held or saved. (Revision 5 of this file
said list entries lack `env` and `args`; that was wrong.) Example values on an exited `cpu5m` pod: `cloud` `SECURE`, `cost` 0.52,
`cpu` `{id: cpu5m, memory: 64, vcpuCount: 8}`, `dataCenterId` `EU-RO-1`, `status` `EXITED`, `actions`
`[start, terminate]`, `createdAt` `2026-10-19T14:24:35.226Z` (ISO 8601 with milliseconds; v1 wrote
`2026-11-14 23:24:47.000 +0000 UTC`).

| v1 field the route read | v2 field | how the route uses it |
|:--|:--|:--|
| `machine.secureCloud` (true) | `cloud` (`SECURE` or `COMMUNITY`) | D7 check and admission rule 7: `cloud == "SECURE"` on the create response and the GET record |
| `costPerHr` | `cost` (USD per hour; returned in every status) | equal, to 4 decimals, to the catalog price read before create; cost receipts (`cost_per_hr`) |
| `cpuFlavorId`, `vcpuCount`, `memoryInGb` | `cpu.id`, `cpu.vcpuCount`, `cpu.memory` (GB) | D7 and rule 7: `cpu5m`, 32, at least 256 GB (owner decision 24) |
| `machine.dataCenterId` | `dataCenterId` (documented as assigned by the scheduler; smoke3 had it at create) | checked non-empty on the record read once the runner answers (`pod-record-ready.json`), before upload |
| `machineId`, `machine {...}` | none | not recorded any more; the CPU model still comes from `host.txt` |
| `desiredStatus` (`RUNNING`, `EXITED`, `TERMINATED`) | `status` (`PROVISIONING`, `STARTING`, `RUNNING`, `EXITED`, `ERROR`, `TERMINATED`) | loss evidence: an HTTP 200 record whose `status` is not `RUNNING` (P3/P6) |
| `lastStartedAt` | `startedAt` | recorded at every check-in; a change alone is never evidence (S1) |
| `lastStatusChange` (for example "Rented by User: …") | **none** | v1's first form of host-side evidence ("a RunPod-side status change") cannot be observed. It never counts: a LOST gets relaunch `auto` only from the second form, the pod gone from a successful list with GET 404. Fail closed: fewer automatic relaunches, more cases for the owner. |
| `interruptible` (request; the record returned null) | **none** in the request or the record | v2 has no spot or interruptible pod field: the spec's `CreatePodRequest` has none and unknown fields are refused, so a v2 pod in `cloud: SECURE` is on-demand. `pod-request.json` must not contain the key (rule 7). |
| `env` (GET, used for token recovery) | `env` (GET of one pod, and in every list entry) | the token recovery of decision N6 reads `.env.RUNNER_TOKEN` into `RPC_STATE` only; the observed CPU pod returned `env` with key `PUBLIC_KEY` |
| `consumerUserId` | none | the saved records drop `env`, `args`, `cmd`, `entrypoint`, `ssh` and `consumerUserId` |
| list: bare array | `{pods, pagination}` | every page is read |

## Create body

`run.sh` sends main's v2 shape [pcc:unverified]: `{name, cloud: "SECURE", cpu: {id: "cpu5m", vcpuCount: 32}, image, disk: 20,
ports: ["8000/http"], env: {RUNNER_TOKEN}, entrypoint: ["bash", "-c"], cmd: [<boot script>]}`. The image is main's pinned
digest `python:3.12-slim@sha256:5e2bdb1b…` (same tag as the two development smokes, now by digest). The spec says
`entrypoint` and `cmd` are encoded into `args`, which the record returns as one string; the route never reads `args`
and drops it from every saved record. The v1 fields `computeType`, `cloudType`, `cpuFlavorIds`, `cpuFlavorPriority`,
`vcpuCount` (top level), `interruptible`, `imageName`, `containerDiskInGb`, `dockerEntrypoint` and `dockerStartCmd`
are gone.

## Not touched by the port

The pod-proxy calls (`rp_pinned.sh upload | start | status | fetch`, to `https://<pod id>-8000.proxy.runpod.net`, with
the pod's own token) do not use the REST API, so the v1 to v2 change does not reach them. `rp_pinned.sh` still contains
v1 `create`, `pod` and `delete` commands; the route never calls them (offline test G-rpuse). Main's current `rp.sh`
`start` reads the pod's price over REST; the pinned copy's `start` does not.

## What a pod has shown, and what is still unobserved

The third development smoke (`pkm-x-dev-smoke3`, pod `klogu58zoe2owr`, 2026-11-21, `receipts_dev/pkm-x-dev-smoke3/`,
t = 8, not data) showed with REST v2 [pcc:unverified]:
- the create answered HTTP 201 with `cloud` `SECURE`, `cpu` `{id: cpu5m, memory: 256, vcpuCount: 32}`, `cost` 2.08
  (equal to the catalog price) and `dataCenterId` `EUR-IS-1` already set (so the value may be set at create; the route
  still reads it from the record taken when the runner answers);
- the GET record and the ready record: the same `cpu`, `status` `RUNNING`, `dataCenterId` `EUR-IS-1`, `startedAt`;
- the runner answered through the proxy URL on the v2-created pod, with the bash entrypoint and the pinned image;
- the cgroup `memory.max` before start: 256,000,000,000 bytes;
- the git gate passed, `DELETE` answered 204 and the pod was gone from the list; pod life 155 s, about US$0.09;
- a later read-only GET of the deleted pod answered 404 with `{"detail":"pod not found",...}` (review 4).

Still unobserved on a pod:
- token recovery from `GET /pods/{id}` `env.RUNNER_TOKEN` (offline tests C17 and F10 only);
- whether `startedAt` changes on a container restart, and which `status` a pod shows while RunPod restarts or migrates
  it (no smoke provokes either);
- `memory.oom.group` and `vm.max_map_count` on a 256 GB pod (recorded from now on in `cgroup-prestart.json`).
