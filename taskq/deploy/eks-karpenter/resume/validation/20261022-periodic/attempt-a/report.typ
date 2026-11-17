#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Periodic checkpoint qualification A — failed
#raw("ecbench-periodic-20261022-a")

#image("flow.svg", width: 100%)

== Observed failure
The first CRIU dump returned exit 0. Root PID 531 was stopped; child 5463 had already exited and was a zombie. The supervisor rejected that process state and stopped computation. No snapshot generation committed, no restoration occurred, and no fresh L2 benchmark ran. Partial computational records are L0 and do not count as benchmark timing. No A/A result exists.

== Frozen scope and evidence
The public 32-bit fixture requested 600 rounds, one warmup, two targets and three arms, with 30-second checkpoints and a 3,600-second deadline. Runtime source: #raw("e05ccbc73fbfaa9fec5af6f32d60de1bdcb275f8"). See protocol.json for the exact source/image/binary/spec and request identities. See generation-metadata for CRIU's zombie observation and stopped-parent receipt; failed-state.json records the terminal result.

The complete failed computational output is retained in public-evidence.tar.xz. Process memory stays in private, versioned AES256 S3; upload/readback receipts and its SHA-256 are included. The full readback matched. The exact owned Pod and PVC were deleted after preservation.

== Corrective change and limits
The repair preserves strict stopped-process and credential checks. An uncommitted capture containing a zombie is retained; the same computation continues to reap the child, then is captured again, at most eight times before the original deadline. No benchmark epoch is retried. A separate live run must validate the fix and successive committed generations.

This is infrastructure correctness evidence, not an algorithm performance claim. No comparative values were admitted, so the visual is a process diagram. Algorithm leaderboards and performance graphs are unchanged. REPORT.md links CRIU's final-state and zombie-restore source references and the seccomp review correction.
