# Results: dreg closure at (m, ℓ) = (4, 6), 2026-10-18

Reading of PROTOCOL.md §4 over the records in `runs/` (run report:
`reports/run-20261018T1134Z/`, PDF `report.pdf`). Class: **measurement**
(a degree observation on a frozen cell), not an improvement, not a cost row,
not a claim about ECC2K-130; AGENTS.md §5a's rho pricing does not apply
because no cost is compared here.

## A1 (m = 4, n = 15, ℓ = 6, N = 54, S = −9): refuted at degree 4

| draw | kind | degree-4 closure | record |
|--:|---|---|---|
| 42 | unsatisfiable (committed count 0) | **refuted** (closure rank 1) | `runs/fresh-m4-15-6.d42.jsonl` |
| 214 | unsatisfiable | **refuted** (closure rank 1) | `runs/fresh-m4-15-6.d214.jsonl` |
| 0 | satisfiable control (1,226 solutions) | not refuted (rank 214,391 of 342,537) | `runs/control-m4-15-6.d0.jsonl` |

Every unsatisfiable draw refutes at the built degree and the control does
not, so by §4 row A: **"m·ℓ ≥ 21" is refuted as the explanation of the
degree-5 step; "ℓ = 7" survives.** Read on 2 unsatisfiable draws, not the
4 the protocol asked for (§10: 2 found in 251 at S = −9); the row says so.
Prediction §5.1 (A refutes at 4 on every draw, confidence 0.6) held on the
draws measured. Prediction §5.4 (RSS within 1.5× of projection) held:
25.8 GB measured against 44 GB projected.

XL at degree 4 did not refute on any draw (rank 23,757 on all three), so
the refutation is the closure's, as in the earlier m = 4 cells.

## A2, B1, B2, C2: not run

Deferred by §8 (B, C2) and §10 (A2). No reading; Q2 stays open.

## C1 (m = 3, n = 19, ℓ = 7, degree 5): capped

The degree-5 closure on draw 0 exhausted memory at 169 GB resident under the
184 GB cap after 17.5 min (projection 217 GB). §4: capped, **inconclusive**.
Prediction §5.3 is untested. Q3 needs a host above about 250 GB, or a
sparser closure.

## Recorded against expectation

- Unsatisfiable-draw rate at S = −9 for (4, 6): 2/251, 95 % Clopper–Pearson
  interval 0.1 %–2.8 %, against the §5 expectation of about 4 in 150. The
  (4, 5) driver cells found them faster at the same surplus; the rate at
  ℓ = 6 is now a recorded number for the next protocol's draw budget.
- Dense closure memory at 341k columns: 25.8 GB, 0.59× the 3·cols²/8
  projection.

## Consequence for the ECC2K-130 m = 4 note

None yet. The note's degree-5 bracket (+10.32 bits over rho, converted to
+12.48 by PR #2164's K) rests on the degree holding at 4 through larger
cells; this cell extends the "held at 4" line to m·ℓ = 24 at N = 54, which
is consistent with the optimistic row and does not move it. Q3 (whether the
degree is exactly 5 at ℓ = 7 for m = 3) is still unmeasured.

## Next improvement to try

Run C1 on a 256 GB-plus host (one cell, about 1 h at the measured scaling),
and A2 with a draw budget sized by the 2/251 rate (about 700 draws, 14 h of
generation, or a parallel ladder). Both are host-cost questions, not
instrument questions.
