# Periodic checkpoint qualification A — failed

Run `ecbench-periodic-20261022-a` used controller source
`e05ccbc73fbfaa9fec5af6f32d60de1bdcb275f8` and the runtime pins in `protocol.json`.
The frozen public 32-bit fixture requested 600 rounds, one warmup, two targets,
three arms, 30-second checkpoints and a 3,600-second absolute deadline. This is
infrastructure correctness validation, not an algorithm performance comparison.

CRIU returned exit 0 on its first capture. Its log records root PID 531 stopped
and child 5463 as a zombie (`Obtaining zombie stat`). The supervisor required
`T (stopped)` for both and rejected the generation with
`dump did not leave complete tree stopped`. The controller stopped computation;
no EBS snapshot or S3 generation was committed, no restoration occurred, and no
fresh L2 benchmark ran. The 1221 recorded computational observations are partial
L0 progress and are not admitted timing results. No A/A result exists.

The failed public computation archive and metadata are retained in this report's
`public-evidence.tar.xz`. Private process memory was uploaded to versioned,
AES256-encrypted S3 at
`s3://isolab-results-104186811093-us-east-2/recovery/runs/ecbench-periodic-20261022-a/failed-private-generation.tar`.
The versioned full download matched the local SHA-256; the receipts are included.
Raw process memory is excluded from Git. The exact owned Pod and PVC were
removed after preservation; no other worker was terminated.

The corrective implementation keeps strict stopped-process and credential
admission. When a capture includes an exited child, it retains that capture,
continues the same computation so the parent can reap the child, then captures
again, at most eight times within the original deadline. It commits no zombie
capture. Running, sleeping, uninterruptible and ptrace-stopped tasks remain
errors. Every thread is checked. Successful computation completion during
recapture proceeds to native replay. This does not retry a benchmark epoch.
The repaired runtime requires a separate live qualification.

The source review's missing-seccomp allegation is also resolved by the explicit
`PR_SET_SECCOMP` call in `progress/src/main.rs`, executed before the tested
binary. Existing F restored process receipts show Seccomp 2; supervisor/progress
source at this attempt matches the qualified source. This compatibility filter
denies perf descriptors and is not an untrusted-code sandbox. CRIU's documented
[final states](https://criu.org/Final_states) and its
[v4.2.1 zombie restore path](https://github.com/checkpoint-restore/criu/blob/v4.2.1/criu/cr-restore.c#L750)
explain why zombie state is distinct from the stopped credential contract.

A process diagram is used because there are no admitted comparative benchmark
values. Algorithm leaderboards and canonical performance graphs are unchanged.
