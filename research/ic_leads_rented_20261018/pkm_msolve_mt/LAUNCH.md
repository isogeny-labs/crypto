# Launch cards P2a and P2b: PKM Kummer tower, m = 3, msolve `-t` with 22 h walls

**Protocol:** [`research/pkm_m3_n21_msolve_20261018/PROTOCOL.md`](../../pkm_m3_n21_msolve_20261018/PROTOCOL.md), Amendment A1 (frozen 2026-10-18 before any multithreaded run).
**Question:** does msolve's F4 finish degree 7 (plateau, `D = 7`) or take a productive degree-8 step (H0) on the m = 3 Kummer systems at N = 21 (P2a) and N = 24 (P2b)? The first run stopped at its 2–4 h single-thread walls inside degree 7 (RESULTS.md).
**Decision rule:** A1.4. A productive degree ≥ 8 round on either target → H0. Both finished with D = 7 → plateau. A wall or memory stop → inconclusive.

| | P2a (N = 21, both t7 targets) | P2b (N = 24, both t8 targets) |
|---|---|---|
| host | RunPod `cpu3m-32-256` (32 vCPU, 256 GB) | same |
| price | **$1.76/h** (queried 2026-10-18 21:13Z) | $1.76/h |
| per process | 16 threads, `CAP` = 0.85 × limit / 2 ≈ 108 GB | same |
| stages | boot + msolve package 0.2 h · control m3_t5 at `-t 16` 0.1–0.3 h · two systems at once, wall 22 h · serving 3 h | same |
| expected hours | **22.5 h** if both reach the wall; less if degree 7 ends and degree 8 resolves early (range 6–23 h) | 22.5 h (range 6–23 h) |
| expected cost | **$39.60** (range $10.60–$40.50); with the full serving window $45; lifetime cap 26.6 h = **$46.73 at most** | same |
| P(decision) | about 0.5 (A1.5: degree 8 may need 40–120 GB) | lower; N = 24 is larger |

Run P2a first. P2b's reading at N = 24 matters only together with N = 21's, so P2b waits for P2a unless both are funded at once.

## Minimal configuration (the about $12 left for P2 + P4 together)

One system only, `m3_t7_x221026520` (target 0), on `cpu3m-16-128` ($0.88/h, queried 22:04Z): 16 threads, `CAP` ≈ 109 GB, wall 6 h. Expected 6.6 h, **$5.80**, cap 7.5 h = $6.60. It reads only H0 (a productive degree-8 step on target 0). "Finished D = 7" needs both targets (A1.4), so a finished run here is recorded but the plateau row stays open.

```sh
python3 launch.py run --job pkm-msolve-mt --name pkm-msolve-mt-t7-min --branch main \
  --flavors cpu3m --vcpu 16 --disk 40 --max-seconds 27000 --env SERVE_SECONDS=1800 \
  --env MSOLVE_SYSTEMS=pkm_kummer_p2013265921_m3_t7_x221026520 --env MSOLVE_WALL_MAX=21600
```

## Launch (full configuration)

```sh
cd research/dreg_closure_m4l6_20261018/pod
# P2a
python3 launch.py run --job pkm-msolve-mt --name pkm-msolve-mt-t7 --branch main \
  --flavors cpu3m --vcpu 32 --disk 60 --max-seconds 95600 --env SERVE_SECONDS=10800
# P2b
python3 launch.py run --job pkm-msolve-mt --name pkm-msolve-mt-t8 --branch main \
  --flavors cpu3m --vcpu 32 --disk 60 --max-seconds 95600 --env SERVE_SECONDS=10800 \
  --env "MSOLVE_SYSTEMS=pkm_kummer_p2013265921_m3_t8_x1227422598 pkm_kummer_p2013265921_m3_t8_x1272776806"
```

A 12 h variant (the owner's lower bound) adds `--env MSOLVE_WALL_MAX=43200` and uses `--max-seconds 59600`: about $22 per pod.

## Watch and fetch

`STATUS` lines: `MSOLVE_MT plan …`, `MSOLVE_MT control agrees (…wall_s=…)` or `MSOLVE_MT_CONTROL_FAILED`, `MSOLVE_MT running`, `MSOLVE_MT_DONE`. The live traces are at `msolve-mt/<system>.log`. A degree-8 line with `new` > 0 in either log is the H0 reading, so the run can be stopped there to save money (A1.4 needs nothing more).

```sh
P=research/dreg_closure_m4l6_20261018/pod/launch.py
python3 $P fetch ID research/pkm_m3_n21_msolve_20261018/runs-mt --path msolve-mt/
curl -sS https://ID-8000.proxy.runpod.net/pkm-msolve/progress-mt.txt -o research/pkm_m3_n21_msolve_20261018/progress-mt.txt
python3 $P terminate ID
```

## Tested before launch (authoring container, 4 vCPU, msolve 0.6.5 from the Ubuntu package)

`job.sh` ran end to end locally with `MSOLVE_SYSTEMS` set to a small m = 2 system: host record, the control m3_t5 at `-t 4` under the gate (`cmp` against the committed `.out`), then the measured system. The control agreed (its `.out` is identical to the committed one) in 1,917 s at `-t 4` on a contended 4 vCPU box, which is not a speed-up figure. The m2 system then ran (exit 0), and `MSOLVE_MT_DONE` followed. Records are in `test_local/`.
