#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Round24: early residual rejection
All 180 executions completed: 162 measured and 18 warmups. All 18 cells passed the frozen analyzer, with byte-identical full certificates across all six arms. All 379 worker-manifest entries verified before and after native export generation. No failed or missing executions, and no discarded timing rows.

Run: `hashed-ecdl-cpu-round24-20261030`, job UID `bbe5d31b-1533-4e07-b26f-f9a8598349c2`. Submitted 2026-11-03 17:39:58 UTC, completed 17:41:12 UTC: 74 seconds including provisioning. Container ran 17:40:57–17:41:02 UTC. One attempt, zero restarts/retries, 2 vCPUs / 4 GiB, one software thread, 300-second deadline, normal Spot autoscaling and unchanged one-node cap. CPU identity is retained in results/cpuinfo.txt.

== Complete recovery cost
Recorded medians in milliseconds, n=9 per cell. u/x are the original baseline and identical A/A control; v/w early residual rejection at batch64/batch1; y/z the suffix-only parent at batch64/batch1. No profiler clocks are enabled.
#table(columns: 7, inset: 4pt,
 [Fixture], [u], [x], [v], [y], [w], [z],
 [0], [.385321], [.384221], [.145871], [.144430], [.144040], [.142611],
 [1], [5.899407], [5.900647], [.903933], [.819213], [1.034953], [1.001373],
 [2], [6.214728], [6.215787], [1.142073], [1.031493], [1.372484], [1.319604],
)

Early rejection increases complete medians in all six matched cells. Derived increases, computed as `100*(candidate/parent-1)`, are 1.00%, 10.34%, 10.72% at batch64 and 1.00%, 3.35%, 4.01% at batch1. These are descriptive within-panel ratios, not confidence intervals. Fixture0 ranges overlap at both batch sizes; fixture1 batch1 also overlaps. Fixture1 batch64 and fixture2 both batches do not overlap. Baseline A/A ranges overlap in all fixtures. Page2 shows all recorded ranges.

The candidate checks both suffix parity bits after computing residual x. When both are absent it skips the final y multiplication and two subtractions. This retains exact witnesses but adds flag writes, a bitmap test, branches and scratch storage. The complete-cost result rejects this candidate for the measured fixtures; the faster suffix-only parent remains the basis for further CPU optimization. The panel does not separately attribute the loss to any one overhead.

== Workload and accounting
Case0: p211, a0, b7, r199, P=(3,33), m3/tau40/scalar42/seed111. Cases1/2: p65521, a65518, b3, r65563, P=(0,64894); case1 m3/tau80/scalar45678/seed222; case2 m4/tau64/scalar54321/seed333. Limits16 trials and1,000,000 work units. Six arms, three fixtures, one warmup and nine measured repetitions, frozen rotation/reversal. Known-answer positive checks and corrupted-scalar rejection passed.

Complete timing includes curve/target construction, setup and discarded trials, collection, reduction, both replays and serialization. It excludes process startup, transfer I/O and the negative check. No cross-process replay caches. No earlier timing rows are pooled. Shared-host L0 timing remains diagnostic.

#pagebreak()
= All arms: complete latency and ranges
#image("latency.svg", width: 100%)
Source: frozen round24/analyze.rs and results/rows.csv. Bars show recorded complete medians; lines show min–max, n=9. Run: hashed-ecdl-cpu-round24-20261030. Labels are defined on page1.

== Reproducibility and custody
Baseline source: `2bd4588fba603ba1787e1d8ad82f084ef461d7cf`. Early rejection: `b19d9b3beae66488c54ca50ec28b065e5683eccb`. Suffix-only parent: `4939261c9fcc48bf86f17ac36fa458f52a155b2e`. The previous/inverse binaries are identical copies selected with different batch flags. Frozen bundle hashes matched before upload. Exact submitted argv, image digest and limits are in submission-request.json; environment records remain with raw results.

Protocol: `HASHED-ECDL-CPU-RESIDUAL-FILTER-20261101`.
#text(size: 8pt)[Hash: `a6522d1d780ff9991aee96f386d1fe5234de77a9d31275b8681ee08b1186b321`.]
Pod UID: `cbbb292b-9af7-439c-b42d-7971b0f4f1ae`. The S3 archive is under the exact job's result prefix and this pod UID. Native export reran the frozen analyzer and checked manifests, row coverage and exact certificate equality. This is execution validation, not independent scientific review or full schema audit.

The owner's continuing budget authorization covers further CPU experiments. This round retained its frozen resource and retry limits. Initial scheduling waited for normal Spot provisioning; the UID1000 username warning was nonfatal. Historical preparation records remain unchanged; EXECUTION.md and execution-outcome.json supersede their operational status.

Reference basis: residual-filter/README.md, oracle-filter/README.md with Bloom's primary paper, round23 complete-cost evidence, and the frozen round24 protocol. Next: remove the regressing path while retaining this experiment, then test the next complete-cost candidate against the suffix-only parent.
