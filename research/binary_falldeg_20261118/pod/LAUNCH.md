# Launch record, 2026-11-18: binary d_F ladder (Amendment A1 screen + degree-5 opt-in)

Owner choice (this session): "Screen + degree 5 now". Every number this job produces is `[pcc:unverified]`
(no S3 resume checkpoint, bench.record or pcc claim reachable from the session container).

| item | value |
|---|---|
| pod | `bl-ladder`, `k7fjlexahsl468`, cpu3m (cpu5m sold out), 32 vCPU, 256 GB, EU-RO-1, container disk 80 GB |
| host | AMD EPYC 9654 (AVX-512), load average 1.8-2.6 at start |
| price, cap | US$1.76/h, lifetime cap 374,400 s, worst case US$183.04 |
| pushed (SHA-256 verified by the pod) | `binary_falldeg_arms` 2ed36cca... (7,368,384 B), `job.sh` d0922cf9... |
| smoke | `job.sh dry-run`, 04:07 UTC: DRY_2-23-11 ok, 2 rows, 15 s |
| job | `ladder`, 04:08:54 UTC: `DEG5=1 TOTAL_BUDGET_H=100 bash job.sh` (base plan in order, then P1x_3-21-7 at degree 5, keep 1) |

Teardown: fetch `/workspace/out/`, `pod.py terminate k7fjlexahsl468 --yes`, remove the proxy host from the environment.

## 2026-11-19

- 04:45 UTC: last contact with job `ladder`: degree-5 cell P1x_3-21-7 (3:21:7, cmax 5, one draw per arm) at
  4,396 s wall, RSS 64.7 GB, 1 of 3 rows written; 06:26 UTC: 10,479 s, RSS 86.0 GB, still 1 row.
- 08:47 UTC: the pod `k7fjlexahsl468` no longer exists: the control server answers 404, the RunPod API
  GET /pods/k7fjlexahsl468 returns 404, and the pod is absent from `pod.py ls --all`. This session did not
  terminate it. No P1x row beyond the first was fetched; the degree-5 cell is **not completed** (no d_F >= 5
  reading). Every S1/P2/AA cell had been fetched and committed before (rows in raw/pod-20261118/).
- Spend: about 04:08 UTC 2026-11-18 to at most 08:47 UTC 2026-11-19, about 28.6 h x US$1.76 = at most about US$50.
