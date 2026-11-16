# EKS CPU-accounting diagnostic

The controlled diagnostic recorded **7 excess-tick failures with full dynticks,
and 0 with the periodic tick**, retaining all 2,000 samples in each run. This
supports the v5 bootstrap workaround. It does not establish a precise kernel
defect or replace full crypto L2 qualification.

| Run | Placement and outcome | S3 payload hashes |
| --- | --- | --- |
| A | c8a requested; 600-second provisioning deadline expired before execution | No emitted payloads |
| B | c7i.2xlarge, full dynticks; 7 of 2,000 samples exceeded two excess ticks | 6/6 matched |
| C | c7i.2xlarge, periodic tick; 0 of 2,000 samples exceeded two excess ticks | 6/6 matched |

B and C used the same image, kernel, instance type and CPU placement, on different
physical workers. Each alternated 1,000 spin children with 1,000 children touching
20 MiB before a 20 ms spin. The observer and child shared SMT siblings 1 and 5;
these are counter diagnostics, not L2 benchmark results. B's seven flagged windows
reported 50 ms of busy time in approximately 25 ms of bracketed wall time, with no
IRQ, softirq or steal increase. C's actual full-dynticks mask was inactive.

[verification.json](verification.json) records complete source/image/pod/S3
identities. [Native summaries B](probe-b-summary.json) and
[C](probe-c-summary.json) retain the histograms and flagged records. The
[graph](histograms.svg), [rendering source](render.py) and
[visually inspected PDF](report.pdf) describe the observations and limits.

Expand the byte-preserving raw archive with `tar -xJf runs.tar.xz` after checking
`sha256sum -c runs.tar.xz.sha256`. It contains every emitted B/C payload and
manifest, protocols, Job/Pod receipts, and A's provisioning failure. A's original
protocol timestamp had a transcription error; its retained erratum records file
and submission times rather than rewriting that protocol. No old benchmark was
regraded, no sample discarded and no L2 threshold changed.

The separately frozen 600-round c8a qualification D is queued for shared Spot
capacity at report time. It must independently pass 3,606 L2 records, 3,600 native
replays, 600 A/A pairs and 19 S3 hashes. Exact-head review/CI and the main-only
GitHub Action are also still pending. Earlier CRIU restore evidence remains
separate. Algorithm leaderboards and curve graphs are unchanged because this
report makes no algorithm performance or security claim.

Primary references: [Linux NO_HZ documentation](https://docs.kernel.org/timers/no_hz.html),
[Linux v6.18 CPU accounting](https://github.com/torvalds/linux/blob/v6.18/kernel/sched/cputime.c),
[/proc/stat producer](https://github.com/torvalds/linux/blob/v6.18/fs/proc/stat.c).
These explain the separate accounting paths; they do not prove a specific defect
in the tested Amazon Linux kernel build.
