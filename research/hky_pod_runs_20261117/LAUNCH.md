# Launch record, 2026-11-17: hky2 at depth 13 and 14, optlevel on rented hosts

Owner-authorized ("Do 1-2", this session), environment RunPod (isolab-eks, S3, Conductor and
research-kb are not reachable from the session container, so no resume checkpoint in S3, no
bench.record and no pcc claim exist for these runs; every number they produce is `[pcc:unverified]`).

| item | CPU pod | GPU pod |
|---|---|---|
| name, id | `hky2-cpu-t14`, `gz5n46wfmmg86r` | `optlevel-gpu`, `qkscc4xn0nnj2o` |
| hardware | cpu5m, 32 vCPU, 256 GB, EUR-IS-1; host is a 128-thread AMD EPYC (AVX-512), shared: load average 24 to 65 at start | NVIDIA RTX 2000 Ada 16 GB, 6 vCPU, 31 GB, EU-RO-1; host AMD EPYC 7443 |
| image | `runpod/base:1.0.2-ubuntu2404` | `runpod/pytorch:1.0.2-cu1281-torch280-ubuntu2404` (nvcc 12.8, ncu present, nsys absent) |
| price, cap | US$2.08/h, 86,400 s, worst case US$49.92 | US$0.24/h, 7,200 s, worst case US$0.48 |
| created | 15:5x UTC, REST v2, `pod.py` (the lfd-summation-systems launcher with a CPU branch added) | same |
| pushed (SHA-256 verified by the pod) | `pilot_candidate` 47d6c281… (the frozen binary of `research/hky_engine_speed_20261117`, 7,666,096 bytes), `optlevel.tar.gz` 84d2bb71…, `job.py` 82899a74…, `cpu_job.sh` 16b28418… | `optlevel.tar.gz` 84d2bb71…, `job.py` 82899a74…, `gpu_job.sh` 30fc9cdc… |
| job | `cpu`: optlevel probe, controls, hky2 t = 10 under `measure`; then t = 13 draws 0 and 1 (c <= 6); then t = 14 draw 0 (c <= 6, budget 79,200 s) | `gpu`: optlevel probe, `gpu probe`, `gpu calibrate`, CPU controls |

## Predictions written before any result (from `research/hky_engine_speed_20261117/README.md`)

- t = 13, draw 0: the same dims as on the 4-vCPU container (V_5: 39,657 of 83,682, no 1; 1 enters at c = 6),
  and the same `muladds` (2.4217e13) if the schedule is thread-count independent; this is not known.
  Draw 1: 1 enters at c = 6 (expected last fall degree 6 at t = 13).
- t = 14, c = 6: closure without 1, dimension 448,922 of 499,178 (value of the other repository's
  engine). A closure that contains 1 at c = 6, or another dimension, contradicts that engine.
  Cost estimate 1.2e14 to 4e14 muladds; may be off by 3x or more.
- optlevel: `perf` is not expected to work in a container (GPU host reports `perf_event_paranoid` = 4);
  if so the T-pmu path stays unvalidated and that is the finding.

## Known confounders

- The CPU host is shared and was loaded at start, so the "quiet host" condition for the optlevel
  full-parallel points is not met; the records carry the load average.
- Wall times on this host are not comparable with the container's or the earlier pods' beyond a factor.
