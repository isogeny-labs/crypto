#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= L2 bootstrap qualification D: provisioning failure
#image("flow.svg", width: 100%)

== Requested validation
Job ecbench-periodic-tick-20261023-d retained the prior failed epoch's immutable crypto image and source, public 32-bit fixture, two targets, three arms, 600 rounds plus one warmup, order and seeds. It requested c8a.2xlarge (no SMT), four pod CPUs, 4 GiB memory and a 1,800-second deadline including provisioning. Its cloned worker bootstrap removed only nohz_full, retaining CPU/IRQ/workqueue/RCU placement. Zero measurement retries were allowed.

The required result remains 3,606 verified L2 observations, 3,600 matching native measured replays, an admitted 600-pair A/A comparison and 19 S3 payload hashes. That result was not obtained.

== Observed failure
The Job was submitted at 2026-10-23 00:50:14 UTC. AWS rejected nominated worker launches with MaxSpotInstanceCountExceeded. The shared account quota remained 96 vCPUs; other lab work occupied the available worker slot. The controller reported DeadlineExceeded at 01:20:14 UTC, then Failed at 01:20:15. The observer saw only Pending, with no assigned worker, Running phase or startup capture. Retained events contain no Scheduled event. Kubernetes removed the unassigned pod at the deadline; the bounded observer later ended without a terminal Pod capture.

No native benchmark started. L2, replay and A/A are unmeasured. Listing the run's S3 prefix found zero measurement payloads; this is not a successful S3 result. The frozen protocol was uploaded before submission and its version receipt is retained. The full test remains required, and the separate 2,000-child diagnostic C does not substitute for it. Earlier failed measurements were not regraded.

== Identity and retained evidence
job_uid: #raw("fd777dcb-91a1-4d62-93f5-eeb6762440ce")

pod_uid: #raw("840c1937-e0ea-461f-b87e-639b16ea160c")

The Job image, source/binary hashes, full spec and conditional protocol are in runs/protocol.json and runs/job.json. runs.tar.xz with its SHA-256 file retains those inputs, AWS/Kubernetes failure events, submission/terminal receipts, zero-object S3 listing and observer log. verification.json summarizes execution status. Expand the raw archive after verifying its hash.

No algorithm timing or speed result exists, so this report uses a process diagram. Algorithm leaderboards and curve graphs are unchanged. The v5 code and prepared launcher packages remain unpromoted pending full qualification. PR merge and the main-only GitHub Action still require their final review, CI and validation gates.
