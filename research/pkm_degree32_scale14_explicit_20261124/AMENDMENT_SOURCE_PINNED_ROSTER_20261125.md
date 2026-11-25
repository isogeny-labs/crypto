# Amendment: extend the deterministic explicit-curve roster

The first 16 curves in `PROTOCOL.md` were screened with the source-pinned
binary. Only indices 6 and 13 had equal control/candidate factor-base sizes;
therefore the required three-curve panel cannot use that roster. Before any
timed call, extend the exact same deterministic prime/`a`/`b` enumeration
past index 16, stopping at the first additional curve whose source-pinned
preflights are valid and size matched, or after index 64. Preserve every
candidate curve and preflight in the append-only store. Check that enumerated
indices 1..16 reproduce the frozen roster before scanning index 17. The
selected set is the first three qualifying indices in roster order. If fewer
than three qualify by 64, terminate without a timed comparison.

This amendment changes only the roster's length. The binary, target stream,
factor-base constructions, solver, timing order and cost gates in
`PROTOCOL_SOURCE_PINNED_20261125.md` remain fixed.
