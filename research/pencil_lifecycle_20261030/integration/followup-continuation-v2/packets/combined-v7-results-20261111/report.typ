#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Combined signed/Kummer v7 result
Terminal receipt report · 2026-11-11

Materialization passed: 4,485 source files, 2,165 cache-file modes and 82 executable cache inputs were recorded as preserved. Compilation started, then the monitor raised FileNotFoundError while traversing target/release/deps/rmetaOgIf8S. The compiler process was killed and reaped with exit -9. No tests, full739 driver calls, forced probes or adapter negatives ran. Terminal sealing passed and cleanup completed.

#image("timing.svg",width:100%)

== Accounting and resource observations
The full 2,400-worker-second attempt reservation is charged: 18,180 charged and 111,420 remaining at this terminal boundary. Deadline: November 11, 12:00 UTC. The recorded cgroup CPU time is 318.248421 seconds. Runtime controls were 2 CPUs, 16 GiB memory and zero swap. Dollar cost remains unmeasured.

== Evidence and scope
The raw archive preserves exact native and caller receipts, logs, and materialization.json. All 11 native terminal-manifest entries and the completion-bound manifest digest were rehashed. The worker-bound materialization receipt matches. These are producer-report self-checks; independent result review remains separate. Prior v6 permission failure is retained in its original result packet. The v7 failure occurred in filesystem monitoring during compilation; it does not supply test outcomes.
