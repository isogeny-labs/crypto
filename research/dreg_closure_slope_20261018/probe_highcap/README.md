# Probes after the registered study: the engines at degree 5 on m3 (19, 7) draw 0 (N = 40)

Not registered; reported for completeness. Neither enters the slope statistic.

| probe | command | outcome |
|---|---|---|
| L3, sparse closure with the entry cap raised | `dreg_closure --sparse 5 --max-entries 2500000000`, address space 13.5 GB, 3-hour limit, started 14:27 UTC | killed by the memory cgroup at 16:27 UTC, 7.2 GB RSS (`dmesg`: "Memory cgroup out of memory: Killed process 208 (dreg_closure) total-vm:7165576kB, anon-rss:7157228kB"), exit 137 after about 2 h and 92 CPU-minutes, while the dense probe below held 3.5 GB in the same cgroup. No record written (`m3-19-7.d0.D5.jsonl` is empty): unmeasured. |
| L2 test, dense closure scanning to degree 5 | `dreg_closure --to 5 --max-cols 800000` (degree-5 XL rows are only `42 × C(40, ≤2) = 34,482` over 760 k columns), same limits | see `m3-19-7.d0.dense-to5.{jsonl,log}` |

Reading of L3: with 4-byte ranks the 350 M cap of the registered study was 1.4 GB, but the closure's fill-in past that point grows faster than the entry count suggests; at 2.5 G entries the process reached the cgroup limit before either refuting or capping. Degree 5 at 40 unknowns therefore needs more than about 7 GB for this engine too, consistent with the F4 probe's 12.6 GB kill.
