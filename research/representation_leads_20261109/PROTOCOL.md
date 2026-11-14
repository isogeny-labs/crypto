# Protocol: refutation degree of quadratised chain representations (frozen 2026-11-14)

Lead: `LEAD-ICE-quadratised-chain-representation` (crypto-autoresearcher atlas).
Status: **frozen, not run**. Running it needs the resume checkpoint (`CHECKPOINT_UPLOAD_TOKEN`),
which the authoring session lacks. Amendments after this date are dated below; arms, cells,
metric and decision rule do not change once any cell has run.

## Question
Does the line norm-form quadratisation (mu = e2), or the explicit implied linear trace equations,
lower the closure (MutantXL) refutation degree of the Weil-descended chained decomposition system
below that of its matched control?

## Arms and matched pairs (identical draws in every arm)
| arm | system | matched control | variables (cells below) |
|:--|:--|:--|:--|
| `chain_s3` | chained S3, cubic | — | 20 / 23 / 22 / 34 |
| `chain_trace` | `chain_s3` + Tr(x_a + x_b + x_c + a) = 0 per merge (linear, implied) | `chain_s3` | same as `chain_s3` |
| `quad_generic` | auxiliary w = x_a x_b on each cubic merge (degree 2, no geometry) | — | 31 / 34 / 35 / 56 |
| `e2_aux` | auxiliary mu = e2 on each cubic merge (line norm form) | `quad_generic` | same as `quad_generic` |

Each comparison is between arms with the same variable set, so their Macaulay matrices have the
same columns at every degree: the comparison isolates the representation, not the size.

## Cells and draws
(n, m, l) in {(11, 3, 3), (11, 3, 4), (13, 3, 3), (11, 4, 3)}, Koblitz (a, b) in {(0, 1), (1, 1)}:
8 cells. Per cell 16 unsatisfiable draws (random target on E, a fresh random basis V, verified
undecomposable by exhaustion over all m-sums of factor-base points including T = (0, sqrt b)) and
4 planted satisfiable draws (soundness control).

Systems: `derive/` at this commit, `target/release/replead-derive export > results/systems.jsonl`,
deterministic (fixed seed), 640 lines, **sha256
`251302b560bfb05502e5e455997ce6f207a119faa0c959a7480da197ab94b86c`**. A run whose regenerated file
has another hash stops before scanning.

## Instrument
`examples/replead_closure.rs` (this commit), which calls the same `closure_refutation_scan` (dense)
and `sparse_closure_refutes` (sparse) routines as `examples/dreg_closure.rs`. It aborts if a
planted assignment fails any equation, and flags any refutation of a satisfiable draw.

Per draw and arm: dense scan from degree 2 upward while C(N, <= D) <= 70 000 columns; the next
degrees up to 6 on the sparse engine with `--max-entries 1500000000`. A degree not built is
unmeasured (censored), never a refutation.

## Metric
Primary: D* = the smallest degree at which the closure refutes (contains 1), per unsatisfiable draw
and arm; censored at the largest measured degree when no refutation occurs.
Secondary: closure rank and products at D*; XL refutation degree.

## Decision (per matched pair, per cell; computed with arm labels masked)
- **Representation effect**: the candidate (`e2_aux`, resp. `chain_trace`) has a strictly smaller
  D* than its control on >= 12 of 16 unsatisfiable draws, in every cell with m >= 4, and in no
  cell is the control smaller on >= 12 of 16. Then price the candidate at m >= 5 (next step).
- **No effect**: D* equal on >= 12 of 16 draws in every cell: the candidate is a relabelled degree
  reduction (resp. a redundant equation the closure already finds); close the lead.
- **Mixed or censored**: anything else, including > 4 censored draws in a cell: report as
  inconclusive at this scale with the censoring stated; no claim.
- **Void**: any satisfiable draw refuted in any arm (instrument fault) voids that cell.

## Variable ledger
Same draws, seeds, instrument binary, host, column and entry caps in every arm; arm order
interleaved per draw (`--arm` runs in the order chain_s3, e2_aux, chain_trace, quad_generic,
rotated by draw index); arm names masked to A-D in the analysis file until the decision is
computed; host recorded with the run receipt (EKS isolab per CLAUDE.md, L1 sufficient: the metric is
a degree, not a time).

## Amendments
- 2026-11-14: frozen from PROTOCOL_DRAFT.md; instrument bound (`examples/replead_closure.rs`);
  cells reduced to the four above so every arm fits the instrument's 64-variable cap and the dense
  70 000-column cap at degree >= 3; `chain_trace` added as the fourth arm; `line` arm dropped
  (lambda is eliminable, see REPORT.pdf).
