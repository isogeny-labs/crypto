# Kernel-mode census for the relation-rank frontier

Date: 2026-11-18

## Result

The 11 claims transitively downstream of `HEUR-RELATION-RANK` do not share one
linear-algebra obligation.  The current graph conflated four modes:

- target-dependent left-kernel extraction;
- reusable right-kernel/full-column-rank tables;
- graph-cycle elimination; and
- transfer/comparison claims with no relation matrix of their own.

The generic sparse-linear-algebra node is now mode-parametric.  The rank
heuristic qualifies only its reusable-table branch, while
`THM-LEFT-KERNEL-RELATIONS` enables its target-dependent branch.  This removes
the spurious support path from the rank heuristic into the rigorous
Gaudry/GTTD/Diem/GHS chain without claiming that their separate yield,
large-prime or transfer hypotheses have been discharged.

## Classification

| claim | mode | disposition | evidence limit |
|---|---|---|---|
| `ALG-SPARSE-LINEAR-ALGEBRA` | mode-parametric | split the node statement; left-kernel existence is proved, reusable rank remains heuristic | concrete cost still depends on sparsity and implementation |
| `ALG-WD-GAUDRY-2000` | target-dependent left kernel is admissible | no longer inherits reusable-rank support | its relation-count statement remains conditional; primary paper was unavailable |
| `ALG-WD-THERIAULT-2003` | unresolved | remove only the transitive graph contamination | primary paper was unavailable, so no independent kernel-mode claim is made |
| `THM-WD-GTTD-THM1` | target-dependent left-kernel theorem chain | supported through the mode-parametric sparse step | only the simplified algorithm is rigorous; the full variant remains heuristic |
| `THM-WD-DIEM-SMALL-GENUS` | target-dependent left-kernel theorem chain | supported through GTTD and the mode-parametric sparse step | its stated fixed-genus/model hypotheses remain |
| `ATTACK-GHS-WEIL-DESCENT` | no direct relation matrix | remove transitive rank contamination | its transfer-kernel condition remains explicit |
| `ALG-SP-JV-COVER-DECOMPOSITION` | reusable table/right rank appears load-bearing | keep conditional; do not import the left-kernel result automatically | double-large-prime and decomposition assumptions also remain |
| `COND-WD-JV-COVER-DECOMP` | reusable table/right rank appears load-bearing | keep conditional | direct journal source was unavailable; author/ePrint material was read previously |
| `COND-WD-KMPS-INEFFECTIVE` | no direct relation matrix | remove transitive rank contamination | it compares asymptotic algorithms and retains its optimistic Hess premise |
| `COND-HASHED-ECDL-TENSOR-BOUND` | reusable right/full-rank certificate | keep the direct rank condition | the complete typed tensor/rank composition is still open |
| `COND-RP-TWO-SUMMAND-FAMILY-LAW` | graph-cycle alternative | replace the generic rank edge with `GAP-RP-CYCLE-CLOSURE-LAW` | the random-edge first-cycle law is still heuristic |

## Formal boundary checks

`KernelModeCountermodels.unused_column_gives_nonzero_right_kernel` proves that
an unused column supplies a nonzero right-kernel vector.
`surplus_can_have_both_nonzero_kernels` applies the existing
`LeftKernel.exists_left_kernel` theorem to a `(B+1) × B` zero matrix while
exhibiting a nonzero right-kernel vector.  Therefore row surplus is sufficient
for left-kernel existence but not for reusable right uniqueness.

The countermodel does not assert that natural relation rows are zero or that a
specified random ensemble fails.  It refutes only the implication from row
count to full reusable rank without an ensemble/support hypothesis.

## Sources checked

- `papers/ecdlp-index-calculus-treatise/sections/hardness.tex`, especially
  “Relation rank and the left kernel”.
- `research/ecdlp_weakness_lean_20261104/lean/LeftKernel.lean`.
- `docs/ecdlp-hardness-corpus/lit-books.json`, Galbraith–Gaudry §6.2 record.
- Primary-read source records for GTTD, Diem, GHS, Joux–Vitse and KMPS in the
  corpus.  The Gaudry 2000 and Thériault 2003 primary texts remain recorded as
  unavailable/paywalled; their kernel modes are not newly asserted here.

The shared research-KB executable was unavailable; these repository paths are
the bounded fallback receipt.
