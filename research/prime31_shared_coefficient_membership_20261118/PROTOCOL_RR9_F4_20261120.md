# Prime31 common-support RR9 / F4 discriminator

Status: prospective and frozen before any registered RR9 grader or F4 case was
executed on 2026-11-20.  This protocol is separate from the weighted selector
SAT panel: the frozen RR9 predicate has one shared x-coordinate support.

## Question and exact predicate

For the canonical Prime31 base of size `B`, let
`H_B(X) = product_i (X-x_i)`.  For public residual `R=(r,s)`, the seven RR9
chart variables define the monic degree-eight polynomial
`M_(A,V,R)(X)`.  The candidate solves the exact frozen cubic tower

```text
H_B = Q_0 M + R_0                     (B >= 8)
R_(j-1)^2 = Q_j M + R_j, j = 1,2
R_2^2 = Q_3 M
```

with the documented padded `R_0=H_B` case for `B<8`.  This is equivalent to
`M | H_B^8`.  The registered shapes are B4/B8/B12/B16/B24/B31 with
44/52/56/60/68/75 variables and 45/53/57/61/69/76 cubic equations.

The solver receives only the curve, public base, `H_B`, and residual.  Every
returned assignment is checked against all tower equations, reconstructed by
independent monic polynomial division, decoded through Hermite vanishing orders
to a signed eight-multiset, and replayed with independent affine EC arithmetic.

## Inputs and truth boundary

Initial smoke cases are frozen now:

- `prime31-rr9-b8-known-chart`, a positive control whose chart is retained only
  in correctness tests/the isolated truth side and is absent from case JSON;
- `prime31-rr9-b4-natural-00`; and
- `prime31-rr9-b8-natural-00`.

Natural residuals use SplitMix64 seed `0x5252395f50414e45 xor B` and unbiased
nonzero scalars of the frozen generator.  The isolated grader enumerates all
signed eight-multisets: `C(2B+7,8)` = 6,435 at B4 and 490,314 at B8.  Its
index, multiplicities, and witnesses never enter the candidate process.  Case
IDs and solver arms are fixed before those labels are read.

## Source-linked F4 arm

`rr9-f4/build.rs` copies the repository's exact
`src/cryptanalysis/f4_fp.rs`, `reuse.rs`, and `pair_certificate.rs` into the
build output.  The research crate supplies narrow hashing, ordering, primality,
root-finding, and operation-counter interfaces; it does not fork the F4
implementation.  Run single-threaded grevlex with the normal strategy.

The smoke ladder uses degree cap 4 on all three cases.  Degree cap 6 is a
predeclared escalation only for a cap-4 incomplete/timeout row and for the B8
known-positive control; it is not selected as a post-hoc winning arm.  Each row
has a 900-second, 16-GiB outer cap and an internal deadline five seconds
shorter.  Stop after two consecutive cap timeouts or one OOM.

Record the complete `VisitReport`, row-reduction field multiplications,
repository native-operation counters, supplemental field/row/word/probe
counters, variables/equations/input degree, matrices, F4 calls, root-finding
work, CPU/wall/RSS, binary/source/case/tower/command hashes, and all forbidden
counters.  Do not translate these units into group additions.

## Decisions and holdouts

A verified SAT assignment is accepted only after independent decode and EC
replay.  Complete UNSAT is compared with the isolated grader.  Complete UNSAT
on the known-chart case, an invalid decoded model, or an equation/division
disagreement is an implementation failure.  Timeout, OOM, SIGKILL, incomplete
traversal, or degree-bound exhaustion is unknown, not a negative mathematical
result.

Run the 64 natural B4/B8 panels only if one frozen degree arm completes both
smoke controls without disagreement.  B12/B16 are held out and use the same
known chart; enable them only after B8 correctness and only if measured
solver-native work growth is below both transparent controls:
`C(2B+7,8)` signed multisets and `(2B)^4` four-plus-four words.  These are
different work units and are reported side by side.  B24 is a shape rung.
B31 and whole relation collection remain disabled until B12/B16 pass and a
complete oracle-cost model, relation yield, linear algebra, extraction, and
replay protocol is committed.

This experiment can retire only the frozen common-support cubic RR9/F4
engineering path in the tested bounds.  It cannot refute weighted selector
SAT, slot-specific resultants, or all index-calculus representations.

## Provenance

The tower source is recovered from the PR #4692 session record and checked
against PR #4816 commit `040ef64`'s explicitly labelled frozen PR #4692
builder.  The exact F4 engine is the current repository source.  Primary
background remains Petit--Kosters--Messeng (PKC 2016),
Amadori--Pintore--Sala (FFA 2018), and the F4 literature cited by
`src/cryptanalysis/f4_fp.rs`.  The research KB and Conductor were unavailable;
that receipt is recorded in the selector protocol and session log.  Later PR
#4816 outcomes are not imported as measurements here.

All evidence remains `[pcc:unverified]`.
