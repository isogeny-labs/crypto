#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Round21: complete CPU recovery comparison
All 180 planned executions completed: 162 measured and 18 warmups. The frozen native analyzer accepted all 18 cells and byte-identical full certificates across arms for every fixture and repetition. All 379 worker-manifest entries verified before and after native export generation.

Job `hashed-ecdl-cpu-round21-20261030`, UID `3375af37-f645-44d8-b25d-0c63e1ae2bc5`, completed 2026-11-01 17:45:56 UTC. Spot c7a.2xlarge, AMD EPYC 9R14, one software thread, allocation 2 vCPUs / 4 GiB. The container ran 17:45:42–17:45:47 UTC; submission through completion took 74 seconds including provisioning. One attempt, zero retries, 300-second deadline, unchanged one-node pool cap.

== Recorded complete-time medians
All values below are recorded medians in milliseconds, n=9 per cell. The graph on page 2 includes full observed ranges. Ratios are descriptive arithmetic u/v within this shared-host L0 panel, not isolated acceptance evidence.
#table(columns: (1fr,1fr,1fr,1fr,1fr), inset: 5pt,
 [Fixture], [Baseline u], [Candidate v], [A/A x], [u/v],
 [0], [0.379011], [0.144050], [0.381202], [2.631x],
 [1], [5.735005], [0.834772], [5.846248], [6.870x],
 [2], [6.051117], [1.028556], [5.997787], [5.883x],
)

Arms u/x: original baseline `2bd4588fba60`; v/w: accumulated candidate `5ad00c65dc4f`; y: projective replay before compact pivots `2f08282ec3f5`; z: parent before projective replay `5b37f957e00d`. Batch64 except w (batch1). Every arm/fixture has one warmup and nine measured repetitions, with frozen interleaving and reversal. No rows were discarded.

The compact-pivot contrast y/v has complete-time median ratios 1.012x, 0.988x and 1.016x. It is mixed across fixtures, with overlapping observed ranges. The projective-replay contrast z/y has ratios 0.990x, 1.020x and 1.004x, also with overlapping ranges. These small descriptive differences do not establish a uniform constituent gain. Baseline A/A ranges overlap in all three fixtures.

Candidate case1 records oracle median 0.337561 ms and reduction/other 0.300590 ms; case2 records 0.452462 ms and 0.346647 ms. Relation collection and reduction are the next profiling targets. Phase medians are separate summaries and are not additive complete-time measurements.

== Workload and accounting
Case0: p211, a0, b7, order199, P=(3,33), m3, tau40, scalar42, seed111. Cases1/2: p65521, a65518, b3, order65563, P=(0,64894); case1 m3/tau80/scalar45678/seed222; case2 m4/tau64/scalar54321/seed333. Limits16 trials and1,000,000 work units. Known scalars construct public-point fixtures; positive checks, full replays and corrupted-scalar rejection remain mandatory.

The complete interval includes curve and target construction, setup and discarded trials, bases, collection, reduction, internal verification, independent replay and serialization. It excludes process startup, transfer I/O and the negative check. No cross-process replay caches are shared. No new build or local timing run was performed for this execution.

#pagebreak()
= Complete latency: all arms and fixtures
#image("latency.svg", width: 100%)
Source: frozen round21/analyze.rs and round21/results/rows.csv. Bars show complete-time medians; lines show full observed ranges, n=9. Run: hashed-ecdl-cpu-round21-20261030. Shared-host L0 diagnostics retain their measured scope.

== Custody and checks
The exact command, image digest and resource limits are in round21/submission-request.json. The five executable hashes and all staged manifests matched the frozen study before upload. Protocol hash:
#text(size: 8pt)[`e5723fc0a8377ec9d03637803fc2630b5cb5e8680efa9debda21a13a012db09f`].

The native exporter independently reran the frozen analyzer, checked all returned manifests and complete row coverage, and produced results/export.json. This is an execution export, not independent scientific review or a full schema audit. Worker records remain available under the job's S3 prefix and pod UID `6a6b8df3-1bfd-4a3b-a83a-620cf92def29`.

One status observation timed out; a later lookup of the same Job UID established success. The UID1000 username warning was nonfatal. No performance data are missing. The historical qualification and preparation README remain unchanged; EXECUTION.md and execution-outcome.json record the current outcome. No retries, changes to other jobs or pool expansion occurred.

Reference basis: the frozen protocol's EFD projective-coordinate and Rust collection references, plus projective-replay/README.md and pivot-directory/README.md. This report describes the run; isolated acceptance and independent evidence review remain separate steps.
