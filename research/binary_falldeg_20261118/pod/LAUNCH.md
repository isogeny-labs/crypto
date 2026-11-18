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
