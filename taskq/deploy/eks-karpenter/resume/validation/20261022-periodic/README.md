# Periodic checkpoint deployment qualification

The periodic-intent repair at `052692885adba1e571b1f782e2fd26d37675b9ee`
was integrated into #2342 at `e05ccbc73fbfaa9fec5af6f32d60de1bdcb275f8`.
That source passed 18 native Linux x86 controller tests and was deployed with the
exact runtime/controller pins retained here. The initial source-only test archive
omitted the embedded `run-ecbench` script; its failed compile log is retained,
alongside the successful complete-source test.

[Attempt A](attempt-a/REPORT.md) then failed its first checkpoint because CRIU
captured an exited child together with the stopped root. It committed no snapshot
and launched no L2 epoch. Its raw public output, inspected PDF and private archive
receipts are retained. The repaired source keeps strict credential/stopped-process
admission and allows bounded, uncommitted recapture of the same computation.
Twenty controller tests, Clippy, three progress tests and the root library suite
(4,474 passed, 119 ignored) pass.

[Attempt B](attempt-b/REPORT.md) committed two successive encrypted snapshots,
restored generation 2 after worker loss and controller restart, preserved all
2,486 checkpointed records byte for byte, and completed 3,606 verified records
with 3,600 native replays. Both restored processes retained UID/GID 1000,
empty supplementary groups, zero capabilities, no_new_privs and Seccomp 2.
This verifies the periodic-intent repair on the deployed runtime. Both live
captures were admissible on their first capture; the zombie recapture branch
has native regression coverage, not a forced live success.

**B failed its frozen overall acceptance criteria.** Its one distinct fresh
epoch had 3,603 L2 and 3 L1 observations. Native replay passed, but the unchanged
L2 gate exited 42 before A/A evaluation; all 18 emitted S3 payload hashes matched.
The controller marked the logical run failed and did not retry measurement.
The source of the three excess-busy-tick observations is not established by the
retained counters. They are not discarded or reclassified. The runtime pin now
matches the deployed controller, with this narrower qualification recorded;
it does not claim an all-L2 pass for B. Earlier F remains separate positive
evidence for the fresh-epoch path. The main-only Action remains unverified.

The missing-seccomp review allegation does not match the launcher source or the
saved EKS receipts: `isolab-criu-progress` installs the filter before exec and
both recorded F restored payloads show Seccomp 2. `existing-restored-credentials.txt`
contains that evidence; it was extracted from the frozen prior report archive.
The filter is for perf/CRIU compatibility, not a sandbox for untrusted code.

The local merge continuation also passed 16 guard tests after repairing its
legacy state label and preserving other reviewers' sections during status updates.
It now waits for missing required checks. Its migrated systemd service was
restored. Merge remains conditional on
exact-head CI and review; no full main-only Action result is claimed here.
