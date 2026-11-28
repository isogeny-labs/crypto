#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Round19: bounded CPU job scheduling timeout
The single authorized job expired without an assigned node. None of the 180 planned benchmark executions started; no timing rows, full certificates or worker logs were returned. The frozen deadline/missing-cell rule invalidates this panel. No retry or deadline extension was submitted.

#image("pipeline.svg", width: 100%)
This diagram represents the recorded execution boundary. There are no measured trial values to plot. Source: round19 submission.json, initial-status.json, terminal-status.json, terminal-logs.json and terminal-s3-list.json.

== Attempt and resource boundary
Job `hashed-ecdl-cpu-round19-20261030`; UID `e8ae4f11-53d1-4994-b2c0-a858eafc2417`. Submitted 2026-10-30 21:32:54 UTC; DeadlineExceeded at 21:37:54; terminal Failed at 21:37:55. Requested 2 vCPUs and 4 GiB, one software thread, 300-second deadline, zero retries. The 300 seconds is scheduling elapsed time, not CPU time. No worker CPU usage or host model was measured.

Kubernetes reported zero of five existing nodes eligible: one conflicted with pod anti-affinity, two with node affinity/selector, and two with untolerated taints. The observed pod had no node assignment. At the terminal readback the pod list was empty; bounded logs were empty and the result prefix contained no objects. No broader account cost or billing saving is inferred from this job status.

== Frozen workload and controls
Protocol `HASHED-ECDL-CPU-RELATION-ARITHMETIC-20261030`; SHA-256 `3a24606b951d7da114f0d933876253c6625e8cb57dfd977a31a6dbafac2ca8e4`.

Six arms: u/x original baseline `2bd4588fba60` and its identical A/A duplicate; v/w integrated candidate `5cb7a4072078` at batch sizes 64/1; y matched parent `fd4b1ec078fa`; z the same parent with only the cached-affine FixedPair branches disabled by a retained patch. Arm z is explicitly non-integrated. Each arm/fixture cell planned one warmup and nine measured repetitions: 18 warmups and 162 measured executions. None ran; no control passed or failed at the algorithm level.

Fixture 0: p=211, a=0, b=7, subgroup order 199, P=(3,33), m=3, tau=40, scalar=42, seed=111. Fixtures 1/2: p=65521, a=65518, b=3, subgroup order 65563, P=(0,64894); fixture 1 m=3, tau=80, scalar=45678, seed=222; fixture 2 m=4, tau=64, scalar=54321, seed=333. Limits: 16 trials and 1,000,000 work units. Known scalars construct the public-point fixtures.

The intended complete timer charges curve/target construction, setup, discarded trials, factor bases, query generation, reduction, internal verification, independent replay and certificate serialization. Process launch, artifact I/O and the corrupted-scalar negative check remain outside that timer. The panel is L0 diagnostic even when complete.

== Reproducibility and next dependency
The exact shell argv, digest-pinned runtime image and resource envelope are in round19/submission-request.json. Frozen run.sh was unchanged and would receive EPHEMERAL_OUT=/work/results. AWS CLI transfer used the existing scoped job role; no Python research harness was invoked. All five local binary SHA-256 checks passed before staging; staged-input.sha256 binds the uploaded bundle. No success export was manufactured.

The software and lockfile head at submission was `e63c1f5719a4b72745dab2ffd8ba8ba2e6143ebf`; submission receipts were subsequently added as evidence. T-1019 owns this bounded execution/report. The existing cumulative-improvement goal remains active. The next execution depends on an eligible node and a separately admitted attempt; this attempt is terminal and will not be resumed or silently repeated. Existing round18 timings are not reused as measurements of these candidate changes.
