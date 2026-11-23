# Results: the unspecialised (22, 7) system does not refute at degree 4 on 4 of 4 draws

Scored natively by `dreg_score null runs/m3-22-7.*.jsonl runs/fresh-m3-19-7.d0.jsonl`
(`score-output.txt`) against `PREREGISTRATION.md` (commit `e3091a668`, no
amendments).

- **Records.** Every value comes from a committed record in `runs/`.
- **Run.** 2026-10-28, 03:35–04:15 UTC: 11 processes (1 N7, 4 B, 2 S, 4 N0), all
  exited 0 (`runs/queue.log`). None was killed, empty or rerun. A worker restart
  happened after the last run and before scoring and affected nothing.
- **Draws.** m = 3, (n, ℓ) = (22, 7), N = 43, 44 equations, S = +1, seed
  20261028 (`draws/m3-22-7.jsonl`). Unsatisfiable draws 1, 2, 3, 4 (the first
  four, exact count 0); satisfiable draws 0 and 8 (the first two, 6 solutions
  each).
- **Draw generation, disclosed.** It ran as four shards (the registered
  `--shard` replay of the same seeded sequence). Draws 0–11 were generated; I
  stopped the shards once draw 8, the second satisfiable draw, was certified. The
  frozen file holds draws 0–8; draws 9–11 (all unsatisfiable) stay in
  `draws/shard-*.jsonl` and were not used. The frozen file was written in two
  steps (draws 0–7, then draw 8 appended before the satisfiable runs started); its
  first 8 lines are byte-identical across the two.
- **Binaries.** `runs/binary.sha256`, taken before the first run and re-verified
  at scoring: `dreg_closure` `d7246453…2ec4` (unchanged from the three-guess
  study), `dreg_score` `4e6150b0…ecc6a`, `dreg_ladder` `890d47c7…2329`.
- **Memory.** Peak VmHWM 7.05 GB, against the 15 GB `prlimit`; no run was close to
  the limit. Degree-4 columns: 136,698 per N0 run, 102,091 per B run. Scan time:
  245–277 s per N0 run, 130–210 s per B run.
- **Run report** with graph and PDF: `reports/run-20261028T0335Z/`.
- **Scorer note (added after review, verdict unchanged).** The first version of
  `dreg_score null` scored only the files it was given and accepted any non-empty
  satisfiable-control record. It now enumerates the registered draws from the
  frozen draw file, counts a missing or short record as unmeasured, and counts a
  satisfiable control only when its scan reached degree 4, saturated and unrefuted
  (unit tests in `examples/dreg_score.rs`; checked by hand with a missing draw, a
  missing control and a capped control, which give "incomplete", "controls not
  valid" and "controls not valid"). `score-output.txt` was regenerated: the table
  and verdict are unchanged and only two header lines gained wording. The graph's
  counts now come from `dreg_score null-plot-data` (`reports/run-20261028T0335Z/plot-data.json`);
  `plot.py` only draws them. The counts are the same as before.

## The measurement

| draw | N0 (unspecialised, N = 43) | closure rank at degree 4 (of 136,698) | B (6, 6, 6, N = 40) |
|--:|---|--:|---|
| 1 | does not refute | 129,221 | refutes |
| 2 | does not refute | 128,885 | refutes |
| 3 | does not refute | 129,053 | refutes |
| 4 | does not refute | 129,893 | refutes |

- **Registered verdict: "N0 does not refute on at least 3 draws: the pure ℓ = 7
  reading."** N0 does not refute on 4 of 4: degree 4 saturates on every draw
  without the constant `1`.
- **Controls valid.**
  - **N7 (identity):** matches the committed scale-study record
    `research/dreg_closure_scale_20261016/runs/fresh-m3-19-7.d0.jsonl` on every
    non-timing field of both scan entries.
  - **B (positive):** refutes at degree 4 on 4 of 4 draws (need at least 3).
  - **S (soundness):** neither satisfiable draw is refuted.
- **Predictions (preregistered): all held.** N7 matched (0.97), B refuted on at
  least 3 of 4 (0.9), no satisfiable draw refuted (0.99), N0 not refuting on at
  least 3 of 4 (0.75).
- **The XL rank is the same on all four N0 draws** (21,491). The closure ranks
  differ by draw, and two of them (129,221 and 129,893) equal the ranks of failing
  three-guess runs on ℓ = 8 draws 0 and 5; the other two N0 ranks and the third
  failing three-guess rank (128,213) do not match. No inference is drawn from that.

## What it does and does not say

- **It says** that, at N = 43 and 136,698 columns, an unspecialised ℓ = 7 system
  does not refute at degree 4 on any of 4 draws. With the closure-at-scale cells
  (n = 19 and 21), that is **12 of 12 unspecialised ℓ = 7 draws** not refuting,
  over N = 40, 42, 43 and surplus S = −2, 0, +1. So the earlier ℓ = 7 failure is
  not an artefact of a non-positive surplus, the one confound the registration
  kept.
- **Relation to the three-guess result.** At ℓ = 8, the 3-set (variables 7, 15,
  23) failed on 3 of 4 draws and refuted on one (draw 6). The unspecialised
  ℓ = 7 cell with the same N and column count fails on 4 of 4. With 4 draws each,
  "3 of 4" and "4 of 4" are not distinguishable, so this study does not say the
  two cells differ. The draw-6 exception of the 3-set is not explained.
- **It does not say** that the 3-set systems are identical to random ℓ = 7
  systems. They come from ℓ = 8 draws with fixed coordinates, so the match is an
  analogy supported by one measured behaviour, not a structural identity.
- **Only degree 4 is measured.** "Does not refute" means the closure degree is
  at least 5; degree 5 is not measured here.
- **Not measured:** other three-variable sets, ℓ = 9, m = 4, any other surplus.
- **Scope:** m = 3, one cell, four draws, one container (4-core, 16 GB, not
  isolated). Class: stage diagnostic. No S, no rho ratio, no speed claim.

## What this means for ECC2K-130 (derived)

- **The reading from the three-guess study is supported.** Fixing the top
  coordinate of each summand brings ℓ = 8 down to ℓ = 7, and an ℓ = 7 system is
  where degree 4 stops refuting. So the three-guess failure is what ℓ = 7 does,
  and the four-guess success needs a further guess on top of the three that reach
  ℓ = 7: the pattern `m·(ℓ−7)+1`, which gives 4 at m = 3, ℓ = 8.
- **Extended linearly, that rule gives 4·(33−7)+1 = 105 guesses at the priced
  m = 4, ℓ = 33 cell,** against a break-even of k ≤ 6 for the guess-then-degree-4
  oracle to beat degree 5 (`2^k · C(N−k, ≤4)` against `C(N, ≤5)` at N = 394). That
  is an extrapolation from ℓ = 7 (about 1 guess) and ℓ = 8 (4 guesses suffice,
  3 of that kind do not), not a measurement at ℓ ≥ 9.
- **What would change it.** Only a way to bring ℓ down that costs far less than one
  guessed coordinate per summand and per unit of ℓ, or a degree-5 closure that
  holds past ℓ = 7. Neither is measured here.
- **No row moves and no row beats rho.** The free-oracle floor still binds every
  row.
- **Against the success criterion.** This narrows the extrapolation rule for one
  oracle model and closes a registered interpretation question. It is not an
  improvement over the index-calculus baseline.

## Successor

- **The shape arm**, which the reading predicts will fail: a three-variable set
  that leaves one summand at dimension 8 (for example variables 7, 15 and 24) at
  ℓ = 8 on the same four draws (0, 3, 5, 6). If it refutes on most draws, the
  "bring each summand to ℓ = 7" reading is wrong. It needs 136,698 columns and
  about 7 GB, which fit this container.
- **A degree-5 measurement at ℓ = 7** (the registered "C1" cell of the
  m = 4, ℓ = 6 study was capped at 169 GB): needs a host above about 250 GB or a
  sparse closure.
- **Other leads** from `research/notes/ecc2k130/RESEARCH_ECC2K130_NEW_LEADS_20261023.md`:
  the E-capped closure and the tree-against-chain encoding are the two that
  could lower the closure's cost at a fixed degree.
