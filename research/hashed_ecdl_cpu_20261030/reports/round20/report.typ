#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Round20: no placement before the deadline
The single authorized CPU panel job expired without a node assignment. None of its 180 planned executions started. Worker logs were empty, and the terminal S3 listing contained no result objects. The frozen missing-cell rule invalidates the panel; no success export or timing result was created. No retry or deadline extension occurred.

#image("pipeline.svg", width: 100%)
The diagram shows the recorded operational boundary, not benchmark latency. There are no comparable trial values to plot. Sources: round20/submission.json, initial-status.json, terminal-status.json, terminal-logs.json and terminal-s3-list.json. Raw records were published in commit 283d163366a3 before this report.

== Attempt and resources
Job `hashed-ecdl-cpu-round20-20261030`; Job UID `79973c63-7787-4804-b55e-63374a15bf2b`. Observed Pod UID `991e9449-9982-46f8-a9ce-2db9bdfa58bc`. Submitted 2026-11-01 03:50:31 UTC; DeadlineExceeded at 03:55:31; terminal Failed at 03:55:32. Requested 2 vCPUs, 4 GiB, one software thread, a 300-second deadline and zero retries. The 300 seconds is scheduling elapsed time, not measured worker CPU time. Host model, algorithm timings and CPU usage are missing.

The owner explicitly allowed normal Spot-pool autoscaling after the pool was observed empty. During the attempt Kubernetes reported 0/3 nodes eligible: one conflicted with pod anti-affinity and two with the required node selector. A separate job occupied the observed worker. These receipts do not establish why a second eligible worker was not provisioned or attribute account-level billing. The next infrastructure task is to inspect autoscaler events and capacity constraints before admitting another panel.

== Frozen workload and controls
Protocol `HASHED-ECDL-CPU-STREAM-ENDPOINTS-20261030`; hash:
#text(size: 8pt)[`2b263251e9c81fb7f7da12e4a3387e9f1f5c44529f5977a49a90553904017b8b`].

Six arms retain the original baseline u/x at source `2bd4588fba60` (identical A/A executables), accumulated candidate v/w at `fab61dde9ada` (batch64/batch1), parent y at `a6117b9ec774` (before endpoint-product removal), and parent z at `5cb7a4072078` (before streamed matrix staging). All are integrated source revisions; the binary basename inverse is only the label for z. y/v isolates endpoints; z/y isolates matrix staging. Round21's later replay and pivot candidates were not substituted.

Every arm/fixture cell planned one warmup and nine measured repetitions: 18 warmups and 162 measured executions. All are missing. No positive, negative or A/A control executed; no full certificate was produced or verified by the worker.

Fixture 0: p=211, a=0, b=7, order 199, P=(3,33), m=3, tau40, scalar42, seed111. Fixtures 1/2: p=65521, a=65518, b=3, order65563, P=(0,64894); case1 uses m3/tau80/scalar45678/seed222, case2 m4/tau64/scalar54321/seed333. Limits remain 16 trials and 1,000,000 work units.

== Accounting and custody
The intended complete interval includes curve/target construction, all setup and discarded trials, factor bases, relation collection, reduction, internal verification, independent full replay and serialization. Process launch, transfer I/O and the corrupted-scalar negative check are excluded. A completed panel would still be L0 diagnostic, not isolated 10x acceptance evidence.

The exact shell command, digest-pinned runtime image and limits are in submission-request.json. Five binary hashes and the unchanged frozen protocol were verified before upload; staged-input.sha256 binds the input files. Native exporter rebuild and 12 synthetic tests passed locally, which does not constitute a worker run. Historical preparation receipts remain unchanged and are superseded operationally by execution-admission.json and execution-outcome.json. The consumed attempt cannot be silently resumed or repeated; round21 remains separately unadmitted.
