#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Full-cache query: retained preflight failure
Run fullcache-20261101-v1 · 2026-11-01 04:00 UTC

The worker stopped before any recovery cells or candidate CUDA compilation.
Cargo rejected the standalone driver's stale lockfile after the merged
hashed-ecdl-tensor manifest added sha2. The final archive was collected and
hash-checked; deletion of the owned RTX 4090 was verified.

#image("pipeline.svg", width: 100%)

The diagram replaces a timing plot because no comparable recovery values were
produced. Requested: 36 cells; attempted: 0; completed: 0. The preflight process
exited 2 (Cargo 101), and the controller exited 1. Certificate and negative-control
outcomes are unavailable because those stages did not execute.

== Frozen scope and source
Two public inputs at each of 52 and 63 bits, seeds 101 and 202. Baseline b and
A/A control x use the same generic affine query binary; candidate v uses the
uninstrumented full-cache specialization. Five rounds per small input and one
per large input, alternating b/v/x and x/v/b. All timing would be L0 diagnostic.

Source: #raw("736dd1f4fff3451115772f08b1e593a6070b9440").
The source checkout was clean. Command:
#raw("bash research/explicit_relations_fullcache_20261101/run.sh").
The worker ran on one RunPod RTX 4090 in EUR-IS-2 with the pinned CUDA 13.2
image. Worker budget: 18,000 seconds; controller deadline: 18,300 seconds.

== Failure custody and follow-up
Pod: #raw("ncbo90ya2qm4oy"). Public receipts and raw logs:
#raw("../../results-v1/{failure.json,bootstrap.log,receipt.json,deletion.txt}").
Full raw archive: #raw("s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/fullcache-20261101-v1/results.tgz").
SHA-256:
#raw("f329552821caa3a845e885359d3340bf7b5358f3d45095309f1d210ed0ff22bd").

The standalone lockfile was repaired and locked offline metadata resolves.
Version 2 preserves inputs, kernels, controls and analysis. A conservative
180-second charge for this failed allocation reduces the replacement worker
budget to 17,820 seconds. The repair's GPU build and the recovery measurements
remain pending at this report's filing. No performance result is inferred.
