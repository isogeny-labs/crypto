# PMU probe on RunPod (item 5), 2026-10-18

**Result: hardware counters are not available to RunPod containers.** The host has a PMU, but the container is not allowed to use it.

One pod, `ax0ykpqnzxvbqh`: cheapest CPU instance `cpu3c-2-4` (2 vCPU, 4 GB) at $0.06/h, secure cloud, lifetime cap 900 s, launched 21:14:27Z. It answered at 21:17:37Z and was terminated at 21:17:50Z, after about 3.5 minutes (cost under $0.01). It is gone from `launch.py ls`.

| field | value |
|---|---|
| CPU | AMD EPYC 9655 96-Core (192 CPUs visible), kernel 6.8.0-90-generic |
| `/sys/bus/event_source/devices` | includes `cpu`, `ibs_fetch`, `ibs_op`: a real PMU |
| `/proc/sys/kernel/perf_event_paranoid` | **4** |
| `perf_event_open(PERF_COUNT_HW_CPU_CYCLES, user only, self)` | **EPERM** (Operation not permitted) |
| `perf_event_open(PERF_COUNT_HW_INSTRUCTIONS, user only, self)` | **EPERM** |
| status | `unsupported`, exit 3 |

Level 4 forbids even user-space counting of one's own process, and a container cannot lower it. Consequence: card P5 (K at degree 4) needs the owner's own Linux host or bare metal; see [`../k_conversion_perf/LAUNCH.md`](../k_conversion_perf/LAUNCH.md).

Controls of the probe itself, on the authoring container: there it reports `unsupported` with ENOENT (no `cpu` event source: a negative control with a different cause). `probe.py --selftest` opens two software events through the same ctypes path and counts them (`selftest_ok`), which shows the path works.

Files: `probe.py`, `boot.sh` (the pod's boot: fetch the probe, run it, serve the JSON), and `result-20261018/` (`pmu.json`, `probe.exit`, `host.txt`, `probe.err` as fetched from the pod).
