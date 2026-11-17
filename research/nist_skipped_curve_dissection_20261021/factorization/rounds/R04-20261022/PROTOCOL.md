# R04: native continuation from the recognized P-256 polynomial checkpoint

Frozen before execution. R03 discovered that copying the per-thread
polynomial file alone silently restarted the native search. Its attempt
was stopped (SIGINT, then SIGTERM, exit 143) as an infrastructure failure.
Native PARI has now validated 242 complete saved candidates against the
literal input, rejected a wrong-input control, and prepared the master
file with the correct `n:` header and no invented time record. A separate
three-second diagnostic confirmed YAFU's input match and last leading
coefficient 660. These are prerequisite controls, not factor discoveries.

Resume that corrected checkpoint in a fresh raw round directory. The
literal residual is

```
33492266050798185653682380062483799213318454801653963493134064721059735936944520907295931496614455858076048437184499090478420204888353
```

Use unchanged pinned native YAFU c7d90283972820f99b3b4920bfc1eff05c1f6342
and its sievers, seed 20261022, two threads and two linear-algebra threads
in their separate phase, `-gnfs -xover 81`, **`-pbatch 120`**, 4 GiB
per-process virtual-memory limit, native 1800-second allocation, outer
1860 seconds with SIGINT and 30-second kill grace. The smaller batch is a
checkpoint-safety configuration, not a performance claim. One production
attempt only. No paid or remote resource is launched.

Retain the corrected input checkpoint, pre-resume snapshot, source hashes,
exact command, native logs, status and intermediate state. The wrapper
must refuse per-thread-only state. If native state again does not resume,
stop and record an infrastructure failure, never claim new search coverage.
Stop on fully certified factors or the bound. Any additional allocation
requires a separate recorded decision. Do not reset R02 or R03 budgets.

YAFU factors remain untrusted hints: require exact proper division,
native PARI prime certificates and separate full-target product replay.
Keep partial composite residuals explicit. A finite search or incomplete
execution is no lower bound, security claim, speed claim or historical
selection explanation. Preserve failed attempts and publish a separate
report, graph and inspected PDF for this round.
