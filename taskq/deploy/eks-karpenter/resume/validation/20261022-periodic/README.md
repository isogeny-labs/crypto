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
(4,474 passed, 119 ignored) pass. Live qualification of that repair remains pending.

The missing-seccomp review allegation does not match the launcher source or the
saved EKS receipts: `isolab-criu-progress` installs the filter before exec and
both recorded F restored payloads show Seccomp 2. `existing-restored-credentials.txt`
contains that evidence; it was extracted from the frozen prior report archive.
The filter is for perf/CRIU compatibility, not a sandbox for untrusted code.

The local merge continuation also passed 15 guard tests after repairing its
legacy state label and preserving other reviewers' sections during status updates.
Its missing migrated systemd service was restored. Merge remains conditional on
exact-head CI and review; no full main-only Action result is claimed here.
