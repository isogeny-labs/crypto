#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Round23: exact suffix-filter comparison
All 180 executions completed: 162 measured and 18 warmups. All 18 cells passed the frozen analyzer, with byte-identical full certificates across all six arms. All 379 worker-manifest entries verified before and after native export generation. No failed or missing executions, and no discarded timing rows.

Run: `hashed-ecdl-cpu-round23-20261030`, job UID `12487b10-22bd-4fe8-b193-48c40c6308de`. Submitted 2026-11-01 22:54:38 UTC, completed 22:55:50 UTC: 72 seconds including provisioning. Container ran 22:55:37–22:55:41 UTC. One attempt, zero restarts/retries, 2 vCPUs / 4 GiB, one software thread, 300-second deadline, normal Spot autoscaling and unchanged one-node cap. CPU model is retained in results/cpuinfo.txt.

== Complete recovery cost
Recorded medians in milliseconds, n=9 per cell. u/x are the original baseline and identical A/A control; v/w filter-on at batch64/batch1; y/z filter-off at batch64/batch1. No profiler clocks are enabled.
#table(columns: 7, inset: 4pt,
 [Fixture], [u], [x], [v], [y], [w], [z],
 [0], [.366971], [.366961], [.137502], [.143520], [.138250], [.140741],
 [1], [5.597612], [5.618332], [.780351], [.801562], [.944252], [.969892],
 [2], [5.905762], [5.906483], [.976442], [1.006581], [1.248473], [1.280814],
)

Filter-on medians are lower in every matched cell. Derived reductions, computed as `100*(1-on/off)`, are 4.19%, 2.65%, 2.99% at batch64 and 1.77%, 2.64%, 2.53% at batch1. These are descriptive within-panel ratios, not confidence intervals. The observed ranges overlap for fixture0 at both batch sizes; they do not overlap for fixtures1/2 in either matched filter comparison. Baseline A/A ranges overlap in all fixtures. Page2 shows all recorded ranges.

The filter adds a packed low-key occupancy bitmap before exact suffix lookup. A clear bit rejects without full hash mixing/probing; a set bit still requires the original exact lookup. First-witness order, point checks, internal replay and external replay are preserved. Bitmap preparation and retained memory are charged. The source delta, build identities and manifest hashes are retained in round23/.

== Workload and accounting
Case0: p211, a0, b7, r199, P=(3,33), m3/tau40/scalar42/seed111. Cases1/2: p65521, a65518, b3, r65563, P=(0,64894); case1 m3/tau80/scalar45678/seed222; case2 m4/tau64/scalar54321/seed333. Limits16 trials and1,000,000 work units. Six arms, three fixtures, one warmup and nine measured repetitions, frozen rotation/reversal. Known-answer positive checks and corrupted-scalar rejection passed.

Complete timing includes curve/target construction, setup and discarded trials, collection, reduction, both replays and serialization. It excludes process startup, transfer I/O and the negative check. No cross-process replay caches. No data from earlier rounds are pooled. Shared-host L0 timing remains diagnostic; the tenfold objective requires separate acceptance evidence.

#pagebreak()
= All arms: complete latency and ranges
#image("latency.svg", width: 100%)
Source: frozen round23/analyze.rs and results/rows.csv. Bars show recorded complete medians; lines show min–max, n=9. Run: hashed-ecdl-cpu-round23-20261030. Labels are defined on page1.

== Reproducibility and custody
Baseline source: `2bd4588fba603ba1787e1d8ad82f084ef461d7cf`. Filter-on: `4939261c9fcc48bf86f17ac36fa458f52a155b2e`. Filter-off: `972d0d2fed4d1a40ec1a0b4bd2b504e76ec98942`. The filter-off previous/inverse binaries are identical copies selected with different batch flags. Frozen bundle hashes matched before upload. Exact command, image digest and limits are in submission-request.json; environment records remain with raw results.

Protocol: `HASHED-ECDL-CPU-SUFFIX-FILTER-20261101`.
#text(size: 8pt)[Hash: `193353565f8af3d686b57efb44912874a4e0f595bd6379a998c8f49a53e982c7`.]
Pod UID: `34f89be0-5bd1-4f9b-aae3-92d4048c7ba7`. The S3 archive is under the exact job's result prefix and this pod UID. Native export reran the frozen analyzer and checked manifests, row coverage and exact certificate equality. This is execution validation, not independent scientific review or full schema audit.

Initial scheduling waited for normal Spot provisioning. The UID1000 username warning was nonfatal. No retries, deadline extension, cap changes or missing performance data. The single authorization is consumed. Historical preparation records remain unchanged; EXECUTION.md and execution-outcome.json supersede their operational status.

Reference basis: oracle-filter/README.md with Bloom's primary paper, round22 stage evidence, and the frozen round23 protocol. The recorded improvement directs further CPU work while retaining complete-cost accounting and exact witnesses.
