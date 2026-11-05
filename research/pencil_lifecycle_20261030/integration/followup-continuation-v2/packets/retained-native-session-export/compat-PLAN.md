# Prospective historical metadata compatibility diagnostic

Root authorizes at most three rustc metadata-only probes, total 60 seconds, hard AS 2 GiB, 16 MiB combined compiler logs, stopping on first success. No exporter compilation/test, source rebuild, or scientific runtime. Prior attempts remain immutable.

Selection order: n83-r38-target has the earliest surviving artifact timestamp (Nov 1 20:36) and the nine-source-file graph dependency list; crypto-ic-wide-f2m4-target retains the same nine-source list with distinct historical bytes (its depinfo points to prime-root origin, explicitly not independent build provenance); crypto-ml-prime-root-target retains a third distinct byte identity and matching nine-source list. Current graph expanded to much larger metadata. These are candidates only; source paths/timestamps do not authenticate compiler compatibility. Byte hashes and depinfo retained in compat-inputs/candidates JSON before invocation.

Each isolated dependency directory contains symlinks to exactly the existing shared library/metadata files except all pencil_symbolic_graph artifacts are replaced by one candidate pair. Shared files are never overwritten. Retained crypto_lib remains exact14c28c4c... . No compiler options change except dependency search directory and output path.

Exact outer command: timeout -k 5s 55s python3 compat-probe.py. Runner fixes child argv and RUSTC_LOG=rustc_metadata::locator=info. Per-stream file ceiling 2796202 bytes gives at most 16777212 bytes for six compiler streams. A shared 48-second inner deadline includes pre/post hashing; no invocation begins after deadline. Outer TERM55 + KILL5; SIGTERM invokes tracked child-group kill/reap. No automatic fourth probe or memory increase.
