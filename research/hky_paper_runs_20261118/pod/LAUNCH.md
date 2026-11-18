# Launch record, 2026-11-18 06:11 UTC: J1-J4 (paper V_6 closures, depths 14-16)

Owner request: "Do all 4 all at once" after an optimisation round. Every number is `[pcc:unverified]`
(no S3 checkpoint, bench.record or pcc claim reachable from the session container).

Binary `pkm_tower_pilot` 39acdeac... (450d5b295b, merged with engine-opt c51685b6a9, gate PASS),
`run_job.sh`, `BINARY.sha256`; J1/J2 also `hky_lift` 6824a8bd... + `pod_k14.sh` + the recorded lift JSON.
All pushed to `/workspace/in/code/`, SHA-256 verified by each pod. All pods cpu3m (cpu5m sold out),
32 vCPU, cgroup 256 GB (job limit 250 GB), US$1.76/h.

| job | pod | host CPU | instance | flags | disk | lifetime cap (worst case) |
|---|---|---|---|---|---|---|
| J1 | hky-j1 `j5ym8cqln1m237` US-CA-2 | EPYC 7713 | unit-p65537-a1-s10-k14 | VERIFY=1, export, then lift D6->7 | 200 GB | 43,200 s (US$21.12) |
| J2 | hky-j2 `5kmpu4gisqvd24` US-MD-1 | EPYC 7702P | unit-p163841-a3-s21-k14 | VERIFY=1, export, then lift | 200 GB | 43,200 s (US$21.12) |
| J3 | hky-j3 `g9w09ju7t3idas` EU-RO-1 | EPYC 9655 | unit-p65537-a1-s10-k15 | VERIFY=1, export | 320 GB | 216,000 s (US$105.60) |
| J4 | hky-j4 `tonzchna7tb5p8` US-MD-1 | EPYC 7702P | unit-p786433-a1-s10-k16 | EXPORT=0, VERIFY=0 | 320 GB (max) | 864,000 s (US$422.40) |

Predictions (PREDICTIONS.md, before any pod data): J1/J2 12,000 s (7k-24k), 31 GB (x2 with VERIFY);
J3 70,000 s (33k-170k), 75 GB; J4 370,000 s (170k-950k), 170 GB. Expected for J1/J2: profile
0,0,2,154,4451,73587,448922 with 1 not in V_6; lift: 1 in V_7, dim V_7 1,683,218, dim F 50,256.
Note: J4 checkpoints (about 1x RSS, two kept while writing) may exceed the 320 GB disk cap late in
the run; a failed checkpoint is recorded and the closure continues (450d5b295b).
