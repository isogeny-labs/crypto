#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(leading: 0.5em)
= Periodic-tick L2 qualification E: failed
*The unchanged full crypto workload ran, but 48 observations failed the native preemption gate. The candidate is not promoted.*

#image("grades.svg", width: 100%)

== Native outcomes
All 3,606 computations verified and all 3,600 measured native replays matched. Grades were 3,558 L2 and 48 L1. Every L1 record named involuntary preemptions as its only L2 blocker; none failed excess-busy-tick or steal checks. The threshold remains 50 involuntary preemptions per second. Gate exit 42 prevented the A/A step. All 18 emitted payload hashes matched a full S3 download; the prefix contains 19 objects including the manifest. This is a failed qualification, not an admitted timing comparison.

== Frozen workload and placement
Public 32-bit prime_search seed 59297; two targets, target seed 20261002; rho.negation reference, rho.plain baseline and rho.negation control. Exactly 600 rounds, one warmup, alternate order, seed 1 and 120-second per-child timeout. Four exclusive Pod CPUs, 4 GiB, 1,800 seconds including provisioning, zero retries. The c8a.2xlarge worker had eight cores with one thread per core. Boot readback completed during the fixed 60-second setup, before measurement: nohz_full inactive, managed IRQ/RCU/workqueue placement retained. Native image, source, executable and spec identities matched the frozen protocol.

== Interpretation limits and next diagnosis
The periodic-tick change avoided the earlier accounting failure in this attempt but did not establish reliable L2. Post-run inspection showed RCU threads on worker CPUs. Their causal role requires a separate scheduler trace; the kernel documentation requires affinity for offloaded RCU threads. No production grader or threshold changed. The prior D attempt remains a provisioning failure without a container start. E remains a complete failed measurement; it is not selectively retried. No new CRIU restore or main-only GitHub Action is claimed.

== Identity and complete evidence
Job: #raw("ecbench-periodic-tick-20261023-e")

Job UID: #raw("19510979-be9c-431b-b99f-88fb200485f1")

Pod UID: #raw("b30bd223-b6f7-427a-baf7-857503feef6d")

Crypto source: #raw("cb2ec6873f30b683f7885b53c410d69965f02c20")

Binary SHA-256: #raw("583e62f0836eae43330c4b36bbe3d145dacb01de3132436c71f6924445c6a5a9")

Instance: #raw("i-0dd644e0372e4a9dc"); image digest: #raw("sha256:67eea58f5d4a7a8882b55f089c6063bb49fad58d40260ee9b0ceb349fc0e4581").

The complete S3 prefix and worker identities are in verification.json. runs.tar.xz plus its SHA-256 sidecar retain the frozen inputs, every native payload, receipts and hash checks. The initial observer mistakenly treated a paginated S3 listing without KeyCount as empty; the original and explicit correction are retained. The later SSM host listing is partial (24,000-byte API output limit); native artifacts are complete. Graph source grades.svg copies only native grade counts. No algorithm leaderboard or canonical curve graph changed.

Source: https://docs.kernel.org/admin-guide/kernel-per-CPU-kthreads.html (offloaded RCU thread affinity). Full report: REPORT.md; source bootstrap: ../../../nodepool.yaml. Final-head CI/review and a passing fresh qualification remain required before merge/promotion.
