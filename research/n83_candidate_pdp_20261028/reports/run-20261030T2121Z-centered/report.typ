#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Centered panel: retained EKS scheduling outcome
Run n83-centered-panel-20261030-r1 · T-1008 · 30 October 2026

The 84-case source-bound centered panel was submitted once at 21:21:00 UTC. Its 1,800-second scheduling/execution deadline expired at 21:51:00 UTC; the Job was terminal Failed at 21:51:01 UTC. No arithmetic records were produced. The last observed Pod remained unscheduled, with no worker node assigned; the terminal response retained no Pod. Empty logs and an empty final S3 raw-prefix listing are preserved.

#image("pipeline.svg", width: 100%)

== Inputs, instrument and limits
The prepared panel comprises 42 retained prime n83 m3–9 cases and 42 explicitly synthetic centered/adjacent controls. Original/baseline/spike40 factor-base identities are preserved. Every prepared wrapper embeds its exact parent input/source and the validated effective source. Synthetic controls make no rational target-lift claim.

Source commit: `a27e8321d7f12657d98732a8d7ca972095fa3d56`.
Protocol hash: `be52f21c613f224137bdf7039525cf99e6309941081eb753fe2a8541e39b9b4f`.
Image digest: `sha256:dc4df3f5da5d33678682437ca26b8c54dbc47e82f4328c3ab9f7b4f9d3405780`.
Job UID: `9eefa95f-ad54-4cab-b836-432e8a987f37`.

The frozen request used 2 CPUs, 3 GiB pod memory, zero retries, two 600-second producer/replay phase limits, a 2 GiB address-space limit per phase and a 1,500-second worker cap. No worker timing or resource measurements exist. Preproduction native/container tests passed 19 checks; these remain preparation evidence, separate from the absent production output.

== Recorded result and interpretation
One Job submission covered 84 planned slots; 0 completed arithmetic records and 0 raw S3 objects were observed. The unified export retains one infrastructure-error row per planned slot, with null start/worker/time values and verification not_run. Its attempted/attempts fields count submitted slots, not started solver processes. The corrected export passes the protocol checker and file-hash verification; an initial instance-numbering failure is preserved in scratch/S3 support.

The scheduler reported node anti-affinity, selector and taint constraints. This establishes a scheduling failure, with no degree, projection, FFD or LFD result. A separately frozen existing-host fallback can reuse the identical prepared inputs and instrument after this terminal attempt; it must write fresh output and retain this attempt unchanged.

== Custody and file structure
See `evidence/centered-panel-r1/`, `experiment-centered-panel-r1.export.json`, `protocol-centered-panel.json` and `CENTERED_PANEL_RUN_FORMAT.json`. The evidence directory retains request/submission identities, pending and terminal status, empty logs/results, final raw-prefix listing and the explicit outcome record.

S3 run prefix: `s3://setec-astronomy-artifacts/runs/n83-centered-panel-20261030-r1/`. Preparation artifacts are distinct from raw results. No quantitative comparison graph is possible because no worker measurements were recorded; the diagram shows the verified execution boundary.
