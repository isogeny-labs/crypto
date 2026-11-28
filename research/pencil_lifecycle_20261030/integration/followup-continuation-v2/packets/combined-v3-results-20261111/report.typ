#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Combined signed/Kummer v3 terminal result
Receipt report · 2026-11-11

The attempt ended during materialization. The worker monitor traversed an absent cache/target directory and raised FileNotFoundError. Its sole child command, the materializer, was killed and reaped with exit code -9. No compiler or test command was invoked. The launcher completed owned cleanup, and the terminal output seal passed.

#image("timing.svg", width: 100%)

== Resource and campaign accounting
The native cgroup reported 2 CPU quota, 16 GiB memory, zero swap and 512 task limit. Recorded terminal CPU usage was 114,182,000 ns. The full 2,400-worker-second reservation is charged. Campaign accounting after this attempt is 6,180 charged and 123,420 remaining from 129,600 total, subject to the November 11 12:00 UTC deadline. Dollar cost and scientific cost ratio are null.

== Exact evidence
The packet retains 18 copied raw files: native receipts, caller receipts and campaign ledger. All nine entries in the native terminal manifest and its completion-bound digest were rehashed. The completion result is failed with seal_status passed. The caller exited 1; automatic retry was false.

Publication observed by the caller: a70570a105f9c555b1efb62bccf7c8d1b7f1309e. Admission hash: 2ef4a223a05e645ea18130c163ac4aa4befdb006eac6400f2f2379f1aacf8769.

== Review files
summary.json and metrics.csv preserve exact timing values. graph-data.csv supplies plotted values. RAW_MANIFEST.json binds copied bytes; raw-receipts.tar.gz contains those receipts only. Source repair and another invocation are separate work.
