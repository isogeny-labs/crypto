# Deterministic inherited-descriptor check

One planned local software check passed with exit code zero. A pipe barrier made the descriptor lifetime deterministic, without sleeps or repeated trials.

| State | Independent nonblocking acquisition |
|---|---|
| No child; parent acquired then closed | Succeeded |
| Child confirmed inherited descriptor open; parent closed its descriptor | Rejected: errno 11, EAGAIN |
| Child closed inherited descriptor and was reaped, exit zero | Succeeded |

The descriptor had FD_CLOEXEC set before fork. Thus close-on-exec does not prevent its pre-exec inheritance. This run did not execute another program; it checked the held-descriptor interval and the closed/reaped negative control.

The host exhibited the proposed flock mechanism under controlled ordering. This does not identify the errno, child or ordering in the original GPU capsule failure. The original parallel suite failure and isolated replay remain distinct evidence in /var/tmp/pencil-gpu-lock-diagnosis-20261031-v1.

PLAN.md and check.py were hash-frozen in preexecution.json before the one execution. stdout records exact control results and host kernel; receipt.json retains the command, native exit code and elapsed time. The source uses an internal 15-second alarm and outer 60-second timeout. No repository file, runtime lock implementation, cloud job or scientific workload changed.
