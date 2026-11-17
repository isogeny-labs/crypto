# strict-4E factor-base selection in the Rust IC framework (2026-10-27)

This study is a port of the rule measured in `ml-cryptanalysis:experiments/fb-cost-model-construct`, which uses the exact 4E membership test and a sieve-built strict subspace. It is tested here on complete, verified DLPs through `ecbench`. Protocol: [`PROTOCOL.md`](PROTOCOL.md). Run report and PDF: [`reports/run-20261027T0802Z/`](reports/run-20261027T0802Z/report.md).

## Result

**Unresolved against the default base. The order flips inside the §5a band, and matched rho is cheaper than both IC arms in every cell at every weight.**

- **At 0 GAE the strict base costs less.** At the framework's recorded charge, the strict-4E base solves complete DLPs with **2.11 bits [1.63, 2.55]** fewer counted operations than the default prefix base. That is the pooled value over 6 cells (n = 13, 19, 23; l = 3, 4), and every one of the 24 per-seed intervals excludes 1.
- **Building the base is not charged.** It takes 2^n membership tests (one inversion, one half-trace and three traces each). Priced at 0.1 GAE per operation, the pooled gain is −0.21 [−1.09, +0.63]. At 1 GAE it is −2.35 [−3.41, −1.31].
- **The preregistered decision rule is not met.** The rule needs the pooled lower end above 0 with the order holding across the band. Under AGENTS.md §5a the two arms are not ranked.
- **Gap to rho.** At 0 GAE, strict4e closes 1.3 to 2.7 bits of the gap to `rho.signed_frobenius_strong`. The closest cell is n = 13, l = 4, at 0.44 bits, about 1.36× rho. At 1 GAE the strict arm is 4.0 to 12.1 bits above rho.

## What it would take

The construction scan is the whole problem. Possible fixes:

- amortise one bitmap per curve across many targets (the framework charges per run);
- a sieve that tests only the candidates it draws, about 4^(2^(l−1)) draws for the last basis vector, instead of all of F_2^n;
- a closed-form strict subspace.

None of these was tried. At ECC2K-130 (n = 131), strict subspaces exist only up to l ≈ 9 (first-moment count in the ml report), so the mechanism gives about 2× yield at l ≈ 43 rather than 16×.

## Validation gate

`python3 scripts/validation_gate.py evidence research/fb_strict4e_20261027` **refuses** this directory, because the validator report is not independent: it was written and run by the producer and only re-run by the coordinating session. The replay certificate is the same-host `ecbench verify --replay-all` receipt. No row goes to the IC leaderboard or `benchmarks/README.md`. Under the gate this directory is a recorded result, not an admitted row.
