# Engine equivalence after the 2026-10-19 merge of main (PR #1902 conflict resolution)

`origin/main` changed `f4_fp.rs` (+211 lines: a blocked reduction of pair
rows, `reduce_pairs_blocked`, switchable with `F4_FP_FL_BLOCKED=0`) and
`gaudry_quartic.rs` (visibility only) between the frozen binary and the
current branch head.

The frozen binary (SHA-256 `622a47d3482961d8949dc4aed62350121cb56f6ca006180bcb7774d746c75297`,
built 2026-10-16T03:39Z from the branch at commit `c2cb903ac`) ran every
remaining DLP arm, and is the one every committed DLP record came from.

Check: `joux_vitse_edwards cost --p 271 --seed 1 --residuals 6` on (a) the
frozen binary, (b) a binary built from the merged head (commit `bccc090c9`), (c) the
merged binary with `F4_FP_FL_BLOCKED=0`.  Compared per record: trace
recording multiplications, symmetrised-`S₅` multiplications, base size,
committed-solve and replay multiplications, Weil multiplications, number of
decompositions, and the three correctness flags.

Result: **all records identical in (a), (b) and (c)** on this sample.  The
new reduction is a reordering with the same counted multiplications.  The
files here are the three outputs; the comparison is the identity of their
fields as listed.

Scope: six residuals at one size and one seed, cost mode only.  It does not
cover other sizes, and it does not check the multiplication counters under
the DLP driver's concurrent batches.  Final tables therefore cite the frozen
binary's records and are regenerated with tools built from the merged head
only for the summary arithmetic, which reads recorded values and does not
call `f4_fp`.
