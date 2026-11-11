# Draft protocol: refutation degree of quadratised chain representations (not frozen)

Status: **draft**. It freezes only when the instrument binding below is filled in and reviewed;
running it needs the resume checkpoint (`CHECKPOINT_UPLOAD_TOKEN`), which this session lacks.

## Question
Does the line norm-form quadratisation (mu = e2), or adding the implied linear trace equations,
lower the closure (refutation) degree of the Weil-descended chained decomposition system below
what a structureless quadratisation (w = x_a x_b) gives?

## Arms (identical draws in every arm)
1. `chain_s3`: chained S3, cubic (baseline).
2. `quad_generic`: auxiliary w = x_a x_b on every cubic merge (confound control: degree 2 with no geometry).
3. `e2_aux`: auxiliary mu = e2 on every cubic merge (the candidate).
4. `chain_trace`: chain_s3 plus Tr(x_a + x_b + x_c + a) = 0 per merge, linear (implied; candidate).

Systems are exported by `derive/` (`systems::build`), which already verifies the planted
solution and single-bit-flip detection for every arm.

## Cells
(m, l) in {(3, 5), (4, 5), (4, 6)}; n in {17, 19, 23}; Koblitz (a, b) = (0, 1) and (1, 1).
Per cell: 16 unsatisfiable draws (random target x_R on E, verified UNSAT by exhaustion over V^m
at these l) and 4 planted-satisfiable draws (positive control: closure must not refute them).

## Metric and instrument
Primary: smallest degree D at which the instrument derives 1 (refutation) on unsatisfiable draws;
secondary: Macaulay matrix rows and columns at that D. Instrument: the closure-degree tool used by
`LEAD-ICE-descent-closure-degree-at-scale` (binding to be filled: path, commit, CLI; an exporter
from `derive/` to its input format is the remaining work).

## Decision
- `e2_aux` or `chain_trace` refutes at a lower D than `quad_generic` on >= 12/16 unsatisfiable
  draws in every cell with m >= 4: a representation effect; price it at m >= 5.
- Equal D (within one degree on >= 12/16): relabelled generic degree reduction; close the lead.
- Any planted-satisfiable draw refuted: instrument or exporter fault; the cell is void.

## Variable ledger
Same draws, seeds, instrument, host and degree cap in every arm; arm order interleaved per draw;
arm labels masked until the decision is computed.
