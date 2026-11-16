# Full-D18 native XOR-cadence gate

This source-bound experiment changes only the native SAT solver's XOR
propagation cadence after decisions. Its input is the primary `a=0`
GF(2^83) curve, the complete direct-subgroup D18 factor base with 32,643
signed-Frobenius columns and 5,418,738 point rows, and the unchanged
`pilot-01` constructed and first public-natural target. Freshly GET the
immutable S3 object and wrapper with full-object provider checksums,
compressed-object SHA-256
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`,
wrapper SHA-256
`1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616`,
and producer-manifest binding. Use a clean committed source and one release
binary for both arms.

Retain the shared-root Frobenius domain, free selectors and phases, x/phase
priority, and exact first-pair lex ordering from the cadence-one source.
The frozen formula shapes remain 329,808 variables, 2,807,999 constructed or
2,807,501 natural CNF clauses, and 3,901 XOR rows. The new option defers a
full native XOR elimination until every 32nd decision level, level zero,
restart, or complete assignment. It never returns SAT before the complete
assignment has passed all original XOR rows. The default cadence remains
one. The generic solver now checks its deadline on conflict-free solve-loop
iterations as well as conflicts, while the independent process watchdog
still bounds a long individual propagation pass. Unit and exhaustive
small-system tests must pass before this N83 run.

First run one constructed control under a 300-second whole-worker cap and
310-second supervisor. Require native SAT, original field equations, checked
four-point group sum, signed-orbit row, rank one, altered-target rejection
and independent original-field replay. Preserve the
`positive-independent-gate.json` receipt before launching natural search.
Only then run one public natural
query under a 2,400-second whole-worker cap and 2,410-second supervisor,
with a 100,000-conflict cap in each arm. Preserve raw phase, stdout/stderr,
exact source/binary/S3 hashes, host load, wall and memory receipts. Record
solver decisions, conflicts, XOR passes, row operations/scans, clause
visits, and separate clause/XOR propagation nanoseconds on any normal
solver return. A watchdog kill leaves those counters unknown; never
reconstruct them from a stack sample.

The independent auditor must check shape and source identity, all terminal
limits and raw receipts, the positive original-field relation and two
tamper controls. A natural model advances only after its point indices and
signed-Frobenius row replay against the exact S3 base and target. A cap or
missing model remains censored. This one setting and query cannot select a
factor base. Natural relation rate, useful rank, factor-base logs,
unseen-target descent, complete cold one-target accounting and matched
same-target rho remain separate gates. Keep
`selected_best_total_runtime` null.
The Python original-field replay uses pinned `blake3==1.0.8`.

If the positive worker reaches its wall cap before `import_checked`, retain
the raw cap and supervisor receipts, classify it as capacity censored before
model construction, and skip natural search. A repeat requires a changed,
recorded resource condition; the censored arm gives no cadence comparison.

This is an opt-in experimental solver policy. It is not an assumption that
fewer XOR passes improve wall time or relation yield; the first run checks
correctness and measures one bounded natural-search interval. The local host
has no hard memory cgroup, so its times are labelled L0 shared-host.
