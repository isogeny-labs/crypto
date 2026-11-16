# EC2 Tier 1 full-worker panel

This panel compares the complete secp256k1 rho worker's serial and
32-wide batch-inversion paths on 32 frozen public Q points. Each arm runs
128 walkers with `dp_bits=12` and an 8,192-step cap. The plan schedules
384 alternating same-Q pairs, including serial/serial and batch/batch A/A
controls, with four repetitions. Its charge is Q-dependent setup plus the
complete worker lane. Independent distinguished-point replay and exact
ordered check-in comparison accompany every pair.

The initial plan sets `batch_width=32` and `checkin_every=8`. The worker
flushes at each check-in, so an actual simultaneous batch contains at most
eight walkers. A separate `checkin_every=64` panel is needed to measure
the full requested width under the production cadence; it will have its own
frozen source, plan, observer, and audit.

The fresh `c8i.8xlarge` EC2 allocation is instance
`i-04455af916e7aefe4`, launched on 2026-10-18 in `us-east-2a`.
[AWS allocation](ec2-instance.json) exposes 16 guest vCPUs on 16 configured
cores with SMT disabled. Guest CPU 15 has `nohz_full` and RCU callback diversion; system
and user slices are restricted to CPUs 0–14, and all 72 inspected guest IRQs
exclude CPU 15. The benchmark child alone is pinned to CPU 15. The VM has
one exposed NUMA node; CPU frequency controls are unavailable in the guest.
The [allocation receipt](allocation-r2.json) captures those facts and the
source and binary hashes. The host's physical CPU and IRQ ownership outside
the VM cannot be established here, so this is **Tier 1 cloud-instance**
evidence. The systemd shutdown timer is set to 2026-10-19 03:15 UTC, and
[AWS shutdown behavior](shutdown-behavior.json) is `terminate`.

The first dispatch used [plan](plan.json) SHA-256
`385d94d1a56523d9f8fdac88a623b4cd9cd70adfe76f164f6c1ef2d9ceeebb67`.
Its process observer passed the five-second quiet window, but the runner
stopped at preflight with zero timed arms. On this VM, `cpu.max` and
`memory.max` are exposed in the systemd scope, while the runner read the
cgroup mount root. The [complete preflight receipt](attempt1-preflight.tar.gz)
is SHA-256 `676215b5c493ad4413ec6a6369a501925d3a3f8a649814f7c627d94978efbe4c`.

The corrected runner reads the current cgroup v2 scope and records its
`cpu.stat` and `memory.events` deltas. The second scope enables CPU and
memory accounting and declares a 16-core quota (`CPUQuota=1600%`), exposing
`cpu.max` without constraining the one-core child. The replacement source
archive was generated twice with identical SHA-256
`2a0a7b6e04c8592f2d2be5c58ff7f226b9f39edf2a8d45b8d27afd04bcecf20c`.
The [second plan](plan-r2.json) is SHA-256
`694468acea0068e0c6d1a3a58301b1dc213ae62c73bb36e61f39542fcf94792b`;
the [second allocation](allocation-r2.json) is SHA-256
`19ac8cda5d1af7ecd8298b26d9e4a55b6dad379c7ef35e0d063abfab2610db1d`.
The second [launch procedure](launch-r2.sh) was committed before dispatch.
Its preflight passed. The complete [raw archive](attempt2-complete.tar.gz)
contains 768 arms, 384 pairs, all stdout/stderr, the source bundle and
Linux executable, build log, allocation, observer, barrier, and runner
receipts. Its SHA-256 is
`577c6b6c4a8ee78f95eebc6a91a1b4d583907f58bd394cd3329000343bccd4d9`;
the transferred copy matched the VM hash and its 1,565 members were checked.
The end-state [allocation receipt](allocation-r2-end.json) again found no
guest IRQ affinity on CPU 15.

The [independent audit](audit-r2.json) found no structural or peak-memory
failures and 31 valid Q clusters. One arm recorded CPU steal time and was
marked invalid (arm 553, `q28-comparison`, serial, repetition 2). Four A/A
ratios fell outside the frozen `[0.90, 1.11]` interval:

| Case | Repetition | Reference/candidate ratio |
| --- | ---: | ---: |
| `q18-aa_batch` | 1 | 1.2771 |
| `q28-aa_serial` | 2 | 1.1720 |
| `q26-aa_batch` | 3 | 0.8620 |
| `q29-aa_batch` | 3 | 1.3502 |

The process-affinity observer completed with `passed`; it did not see a
guest user-space task eligible on CPU 15 during measured work. The runner
reported `completed` for all scheduled arms, with 383 valid exact-output
pairs and one pair containing the CPU-steal arm. The independent decision is
`unknown_gate_failed`, with the aggregate ratio and interval null. The
diagnostic medians for valid arms were 10.576 seconds serial and 3.836
seconds batch; the batch peak RSS median was 5,556 KiB versus 5,560 KiB
serial. Those medians do not select default routing. The next comparison
uses the physical-host isolation gate and `checkin_every=64` to exercise
the full requested batch width.
