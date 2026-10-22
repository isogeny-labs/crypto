# Results: four guessed variables bring every ℓ = 8 draw back to degree 4

Scored natively by `dreg_score guess runs/m3-22-8.*.jsonl` (`score-output.txt`)
against `PREREGISTRATION.md` (commit `cd4b90dc7`, Amendments 1 and 2 made
before any degree-4 reading).

- **Records.** Every value comes from a committed record in `runs/`.
- **Run.** 2026-10-22, 05:55–09:51 UTC: 68 processes (4 G6 and 64 G4), all
  exited 0.
- **Interruptions, disclosed in `runs/queue.log`.** The session's 2-hour
  background limit stopped the queue during `d5 G4b7`. That run left an
  empty output and was rerun from scratch. The rest ran in foreground
  batches of two (`MAX_RUNS=2`), with the same binary
  (`runs/binary.sha256`).
- **Instrument fixes before measurement (Amendment 2).** `dreg_closure` now
  reads `--max-cols` against the variables that occur after `--fix`, and
  `run.sh` stops a draw only on a saturated, non-refuting degree-4 entry.
  The 8 degree-3-only outputs of the first launch are kept unscored in
  `runs/invalid-cap-bug/`.
- **Binary provenance.** Every scored run used `dreg_closure` built from
  commit `4fc883827` (SHA-256 `a11dddbb…1349`, `runs/binary.sha256`). A later
  rustfmt-only change to `examples/dreg_closure.rs` (line wrapping, no
  logic) gives a different binary hash, so a rebuild from this PR's head
  will not match the recorded hash.
- **Draws.** m = 3, (n, ℓ) = (22, 8), N = 46 unknowns, 44 equations,
  S = −2. Draws 0, 3, 5 and 6 are unsatisfiable (exact count 0); draws 1, 2
  and 4 are satisfiable (`draws/m3-22-8.jsonl`, seed 20261022).
- **Peak RSS:** 10.6 GB. Degree-4 columns: 124,314 per G4 branch, 102,091
  per G6 run. Degree-4 scan time: 124–331 s per G4 branch.
- **Run report** with graph and PDF: `reports/run-20261022T0555Z/`.

## The measurement

| draw | G6 (6,6,6) | G4 branches refuting at 4 | not refuting | four guesses |
|--:|---|--:|--:|---|
| 0 | refutes | 16 | 0 | suffice |
| 3 | refutes | 16 | 0 | suffice |
| 5 | refutes | 16 | 0 | suffice |
| 6 | refutes | 16 | 0 | suffice |

- **Positive control G6:** refutes on 4 of 4, which is valid.
- **G4:** all 64 branches refute at degree 4. Four guesses suffice on 4 of
  4 draws.
- **Registered verdict: K_marginal favoured, "four guesses suffice at
  ℓ = 8".** K_excess, which predicted 6 guesses (`m·(ℓ−6)`) and so a
  failing branch, is rejected for this guess set.

## What it does and does not say

- **k = 4 is an upper bound.** It is the smallest guess count this
  container can measure at N = 46 (`C(46−k, ≤4)` must stay under 130,000
  columns). Fewer guesses may suffice. K_marginal (`m·(ℓ−7)+1` = 4) is
  favoured only in the registered sense that 4 is enough; a constant k
  (for example k = 1, as at ℓ = 7) is not excluded.
- **One guess set.** Variables 7, 15, 23 and 24: one coordinate per summand
  and one intermediate coordinate. Other four-variable sets are unmeasured.
- **The unspecialised ℓ = 8 system is not measured at degree 4** (179,447
  columns). At ℓ = 7 the unspecialised closure did not refute at 4 on 8 of 8
  draws; at ℓ = 8 that is assumed, not observed.
- **Scope:** m = 3, one cell, four draws, one container (4-core, 16 GB, not
  isolated).

## What this means for ECC2K-130 (derived)

- **The guess-then-degree-4 oracle stays alive, but only if k stays
  small.**
  - Per target it costs `2^k · C(N−k, ≤4)^w`; degree 5 costs
    `C(N, ≤5)^w`.
  - At the cell where the m = 4 degree-4 row is priced (ℓ = 33,
    N = 2·131 + 4·33 = 394, w = 1), the oracle ratio
    `C(394, ≤5) / (2^k · C(394−k, ≤4))` is 78.2 at k = 0, 5.09 at k = 4,
    1.30 at k = 6 and 0.66 at k = 7. **There the hybrid beats degree 5
    only if k ≤ 6.**
  - Both registered rules grow linearly in ℓ and give k ≈ 105–108 there.
    Under either, the hybrid is far worse than degree 5.
  - The measurements so far (k ≈ 1 at ℓ = 7 on 5 of 8 draws, k ≤ 4 at ℓ = 8
    on 4 of 4) do not exclude a k that stays bounded.
- **No row moves and no row beats rho.** The hybrid's k = 0 case is the
  degree-4 row itself, so even a bounded k can at best bring back the m = 4
  `2^68.43` row (`2^+7.62` over rho, unconverted units), which stays
  "contradicted" (closure-driver study) until k is measured to stay bounded.
  The free-oracle floor still binds every row.
- **Against the success criterion.** This narrows the extrapolation rule
  for one oracle model. It is not an improvement over the baseline.

## Successor

- **Is k < 4 enough at ℓ = 8?** k = 3 needs 136,698 columns, about 10%
  over this run's cap. At this run's 10.6 GB peak for 124,314 columns it
  may fit the 15 GB `prlimit` with `--max-cols 140000`: 8 branches per
  draw. k = 1 or 2 (150k–164k columns) needs a larger host.
- **k at ℓ = 9** (m = 3, N ≥ 49) to see whether k grows at all. That needs
  a larger host too.
- Still open from earlier studies: m = 4, ℓ = 6, and degree 5 past ℓ = 7.
