# R05: one bounded local sieving allocation

Frozen before production. Continue the still-composite 444-bit P-256
block-876 cofactor from R04's native-selected polynomial and verified
portable checkpoint. The literal input is

```
33492266050798185653682380062483799213318454801653963493134064721059735936944520907295931496614455858076048437184499090478420204888353
```

Prerequisites: the R02 genuine-GNFS known-answer/resume controls passed;
R04's master, selected job and polynomial parameter file match this input
and pass exact native common-root checks; its whole-target prime/product
replay passes. Preserve R04 unchanged. Use an immutable exported source
snapshot and a new raw directory, not the previous search's active state.

One local allocation: pinned YAFU c7d90283972820f99b3b4920bfc1eff05c1f6342,
same executable/sievers, seed 20261022, `-gnfs -xover 81 -pbatch 120`, two
sieve/linear-algebra threads in separate phases, native limit 3600 seconds,
outer 3660 seconds plus 30 seconds kill grace, 4 GiB virtual-memory limit
per native process. The launcher also applies a **1 GiB per-file output
cap** using Bash `ulimit -f 1048576` (1024-byte units). This session has
limited remaining disk. The cap may stop relation collection before the
native 46,000,000-relation target; this is a resource-limited continuation,
not a claim to finish the full NFS computation within this allocation.
The complete prime factorization remains required and unresolved.

Before production, exercise the version-2 receipt writer on the small
known input 1009003027, with a one-second native limit. This only validates
the command/receipt path; it is not additional genuine-GNFS coverage.
Preserve that control separately. GNU timeout's ignored-SIGINT/grace-kill
control already confirms the new signal logging.

Stop on certified completion, the native/outer bound, output-size cap or
execution failure. Preserve every relation, native state, command, source
hash, status and failed attempt. Treat every native factor label as a hint;
require exact division, native PARI prime certificates and independent
whole-target replay. A partial partition or collection is not completion,
a lower bound, a security result, a speed claim or a historical explanation.
No paid or remote machine is launched. No subsequent allocation is
authorized by this record.

An interim report must name its observation time and actual running status.
After termination, archive the full raw tree deterministically and produce
the final native replay, report, graph and inspected PDF; until then those
final deliverables remain explicitly pending.
