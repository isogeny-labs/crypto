# Growth-law cells rerun: result (2026-11-18)

Plan: [`PLAN.md`](PLAN.md), frozen 2026-11-15 before any row. Binary `pkm_tower_pilot` from
commit `3ee769f4cc` (see [`source_commit.txt`](source_commit.txt)), SHA-256
`4da13ccf…c620b` ([`binary.sha256`](binary.sha256)), checked on the pod before any cell
(`out/host.txt`). Every value here is `[pcc:unverified]`: producer-reported, no pcc claim
registered, no independent replay.

## What ran

One RunPod CPU pod (`cpu5m`, 16 vCPU, 128 GB cgroup, pod `es6cpobseelc9t`), job
[`cells_job.sh`](cells_job.sh) started 2026-11-15T19:52:17Z. The session copied `out/` back at
2026-11-16T00:06Z. The next poll, due 04:07Z, was delivered on 2026-11-18T01:09Z; by then the pod
no longer existed in the account (absent from the pod list, endpoint 404). Who or what
terminated it is not recorded here. Everything after the 00:06Z copy is lost. `out/` below is
that copy.

## Rows (per draw, from `out/prog_*.jsonl`)

| cell | W | t | draw | degree c | `1 ∈ V_c` | columns | multiply-adds | seconds |
|:--|--:|--:|--:|--:|:--|--:|--:|--:|
| (a) | 1 | 12 | 0 | 6 | yes | 190,051 | 23,614,429,269,316 | 10,479.2 |
| (a) | 1 | 12 | 1 | 6 | yes | 190,051 | 23,614,429,251,323 | 10,484.6 |
| (b) | 3 | 13 | 0 | 4 | no | 17,902 | 32 | 0.0 |
| (b) | 3 | 13 | 0 | 5 | no | 83,682 | 24,640,159,831 | 13.2 |
| (b) | 3 | 13 | 1 | 4 | no | 17,902 | 32 | 0.0 |
| (b) | 3 | 13 | 1 | 5 | no | 83,682 | 24,640,159,831 | 14.3 |

None of these closures is censored. Peak memory during (a): 85,022 MB (`out/commands.log`).
Cell (b)'s degree-6 closure started at 22:47:18Z and was still running at the 00:06Z copy
(at least 79 minutes); it has no row. Cell (c) never started.

## Reading (rules of PLAN.md)

- **(a) W = 1, t = 12: holds on both draws.** `1 ∈ V_6` on both draws, and the degree-5
  closures were already complete without `1` on both draws (`../falsify/`). So `d_F = 6`, the
  registered prediction. The final rows (`out/nullw1_m2_t12_k*.jsonl`) give
  `last_fall_degree = 6`.
- **(b) W = 3, t = 13: censored.** `1 ∉ V_5` on both draws, uncensored, so `d_F ≥ 6`. The
  prediction `d_F = 7` is neither confirmed nor refuted; no lower fall was seen.
- **(c) W = 2, t = 14: censored, no data.**

Conjecture G is not falsified by this run. One of its three open cells is now decided in its
favour.

## Spend

The pod's billed hours are not known from this session (the REST API was unreachable on
2026-11-18 and the pod record is gone). At the plan's rate of about USD 1.06/h, the window
from launch to the last confirmed alive time (00:06Z) is about USD 4.5. The upper bound is
the plan's USD 80 ceiling.

## What a second attempt needs

PLAN.md allows no retry; a new plan is needed for (b) and (c). Lessons for it: poll
through a mechanism that survives container restarts (the session's check-ins were delayed
by about 45 h), copy `out/` to durable storage from the pod itself (for example S3 on each
progress row), and record the benchmark record outside the container (the open benchrec
record for this run was lost when `/dev/shm` was wiped on 2026-11-15).

## Files in `out/` (SHA-256)

```
28488b6731bbdc134efc1528211e07601b30a4c03687c1aaba78be9181b80c94  commands.log
b3b9550b2f8ef7011a61f8a501e619f4ed9ee649913324349a51a25645bc5c0b  host.txt
8f34c8874822328fc99db4a4c6987a4e4646e408389402a8217e755f61be63a5  prog_nullw1_m2_t12_k0.jsonl
324e07a789bd20cf9839b8c388d787bd4293a69d996d9a462eddbd48fb5d53db  prog_nullw1_m2_t12_k1.jsonl
72e0d0a2fa0891c9142de008ba12089e6a3120db3fda233c92c60e0e70dea005  prog_nullw3_m2_t13_k0.jsonl
12a9e04237774527b3663c90eb272a76e6dbe4ed1343ce7504231ace873a1520  prog_nullw3_m2_t13_k1.jsonl
c81fdee232c55464609cc9e1fb1944ea5b0fe8f7d0eceab73078661ae2ce4b02  nullw1_m2_t12_k0.jsonl
f12a029c74a9a07dcb881dcbf9bb6c41b95ac76125f517dc2c5a058a9d6898ee  nullw1_m2_t12_k1.jsonl
bd4e6eeb0cfbb385eef2f73fe248ae0fe7c7fb5310a45855954fdcc58f1dc6b9  run_nullw1_m2_t12_k0.log
da8ed6724aa64885810ce41f8c76b2bdc35d3d8cd9be0e8c184be4abdf4d36c0  run_nullw1_m2_t12_k1.log
d5919a864975bca9d25544d6a1860865313513b8a17d93aa1499ca0fb2fede7c  run_nullw3_m2_t13_k0.log
91497770be1e9b45a6447a49f89dc1519501def71382cd7db75b688ba9ddee8f  run_nullw3_m2_t13_k1.log
```
