# `inherited-f4-batched`: fewer counted rewrites, same decisions (2026-11-12)

Owner's ask (2026-11-12): "do the rest of this stuff. Keep working on these
things". This is the third lever named after the previous round: the eager
rewrite of kept rows was 11 to 20 % of `inherited-f4`'s instructions
([`../inherited_f4_next_20261112/`](../inherited_f4_next_20261112/) profile).

## What changed

Written by an Opus subagent to a frozen brief, reviewed in the main session;
`src/cryptanalysis/inherited_f4.rs`, `src/cryptanalysis/koblitz_groebner.rs`,
engine registration in `src/cryptanalysis/ic_framework/solvers.rs`.

Between two splits a search node specialises several times (the split
variable, then each forced variable), and the default eager mode rewrites
every kept row of the reduced basis at every specialisation, although a node
that is refuted or ends at a leaf never reads most of them. With the switch
`koblitz_groebner::with_batched_rewrite` on (engine `inherited-f4-batched`):

- kept rows stay in their old column layout at a specialisation; displaced
  rows are rewritten and re-reduced exactly as before;
- a row is brought current when it is read (pivot of a re-reduction, tail
  row, completion, closure), through the composition of every step since its
  last write, with the eager kernel and the eager per-row charge;
- at a split every stale row is brought current once, before the two
  children share the basis (charged inside the counted region).

**Why decisions cannot change.** A rewrite changes only the column layout of
a row, not the polynomial; the composed map is the composition of the step
maps, so a row brought current late is the row the eager path holds. Kept /
displaced is decided by pivot columns, tracked eagerly in both modes, and
displaced rows are inserted in the same order against the same pivots. The
basis after every operation is therefore identical, and so are the linear
tail, refutations, forced variables, splits and roots. Every tree statistic
(reductions, splits, propagations, infeasible branches, eliminated, oversize,
tail enumerations, degree reached, exhausted) is identical; only `ops` (word
reads and writes of the specialisation) changes.

**Why the count only drops for work no longer done.** A batched rewrite
across steps e→now charges the first read and the last write of that chain,
never more than the eager steps it replaces; the split materialisation sits
where the eager parent's last rewrite sat; rows never read before a leaf or a
refutation are not rewritten at all. Tests check the stronger statement that
`ops − rewrite words` is identical in both modes on every system. Worked
example (pin system 23, 10 unknowns): 80 → 66 word operations, the 14 being
18 words of eager kept-row rewrites replaced by 4 words of two late single-row
rewrites; seven of the nine eagerly rewritten rows were never read.

Tests: `koblitz_groebner::tests::batched_rewrite_leaves_decisions_and_never_adds_word_xors`
(33 pin systems + 8 dense at 12/14/16/20 unknowns, both modes, cold and warm),
`inherited_f4::tests::batched_rewrite_builds_identical_bases_with_no_more_work`
(80 basis chains, both kernels), and
`ic_framework::solvers::tests::size_rule_and_inherited_fold_match_the_reference`
(extended: batched gives the same solutions with no more ops). Ops ratio
batched/eager on the 41 test systems: 0.9843 in total, 1.0 worst, 21 systems
cheaper `[pcc:unverified]`.

## Known cost before measuring, and the fix

The first version (1da7694876, merged in #4005) was checked unpaired by the
subagent on Koblitz 19:10:2 and 23:12:2: 0.879 and 0.830 of the ops per
target and **8 to 13 % more wall time** `[pcc:unverified]`. Callgrind put
the cost in the allocator: one allocation and free per row brought current
on its own (about 27 M allocator instructions against 9 M for eager on
23:12:2, while the batched arm already executed about 3 % fewer
instructions overall).

4f0eca9eb4 rewrites such rows into spans of a per-thread write-once arena
(zeroed 16K-word chunks; rows above 4K words keep their own buffer) and reads
every row through a pointer and length with an owner that keeps the buffer
alive. No counted value moves: the test
`koblitz_groebner::tests::batched_rewrite_counts_are_pinned` pins `ops`,
`bitop/v1` word ops and hash probes of the 41 systems, cold and warm, to the
values of 1da7694876. Callgrind after the fix, `HybridF4::solve` on 23:12:2:
eager 273.2 M, batched 249.5 M instructions (0.913); the subagent's unpaired
wall check: batched 0.92 to 0.96 of eager at 23:12:2 and parity within noise
at 19:10:2 `[pcc:unverified]`. The overhead outside the counted unit no longer
grows, so a lower S here rests on counted work that disappears without a
hidden wall cost.

## Run 1: paired per call (`run_ab.sh`)

`ic descent --solver inherited-f4 --solver inherited-f4-batched --solver
fes-f2`, the 13 cells of the previous study (14 to 32 unknowns), 6 targets × 3
repeats, one thread, frozen binary (`IC_BIN`).

Measured at 4f0eca9eb4 (the arena fix), frozen binary. Expectation: ops
batched/old ≤ 1 on every target, about 0.83 to 0.88 at 20 to 24 unknowns,
closer to 1 below; 0 disagreements; wall batched/old at or below 1 (about
0.9 to 1.0) from 20 unknowns up, near 1 below. Falsifier: any target with
more ops, any disagreement, no ops reduction at 20 unknowns and up, or a
median paired wall ratio above 1.05 at 20 unknowns and up.

## Run 2: end to end (`run_e2e.sh`)

`ic bench --sweep` on the six e2e-ladder instances, `inherited-f4` and
`inherited-f4-batched`, 3 repeats. Expectation: same planted logarithms
recovered, S of the batched row below `inherited-f4`'s roughly by the ops ratio
of its cell (the solver is 88 to 100 % of the row's GAE): about 0.85 to 0.9 at
Koblitz 19 and 23, closer to 1 at the small instances. Falsifier: batched S
above `inherited-f4`'s on any instance, or not below at Koblitz 19 and 23.

## Results

Filled after the runs.
