# Primitive normalization and complete single-target integration

Primitive normalization recovers additional valid lower-degree relations and
reduces required candidate visits in11of36paired rank arms. The full solver
recovers all12frozen single-target scalars. These are finite correctness/count
results, not a cold-runtime improvement or a novelty claim.

Protocol9e47e6b6f2precedes normalization implementation. The synthetic branch
control correction8aa77773baprecedes its rerun; the failed output remains
PRIMITIVE-GATE-INITIAL.txt. Integration protocolee7b92d6e8precedes the solver.
Exact sources, seeds, caps, source receipts and known prior art are retained
in PRIMITIVE-PROTOCOL.md and RATIONAL-SOLVE-PROTOCOL.md.

## Mathematical accounting

Let h=gcd(a,b), a'=a/h and b'=b/h. Divide the function by h, remove every
original anchor condition with h(xi)=0, and recompute the residual from
a'^2-f*b'^2 and the retained anchor product. An invalidated anchor need not
remain a zero of the reduced function. Its actual zero, if any, reappears
through the residual and is oriented by a'/b'. With t=deg h and k removed
distinct anchors, residual degree is at most g-2t+k<=g-t.

The norm-factor identity has evaluated certificateIDC1hf2c6f973357f42f7,
PRIMITIVE-IDENTITY.json:32derived finite-field points plus16extra producer
recheck points, proof=null. This certifies the scalar polynomial identity,
not the divisor bookkeeping or performance. The latter is checked by full
group identities. Coprime rational reduction is established prior art:
https://arxiv.org/html/1211.6895v3, Proposition2.1and Theorems2.2/2.3.

## Algebra and rank evidence

All9408generated signed curve cases pass normalization. There are1370split
primitive group checks, including262cases previously rejected for common
factors (106at genus2,72at genus3,84at genus4). Corrected controls cover
repeated/branch/away-from-anchor factors, zero pairs and constant primitive
functions. All7algebra tests pass. The initial synthetic control used x^5+x
with coordinates belonging to x^5-x; exact division rejected it. Only the
synthetic x coefficient was corrected, with failure and amendment retained.

Default rejection replay is byte-identical to RATIONAL-RANK-VERIFIED.jsonl.
Primitive mode preserves curves/H/d/tuple streams/seeds/stop rules/caps and
independently verifies all36ranks with rank(R)-rank(R_off). It adds181normalized
admissions before arm-specific stopping,37directly emitting projected rows.
Both modes have23rank attainments,12family exhaustions and one tuple cap.
Do not infer a paired yield rate from aggregate admissions: prefixes differ.

| g,p | H,d | Rejection visits | Primitive visits | Count ratio |
|---|---|---:|---:|---:|
| 2,1009 | 12,2 | 649 | 619 | 1.0485 |
| 3,503 | 14,2 | 10611 | 7950 | 1.3347 |
| 4,127 | 16,2 | 6924 | 5104 | 1.3566 |
| 4,127 | 16,3 | 8759 | 6232 | 1.4055 |

These representative rows are drawn from all36paired arms shown in
PRIMITIVE-RANK.svg/PDF;11improve,25are unchanged. Ratios are derived as
rejection visits / primitive visits on rank-reaching arms, not seconds.
Native reduction: PRIMITIVE-SUMMARY.json; raw PRIMITIVE-RANK.jsonl.

## Complete solver gate

rational_solve.rs takes curve,D1,D2,ell,seed,requestedH,d,gate. The scalar is
only in the harness, not a solver input. The solver builds tables/catalogue,
descends both divisors, includes every descent place in the anchor set,
constructs complete signed batches, normalizes, factors/orients, deduplicates,
recombines with DenseForest, solves incrementally and verifies k*D1=D2.
It retains effectiveH, constructed/consumed outputs, common-factor admissions,
all phase clocks and terminal statuses. These gated clocks are diagnostic.

All12frozen cases recover k and pass every raw/projected group check. Requested
H9becomes H10in one genus3case; H10becomes H14/H13in the genus4cases. Requested
H28is clamped to16availableFBplaces at genus4p31. The untargeted first-H rank
screen therefore cannot price target-specific setup or effective dimension.
RATIONAL-SOLVE-GATE.txt retains all12cases. All22tests in the full heldout
Cargo example target pass, and full Cargo formatting passes.

## Replay and next scientific action

With native Rust and jq on PATH and an explicit CARGO_TARGET_DIR:

    sh research/hec_anchor_relations_20261114/primitive_gate.sh /tmp/hec-primitive-replay
    cargo test --release --manifest-path crates/hec-native-ic/Cargo.toml --example heldout

The replay checks both modes byte-for-byte, both independent ranks and the
complete12target gate; source/library/binary hashes are emitted. Native Rust
renders the TSV count plot; native Typst compiles PRIMITIVE-REPORT.pdf. Both
pages were rendered with Poppler and inspected. Original timing graphs remain
unchanged because this screen adds no comparable runtime observation.

Next: a separately frozen same-target cold comparison including setup, all
constructed outputs and primitive normalization, root orientations, target
handling, recombination and final verification. It must compare competitive
implementations and distinguish an implementation loss from a mathematical
limitation. No10x conclusion or multiplication with prior timing ratios follows
from the present gates. The full objective remains active.
