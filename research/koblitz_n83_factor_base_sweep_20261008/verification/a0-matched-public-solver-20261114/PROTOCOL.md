# N83 a=0 same-point natural solver pair — frozen 2026-11-14 UTC

## Contribution and input

This bounded pair measures whether the retained ordered full-D18 four-summand
and K2048 five-summand constructions produce a checked relation on the same
second sealed public point,
`(5809312440517013698248638,3568442427236746854022671)`, in the primary
`a=0` N83 subgroup. The point and both formulas are fixed by the export
receipts in the preceding public-fixture PR. The D18 base contains 5,418,738
subgroup-usable points and 32,643 folded columns; K2048 contains 339,968
subgroup-usable points and 2,048 folded columns. The different summand counts
are part of each complete candidate, rather than a controlled base-size-only
change.

| Arm | Source commit | Formula SHA-256 | Formula bytes | Shape (CNF clauses, XOR rows) |
| --- | --- | --- | ---: | --- |
| D18 m4 ordered | `cfa916b8d169962c9b906828afe0899136865d69` | `e0e29d928ebedee1177eefb5aa1d88d247a5e28329bffa9b93da5aefac167fe9` | 50,520,592 | 2,807,501; 3,901 |
| K2048 m5 | `34cb2f0bfe30b0609a2f55a6d396ecfc60c53e55` | `ecf8badc76aa4295b7d005f8c2f55359979e7d4f1210edf829ad6c68d1fb9bef` | 20,511,051 | 1,140,557; 4,980 |

The previously checked constructed-positive assignment and original
CNF/XOR/group relation replay for each base are prerequisites. The runner
checks their committed hashes and statuses before launch. It also checks both
raw formula hashes, receipt hashes, public target, solver binary and its own
committed source. The exact `LOCK.json` is committed before any attempt.

## Bounded execution

Run D18 first, then K2048, sequentially on this Mac. Both use the same
CryptoMiniSat 5.14.7 binary, one thread, seed `20261114`, VSIDS branching,
`--maxconfl 1000000`, `--maxtime 600`, at most 32 active Gaussian matrices,
and forced Gaussian admission. K2048 raises the Gaussian column ceiling to
25,000 to admit its measured wide matrices; D18 retains the prior exact
VSIDS policy. An independent 660-second process-group watchdog encloses each
call. For each arm the runner samples one-minute load and swapouts three times
at 15-second intervals immediately before launch; load must remain at most
24, swapouts unchanged, and scratch free space at least 2 GiB. Preserve a
rejected host gate as an output directory with its samples.

Retain the exact command, stdout, stderr, `wait4` CPU and peak RSS, formula
and solver hashes, conflict count, terminal status, and host samples for both
arms. A SAT result enters `SAT_PENDING_REPLAY`; check its original CNF and XOR
constraints, archive point lift, subgroup and group sum, and inserted relation
row with a separate verifier before counting a relation or novel rank.
`INDETERMINATE`, watchdog, host rejection, unverified UNSAT, and producer
failure remain distinct terminal statuses. The pair is an algorithmic
diagnostic on a shared Mac host. The follow-on selector requires a larger
common-query panel with checked relation yield and novel rank, then complete
one-target IC/rho measurements on an admitted Tier 1 or Tier 0 host.
