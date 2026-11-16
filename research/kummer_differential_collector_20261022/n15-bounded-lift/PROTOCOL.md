# N15 bounded signed-lift protocol (frozen before replay)

Source base: `87b37edd6bd`. Use the existing 21 frozen odd-prime models and
their target-index-zero subgroup generator. No new models, target logs, or
external jobs. Enumerate every scalar in each of the three 16-bit subgroups;
for each of the remaining 18 models use the N14 fixed 1,029-scalar set
`{0,1,2,r-2,r-1} ∪ {floor((2i+1)r/2048): 0 ≤ i < 1024}`.

For each signed input `(K(Q), K(Q+G))`, first try the existing fast Recover
source. On any failure, enumerate candidate divisors from the algebraic
inverse of `K(Q)`: try the direct general conversion and the five fixed
2-torsion action translates, with modular square roots of `b1²`; if `b1=0`,
solve `b0²` separately. Undo a translated candidate by adding its known
2-torsion divisor. The proposed runtime path checks divisors with the native
Jacobian validity predicate and matches both Kummer inputs projectively;
it must never compare with the oracle Q or its scalar. Count candidate
enumerations and field/Jacobian operations separately. The independent
validation path constructs `Q=kG` and checks the output against it.

For every state retain prior fast status, fallback status, final correctness,
candidate counts, per-branch counts, and the native operation counters. Bind
raw rows and source/input files by SHA-256. Report failures and nulls, and
do not infer an all-state bound from finite replay. State a separate
algebraic completeness argument and its assumptions, including degree<2,
2-torsion actions, zero square roots, and sign selection. Derive xADD and
general conversion denominator conditions exactly. Report module setup and
online costs separately. No rho ratio without a full matched collector run.

Render a neutral graph and PDF from the checked rows. The native validator
must independently recompute row counts, status/cost totals, frozen sample
continuity, source hashes, and raw digest before results are reported.
Correction after the first replay: the frozen source has five rational
2-torsion matrices, not four. The implemented loop already traversed all
five; its status-label array covered only four, but no replayed state reached
that branch. The protocol's scalar set, candidate method, and measurement
fields are unchanged. The bound and label are corrected before final replay.
