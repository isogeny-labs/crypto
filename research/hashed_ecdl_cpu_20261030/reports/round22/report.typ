#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Round22: CPU stage profile
All 180 executions completed: 162 measured and 18 warmups. The frozen analyzer accepted all 18 cells, all 60 stage records and byte-identical full certificates. All 500 worker-manifest entries verified before and after native export generation. No data were dropped.

Run: `hashed-ecdl-cpu-round22-20261030`. Job UID: `eaab9dfe-dd18-4e86-8949-8dcc8e518628`. Submission to completion: 2026-11-01 22:35:05–22:36:19 UTC (74 seconds including provisioning); container: 22:36:03–22:36:09 UTC. One attempt, 2 vCPUs / 4 GiB, one software thread, 300-second deadline, zero retries. Normal Spot autoscaling, unchanged one-node cap. No new local benchmark or build.

== Recorded complete-time medians (ms)
Nine measured repetitions per cell; ranges appear in the chart on page 2. u/x are the original baseline A/A pair; v/w ordinary candidate batch64/batch1; y/z the identical candidate with stage clocks, batch64/batch1.
#table(columns: 7, inset: 4pt,
 [Fixture], [u], [x], [v], [w], [y], [z],
 [0], [.385971], [.385520], [.144650], [.146401], [.153320], [.154021],
 [1], [5.910832], [5.922442], [.841131], [1.030612], [.873232], [1.060822],
 [2], [6.241442], [6.231783], [1.058703], [1.350282], [1.086193], [1.382443],
)

== Recorded profiled batch64 stages (ms)
These are separate medians, n=9; they are not an exhaustive partition and must not be summed. Oracle is the largest individual recorded stage in fixtures 1/2. Targets includes setup and accepted-relation checks; linear includes matrix construction, dependency and extraction.
#table(columns: 4, inset: 4pt,
 [Stage], [Fixture 0], [Fixture 1], [Fixture 2],
 [Bases], [.009849], [.032200], [.034770],
 [Targets/checks], [.017436], [.091913], [.089270],
 [Oracle], [.031210], [.360492], [.483973],
 [Linear], [.018710], [.106291], [.144570],
 [Internal replay], [.019409], [.101860], [.118600],
 [External replay], [.015480], [.091030], [.106840],
)

The ordinary controls retain full cost; y/v and z/w expose instrumentation overhead. These are shared-host L0 and L0-profile diagnostics. The next source investigation is oracle preparation versus residual probing, preserving full witnesses and both replays. This panel does not establish tenfold acceptance.

== Workload and identity
Case0: p211, a0, b7, r199, P=(3,33), m3, tau40, scalar42, seed111. Cases1/2: p65521, a65518, b3, r65563, P=(0,64894); case1 m3/tau80/scalar45678/seed222; case2 m4/tau64/scalar54321/seed333. Limits: 16 trials, 1,000,000 work units. Six arms, three fixtures, one warmup plus nine measured repetitions, frozen rotating/reversed order. Corrupted-scalar rejection and full replay remain mandatory.

#pagebreak()
= Complete latency and artifact custody
#image("latency.svg", width: 100%)
Source: round22/analyze.rs and results/rows.csv. Bars: medians; lines: observed min–max, n=9. Run: hashed-ecdl-cpu-round22-20261030. All arms retained.

Baseline source: `2bd4588fba603ba1787e1d8ad82f084ef461d7cf`. Candidate source: `972d0d2fed4d1a40ec1a0b4bd2b504e76ec98942`. candidate/previous/inverse are byte-identical executable copies, not different algorithm revisions. Frozen source/build and executable manifests are in round22/. The execution command and image digest are in submission-request.json; environment and CPU identity are in results/cpuinfo.txt and kernel.txt.

Protocol: `HASHED-ECDL-CPU-STAGE-PROFILE-20261101`.
#text(size: 8pt)[Hash: `88c80abe26f704055292a5b8bfade5baa91a7f86167f4f4a5222d1817fafc1de`.]
Original worker outputs are retained under the job's S3 result prefix and pod UID `ab82b990-2fbd-43ee-a748-49e97444609a`. The native exporter reran the frozen analyzer, checked row/stage binding and certificate coverage, and produced results/export.json. This is execution validation, not an independent scientific review or full schema audit.

Complete timing includes curve/target construction, setup/discarded trials, collection, reduction, internal replay, external replay and serialization. It excludes process startup, transfer I/O and the negative check. No cross-process replay caches are shared.

One status observation timed out; the same UID later established success. The missing username warning for UID1000 was nonfatal. No retry, deadline extension or pool changes occurred. The single authorization is consumed. Historical preparation receipts remain unchanged; EXECUTION.md and execution-outcome.json record completion.

Reference basis: the frozen protocol, stage-profile/README.md, round21 evidence, and the shared ReductionStats/solve_reduction implementation. No new arithmetic or novelty claim is made. Stage-summary.csv retains all min/median/max values for both profiled arms.

#pagebreak()
= Stage detail: profiled batch64
#image("stages.svg", width: 100%)
Recorded medians and observed ranges, nine measured repetitions per stage/fixture. Source: stage-summary.csv, generated from all validated profile rows by the frozen analyzer. This chart is rendered by stages.awk; no additional sampling or reruns. Run: hashed-ecdl-cpu-round22-20261030. Profiling overhead is retained; the ordinary controls on page 1 show complete recovery without these extra clocks.
