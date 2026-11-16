#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(leading: 0.5em)
= EKS CPU accounting and L2 bootstrap repair
*Observed: the controlled probe removed the excess-tick failures. Full crypto L2 qualification remains pending.*

#image("histograms.svg", width: 100%)

== Scope and retained failures
The prior periodic-recovery qualification B remains failed: 3,603 L2 and three L1 observations, native replay correct, A/A not evaluated. This report does not regrade it. Probe A requested c8a with four CPUs but expired before a container started: provisioning consumed its 600-second deadline. Its original protocol timestamp contained a 23-second transcription error; the retained erratum records that the actual file and job manifest preceded submission. A supplied no measurement.

== Controlled diagnostic B and C
Each probe executed 1,000 20 ms spin children alternating with 1,000 children touching 20 MiB before the same spin. Native code recorded every raw per-component /proc/stat delta, monotonic timestamps bracketing both reads, wait4 user/system time, faults and context switches. Both used c7i.2xlarge, the same immutable image and AL2023 6.18.54 kernel. Observer CPU 1 and child CPU 5 were SMT siblings in a two-vCPU allocation; these probes do not establish L2 or algorithm performance.

B, with nohz_full active, recorded seven faulting children with 50 ms reported busy time inside approximately 25 ms wall windows: four user ticks and one system tick, with no IRQ, softirq or steal increment. C removed only nohz_full from the cloned bootstrap and retained CPU/IRQ/workqueue/RCU placement. All 2,000 C samples completed; none exceeded two excess ticks, and all 1,000 spin samples had zero excess ticks. Actual nohz_full read back inactive. These observations support the boot-configuration workaround; they do not identify a precise upstream kernel defect or establish every instance family's behavior.

== Qualification still required
The separate 600-round c8a protocol was frozen before submission and is queued for shared Spot capacity at report time. It must pass all 3,606 L2 observations, 3,600 native replays, 600 A/A pairs and 19 S3 payload hashes before the bootstrap is promoted. This diagnostic does not substitute for that test, a new CRIU recovery run, or the requested main-only GitHub Action.

== Identity and evidence
Probe image digest: #raw("sha256:dc7b9d38d5243a180243f8275cf11ca2fde2fde347f078917698ef8f9bba1989")

Probe source commit: #raw("000f1ecd043b4913a59476d2f15b8abef9c606e7")

Baseline B Pod UID: #raw("cbb7805a-60d5-4bc4-adbf-b7dadc4979b9")

Changed-boot C Pod UID: #raw("2b8c879a-a271-41ff-9566-f93ecc34986a")

Changed-boot C instance: #raw("i-0865613b4a5a49962")

The checked-in verification.json contains complete image, worker and S3 identities. runs.tar.xz and its SHA-256 manifest retain protocols, submission/terminal receipts and every emitted payload from B and C plus A's no-execution receipts. Expand it beside this report to inspect runs/; the original S3 prefixes are also retained. Probe source is ../../cpu-accounting/; native production grading was not changed.

Profile v5 removes nohz_full, rejects active full-dynticks CPUs before kubelet starts, and preserves static whole-core CPU reservations, housekeeping affinity, managed IRQs, RCU callbacks, workqueue routing and normal scheduler load balancing. The two submission packages change only submit-job and its integrity manifest. Temporary diagnostic capacity was reserved from the existing 64-vCPU lab cap; the regional quota remained 96. Existing independent workers were not interrupted.

The GitHub conflict resolution retained both problem-index additions. The CodeQL merge reconciliation analyzes the same complete Rust database once per suite, retaining full source coverage and exact bundle versions. Local validation passed 4,487 library tests, 18 budget tests, all 444 workflow budget checks, actionlint, request/build/publisher contracts, bootstrap syntax and positive/negative full-dynticks guard checks. The full L2 qualification, final-head review/CI and the main-only Action remain separate gates.

Algorithm leaderboards and curve diagrams were checked for scope and left unchanged: this is infrastructure and measurement-validity evidence. Source references: Linux kernel documentation, timers/no_hz.html; Linux v6.18 kernel/sched/cputime.c and fs/proc/stat.c. Those references explain the separate accounting path, not a proved root cause for this kernel build.
