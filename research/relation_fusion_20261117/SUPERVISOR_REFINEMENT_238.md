# Parent-death refinement238

The first supervisor passes four OS controls and216functional proof cases (229/234), with a216case saved-artifact audit237. Review found that its Rust Drop cleanup did not execute if the supervising collector itself was killed. The old records remain unchanged and do not establish parent-death protection. [pcc:unverified]

The refinement registers Linux PR_SET_PDEATHSIG(SIGKILL) before exec and checks the original parent PID to close the fork-to-registration race. The hook does no allocation or logging. Five OS tests238pass at exit0, including a new test that kills the supervisor and observes termination of its running worker. The intended workers are fixed unprivileged single-process binaries; no credential changes, privileged exec or guard disabling are supported. This is a research supervisor, not a general confinement mechanism. [pcc:unverified]

Final source/tools239are frozen before a repeat of the same216wrapped/native functional cases and six transport controls from plan230. Every complete proof, basis, statistic and constructor identity must remain equal. No parent timing statistics or15xclaim are licensed. Strongest controls must receive this same generic improvement in any future comparative protocol. Source/protocol/export validation remains a separate open dependency. [pcc:unverified]

Sources checked: Linux man-pages PR_SET_PDEATHSIG, pidfd_open, poll, getrlimit and setsid; official Rust CommandExt pre_exec safety documentation; local glibc/Linux headers. Runtime probes, source hashes and all failed/intermediate builds remain. UserLocalCPU0/one-thread/zero-cloud and checkpoint/S3 waiver are unchanged.
